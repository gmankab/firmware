// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x66000,0x67000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x66000;
pub const END:usize=0x67000;
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
 // Executable A32 block 0x66000..0x67000.
 c.check(0x66000)?;
    // 0x66000: blo 0x6602c <entry+0x1082c> @ imm = #0x24
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6602c,false)?)?;
    // 0x66004: ldr r1, [r2, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,116)?)?;
    // 0x66008: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x6600c: bne 0x6602c <entry+0x1082c> @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6602c,false)?)?;
    // 0x66010: ldr r1, [r2, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,108)?)?;
    // 0x66014: ldr r2, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-52)?)?;
    // 0x66018: ldr r2, [r2, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,92)?)?;
    // 0x6601c: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // 0x66020: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x66024: ldrlo r1, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Cc,true,R::R1,R::R11,-64)?)?;
    // 0x66028: ldrlo r9, [r1, #0x9d0]
    c.emit(arm32::transfer_word(C::Cc,true,R::new(9)?,R::R1,2512)?)?;
    // 0x6602c: ldr r7, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-60)?)?;
    // 0x66030: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x66034: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x66038: ldr r0, [r7, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    // 0x6603c: bl 0x471c <entry+0xfffffffffffaef1c> @ imm = #-0x61928
    c.emit(arm32::branch(C::Al,c.at as u32,0x471c,true)?)?;
    // 0x66040: ldr r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // 0x66044: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x66048: ldrb r10, [r0, #0x3e5]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(10)?,R::R0,997)?)?;
    // 0x6604c: ldr r0, [r7, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(10)?,Shift::Lsl,2)?)?;
    // 0x66050: str r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-56)?)?;
    // 0x66054: ldr r7, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,68)?)?;
    // 0x66058: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x6605c: ldrb r1, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,1)?)?;
    // 0x66060: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x66064: strb r1, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1)?)?;
    // 0x66068: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x6606c: cmp r5, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x66070: bne 0x65f7c <entry+0x1077c> @ imm = #-0xfc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65f7c,false)?)?;
    // 0x66074: ldr r5, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-48)?)?;
    // 0x66078: ldr r10, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-52)?)?;
    // 0x6607c: ldr r0, [r5, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,400)?)?;
    // 0x66080: b 0x6608c <entry+0x1088c> @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x6608c,false)?)?;
    // 0x66084: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x66088: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x6608c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x66090: bne 0x660ec <entry+0x108ec> @ imm = #0x54
    c.emit(arm32::branch(C::Ne,c.at as u32,0x660ec,false)?)?;
    // 0x66094: ldr r4, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-64)?)?;
    // 0x66098: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x6609c: bne 0x6612c <entry+0x1092c> @ imm = #0x88
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6612c,false)?)?;
    // 0x660a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x660a4: str r0, [r5, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,400)?)?;
    // 0x660a8: ldrb r1, [r5, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,1005)?)?;
    // 0x660ac: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x660b0: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x660b4: strb r1, [r5, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,1005)?)?;
    // 0x660b8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x660bc: strb r1, [r5, #0x3ee]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,1006)?)?;
    // 0x660c0: strb r0, [r5, #0x172]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,370)?)?;
    // 0x660c4: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x660c8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x660cc: ldreq r0, [r4, #0x2e8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,744)?)?;
    // 0x660d0: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x660d4: bne 0x6612c <entry+0x1092c> @ imm = #0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6612c,false)?)?;
    // 0x660d8: ldr r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
    // 0x660dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x660e0: beq 0x660f4 <entry+0x108f4> @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x660f4,false)?)?;
    // 0x660e4: ldr r0, [r10, #0x154]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,340)?)?;
    // 0x660e8: b 0x66128 <entry+0x10928> @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x66128,false)?)?;
    // 0x660ec: ldr r4, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-64)?)?;
    // 0x660f0: b 0x6612c <entry+0x1092c> @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x6612c,false)?)?;
    // 0x660f4: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x660f8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x660fc: bne 0x66120 <entry+0x10920> @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66120,false)?)?;
    // 0x66100: ldr r1, [r4, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1236)?)?;
    // 0x66104: ldr r0, [r5, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,420)?)?;
    // 0x66108: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x6610c: ldrbne r1, [r5, #0x17a]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::new(5)?,378)?)?;
    // 0x66110: addne r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x66114: ldr r0, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,108)?)?;
    // 0x66118: lsr r0, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,8)?)?;
    // 0x6611c: b 0x66128 <entry+0x10928> @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x66128,false)?)?;
    // 0x66120: ldr r0, [r5, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,420)?)?;
    // 0x66124: ldr r0, [r0, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,112)?)?;
    // 0x66128: strb r0, [r5, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,380)?)?;
    // 0x6612c: add r0, r5, #368
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,368)?)?;
    // 0x66130: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x66134: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x66138: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x6613c: beq 0x661cc <entry+0x109cc> @ imm = #0x88
    c.emit(arm32::branch(C::Eq,c.at as u32,0x661cc,false)?)?;
    // 0x66140: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x66144: bne 0x661e8 <entry+0x109e8> @ imm = #0x9c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x661e8,false)?)?;
    // 0x66148: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x6614c: bl 0x7b65c <entry+0x25e5c> @ imm = #0x15508
    c.emit(arm32::branch(C::Al,c.at as u32,0x7b65c,true)?)?;
    // 0x66150: ldr r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,908)?)?;
    // 0x66154: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66158: beq 0x6616c <entry+0x1096c> @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6616c,false)?)?;
    // 0x6615c: ldrb r0, [r4, #0x399]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,921)?)?;
    // 0x66160: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x66164: movne r0, r10
    c.emit(arm32::mov_register(C::Ne,R::R0,R::new(10)?))?;
    // 0x66168: blne 0x67104 <entry+0x11904> @ imm = #0xf94
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67104,true)?)?;
    // 0x6616c: ldr r7, [r10, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,588)?)?;
    // 0x66170: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x66174: beq 0x661e0 <entry+0x109e0> @ imm = #0x64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x661e0,false)?)?;
    // 0x66178: ldr r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,88)?)?;
    // 0x6617c: add r1, r10, #676
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,676)?)?;
    // 0x66180: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x66184: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x66188: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x6618c: ldr r6, [r1, #-0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,-12)?)?;
    // 0x66190: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x66194: beq 0x661b4 <entry+0x109b4> @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x661b4,false)?)?;
    // 0x66198: ldr r6, [r1, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,-4)?)?;
    // 0x6619c: ldr r7, [r1, #-0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,-20)?)?;
    // 0x661a0: and r6, r6, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x661a4: cmp r7, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x661a8: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x661ac: movhi r2, r7
    c.emit(arm32::mov_register(C::Hi,R::R2,R::new(7)?))?;
    // 0x661b0: ldr r7, [r10, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,588)?)?;
    // 0x661b4: ldr r5, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-48)?)?;
    // 0x661b8: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x661bc: add r1, r1, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,72)?)?;
    // 0x661c0: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x661c4: blo 0x6618c <entry+0x1098c> @ imm = #-0x40
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6618c,false)?)?;
    // 0x661c8: b 0x661e4 <entry+0x109e4> @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x661e4,false)?)?;
    // 0x661cc: ldr r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,908)?)?;
    // 0x661d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x661d4: movne r0, r10
    c.emit(arm32::mov_register(C::Ne,R::R0,R::new(10)?))?;
    // 0x661d8: blne 0x67104 <entry+0x11904> @ imm = #0xf24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67104,true)?)?;
    // 0x661dc: b 0x661e8 <entry+0x109e8> @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x661e8,false)?)?;
    // 0x661e0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x661e4: str r2, [r10, #0x660]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,1632)?)?;
    // 0x661e8: ldrb r1, [r9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x661ec: ldrb r2, [r9, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,1)?)?;
    // 0x661f0: ldrb r3, [r9, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,2)?)?;
    // 0x661f4: ldr r0, [r5, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,988)?)?;
    // 0x661f8: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x661fc: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x66200: add r4, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x66204: ldr r1, [r2, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,44)?)?;
    // 0x66208: ldr r10, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R1,36)?)?;
    // 0x6620c: ldr r8, [r10, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(10)?,368)?)?;
    // 0x66210: ldrb r1, [r8, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1)?)?;
    // 0x66214: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x66218: mvnne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x6621c: strne r1, [r10, #0x278]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(10)?,632)?)?;
    // 0x66220: strne r1, [r10, #0x27c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(10)?,636)?)?;
    // 0x66224: strne r1, [r10, #0x280]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(10)?,640)?)?;
    // 0x66228: strne r1, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(10)?,644)?)?;
    // 0x6622c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x66230: beq 0x66258 <entry+0x10a58> @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66258,false)?)?;
    // 0x66234: add r1, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,4)?)?;
    // 0x66238: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x6623c: sub r3, r11, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R11,44)?)?;
    // 0x66240: ldrsb r6, [r1], #1
    c.emit(emit_signed_byte_postindexed(C::Al,R::new(6)?,R::R1,1)?)?;
    // 0x66244: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x66248: cmp r2, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x6624c: add r6, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x66250: str r6, [r3], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(6)?,R::new(3)?,4)?)?;
    // 0x66254: bne 0x66240 <entry+0x10a40> @ imm = #-0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66240,false)?)?;
    // 0x66258: ldr r0, [r10, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,588)?)?;
    // 0x6625c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66260: beq 0x663a8 <entry+0x10ba8> @ imm = #0x140
    c.emit(arm32::branch(C::Eq,c.at as u32,0x663a8,false)?)?;
    // 0x66264: add r6, r10, #656
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(10)?,656)?)?;
    // 0x66268: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x6626c: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x66270: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x66274: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // 0x66278: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6627c: beq 0x66368 <entry+0x10b68> @ imm = #0xe4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66368,false)?)?;
    // 0x66280: ldrb r1, [r8, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1)?)?;
    // 0x66284: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x66288: bne 0x66374 <entry+0x10b74> @ imm = #0xe4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66374,false)?)?;
    // 0x6628c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x66290: beq 0x662bc <entry+0x10abc> @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x662bc,false)?)?;
    // 0x66294: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x66298: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x6629c: sub r3, r11, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R11,44)?)?;
    // 0x662a0: ldr r7, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x662a4: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x662a8: beq 0x662c4 <entry+0x10ac4> @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x662c4,false)?)?;
    // 0x662ac: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x662b0: add r3, r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4)?)?;
    // 0x662b4: cmp r2, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x662b8: blt 0x662a0 <entry+0x10aa0> @ imm = #-0x20
    c.emit(arm32::branch(C::Lt,c.at as u32,0x662a0,false)?)?;
    // 0x662bc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x662c0: b 0x662c8 <entry+0x10ac8> @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x662c8,false)?)?;
    // 0x662c4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x662c8: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x662cc: bne 0x662e4 <entry+0x10ae4> @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x662e4,false)?)?;
    // 0x662d0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x662d4: ldrbeq r0, [r8, #0x1dc]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(8)?,476)?)?;
    // 0x662d8: subeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x662dc: strbeq r0, [r8, #0x1dc]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::new(8)?,476)?)?;
    // 0x662e0: b 0x66368 <entry+0x10b68> @ imm = #0x80
    c.emit(arm32::branch(C::Al,c.at as u32,0x66368,false)?)?;
    // 0x662e4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x662e8: bne 0x66368 <entry+0x10b68> @ imm = #0x78
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66368,false)?)?;
    // 0x662ec: ldr r0, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,76)?)?;
    // 0x662f0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x662f4: bne 0x6632c <entry+0x10b2c> @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6632c,false)?)?;
    // 0x662f8: ldrb r1, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,10)?)?;
    // 0x662fc: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // 0x66300: blo 0x6632c <entry+0x10b2c> @ imm = #0x24
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6632c,false)?)?;
    // 0x66304: ldrb r0, [r8, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,2)?)?;
    // 0x66308: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6630c: beq 0x66368 <entry+0x10b68> @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66368,false)?)?;
    // 0x66310: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x66314: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x66318: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x6631c: bl 0x61960 <entry+0xc160>  @ imm = #-0x49c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x61960,true)?)?;
    // 0x66320: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x66324: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x66328: b 0x66368 <entry+0x10b68> @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x66368,false)?)?;
    // 0x6632c: ldr r1, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-64)?)?;
    // 0x66330: ldr r1, [r1, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1392)?)?;
    // 0x66334: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x66338: beq 0x66364 <entry+0x10b64> @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66364,false)?)?;
    // 0x6633c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x66340: bne 0x66364 <entry+0x10b64> @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66364,false)?)?;
    // 0x66344: ldrb r0, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,10)?)?;
    // 0x66348: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x6634c: blo 0x66364 <entry+0x10b64> @ imm = #0x10
    c.emit(arm32::branch(C::Cc,c.at as u32,0x66364,false)?)?;
    // 0x66350: ldr r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-64)?)?;
    // 0x66354: ldr r0, [r0, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,908)?)?;
    // 0x66358: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6635c: streq r12, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R12,R::new(6)?,8)?)?;
    // 0x66360: b 0x66368 <entry+0x10b68> @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x66368,false)?)?;
    // 0x66364: str r12, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,8)?)?;
    // 0x66368: ldrb r0, [r8, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,1)?)?;
    // 0x6636c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66370: beq 0x66380 <entry+0x10b80> @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66380,false)?)?;
    // 0x66374: str r12, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,8)?)?;
    // 0x66378: strb lr, [r6, #0x2a]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(6)?,42)?)?;
    // 0x6637c: b 0x6638c <entry+0x10b8c> @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x6638c,false)?)?;
    // 0x66380: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // 0x66384: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66388: strbeq lr, [r6, #0x2a]
    c.emit(arm32::transfer_byte(C::Eq,false,R::LR,R::new(6)?,42)?)?;
    // 0x6638c: cmp r5, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,7)?)?;
    // 0x66390: bgt 0x663a8 <entry+0x10ba8> @ imm = #0x10
    c.emit(arm32::branch(C::Gt,c.at as u32,0x663a8,false)?)?;
    // 0x66394: ldr r0, [r10, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,588)?)?;
    // 0x66398: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x6639c: add r6, r6, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,72)?)?;
    // 0x663a0: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x663a4: blo 0x66274 <entry+0x10a74> @ imm = #-0x138
    c.emit(arm32::branch(C::Cc,c.at as u32,0x66274,false)?)?;
    // 0x663a8: ldr r10, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-52)?)?;
    // 0x663ac: ldr r0, [r10, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,588)?)?;
    // 0x663b0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x663b4: beq 0x663f0 <entry+0x10bf0> @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x663f0,false)?)?;
    // 0x663b8: ldr r8, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-48)?)?;
    // 0x663bc: add r1, r10, #684
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,684)?)?;
    // 0x663c0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x663c4: ldr r2, [r1, #-0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,-20)?)?;
    // 0x663c8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x663cc: bne 0x663dc <entry+0x10bdc> @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x663dc,false)?)?;
    // 0x663d0: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // 0x663d4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x663d8: bne 0x663f8 <entry+0x10bf8> @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x663f8,false)?)?;
    // 0x663dc: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x663e0: add r1, r1, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,72)?)?;
    // 0x663e4: cmp r4, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x663e8: blo 0x663c4 <entry+0x10bc4> @ imm = #-0x2c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x663c4,false)?)?;
    // 0x663ec: b 0x663f8 <entry+0x10bf8> @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x663f8,false)?)?;
    // 0x663f0: ldr r8, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-48)?)?;
    // 0x663f4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x663f8: cmp r4, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x663fc: bne 0x66434 <entry+0x10c34> @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66434,false)?)?;
    // 0x66400: ldr r1, [pc, #0xc8c]        @ 0x11894 <entry+0x11894>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3212)?)?;
    // 0x66404: ldr r2, [pc, #0xc8c]        @ 0x11898 <entry+0x11898>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3212)?)?;
    // 0x66408: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x6640c: mov r3, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,716)?)?;
    // 0x66410: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x37784
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x66414: ldr r1, [pc, #0xc80]        @ 0x1189c <entry+0x1189c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3200)?)?;
    // 0x66418: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x6641c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x37790
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x66420: ldr r2, [pc, #0xc78]        @ 0x118a0 <entry+0x118a0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3192)?)?;
    // 0x66424: mov r1, #205
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,205)?)?;
    // 0x66428: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6642c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x66430: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x64c88
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x66434: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x66438: add r1, r4, r4, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::new(4)?,Shift::Lsl,3,false)?)?;
    // 0x6643c: str r4, [r0, #0x248]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(4)?,R::R0,584)?)?;
    // 0x66440: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // 0x66444: add r5, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66448: ldr r0, [r5, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,100)?)?;
    // 0x6644c: str r0, [r8, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,424)?)?;
    // 0x66450: str r4, [r8, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,428)?)?;
    // 0x66454: ldr r4, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-64)?)?;
    // 0x66458: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x6645c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x66460: beq 0x66478 <entry+0x10c78> @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66478,false)?)?;
    // 0x66464: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x66468: bne 0x66490 <entry+0x10c90> @ imm = #0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66490,false)?)?;
    // 0x6646c: ldr r0, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,924)?)?;
    // 0x66470: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66474: beq 0x66490 <entry+0x10c90> @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66490,false)?)?;
    // 0x66478: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // 0x6647c: add r0, r10, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66480: ldr r0, [r0, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,692)?)?;
    // 0x66484: str r0, [r8, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,440)?)?;
    // 0x66488: str r0, [r8, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,444)?)?;
    // 0x6648c: b 0x664d4 <entry+0x10cd4> @ imm = #0x40
    c.emit(arm32::branch(C::Al,c.at as u32,0x664d4,false)?)?;
    // 0x66490: ldr r0, [r10, #0x668]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1640)?)?;
    // 0x66494: ldr r1, [r4, #0x608]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1544)?)?;
    // 0x66498: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x6649c: bl 0x81850 <entry+0x2c050> @ imm = #0x1b3ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x664a0: add r0, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x664a4: add r2, r10, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(10)?,648)?)?;
    // 0x664a8: str r1, [r10, #0x668]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1640)?)?;
    // 0x664ac: add r0, r2, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,3,false)?)?;
    // 0x664b0: ldr r0, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,44)?)?;
    // 0x664b4: str r0, [r8, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,444)?)?;
    // 0x664b8: ldr r0, [r10, #0x66c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1644)?)?;
    // 0x664bc: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x664c0: add r0, r2, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,3,false)?)?;
    // 0x664c4: ldr r0, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,44)?)?;
    // 0x664c8: str r0, [r8, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,440)?)?;
    // 0x664cc: str r1, [r10, #0x66c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1644)?)?;
    // 0x664d0: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // 0x664d4: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,60)?)?;
    // 0x664d8: add r7, r10, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(10)?,R::R1,Shift::Lsl,3,false)?)?;
    // 0x664dc: add r3, r5, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(5)?,64)?)?;
    // 0x664e0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x664e4: strb r0, [r7, #0x2b9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,697)?)?;
    // 0x664e8: ldrb r0, [r8, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,378)?)?;
    // 0x664ec: str r0, [r7, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,668)?)?;
    // 0x664f0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x664f4: strb r0, [r1, #0x2b0]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::R1,688)?)?;
    // 0x664f8: str r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,44)?)?;
    // 0x664fc: ldrb r1, [r8, #0x172]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,370)?)?;
    // 0x66500: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x66504: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x66508: bne 0x6651c <entry+0x10d1c> @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6651c,false)?)?;
    // 0x6650c: ldr r2, [r10, #0x670]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1648)?)?;
    // 0x66510: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x66514: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x66518: moveq r1, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x6651c: add r2, r4, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,640)?)?;
    // 0x66520: str r2, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,36)?)?;
    // 0x66524: add r2, r10, #584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(10)?,584)?)?;
    // 0x66528: str r2, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,56)?)?;
    // 0x6652c: str r1, [r7, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,664)?)?;
    // 0x66530: ldr r1, [r8, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,988)?)?;
    // 0x66534: str r1, [r7, #0x290]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,656)?)?;
    // 0x66538: ldr r1, [r8, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,676)?)?;
    // 0x6653c: str r1, [r7, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,672)?)?;
    // 0x66540: ldr r1, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,400)?)?;
    // 0x66544: strb r1, [r7, #0x2b8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(7)?,696)?)?;
    // 0x66548: ldr r1, [r8, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,420)?)?;
    // 0x6654c: ldr r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,20)?)?;
    // 0x66550: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,16)?)?;
    // 0x66554: str r3, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,60)?)?;
    // 0x66558: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x6655c: str r1, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,4)?)?;
    // 0x66560: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x66564: add r2, r7, #696
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,696)?)?;
    // 0x66568: strb r0, [r1, #0x3ec]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::R1,1004)?)?;
    // 0x6656c: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x66570: strb r0, [r1, #-0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,-1)?)?;
    // 0x66574: mov r1, #189
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,189)?)?;
    // 0x66578: str r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,40)?)?;
    // 0x6657c: add r2, r7, #656
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,656)?)?;
    // 0x66580: str r7, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,68)?)?;
    // 0x66584: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x66588: str r2, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,48)?)?;
    // 0x6658c: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x66590: add r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x66594: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x66598: strb r2, [r1, #-0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,-2)?)?;
    // 0x6659c: strb r2, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x665a0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x665a4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x665a8: bne 0x66594 <entry+0x10d94> @ imm = #-0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66594,false)?)?;
    // 0x665ac: mov r0, #1012
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1012)?)?;
    // 0x665b0: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x665b4: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x665b8: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // 0x665bc: mov r7, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4278190080)?)?;
    // 0x665c0: strh r1, [r2, r0]!
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::R2,R::R0,true)?)?;
    // 0x665c4: mov r0, #246
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,246)?)?;
    // 0x665c8: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x665cc: orr r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
    // 0x665d0: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x665d4: strh r1, [r2, r0]!
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::R2,R::R0,true)?)?;
    // 0x665d8: mov r0, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1016)?)?;
    // 0x665dc: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x665e0: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x665e4: strh r1, [r2, r0]!
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::R2,R::R0,true)?)?;
    // 0x665e8: mov r0, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,250)?)?;
    // 0x665ec: orr r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
    // 0x665f0: str r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // 0x665f4: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x665f8: strh r1, [r8, r0]
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::new(8)?,R::R0,false)?)?;
    // 0x665fc: mov r0, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1020)?)?;
    // 0x66600: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // 0x66604: strh r1, [r6, r0]!
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::new(6)?,R::R0,true)?)?;
    // 0x66608: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x6660c: strh r1, [r6, #2]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(6)?,2)?)?;
    // 0x66610: strb r0, [r6, #-0x15]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,-21)?)?;
    // 0x66614: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x66618: add r1, r4, #984
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,984)?)?;
    // 0x6661c: strb r0, [r6, #-0x16]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,-22)?)?;
    // 0x66620: add lr, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R1,1024)?)?;
    // 0x66624: ldrsb r2, [r4, r2]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R2,R::new(4)?,R::R2)?)?;
    // 0x66628: ldr r3, [lr, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::LR,R::R2,Shift::Lsl,2)?)?;
    // 0x6662c: add r7, r7, r2, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::R2,Shift::Lsl,24,false)?)?;
    // 0x66630: str r3, [r6, #-0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,-568)?)?;
    // 0x66634: add r3, r4, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(4)?,R::R2,Shift::Lsl,2,false)?)?;
    // 0x66638: ldr r3, [r3, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,2032)?)?;
    // 0x6663c: str r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,32)?)?;
    // 0x66640: str r3, [r6, #-0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,-564)?)?;
    // 0x66644: and r3, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R2,255)?)?;
    // 0x66648: asr r6, r7, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(7)?,Shift::Asr,24)?)?;
    // 0x6664c: add r5, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(3)?,255)?)?;
    // 0x66650: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x66654: strb r5, [r4, #0x809]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2057)?)?;
    // 0x66658: ldrb r1, [r4, #0x80a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2058)?)?;
    // 0x6665c: addlt r5, r1, #255
    c.emit(arm32::data_immediate(C::Lt,Op::Add,R::new(5)?,R::R1,255)?)?;
    // 0x66660: lsl r7, r1, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R1,Shift::Lsl,24)?)?;
    // 0x66664: strblt r5, [r4, #0x809]
    c.emit(arm32::transfer_byte(C::Lt,false,R::new(5)?,R::new(4)?,2057)?)?;
    // 0x66668: lsl r6, r5, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(5)?,Shift::Lsl,24)?)?;
    // 0x6666c: asr r7, r7, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Asr,24)?)?;
    // 0x66670: ldr r1, [lr, r6, asr #22]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::LR,R::new(6)?,Shift::Asr,22)?)?;
    // 0x66674: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // 0x66678: str r1, [r8, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,448)?)?;
    // 0x6667c: add r1, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(3)?,1)?)?;
    // 0x66680: str r2, [r10, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,596)?)?;
    // 0x66684: lsl r2, r1, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,24)?)?;
    // 0x66688: asr r2, r2, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,24)?)?;
    // 0x6668c: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x66690: movlt r0, r1
    c.emit(arm32::mov_register(C::Lt,R::R0,R::R1))?;
    // 0x66694: strb r0, [r4, #0x808]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2056)?)?;
    // 0x66698: ldr r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,908)?)?;
    // 0x6669c: ldr r8, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,56)?)?;
    // 0x666a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x666a4: beq 0x666b4 <entry+0x10eb4> @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x666b4,false)?)?;
    // 0x666a8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x666ac: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x666b0: bl 0x671e0 <entry+0x119e0> @ imm = #0xb28
    c.emit(arm32::branch(C::Al,c.at as u32,0x671e0,true)?)?;
    // 0x666b4: ldr r0, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,400)?)?;
    // 0x666b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x666bc: beq 0x66778 <entry+0x10f78> @ imm = #0xb4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66778,false)?)?;
    // 0x666c0: ldrb r7, [r9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,0)?)?;
    // 0x666c4: ldr r4, [r10, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,1656)?)?;
    // 0x666c8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x666cc: str r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // 0x666d0: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x666d4: beq 0x66804 <entry+0x11004> @ imm = #0x128
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66804,false)?)?;
    // 0x666d8: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x666dc: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x666e0: bne 0x66808 <entry+0x11008> @ imm = #0x120
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66808,false)?)?;
    // 0x666e4: add r4, r9, #11
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,11)?)?;
    // 0x666e8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x666ec: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x666f0: ldrb r0, [r4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x666f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x666f8: beq 0x6675c <entry+0x10f5c> @ imm = #0x5c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6675c,false)?)?;
    // 0x666fc: ldrsb r0, [r4, #-7]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::new(4)?,-7)?)?;
    // 0x66700: ldr r1, [r6, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,988)?)?;
    // 0x66704: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x66708: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x6670c: bl 0x670a4 <entry+0x118a4> @ imm = #0x990
    c.emit(arm32::branch(C::Al,c.at as u32,0x670a4,true)?)?;
    // 0x66710: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66714: blt 0x66734 <entry+0x10f34> @ imm = #0x18
    c.emit(arm32::branch(C::Lt,c.at as u32,0x66734,false)?)?;
    // 0x66718: ldrb r1, [r9, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,3)?)?;
    // 0x6671c: cmp r10, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x66720: ldrlt r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Lt,true,R::R1,R::SP,68)?)?;
    // 0x66724: addlt r1, r1, r10
    c.emit(arm32::data_register(C::Lt,Op::Add,R::R1,R::R1,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x66728: addlt r10, r10, #1
    c.emit(arm32::data_immediate(C::Lt,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // 0x6672c: strblt r0, [r1, #0x2bb]
    c.emit(arm32::transfer_byte(C::Lt,false,R::R0,R::R1,699)?)?;
    // 0x66730: b 0x6675c <entry+0x10f5c> @ imm = #0x24
    c.emit(arm32::branch(C::Al,c.at as u32,0x6675c,false)?)?;
    // 0x66734: ldr r1, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-64)?)?;
    // 0x66738: ldr r0, [r1, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,908)?)?;
    // 0x6673c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66740: beq 0x6675c <entry+0x10f5c> @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6675c,false)?)?;
    // 0x66744: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x66748: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x6674c: str r2, [r0, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1656)?)?;
    // 0x66750: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x66754: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x66758: bl 0x671e0 <entry+0x119e0> @ imm = #0xa80
    c.emit(arm32::branch(C::Al,c.at as u32,0x671e0,true)?)?;
    // 0x6675c: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x66760: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x66764: cmp r5, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x66768: bne 0x666f0 <entry+0x10ef0> @ imm = #-0x80
    c.emit(arm32::branch(C::Ne,c.at as u32,0x666f0,false)?)?;
    // 0x6676c: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x66770: ldr r4, [r0, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,1656)?)?;
    // 0x66774: b 0x66808 <entry+0x11008> @ imm = #0x8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x66808,false)?)?;
    // 0x66778: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,60)?)?;
    // 0x6677c: ldrd r0, r1, [r0]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x66780: str r0, [r6, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,640)?)?;
    // 0x66784: str r1, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,644)?)?;
    // 0x66788: str r0, [r6, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,648)?)?;
    // 0x6678c: str r1, [r6, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,652)?)?;
    // 0x66790: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // 0x66794: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x66798: str r0, [r6, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,668)?)?;
    // 0x6679c: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x667a0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x667a4: moveq r0, r4
    c.emit(arm32::mov_register(C::Eq,R::R0,R::new(4)?))?;
    // 0x667a8: bleq 0x617ec <entry+0xbfec>  @ imm = #-0x4fc4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x617ec,true)?)?;
    // 0x667ac: ldr r0, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1392)?)?;
    // 0x667b0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x667b4: ldrbeq r0, [r6, #0x17a]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(6)?,378)?)?;
    // 0x667b8: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x667bc: bne 0x66fa4 <entry+0x117a4> @ imm = #0x7e0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66fa4,false)?)?;
    // 0x667c0: ldr r0, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,76)?)?;
    // 0x667c4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x667c8: bne 0x66fa4 <entry+0x117a4> @ imm = #0x7d4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66fa4,false)?)?;
    // 0x667cc: ldrb r0, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,10)?)?;
    // 0x667d0: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x667d4: blo 0x66fa4 <entry+0x117a4> @ imm = #0x7c8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x66fa4,false)?)?;
    // 0x667d8: ldr r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,908)?)?;
    // 0x667dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x667e0: beq 0x66fa4 <entry+0x117a4> @ imm = #0x7bc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66fa4,false)?)?;
    // 0x667e4: ldrb r0, [r6, #0x171]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,369)?)?;
    // 0x667e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x667ec: bne 0x66fa4 <entry+0x117a4> @ imm = #0x7b0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66fa4,false)?)?;
    // 0x667f0: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x667f4: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x667f8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x667fc: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x66800: b 0x65e24 <entry+0x10624> @ imm = #-0x9e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x65e24,false)?)?;
    // 0x66804: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x66808: ldrb r5, [r9, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(9)?,1)?)?;
    // 0x6680c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x66810: beq 0x668a8 <entry+0x110a8> @ imm = #0x90
    c.emit(arm32::branch(C::Eq,c.at as u32,0x668a8,false)?)?;
    // 0x66814: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x66818: bne 0x668a8 <entry+0x110a8> @ imm = #0x88
    c.emit(arm32::branch(C::Ne,c.at as u32,0x668a8,false)?)?;
    // 0x6681c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x66820: add r4, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,4)?)?;
    // 0x66824: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x66828: str r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // 0x6682c: ldrb r0, [r9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x66830: ldr r1, [r6, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,988)?)?;
    // 0x66834: add r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x66838: ldrsb r0, [r0]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::R0,0)?)?;
    // 0x6683c: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x66840: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x66844: bl 0x670a4 <entry+0x118a4> @ imm = #0x858
    c.emit(arm32::branch(C::Al,c.at as u32,0x670a4,true)?)?;
    // 0x66848: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6684c: blt 0x66878 <entry+0x11078> @ imm = #0x24
    c.emit(arm32::branch(C::Lt,c.at as u32,0x66878,false)?)?;
    // 0x66850: ldrb r1, [r9, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,3)?)?;
    // 0x66854: ldr r2, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-56)?)?;
    // 0x66858: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x6685c: bge 0x66890 <entry+0x11090> @ imm = #0x2c
    c.emit(arm32::branch(C::Ge,c.at as u32,0x66890,false)?)?;
    // 0x66860: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x66864: add r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x66868: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x6686c: strb r0, [r1, #0x2bd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,701)?)?;
    // 0x66870: str r2, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-56)?)?;
    // 0x66874: b 0x66890 <entry+0x11090> @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x66890,false)?)?;
    // 0x66878: ldrb r0, [r6, #0x63c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,1596)?)?;
    // 0x6687c: ldr r1, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-60)?)?;
    // 0x66880: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x66884: ldrb r1, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,1)?)?;
    // 0x66888: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x6688c: strb r1, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1)?)?;
    // 0x66890: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x66894: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x66898: cmp r7, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x6689c: bne 0x6682c <entry+0x1102c> @ imm = #-0x78
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6682c,false)?)?;
    // 0x668a0: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x668a4: ldr r4, [r0, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,1656)?)?;
    // 0x668a8: cmp r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,1)?)?;
    // 0x668ac: blt 0x668f8 <entry+0x110f8> @ imm = #0x44
    c.emit(arm32::branch(C::Lt,c.at as u32,0x668f8,false)?)?;
    // 0x668b0: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x668b4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x668b8: add r7, r8, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(8)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x668bc: ldrb r0, [r9, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,3)?)?;
    // 0x668c0: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x668c4: bge 0x668ec <entry+0x110ec> @ imm = #0x20
    c.emit(arm32::branch(C::Ge,c.at as u32,0x668ec,false)?)?;
    // 0x668c8: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x668cc: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x668d0: ldr r0, [r0, #0x690]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1680)?)?;
    // 0x668d4: add r0, r8, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x668d8: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x668dc: bl 0x670a4 <entry+0x118a4> @ imm = #0x7c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x670a4,true)?)?;
    // 0x668e0: add r1, r7, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x668e4: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // 0x668e8: strb r0, [r1, #0x73]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,115)?)?;
    // 0x668ec: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x668f0: cmp r5, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x668f4: bne 0x668bc <entry+0x110bc> @ imm = #-0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x668bc,false)?)?;
    // 0x668f8: ldr r0, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,400)?)?;
    // 0x668fc: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x66900: beq 0x6698c <entry+0x1118c> @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6698c,false)?)?;
    // 0x66904: ldr r9, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,64)?)?;
    // 0x66908: mov r5, #189
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,189)?)?;
    // 0x6690c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x66910: orr r5, r5, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,512)?)?;
    // 0x66914: bne 0x66a80 <entry+0x11280> @ imm = #0x164
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66a80,false)?)?;
    // 0x66918: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x6691c: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // 0x66920: add r0, r0, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,648)?)?;
    // 0x66924: add r1, r0, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x66928: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x6692c: ldrsb r2, [r1, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,51)?)?;
    // 0x66930: add r2, r2, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66934: add r2, r0, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66938: ldr r2, [r2, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,36)?)?;
    // 0x6693c: str r2, [r6, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,432)?)?;
    // 0x66940: ldrsb r2, [r1, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,51)?)?;
    // 0x66944: add r2, r2, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66948: ldr r2, [r3, r2, lsl #3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(3)?,R::R2,Shift::Lsl,3)?)?;
    // 0x6694c: ldr r3, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,4)?)?;
    // 0x66950: str r2, [r6, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,640)?)?;
    // 0x66954: str r3, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,644)?)?;
    // 0x66958: blt 0x66a78 <entry+0x11278> @ imm = #0x118
    c.emit(arm32::branch(C::Lt,c.at as u32,0x66a78,false)?)?;
    // 0x6695c: ldrsb r2, [r1, #52]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,52)?)?;
    // 0x66960: add r2, r2, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66964: add r2, r0, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66968: ldr r2, [r2, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,36)?)?;
    // 0x6696c: str r2, [r6, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,436)?)?;
    // 0x66970: ldrsb r1, [r1, #52]
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R1,52)?)?;
    // 0x66974: add r1, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66978: ldr r1, [r0, r1, lsl #3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,3)?)?;
    // 0x6697c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x66980: str r1, [r6, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,648)?)?;
    // 0x66984: str r0, [r6, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,652)?)?;
    // 0x66988: b 0x66a80 <entry+0x11280> @ imm = #0xf0
    c.emit(arm32::branch(C::Al,c.at as u32,0x66a80,false)?)?;
    // 0x6698c: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x66990: mov r5, #189
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,189)?)?;
    // 0x66994: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x66998: orr r5, r5, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,512)?)?;
    // 0x6699c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x669a0: bne 0x669e8 <entry+0x111e8> @ imm = #0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x669e8,false)?)?;
    // 0x669a4: ldr r2, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,68)?)?;
    // 0x669a8: ldrb r1, [r9, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,3)?)?;
    // 0x669ac: add r0, r2, #187
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,187)?)?;
    // 0x669b0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x669b4: add r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,512)?)?;
    // 0x669b8: bne 0x669c8 <entry+0x111c8> @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x669c8,false)?)?;
    // 0x669bc: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x669c0: add r1, r2, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x669c4: b 0x669e0 <entry+0x111e0> @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x669e0,false)?)?;
    // 0x669c8: ldrb r1, [r2, #0x2bc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,700)?)?;
    // 0x669cc: strb r1, [r2, #0x2bd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,701)?)?;
    // 0x669d0: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x669d4: strb r0, [r2, #0x2be]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R2,702)?)?;
    // 0x669d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x669dc: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x669e0: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x669e4: ldr r2, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,400)?)?;
    // 0x669e8: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x669ec: ldr r9, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,64)?)?;
    // 0x669f0: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x669f4: add r7, r0, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,648)?)?;
    // 0x669f8: add r1, r7, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x669fc: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x66a00: ldrsb r3, [r1, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::R1,51)?)?;
    // 0x66a04: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x66a08: add r3, r7, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x66a0c: ldr r3, [r3, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,36)?)?;
    // 0x66a10: str r3, [r6, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,432)?)?;
    // 0x66a14: ldrsb r3, [r1, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::R1,51)?)?;
    // 0x66a18: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x66a1c: ldr r3, [r0, r3, lsl #3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,3)?)?;
    // 0x66a20: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x66a24: str r3, [r6, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,640)?)?;
    // 0x66a28: str r0, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,644)?)?;
    // 0x66a2c: bne 0x66a78 <entry+0x11278> @ imm = #0x44
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66a78,false)?)?;
    // 0x66a30: ldrsb r0, [r1, #53]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::R1,53)?)?;
    // 0x66a34: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x66a38: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66a3c: add r0, r7, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66a40: ldr r0, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,36)?)?;
    // 0x66a44: str r0, [r6, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,436)?)?;
    // 0x66a48: ldrsb r0, [r1, #53]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::R1,53)?)?;
    // 0x66a4c: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66a50: ldr r0, [r2, r0, lsl #3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R2,R::R0,Shift::Lsl,3)?)?;
    // 0x66a54: ldr r2, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,4)?)?;
    // 0x66a58: str r0, [r6, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,648)?)?;
    // 0x66a5c: str r2, [r6, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,652)?)?;
    // 0x66a60: ldrsb r0, [r1, #53]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::R1,53)?)?;
    // 0x66a64: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66a68: add r0, r7, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66a6c: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x66a70: str r0, [r6, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,668)?)?;
    // 0x66a74: b 0x66a80 <entry+0x11280> @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x66a80,false)?)?;
    // 0x66a78: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x66a7c: str r0, [r6, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,436)?)?;
    // 0x66a80: strb r10, [r6, #0x179]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(6)?,377)?)?;
    // 0x66a84: add r0, r5, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x66a88: lsl r1, r9, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(9)?,Shift::Lsl,3)?)?;
    // 0x66a8c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x66a90: ldr r10, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-52)?)?;
    // 0x66a94: add r7, r10, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x66a98: mvn r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(4)?,R::R0,0)?)?;
    // 0x66a9c: mvn r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,0)?)?;
    // 0x66aa0: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x66aa4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x66aa8: ldrsb r5, [r7, #-2]
    c.emit(arm32::load_signed_byte(C::Al,R::new(5)?,R::new(7)?,-2)?)?;
    // 0x66aac: cmn r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(5)?,1)?)?;
    // 0x66ab0: addne r6, r5, r5, lsl #3
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(6)?,R::new(5)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x66ab4: add r5, r10, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(10)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x66ab8: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // 0x66abc: addne r6, r10, r6, lsl #3
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(6)?,R::new(10)?,R::new(6)?,Shift::Lsl,3,false)?)?;
    // 0x66ac0: ldrne r4, [r6, #0x290]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(4)?,R::new(6)?,656)?)?;
    // 0x66ac4: ldrne r6, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(6)?,R::R11,-48)?)?;
    // 0x66ac8: str r4, [r5, #0x2c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,704)?)?;
    // 0x66acc: ldrsb r7, [r7]
    c.emit(arm32::load_signed_byte(C::Al,R::new(7)?,R::new(7)?,0)?)?;
    // 0x66ad0: cmn r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(7)?,1)?)?;
    // 0x66ad4: addne r3, r7, r7, lsl #3
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(3)?,R::new(7)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x66ad8: addne r3, r10, r3, lsl #3
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(3)?,R::new(10)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x66adc: ldrne r3, [r3, #0x290]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(3)?,656)?)?;
    // 0x66ae0: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x66ae4: str r3, [r5, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,712)?)?;
    // 0x66ae8: bne 0x66a94 <entry+0x11294> @ imm = #-0x5c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66a94,false)?)?;
    // 0x66aec: ldr r4, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-64)?)?;
    // 0x66af0: ldr r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,908)?)?;
    // 0x66af4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66af8: beq 0x66b40 <entry+0x11340> @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66b40,false)?)?;
    // 0x66afc: add r1, r10, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,648)?)?;
    // 0x66b00: add r0, r1, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x66b04: ldrsb r0, [r0, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::R0,51)?)?;
    // 0x66b08: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66b0c: add r1, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66b10: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,16)?)?;
    // 0x66b14: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x66b18: bne 0x66b40 <entry+0x11340> @ imm = #0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66b40,false)?)?;
    // 0x66b1c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x66b20: add r0, r10, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66b24: str r1, [r10, #0x67c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1660)?)?;
    // 0x66b28: ldr r0, [r0, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,680)?)?;
    // 0x66b2c: str r0, [r10, #0x690]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1680)?)?;
    // 0x66b30: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x66b34: ldr r2, [r6, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,972)?)?;
    // 0x66b38: orr r0, r2, r1, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::R2,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x66b3c: str r0, [r6, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,972)?)?;
    // 0x66b40: ldr r7, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,644)?)?;
    // 0x66b44: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x66b48: bne 0x66bd8 <entry+0x113d8> @ imm = #0x88
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66bd8,false)?)?;
    // 0x66b4c: ldr r0, [r10, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,584)?)?;
    // 0x66b50: add r1, r10, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,648)?)?;
    // 0x66b54: ldr r6, [r10, #0x660]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,1632)?)?;
    // 0x66b58: ldr r7, [r10, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,368)?)?;
    // 0x66b5c: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66b60: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66b64: ldrsb r2, [r0, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R0,51)?)?;
    // 0x66b68: add r2, r2, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66b6c: add r1, r1, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66b70: ldr r2, [r10, #0x67c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1660)?)?;
    // 0x66b74: ldr r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,8)?)?;
    // 0x66b78: cmp r6, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x66b7c: bne 0x66b90 <entry+0x11390> @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66b90,false)?)?;
    // 0x66b80: ldr r6, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-48)?)?;
    // 0x66b84: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x66b88: bne 0x66b9c <entry+0x1139c> @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66b9c,false)?)?;
    // 0x66b8c: b 0x66bcc <entry+0x113cc> @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x66bcc,false)?)?;
    // 0x66b90: ldr r6, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-48)?)?;
    // 0x66b94: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x66b98: beq 0x66fe0 <entry+0x117e0> @ imm = #0x440
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66fe0,false)?)?;
    // 0x66b9c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x66ba0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x66ba4: ldr r0, [r1, #0x144]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::R1,324)?)?;
    // 0x66ba8: add r1, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66bac: str r2, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,12)?)?;
    // 0x66bb0: ldr r1, [r10, #0x690]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1680)?)?;
    // 0x66bb4: add r2, r7, #324
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,324)?)?;
    // 0x66bb8: add r0, r2, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66bbc: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x66bc0: ldr r0, [r7, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,324)?)?;
    // 0x66bc4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x66bc8: str r0, [r7, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,324)?)?;
    // 0x66bcc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x66bd0: bl 0x617ec <entry+0xbfec>  @ imm = #-0x53ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x617ec,true)?)?;
    // 0x66bd4: ldr r7, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,644)?)?;
    // 0x66bd8: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x66bdc: bne 0x66ca0 <entry+0x114a0> @ imm = #0xbc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66ca0,false)?)?;
    // 0x66be0: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x66be4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x66be8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66bec: beq 0x66c58 <entry+0x11458> @ imm = #0x64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66c58,false)?)?;
    // 0x66bf0: add r4, r10, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(10)?,648)?)?;
    // 0x66bf4: ldr r1, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-64)?)?;
    // 0x66bf8: add r9, r4, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::new(4)?,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x66bfc: ldrsb r0, [r9, #53]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::new(9)?,53)?)?;
    // 0x66c00: ldrb r5, [r1, #0x996]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R1,2454)?)?;
    // 0x66c04: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66c08: add r0, r4, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66c0c: ldrb r0, [r0, #0x31]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,49)?)?;
    // 0x66c10: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x66c14: bl 0x81850 <entry+0x2c050> @ imm = #0x1ac34
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x66c18: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x66c1c: add r0, r0, #42
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,42)?)?;
    // 0x66c20: add r6, r0, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,1792)?)?;
    // 0x66c24: ldrsb r0, [r9, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::new(9)?,51)?)?;
    // 0x66c28: ldrb r10, [r6, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(10)?,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x66c2c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x66c30: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66c34: add r0, r4, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x66c38: ldr r4, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-64)?)?;
    // 0x66c3c: ldrb r0, [r0, #0x31]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,49)?)?;
    // 0x66c40: bl 0x81850 <entry+0x2c050> @ imm = #0x1ac08
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x66c44: ldrb r0, [r6, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x66c48: ldr r6, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-48)?)?;
    // 0x66c4c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x66c50: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x66c54: movhi r1, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x66c58: ldr r2, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,400)?)?;
    // 0x66c5c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x66c60: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x66c64: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x66c68: bne 0x66c7c <entry+0x1147c> @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66c7c,false)?)?;
    // 0x66c6c: ldrb r3, [r4, #0x468]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1128)?)?;
    // 0x66c70: cmp r3, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,10)?)?;
    // 0x66c74: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x66c78: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x66c7c: strb r3, [r6, #0x3e6]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(6)?,998)?)?;
    // 0x66c80: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x66c84: rsb r5, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(5)?,R::new(3)?,1)?)?;
    // 0x66c88: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x66c8c: ldr r10, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-52)?)?;
    // 0x66c90: ldr r9, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,64)?)?;
    // 0x66c94: moveq r2, r5
    c.emit(arm32::mov_register(C::Eq,R::R2,R::new(5)?))?;
    // 0x66c98: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x66c9c: b 0x66ce0 <entry+0x114e0> @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x66ce0,false)?)?;
    // 0x66ca0: ldr r1, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,400)?)?;
    // 0x66ca4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x66ca8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x66cac: strb r3, [r6, #0x3e6]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(6)?,998)?)?;
    // 0x66cb0: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x66cb4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x66cb8: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x66cbc: bne 0x66d14 <entry+0x11514> @ imm = #0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66d14,false)?)?;
    // 0x66cc0: ldr r6, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,924)?)?;
    // 0x66cc4: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x66cc8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x66ccc: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x66cd0: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x66cd4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x66cd8: ldr r6, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-48)?)?;
    // 0x66cdc: beq 0x66d14 <entry+0x11514> @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66d14,false)?)?;
    // 0x66ce0: add r1, r10, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,648)?)?;
    // 0x66ce4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66ce8: add r3, r1, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x66cec: add r2, r3, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,1,false)?)?;
    // 0x66cf0: ldrsb r2, [r2, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R2,51)?)?;
    // 0x66cf4: add r2, r2, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66cf8: add r1, r1, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66cfc: ldr r1, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,44)?)?;
    // 0x66d00: str r1, [r6, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,440)?)?;
    // 0x66d04: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x66d08: strb r1, [r6, #0x3e7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,999)?)?;
    // 0x66d0c: bne 0x66d24 <entry+0x11524> @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66d24,false)?)?;
    // 0x66d10: b 0x66d64 <entry+0x11564> @ imm = #0x4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x66d64,false)?)?;
    // 0x66d14: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x66d18: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x66d1c: strb r3, [r6, #0x3e7]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(6)?,999)?)?;
    // 0x66d20: bne 0x66f2c <entry+0x1172c> @ imm = #0x204
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66f2c,false)?)?;
    // 0x66d24: add r0, r10, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,648)?)?;
    // 0x66d28: add r1, r0, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x66d2c: ldrsb r2, [r1, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,51)?)?;
    // 0x66d30: add r2, r2, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66d34: add r2, r0, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R2,Shift::Lsl,3,false)?)?;
    // 0x66d38: ldr r3, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,8)?)?;
    // 0x66d3c: ldr r2, [r6, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,988)?)?;
    // 0x66d40: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x66d44: bhs 0x66d64 <entry+0x11564> @ imm = #0x18
    c.emit(arm32::branch(C::Cs,c.at as u32,0x66d64,false)?)?;
    // 0x66d48: ldrsb r1, [r1, #53]
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R1,53)?)?;
    // 0x66d4c: add r1, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66d50: add r0, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66d54: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x66d58: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x66d5c: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x66d60: strblo r0, [r6, #0x3e7]
    c.emit(arm32::transfer_byte(C::Cc,false,R::R0,R::new(6)?,999)?)?;
    // 0x66d64: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x66d68: bne 0x66f2c <entry+0x1172c> @ imm = #0x1bc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66f2c,false)?)?;
    // 0x66d6c: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x66d70: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x66d74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66d78: beq 0x66f2c <entry+0x1172c> @ imm = #0x1ac
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66f2c,false)?)?;
    // 0x66d7c: ldr r2, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,68)?)?;
    // 0x66d80: ldr r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // 0x66d84: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x66d88: ldr r4, [r2, #0x2c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R2,704)?)?;
    // 0x66d8c: ldr r7, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // 0x66d90: bne 0x66e60 <entry+0x11660> @ imm = #0xc8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66e60,false)?)?;
    // 0x66d94: ldr r5, [r2, #0x2c4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,708)?)?;
    // 0x66d98: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x66d9c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x66da0: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x66da4: cmn r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(5)?,1)?)?;
    // 0x66da8: moveq r5, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x66dac: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x66db0: bl 0x67378 <entry+0x11b78> @ imm = #0x5c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x67378,true)?)?;
    // 0x66db4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x66db8: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x66dbc: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x66dc0: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x66dc4: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x66dc8: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x66dcc: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x66dd0: moveq r0, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x66dd4: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x66dd8: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x66ddc: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x66de0: bl 0x67378 <entry+0x11b78> @ imm = #0x590
    c.emit(arm32::branch(C::Al,c.at as u32,0x67378,true)?)?;
    // 0x66de4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x66de8: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x66dec: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x66df0: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x66df4: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x66df8: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x66dfc: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x66e00: moveq r0, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x66e04: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x66e08: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x66e0c: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x66e10: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x66e14: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x66e18: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x66e1c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x66e20: ldr r5, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,60)?)?;
    // 0x66e24: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x66e28: bl 0x67404 <entry+0x11c04> @ imm = #0x5d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x67404,true)?)?;
    // 0x66e2c: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x66e30: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x66e34: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x66e38: ldr r2, [r0, #0x2c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,708)?)?;
    // 0x66e3c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x66e40: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x66e44: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x66e48: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x66e4c: cmn r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R2,1)?)?;
    // 0x66e50: moveq r2, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x66e54: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x66e58: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x66e5c: b 0x66f24 <entry+0x11724> @ imm = #0xc0
    c.emit(arm32::branch(C::Al,c.at as u32,0x66f24,false)?)?;
    // 0x66e60: ldr r9, [r2, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R2,712)?)?;
    // 0x66e64: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x66e68: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x66e6c: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x66e70: str r4, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-48)?)?;
    // 0x66e74: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // 0x66e78: bl 0x67378 <entry+0x11b78> @ imm = #0x4f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x67378,true)?)?;
    // 0x66e7c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x66e80: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x66e84: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x66e88: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x66e8c: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x66e90: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x66e94: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x66e98: moveq r0, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x66e9c: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x66ea0: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x66ea4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x66ea8: bl 0x67378 <entry+0x11b78> @ imm = #0x4c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x67378,true)?)?;
    // 0x66eac: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x66eb0: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x66eb4: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x66eb8: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x66ebc: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x66ec0: moveq r0, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x66ec4: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x66ec8: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x66ecc: add r0, r0, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x66ed0: ldr r9, [r0, #0x2c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R0,704)?)?;
    // 0x66ed4: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x66ed8: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x66edc: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x66ee0: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // 0x66ee4: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x66ee8: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x66eec: ldr r4, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,60)?)?;
    // 0x66ef0: ldr r2, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-48)?)?;
    // 0x66ef4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x66ef8: bl 0x67404 <entry+0x11c04> @ imm = #0x504
    c.emit(arm32::branch(C::Al,c.at as u32,0x67404,true)?)?;
    // 0x66efc: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x66f00: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // 0x66f04: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x66f08: ldr r2, [r0, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,712)?)?;
    // 0x66f0c: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x66f10: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x66f14: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x66f18: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x66f1c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x66f20: ldr r9, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,64)?)?;
    // 0x66f24: bl 0x67404 <entry+0x11c04> @ imm = #0x4d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x67404,true)?)?;
    // 0x66f28: ldr r4, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-64)?)?;
    // 0x66f2c: ldr r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // 0x66f30: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x66f34: bl 0x62c00 <entry+0xd400>  @ imm = #-0x433c
    c.emit(arm32::branch(C::Al,c.at as u32,0x62c00,true)?)?;
    // 0x66f38: ldr r0, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1392)?)?;
    // 0x66f3c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x66f40: ldrbeq r0, [r6, #0x17a]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(6)?,378)?)?;
    // 0x66f44: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66f48: beq 0x66fac <entry+0x117ac> @ imm = #0x5c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66fac,false)?)?;
    // 0x66f4c: ldr r0, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,400)?)?;
    // 0x66f50: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66f54: beq 0x66f84 <entry+0x11784> @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66f84,false)?)?;
    // 0x66f58: add r0, r10, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,648)?)?;
    // 0x66f5c: add r1, r0, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x66f60: ldrsb r1, [r1, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R1,51)?)?;
    // 0x66f64: add r1, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66f68: add r0, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66f6c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x66f70: ldrb r0, [r0, #0x28]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,40)?)?;
    // 0x66f74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66f78: ldr r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x66f7c: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x66f80: strb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x66f84: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,60)?)?;
    // 0x66f88: str r0, [r6, #0x634]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1588)?)?;
    // 0x66f8c: add r0, r10, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,648)?)?;
    // 0x66f90: add r1, r0, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x66f94: ldrsb r1, [r1, #51]
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R1,51)?)?;
    // 0x66f98: add r1, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66f9c: add r0, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x66fa0: str r0, [r6, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1592)?)?;
    // 0x66fa4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x66fa8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x66fac: ldr r0, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,76)?)?;
    // 0x66fb0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x66fb4: bne 0x66f4c <entry+0x1174c> @ imm = #-0x70
    c.emit(arm32::branch(C::Ne,c.at as u32,0x66f4c,false)?)?;
    // 0x66fb8: ldrb r0, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,10)?)?;
    // 0x66fbc: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x66fc0: blo 0x66f4c <entry+0x1174c> @ imm = #-0x7c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x66f4c,false)?)?;
    // 0x66fc4: ldr r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,908)?)?;
    // 0x66fc8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x66fcc: beq 0x66f4c <entry+0x1174c> @ imm = #-0x88
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66f4c,false)?)?;
    // 0x66fd0: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x66fd4: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x66fd8: bl 0x65e24 <entry+0x10624> @ imm = #-0x11bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x65e24,true)?)?;
    // 0x66fdc: b 0x66f4c <entry+0x1174c> @ imm = #-0x98
    c.emit(arm32::branch(C::Al,c.at as u32,0x66f4c,false)?)?;
    // 0x66fe0: ldr r2, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,80)?)?;
    // 0x66fe4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x66fe8: beq 0x66bcc <entry+0x113cc> @ imm = #-0x424
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66bcc,false)?)?;
    // 0x66fec: ldr r2, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,24)?)?;
    // 0x66ff0: ldr r1, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,24)?)?;
    // 0x66ff4: ldr r3, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,88)?)?;
    // 0x66ff8: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x66ffc: bls 0x67040 <entry+0x11840> @ imm = #0x3c
    c.emit(arm32::branch(C::Ls,c.at as u32,0x67040,false)?)?;
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
