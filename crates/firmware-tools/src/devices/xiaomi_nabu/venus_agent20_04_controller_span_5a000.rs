// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x5a000,0x5b000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x5a000;
pub const END:usize=0x5b000;
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
fn emit_umull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 UMULL operands".into())}
 Ok(((cond as u32)<<28)|0x0080_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
fn emit_smull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 SMULL operands".into())}
 Ok(((cond as u32)<<28)|0x00c0_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x5a000..0x5a648.
 c.check(0x5a000)?;
    // 0x5a000: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x5a004: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a008: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58860
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a00c: ldr r0, [r4, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1708)?)?;
    // 0x5a010: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a014: lsr r2, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsr,1)?)?;
    // 0x5a018: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a01c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24a48
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a020: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x5a024: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a028: str r0, [r4, #0x7b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1968)?)?;
    // 0x5a02c: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x5a030: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a034: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a038: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58890
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a03c: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x5a040: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x5a044: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a048: bne 0x5a630 <entry+0x4e30>  @ imm = #0x5e0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a630,false)?)?;
    // 0x5a04c: ldr r0, [r4, #0x764]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1892)?)?;
    // 0x5a050: ldr r1, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,664)?)?;
    // 0x5a054: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x5a058: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a05c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a060: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24a8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a064: str r0, [r4, #0x7bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1980)?)?;
    // 0x5a068: ldr r0, [r4, #0x768]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1896)?)?;
    // 0x5a06c: ldr r1, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,664)?)?;
    // 0x5a070: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x5a074: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a078: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a07c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24aa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a080: str r0, [r4, #0x7c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1984)?)?;
    // 0x5a084: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a088: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a08c: ldr r2, [r4, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1756)?)?;
    // 0x5a090: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24abc
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a094: str r0, [r4, #0x7b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1972)?)?;
    // 0x5a098: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a09c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a0a0: ldr r2, [r4, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1760)?)?;
    // 0x5a0a4: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24ad0
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a0a8: str r0, [r4, #0x7b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1976)?)?;
    // 0x5a0ac: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a0b0: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a0b4: ldr r2, [r4, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1752)?)?;
    // 0x5a0b8: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24ae4
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a0bc: str r0, [r4, #0x7c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1988)?)?;
    // 0x5a0c0: add r1, r4, #748
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,748)?)?;
    // 0x5a0c4: add r0, r4, #804
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,804)?)?;
    // 0x5a0c8: ldr r2, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2744)?)?;
    // 0x5a0cc: add r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // 0x5a0d0: ldrb r2, [r2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x5a0d4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5a0d8: addne r1, r0, #1024
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R0,1024)?)?;
    // 0x5a0dc: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a0e0: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // 0x5a0e4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a0e8: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24b14
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a0ec: str r0, [r4, #0x8d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2264)?)?;
    // 0x5a0f0: ldrb r0, [r4, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // 0x5a0f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a0f8: beq 0x5a120 <entry+0x4920>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a120,false)?)?;
    // 0x5a0fc: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x5a100: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x5a104: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5a108: bne 0x5a120 <entry+0x4920>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a120,false)?)?;
    // 0x5a10c: ldr r2, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1776)?)?;
    // 0x5a110: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a114: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a118: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24b44
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a11c: str r0, [r4, #0x8dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2268)?)?;
    // 0x5a120: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a124: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a128: mov r2, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20480)?)?;
    // 0x5a12c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24b58
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a130: str r0, [r4, #0x968]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2408)?)?;
    // 0x5a134: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a138: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x5a13c: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // 0x5a140: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24b6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a144: str r0, [r4, #0x960]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2400)?)?;
    // 0x5a148: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a14c: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x5a150: mov r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4096)?)?;
    // 0x5a154: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24b80
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a158: str r0, [r4, #0x964]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2404)?)?;
    // 0x5a15c: mov r9, r4
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(4)?))?;
    // 0x5a160: ldrb r0, [r4, #0x80a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2058)?)?;
    // 0x5a164: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a168: beq 0x5a234 <entry+0x4a34>  @ imm = #0xc4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a234,false)?)?;
    // 0x5a16c: ldr r4, [r9, #0x684]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,1668)?)?;
    // 0x5a170: ldr r6, [pc, #0x4d0]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,1232)?)?;
    // 0x5a174: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // 0x5a178: add r0, r9, #984
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,984)?)?;
    // 0x5a17c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5a180: add r7, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,1024)?)?;
    // 0x5a184: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x5a188: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x5a18c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a190: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a194: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x5a198: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24bc4
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a19c: mov r1, #103
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,103)?)?;
    // 0x5a1a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a1a4: str r0, [r7, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x5a1a8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a1ac: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a1b0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a1b4: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58a0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a1b8: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5a1bc: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x5a1c0: ldrsb r0, [r9, r1]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::R0,R::new(9)?,R::R1)?)?;
    // 0x5a1c4: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5a1c8: blo 0x5a188 <entry+0x4988>  @ imm = #-0x48
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5a188,false)?)?;
    // 0x5a1cc: ands r0, r0, #255
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,255,true)?)?;
    // 0x5a1d0: beq 0x5a234 <entry+0x4a34>  @ imm = #0x5c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a234,false)?)?;
    // 0x5a1d4: ldr r5, [r9, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,1672)?)?;
    // 0x5a1d8: ldr r7, [pc, #0x468]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::PC,1128)?)?;
    // 0x5a1dc: add r4, r9, #2032
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,2032)?)?;
    // 0x5a1e0: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5a1e4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a1e8: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a1ec: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x5a1f0: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24c1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a1f4: mov r1, #114
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,114)?)?;
    // 0x5a1f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a1fc: str r0, [r4, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x5a200: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x5a204: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a208: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a20c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58a64
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a210: ldr r0, [r4, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x5a214: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5a218: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x5a21c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x57038
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x5a220: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x5a224: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x5a228: ldrsb r0, [r9, r0]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::R0,R::new(9)?,R::R0)?)?;
    // 0x5a22c: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5a230: blo 0x5a1e4 <entry+0x49e4>  @ imm = #-0x54
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5a1e4,false)?)?;
    // 0x5a234: ldr r2, [r9, #0x68c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1676)?)?;
    // 0x5a238: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a23c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a240: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24c6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a244: ldr r6, [pc, #0x3fc]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,1020)?)?;
    // 0x5a248: mov r1, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,124)?)?;
    // 0x5a24c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a250: str r0, [r9, #0x80c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,2060)?)?;
    // 0x5a254: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a258: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a25c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a260: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58ab8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a264: ldr r2, [r9, #0x690]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1680)?)?;
    // 0x5a268: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a26c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a270: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24c9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a274: mov r1, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,132)?)?;
    // 0x5a278: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a27c: str r0, [r9, #0x810]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,2064)?)?;
    // 0x5a280: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a284: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a288: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a28c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58ae4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a290: ldr r2, [r9, #0x694]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1684)?)?;
    // 0x5a294: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a298: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a29c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24cc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a2a0: mov r1, #140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,140)?)?;
    // 0x5a2a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a2a8: str r0, [r9, #0x7c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1992)?)?;
    // 0x5a2ac: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a2b0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a2b4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a2b8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58b10
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a2bc: ldr r0, [r9, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,644)?)?;
    // 0x5a2c0: mov r10, r9
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // 0x5a2c4: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x5a2c8: bne 0x5a2f8 <entry+0x4af8>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a2f8,false)?)?;
    // 0x5a2cc: ldr r2, [r10, #0xb80]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2944)?)?;
    // 0x5a2d0: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x5a2d4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a2d8: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24d04
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a2dc: ldr r2, [pc, #0x364]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,868)?)?;
    // 0x5a2e0: mov r1, #179
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,179)?)?;
    // 0x5a2e4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a2e8: str r0, [r10, #0x954]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,2388)?)?;
    // 0x5a2ec: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a2f0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a2f4: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58b4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a2f8: ldrb r0, [r10, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,3104)?)?;
    // 0x5a2fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a300: beq 0x5a484 <entry+0x4c84>  @ imm = #0x17c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a484,false)?)?;
    // 0x5a304: ldr r9, [pc, #0x33c]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::PC,828)?)?;
    // 0x5a308: mov r1, #190
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,190)?)?;
    // 0x5a30c: add r0, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,12)?)?;
    // 0x5a310: add r8, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,24)?)?;
    // 0x5a314: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a318: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a31c: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x5a320: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58b78
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a324: mov r5, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,716)?)?;
    // 0x5a328: mov r6, #194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,194)?)?;
    // 0x5a32c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5a330: add r4, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::SP,44)?)?;
    // 0x5a334: add r10, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,12)?)?;
    // 0x5a338: orr r5, r5, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,15360)?)?;
    // 0x5a33c: orr r6, r6, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1024)?)?;
    // 0x5a340: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a344: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a348: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x5a34c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24d78
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a350: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a354: str r0, [r4], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x5a358: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x5a35c: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x5a360: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a364: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58bbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a368: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x5a36c: cmp r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4)?)?;
    // 0x5a370: bne 0x5a340 <entry+0x4b40>  @ imm = #-0x38
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a340,false)?)?;
    // 0x5a374: ldr r10, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,8)?)?;
    // 0x5a378: mov r2, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,716)?)?;
    // 0x5a37c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5a380: add r3, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,44)?)?;
    // 0x5a384: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x5a388: orr r2, r2, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,15360)?)?;
    // 0x5a38c: ldr r0, [r10, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,140)?)?;
    // 0x5a390: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x5a394: bl 0x7f118 <entry+0x29918> @ imm = #0x24d7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x7f118,true)?)?;
    // 0x5a398: ldrb r0, [r10, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,3104)?)?;
    // 0x5a39c: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x5a3a0: beq 0x5a3ec <entry+0x4bec>  @ imm = #0x44
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a3ec,false)?)?;
    // 0x5a3a4: ldr r4, [r10, #0xc24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,3108)?)?;
    // 0x5a3a8: ldr r6, [pc, #0x298]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,664)?)?;
    // 0x5a3ac: mov r9, #207
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,207)?)?;
    // 0x5a3b0: add r5, r10, #3120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(10)?,3120)?)?;
    // 0x5a3b4: orr r9, r9, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,1024)?)?;
    // 0x5a3b8: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x5a3bc: mov r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4096)?)?;
    // 0x5a3c0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x5a3c4: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24df0
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a3c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a3cc: str r0, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::new(5)?,4)?)?;
    // 0x5a3d0: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x5a3d4: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a3d8: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a3dc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58c34
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a3e0: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x5a3e4: cmp r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,5)?)?;
    // 0x5a3e8: bne 0x5a3b8 <entry+0x4bb8>  @ imm = #-0x38
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a3b8,false)?)?;
    // 0x5a3ec: ldr r6, [pc, #0x254]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,596)?)?;
    // 0x5a3f0: ldr r4, [r10, #0xc28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,3112)?)?;
    // 0x5a3f4: mov r1, #217
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,217)?)?;
    // 0x5a3f8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a3fc: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a400: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a404: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58c5c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a408: ldr r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,4)?)?;
    // 0x5a40c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a410: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x5a414: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a418: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24e44
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a41c: mov r1, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,221)?)?;
    // 0x5a420: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a424: str r0, [r10, #0xc44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,3140)?)?;
    // 0x5a428: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a42c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a430: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a434: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58c8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a438: ldr r4, [r10, #0xc2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,3116)?)?;
    // 0x5a43c: mov r1, #227
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,227)?)?;
    // 0x5a440: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a444: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a448: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a44c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58ca4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a450: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a454: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a458: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x5a45c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24e88
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a460: mov r1, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,230)?)?;
    // 0x5a464: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a468: str r0, [r10, #0xc48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,3144)?)?;
    // 0x5a46c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5a470: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a474: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a478: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58cd0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a47c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5a480: bl 0x52954 <entry+0xffffffffffffd154> @ imm = #-0x7b34
    c.emit(arm32::branch(C::Al,c.at as u32,0x52954,true)?)?;
    // 0x5a484: ldr r0, [r10, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1256)?)?;
    // 0x5a488: ldr r8, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,4)?)?;
    // 0x5a48c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5a490: ldrbeq r0, [r10, #0x484]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(10)?,1156)?)?;
    // 0x5a494: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a498: beq 0x5a638 <entry+0x4e38>  @ imm = #0x198
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a638,false)?)?;
    // 0x5a49c: ldrb r7, [r10, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,991)?)?;
    // 0x5a4a0: ldr r5, [r10, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,1656)?)?;
    // 0x5a4a4: ldr r9, [pc, #0x19c]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::PC,412)?)?;
    // 0x5a4a8: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5a4ac: mov r4, #252
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,252)?)?;
    // 0x5a4b0: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x5a4b4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a4b8: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x5a4bc: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24ee8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a4c0: ldr r1, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,44)?)?;
    // 0x5a4c4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a4c8: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x5a4cc: str r0, [r1, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x5a4d0: mov r1, #247
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,247)?)?;
    // 0x5a4d4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a4d8: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x5a4dc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58d34
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a4e0: add r0, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // 0x5a4e4: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x5a4e8: add r4, r4, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,28)?)?;
    // 0x5a4ec: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x5a4f0: bne 0x5a4b0 <entry+0x4cb0>  @ imm = #-0x48
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a4b0,false)?)?;
    // 0x5a4f4: ldr r0, [r10, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2668)?)?;
    // 0x5a4f8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x5a4fc: lsl r2, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsl,1)?)?;
    // 0x5a500: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x5a504: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24f30
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a508: ldr r2, [pc, #0x138]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,312)?)?;
    // 0x5a50c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a510: str r0, [r10, #0x538]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1336)?)?;
    // 0x5a514: mov r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1280)?)?;
    // 0x5a518: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a51c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58d74
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a520: ldr r1, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,44)?)?;
    // 0x5a524: ldr r0, [r10, #0x538]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1336)?)?;
    // 0x5a528: mov r9, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,13)?)?;
    // 0x5a52c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x5a530: mov r6, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,44)?)?;
    // 0x5a534: orr r9, r9, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,1280)?)?;
    // 0x5a538: ldr r1, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,40)?)?;
    // 0x5a53c: str r0, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,72)?)?;
    // 0x5a540: ldr r0, [r10, #0x4ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1260)?)?;
    // 0x5a544: ldrb r7, [r10, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,991)?)?;
    // 0x5a548: lsl r5, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R0,Shift::Lsl,1)?)?;
    // 0x5a54c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x5a550: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x5a554: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x5a558: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24f84
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a55c: ldr r1, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,44)?)?;
    // 0x5a560: ldr r2, [pc, #0xe0]         @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,224)?)?;
    // 0x5a564: ldr r1, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,40)?)?;
    // 0x5a568: str r0, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x5a56c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x5a570: ldr r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,44)?)?;
    // 0x5a574: ldr r0, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,40)?)?;
    // 0x5a578: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x5a57c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a580: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a584: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58ddc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a588: add r0, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1)?)?;
    // 0x5a58c: cmp r4, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x5a590: add r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // 0x5a594: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5a598: bne 0x5a54c <entry+0x4d4c>  @ imm = #-0x54
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a54c,false)?)?;
    // 0x5a59c: ldr r4, [r10, #0xd04]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,3332)?)?;
    // 0x5a5a0: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x5a5a4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a5a8: mov r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256)?)?;
    // 0x5a5ac: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24fd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a5b0: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x5a5b4: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x5a5b8: beq 0x5a614 <entry+0x4e14>  @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a614,false)?)?;
    // 0x5a5bc: ldr r9, [pc, #0x84]         @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::PC,132)?)?;
    // 0x5a5c0: mov r8, #27
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,27)?)?;
    // 0x5a5c4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5a5c8: mov r6, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,76)?)?;
    // 0x5a5cc: orr r8, r8, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,1280)?)?;
    // 0x5a5d0: ldr r1, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,44)?)?;
    // 0x5a5d4: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x5a5d8: str r0, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x5a5dc: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5a5e0: ldr r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,44)?)?;
    // 0x5a5e4: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x5a5e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a5ec: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a5f0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58e48
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a5f4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a5f8: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5a5fc: mov r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256)?)?;
    // 0x5a600: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x2502c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5a604: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x5a608: add r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // 0x5a60c: cmp r7, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x5a610: bne 0x5a5d0 <entry+0x4dd0>  @ imm = #-0x48
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a5d0,false)?)?;
    // 0x5a614: ldr r2, [pc, #0x2c]         @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,44)?)?;
    // 0x5a618: mov r1, #35
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,35)?)?;
    // 0x5a61c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a620: str r0, [r10, #0x81c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,2076)?)?;
    // 0x5a624: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5a628: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x5a62c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58e84
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a630: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x5a634: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5a638: ldrb r0, [r10, #0x401]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1025)?)?;
    // 0x5a63c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a640: bne 0x5a49c <entry+0x4c9c>  @ imm = #-0x1ac
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a49c,false)?)?;
    // 0x5a644: b 0x5a59c <entry+0x4d9c>  @ imm = #-0xb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5a59c,false)?)?;
 c.check(0x5a648)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5A648:[u32;1]=[0x000ee470];
 for value in INLINE_DATA_5A648{c.emit(value)?;}
 // Executable A32 block 0x5a64c..0x5a708.
 c.check(0x5a64c)?;
    // 0x5a64c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5a650: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x5a654: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x5a658: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5a65c: mov r6, r3
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // 0x5a660: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x5a664: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x5a668: orr r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // 0x5a66c: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x57778
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x5a670: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x5a674: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x5a678: beq 0x5a6e8 <entry+0x4ee8>  @ imm = #0x68
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a6e8,false)?)?;
    // 0x5a67c: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x5a680: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a684: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5a688: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x5a68c: orr r2, r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x5a690: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x574ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x5a694: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a698: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x5a69c: bl 0x58c38 <entry+0x3438>  @ imm = #-0x1a6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x58c38,true)?)?;
    // 0x5a6a0: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // 0x5a6a4: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x5a6a8: str r4, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,16)?)?;
    // 0x5a6ac: str r7, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,20)?)?;
    // 0x5a6b0: str r8, [r5, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,276)?)?;
    // 0x5a6b4: str r6, [r5, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,280)?)?;
    // 0x5a6b8: str r6, [r5, #0x570]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,1392)?)?;
    // 0x5a6bc: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x577c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x5a6c0: ldr r1, [pc, #0x40]         @ 0x4f08 <entry+0x4f08>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,64)?)?;
    // 0x5a6c4: str r0, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,40)?)?;
    // 0x5a6c8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a6cc: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x5a6d0: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x5a6d4: str r6, [r5, #0xcb4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,3252)?)?;
    // 0x5a6d8: bl 0x804fc <entry+0x2acfc> @ imm = #0x25e1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x804fc,true)?)?;
    // 0x5a6dc: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x5a6e0: str r8, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(8)?,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5a6e4: b 0x5a700 <entry+0x4f00>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x5a700,false)?)?;
    // 0x5a6e8: ldr r2, [pc, #0x1c]         @ 0x4f0c <entry+0x4f0c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,28)?)?;
    // 0x5a6ec: mov r1, #125
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,125)?)?;
    // 0x5a6f0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a6f4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5a6f8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x5a6fc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58f54
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5a700: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x5a704: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x5a708)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5A708:[u32;2]=[0x00105964,0x000ee470];
 for value in INLINE_DATA_5A708{c.emit(value)?;}
 // Executable A32 block 0x5a710..0x5a964.
 c.check(0x5a710)?;
    // 0x5a710: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5a714: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x5a718: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5a71c: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x5a720: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5a724: bne 0x5a744 <entry+0x4f44>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a744,false)?)?;
    // 0x5a728: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x5a72c: ldr r1, [pc, #0x230]        @ 0x5164 <entry+0x5164>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,560)?)?;
    // 0x5a730: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5a734: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x5a738: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x5a73c: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x5a740: bl 0x2674 <entry+0xffffffffffface74> @ imm = #-0x580d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2674,true)?)?;
    // 0x5a744: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5a748: bl 0x7389c <entry+0x1e09c> @ imm = #0x1914c
    c.emit(arm32::branch(C::Al,c.at as u32,0x7389c,true)?)?;
    // 0x5a74c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a750: str r0, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,84)?)?;
    // 0x5a754: str r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,88)?)?;
    // 0x5a758: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x5a75c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a760: beq 0x5a7a0 <entry+0x4fa0>  @ imm = #0x38
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a7a0,false)?)?;
    // 0x5a764: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x5a768: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5a76c: bne 0x5a78c <entry+0x4f8c>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a78c,false)?)?;
    // 0x5a770: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x5a774: ldr r1, [pc, #0x1e8]        @ 0x5164 <entry+0x5164>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,488)?)?;
    // 0x5a778: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5a77c: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x5a780: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x5a784: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x5a788: bl 0x2674 <entry+0xffffffffffface74> @ imm = #-0x5811c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2674,true)?)?;
    // 0x5a78c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5a790: bl 0x76138 <entry+0x20938> @ imm = #0x1b9a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x76138,true)?)?;
    // 0x5a794: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a798: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // 0x5a79c: str r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // 0x5a7a0: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x5a7a4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5a7a8: bne 0x5a7c8 <entry+0x4fc8>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a7c8,false)?)?;
    // 0x5a7ac: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x5a7b0: ldr r1, [pc, #0x1ac]        @ 0x5164 <entry+0x5164>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,428)?)?;
    // 0x5a7b4: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5a7b8: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x5a7bc: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x5a7c0: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x5a7c4: bl 0x2674 <entry+0xffffffffffface74> @ imm = #-0x58158
    c.emit(arm32::branch(C::Al,c.at as u32,0x2674,true)?)?;
    // 0x5a7c8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5a7cc: bl 0x74b10 <entry+0x1f310> @ imm = #0x1a33c
    c.emit(arm32::branch(C::Al,c.at as u32,0x74b10,true)?)?;
    // 0x5a7d0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5a7d4: str r5, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,68)?)?;
    // 0x5a7d8: str r5, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,72)?)?;
    // 0x5a7dc: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x5a7e0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5a7e4: bne 0x5a804 <entry+0x5004>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a804,false)?)?;
    // 0x5a7e8: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x5a7ec: ldr r1, [pc, #0x170]        @ 0x5164 <entry+0x5164>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,368)?)?;
    // 0x5a7f0: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5a7f4: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x5a7f8: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x5a7fc: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // 0x5a800: bl 0x2674 <entry+0xffffffffffface74> @ imm = #-0x58194
    c.emit(arm32::branch(C::Al,c.at as u32,0x2674,true)?)?;
    // 0x5a804: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5a808: bl 0x79214 <entry+0x23a14> @ imm = #0x1ea04
    c.emit(arm32::branch(C::Al,c.at as u32,0x79214,true)?)?;
    // 0x5a80c: str r5, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,60)?)?;
    // 0x5a810: str r5, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,64)?)?;
    // 0x5a814: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x5a818: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5a81c: bne 0x5a838 <entry+0x5038>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a838,false)?)?;
    // 0x5a820: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x5a824: ldr r1, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,24)?)?;
    // 0x5a828: bl 0x2808 <entry+0xfffffffffffad008> @ imm = #-0x58028
    c.emit(arm32::branch(C::Al,c.at as u32,0x2808,true)?)?;
    // 0x5a82c: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x5a830: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5a834: bl 0x2f940 <entry+0xfffffffffffda140> @ imm = #-0x2aefc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f940,true)?)?;
    // 0x5a838: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x5a83c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a840: beq 0x5a850 <entry+0x5050>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a850,false)?)?;
    // 0x5a844: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x57930
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5a848: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a84c: str r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,36)?)?;
    // 0x5a850: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x5a854: ldr r0, [r5, #0x88]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,136)?)?;
    // 0x5a858: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a85c: beq 0x5a870 <entry+0x5070>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a870,false)?)?;
    // 0x5a860: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a864: bl 0x7e7d4 <entry+0x28fd4> @ imm = #0x23f68
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e7d4,true)?)?;
    // 0x5a868: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a86c: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x5a870: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x5a874: ldr r0, [r5, #0x84]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,132)?)?;
    // 0x5a878: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a87c: beq 0x5a890 <entry+0x5090>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a890,false)?)?;
    // 0x5a880: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a884: bl 0x7e7d4 <entry+0x28fd4> @ imm = #0x23f48
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e7d4,true)?)?;
    // 0x5a888: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a88c: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x5a890: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x5a894: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a898: beq 0x5a8a8 <entry+0x50a8>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a8a8,false)?)?;
    // 0x5a89c: bl 0x2bcc <entry+0xfffffffffffad3cc> @ imm = #-0x57cd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bcc,true)?)?;
    // 0x5a8a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a8a4: str r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // 0x5a8a8: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // 0x5a8ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a8b0: beq 0x5a8ec <entry+0x50ec>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a8ec,false)?)?;
    // 0x5a8b4: ldr r1, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,96)?)?;
    // 0x5a8b8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5a8bc: beq 0x5a8e0 <entry+0x50e0>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a8e0,false)?)?;
    // 0x5a8c0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5a8c4: ldr r0, [r0, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x5a8c8: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x579b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5a8cc: ldr r1, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,96)?)?;
    // 0x5a8d0: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // 0x5a8d4: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x5a8d8: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5a8dc: blo 0x5a8c4 <entry+0x50c4>  @ imm = #-0x20
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5a8c4,false)?)?;
    // 0x5a8e0: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x579cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5a8e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a8e8: str r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,92)?)?;
    // 0x5a8ec: ldr r0, [r4, #0xb1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2844)?)?;
    // 0x5a8f0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a8f4: beq 0x5a904 <entry+0x5104>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a904,false)?)?;
    // 0x5a8f8: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x579e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5a8fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a900: str r0, [r4, #0xb1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2844)?)?;
    // 0x5a904: ldr r0, [r4, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1256)?)?;
    // 0x5a908: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x5a90c: bne 0x5a920 <entry+0x5120>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5a920,false)?)?;
    // 0x5a910: ldr r0, [r4, #0xb14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2836)?)?;
    // 0x5a914: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x57a00
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5a918: ldr r0, [r4, #0xb10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2832)?)?;
    // 0x5a91c: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x57a08
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5a920: ldr r0, [r4, #0xb18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2840)?)?;
    // 0x5a924: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a928: beq 0x5a938 <entry+0x5138>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a938,false)?)?;
    // 0x5a92c: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x57a18
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5a930: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a934: str r0, [r4, #0xb18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2840)?)?;
    // 0x5a938: ldr r0, [r4, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2508)?)?;
    // 0x5a93c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a940: beq 0x5a950 <entry+0x5150>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a950,false)?)?;
    // 0x5a944: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x57a30
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5a948: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a94c: str r0, [r4, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2508)?)?;
    // 0x5a950: ldr r0, [r4, #0x81c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2076)?)?;
    // 0x5a954: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a958: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a95c: strne r0, [r4, #0x81c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,2076)?)?;
    // 0x5a960: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x5a964)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5A964:[u32;1]=[0x0010ceac];
 for value in INLINE_DATA_5A964{c.emit(value)?;}
 // Executable A32 block 0x5a968..0x5ab40.
 c.check(0x5a968)?;
    // 0x5a968: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5a96c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x5a970: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5a974: ldr r0, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,292)?)?;
    // 0x5a978: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a97c: beq 0x5a98c <entry+0x518c>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a98c,false)?)?;
    // 0x5a980: bl 0x2bcc <entry+0xfffffffffffad3cc> @ imm = #-0x57dbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bcc,true)?)?;
    // 0x5a984: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a988: str r0, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,292)?)?;
    // 0x5a98c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5a990: ldr r1, [r0, #0x7c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,124)?)?;
    // 0x5a994: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5a998: beq 0x5a9a8 <entry+0x51a8>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a9a8,false)?)?;
    // 0x5a99c: bl 0x7e7d4 <entry+0x28fd4> @ imm = #0x23e30
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e7d4,true)?)?;
    // 0x5a9a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a9a4: str r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // 0x5a9a8: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x5a9ac: ldr r0, [r5, #0x8c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,140)?)?;
    // 0x5a9b0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a9b4: beq 0x5a9c8 <entry+0x51c8>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a9c8,false)?)?;
    // 0x5a9b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5a9bc: bl 0x7e7d4 <entry+0x28fd4> @ imm = #0x23e10
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e7d4,true)?)?;
    // 0x5a9c0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a9c4: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x5a9c8: ldr r0, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,240)?)?;
    // 0x5a9cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a9d0: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a9d4: strne r0, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,240)?)?;
    // 0x5a9d8: strne r0, [r4, #0xf4]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,244)?)?;
    // 0x5a9dc: ldr r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // 0x5a9e0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a9e4: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a9e8: strne r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,248)?)?;
    // 0x5a9ec: strne r0, [r4, #0xfc]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,252)?)?;
    // 0x5a9f0: ldr r0, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,256)?)?;
    // 0x5a9f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5a9f8: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5a9fc: strne r0, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,256)?)?;
    // 0x5aa00: strne r0, [r4, #0x104]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,260)?)?;
    // 0x5aa04: ldr r0, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,264)?)?;
    // 0x5aa08: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5aa0c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5aa10: strne r0, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,264)?)?;
    // 0x5aa14: strne r0, [r4, #0x10c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,268)?)?;
    // 0x5aa18: ldr r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x5aa1c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5aa20: beq 0x5aa30 <entry+0x5230>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5aa30,false)?)?;
    // 0x5aa24: bl 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x57b10
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,true)?)?;
    // 0x5aa28: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5aa2c: str r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // 0x5aa30: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x5aa34: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5aa38: bne 0x5aa58 <entry+0x5258>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5aa58,false)?)?;
    // 0x5aa3c: mov r0, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1004)?)?;
    // 0x5aa40: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x5aa44: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x5aa48: ldr r0, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5aa4c: bl 0x2bcc <entry+0xfffffffffffad3cc> @ imm = #-0x57e88
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bcc,true)?)?;
    // 0x5aa50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5aa54: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x5aa58: add r0, r4, #492
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,492)?)?;
    // 0x5aa5c: bl 0x35bec <entry+0xfffffffffffe03ec> @ imm = #-0x24e78
    c.emit(arm32::branch(C::Al,c.at as u32,0x35bec,true)?)?;
    // 0x5aa60: add r0, r4, #456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,456)?)?;
    // 0x5aa64: bl 0x35bec <entry+0xfffffffffffe03ec> @ imm = #-0x24e80
    c.emit(arm32::branch(C::Al,c.at as u32,0x35bec,true)?)?;
    // 0x5aa68: add r0, r4, #528
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,528)?)?;
    // 0x5aa6c: bl 0x35bec <entry+0xfffffffffffe03ec> @ imm = #-0x24e88
    c.emit(arm32::branch(C::Al,c.at as u32,0x35bec,true)?)?;
    // 0x5aa70: add r0, r4, #564
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,564)?)?;
    // 0x5aa74: bl 0x35bec <entry+0xfffffffffffe03ec> @ imm = #-0x24e90
    c.emit(arm32::branch(C::Al,c.at as u32,0x35bec,true)?)?;
    // 0x5aa78: add r0, r4, #600
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,600)?)?;
    // 0x5aa7c: bl 0x35bec <entry+0xfffffffffffe03ec> @ imm = #-0x24e98
    c.emit(arm32::branch(C::Al,c.at as u32,0x35bec,true)?)?;
    // 0x5aa80: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5aa84: pop {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5aa88: b 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x57b74
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,false)?)?;
    // 0x5aa8c: ldr r1, [r0, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,240)?)?;
    // 0x5aa90: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5aa94: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5aa98: strne r1, [r0, #0xf0]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,240)?)?;
    // 0x5aa9c: strne r1, [r0, #0xf4]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,244)?)?;
    // 0x5aaa0: ldr r1, [r0, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // 0x5aaa4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5aaa8: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5aaac: strne r1, [r0, #0xf8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,248)?)?;
    // 0x5aab0: strne r1, [r0, #0xfc]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,252)?)?;
    // 0x5aab4: ldr r1, [r0, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,256)?)?;
    // 0x5aab8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5aabc: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5aac0: strne r1, [r0, #0x100]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,256)?)?;
    // 0x5aac4: strne r1, [r0, #0x104]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,260)?)?;
    // 0x5aac8: ldr r1, [r0, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,264)?)?;
    // 0x5aacc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5aad0: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5aad4: strne r1, [r0, #0x108]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,264)?)?;
    // 0x5aad8: strne r1, [r0, #0x10c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,268)?)?;
    // 0x5aadc: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x5aae0: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x5aae4: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x5aae8: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x5aaec: ldr r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,20)?)?;
    // 0x5aaf0: ldr r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,32)?)?;
    // 0x5aaf4: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x5aaf8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5aafc: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x5ab00: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5ab04: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5ab08: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5ab0c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5ab10: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x30188
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5ab14: ldr r1, [pc, #0x24]         @ 0x5340 <entry+0x5340>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,36)?)?;
    // 0x5ab18: ldr r2, [pc, #0x24]         @ 0x5344 <entry+0x5344>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,36)?)?;
    // 0x5ab1c: mov r3, #142
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,142)?)?;
    // 0x5ab20: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ab24: orr r3, r3, #5376
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5376)?)?;
    // 0x5ab28: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2be9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ab2c: ldr r1, [pc, #0x14]         @ 0x5348 <entry+0x5348>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,20)?)?;
    // 0x5ab30: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ab34: mov sp, r11
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // 0x5ab38: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5ab3c: b 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2beb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,false)?)?;
 c.check(0x5ab40)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5AB40:[u32;3]=[0x000ee759,0x000ee781,0x000e2788];
 for value in INLINE_DATA_5AB40{c.emit(value)?;}
 // Executable A32 block 0x5ab4c..0x5ac28.
 c.check(0x5ab4c)?;
    // 0x5ab4c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5ab50: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x5ab54: sub sp, sp, #84
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,84)?)?;
    // 0x5ab58: add r8, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,4)?)?;
    // 0x5ab5c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5ab60: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x5ab64: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5ab68: add r9, r8, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(8)?,56)?)?;
    // 0x5ab6c: add r5, r8, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(8)?,68)?)?;
    // 0x5ab70: add r6, r8, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(8)?,64)?)?;
    // 0x5ab74: ldr r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,124)?)?;
    // 0x5ab78: sub r1, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // 0x5ab7c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5ab80: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x5ab84: bl 0x7f990 <entry+0x2a190> @ imm = #0x24e04
    c.emit(arm32::branch(C::Al,c.at as u32,0x7f990,true)?)?;
    // 0x5ab88: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x5ab8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5ab90: beq 0x5ac1c <entry+0x541c>  @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5ac1c,false)?)?;
    // 0x5ab94: ldr r0, [pc, #0x8c]         @ 0x5428 <entry+0x5428>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,140)?)?;
    // 0x5ab98: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x5ab9c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x5aba0: str r10, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,24)?)?;
    // 0x5aba4: ldr r0, [r4, #0x63c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1596)?)?;
    // 0x5aba8: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x5abac: str r10, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,52)?)?;
    // 0x5abb0: str r10, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,44)?)?;
    // 0x5abb4: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x5abb8: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x5abbc: mov r0, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,76)?)?;
    // 0x5abc0: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x5abc4: str r10, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,20)?)?;
    // 0x5abc8: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x5abcc: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5abd0: str r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,28)?)?;
    // 0x5abd4: str r10, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,56)?)?;
    // 0x5abd8: bl 0x8025c <entry+0x2aa5c> @ imm = #0x2567c
    c.emit(arm32::branch(C::Al,c.at as u32,0x8025c,true)?)?;
    // 0x5abdc: str r10, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,36)?)?;
    // 0x5abe0: str r10, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,32)?)?;
    // 0x5abe4: str r10, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,64)?)?;
    // 0x5abe8: ldr r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,124)?)?;
    // 0x5abec: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x5abf0: bl 0x7ec70 <entry+0x29470> @ imm = #0x24078
    c.emit(arm32::branch(C::Al,c.at as u32,0x7ec70,true)?)?;
    // 0x5abf4: ldr r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // 0x5abf8: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5abfc: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x5ac00: str r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // 0x5ac04: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5ac08: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x30398
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x5ac0c: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x5ac10: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x5ac14: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5ac18: bne 0x5ab74 <entry+0x5374>  @ imm = #-0xac
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5ab74,false)?)?;
    // 0x5ac1c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x5ac20: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x5ac24: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x5ac28)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5AC28:[u32;1]=[0x00221008];
 for value in INLINE_DATA_5AC28{c.emit(value)?;}
 // Executable A32 block 0x5ac2c..0x5b000.
 c.check(0x5ac2c)?;
    // 0x5ac2c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5ac30: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x5ac34: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x5ac38: add r8, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x5ac3c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5ac40: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x5ac44: mov r7, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16384)?)?;
    // 0x5ac48: mov r9, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,256)?)?;
    // 0x5ac4c: mov r10, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,128)?)?;
    // 0x5ac50: mov r12, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,32)?)?;
    // 0x5ac54: mov r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16)?)?;
    // 0x5ac58: add r0, r8, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,40)?)?;
    // 0x5ac5c: str r6, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,0)?)?;
    // 0x5ac60: str r6, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,4)?)?;
    // 0x5ac64: str r7, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,12)?)?;
    // 0x5ac68: str r9, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,20)?)?;
    // 0x5ac6c: str r10, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,8)?)?;
    // 0x5ac70: str r12, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,16)?)?;
    // 0x5ac74: str r7, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,28)?)?;
    // 0x5ac78: str r9, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,36)?)?;
    // 0x5ac7c: str r10, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,24)?)?;
    // 0x5ac80: str r5, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,32)?)?;
    // 0x5ac84: str r3, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // 0x5ac88: cmp r0, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x5ac8c: bls 0x5acb4 <entry+0x54b4>  @ imm = #0x20
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5acb4,false)?)?;
    // 0x5ac90: ldr r2, [pc, #0x380]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,896)?)?;
    // 0x5ac94: mov r1, #227
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,227)?)?;
    // 0x5ac98: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5ac9c: orr r1, r1, #5376
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5376)?)?;
    // 0x5aca0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x594f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5aca4: mov r7, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16384)?)?;
    // 0x5aca8: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x5acac: mov r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16)?)?;
    // 0x5acb0: mov r12, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,32)?)?;
    // 0x5acb4: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x5acb8: add r1, r4, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,40)?)?;
    // 0x5acbc: str r7, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,52)?)?;
    // 0x5acc0: str r9, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,60)?)?;
    // 0x5acc4: stm r1, {r0, r6, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(6)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5acc8: str r12, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,56)?)?;
    // 0x5accc: str r7, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,68)?)?;
    // 0x5acd0: str r9, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,76)?)?;
    // 0x5acd4: str r10, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,64)?)?;
    // 0x5acd8: str r5, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,72)?)?;
    // 0x5acdc: add r0, r8, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,80)?)?;
    // 0x5ace0: ldr r6, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,0)?)?;
    // 0x5ace4: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5ace8: bls 0x5ad00 <entry+0x5500>  @ imm = #0x10
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5ad00,false)?)?;
    // 0x5acec: ldr r2, [pc, #0x324]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,804)?)?;
    // 0x5acf0: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x5acf4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5acf8: orr r1, r1, #5376
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5376)?)?;
    // 0x5acfc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59554
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5ad00: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5ad04: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x5ad08: mov r9, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,16384)?)?;
    // 0x5ad0c: mov r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,256)?)?;
    // 0x5ad10: mov r5, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,128)?)?;
    // 0x5ad14: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x5ad18: mov r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,16)?)?;
    // 0x5ad1c: str r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // 0x5ad20: add r0, r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,120)?)?;
    // 0x5ad24: str r1, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,84)?)?;
    // 0x5ad28: str r9, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,92)?)?;
    // 0x5ad2c: str r7, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,100)?)?;
    // 0x5ad30: str r5, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,88)?)?;
    // 0x5ad34: str r2, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,96)?)?;
    // 0x5ad38: str r9, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,108)?)?;
    // 0x5ad3c: str r7, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,116)?)?;
    // 0x5ad40: str r5, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,104)?)?;
    // 0x5ad44: str r10, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,112)?)?;
    // 0x5ad48: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5ad4c: bls 0x5ad6c <entry+0x556c>  @ imm = #0x18
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5ad6c,false)?)?;
    // 0x5ad50: ldr r2, [pc, #0x2c0]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,704)?)?;
    // 0x5ad54: mov r1, #27
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,27)?)?;
    // 0x5ad58: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5ad5c: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x5ad60: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x595b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5ad64: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x5ad68: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x5ad6c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5ad70: add r3, r4, #120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,120)?)?;
    // 0x5ad74: str r7, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,140)?)?;
    // 0x5ad78: stm r3, {r0, r1, r5, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::new(5)?.bit()|R::new(9)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5ad7c: add r0, r4, #144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,144)?)?;
    // 0x5ad80: str r2, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,136)?)?;
    // 0x5ad84: str r7, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,156)?)?;
    // 0x5ad88: stm r0, {r5, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(5)?.bit()|R::new(9)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5ad8c: add r0, r8, #160
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,160)?)?;
    // 0x5ad90: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5ad94: bls 0x5adac <entry+0x55ac>  @ imm = #0x10
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5adac,false)?)?;
    // 0x5ad98: ldr r2, [pc, #0x278]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,632)?)?;
    // 0x5ad9c: mov r1, #55
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,55)?)?;
    // 0x5ada0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5ada4: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x5ada8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59600
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5adac: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x5adb0: mov r9, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,16384)?)?;
    // 0x5adb4: mov r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,256)?)?;
    // 0x5adb8: mov r5, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,128)?)?;
    // 0x5adbc: mov r10, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,32)?)?;
    // 0x5adc0: str r0, [r4, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,160)?)?;
    // 0x5adc4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5adc8: str r0, [r4, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,164)?)?;
    // 0x5adcc: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x5add0: str r9, [r4, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,172)?)?;
    // 0x5add4: str r7, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,180)?)?;
    // 0x5add8: str r5, [r4, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,168)?)?;
    // 0x5addc: str r10, [r4, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,176)?)?;
    // 0x5ade0: str r9, [r4, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,188)?)?;
    // 0x5ade4: str r7, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,196)?)?;
    // 0x5ade8: str r5, [r4, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,184)?)?;
    // 0x5adec: str r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,192)?)?;
    // 0x5adf0: add r0, r8, #184
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,184)?)?;
    // 0x5adf4: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5adf8: bls 0x5ae10 <entry+0x5610>  @ imm = #0x10
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5ae10,false)?)?;
    // 0x5adfc: ldr r2, [pc, #0x214]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,532)?)?;
    // 0x5ae00: mov r1, #83
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,83)?)?;
    // 0x5ae04: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5ae08: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x5ae0c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59664
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5ae10: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x5ae14: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x5ae18: add r1, r4, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,200)?)?;
    // 0x5ae1c: stm r1, {r0, r2, r5, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(5)?.bit()|R::new(9)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5ae20: add r0, r8, #208
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,208)?)?;
    // 0x5ae24: str r7, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,220)?)?;
    // 0x5ae28: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5ae2c: bls 0x5ae48 <entry+0x5648>  @ imm = #0x14
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5ae48,false)?)?;
    // 0x5ae30: ldr r2, [pc, #0x1e0]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,480)?)?;
    // 0x5ae34: mov r1, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,616)?)?;
    // 0x5ae38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5ae3c: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x5ae40: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59698
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5ae44: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x5ae48: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x5ae4c: mov r10, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,16384)?)?;
    // 0x5ae50: mov r9, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,128)?)?;
    // 0x5ae54: add r3, r4, #224
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,224)?)?;
    // 0x5ae58: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5ae5c: mov r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,32)?)?;
    // 0x5ae60: stm r3, {r0, r2, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R2.bit()|R::new(9)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5ae64: add r0, r8, #232
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,232)?)?;
    // 0x5ae68: str r5, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,240)?)?;
    // 0x5ae6c: str r1, [r4, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,244)?)?;
    // 0x5ae70: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5ae74: bls 0x5ae90 <entry+0x5690>  @ imm = #0x14
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5ae90,false)?)?;
    // 0x5ae78: ldr r2, [pc, #0x198]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,408)?)?;
    // 0x5ae7c: mov r1, #125
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,125)?)?;
    // 0x5ae80: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5ae84: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x5ae88: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x596e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5ae8c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5ae90: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ae94: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x5ae98: add r2, r4, #248
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,248)?)?;
    // 0x5ae9c: stm r2, {r0, r7, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::new(7)?.bit()|R::new(9)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5aea0: add r0, r8, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,256)?)?;
    // 0x5aea4: str r5, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,264)?)?;
    // 0x5aea8: str r1, [r4, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,268)?)?;
    // 0x5aeac: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5aeb0: bls 0x5aec8 <entry+0x56c8>  @ imm = #0x10
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5aec8,false)?)?;
    // 0x5aeb4: ldr r2, [pc, #0x15c]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,348)?)?;
    // 0x5aeb8: mov r1, #146
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,146)?)?;
    // 0x5aebc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5aec0: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x5aec4: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5971c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5aec8: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x5aecc: mov r10, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,128)?)?;
    // 0x5aed0: add r3, r4, #272
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,272)?)?;
    // 0x5aed4: mov r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // 0x5aed8: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5aedc: mov r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,32)?)?;
    // 0x5aee0: stm r3, {r0, r7, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::new(7)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5aee4: add r0, r8, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,280)?)?;
    // 0x5aee8: str r2, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,284)?)?;
    // 0x5aeec: str r5, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,288)?)?;
    // 0x5aef0: str r1, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,292)?)?;
    // 0x5aef4: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5aef8: bls 0x5af18 <entry+0x5718>  @ imm = #0x18
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5af18,false)?)?;
    // 0x5aefc: ldr r2, [pc, #0x114]        @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,276)?)?;
    // 0x5af00: mov r1, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,167)?)?;
    // 0x5af04: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5af08: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x5af0c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59764
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5af10: mov r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // 0x5af14: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5af18: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x5af1c: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // 0x5af20: add r3, r4, #296
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,296)?)?;
    // 0x5af24: stm r3, {r0, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::new(9)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5af28: add r0, r8, #304
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,304)?)?;
    // 0x5af2c: str r2, [r4, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,308)?)?;
    // 0x5af30: str r5, [r4, #0x138]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,312)?)?;
    // 0x5af34: str r1, [r4, #0x13c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,316)?)?;
    // 0x5af38: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5af3c: bls 0x5af54 <entry+0x5754>  @ imm = #0x10
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5af54,false)?)?;
    // 0x5af40: ldr r2, [pc, #0xd0]         @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,208)?)?;
    // 0x5af44: mov r1, #700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,700)?)?;
    // 0x5af48: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5af4c: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x5af50: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x597a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5af54: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x5af58: mov r10, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,128)?)?;
    // 0x5af5c: add r3, r4, #320
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,320)?)?;
    // 0x5af60: mov r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // 0x5af64: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5af68: mov r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,32)?)?;
    // 0x5af6c: stm r3, {r0, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::new(9)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5af70: add r0, r8, #328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,328)?)?;
    // 0x5af74: str r2, [r4, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,332)?)?;
    // 0x5af78: str r5, [r4, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,336)?)?;
    // 0x5af7c: str r1, [r4, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,340)?)?;
    // 0x5af80: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5af84: bls 0x5afa4 <entry+0x57a4>  @ imm = #0x18
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5afa4,false)?)?;
    // 0x5af88: ldr r2, [pc, #0x88]         @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,136)?)?;
    // 0x5af8c: mov r1, #209
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,209)?)?;
    // 0x5af90: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5af94: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x5af98: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x597f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5af9c: mov r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // 0x5afa0: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5afa4: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x5afa8: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x5afac: add r3, r4, #344
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,344)?)?;
    // 0x5afb0: stm r3, {r0, r7, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::new(7)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5afb4: add r0, r8, #352
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,352)?)?;
    // 0x5afb8: str r2, [r4, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,356)?)?;
    // 0x5afbc: str r5, [r4, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,360)?)?;
    // 0x5afc0: str r1, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,364)?)?;
    // 0x5afc4: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5afc8: bls 0x5afe0 <entry+0x57e0>  @ imm = #0x10
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5afe0,false)?)?;
    // 0x5afcc: ldr r2, [pc, #0x44]         @ 0x5818 <entry+0x5818>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,68)?)?;
    // 0x5afd0: mov r1, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,230)?)?;
    // 0x5afd4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5afd8: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x5afdc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59834
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5afe0: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x5afe4: mov r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16384)?)?;
    // 0x5afe8: mov r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256)?)?;
    // 0x5afec: mov r3, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x5aff0: mov r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,32)?)?;
    // 0x5aff4: str r0, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,368)?)?;
    // 0x5aff8: str r7, [r4, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,372)?)?;
    // 0x5affc: str r3, [r4, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,376)?)?;
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
