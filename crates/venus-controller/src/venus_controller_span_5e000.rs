// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x5e000,0x5f000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x5e000;
pub const END:usize=0x5f000;
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
 // Executable A32 block 0x5e000..0x5e024.
 c.check(0x5e000)?;
    // 0x5e000: ldr r1, [sp, #0xc8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,200)?)?;
    // 0x5e004: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e008: ldr r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,28)?)?;
    // 0x5e00c: ldr r1, [sp, #0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,204)?)?;
    // 0x5e010: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e014: ldr r1, [pc, #0x978]        @ 0x9194 <entry+0x9194>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2424)?)?;
    // 0x5e018: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5e01c: ldr r0, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,16)?)?;
    // 0x5e020: b 0x5e070 <entry+0x8870>  @ imm = #0x48
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e070,false)?)?;
 c.check(0x5e024)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E024:[u32;2]=[0x000ef737,0x02105002];
 for value in INLINE_DATA_5E024{c.emit(value)?;}
 // Executable A32 block 0x5e02c..0x5e1d0.
 c.check(0x5e02c)?;
    // 0x5e02c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e030: add r1, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,16)?)?;
    // 0x5e034: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5e038: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5e03c: blo 0x5e870 <entry+0x9070>  @ imm = #0x82c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e870,false)?)?;
    // 0x5e040: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5e044: ldr r1, [sp, #0xd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,208)?)?;
    // 0x5e048: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e04c: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5e050: ldr r1, [r11, #-0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-204)?)?;
    // 0x5e054: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e058: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5e05c: ldr r1, [r11, #-0xc8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-200)?)?;
    // 0x5e060: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e064: ldr r1, [pc, #0x924]        @ 0x9190 <entry+0x9190>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2340)?)?;
    // 0x5e068: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5e06c: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5e070: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5e074: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5e078: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f3ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e07c: ldr r1, [pc, #0x8c0]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2240)?)?;
    // 0x5e080: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5e084: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f3f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e088: b 0x5c13c <entry+0x693c>  @ imm = #-0x1f54
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c13c,false)?)?;
    // 0x5e08c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e090: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e094: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e098: blo 0x5e87c <entry+0x907c>  @ imm = #0x7dc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e87c,false)?)?;
    // 0x5e09c: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e0a0: ldr r0, [r11, #-0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-188)?)?;
    // 0x5e0a4: ldr r1, [pc, #0x8c4]        @ 0x9170 <entry+0x9170>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2244)?)?;
    // 0x5e0a8: strb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5e0ac: b 0x5e478 <entry+0x8c78>  @ imm = #0x3c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
    // 0x5e0b0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e0b4: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e0b8: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e0bc: blo 0x5e888 <entry+0x9088>  @ imm = #0x7c4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e888,false)?)?;
    // 0x5e0c0: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e0c4: ldr r0, [r11, #-0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-192)?)?;
    // 0x5e0c8: ldr r1, [pc, #0x8a8]        @ 0x9178 <entry+0x9178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2216)?)?;
    // 0x5e0cc: strb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5e0d0: b 0x5e478 <entry+0x8c78>  @ imm = #0x3a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
    // 0x5e0d4: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e0d8: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e0dc: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e0e0: blo 0x5e894 <entry+0x9094>  @ imm = #0x7ac
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e894,false)?)?;
    // 0x5e0e4: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e0e8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5e0ec: strb r2, [r9, #0xe8c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,3724)?)?;
    // 0x5e0f0: ldr r1, [r11, #-0xa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-164)?)?;
    // 0x5e0f4: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e0f8: ldr r1, [pc, #0x874]        @ 0x9174 <entry+0x9174>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2164)?)?;
    // 0x5e0fc: b 0x5e478 <entry+0x8c78>  @ imm = #0x374
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
    // 0x5e100: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e104: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e108: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e10c: blo 0x5e8a0 <entry+0x90a0>  @ imm = #0x78c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e8a0,false)?)?;
    // 0x5e110: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5e114: ldr r1, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-52)?)?;
    // 0x5e118: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5e11c: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e120: and r1, r0, #254
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,254)?)?;
    // 0x5e124: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x5e128: blo 0x5e170 <entry+0x8970>  @ imm = #0x40
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e170,false)?)?;
    // 0x5e12c: ldr r1, [r9, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,3372)?)?;
    // 0x5e130: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x5e134: bne 0x5e170 <entry+0x8970>  @ imm = #0x34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5e170,false)?)?;
    // 0x5e138: ldr r1, [pc, #0x880]        @ 0x91c0 <entry+0x91c0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2176)?)?;
    // 0x5e13c: ldr r2, [pc, #0x7fc]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2044)?)?;
    // 0x5e140: mov r3, #121
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,121)?)?;
    // 0x5e144: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e148: orr r3, r3, #6912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6912)?)?;
    // 0x5e14c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f4c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e150: ldr r1, [pc, #0x7ec]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2028)?)?;
    // 0x5e154: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e158: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f4cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e15c: ldr r1, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-52)?)?;
    // 0x5e160: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x5e164: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e168: ldr r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-56)?)?;
    // 0x5e16c: add r6, r1, #9
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,9)?)?;
    // 0x5e170: ands r2, r0, #255
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R0,255,true)?)?;
    // 0x5e174: ldr r1, [pc, #0x848]        @ 0x91c4 <entry+0x91c4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2120)?)?;
    // 0x5e178: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x5e17c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5e180: strb r0, [r9, #0xe30]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3632)?)?;
    // 0x5e184: b 0x5e264 <entry+0x8a64>  @ imm = #0xd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e264,false)?)?;
    // 0x5e188: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e18c: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e190: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e194: blo 0x5e8ac <entry+0x90ac>  @ imm = #0x710
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e8ac,false)?)?;
    // 0x5e198: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e19c: ldr r0, [r11, #-0xb8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-184)?)?;
    // 0x5e1a0: ldr r1, [pc, #0x7c4]        @ 0x916c <entry+0x916c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1988)?)?;
    // 0x5e1a4: strb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5e1a8: b 0x5e478 <entry+0x8c78>  @ imm = #0x2c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
    // 0x5e1ac: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e1b0: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e1b4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e1b8: blo 0x5e8b8 <entry+0x90b8>  @ imm = #0x6f8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e8b8,false)?)?;
    // 0x5e1bc: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e1c0: ldr r0, [r11, #-0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-180)?)?;
    // 0x5e1c4: ldr r1, [pc, #0x79c]        @ 0x9168 <entry+0x9168>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1948)?)?;
    // 0x5e1c8: strb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5e1cc: b 0x5e478 <entry+0x8c78>  @ imm = #0x2a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
 c.check(0x5e1d0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E1D0:[u32;2]=[0x7f100003,0x000efce0];
 for value in INLINE_DATA_5E1D0{c.emit(value)?;}
 // Executable A32 block 0x5e1d8..0x5e2a4.
 c.check(0x5e1d8)?;
    // 0x5e1d8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e1dc: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e1e0: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e1e4: blo 0x5e8c4 <entry+0x90c4>  @ imm = #0x6d8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e8c4,false)?)?;
    // 0x5e1e8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5e1ec: ldr r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // 0x5e1f0: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e1f4: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x5e1f8: and r0, r0, #252
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,252)?)?;
    // 0x5e1fc: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x5e200: bhi 0x5e214 <entry+0x8a14>  @ imm = #0xc
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5e214,false)?)?;
    // 0x5e204: ldr r0, [r9, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3372)?)?;
    // 0x5e208: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5e20c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x5e210: beq 0x5e260 <entry+0x8a60>  @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5e260,false)?)?;
    // 0x5e214: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x5e218: ldr r1, [pc, #0x798]        @ 0x91b8 <entry+0x91b8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1944)?)?;
    // 0x5e21c: ldr r2, [pc, #0x71c]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1820)?)?;
    // 0x5e220: mov r3, #920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,920)?)?;
    // 0x5e224: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e228: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5e22c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f5a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e230: ldr r1, [pc, #0x70c]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1804)?)?;
    // 0x5e234: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e238: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f5ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e23c: ldr r2, [pc, #0x704]        @ 0x9148 <entry+0x9148>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1796)?)?;
    // 0x5e240: mov r1, #154
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,154)?)?;
    // 0x5e244: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5e248: orr r1, r1, #6912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6912)?)?;
    // 0x5e24c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5caa4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5e250: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e254: add r6, r0, #43
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,43)?)?;
    // 0x5e258: ldr r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // 0x5e25c: ldrb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x5e260: ldr r1, [pc, #0x754]        @ 0x91bc <entry+0x91bc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1876)?)?;
    // 0x5e264: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5e268: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f5dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e26c: ldr r1, [pc, #0x6d0]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1744)?)?;
    // 0x5e270: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5e274: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f5e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e278: b 0x5c2b8 <entry+0x6ab8>  @ imm = #-0x1fc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c2b8,false)?)?;
    // 0x5e27c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e280: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e284: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e288: blo 0x5e8d4 <entry+0x90d4>  @ imm = #0x644
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e8d4,false)?)?;
    // 0x5e28c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5e290: ldr r1, [sp, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,172)?)?;
    // 0x5e294: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e298: ldr r1, [pc, #0x710]        @ 0x91b0 <entry+0x91b0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1808)?)?;
    // 0x5e29c: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x5e2a0: b 0x5e478 <entry+0x8c78>  @ imm = #0x1d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
 c.check(0x5e2a4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E2A4:[u32;2]=[0x000f04e1,0x000ef50b];
 for value in INLINE_DATA_5E2A4{c.emit(value)?;}
 // Executable A32 block 0x5e2ac..0x5e38c.
 c.check(0x5e2ac)?;
    // 0x5e2ac: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e2b0: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e2b4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e2b8: blo 0x5e8e0 <entry+0x90e0>  @ imm = #0x620
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e8e0,false)?)?;
    // 0x5e2bc: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5e2c0: lsl r1, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsl,24)?)?;
    // 0x5e2c4: strb r0, [r9, #0xe40]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3648)?)?;
    // 0x5e2c8: cmp r1, #83886080
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,83886080)?)?;
    // 0x5e2cc: bgt 0x5e2e0 <entry+0x8ae0>  @ imm = #0xc
    c.emit(arm32::branch(C::Gt,c.at as u32,0x5e2e0,false)?)?;
    // 0x5e2d0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5e2d4: bge 0x5e33c <entry+0x8b3c>  @ imm = #0x60
    c.emit(arm32::branch(C::Ge,c.at as u32,0x5e33c,false)?)?;
    // 0x5e2d8: cmp r1, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4278190080)?)?;
    // 0x5e2dc: beq 0x5e33c <entry+0x8b3c>  @ imm = #0x58
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5e33c,false)?)?;
    // 0x5e2e0: ldr r3, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // 0x5e2e4: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5e2e8: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5e2ec: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5e2f0: add r3, r3, #46
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,46)?)?;
    // 0x5e2f4: str r3, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // 0x5e2f8: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x5e2fc: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5e300: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5e304: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5e308: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5e30c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x33984
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5e310: ldr r1, [pc, #0x6bc]        @ 0x91d4 <entry+0x91d4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1724)?)?;
    // 0x5e314: ldr r2, [pc, #0x624]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1572)?)?;
    // 0x5e318: mov r3, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,38)?)?;
    // 0x5e31c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e320: orr r3, r3, #6656
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6656)?)?;
    // 0x5e324: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f698
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e328: ldr r1, [pc, #0x614]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1556)?)?;
    // 0x5e32c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e330: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f6a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e334: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x5e338: strb r0, [r9, #0xe40]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3648)?)?;
    // 0x5e33c: add r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1)?)?;
    // 0x5e340: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x5e344: lsl r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,24)?)?;
    // 0x5e348: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x5e34c: asr r2, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Asr,24)?)?;
    // 0x5e350: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5e354: ldr r1, [pc, #0x67c]        @ 0x91d8 <entry+0x91d8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1660)?)?;
    // 0x5e358: movhi r3, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x5e35c: strb r3, [r9, #0xe41]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(9)?,3649)?)?;
    // 0x5e360: b 0x5c660 <entry+0x6e60>  @ imm = #-0x1d08
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c660,false)?)?;
    // 0x5e364: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e368: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e36c: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e370: blo 0x5e8ec <entry+0x90ec>  @ imm = #0x574
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e8ec,false)?)?;
    // 0x5e374: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5e378: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // 0x5e37c: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e380: ldr r1, [pc, #0x648]        @ 0x91d0 <entry+0x91d0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1608)?)?;
    // 0x5e384: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x5e388: b 0x5e478 <entry+0x8c78>  @ imm = #0xe8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
 c.check(0x5e38c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E38C:[u32;1]=[0x000ef763];
 for value in INLINE_DATA_5E38C{c.emit(value)?;}
 // Executable A32 block 0x5e390..0x5e458.
 c.check(0x5e390)?;
    // 0x5e390: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e394: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e398: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e39c: blo 0x5e8f8 <entry+0x90f8>  @ imm = #0x554
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e8f8,false)?)?;
    // 0x5e3a0: ldr r1, [r11, #-0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-176)?)?;
    // 0x5e3a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5e3a8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5e3ac: ldr r1, [pc, #0x5b0]        @ 0x9164 <entry+0x9164>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1456)?)?;
    // 0x5e3b0: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e3b4: ldr r0, [r11, #-0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-172)?)?;
    // 0x5e3b8: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5e3bc: b 0x5e478 <entry+0x8c78>  @ imm = #0xb4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
    // 0x5e3c0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e3c4: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e3c8: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e3cc: blo 0x5e904 <entry+0x9104>  @ imm = #0x530
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e904,false)?)?;
    // 0x5e3d0: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e3d4: ldr r0, [r11, #-0xa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-160)?)?;
    // 0x5e3d8: ldr r1, [pc, #0x580]        @ 0x9160 <entry+0x9160>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1408)?)?;
    // 0x5e3dc: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5e3e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5e3e4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f758
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e3e8: b 0x5e42c <entry+0x8c2c>  @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e42c,false)?)?;
    // 0x5e3ec: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e3f0: add r10, r10, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,52)?)?;
    // 0x5e3f4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e3f8: blo 0x5e910 <entry+0x9110>  @ imm = #0x510
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e910,false)?)?;
    // 0x5e3fc: ldr r0, [r11, #-0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-168)?)?;
    // 0x5e400: ldm r5!, {r1, r2, r3, r4, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5e404: stm r0!, {r1, r2, r3, r4, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5e408: ldm r5, {r1, r2, r3, r4, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5e40c: stm r0, {r1, r2, r3, r4, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5e410: ldr r1, [pc, #0x540]        @ 0x9158 <entry+0x9158>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1344)?)?;
    // 0x5e414: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5e418: mov r4, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,20480)?)?;
    // 0x5e41c: str r0, [r9, #0xc50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3152)?)?;
    // 0x5e420: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5e424: orr r4, r4, #35651584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,35651584)?)?;
    // 0x5e428: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f79c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e42c: ldr r1, [pc, #0x510]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1296)?)?;
    // 0x5e430: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5e434: b 0x5e4c0 <entry+0x8cc0>  @ imm = #0x84
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4c0,false)?)?;
    // 0x5e438: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e43c: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e440: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e444: blo 0x5e91c <entry+0x911c>  @ imm = #0x4d0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e91c,false)?)?;
    // 0x5e448: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e44c: ldr r1, [pc, #0x4fc]        @ 0x9150 <entry+0x9150>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1276)?)?;
    // 0x5e450: strb r2, [r9, #0xde4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,3556)?)?;
    // 0x5e454: b 0x5e478 <entry+0x8c78>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
 c.check(0x5e458)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E458:[u32;1]=[0x000ef56b];
 for value in INLINE_DATA_5E458{c.emit(value)?;}
 // Executable A32 block 0x5e45c..0x5e48c.
 c.check(0x5e45c)?;
    // 0x5e45c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e460: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e464: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e468: blo 0x5e928 <entry+0x9128>  @ imm = #0x4b8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e928,false)?)?;
    // 0x5e46c: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e470: ldr r1, [pc, #0x4d4]        @ 0x914c <entry+0x914c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1236)?)?;
    // 0x5e474: strb r2, [r9, #0xdf9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,3577)?)?;
    // 0x5e478: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5e47c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f7f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e480: ldr r1, [pc, #0x4bc]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1212)?)?;
    // 0x5e484: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5e488: b 0x5e4c0 <entry+0x8cc0>  @ imm = #0x30
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4c0,false)?)?;
 c.check(0x5e48c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E48C:[u32;1]=[0x000ef07e];
 for value in INLINE_DATA_5E48C{c.emit(value)?;}
 // Executable A32 block 0x5e490..0x5e4d4.
 c.check(0x5e490)?;
    // 0x5e490: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5e494: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5e498: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5e49c: blo 0x5e934 <entry+0x9134>  @ imm = #0x490
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e934,false)?)?;
    // 0x5e4a0: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5e4a4: ldr r0, [sp, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,156)?)?;
    // 0x5e4a8: ldr r1, [pc, #0x4fc]        @ 0x91ac <entry+0x91ac>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1276)?)?;
    // 0x5e4ac: strb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5e4b0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5e4b4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f828
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e4b8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5e4bc: ldr r1, [pc, #0x480]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1152)?)?;
    // 0x5e4c0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f834
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e4c4: subs r8, r8, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::new(8)?,R::new(8)?,1,true)?)?;
    // 0x5e4c8: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x5e4cc: bne 0x5c9a0 <entry+0x71a0>  @ imm = #-0x1b34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c9a0,false)?)?;
    // 0x5e4d0: b 0x5b890 <entry+0x6090>  @ imm = #-0x2c48
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b890,false)?)?;
 c.check(0x5e4d4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E4D4:[u32;3]=[0x000f04a9,0x000ef01b,0x000ef044];
 for value in INLINE_DATA_5E4D4{c.emit(value)?;}
 // Executable A32 block 0x5e4e0..0x5e54c.
 c.check(0x5e4e0)?;
    // 0x5e4e0: ldr r1, [pc, #0x500]        @ 0x91e8 <entry+0x91e8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1280)?)?;
    // 0x5e4e4: ldr r2, [pc, #0x454]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1108)?)?;
    // 0x5e4e8: mov r3, #162
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,162)?)?;
    // 0x5e4ec: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5e4f0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e4f4: orr r3, r3, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7936)?)?;
    // 0x5e4f8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f86c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e4fc: ldr r1, [pc, #0x440]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1088)?)?;
    // 0x5e500: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e504: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f878
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e508: mov r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,7)?)?;
    // 0x5e50c: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x5e510: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5e514: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5e518: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x5e51c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5e520: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5e524: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5e528: stm sp, {r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5e52c: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x5e530: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x33ba8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5e534: ldr r1, [pc, #0x4b0]        @ 0x91ec <entry+0x91ec>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1200)?)?;
    // 0x5e538: ldr r2, [pc, #0x400]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1024)?)?;
    // 0x5e53c: mov r3, #170
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,170)?)?;
    // 0x5e540: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x5e544: orr r3, r3, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7936)?)?;
    // 0x5e548: b 0x5b87c <entry+0x607c>  @ imm = #-0x2cd4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b87c,false)?)?;
 c.check(0x5e54c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E54C:[u32;1]=[0x000eeed2];
 for value in INLINE_DATA_5E54C{c.emit(value)?;}
 // Executable A32 block 0x5e550..0x5e574.
 c.check(0x5e550)?;
    // 0x5e550: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x5e554: orr r6, r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,8192)?)?;
    // 0x5e558: b 0x5e510 <entry+0x8d10>  @ imm = #-0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e55c: ldr r6, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,60)?)?;
    // 0x5e560: b 0x5e510 <entry+0x8d10>  @ imm = #-0x58
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e564: ldr r1, [pc, #0x43c]        @ 0x91a8 <entry+0x91a8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1084)?)?;
    // 0x5e568: ldr r2, [pc, #0x3d0]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,976)?)?;
    // 0x5e56c: mov r3, #173
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,173)?)?;
    // 0x5e570: b 0x5e600 <entry+0x8e00>  @ imm = #0x88
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e600,false)?)?;
 c.check(0x5e574)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E574:[u32;1]=[0x000f06fb];
 for value in INLINE_DATA_5E574{c.emit(value)?;}
 // Executable A32 block 0x5e578..0x5e59c.
 c.check(0x5e578)?;
    // 0x5e578: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x5e57c: add r6, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,4)?)?;
    // 0x5e580: b 0x5e510 <entry+0x8d10>  @ imm = #-0x78
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e584: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x5e588: add r6, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,7)?)?;
    // 0x5e58c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x84
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e590: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x5e594: add r6, r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,10)?)?;
    // 0x5e598: b 0x5e510 <entry+0x8d10>  @ imm = #-0x90
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e59c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E59C:[u32;1]=[0x000f039c];
 for value in INLINE_DATA_5E59C{c.emit(value)?;}
 // Executable A32 block 0x5e5a0..0x5e5c4.
 c.check(0x5e5a0)?;
    // 0x5e5a0: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x5e5a4: add r6, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,12)?)?;
    // 0x5e5a8: b 0x5e510 <entry+0x8d10>  @ imm = #-0xa0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e5ac: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x5e5b0: b 0x5e50c <entry+0x8d0c>  @ imm = #-0xac
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e50c,false)?)?;
    // 0x5e5b4: ldr r6, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,64)?)?;
    // 0x5e5b8: b 0x5e510 <entry+0x8d10>  @ imm = #-0xb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e5bc: mov r6, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,21)?)?;
    // 0x5e5c0: b 0x5e50c <entry+0x8d0c>  @ imm = #-0xbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e50c,false)?)?;
 c.check(0x5e5c4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E5C4:[u32;1]=[0x000efd17];
 for value in INLINE_DATA_5E5C4{c.emit(value)?;}
 // Executable A32 block 0x5e5c8..0x5e5e0.
 c.check(0x5e5c8)?;
    // 0x5e5c8: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x5e5cc: add r6, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,4096)?)?;
    // 0x5e5d0: b 0x5e510 <entry+0x8d10>  @ imm = #-0xc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e5d4: mov r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x5e5d8: orr r6, r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,8192)?)?;
    // 0x5e5dc: b 0x5e510 <entry+0x8d10>  @ imm = #-0xd4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e5e0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E5E0:[u32;1]=[0x000efd5b];
 for value in INLINE_DATA_5E5E0{c.emit(value)?;}
 // Executable A32 block 0x5e5e4..0x5e5f0.
 c.check(0x5e5e4)?;
    // 0x5e5e4: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x5e5e8: orr r6, r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,8192)?)?;
    // 0x5e5ec: b 0x5e510 <entry+0x8d10>  @ imm = #-0xe4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e5f0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E5F0:[u32;1]=[0x000efdab];
 for value in INLINE_DATA_5E5F0{c.emit(value)?;}
 // Executable A32 block 0x5e5f4..0x5e630.
 c.check(0x5e5f4)?;
    // 0x5e5f4: ldr r1, [pc, #0x3a8]        @ 0x91a4 <entry+0x91a4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,936)?)?;
    // 0x5e5f8: ldr r2, [pc, #0x340]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,832)?)?;
    // 0x5e5fc: mov r3, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,180)?)?;
    // 0x5e600: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7168)?)?;
    // 0x5e604: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e608: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f97c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e60c: ldr r1, [pc, #0x330]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,816)?)?;
    // 0x5e610: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e614: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f988
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e618: ldr r6, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,36)?)?;
    // 0x5e61c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x114
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e620: ldr r1, [pc, #0x380]        @ 0x91a8 <entry+0x91a8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,896)?)?;
    // 0x5e624: ldr r2, [pc, #0x314]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,788)?)?;
    // 0x5e628: mov r3, #246
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,246)?)?;
    // 0x5e62c: b 0x5e640 <entry+0x8e40>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e640,false)?)?;
 c.check(0x5e630)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E630:[u32;1]=[0x000eef0d];
 for value in INLINE_DATA_5E630{c.emit(value)?;}
 // Executable A32 block 0x5e634..0x5e668.
 c.check(0x5e634)?;
    // 0x5e634: ldr r1, [pc, #0x368]        @ 0x91a4 <entry+0x91a4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,872)?)?;
    // 0x5e638: ldr r2, [pc, #0x300]        @ 0x9140 <entry+0x9140>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,768)?)?;
    // 0x5e63c: mov r3, #253
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,253)?)?;
    // 0x5e640: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7168)?)?;
    // 0x5e644: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e648: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f9bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e64c: ldr r1, [pc, #0x2f0]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,752)?)?;
    // 0x5e650: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5e654: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f9c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5e658: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x5e65c: b 0x5e50c <entry+0x8d0c>  @ imm = #-0x158
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e50c,false)?)?;
    // 0x5e660: mov r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x5e664: b 0x5e50c <entry+0x8d0c>  @ imm = #-0x160
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e50c,false)?)?;
 c.check(0x5e668)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E668:[u32;1]=[0x000ef2de];
 for value in INLINE_DATA_5E668{c.emit(value)?;}
 // Executable A32 block 0x5e66c..0x5e6d4.
 c.check(0x5e66c)?;
    // 0x5e66c: mov r6, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,5)?)?;
    // 0x5e670: b 0x5e50c <entry+0x8d0c>  @ imm = #-0x16c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e50c,false)?)?;
    // 0x5e674: mov r6, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,11)?)?;
    // 0x5e678: b 0x5e50c <entry+0x8d0c>  @ imm = #-0x174
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e50c,false)?)?;
    // 0x5e67c: mov r6, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,17)?)?;
    // 0x5e680: b 0x5e50c <entry+0x8d0c>  @ imm = #-0x17c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e50c,false)?)?;
    // 0x5e684: mov r6, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,23)?)?;
    // 0x5e688: b 0x5e50c <entry+0x8d0c>  @ imm = #-0x184
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e50c,false)?)?;
    // 0x5e68c: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e690: add r6, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,2)?)?;
    // 0x5e694: b 0x5e510 <entry+0x8d10>  @ imm = #-0x18c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e698: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e69c: add r6, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,3)?)?;
    // 0x5e6a0: b 0x5e510 <entry+0x8d10>  @ imm = #-0x198
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e6a4: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e6a8: add r6, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,4)?)?;
    // 0x5e6ac: b 0x5e510 <entry+0x8d10>  @ imm = #-0x1a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e6b0: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e6b4: add r6, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,5)?)?;
    // 0x5e6b8: b 0x5e510 <entry+0x8d10>  @ imm = #-0x1b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e6bc: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e6c0: add r6, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,6)?)?;
    // 0x5e6c4: b 0x5e510 <entry+0x8d10>  @ imm = #-0x1bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e6c8: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e6cc: add r6, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,7)?)?;
    // 0x5e6d0: b 0x5e510 <entry+0x8d10>  @ imm = #-0x1c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e6d4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E6D4:[u32;1]=[0x000f0427];
 for value in INLINE_DATA_5E6D4{c.emit(value)?;}
 // Executable A32 block 0x5e6d8..0x5e7c8.
 c.check(0x5e6d8)?;
    // 0x5e6d8: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e6dc: add r6, r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,9)?)?;
    // 0x5e6e0: b 0x5e510 <entry+0x8d10>  @ imm = #-0x1d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e6e4: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e6e8: add r6, r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,10)?)?;
    // 0x5e6ec: b 0x5e510 <entry+0x8d10>  @ imm = #-0x1e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e6f0: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x5e6f4: add r6, r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,11)?)?;
    // 0x5e6f8: b 0x5e510 <entry+0x8d10>  @ imm = #-0x1f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e6fc: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // 0x5e700: b 0x5e510 <entry+0x8d10>  @ imm = #-0x1f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e704: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x5e708: add r6, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,1)?)?;
    // 0x5e70c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x204
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e710: ldr r6, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-40)?)?;
    // 0x5e714: b 0x5e510 <entry+0x8d10>  @ imm = #-0x20c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e718: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e71c: add r6, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,4096)?)?;
    // 0x5e720: b 0x5e510 <entry+0x8d10>  @ imm = #-0x218
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e724: ldr r6, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-56)?)?;
    // 0x5e728: b 0x5e510 <entry+0x8d10>  @ imm = #-0x220
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e72c: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e730: add r6, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,1)?)?;
    // 0x5e734: b 0x5e510 <entry+0x8d10>  @ imm = #-0x22c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e738: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e73c: add r6, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,6)?)?;
    // 0x5e740: b 0x5e510 <entry+0x8d10>  @ imm = #-0x238
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e744: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e748: add r6, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,7)?)?;
    // 0x5e74c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x244
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e750: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e754: add r6, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,8)?)?;
    // 0x5e758: b 0x5e510 <entry+0x8d10>  @ imm = #-0x250
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e75c: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e760: add r6, r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,9)?)?;
    // 0x5e764: b 0x5e510 <entry+0x8d10>  @ imm = #-0x25c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e768: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e76c: add r6, r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,11)?)?;
    // 0x5e770: b 0x5e510 <entry+0x8d10>  @ imm = #-0x268
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e774: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e778: add r6, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,12)?)?;
    // 0x5e77c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x274
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e780: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e784: add r6, r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,13)?)?;
    // 0x5e788: b 0x5e510 <entry+0x8d10>  @ imm = #-0x280
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e78c: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e790: add r6, r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,14)?)?;
    // 0x5e794: b 0x5e510 <entry+0x8d10>  @ imm = #-0x28c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e798: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e79c: add r6, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,15)?)?;
    // 0x5e7a0: b 0x5e510 <entry+0x8d10>  @ imm = #-0x298
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e7a4: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e7a8: add r6, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,16)?)?;
    // 0x5e7ac: b 0x5e510 <entry+0x8d10>  @ imm = #-0x2a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e7b0: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e7b4: add r6, r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,17)?)?;
    // 0x5e7b8: b 0x5e510 <entry+0x8d10>  @ imm = #-0x2b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e7bc: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e7c0: add r6, r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,18)?)?;
    // 0x5e7c4: b 0x5e510 <entry+0x8d10>  @ imm = #-0x2bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e7c8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E7C8:[u32;1]=[0x000ef1c0];
 for value in INLINE_DATA_5E7C8{c.emit(value)?;}
 // Executable A32 block 0x5e7cc..0x5e814.
 c.check(0x5e7cc)?;
    // 0x5e7cc: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x5e7d0: add r6, r0, #19
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,19)?)?;
    // 0x5e7d4: b 0x5e510 <entry+0x8d10>  @ imm = #-0x2cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e7d8: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e7dc: add r6, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,1)?)?;
    // 0x5e7e0: b 0x5e510 <entry+0x8d10>  @ imm = #-0x2d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e7e4: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e7e8: add r6, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,2)?)?;
    // 0x5e7ec: b 0x5e510 <entry+0x8d10>  @ imm = #-0x2e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e7f0: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e7f4: add r6, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,3)?)?;
    // 0x5e7f8: b 0x5e510 <entry+0x8d10>  @ imm = #-0x2f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e7fc: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e800: add r6, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,8)?)?;
    // 0x5e804: b 0x5e510 <entry+0x8d10>  @ imm = #-0x2fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e808: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e80c: add r6, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,12)?)?;
    // 0x5e810: b 0x5e510 <entry+0x8d10>  @ imm = #-0x308
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e814)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E814:[u32;1]=[0x000ef267];
 for value in INLINE_DATA_5E814{c.emit(value)?;}
 // Executable A32 block 0x5e818..0x5e860.
 c.check(0x5e818)?;
    // 0x5e818: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e81c: add r6, r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,13)?)?;
    // 0x5e820: b 0x5e510 <entry+0x8d10>  @ imm = #-0x318
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e824: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e828: add r6, r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,17)?)?;
    // 0x5e82c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x324
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e830: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e834: add r6, r0, #19
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,19)?)?;
    // 0x5e838: b 0x5e510 <entry+0x8d10>  @ imm = #-0x330
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e83c: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e840: add r6, r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,20)?)?;
    // 0x5e844: b 0x5e510 <entry+0x8d10>  @ imm = #-0x33c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e848: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e84c: add r6, r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,21)?)?;
    // 0x5e850: b 0x5e510 <entry+0x8d10>  @ imm = #-0x348
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e854: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e858: add r6, r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,22)?)?;
    // 0x5e85c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x354
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e860)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E860:[u32;1]=[0x000efc4f];
 for value in INLINE_DATA_5E860{c.emit(value)?;}
 // Executable A32 block 0x5e864..0x5e8d0.
 c.check(0x5e864)?;
    // 0x5e864: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e868: add r6, r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,28)?)?;
    // 0x5e86c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x364
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e870: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e874: add r6, r0, #29
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,29)?)?;
    // 0x5e878: b 0x5e510 <entry+0x8d10>  @ imm = #-0x370
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e87c: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e880: add r6, r0, #33
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,33)?)?;
    // 0x5e884: b 0x5e510 <entry+0x8d10>  @ imm = #-0x37c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e888: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e88c: add r6, r0, #34
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,34)?)?;
    // 0x5e890: b 0x5e510 <entry+0x8d10>  @ imm = #-0x388
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e894: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e898: add r6, r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,36)?)?;
    // 0x5e89c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x394
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e8a0: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e8a4: add r6, r0, #37
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,37)?)?;
    // 0x5e8a8: b 0x5e510 <entry+0x8d10>  @ imm = #-0x3a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e8ac: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e8b0: add r6, r0, #38
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,38)?)?;
    // 0x5e8b4: b 0x5e510 <entry+0x8d10>  @ imm = #-0x3ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e8b8: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e8bc: add r6, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,40)?)?;
    // 0x5e8c0: b 0x5e510 <entry+0x8d10>  @ imm = #-0x3b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e8c4: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e8c8: add r6, r0, #43
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,43)?)?;
    // 0x5e8cc: b 0x5e510 <entry+0x8d10>  @ imm = #-0x3c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e8d0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E8D0:[u32;1]=[0x000effeb];
 for value in INLINE_DATA_5E8D0{c.emit(value)?;}
 // Executable A32 block 0x5e8d4..0x5e940.
 c.check(0x5e8d4)?;
    // 0x5e8d4: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e8d8: add r6, r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,44)?)?;
    // 0x5e8dc: b 0x5e510 <entry+0x8d10>  @ imm = #-0x3d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e8e0: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e8e4: add r6, r0, #46
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,46)?)?;
    // 0x5e8e8: b 0x5e510 <entry+0x8d10>  @ imm = #-0x3e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e8ec: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e8f0: add r6, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,48)?)?;
    // 0x5e8f4: b 0x5e510 <entry+0x8d10>  @ imm = #-0x3ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e8f8: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e8fc: add r6, r0, #50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,50)?)?;
    // 0x5e900: b 0x5e510 <entry+0x8d10>  @ imm = #-0x3f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e904: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e908: add r6, r0, #51
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,51)?)?;
    // 0x5e90c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x404
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e910: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e914: add r6, r0, #53
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,53)?)?;
    // 0x5e918: b 0x5e510 <entry+0x8d10>  @ imm = #-0x410
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e91c: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e920: add r6, r0, #54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,54)?)?;
    // 0x5e924: b 0x5e510 <entry+0x8d10>  @ imm = #-0x41c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e928: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5e92c: add r6, r0, #55
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,55)?)?;
    // 0x5e930: b 0x5e510 <entry+0x8d10>  @ imm = #-0x428
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5e934: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x5e938: add r6, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,4096)?)?;
    // 0x5e93c: b 0x5e510 <entry+0x8d10>  @ imm = #-0x434
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5e940)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5E940:[u32;44]=[0x000eeb65,0x000e2788,0x000ee470,0x000f06b8,0x000f067a,0x000f0614,0x000f05dc,0x000f058f,0x000f03e6,0x000f035a,0x000f0320,0x000f0182,0x000f0151,0x000f0119,0x000f00e3,0x000f00a7,0x000effb4,0x000eff89,0x000eff4a,0x000eff1f,0x000efe98,0x000efe3a,0x000efe08,0x000efcb5,0x000efc8a,0x000ef8ba,0x000ef89c,0x000ef70c,0x000ef5a3,0x000ef53b,0x000ef490,0x000ef4c4,0x000ef41f,0x000ef449,0x000ef342,0x000ef169,0x000eefe1,0x000eef3f,0x000eef96,0x000eed92,0x000eed53,0x000eed13,0x000f072e,0x000f0764];
 for value in INLINE_DATA_5E940{c.emit(value)?;}
 // Executable A32 block 0x5e9f0..0x5ec34.
 c.check(0x5e9f0)?;
    // 0x5e9f0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5e9f4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x5e9f8: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x5e9fc: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5ea00: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5ea04: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // 0x5ea08: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x5ea0c: bne 0x5ea60 <entry+0x9260>  @ imm = #0x4c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5ea60,false)?)?;
    // 0x5ea10: ldr r1, [pc, #0x358]        @ 0x9570 <entry+0x9570>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,856)?)?;
    // 0x5ea14: ldr r2, [pc, #0x338]        @ 0x9554 <entry+0x9554>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,824)?)?;
    // 0x5ea18: mov r3, #193
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,193)?)?;
    // 0x5ea1c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ea20: orr r3, r3, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7936)?)?;
    // 0x5ea24: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2fd98
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ea28: ldr r1, [pc, #0x328]        @ 0x9558 <entry+0x9558>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,808)?)?;
    // 0x5ea2c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ea30: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2fda4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ea34: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x5ea38: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5ea3c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5ea40: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5ea44: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5ea48: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5ea4c: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5ea50: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x5ea54: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x5ea58: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x340d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5ea5c: b 0x5ebd8 <entry+0x93d8>  @ imm = #0x174
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ebd8,false)?)?;
    // 0x5ea60: ldr r2, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,28)?)?;
    // 0x5ea64: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5ea68: bne 0x5eaac <entry+0x92ac>  @ imm = #0x3c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5eaac,false)?)?;
    // 0x5ea6c: cmp r2, #536870912
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,536870912)?)?;
    // 0x5ea70: blo 0x5eac0 <entry+0x92c0>  @ imm = #0x48
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5eac0,false)?)?;
    // 0x5ea74: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5ea78: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5ea7c: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x5ea80: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5ea84: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5ea88: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5ea8c: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5ea90: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x5ea94: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x5ea98: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x34110
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5ea9c: ldr r1, [pc, #0x2c8]        @ 0x956c <entry+0x956c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,712)?)?;
    // 0x5eaa0: ldr r2, [pc, #0x2ac]        @ 0x9554 <entry+0x9554>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,684)?)?;
    // 0x5eaa4: mov r3, #203
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,203)?)?;
    // 0x5eaa8: b 0x5ebc0 <entry+0x93c0>  @ imm = #0x110
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ebc0,false)?)?;
    // 0x5eaac: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x5eab0: bne 0x5eb58 <entry+0x9358>  @ imm = #0xa0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5eb58,false)?)?;
    // 0x5eab4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5eab8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x5eabc: b 0x5eb08 <entry+0x9308>  @ imm = #0x44
    c.emit(arm32::branch(C::Al,c.at as u32,0x5eb08,false)?)?;
    // 0x5eac0: lsl r0, r2, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsl,3)?)?;
    // 0x5eac4: cmn r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,32)?)?;
    // 0x5eac8: blo 0x5eb08 <entry+0x9308>  @ imm = #0x38
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5eb08,false)?)?;
    // 0x5eacc: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5ead0: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5ead4: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x5ead8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5eadc: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5eae0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5eae4: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5eae8: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x5eaec: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x5eaf0: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x34168
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5eaf4: ldr r1, [pc, #0x260]        @ 0x955c <entry+0x955c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,608)?)?;
    // 0x5eaf8: ldr r2, [pc, #0x254]        @ 0x9554 <entry+0x9554>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,596)?)?;
    // 0x5eafc: mov r3, #980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,980)?)?;
    // 0x5eb00: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7168)?)?;
    // 0x5eb04: b 0x5ebc4 <entry+0x93c4>  @ imm = #0xb8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ebc4,false)?)?;
    // 0x5eb08: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x5eb0c: add r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x5eb10: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5eb14: bne 0x5ebe4 <entry+0x93e4>  @ imm = #0xc8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5ebe4,false)?)?;
    // 0x5eb18: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5eb1c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5eb20: beq 0x5ec18 <entry+0x9418>  @ imm = #0xf0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5ec18,false)?)?;
    // 0x5eb24: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5eb28: ldr r1, [pc, #0x234]        @ 0x9564 <entry+0x9564>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,564)?)?;
    // 0x5eb2c: ldr r2, [pc, #0x220]        @ 0x9554 <entry+0x9554>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,544)?)?;
    // 0x5eb30: mov r3, #119
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,119)?)?;
    // 0x5eb34: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x5eb38: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5eb3c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5eb40: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2feb4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5eb44: ldr r1, [pc, #0x20c]        @ 0x9558 <entry+0x9558>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,524)?)?;
    // 0x5eb48: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5eb4c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2fec0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5eb50: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x5eb54: b 0x5ea38 <entry+0x9238>  @ imm = #-0x124
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ea38,false)?)?;
    // 0x5eb58: ldr r1, [pc, #0x1f0]        @ 0x9550 <entry+0x9550>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,496)?)?;
    // 0x5eb5c: ldr r2, [pc, #0x1f0]        @ 0x9554 <entry+0x9554>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,496)?)?;
    // 0x5eb60: mov r3, #222
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,222)?)?;
    // 0x5eb64: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5eb68: orr r3, r3, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7936)?)?;
    // 0x5eb6c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2fee0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5eb70: ldr r1, [pc, #0x1e0]        @ 0x9558 <entry+0x9558>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,480)?)?;
    // 0x5eb74: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5eb78: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2feec
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5eb7c: ldr r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // 0x5eb80: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x5eb84: cmn r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,32)?)?;
    // 0x5eb88: blo 0x5ec9c <entry+0x949c>  @ imm = #0x10c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5ec9c,false)?)?;
    // 0x5eb8c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5eb90: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5eb94: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x5eb98: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5eb9c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5eba0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5eba4: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5eba8: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x5ebac: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x5ebb0: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x34228
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5ebb4: ldr r1, [pc, #0x1a0]        @ 0x955c <entry+0x955c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,416)?)?;
    // 0x5ebb8: ldr r2, [pc, #0x194]        @ 0x9554 <entry+0x9554>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,404)?)?;
    // 0x5ebbc: mov r3, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,230)?)?;
    // 0x5ebc0: orr r3, r3, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7936)?)?;
    // 0x5ebc4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ebc8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ff3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ebcc: ldr r1, [pc, #0x184]        @ 0x9558 <entry+0x9558>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,388)?)?;
    // 0x5ebd0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ebd4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ff48
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ebd8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5ebdc: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x5ebe0: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5ebe4: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x5ebe8: ldr r1, [pc, #0x170]        @ 0x9560 <entry+0x9560>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,368)?)?;
    // 0x5ebec: ldr r2, [pc, #0x160]        @ 0x9554 <entry+0x9554>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,352)?)?;
    // 0x5ebf0: mov r3, #4080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4080)?)?;
    // 0x5ebf4: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x5ebf8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ebfc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5ec00: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ff74
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ec04: ldr r1, [pc, #0x14c]        @ 0x9558 <entry+0x9558>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,332)?)?;
    // 0x5ec08: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ec0c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ff80
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ec10: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x5ec14: b 0x5ea38 <entry+0x9238>  @ imm = #-0x1e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ea38,false)?)?;
    // 0x5ec18: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // 0x5ec1c: sub r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,2)?)?;
    // 0x5ec20: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x5ec24: bhi 0x5ec78 <entry+0x9478>  @ imm = #0x4c
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5ec78,false)?)?;
    // 0x5ec28: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x5ec2c: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x5ec30: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x5ec34)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5EC34:[u32;7]=[0x0005ed44,0x0005ec78,0x0005ec50,0x0005ec78,0x0005eca4,0x0005ecd8,0x0005ed0c];
 for value in INLINE_DATA_5EC34{c.emit(value)?;}
 // Executable A32 block 0x5ec50..0x5ed50.
 c.check(0x5ec50)?;
    // 0x5ec50: ldr r0, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // 0x5ec54: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x5ec58: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5ec5c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5ec60: ldrne r1, [r6, #0x10]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Ne,true,R::R1,R::new(6)?,16)?)?;
    // 0x5ec64: cmpne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5ec68: beq 0x5ea3c <entry+0x923c>  @ imm = #-0x234
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5ea3c,false)?)?;
    // 0x5ec6c: str r0, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,240)?)?;
    // 0x5ec70: mov r0, #244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,244)?)?;
    // 0x5ec74: b 0x5ed3c <entry+0x953c>  @ imm = #0xc0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ed3c,false)?)?;
    // 0x5ec78: ldr r1, [pc, #0xe8]         @ 0x9568 <entry+0x9568>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,232)?)?;
    // 0x5ec7c: ldr r2, [pc, #0xd0]         @ 0x9554 <entry+0x9554>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,208)?)?;
    // 0x5ec80: mov r3, #111
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,111)?)?;
    // 0x5ec84: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ec88: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x5ec8c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x30000
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ec90: ldr r1, [pc, #0xc0]         @ 0x9558 <entry+0x9558>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,192)?)?;
    // 0x5ec94: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ec98: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3000c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ec9c: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x5eca0: b 0x5ea38 <entry+0x9238>  @ imm = #-0x270
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ea38,false)?)?;
    // 0x5eca4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5eca8: add r1, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,32)?)?;
    // 0x5ecac: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5ecb0: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x5ecb4: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5ecb8: bl 0x5ed74 <entry+0x9574>  @ imm = #0xb4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ed74,true)?)?;
    // 0x5ecbc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5ecc0: bne 0x5ebd8 <entry+0x93d8>  @ imm = #-0xf0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5ebd8,false)?)?;
    // 0x5ecc4: ldr r0, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // 0x5ecc8: add r6, r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,16)?)?;
    // 0x5eccc: str r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,248)?)?;
    // 0x5ecd0: mov r0, #252
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,252)?)?;
    // 0x5ecd4: b 0x5ed3c <entry+0x953c>  @ imm = #0x60
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ed3c,false)?)?;
    // 0x5ecd8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5ecdc: add r1, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,32)?)?;
    // 0x5ece0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5ece4: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x5ece8: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5ecec: bl 0x5ed74 <entry+0x9574>  @ imm = #0x80
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ed74,true)?)?;
    // 0x5ecf0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5ecf4: bne 0x5ebd8 <entry+0x93d8>  @ imm = #-0x124
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5ebd8,false)?)?;
    // 0x5ecf8: ldr r0, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // 0x5ecfc: add r6, r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,16)?)?;
    // 0x5ed00: str r0, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,256)?)?;
    // 0x5ed04: mov r0, #260
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,260)?)?;
    // 0x5ed08: b 0x5ed3c <entry+0x953c>  @ imm = #0x2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ed3c,false)?)?;
    // 0x5ed0c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5ed10: add r1, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,32)?)?;
    // 0x5ed14: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5ed18: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x5ed1c: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5ed20: bl 0x5ed74 <entry+0x9574>  @ imm = #0x4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ed74,true)?)?;
    // 0x5ed24: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5ed28: bne 0x5ebd8 <entry+0x93d8>  @ imm = #-0x158
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5ebd8,false)?)?;
    // 0x5ed2c: ldr r0, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // 0x5ed30: add r6, r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,16)?)?;
    // 0x5ed34: str r0, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,264)?)?;
    // 0x5ed38: mov r0, #268
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,268)?)?;
    // 0x5ed3c: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x5ed40: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5ed44: bl 0x20a0 <entry+0xfffffffffffac8a0> @ imm = #-0x5ccac
    c.emit(arm32::branch(C::Al,c.at as u32,0x20a0,true)?)?;
    // 0x5ed48: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5ed4c: b 0x5ebd8 <entry+0x93d8>  @ imm = #-0x17c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ebd8,false)?)?;
 c.check(0x5ed50)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5ED50:[u32;9]=[0x000f0878,0x000f07c7,0x000e2788,0x000f0837,0x000f08ac,0x000f0913,0x000f08ec,0x000f07de,0x000f0799];
 for value in INLINE_DATA_5ED50{c.emit(value)?;}
 // Executable A32 block 0x5ed74..0x5eda4.
 c.check(0x5ed74)?;
    // 0x5ed74: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5ed78: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x5ed7c: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x5ed80: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5ed84: sub r0, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(3)?,1)?)?;
    // 0x5ed88: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x5ed8c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5ed90: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x5ed94: bhi 0x5eea4 <entry+0x96a4>  @ imm = #0x108
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5eea4,false)?)?;
    // 0x5ed98: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x5ed9c: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x5eda0: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x5eda4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5EDA4:[u32;8]=[0x0005ede8,0x0005ee1c,0x0005eea4,0x0005edc4,0x0005eea4,0x0005edc4,0x0005edc4,0x0005edc4];
 for value in INLINE_DATA_5EDA4{c.emit(value)?;}
 // Executable A32 block 0x5edc4..0x5ef10.
 c.check(0x5edc4)?;
    // 0x5edc4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5edc8: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x5edcc: cmn r1, #-2147483647
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,2147483649)?)?;
    // 0x5edd0: beq 0x5ee48 <entry+0x9648>  @ imm = #0x70
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5ee48,false)?)?;
    // 0x5edd4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5edd8: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5eddc: blo 0x5edc8 <entry+0x95c8>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5edc8,false)?)?;
    // 0x5ede0: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5ede4: b 0x5ee98 <entry+0x9698>  @ imm = #0xac
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ee98,false)?)?;
    // 0x5ede8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5edec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5edf0: bne 0x5edc4 <entry+0x95c4>  @ imm = #-0x34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5edc4,false)?)?;
    // 0x5edf4: ldr r0, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    // 0x5edf8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5edfc: bne 0x5eec4 <entry+0x96c4>  @ imm = #0xc0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5eec4,false)?)?;
    // 0x5ee00: ldr r2, [pc, #0x114]        @ 0x971c <entry+0x971c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,276)?)?;
    // 0x5ee04: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x5ee08: mov r1, #151
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,151)?)?;
    // 0x5ee0c: movne r6, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5ee10: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x5ee14: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5d66c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5ee18: b 0x5ede0 <entry+0x95e0>  @ imm = #-0x40
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ede0,false)?)?;
    // 0x5ee1c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5ee20: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5ee24: beq 0x5eeb0 <entry+0x96b0>  @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5eeb0,false)?)?;
    // 0x5ee28: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5ee2c: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x5ee30: cmn r1, #-2147483647
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,2147483649)?)?;
    // 0x5ee34: beq 0x5ee48 <entry+0x9648>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5ee48,false)?)?;
    // 0x5ee38: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5ee3c: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x5ee40: blo 0x5ee2c <entry+0x962c>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5ee2c,false)?)?;
    // 0x5ee44: b 0x5ede0 <entry+0x95e0>  @ imm = #-0x6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ede0,false)?)?;
    // 0x5ee48: ldr r2, [r5, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(5)?,R::R0,Shift::Lsl,2)?)?;
    // 0x5ee4c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5ee50: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5ee54: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x5ee58: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5ee5c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5ee60: stm sp, {r2, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5ee64: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5ee68: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x5ee6c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x344e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5ee70: ldr r1, [pc, #0xac]         @ 0x9724 <entry+0x9724>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,172)?)?;
    // 0x5ee74: ldr r2, [pc, #0x98]         @ 0x9714 <entry+0x9714>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,152)?)?;
    // 0x5ee78: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ee7c: mov r3, #181
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,181)?)?;
    // 0x5ee80: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x301f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ee84: ldr r1, [pc, #0x8c]         @ 0x9718 <entry+0x9718>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,140)?)?;
    // 0x5ee88: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ee8c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x30200
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ee90: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5ee94: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x5ee98: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5ee9c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x5eea0: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5eea4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5eea8: bl 0x5aae0 <entry+0x52e0>  @ imm = #-0x43d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5aae0,true)?)?;
    // 0x5eeac: b 0x5edc4 <entry+0x95c4>  @ imm = #-0xf0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5edc4,false)?)?;
    // 0x5eeb0: ldr r1, [pc, #0x68]         @ 0x9720 <entry+0x9720>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,104)?)?;
    // 0x5eeb4: ldr r2, [pc, #0x58]         @ 0x9714 <entry+0x9714>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,88)?)?;
    // 0x5eeb8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5eebc: mov r3, #121
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,121)?)?;
    // 0x5eec0: b 0x5eed4 <entry+0x96d4>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5eed4,false)?)?;
    // 0x5eec4: ldr r1, [pc, #0x44]         @ 0x9710 <entry+0x9710>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,68)?)?;
    // 0x5eec8: ldr r2, [pc, #0x44]         @ 0x9714 <entry+0x9714>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,68)?)?;
    // 0x5eecc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5eed0: mov r3, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,156)?)?;
    // 0x5eed4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x30248
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5eed8: ldr r1, [pc, #0x38]         @ 0x9718 <entry+0x9718>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,56)?)?;
    // 0x5eedc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5eee0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x30254
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5eee4: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5eee8: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5eeec: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5eef0: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x5eef4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5eef8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5eefc: stm sp, {r2, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5ef00: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5ef04: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x5ef08: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x34580
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5ef0c: b 0x5ee98 <entry+0x9698>  @ imm = #-0x7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ee98,false)?)?;
 c.check(0x5ef10)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5EF10:[u32;6]=[0x000f1541,0x000f152f,0x000e2788,0x000ee470,0x000f150d,0x000f1563];
 for value in INLINE_DATA_5EF10{c.emit(value)?;}
 // Executable A32 block 0x5ef28..0x5f000.
 c.check(0x5ef28)?;
    // 0x5ef28: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5ef2c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x5ef30: sub sp, sp, #156
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,156)?)?;
    // 0x5ef34: ldr r10, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R1,12)?)?;
    // 0x5ef38: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5ef3c: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x5ef40: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x5ef44: add r2, r3, r10, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::new(10)?,Shift::Lsl,2,false)?)?;
    // 0x5ef48: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x5ef4c: bne 0x5f018 <entry+0x9818>  @ imm = #0xc4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f018,false)?)?;
    // 0x5ef50: ldr r6, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,40)?)?;
    // 0x5ef54: ldr r0, [pc, #0xe30]        @ 0xa58c <entry+0xa58c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,3632)?)?;
    // 0x5ef58: mov r12, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,8192)?)?;
    // 0x5ef5c: orr r12, r12, #2097152
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,2097152)?)?;
    // 0x5ef60: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x5ef64: str r10, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,12)?)?;
    // 0x5ef68: add r7, r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(6)?,16)?)?;
    // 0x5ef6c: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x5ef70: str r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // 0x5ef74: add r0, r4, #812
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,812)?)?;
    // 0x5ef78: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x5ef7c: add r3, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,16)?)?;
    // 0x5ef80: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x5ef84: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x5ef88: add r0, r4, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,60)?)?;
    // 0x5ef8c: add r8, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,5120)?)?;
    // 0x5ef90: add r0, r4, #225
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,225)?)?;
    // 0x5ef94: add lr, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R0,4864)?)?;
    // 0x5ef98: add r0, r4, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,84)?)?;
    // 0x5ef9c: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5efa0: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // 0x5efa4: add r0, r4, #1104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1104)?)?;
    // 0x5efa8: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5efac: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x5efb0: add r0, r4, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,76)?)?;
    // 0x5efb4: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5efb8: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // 0x5efbc: add r0, r4, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,88)?)?;
    // 0x5efc0: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5efc4: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // 0x5efc8: sub r0, r11, #88
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,88)?)?;
    // 0x5efcc: add r2, r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,36)?)?;
    // 0x5efd0: add r9, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R0,12)?)?;
    // 0x5efd4: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // 0x5efd8: add r2, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,24)?)?;
    // 0x5efdc: add r0, r4, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,640)?)?;
    // 0x5efe0: str r2, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,48)?)?;
    // 0x5efe4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x5efe8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x5efec: ldr r0, [pc, #0xd9c]        @ 0xa590 <entry+0xa590>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,3484)?)?;
    // 0x5eff0: str r8, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,36)?)?;
    // 0x5eff4: str lr, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,72)?)?;
    // 0x5eff8: str r6, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,80)?)?;
    // 0x5effc: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
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
