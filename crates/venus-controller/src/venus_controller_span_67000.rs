// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x67000,0x68000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x67000;
pub const END:usize=0x68000;
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
fn emit_rrx(cond:C,rd:R,rm:R)->Result<u32,String>{
 if [rd,rm].contains(&R::PC) {return Err("invalid A32 RRX operands".into())}
 Ok(arm32::mov_register(cond,rd,rm) | (3<<5))
}
fn emit_adc_register(cond:C,rd:R,rn:R,rm:R,flags:bool)->Result<u32,String>{
 Ok(arm32::data_register(cond,Op::Add,rd,rn,rm,Shift::Lsl,0,flags)? | (1<<21))
}
fn emit_adc_immediate(cond:C,rd:R,rn:R,value:u32,flags:bool)->Result<u32,String>{
 Ok(arm32::data_immediate_with_flags(cond,Op::Add,rd,rn,value,flags)? | (1<<21))
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x67000..0x67094.
 c.check(0x67000)?;
    // 0x67000: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x67004: sub r5, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67008: sub r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x6700c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x67010: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x67014: add r9, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67018: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x6701c: cmp r5, r9
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x67020: bls 0x67060 <entry+0x11860> @ imm = #0x38
    c.emit(arm32::branch(C::Ls,c.at as u32,0x67060,false)?)?;
    // 0x67024: ldr r2, [pc, #0x74]         @ 0x118a0 <entry+0x118a0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,116)?)?;
    // 0x67028: mov r1, #79
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,79)?)?;
    // 0x6702c: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x67030: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x65888
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x67034: mov r5, r9
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x67038: ldr r9, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,64)?)?;
    // 0x6703c: b 0x67078 <entry+0x11878> @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x67078,false)?)?;
    // 0x67040: sub r0, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x67044: add r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x67048: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x6704c: sub r5, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67050: cmp r0, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x67054: movhi r4, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x67058: movls r5, r0
    c.emit(arm32::mov_register(C::Ls,R::new(5)?,R::R0))?;
    // 0x6705c: b 0x67078 <entry+0x11878> @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x67078,false)?)?;
    // 0x67060: ldr r2, [pc, #0x38]         @ 0x118a0 <entry+0x118a0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,56)?)?;
    // 0x67064: mov r1, #73
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,73)?)?;
    // 0x67068: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x6706c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x658c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x67070: ldr r9, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,64)?)?;
    // 0x67074: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x67078: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x6707c: ldr r0, [r1, #0x144]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::R1,324)?)?;
    // 0x67080: add r1, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x67084: str r4, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,12)?)?;
    // 0x67088: sub r1, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(5)?,1)?)?;
    // 0x6708c: ldr r4, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-64)?)?;
    // 0x67090: b 0x66bb4 <entry+0x113b4> @ imm = #-0x4e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x66bb4,false)?)?;
 c.check(0x67094)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_67094:[u32;4]=[0x000f18bb,0x000f18f1,0x000e2788,0x000f1748];
 for value in INLINE_DATA_67094{c.emit(value)?;}
 // Executable A32 block 0x670a4..0x671dc.
 c.check(0x670a4)?;
    // 0x670a4: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x670a8: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x670ac: ldr lr, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R1,4)?)?;
    // 0x670b0: mov r12, r0
    c.emit(arm32::mov_register(C::Al,R::R12,R::R0))?;
    // 0x670b4: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x670b8: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x670bc: popeq {r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x670c0: add r2, r1, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,72)?)?;
    // 0x670c4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x670c8: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // 0x670cc: cmp r3, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x670d0: bne 0x670f0 <entry+0x118f0> @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x670f0,false)?)?;
    // 0x670d4: ldr r3, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,8)?)?;
    // 0x670d8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x670dc: beq 0x670f0 <entry+0x118f0> @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x670f0,false)?)?;
    // 0x670e0: ldrb r3, [r2, #0x2a]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,42)?)?;
    // 0x670e4: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x670e8: moveq r0, r1
    c.emit(arm32::mov_register(C::Eq,R::R0,R::R1))?;
    // 0x670ec: popeq {r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x670f0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x670f4: add r2, r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,72)?)?;
    // 0x670f8: cmp r1, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // 0x670fc: blo 0x670c8 <entry+0x118c8> @ imm = #-0x3c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x670c8,false)?)?;
    // 0x67100: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x67104: ldr r1, [r0, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,368)?)?;
    // 0x67108: ldrb r2, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,1)?)?;
    // 0x6710c: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x67110: ldrbeq r2, [r1, #0x1dd]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R2,R::R1,477)?)?;
    // 0x67114: cmpeq r2, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x67118: bne 0x6713c <entry+0x1193c> @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6713c,false)?)?;
    // 0x6711c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x67120: str r2, [r0, #0x674]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1652)?)?;
    // 0x67124: str r2, [r0, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1716)?)?;
    // 0x67128: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // 0x6712c: str r2, [r0, #0x278]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,632)?)?;
    // 0x67130: str r2, [r0, #0x27c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,636)?)?;
    // 0x67134: str r2, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,640)?)?;
    // 0x67138: str r2, [r0, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,644)?)?;
    // 0x6713c: ldr r2, [r0, #0x674]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,1652)?)?;
    // 0x67140: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x67144: bne 0x671d8 <entry+0x119d8> @ imm = #0x8c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x671d8,false)?)?;
    // 0x67148: ldr r2, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,80)?)?;
    // 0x6714c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x67150: ldrbne r1, [r1, #0xa]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::R1,10)?)?;
    // 0x67154: cmpne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x67158: bne 0x671d8 <entry+0x119d8> @ imm = #0x78
    c.emit(arm32::branch(C::Ne,c.at as u32,0x671d8,false)?)?;
    // 0x6715c: ldr r1, [r0, #0x684]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1668)?)?;
    // 0x67160: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x67164: bne 0x671a4 <entry+0x119a4> @ imm = #0x38
    c.emit(arm32::branch(C::Ne,c.at as u32,0x671a4,false)?)?;
    // 0x67168: ldr r1, [r0, #0x670]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1648)?)?;
    // 0x6716c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x67170: bxne lr
    c.emit(arm32::bx(C::Ne,R::LR))?;
    // 0x67174: ldr r12, [r0, #0x680]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,1664)?)?;
    // 0x67178: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x6717c: beq 0x671b8 <entry+0x119b8> @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x671b8,false)?)?;
    // 0x67180: add r3, r0, #632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,632)?)?;
    // 0x67184: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x67188: ldr r1, [r3, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // 0x6718c: cmn r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // 0x67190: beq 0x671c0 <entry+0x119c0> @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x671c0,false)?)?;
    // 0x67194: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x67198: cmp r2, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // 0x6719c: blo 0x67188 <entry+0x11988> @ imm = #-0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x67188,false)?)?;
    // 0x671a0: b 0x671cc <entry+0x119cc> @ imm = #0x24
    c.emit(arm32::branch(C::Al,c.at as u32,0x671cc,false)?)?;
    // 0x671a4: ldr r2, [pc, #0x30]         @ 0x119dc <entry+0x119dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,48)?)?;
    // 0x671a8: mov r1, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33)?)?;
    // 0x671ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x671b0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x671b4: b 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x65a0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,false)?)?;
    // 0x671b8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x671bc: b 0x671cc <entry+0x119cc> @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x671cc,false)?)?;
    // 0x671c0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x671c4: str r1, [r0, #0x670]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1648)?)?;
    // 0x671c8: str r2, [r0, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1672)?)?;
    // 0x671cc: cmp r2, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // 0x671d0: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x671d4: streq r1, [r0, #0x674]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,1652)?)?;
    // 0x671d8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
 c.check(0x671dc)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_671DC:[u32;1]=[0x000f1748];
 for value in INLINE_DATA_671DC{c.emit(value)?;}
 // Executable A32 block 0x671e0..0x6736c.
 c.check(0x671e0)?;
    // 0x671e0: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x671e4: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x671e8: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x671ec: ldr r3, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,44)?)?;
    // 0x671f0: mov r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,255)?)?;
    // 0x671f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x671f8: orr r12, r12, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,65280)?)?;
    // 0x671fc: ldr r8, [r3, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(3)?,36)?)?;
    // 0x67200: ldr lr, [r8, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(8)?,584)?)?;
    // 0x67204: str r12, [r3, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,972)?)?;
    // 0x67208: str r0, [r8, #0x67c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1660)?)?;
    // 0x6720c: ldr r0, [r8, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1656)?)?;
    // 0x67210: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x67214: ldrne r0, [r3, #0x190]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(3)?,400)?)?;
    // 0x67218: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6721c: beq 0x672a0 <entry+0x11aa0> @ imm = #0x7c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x672a0,false)?)?;
    // 0x67220: ldr r5, [r8, #0x68c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,1676)?)?;
    // 0x67224: add r6, r8, #632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(8)?,632)?)?;
    // 0x67228: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x6722c: mvn r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(4)?,R::R0,0)?)?;
    // 0x67230: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x67234: tst r5, r7, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,R::R2,false)?)?;
    // 0x67238: beq 0x6724c <entry+0x11a4c> @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6724c,false)?)?;
    // 0x6723c: ldr r0, [r6, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::R2,Shift::Lsl,2)?)?;
    // 0x67240: cmp r0, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x67244: movgt r4, r0
    c.emit(arm32::mov_register(C::Gt,R::new(4)?,R::R0))?;
    // 0x67248: strgt r2, [r8, #0x690]
    c.emit(arm32::transfer_word(C::Gt,false,R::R2,R::new(8)?,1680)?)?;
    // 0x6724c: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x67250: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x67254: bne 0x67234 <entry+0x11a34> @ imm = #-0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67234,false)?)?;
    // 0x67258: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x6725c: beq 0x672b4 <entry+0x11ab4> @ imm = #0x50
    c.emit(arm32::branch(C::Eq,c.at as u32,0x672b4,false)?)?;
    // 0x67260: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x67264: bge 0x672b4 <entry+0x11ab4> @ imm = #0x48
    c.emit(arm32::branch(C::Ge,c.at as u32,0x672b4,false)?)?;
    // 0x67268: ldr r1, [pc, #0xfc]         @ 0x11b6c <entry+0x11b6c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,252)?)?;
    // 0x6726c: ldr r2, [pc, #0xfc]         @ 0x11b70 <entry+0x11b70>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,252)?)?;
    // 0x67270: mov r3, #81
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,81)?)?;
    // 0x67274: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x67278: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x6727c: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x67280: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x385f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x67284: ldr r1, [pc, #0xe8]         @ 0x11b74 <entry+0x11b74>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,232)?)?;
    // 0x67288: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x6728c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x38600
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x67290: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x67294: str r0, [r8, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1656)?)?;
    // 0x67298: str r0, [r8, #0x67c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1660)?)?;
    // 0x6729c: b 0x67364 <entry+0x11b64> @ imm = #0xc0
    c.emit(arm32::branch(C::Al,c.at as u32,0x67364,false)?)?;
    // 0x672a0: add r0, lr, lr, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::LR,Shift::Lsl,3,false)?)?;
    // 0x672a4: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x672a8: add r0, r8, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x672ac: str r1, [r0, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,680)?)?;
    // 0x672b0: b 0x67364 <entry+0x11b64> @ imm = #0xac
    c.emit(arm32::branch(C::Al,c.at as u32,0x67364,false)?)?;
    // 0x672b4: ldr r0, [r8, #0x690]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1680)?)?;
    // 0x672b8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x672bc: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x672c0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x672c4: orr r0, r12, r2, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::R12,R::R2,Shift::Lsl,R::R0,false)?)?;
    // 0x672c8: str r0, [r3, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,972)?)?;
    // 0x672cc: str r2, [r8, #0x67c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,1660)?)?;
    // 0x672d0: bne 0x67364 <entry+0x11b64> @ imm = #0x8c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67364,false)?)?;
    // 0x672d4: ldr r1, [r8, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,588)?)?;
    // 0x672d8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x672dc: beq 0x67364 <entry+0x11b64> @ imm = #0x80
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67364,false)?)?;
    // 0x672e0: add r0, lr, lr, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::LR,Shift::Lsl,3,false)?)?;
    // 0x672e4: add r3, r8, #656
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(8)?,656)?)?;
    // 0x672e8: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x672ec: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x672f0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x672f4: add r0, r8, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x672f8: add r2, r0, #656
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,656)?)?;
    // 0x672fc: ldr r6, [r3, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,8)?)?;
    // 0x67300: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x67304: beq 0x67354 <entry+0x11b54> @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67354,false)?)?;
    // 0x67308: cmp r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,3)?)?;
    // 0x6730c: bne 0x67320 <entry+0x11b20> @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67320,false)?)?;
    // 0x67310: ldr r7, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x67314: cmp r7, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x67318: strbeq lr, [r3, #0x2a]
    c.emit(arm32::transfer_byte(C::Eq,false,R::LR,R::new(3)?,42)?)?;
    // 0x6731c: b 0x67324 <entry+0x11b24> @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x67324,false)?)?;
    // 0x67320: ldr r7, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x67324: cmp r7, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x67328: ble 0x67354 <entry+0x11b54> @ imm = #0x24
    c.emit(arm32::branch(C::Le,c.at as u32,0x67354,false)?)?;
    // 0x6732c: ldr r0, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    // 0x67330: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x67334: bge 0x67354 <entry+0x11b54> @ imm = #0x18
    c.emit(arm32::branch(C::Ge,c.at as u32,0x67354,false)?)?;
    // 0x67338: ldr r0, [r8, #0x694]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1684)?)?;
    // 0x6733c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x67340: bne 0x67354 <entry+0x11b54> @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67354,false)?)?;
    // 0x67344: cmp r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,3)?)?;
    // 0x67348: cmpne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x6734c: bne 0x67354 <entry+0x11b54> @ imm = #0x0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67354,false)?)?;
    // 0x67350: strb r12, [r3, #0x2a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(3)?,42)?)?;
    // 0x67354: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x67358: add r3, r3, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,72)?)?;
    // 0x6735c: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x67360: blo 0x672fc <entry+0x11afc> @ imm = #-0x6c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x672fc,false)?)?;
    // 0x67364: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x67368: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x6736c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_6736C:[u32;3]=[0x000f190b,0x000f193e,0x000e2788];
 for value in INLINE_DATA_6736C{c.emit(value)?;}
 // Executable A32 block 0x67378..0x67400.
 c.check(0x67378)?;
    // 0x67378: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x6737c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x67380: sub r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x67384: sub r1, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x67388: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x6738c: cmp r1, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x67390: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x67394: mov r5, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,127)?)?;
    // 0x67398: cmp r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,127)?)?;
    // 0x6739c: mvn r6, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(6)?,R::R0,127)?)?;
    // 0x673a0: movge r1, r5
    c.emit(arm32::mov_register(C::Ge,R::R1,R::new(5)?))?;
    // 0x673a4: cmn r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,128)?)?;
    // 0x673a8: movle r1, r6
    c.emit(arm32::mov_register(C::Le,R::R1,R::new(6)?))?;
    // 0x673ac: asr r0, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Asr,1)?)?;
    // 0x673b0: add r0, r0, r1, asr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Asr,31,false)?)?;
    // 0x673b4: eor r0, r0, r1, asr #31
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R0,R::R1,Shift::Asr,31,false)?)?;
    // 0x673b8: add r0, r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16384)?)?;
    // 0x673bc: blx 0x82be4 <entry+0x2d3e4> @ imm = #0x1b820
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82be4)?)?;
    // 0x673c0: cmp r4, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,127)?)?;
    // 0x673c4: movlt r5, r4
    c.emit(arm32::mov_register(C::Lt,R::new(5)?,R::new(4)?))?;
    // 0x673c8: cmn r5, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(5)?,128)?)?;
    // 0x673cc: movle r5, r6
    c.emit(arm32::mov_register(C::Le,R::new(5)?,R::new(6)?))?;
    // 0x673d0: mul r1, r0, r5
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(5)?)?)?;
    // 0x673d4: add r0, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,32)?)?;
    // 0x673d8: asr r1, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Asr,6)?)?;
    // 0x673dc: ldr r0, [pc, #0x1c]         @ 0x11c00 <entry+0x11c00>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,28)?)?;
    // 0x673e0: cmn r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,4096)?)?;
    // 0x673e4: blt 0x673fc <entry+0x11bfc> @ imm = #0x10
    c.emit(arm32::branch(C::Lt,c.at as u32,0x673fc,false)?)?;
    // 0x673e8: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x673ec: movge r0, #255
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x673f0: orrge r0, r0, #3840
    c.emit(arm32::data_immediate(C::Ge,Op::Orr,R::R0,R::R0,3840)?)?;
    // 0x673f4: popge {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Ge,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x673f8: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x673fc: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x67400)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_67400:[u32;1]=[0xfffff000];
 for value in INLINE_DATA_67400{c.emit(value)?;}
 // Executable A32 block 0x67404..0x67734.
 c.check(0x67404)?;
    // 0x67404: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x67408: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x6740c: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x67410: ldr r0, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,12)?)?;
    // 0x67414: add r6, r1, #51
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,51)?)?;
    // 0x67418: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x6741c: ldr r9, [r11, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,16)?)?;
    // 0x67420: ldr r8, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,8)?)?;
    // 0x67424: mov r4, r3
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x67428: add r2, r6, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R0,Shift::Lsl,1,false)?)?;
    // 0x6742c: ldrsb r2, [r2]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R2,0)?)?;
    // 0x67430: add r2, r2, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x67434: add r2, r7, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,3,false)?)?;
    // 0x67438: ldrb r2, [r2, #0x70]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,112)?)?;
    // 0x6743c: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x67440: beq 0x6749c <entry+0x11c9c> @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6749c,false)?)?;
    // 0x67444: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x67448: bne 0x674ec <entry+0x11cec> @ imm = #0x9c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x674ec,false)?)?;
    // 0x6744c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x67450: addne r6, r1, #53
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(6)?,R::R1,53)?)?;
    // 0x67454: ldrsb r0, [r6]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::new(6)?,0)?)?;
    // 0x67458: add r1, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6745c: add r1, r7, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,3,false)?)?;
    // 0x67460: ldr r3, [r1, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,120)?)?;
    // 0x67464: cmn r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // 0x67468: beq 0x674f8 <entry+0x11cf8> @ imm = #0x88
    c.emit(arm32::branch(C::Eq,c.at as u32,0x674f8,false)?)?;
    // 0x6746c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x67470: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x67474: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x67478: bl 0x67378 <entry+0x11b78> @ imm = #-0x108
    c.emit(arm32::branch(C::Al,c.at as u32,0x67378,true)?)?;
    // 0x6747c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x67480: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x67484: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x67488: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x6748c: moveq r0, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x67490: strh r0, [r9]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(9)?,0)?)?;
    // 0x67494: ldrb r0, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x67498: b 0x674fc <entry+0x11cfc> @ imm = #0x5c
    c.emit(arm32::branch(C::Al,c.at as u32,0x674fc,false)?)?;
    // 0x6749c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x674a0: addne r6, r1, #53
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(6)?,R::R1,53)?)?;
    // 0x674a4: ldrsb r0, [r6]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::new(6)?,0)?)?;
    // 0x674a8: add r1, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x674ac: add r1, r7, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,3,false)?)?;
    // 0x674b0: ldr r3, [r1, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,120)?)?;
    // 0x674b4: cmn r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // 0x674b8: beq 0x67538 <entry+0x11d38> @ imm = #0x78
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67538,false)?)?;
    // 0x674bc: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x674c0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x674c4: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x674c8: bl 0x67378 <entry+0x11b78> @ imm = #-0x158
    c.emit(arm32::branch(C::Al,c.at as u32,0x67378,true)?)?;
    // 0x674cc: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x674d0: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x674d4: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x674d8: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x674dc: moveq r0, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x674e0: strh r0, [r9]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(9)?,0)?)?;
    // 0x674e4: ldrb r0, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x674e8: b 0x6753c <entry+0x11d3c> @ imm = #0x4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x6753c,false)?)?;
    // 0x674ec: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x674f0: strh r0, [r9]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(9)?,0)?)?;
    // 0x674f4: b 0x67584 <entry+0x11d84> @ imm = #0x88
    c.emit(arm32::branch(C::Al,c.at as u32,0x67584,false)?)?;
    // 0x674f8: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x674fc: lsl r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,24)?)?;
    // 0x67500: asr r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,24)?)?;
    // 0x67504: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x67508: add r0, r7, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6750c: ldr r3, [r0, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,124)?)?;
    // 0x67510: cmn r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // 0x67514: beq 0x67588 <entry+0x11d88> @ imm = #0x6c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67588,false)?)?;
    // 0x67518: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x6751c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x67520: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x67524: bl 0x67378 <entry+0x11b78> @ imm = #-0x1b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x67378,true)?)?;
    // 0x67528: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x6752c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x67530: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x67534: b 0x6757c <entry+0x11d7c> @ imm = #0x40
    c.emit(arm32::branch(C::Al,c.at as u32,0x6757c,false)?)?;
    // 0x67538: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x6753c: lsl r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,24)?)?;
    // 0x67540: asr r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,24)?)?;
    // 0x67544: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x67548: add r0, r7, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6754c: ldr r3, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,128)?)?;
    // 0x67550: cmn r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // 0x67554: popeq {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x67558: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x6755c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x67560: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x67564: bl 0x67378 <entry+0x11b78> @ imm = #-0x1f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x67378,true)?)?;
    // 0x67568: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x6756c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x67570: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x67574: lsl r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
    // 0x67578: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x6757c: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x67580: moveq r0, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x67584: strh r0, [r9, #2]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(9)?,2)?)?;
    // 0x67588: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x6758c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x67590: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x67594: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x67598: mul r2, r0, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R0)?)?;
    // 0x6759c: orrs r7, r4, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,true)?)?;
    // 0x675a0: umull r9, r8, r2, r1
    c.emit(emit_umull(C::Al,R::new(9)?,R::new(8)?,R::R2,R::R1)?)?;
    // 0x675a4: movne r7, r3
    c.emit(arm32::mov_register(C::Ne,R::new(7)?,R::new(3)?))?;
    // 0x675a8: moveq r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x675ac: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x675b0: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x675b4: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x675b8: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x675bc: bl 0x8187c <entry+0x2c07c> @ imm = #0x1a2b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x8187c,true)?)?;
    // 0x675c0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x675c4: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x675c8: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x675cc: mov r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,32)?)?;
    // 0x675d0: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // 0x675d4: movne r6, r1
    c.emit(arm32::mov_register(C::Ne,R::new(6)?,R::R1))?;
    // 0x675d8: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x675dc: lsr r2, r6, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(6)?,Shift::Lsr,16)?)?;
    // 0x675e0: orr r2, r2, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x675e4: orr r12, r2, r3, lsr #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R12,R::R2,R::new(3)?,Shift::Lsr,16,false)?)?;
    // 0x675e8: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x675ec: movne r6, r2
    c.emit(arm32::mov_register(C::Ne,R::new(6)?,R::R2))?;
    // 0x675f0: subs r2, r9, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(9)?,R::new(4)?,Shift::Lsl,0,true)?)?;
    // 0x675f4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x675f8: sbcs r2, r8, r7
    c.emit(arm32::data_register(C::Al,Op::Sbc,R::R2,R::new(8)?,R::new(7)?,Shift::Lsl,0,true)?)?;
    // 0x675fc: mov r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,10)?)?;
    // 0x67600: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x67604: movlo r5, #31
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(5)?,R::R0,31)?)?;
    // 0x67608: movlo r2, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x6760c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x67610: mvnne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mvn,R::R2,R::R0,0)?)?;
    // 0x67614: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x67618: movne r2, r5
    c.emit(arm32::mov_register(C::Ne,R::R2,R::new(5)?))?;
    // 0x6761c: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x67620: addne r2, r2, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,16)?)?;
    // 0x67624: lsrne r3, r3, #16
    c.emit(arm32::mov_shifted_register(C::Ne,R::new(3)?,R::new(3)?,Shift::Lsr,16)?)?;
    // 0x67628: rsbs r5, r6, #255
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsb,R::new(5)?,R::new(6)?,255,true)?)?;
    // 0x6762c: rscs r5, r3, #0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsc,R::new(5)?,R::new(3)?,0,true)?)?;
    // 0x67630: movlo r4, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x67634: addlo r2, r2, #8
    c.emit(arm32::data_immediate(C::Cc,Op::Add,R::R2,R::R2,8)?)?;
    // 0x67638: lsr r5, r6, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Lsr,8)?)?;
    // 0x6763c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x67640: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x67644: orrne r6, r5, r3, lsl #24
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(6)?,R::new(5)?,R::new(3)?,Shift::Lsl,24,false)?)?;
    // 0x67648: lsrne r3, r3, #8
    c.emit(arm32::mov_shifted_register(C::Ne,R::new(3)?,R::new(3)?,Shift::Lsr,8)?)?;
    // 0x6764c: rsbs r5, r6, #15
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsb,R::new(5)?,R::new(6)?,15,true)?)?;
    // 0x67650: rscs r5, r3, #0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsc,R::new(5)?,R::new(3)?,0,true)?)?;
    // 0x67654: movlo r4, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x67658: addlo r2, r2, #4
    c.emit(arm32::data_immediate(C::Cc,Op::Add,R::R2,R::R2,4)?)?;
    // 0x6765c: lsr r5, r6, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Lsr,4)?)?;
    // 0x67660: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x67664: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x67668: orrne r6, r5, r3, lsl #28
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(6)?,R::new(5)?,R::new(3)?,Shift::Lsl,28,false)?)?;
    // 0x6766c: lsrne r3, r3, #4
    c.emit(arm32::mov_shifted_register(C::Ne,R::new(3)?,R::new(3)?,Shift::Lsr,4)?)?;
    // 0x67670: rsbs r5, r6, #3
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsb,R::new(5)?,R::new(6)?,3,true)?)?;
    // 0x67674: rscs r5, r3, #0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsc,R::new(5)?,R::new(3)?,0,true)?)?;
    // 0x67678: movlo r4, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x6767c: addlo r2, r2, #2
    c.emit(arm32::data_immediate(C::Cc,Op::Add,R::R2,R::R2,2)?)?;
    // 0x67680: lsr r5, r6, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Lsr,2)?)?;
    // 0x67684: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x67688: orrne r6, r5, r3, lsl #30
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(6)?,R::new(5)?,R::new(3)?,Shift::Lsl,30,false)?)?;
    // 0x6768c: lsrne r3, r3, #2
    c.emit(arm32::mov_shifted_register(C::Ne,R::new(3)?,R::new(3)?,Shift::Lsr,2)?)?;
    // 0x67690: rsbs r6, r6, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsb,R::new(6)?,R::new(6)?,1,true)?)?;
    // 0x67694: rscs r3, r3, #0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsc,R::new(3)?,R::new(3)?,0,true)?)?;
    // 0x67698: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x6769c: addlo r2, r2, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Add,R::R2,R::R2,1)?)?;
    // 0x676a0: rsb r3, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(3)?,R::R2,32)?)?;
    // 0x676a4: sub r4, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R2,32)?)?;
    // 0x676a8: lsr r5, r6, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(5)?,R::R0,R::new(6)?,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x676ac: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x676b0: lslge r5, r6, r4
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::new(5)?,R::R0,R::new(6)?,Shift::Lsl,R::new(4)?,false)?)?;
    // 0x676b4: lsl r6, r6, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(6)?,R::R0,R::new(6)?,Shift::Lsl,R::R2,false)?)?;
    // 0x676b8: movge r6, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x676bc: lsrs r12, r5, #1
    c.emit(arm32::data_register(C::Al,Op::Mov,R::R12,R::R0,R::new(5)?,Shift::Lsr,1,true)?)?;
    // 0x676c0: rrx r8, r6
    c.emit(emit_rrx(C::Al,R::new(8)?,R::new(6)?)?)?;
    // 0x676c4: umull r5, r6, r0, r7
    c.emit(emit_umull(C::Al,R::new(5)?,R::new(6)?,R::R0,R::new(7)?)?)?;
    // 0x676c8: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x676cc: adds r1, r8, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(8)?,R::new(5)?,Shift::Lsl,0,true)?)?;
    // 0x676d0: add r0, r6, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,1,false)?)?;
    // 0x676d4: adc r0, r12, r0
    c.emit(emit_adc_register(C::Al,R::R0,R::R12,R::R0,false)?)?;
    // 0x676d8: lsr r1, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::R2,false)?)?;
    // 0x676dc: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x676e0: orr r1, r1, r0, lsl r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x676e4: lsrge r1, r0, r4
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R1,R::R0,R::R0,Shift::Lsr,R::new(4)?,false)?)?;
    // 0x676e8: subs r0, r1, #10
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R0,R::R1,10,true)?)?;
    // 0x676ec: ldr r1, [pc, #0x40]         @ 0x11f34 <entry+0x11f34>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,64)?)?;
    // 0x676f0: movls r0, lr
    c.emit(arm32::mov_register(C::Ls,R::R0,R::LR))?;
    // 0x676f4: add r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // 0x676f8: mov r1, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1000)?)?;
    // 0x676fc: ldrh r0, [r0]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::R0,0)?)?;
    // 0x67700: mla r3, r2, r1, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R2,R::R1,R::R0)?)?;
    // 0x67704: mov r0, #45
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,45)?)?;
    // 0x67708: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x6770c: mul r1, r3, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(3)?,R::R0)?)?;
    // 0x67710: add r0, r1, #50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,50)?)?;
    // 0x67714: ldr r1, [pc, #0x1c]         @ 0x11f38 <entry+0x11f38>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,28)?)?;
    // 0x67718: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x6771c: ldr r1, [pc, #0x18]         @ 0x11f3c <entry+0x11f3c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,24)?)?;
    // 0x67720: mvn r0, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,2147483663)?)?;
    // 0x67724: and r0, r0, r3, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsr,1,false)?)?;
    // 0x67728: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x6772c: lsr r0, r3, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,6)?)?;
    // 0x67730: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x67734)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_67734:[u32;3]=[0x000f1cb0,0x51eb851f,0x10624dd3];
 for value in INLINE_DATA_67734{c.emit(value)?;}
 // Executable A32 block 0x67740..0x67780.
 c.check(0x67740)?;
    // 0x67740: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x67744: ldr r0, [pc, #0x34]         @ 0x11f80 <entry+0x11f80>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,52)?)?;
    // 0x67748: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x6774c: smull r2, r3, r1, r0
    c.emit(emit_smull(C::Al,R::R2,R::new(3)?,R::R1,R::R0)?)?;
    // 0x67750: add r2, r3, r3, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::new(3)?,Shift::Lsr,31,false)?)?;
    // 0x67754: ldr r3, [pc, #0x28]         @ 0x11f84 <entry+0x11f84>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,40)?)?;
    // 0x67758: add r0, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x6775c: sub r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // 0x67760: ldr r0, [r3, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(3)?,R::R0,Shift::Lsl,2)?)?;
    // 0x67764: bxlt lr
    c.emit(arm32::bx(C::Lt,R::LR))?;
    // 0x67768: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x6776c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x67770: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // 0x67774: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67778: blt 0x6776c <entry+0x11f6c> @ imm = #-0x14
    c.emit(arm32::branch(C::Lt,c.at as u32,0x6776c,false)?)?;
    // 0x6777c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
 c.check(0x67780)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_67780:[u32;2]=[0x2aaaaaab,0x000f1cc8];
 for value in INLINE_DATA_67780{c.emit(value)?;}
 // Executable A32 block 0x67788..0x67870.
 c.check(0x67788)?;
    // 0x67788: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x6778c: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x67790: mov r3, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,36)?)?;
    // 0x67794: mov lr, r0
    c.emit(arm32::mov_register(C::Al,R::LR,R::R0))?;
    // 0x67798: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6779c: orr r3, r3, #62464
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,62464)?)?;
    // 0x677a0: cmp lr, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x677a4: blo 0x6786c <entry+0x1206c> @ imm = #0xc0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6786c,false)?)?;
    // 0x677a8: ldr r3, [pc, #0xc0]         @ 0x12070 <entry+0x12070>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,192)?)?;
    // 0x677ac: add r0, r1, #51
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,51)?)?;
    // 0x677b0: cmn r1, #45
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,45)?)?;
    // 0x677b4: smull r2, r12, r0, r3
    c.emit(emit_smull(C::Al,R::R2,R::R12,R::R0,R::new(3)?)?)?;
    // 0x677b8: ldr r3, [pc, #0xb4]         @ 0x12074 <entry+0x12074>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,180)?)?;
    // 0x677bc: add r12, r12, r12, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R12,R::R12,Shift::Lsr,31,false)?)?;
    // 0x677c0: add r2, r12, r12, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R12,R::R12,Shift::Lsl,1,false)?)?;
    // 0x677c4: sub r2, r0, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R0,R::R2,Shift::Lsl,1,false)?)?;
    // 0x677c8: ldr r3, [r3, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // 0x677cc: blt 0x677e4 <entry+0x11fe4> @ imm = #0x10
    c.emit(arm32::branch(C::Lt,c.at as u32,0x677e4,false)?)?;
    // 0x677d0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x677d4: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x677d8: lsl r3, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,1)?)?;
    // 0x677dc: cmp r1, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // 0x677e0: blt 0x677d4 <entry+0x11fd4> @ imm = #-0x14
    c.emit(arm32::branch(C::Lt,c.at as u32,0x677d4,false)?)?;
    // 0x677e4: cmp r3, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x677e8: poplo {r11, pc}
    c.emit(arm32::transfer_multiple(C::Cc,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x677ec: ldr r1, [pc, #0x84]         @ 0x12078 <entry+0x12078>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,132)?)?;
    // 0x677f0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x677f4: cmp lr, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::R1,Shift::Lsl,0,false)?)?;
    // 0x677f8: blo 0x6781c <entry+0x1201c> @ imm = #0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6781c,false)?)?;
    // 0x677fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x67800: ldr r2, [pc, #0x74]         @ 0x1207c <entry+0x1207c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,116)?)?;
    // 0x67804: lsr r1, lr, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::LR,Shift::Lsr,1)?)?;
    // 0x67808: add r0, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,6)?)?;
    // 0x6780c: cmp lr, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67810: mov lr, r1
    c.emit(arm32::mov_register(C::Al,R::LR,R::R1))?;
    // 0x67814: bhi 0x67800 <entry+0x12000> @ imm = #-0x1c
    c.emit(arm32::branch(C::Hi,c.at as u32,0x67800,false)?)?;
    // 0x67818: b 0x67820 <entry+0x12020> @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x67820,false)?)?;
    // 0x6781c: mov r1, lr
    c.emit(arm32::mov_register(C::Al,R::R1,R::LR))?;
    // 0x67820: ldr r2, [pc, #0x58]         @ 0x12080 <entry+0x12080>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,88)?)?;
    // 0x67824: mov r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x67828: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x6782c: ldr r2, [pc, #0x50]         @ 0x12084 <entry+0x12084>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,80)?)?;
    // 0x67830: movlo r3, #4
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x67834: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67838: mov r2, #408
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,408)?)?;
    // 0x6783c: orr r2, r2, #83968
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,83968)?)?;
    // 0x67840: movlo r3, #3
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x67844: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67848: mov r2, #249
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,249)?)?;
    // 0x6784c: orr r2, r2, #74752
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,74752)?)?;
    // 0x67850: movlo r3, #2
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x67854: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67858: mov r2, #90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,90)?)?;
    // 0x6785c: orr r2, r2, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65536)?)?;
    // 0x67860: movlo r3, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x67864: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67868: addhs r0, r0, r3
    c.emit(arm32::data_register(C::Cs,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x6786c: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x67870)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_67870:[u32;6]=[0x2aaaaaab,0x000f1cc8,0x0001b775,0x00036ee9,0x00019f0b,0x00016e37];
 for value in INLINE_DATA_67870{c.emit(value)?;}
 // Executable A32 block 0x67888..0x678fc.
 c.check(0x67888)?;
    // 0x67888: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x6788c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x67890: mov r0, #824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,824)?)?;
    // 0x67894: orr r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // 0x67898: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x649a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x6789c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x678a0: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x678a4: beq 0x678e4 <entry+0x120e4> @ imm = #0x38
    c.emit(arm32::branch(C::Eq,c.at as u32,0x678e4,false)?)?;
    // 0x678a8: mov r0, #2864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2864)?)?;
    // 0x678ac: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x678b0: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x678b4: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x678b8: str r1, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x678bc: mov r0, #1168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1168)?)?;
    // 0x678c0: orr r0, r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16384)?)?;
    // 0x678c4: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x649d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x678c8: ldr r2, [pc, #0x2c]         @ 0x120fc <entry+0x120fc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,44)?)?;
    // 0x678cc: mov r1, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,46)?)?;
    // 0x678d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x678d4: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x678d8: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x678dc: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x678e0: b 0x678f0 <entry+0x120f0> @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x678f0,false)?)?;
    // 0x678e4: ldr r2, [pc, #0x10]         @ 0x120fc <entry+0x120fc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,16)?)?;
    // 0x678e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x678ec: mov r1, #564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,564)?)?;
    // 0x678f0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x66148
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x678f4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x678f8: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x678fc)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_678FC:[u32;1]=[0x000f2e4c];
 for value in INLINE_DATA_678FC{c.emit(value)?;}
 // Executable A32 block 0x67900..0x68000.
 c.check(0x67900)?;
    // 0x67900: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x67904: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x67908: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x6790c: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // 0x67910: mov r3, #4080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4080)?)?;
    // 0x67914: ldr r0, [r8, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,88)?)?;
    // 0x67918: ldr r1, [r8, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,84)?)?;
    // 0x6791c: orr r3, r3, #61440
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,61440)?)?;
    // 0x67920: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // 0x67924: cmp r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1280)?)?;
    // 0x67928: and r2, r0, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x6792c: add r0, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,15)?)?;
    // 0x67930: and r3, r0, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x67934: ldr r0, [r8, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1496)?)?;
    // 0x67938: bls 0x6794c <entry+0x1214c> @ imm = #0xc
    c.emit(arm32::branch(C::Ls,c.at as u32,0x6794c,false)?)?;
    // 0x6793c: lsr r1, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(3)?,Shift::Lsr,1)?)?;
    // 0x67940: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // 0x67944: lsr r1, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(3)?,Shift::Lsr,1)?)?;
    // 0x67948: b 0x67950 <entry+0x12150> @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x67950,false)?)?;
    // 0x6794c: mul r1, r2, r3
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R2,R::new(3)?)?)?;
    // 0x67950: blx 0x83c4c <entry+0x2e44c> @ imm = #0x1c2f4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x67954: mov r3, #1808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1808)?)?;
    // 0x67958: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x6795c: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x67960: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x67964: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x67968: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x6796c: add r6, r8, r4, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(8)?,R::new(4)?,Shift::Lsl,3,false)?)?;
    // 0x67970: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x67974: ldr r9, [r6, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(6)?,172)?)?;
    // 0x67978: ldr r6, [r6, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(6)?,168)?)?;
    // 0x6797c: subs r12, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R12,R::R0,R::new(6)?,Shift::Lsl,0,true)?)?;
    // 0x67980: rsc lr, r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsc,R::LR,R::new(9)?,0)?)?;
    // 0x67984: subs r6, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,0,true)?)?;
    // 0x67988: sbcs r7, r9, #0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sbc,R::new(7)?,R::new(9)?,0,true)?)?;
    // 0x6798c: movlo r2, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x67990: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x67994: movne r6, r12
    c.emit(arm32::mov_register(C::Ne,R::new(6)?,R::R12))?;
    // 0x67998: movne r7, lr
    c.emit(arm32::mov_register(C::Ne,R::new(7)?,R::LR))?;
    // 0x6799c: subs r2, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(6)?,R::new(3)?,Shift::Lsl,0,true)?)?;
    // 0x679a0: sbcs r2, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Sbc,R::R2,R::new(7)?,R::R1,Shift::Lsl,0,true)?)?;
    // 0x679a4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x679a8: movlo r2, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x679ac: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x679b0: movne r1, r7
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(7)?))?;
    // 0x679b4: movne r3, r6
    c.emit(arm32::mov_register(C::Ne,R::new(3)?,R::new(6)?))?;
    // 0x679b8: movne r10, r4
    c.emit(arm32::mov_register(C::Ne,R::new(10)?,R::new(4)?))?;
    // 0x679bc: adds r4, r4, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::new(4)?,R::new(4)?,1,true)?)?;
    // 0x679c0: adc r5, r5, #0
    c.emit(emit_adc_immediate(C::Al,R::new(5)?,R::new(5)?,0,false)?)?;
    // 0x679c4: subs r2, r4, #22
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R2,R::new(4)?,22,true)?)?;
    // 0x679c8: sbcs r2, r5, #0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sbc,R::R2,R::new(5)?,0,true)?)?;
    // 0x679cc: blo 0x6796c <entry+0x1216c> @ imm = #-0x68
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6796c,false)?)?;
    // 0x679d0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x679d4: str r0, [r8, #0x650]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1616)?)?;
    // 0x679d8: and r0, r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(10)?,255)?)?;
    // 0x679dc: str r1, [r8, #0x654]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,1620)?)?;
    // 0x679e0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x679e4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x679e8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x679ec: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x679f0: sub sp, sp, #156
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,156)?)?;
    // 0x679f4: mov r8, r2
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R2))?;
    // 0x679f8: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x679fc: mov r0, #2864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2864)?)?;
    // 0x67a00: mov r2, #824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,824)?)?;
    // 0x67a04: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x67a08: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x67a0c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x67a10: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x67a14: mov r10, r8
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(8)?))?;
    // 0x67a18: orr r2, r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x67a1c: ldr r4, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(4)?,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x67a20: ldr r0, [r10, #0x5e0]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(10)?,1504)?)?;
    // 0x67a24: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x67a28: ldr r0, [r8, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1496)?)?;
    // 0x67a2c: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x67a30: ldr r0, [r8, #0x5e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1508)?)?;
    // 0x67a34: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x67a38: ldrb r0, [r9, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,348)?)?;
    // 0x67a3c: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x67a40: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x67a44: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x64860
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x67a48: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x67a4c: beq 0x67a68 <entry+0x12268> @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67a68,false)?)?;
    // 0x67a50: mov r2, #1168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1168)?)?;
    // 0x67a54: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x67a58: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x67a5c: orr r2, r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16384)?)?;
    // 0x67a60: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x6487c
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x67a64: str r4, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,0)?)?;
    // 0x67a68: ldr r0, [r9, #0x254]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,596)?)?;
    // 0x67a6c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x67a70: beq 0x67a80 <entry+0x12280> @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67a80,false)?)?;
    // 0x67a74: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x67a78: bne 0x67ab4 <entry+0x122b4> @ imm = #0x34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67ab4,false)?)?;
    // 0x67a7c: b 0x67a94 <entry+0x12294> @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x67a94,false)?)?;
    // 0x67a80: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x67a84: bne 0x67ab4 <entry+0x122b4> @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67ab4,false)?)?;
    // 0x67a88: ldrb r0, [r9, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,281)?)?;
    // 0x67a8c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x67a90: bne 0x67ab4 <entry+0x122b4> @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67ab4,false)?)?;
    // 0x67a94: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x67a98: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x67a9c: beq 0x67ab4 <entry+0x122b4> @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67ab4,false)?)?;
    // 0x67aa0: ldr r2, [pc, #0xfc4]        @ 0x1326c <entry+0x1326c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,4036)?)?;
    // 0x67aa4: mov r1, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,153)?)?;
    // 0x67aa8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x67aac: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x67ab0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x66308
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x67ab4: ldrb r0, [r9, #0xa04]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2564)?)?;
    // 0x67ab8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x67abc: strb r0, [r8, #0x290]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(8)?,656)?)?;
    // 0x67ac0: ldr r0, [r9, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,484)?)?;
    // 0x67ac4: str r0, [r8, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,132)?)?;
    // 0x67ac8: ldr r2, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,4)?)?;
    // 0x67acc: str r2, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x67ad0: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x67ad4: str r0, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,4)?)?;
    // 0x67ad8: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x67adc: str r0, [r8, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,8)?)?;
    // 0x67ae0: ldrb r0, [r9, #0x210]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,528)?)?;
    // 0x67ae4: strb r0, [r8, #0x21]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(8)?,33)?)?;
    // 0x67ae8: mov r0, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,668)?)?;
    // 0x67aec: strh r1, [r8, r0]
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::new(8)?,R::R0,false)?)?;
    // 0x67af0: ldrb r0, [r9, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,281)?)?;
    // 0x67af4: str r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,28)?)?;
    // 0x67af8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x67afc: bne 0x67b18 <entry+0x12318> @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67b18,false)?)?;
    // 0x67b00: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x67b04: mov r1, #280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,280)?)?;
    // 0x67b08: str r0, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,12)?)?;
    // 0x67b0c: ldrsb r1, [r9, r1]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R1,R::new(9)?,R::R1)?)?;
    // 0x67b10: and r5, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,255)?)?;
    // 0x67b14: b 0x67b44 <entry+0x12344> @ imm = #0x28
    c.emit(arm32::branch(C::Al,c.at as u32,0x67b44,false)?)?;
    // 0x67b18: ldr r0, [r9, #0x254]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,596)?)?;
    // 0x67b1c: str r0, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,12)?)?;
    // 0x67b20: ldr r1, [r9, #0x830]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2096)?)?;
    // 0x67b24: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x67b28: beq 0x67b38 <entry+0x12338> @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67b38,false)?)?;
    // 0x67b2c: ldrb r1, [r9, #0x2e1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,737)?)?;
    // 0x67b30: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x67b34: b 0x67b44 <entry+0x12344> @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x67b44,false)?)?;
    // 0x67b38: ldrb r1, [r6, #0x80]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,128)?)?;
    // 0x67b3c: ldrb r5, [r9, #0x2e1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(9)?,737)?)?;
    // 0x67b40: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x67b44: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x67b48: str r1, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,52)?)?;
    // 0x67b4c: bne 0x67b88 <entry+0x12388> @ imm = #0x34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67b88,false)?)?;
    // 0x67b50: ldrb r3, [r9, #0x1e8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,488)?)?;
    // 0x67b54: cmp r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8)?)?;
    // 0x67b58: beq 0x67b74 <entry+0x12374> @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67b74,false)?)?;
    // 0x67b5c: cmp r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // 0x67b60: beq 0x67b7c <entry+0x1237c> @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67b7c,false)?)?;
    // 0x67b64: cmp r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,15)?)?;
    // 0x67b68: bne 0x67b84 <entry+0x12384> @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67b84,false)?)?;
    // 0x67b6c: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x67b70: b 0x67b80 <entry+0x12380> @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x67b80,false)?)?;
    // 0x67b74: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x67b78: b 0x67b80 <entry+0x12380> @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x67b80,false)?)?;
    // 0x67b7c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x67b80: str r1, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,52)?)?;
    // 0x67b84: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x67b88: ldrb r3, [r9, #0x15d]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,349)?)?;
    // 0x67b8c: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x67b90: str r3, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,20)?)?;
    // 0x67b94: ldrb r3, [r9, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,350)?)?;
    // 0x67b98: str r3, [r8, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,24)?)?;
    // 0x67b9c: ldrb r3, [r9, #0x15f]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,351)?)?;
    // 0x67ba0: strb r3, [r8, #0x1c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(8)?,28)?)?;
    // 0x67ba4: ldrb r3, [r9, #0x160]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,352)?)?;
    // 0x67ba8: strb r3, [r8, #0x1d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(8)?,29)?)?;
    // 0x67bac: ldr r7, [r9, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,2104)?)?;
    // 0x67bb0: ldrb r3, [r7, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,224)?)?;
    // 0x67bb4: strb r3, [r8, #0x30]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(8)?,48)?)?;
    // 0x67bb8: ldrb r7, [r7, #0xdf]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,223)?)?;
    // 0x67bbc: strb r7, [r8, #0x31]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(8)?,49)?)?;
    // 0x67bc0: ldrb r7, [r9, #0x74]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,116)?)?;
    // 0x67bc4: strb r7, [r8, #0x32]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(8)?,50)?)?;
    // 0x67bc8: ldrb r7, [r9, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,348)?)?;
    // 0x67bcc: strb r7, [r8, #0x7e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(8)?,126)?)?;
    // 0x67bd0: ldr r7, [r9, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,304)?)?;
    // 0x67bd4: str r7, [r8, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,128)?)?;
    // 0x67bd8: beq 0x67bf4 <entry+0x123f4> @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67bf4,false)?)?;
    // 0x67bdc: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x67be0: bne 0x67bfc <entry+0x123fc> @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67bfc,false)?)?;
    // 0x67be4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x67be8: beq 0x67c0c <entry+0x1240c> @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67c0c,false)?)?;
    // 0x67bec: ldrb r3, [r9, #0xa8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,168)?)?;
    // 0x67bf0: b 0x67c10 <entry+0x12410> @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x67c10,false)?)?;
    // 0x67bf4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x67bf8: beq 0x67c0c <entry+0x1240c> @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67c0c,false)?)?;
    // 0x67bfc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x67c00: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x67c04: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x67c08: b 0x67c10 <entry+0x12410> @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x67c10,false)?)?;
    // 0x67c0c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x67c10: strb r3, [r8, #0x20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(8)?,32)?)?;
    // 0x67c14: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x67c18: ldr r3, [r9, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,76)?)?;
    // 0x67c1c: str r3, [r8, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,84)?)?;
    // 0x67c20: ldr r3, [r9, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,80)?)?;
    // 0x67c24: str r3, [r8, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,88)?)?;
    // 0x67c28: ldr r3, [r9, #0x7e4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,2020)?)?;
    // 0x67c2c: str r3, [r8, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,56)?)?;
    // 0x67c30: ldr r3, [r9, #0x7e8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,2024)?)?;
    // 0x67c34: str r3, [r8, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,60)?)?;
    // 0x67c38: ldr r3, [r9, #0x888]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,2184)?)?;
    // 0x67c3c: str r3, [r8, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,64)?)?;
    // 0x67c40: ldr r3, [r9, #0x88c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,2188)?)?;
    // 0x67c44: str r3, [r8, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,68)?)?;
    // 0x67c48: ldrbne r3, [r9, #0x202]
    c.emit(arm32::transfer_byte(C::Ne,true,R::new(3)?,R::new(9)?,514)?)?;
    // 0x67c4c: strbne r3, [r8, #0x2c]
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(3)?,R::new(8)?,44)?)?;
    // 0x67c50: ldrbne r3, [r9, #0x203]
    c.emit(arm32::transfer_byte(C::Ne,true,R::new(3)?,R::new(9)?,515)?)?;
    // 0x67c54: strbne r3, [r8, #0x2d]
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(3)?,R::new(8)?,45)?)?;
    // 0x67c58: ldrb r3, [r9, #0x1ff]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,511)?)?;
    // 0x67c5c: strb r3, [r8, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(8)?,46)?)?;
    // 0x67c60: ldrb r3, [r9, #0x1fe]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,510)?)?;
    // 0x67c64: strb r3, [r8, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(8)?,47)?)?;
    // 0x67c68: ldr r3, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,36)?)?;
    // 0x67c6c: lsr r7, r3, #14
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(3)?,Shift::Lsr,14)?)?;
    // 0x67c70: cmp r3, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,65536)?)?;
    // 0x67c74: movlo r7, #4
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x67c78: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x67c7c: str r7, [r8, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,92)?)?;
    // 0x67c80: ldrbeq r2, [r9, #0x1e8]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R2,R::new(9)?,488)?)?;
    // 0x67c84: cmpeq r2, #12
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R2,12)?)?;
    // 0x67c88: beq 0x67ca8 <entry+0x124a8> @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67ca8,false)?)?;
    // 0x67c8c: ldr r2, [r6, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,40)?)?;
    // 0x67c90: str r2, [r8, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,100)?)?;
    // 0x67c94: ldr r7, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,44)?)?;
    // 0x67c98: add r2, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x67c9c: str r7, [r8, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,104)?)?;
    // 0x67ca0: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x67ca4: b 0x67cb8 <entry+0x124b8> @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x67cb8,false)?)?;
    // 0x67ca8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x67cac: str r2, [r8, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,100)?)?;
    // 0x67cb0: str r2, [r8, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,104)?)?;
    // 0x67cb4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x67cb8: str r2, [r8, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,108)?)?;
    // 0x67cbc: ldrb r2, [r9, #0x214]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,532)?)?;
    // 0x67cc0: strb r2, [r8, #0x89]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(8)?,137)?)?;
    // 0x67cc4: ldrb r2, [r9, #0x213]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,531)?)?;
    // 0x67cc8: strb r2, [r8, #0x88]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(8)?,136)?)?;
    // 0x67ccc: ldrb r2, [r9, #0x215]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,533)?)?;
    // 0x67cd0: strb r2, [r8, #0x8a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(8)?,138)?)?;
    // 0x67cd4: ldrb r2, [r9, #0x216]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,534)?)?;
    // 0x67cd8: strb r2, [r8, #0x8b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(8)?,139)?)?;
    // 0x67cdc: ldrb r2, [r9, #0x217]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,535)?)?;
    // 0x67ce0: strb r2, [r8, #0x8c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(8)?,140)?)?;
    // 0x67ce4: ldr r2, [r9, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,540)?)?;
    // 0x67ce8: str r2, [r8, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,144)?)?;
    // 0x67cec: ldrb r2, [r9, #0x218]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,536)?)?;
    // 0x67cf0: strb r2, [r8, #0x8d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(8)?,141)?)?;
    // 0x67cf4: ldr r2, [r9, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,104)?)?;
    // 0x67cf8: str r2, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,16)?)?;
    // 0x67cfc: ldr r2, [r9, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,192)?)?;
    // 0x67d00: str r2, [r8, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,112)?)?;
    // 0x67d04: ldr r2, [r9, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,196)?)?;
    // 0x67d08: str r2, [r8, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,116)?)?;
    // 0x67d0c: ldrb r2, [r9, #0xc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,200)?)?;
    // 0x67d10: strb r2, [r8, #0x25]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(8)?,37)?)?;
    // 0x67d14: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x67d18: ldrb r7, [r9, #0x19b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,411)?)?;
    // 0x67d1c: strb r7, [r8, #0x7c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(8)?,124)?)?;
    // 0x67d20: ldrb r7, [r9, #0x170]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,368)?)?;
    // 0x67d24: strb r7, [r8, #0x7d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(8)?,125)?)?;
    // 0x67d28: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x67d2c: strb r7, [r8, #0x27e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(8)?,638)?)?;
    // 0x67d30: ldr r7, [r9, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,720)?)?;
    // 0x67d34: str r7, [r8, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,648)?)?;
    // 0x67d38: ldr r7, [r9, #0x2d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,724)?)?;
    // 0x67d3c: str r7, [r8, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,652)?)?;
    // 0x67d40: bhi 0x67d64 <entry+0x12564> @ imm = #0x1c
    c.emit(arm32::branch(C::Hi,c.at as u32,0x67d64,false)?)?;
    // 0x67d44: ldr r2, [r9, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,372)?)?;
    // 0x67d48: lsr r3, r3, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsr,16)?)?;
    // 0x67d4c: mul r7, r3, r2
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::new(3)?,R::R2)?)?;
    // 0x67d50: mov r2, #1904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1904)?)?;
    // 0x67d54: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x67d58: cmp r7, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x67d5c: movls r2, #0
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x67d60: strbls r2, [r8, #0x25]
    c.emit(arm32::transfer_byte(C::Ls,false,R::R2,R::new(8)?,37)?)?;
    // 0x67d64: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x67d68: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x67d6c: str r8, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,60)?)?;
    // 0x67d70: str r9, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,36)?)?;
    // 0x67d74: str r6, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,56)?)?;
    // 0x67d78: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x67d7c: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x67d80: beq 0x67f4c <entry+0x1274c> @ imm = #0x1c4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67f4c,false)?)?;
    // 0x67d84: ldr r2, [pc, #0xce0]        @ 0x1326c <entry+0x1326c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3296)?)?;
    // 0x67d88: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // 0x67d8c: mov r1, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,40)?)?;
    // 0x67d90: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x67d94: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x67d98: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x67d9c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x665f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x67da0: ldr r0, [r9, #0x834]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2100)?)?;
    // 0x67da4: ldr r0, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,100)?)?;
    // 0x67da8: str r0, [r8, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,72)?)?;
    // 0x67dac: ldrb r1, [r9, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,281)?)?;
    // 0x67db0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x67db4: bne 0x67dc4 <entry+0x125c4> @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67dc4,false)?)?;
    // 0x67db8: ldrb r1, [r9, #0x170]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,368)?)?;
    // 0x67dbc: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x67dc0: bne 0x67fb0 <entry+0x127b0> @ imm = #0x1e8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67fb0,false)?)?;
    // 0x67dc4: ldr r1, [r8, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,92)?)?;
    // 0x67dc8: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x67dcc: blx 0x83c4c <entry+0x2e44c> @ imm = #0x1be78
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x67dd0: str r0, [r8, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,96)?)?;
    // 0x67dd4: ldr r0, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,52)?)?;
    // 0x67dd8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x67ddc: beq 0x67ee4 <entry+0x126e4> @ imm = #0x100
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67ee4,false)?)?;
    // 0x67de0: ldr r12, [pc, #0xfe4]       @ 0x135cc <entry+0x135cc>
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::PC,4068)?)?;
    // 0x67de4: ldr r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,60)?)?;
    // 0x67de8: add lr, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(6)?,4)?)?;
    // 0x67dec: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // 0x67df0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x67df4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x67df8: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x67dfc: ldr r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // 0x67e00: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x67e04: beq 0x69b30 <entry+0x14330> @ imm = #0x1d24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x69b30,false)?)?;
    // 0x67e08: ldr r3, [r2, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,96)?)?;
    // 0x67e0c: add r1, r1, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x67e10: str r3, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,56)?)?;
    // 0x67e14: ldr r7, [lr, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::LR,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x67e18: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x67e1c: str r7, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,8)?)?;
    // 0x67e20: moveq r9, r7
    c.emit(arm32::mov_register(C::Eq,R::new(9)?,R::new(7)?))?;
    // 0x67e24: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x67e28: bhi 0x67e88 <entry+0x12688> @ imm = #0x58
    c.emit(arm32::branch(C::Hi,c.at as u32,0x67e88,false)?)?;
    // 0x67e2c: ldr r3, [r2, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,52)?)?;
    // 0x67e30: ldr r4, [r2, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R2,108)?)?;
    // 0x67e34: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x67e38: sub r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // 0x67e3c: lsr r4, r4, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(4)?,R::R0,R::new(4)?,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x67e40: str r4, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,32)?)?;
    // 0x67e44: bne 0x67e94 <entry+0x12694> @ imm = #0x48
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67e94,false)?)?;
    // 0x67e48: ldr r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,36)?)?;
    // 0x67e4c: ldr r3, [r3, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,284)?)?;
    // 0x67e50: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x67e54: ldrbne r3, [r2, #0x6c]
    c.emit(arm32::transfer_byte(C::Ne,true,R::new(3)?,R::R2,108)?)?;
    // 0x67e58: tstne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x67e5c: beq 0x67e94 <entry+0x12694> @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67e94,false)?)?;
    // 0x67e60: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // 0x67e64: cmp r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8192)?)?;
    // 0x67e68: bne 0x67e94 <entry+0x12694> @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67e94,false)?)?;
    // 0x67e6c: ldr r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,36)?)?;
    // 0x67e70: ldr r3, [r3, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,2104)?)?;
    // 0x67e74: ldrb r3, [r3, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(3)?,224)?)?;
    // 0x67e78: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x67e7c: bne 0x67e94 <entry+0x12694> @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67e94,false)?)?;
    // 0x67e80: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x67e84: b 0x67e90 <entry+0x12690> @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x67e90,false)?)?;
    // 0x67e88: ldr r3, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,28)?)?;
    // 0x67e8c: lsl r4, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(3)?,Shift::Lsl,1)?)?;
    // 0x67e90: str r4, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,32)?)?;
    // 0x67e94: ldr r3, [r2, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,52)?)?;
    // 0x67e98: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x67e9c: add r1, r3, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x67ea0: add r1, r12, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R12,R::R1,Shift::Lsl,1,false)?)?;
    // 0x67ea4: add r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x67ea8: ldrb r8, [r1, #-0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::R1,-12)?)?;
    // 0x67eac: mla r1, r4, r8, r6
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(4)?,R::new(8)?,R::new(6)?)?)?;
    // 0x67eb0: bne 0x67ec4 <entry+0x126c4> @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x67ec4,false)?)?;
    // 0x67eb4: sub r4, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(3)?,2)?)?;
    // 0x67eb8: add r4, r4, r4, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(4)?,R::new(4)?,Shift::Lsl,1,false)?)?;
    // 0x67ebc: ldrb r4, [r12, r4, lsl #1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(4)?,R::R12,R::new(4)?,Shift::Lsl,1)?)?;
    // 0x67ec0: add r1, r1, r4, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(4)?,Shift::Lsl,2,false)?)?;
    // 0x67ec4: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x67ec8: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x67ecc: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x67ed0: cmp r5, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x67ed4: blo 0x67dfc <entry+0x125fc> @ imm = #-0xe0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x67dfc,false)?)?;
    // 0x67ed8: tst r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,255)?)?;
    // 0x67edc: ldr r9, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,36)?)?;
    // 0x67ee0: beq 0x68038 <entry+0x12838> @ imm = #0x150
    c.emit(arm32::branch(C::Eq,c.at as u32,0x68038,false)?)?;
    // 0x67ee4: ldrb r0, [r9, #0x170]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,368)?)?;
    // 0x67ee8: ldr r8, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,60)?)?;
    // 0x67eec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x67ef0: ldrne r0, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(8)?,52)?)?;
    // 0x67ef4: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x67ef8: beq 0x67f54 <entry+0x12754> @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67f54,false)?)?;
    // 0x67efc: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x67f00: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x67f04: ldr r1, [r8, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,108)?)?;
    // 0x67f08: ldr r7, [r8, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,92)?)?;
    // 0x67f0c: add r5, r0, r4, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(4)?,Shift::Lsl,2,false)?)?;
    // 0x67f10: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x67f14: ldr r3, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,32)?)?;
    // 0x67f18: umull r6, r2, r0, r1
    c.emit(emit_umull(C::Al,R::new(6)?,R::R2,R::R0,R::R1)?)?;
    // 0x67f1c: lsl r0, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x67f20: mul r2, r7, r3
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(7)?,R::new(3)?)?)?;
    // 0x67f24: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x67f28: orr r1, r0, r6, lsr #30
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(6)?,Shift::Lsr,30,false)?)?;
    // 0x67f2c: lsl r0, r6, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x67f30: bl 0x8187c <entry+0x2c07c> @ imm = #0x19944
    c.emit(arm32::branch(C::Al,c.at as u32,0x8187c,true)?)?;
    // 0x67f34: str r0, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,56)?)?;
    // 0x67f38: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x67f3c: ldr r0, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,52)?)?;
    // 0x67f40: cmp r4, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x67f44: blo 0x67f00 <entry+0x12700> @ imm = #-0x4c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x67f00,false)?)?;
    // 0x67f48: b 0x67f54 <entry+0x12754> @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x67f54,false)?)?;
    // 0x67f4c: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // 0x67f50: str r0, [r8, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,72)?)?;
    // 0x67f54: ldr r0, [r8, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,72)?)?;
    // 0x67f58: ldr r1, [r8, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,92)?)?;
    // 0x67f5c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x67f60: blx 0x83c4c <entry+0x2e44c> @ imm = #0x1bce4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x67f64: str r0, [r8, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,96)?)?;
    // 0x67f68: ldrb r0, [r8, #0x89]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,137)?)?;
    // 0x67f6c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x67f70: bne 0x684b0 <entry+0x12cb0> @ imm = #0x538
    c.emit(arm32::branch(C::Ne,c.at as u32,0x684b0,false)?)?;
    // 0x67f74: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x67f78: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x67f7c: beq 0x684b0 <entry+0x12cb0> @ imm = #0x52c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x684b0,false)?)?;
    // 0x67f80: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // 0x67f84: str r1, [r8, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,1496)?)?;
    // 0x67f88: ldr r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // 0x67f8c: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // 0x67f90: stm r10, {r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(10)?,R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x67f94: ldr r1, [r9, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,544)?)?;
    // 0x67f98: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x67f9c: beq 0x680cc <entry+0x128cc> @ imm = #0x128
    c.emit(arm32::branch(C::Eq,c.at as u32,0x680cc,false)?)?;
    // 0x67fa0: str r1, [r8, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,148)?)?;
    // 0x67fa4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x67fa8: ldrb r1, [r9, #0x224]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,548)?)?;
    // 0x67fac: b 0x680d8 <entry+0x128d8> @ imm = #0x124
    c.emit(arm32::branch(C::Al,c.at as u32,0x680d8,false)?)?;
    // 0x67fb0: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // 0x67fb4: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x67fb8: str r0, [r8, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,72)?)?;
    // 0x67fbc: beq 0x68370 <entry+0x12b70> @ imm = #0x3ac
    c.emit(arm32::branch(C::Eq,c.at as u32,0x68370,false)?)?;
    // 0x67fc0: ldr r1, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,52)?)?;
    // 0x67fc4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x67fc8: beq 0x67f54 <entry+0x12754> @ imm = #-0x7c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x67f54,false)?)?;
    // 0x67fcc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x67fd0: b 0x67fd8 <entry+0x127d8> @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x67fd8,false)?)?;
    // 0x67fd4: ldr r0, [r8, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,72)?)?;
    // 0x67fd8: blx 0x83c4c <entry+0x2e44c> @ imm = #0x1bc6c
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x67fdc: ldr r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // 0x67fe0: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x67fe4: add r4, r1, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R1,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x67fe8: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x67fec: bhi 0x68004 <entry+0x12804> @ imm = #0x10
    c.emit(arm32::branch(C::Hi,c.at as u32,0x68004,false)?)?;
    // 0x67ff0: ldr r0, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,52)?)?;
    // 0x67ff4: ldr r1, [r8, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,108)?)?;
    // 0x67ff8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x67ffc: lsr r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsr,R::R0,false)?)?;
 c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();emit_controller_packet(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests{use super::*;
 #[test]fn chunk_len_and_bounds(){assert_eq!(fragment().unwrap().len(),END-START);assert!(END<=0x68000);}
 #[test]fn original_firmware_byte_comparison_when_present(){
  if let Ok(path)=std::env::var("NABU_VENUS_REFERENCE").or_else(|_| std::env::var("AGENT04_PRIVATE_VENUS")){
   let reference=std::fs::read(path).unwrap();let generated=fragment().unwrap();
   assert_eq!(&generated[..],&reference[FILE_OFFSET..FILE_OFFSET+generated.len()]);
  }
 }
}
