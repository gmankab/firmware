// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x59000,0x5a000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x59000;
pub const END:usize=0x5a000;
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
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x59000..0x595f8.
 c.check(0x59000)?;
    // 0x59000: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x59004: blo 0x59028 <entry+0x3828>  @ imm = #0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x59028,false)?)?;
    // 0x59008: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5900c: str r0, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,28)?)?;
    // 0x59010: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x59014: orr r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // 0x59018: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x5901c: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x59020: bic r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,1024)?)?;
    // 0x59024: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x59028: tst r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2048)?)?;
    // 0x5902c: beq 0x59064 <entry+0x3864>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59064,false)?)?;
    // 0x59030: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x59034: bne 0x59044 <entry+0x3844>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59044,false)?)?;
    // 0x59038: ldr r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,80)?)?;
    // 0x5903c: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x59040: blo 0x59064 <entry+0x3864>  @ imm = #0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x59064,false)?)?;
    // 0x59044: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x59048: str r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,32)?)?;
    // 0x5904c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x59050: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // 0x59054: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x59058: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x5905c: bic r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,2048)?)?;
    // 0x59060: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x59064: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x59068: beq 0x590a0 <entry+0x38a0>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x590a0,false)?)?;
    // 0x5906c: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x59070: bne 0x59080 <entry+0x3880>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59080,false)?)?;
    // 0x59074: ldr r1, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,60)?)?;
    // 0x59078: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5907c: blo 0x590a0 <entry+0x38a0>  @ imm = #0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x590a0,false)?)?;
    // 0x59080: ldr r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x59084: str r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,36)?)?;
    // 0x59088: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5908c: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // 0x59090: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x59094: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x59098: bic r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,1)?)?;
    // 0x5909c: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x590a0: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x590a4: beq 0x59168 <entry+0x3968>  @ imm = #0xbc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59168,false)?)?;
    // 0x590a8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x590ac: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x590b0: ldr r1, [r7, #0x82c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,2092)?)?;
    // 0x590b4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x590b8: beq 0x590c8 <entry+0x38c8>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x590c8,false)?)?;
    // 0x590bc: ldr r1, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,44)?)?;
    // 0x590c0: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x590c4: b 0x590e0 <entry+0x38e0>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x590e0,false)?)?;
    // 0x590c8: add r0, r4, #272
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,272)?)?;
    // 0x590cc: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // 0x590d0: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x590d4: bl 0x35fb8 <entry+0xfffffffffffe07b8> @ imm = #-0x23124
    c.emit(arm32::branch(C::Al,c.at as u32,0x35fb8,true)?)?;
    // 0x590d8: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x590dc: beq 0x59164 <entry+0x3964>  @ imm = #0x80
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59164,false)?)?;
    // 0x590e0: ldrb r1, [r7, #0x190]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,400)?)?;
    // 0x590e4: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x590e8: bne 0x59104 <entry+0x3904>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59104,false)?)?;
    // 0x590ec: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x590f0: str r1, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,44)?)?;
    // 0x590f4: ldr r2, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,48)?)?;
    // 0x590f8: ldr r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,4)?)?;
    // 0x590fc: add r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x59100: b 0x59110 <entry+0x3910>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x59110,false)?)?;
    // 0x59104: ldr r1, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,48)?)?;
    // 0x59108: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,4)?)?;
    // 0x5910c: str r1, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,44)?)?;
    // 0x59110: str r2, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,40)?)?;
    // 0x59114: ldr r3, [r7, #0x10c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,268)?)?;
    // 0x59118: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x5911c: addne r1, r2, r1
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x59120: movne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x59124: strne r1, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(5)?,40)?)?;
    // 0x59128: strne r2, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::new(5)?,44)?)?;
    // 0x5912c: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x59130: orr r1, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2)?)?;
    // 0x59134: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x59138: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x5913c: and r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // 0x59140: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x59144: bic r1, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,2)?)?;
    // 0x59148: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5914c: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x59150: ldr r0, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,40)?)?;
    // 0x59154: ldr r1, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,44)?)?;
    // 0x59158: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5915c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x59160: str r0, [r7, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,132)?)?;
    // 0x59164: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x59168: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // 0x5916c: beq 0x591b0 <entry+0x39b0>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x591b0,false)?)?;
    // 0x59170: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x59174: bne 0x59184 <entry+0x3984>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59184,false)?)?;
    // 0x59178: ldr r1, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,68)?)?;
    // 0x5917c: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x59180: blo 0x591b0 <entry+0x39b0>  @ imm = #0x28
    c.emit(arm32::branch(C::Cc,c.at as u32,0x591b0,false)?)?;
    // 0x59184: ldr r1, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,4)?)?;
    // 0x59188: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x5918c: beq 0x591a8 <entry+0x39a8>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x591a8,false)?)?;
    // 0x59190: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x59194: str r0, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,48)?)?;
    // 0x59198: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5919c: orr r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // 0x591a0: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x591a4: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x591a8: bic r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,4)?)?;
    // 0x591ac: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x591b0: tst r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,32)?)?;
    // 0x591b4: beq 0x591ec <entry+0x39ec>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x591ec,false)?)?;
    // 0x591b8: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x591bc: bne 0x591cc <entry+0x39cc>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x591cc,false)?)?;
    // 0x591c0: ldr r1, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,88)?)?;
    // 0x591c4: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x591c8: blo 0x591ec <entry+0x39ec>  @ imm = #0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x591ec,false)?)?;
    // 0x591cc: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // 0x591d0: str r0, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,52)?)?;
    // 0x591d4: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x591d8: orr r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32)?)?;
    // 0x591dc: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x591e0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x591e4: bic r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,32)?)?;
    // 0x591e8: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x591ec: tst r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,16)?)?;
    // 0x591f0: beq 0x59238 <entry+0x3a38>  @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59238,false)?)?;
    // 0x591f4: add r0, r4, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,200)?)?;
    // 0x591f8: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // 0x591fc: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x59200: bl 0x35fb8 <entry+0xfffffffffffe07b8> @ imm = #-0x23250
    c.emit(arm32::branch(C::Al,c.at as u32,0x35fb8,true)?)?;
    // 0x59204: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x59208: beq 0x59234 <entry+0x3a34>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59234,false)?)?;
    // 0x5920c: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x59210: orr r1, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16)?)?;
    // 0x59214: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x59218: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x5921c: and r0, r1, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,4,false)?)?;
    // 0x59220: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x59224: bic r1, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,16)?)?;
    // 0x59228: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5922c: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x59230: b 0x59238 <entry+0x3a38>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x59238,false)?)?;
    // 0x59234: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x59238: tst r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,256)?)?;
    // 0x5923c: beq 0x5928c <entry+0x3a8c>  @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5928c,false)?)?;
    // 0x59240: add r0, r4, #164
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,164)?)?;
    // 0x59244: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // 0x59248: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x5924c: bl 0x35fb8 <entry+0xfffffffffffe07b8> @ imm = #-0x2329c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35fb8,true)?)?;
    // 0x59250: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x59254: beq 0x59288 <entry+0x3a88>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59288,false)?)?;
    // 0x59258: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x5925c: strb r1, [r5, #0x80]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,128)?)?;
    // 0x59260: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x59264: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x59268: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x5926c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x59270: and r0, r1, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,8,false)?)?;
    // 0x59274: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x59278: bic r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,256)?)?;
    // 0x5927c: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x59280: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x59284: b 0x5928c <entry+0x3a8c>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5928c,false)?)?;
    // 0x59288: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x5928c: tst r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,512)?)?;
    // 0x59290: beq 0x592c8 <entry+0x3ac8>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x592c8,false)?)?;
    // 0x59294: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x59298: bne 0x592a8 <entry+0x3aa8>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x592a8,false)?)?;
    // 0x5929c: ldr r1, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,104)?)?;
    // 0x592a0: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x592a4: blo 0x592c8 <entry+0x3ac8>  @ imm = #0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x592c8,false)?)?;
    // 0x592a8: ldr r0, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,136)?)?;
    // 0x592ac: str r0, [r5, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,132)?)?;
    // 0x592b0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x592b4: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x592b8: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x592bc: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x592c0: bic r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,512)?)?;
    // 0x592c4: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x592c8: tst r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,8192)?)?;
    // 0x592cc: beq 0x5931c <entry+0x3b1c>  @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5931c,false)?)?;
    // 0x592d0: add r0, r4, #236
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,236)?)?;
    // 0x592d4: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // 0x592d8: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x592dc: bl 0x35fb8 <entry+0xfffffffffffe07b8> @ imm = #-0x2332c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35fb8,true)?)?;
    // 0x592e0: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x592e4: beq 0x59318 <entry+0x3b18>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59318,false)?)?;
    // 0x592e8: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x592ec: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x592f0: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x592f4: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x592f8: str r1, [r5, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,136)?)?;
    // 0x592fc: mov r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8192)?)?;
    // 0x59300: and r0, r1, r0, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,13,false)?)?;
    // 0x59304: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x59308: bic r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,8192)?)?;
    // 0x5930c: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x59310: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x59314: b 0x5931c <entry+0x3b1c>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5931c,false)?)?;
    // 0x59318: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x5931c: tst r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,65536)?)?;
    // 0x59320: beq 0x5934c <entry+0x3b4c>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5934c,false)?)?;
    // 0x59324: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x59328: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // 0x5932c: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x59330: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,152)?)?;
    // 0x59334: str r0, [r5, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,148)?)?;
    // 0x59338: ldr r0, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,156)?)?;
    // 0x5933c: str r0, [r5, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,152)?)?;
    // 0x59340: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x59344: bic r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,65536)?)?;
    // 0x59348: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x5934c: tst r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,131072)?)?;
    // 0x59350: beq 0x59398 <entry+0x3b98>  @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59398,false)?)?;
    // 0x59354: add r0, r4, #308
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,308)?)?;
    // 0x59358: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // 0x5935c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x59360: bl 0x35fb8 <entry+0xfffffffffffe07b8> @ imm = #-0x233b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x35fb8,true)?)?;
    // 0x59364: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x59368: beq 0x59398 <entry+0x3b98>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59398,false)?)?;
    // 0x5936c: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x59370: str r1, [r5, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,156)?)?;
    // 0x59374: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x59378: orr r1, r1, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,131072)?)?;
    // 0x5937c: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x59380: mov r1, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,131072)?)?;
    // 0x59384: and r0, r1, r0, lsl #17
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,17,false)?)?;
    // 0x59388: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x5938c: bic r1, r1, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,131072)?)?;
    // 0x59390: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x59394: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x59398: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5939c: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x56770
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x593a0: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x593a4: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x593a8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x593ac: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x593b0: sub sp, sp, #92
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,92)?)?;
    // 0x593b4: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x593b8: mov r9, r3
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(3)?))?;
    // 0x593bc: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x593c0: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x593c4: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // 0x593c8: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x593cc: ldreq r9, [r6, #0x100]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(9)?,R::new(6)?,256)?)?;
    // 0x593d0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x593d4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x593d8: bne 0x593ec <entry+0x3bec>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x593ec,false)?)?;
    // 0x593dc: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x593e0: bl 0x54cfc <entry+0xfffffffffffff4fc> @ imm = #-0x46ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x54cfc,true)?)?;
    // 0x593e4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x593e8: bl 0x6383c <entry+0xe03c>  @ imm = #0xa44c
    c.emit(arm32::branch(C::Al,c.at as u32,0x6383c,true)?)?;
    // 0x593ec: ldr r0, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,644)?)?;
    // 0x593f0: add r8, r6, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(6)?,640)?)?;
    // 0x593f4: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x593f8: beq 0x594b0 <entry+0x3cb0>  @ imm = #0xb0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x594b0,false)?)?;
    // 0x593fc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x59400: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x59404: bne 0x594c4 <entry+0x3cc4>  @ imm = #0xb8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x594c4,false)?)?;
    // 0x59408: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5940c: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x59410: ldr r0, [r6, #0x4e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1252)?)?;
    // 0x59414: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // 0x59418: beq 0x59434 <entry+0x3c34>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59434,false)?)?;
    // 0x5941c: ldr r1, [r6, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,332)?)?;
    // 0x59420: ldr r0, [r6, #0x438]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1080)?)?;
    // 0x59424: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x59428: bl 0x35540 <entry+0xfffffffffffdfd40> @ imm = #-0x23ef0
    c.emit(arm32::branch(C::Al,c.at as u32,0x35540,true)?)?;
    // 0x5942c: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x59430: ldr r0, [r6, #0x4e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1252)?)?;
    // 0x59434: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x59438: add r10, sp, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,40)?)?;
    // 0x5943c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x59440: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x59444: ldrne r0, [r6, #0x428]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,1064)?)?;
    // 0x59448: strne r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::SP,32)?)?;
    // 0x5944c: ldrne r0, [r6, #0x42c]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,1068)?)?;
    // 0x59450: strne r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::SP,36)?)?;
    // 0x59454: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59458: bl 0x798c8 <entry+0x240c8> @ imm = #0x20468
    c.emit(arm32::branch(C::Al,c.at as u32,0x798c8,true)?)?;
    // 0x5945c: add r2, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,28)?)?;
    // 0x59460: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59464: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x59468: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x5946c: bl 0x7a3ec <entry+0x24bec> @ imm = #0x20f78
    c.emit(arm32::branch(C::Al,c.at as u32,0x7a3ec,true)?)?;
    // 0x59470: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,76)?)?;
    // 0x59474: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x59478: add r7, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,7)?)?;
    // 0x5947c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59480: add r1, r9, r7, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::new(7)?,Shift::Lsr,3,false)?)?;
    // 0x59484: bl 0x798c8 <entry+0x240c8> @ imm = #0x2043c
    c.emit(arm32::branch(C::Al,c.at as u32,0x798c8,true)?)?;
    // 0x59488: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x5948c: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x59490: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x59494: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x59498: bl 0x7a234 <entry+0x24a34> @ imm = #0x20d94
    c.emit(arm32::branch(C::Al,c.at as u32,0x7a234,true)?)?;
    // 0x5949c: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,76)?)?;
    // 0x594a0: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7)?)?;
    // 0x594a4: lsr r0, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,3)?)?;
    // 0x594a8: add r7, r0, r7, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R0,R::new(7)?,Shift::Lsr,3,false)?)?;
    // 0x594ac: b 0x594c4 <entry+0x3cc4>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x594c4,false)?)?;
    // 0x594b0: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x594b4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x594b8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x594bc: bl 0x7d328 <entry+0x27b28> @ imm = #0x23e64
    c.emit(arm32::branch(C::Al,c.at as u32,0x7d328,true)?)?;
    // 0x594c0: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x594c4: bl 0x20d0 <entry+0xfffffffffffac8d0> @ imm = #-0x573fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x20d0,true)?)?;
    // 0x594c8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x594cc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x594d0: bl 0x205c <entry+0xfffffffffffac85c> @ imm = #-0x5747c
    c.emit(arm32::branch(C::Al,c.at as u32,0x205c,true)?)?;
    // 0x594d4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x594d8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x594dc: sub r2, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(7)?,1)?)?;
    // 0x594e0: orr r0, r0, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // 0x594e4: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x594e8: ldr r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,20)?)?;
    // 0x594ec: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x594f0: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x594f4: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x594f8: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x594fc: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // 0x59500: str r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,16)?)?;
    // 0x59504: str r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,12)?)?;
    // 0x59508: ldr r1, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // 0x5950c: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x59510: bhs 0x595dc <entry+0x3ddc>  @ imm = #0xc4
    c.emit(arm32::branch(C::Cs,c.at as u32,0x595dc,false)?)?;
    // 0x59514: str r5, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,24)?)?;
    // 0x59518: add r1, r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,31)?)?;
    // 0x5951c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x59520: ldr r8, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(6)?,4)?)?;
    // 0x59524: bic r10, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(10)?,R::R1,31)?)?;
    // 0x59528: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x5952c: bl 0x271bc <entry+0xfffffffffffd19bc> @ imm = #-0x32378
    c.emit(arm32::branch(C::Al,c.at as u32,0x271bc,true)?)?;
    // 0x59530: ldr r1, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,4)?)?;
    // 0x59534: cmp r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    // 0x59538: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x5953c: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x59540: moveq r5, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x59544: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x59548: beq 0x595a0 <entry+0x3da0>  @ imm = #0x50
    c.emit(arm32::branch(C::Eq,c.at as u32,0x595a0,false)?)?;
    // 0x5954c: ldr r1, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,4)?)?;
    // 0x59550: ldr r2, [pc, #0xa4]         @ 0x3dfc <entry+0x3dfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,164)?)?;
    // 0x59554: mov r3, #119
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,119)?)?;
    // 0x59558: orr r3, r3, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4864)?)?;
    // 0x5955c: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x59560: ldr r1, [pc, #0x90]         @ 0x3df8 <entry+0x3df8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,144)?)?;
    // 0x59564: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x59568: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5956c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2a8e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x59570: ldr r1, [pc, #0x88]         @ 0x3e00 <entry+0x3e00>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,136)?)?;
    // 0x59574: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x59578: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2a8ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5957c: ldr r1, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,20)?)?;
    // 0x59580: ldr r0, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // 0x59584: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x59588: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5958c: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x59590: str r8, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,4)?)?;
    // 0x59594: str r8, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,8)?)?;
    // 0x59598: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5959c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x2ec14
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x595a0: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x595a4: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x595a8: ldr r0, [r6, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x595ac: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,16)?)?;
    // 0x595b0: ldr r2, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,4)?)?;
    // 0x595b4: ldr r3, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,644)?)?;
    // 0x595b8: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x595bc: stmib sp, {r0, r2, r3, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x595c0: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x595c4: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x595c8: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x595cc: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x595d0: bl 0x51d08 <entry+0xffffffffffffc508> @ imm = #-0x78d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x51d08,true)?)?;
    // 0x595d4: str r7, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,16)?)?;
    // 0x595d8: b 0x595f0 <entry+0x3df0>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x595f0,false)?)?;
    // 0x595dc: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x595e0: cmp r7, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x595e4: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x595e8: movls r0, #6
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x595ec: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x595f0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x595f4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x595f8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_595F8:[u32;3]=[0x000ee628,0x000ee672,0x000e2788];
 for value in INLINE_DATA_595F8{c.emit(value)?;}
 // Executable A32 block 0x59604..0x59b28.
 c.check(0x59604)?;
    // 0x59604: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x59608: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x5960c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x59610: bl 0x6383c <entry+0xe03c>  @ imm = #0xa224
    c.emit(arm32::branch(C::Al,c.at as u32,0x6383c,true)?)?;
    // 0x59614: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59618: bl 0x79314 <entry+0x23b14> @ imm = #0x1fcf4
    c.emit(arm32::branch(C::Al,c.at as u32,0x79314,true)?)?;
    // 0x5961c: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x59620: bl 0x74ab0 <entry+0x1f2b0> @ imm = #0x1b488
    c.emit(arm32::branch(C::Al,c.at as u32,0x74ab0,true)?)?;
    // 0x59624: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x59628: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5962c: popeq {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x59630: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x59634: b 0x76114 <entry+0x20914> @ imm = #0x1cad8
    c.emit(arm32::branch(C::Al,c.at as u32,0x76114,false)?)?;
    // 0x59638: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5963c: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x59640: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x59644: ldr r2, [pc, #0x4dc]        @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1244)?)?;
    // 0x59648: mov r1, #165
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,165)?)?;
    // 0x5964c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x59650: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x59654: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x59658: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5965c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x59660: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59664: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x57ebc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59668: str r5, [r4, #0xb18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,2840)?)?;
    // 0x5966c: str r5, [r4, #0xb1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,2844)?)?;
    // 0x59670: ldr r0, [r4, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1524)?)?;
    // 0x59674: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x59678: str r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,96)?)?;
    // 0x5967c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x59680: str r5, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,100)?)?;
    // 0x59684: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x56790
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x59688: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5968c: str r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,92)?)?;
    // 0x59690: beq 0x59854 <entry+0x4054>  @ imm = #0x1bc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59854,false)?)?;
    // 0x59694: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x59698: str r0, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,120)?)?;
    // 0x5969c: ldr r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // 0x596a0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x596a4: blt 0x59710 <entry+0x3f10>  @ imm = #0x64
    c.emit(arm32::branch(C::Lt,c.at as u32,0x59710,false)?)?;
    // 0x596a8: ldr r9, [pc, #0x478]        @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::PC,1144)?)?;
    // 0x596ac: mov r8, #187
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,187)?)?;
    // 0x596b0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x596b4: mvn r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(5)?,R::R0,0)?)?;
    // 0x596b8: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x596bc: orr r8, r8, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,4864)?)?;
    // 0x596c0: mov r0, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,216)?)?;
    // 0x596c4: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x567d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x596c8: ldr r1, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,92)?)?;
    // 0x596cc: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x596d0: str r0, [r1, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x596d4: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x596d8: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // 0x596dc: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x596e0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x596e4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x596e8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x57f40
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x596ec: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // 0x596f0: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x596f4: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x596f8: str r5, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,168)?)?;
    // 0x596fc: str r7, [r0, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,140)?)?;
    // 0x59700: str r7, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,144)?)?;
    // 0x59704: ldr r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // 0x59708: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5970c: blt 0x596c0 <entry+0x3ec0>  @ imm = #-0x54
    c.emit(arm32::branch(C::Lt,c.at as u32,0x596c0,false)?)?;
    // 0x59710: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x59714: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x59718: str r6, [r5, #0x68]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(6)?,R::new(5)?,104)?)?;
    // 0x5971c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x59720: bl 0x2b8c <entry+0xfffffffffffad38c> @ imm = #-0x56b9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2b8c,true)?)?;
    // 0x59724: ldr r2, [pc, #0x3fc]        @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1020)?)?;
    // 0x59728: mov r1, #195
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,195)?)?;
    // 0x5972c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59730: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x59734: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59738: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x5973c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x57f94
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59740: add r0, r5, #104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,104)?)?;
    // 0x59744: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x59748: str r6, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,4)?)?;
    // 0x5974c: str r6, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,8)?)?;
    // 0x59750: str r6, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,12)?)?;
    // 0x59754: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x59758: bl 0x3564c <entry+0xfffffffffffdfe4c> @ imm = #-0x24114
    c.emit(arm32::branch(C::Al,c.at as u32,0x3564c,true)?)?;
    // 0x5975c: ldrb r0, [r5, #0xbb8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,3000)?)?;
    // 0x59760: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59764: beq 0x597a4 <entry+0x3fa4>  @ imm = #0x38
    c.emit(arm32::branch(C::Eq,c.at as u32,0x597a4,false)?)?;
    // 0x59768: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x5976c: mov r1, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,716)?)?;
    // 0x59770: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x59774: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x59778: str r6, [r5, #0x8c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(6)?,R::new(5)?,140)?)?;
    // 0x5977c: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x59780: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x59784: bl 0x7e544 <entry+0x28d44> @ imm = #0x24db8
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e544,true)?)?;
    // 0x59788: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5978c: ldr r2, [pc, #0x394]        @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,916)?)?;
    // 0x59790: mov r1, #207
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,207)?)?;
    // 0x59794: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x59798: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5979c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x597a0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x57ff8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x597a4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x597a8: str r6, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,132)?)?;
    // 0x597ac: str r6, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,136)?)?;
    // 0x597b0: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x568bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x597b4: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x597b8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x597bc: str r1, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,36)?)?;
    // 0x597c0: beq 0x59860 <entry+0x4060>  @ imm = #0x98
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59860,false)?)?;
    // 0x597c4: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x597c8: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x597cc: add r5, r4, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,640)?)?;
    // 0x597d0: bl 0x2aa0c <entry+0xfffffffffffd520c> @ imm = #-0x2edcc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2aa0c,true)?)?;
    // 0x597d4: ldr r2, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,280)?)?;
    // 0x597d8: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x597dc: bne 0x597f0 <entry+0x3ff0>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x597f0,false)?)?;
    // 0x597e0: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x597e4: ldr r1, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,24)?)?;
    // 0x597e8: bl 0x26d0 <entry+0xfffffffffffaced0> @ imm = #-0x57120
    c.emit(arm32::branch(C::Al,c.at as u32,0x26d0,true)?)?;
    // 0x597ec: ldr r2, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,280)?)?;
    // 0x597f0: add r1, r4, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,44)?)?;
    // 0x597f4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x597f8: str r4, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(4)?,60)?)?;
    // 0x597fc: bl 0x76460 <entry+0x20c60> @ imm = #0x1cc5c
    c.emit(arm32::branch(C::Al,c.at as u32,0x76460,true)?)?;
    // 0x59800: ldr r2, [pc, #0x320]        @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,800)?)?;
    // 0x59804: mov r1, #243
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,243)?)?;
    // 0x59808: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5980c: str r0, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,64)?)?;
    // 0x59810: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59814: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x59818: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58070
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5981c: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x59820: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x59824: bne 0x5986c <entry+0x406c>  @ imm = #0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5986c,false)?)?;
    // 0x59828: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x5982c: ldr r1, [pc, #0x2f8]        @ 0x432c <entry+0x432c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,760)?)?;
    // 0x59830: ldr r3, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,44)?)?;
    // 0x59834: add r2, r4, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,60)?)?;
    // 0x59838: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5983c: ldr r3, [r3, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,20)?)?;
    // 0x59840: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x59844: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x59848: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // 0x5984c: bl 0x2550 <entry+0xfffffffffffacd50> @ imm = #-0x57304
    c.emit(arm32::branch(C::Al,c.at as u32,0x2550,true)?)?;
    // 0x59850: b 0x5987c <entry+0x407c>  @ imm = #0x24
    c.emit(arm32::branch(C::Al,c.at as u32,0x5987c,false)?)?;
    // 0x59854: mov r1, #177
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,177)?)?;
    // 0x59858: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x5985c: b 0x59b18 <entry+0x4318>  @ imm = #0x2b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x59b18,false)?)?;
    // 0x59860: mov r1, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,992)?)?;
    // 0x59864: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x59868: b 0x59b18 <entry+0x4318>  @ imm = #0x2a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x59b18,false)?)?;
    // 0x5986c: ldr r1, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,276)?)?;
    // 0x59870: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x59874: ldr r1, [r1, #0x140]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,320)?)?;
    // 0x59878: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,20)?)?;
    // 0x5987c: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x59880: ldr r1, [pc, #0x2a8]        @ 0x4330 <entry+0x4330>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,680)?)?;
    // 0x59884: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x59888: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2abfc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5988c: ldr r1, [pc, #0x2a0]        @ 0x4334 <entry+0x4334>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,672)?)?;
    // 0x59890: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x59894: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ac08
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x59898: str r4, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(4)?,68)?)?;
    // 0x5989c: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // 0x598a0: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x598a4: ldr r3, [r6, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x598a8: ldr r7, [pc, #0x288]        @ 0x4338 <entry+0x4338>
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::PC,648)?)?;
    // 0x598ac: ldr r2, [r6, r7]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(6)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x598b0: ldr r0, [pc, #0x284]        @ 0x433c <entry+0x433c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,644)?)?;
    // 0x598b4: add r1, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x598b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x598bc: bl 0x73c48 <entry+0x1e448> @ imm = #0x1a384
    c.emit(arm32::branch(C::Al,c.at as u32,0x73c48,true)?)?;
    // 0x598c0: ldr r1, [pc, #0x278]        @ 0x4340 <entry+0x4340>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,632)?)?;
    // 0x598c4: ldr r2, [pc, #0x25c]        @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,604)?)?;
    // 0x598c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x598cc: str r0, [r6, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x598d0: mov r1, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,9)?)?;
    // 0x598d4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x598d8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x598dc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58134
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x598e0: ldr r0, [r6, r7]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x598e4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x598e8: bne 0x59914 <entry+0x4114>  @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59914,false)?)?;
    // 0x598ec: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x598f0: ldr r1, [pc, #0x234]        @ 0x432c <entry+0x432c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,564)?)?;
    // 0x598f4: ldr r3, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,48)?)?;
    // 0x598f8: add r2, r4, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,68)?)?;
    // 0x598fc: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x59900: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x59904: ldr r3, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,4)?)?;
    // 0x59908: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x5990c: bl 0x2550 <entry+0xfffffffffffacd50> @ imm = #-0x573c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2550,true)?)?;
    // 0x59910: b 0x59924 <entry+0x4124>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x59924,false)?)?;
    // 0x59914: ldr r1, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,276)?)?;
    // 0x59918: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x5991c: ldr r1, [r1, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,316)?)?;
    // 0x59920: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x59924: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x59928: ldr r1, [pc, #0x214]        @ 0x4344 <entry+0x4344>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,532)?)?;
    // 0x5992c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x59930: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2aca4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x59934: ldr r1, [pc, #0x1f8]        @ 0x4334 <entry+0x4334>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,504)?)?;
    // 0x59938: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5993c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2acb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x59940: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x59944: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x59948: bne 0x599b0 <entry+0x41b0>  @ imm = #0x60
    c.emit(arm32::branch(C::Ne,c.at as u32,0x599b0,false)?)?;
    // 0x5994c: str r4, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(4)?,76)?)?;
    // 0x59950: add r1, r4, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,52)?)?;
    // 0x59954: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x59958: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // 0x5995c: bl 0x7562c <entry+0x1fe2c> @ imm = #0x1bcc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x7562c,true)?)?;
    // 0x59960: ldr r2, [pc, #0x1c0]        @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,448)?)?;
    // 0x59964: mov r1, #39
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,39)?)?;
    // 0x59968: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5996c: str r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // 0x59970: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59974: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x59978: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x581d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5997c: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x59980: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x59984: bne 0x599bc <entry+0x41bc>  @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x599bc,false)?)?;
    // 0x59988: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x5998c: ldr r1, [pc, #0x198]        @ 0x432c <entry+0x432c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,408)?)?;
    // 0x59990: ldr r3, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,52)?)?;
    // 0x59994: add r2, r4, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,76)?)?;
    // 0x59998: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x5999c: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x599a0: ldr r3, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,4)?)?;
    // 0x599a4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x599a8: bl 0x2550 <entry+0xfffffffffffacd50> @ imm = #-0x57460
    c.emit(arm32::branch(C::Al,c.at as u32,0x2550,true)?)?;
    // 0x599ac: b 0x599cc <entry+0x41cc>  @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x599cc,false)?)?;
    // 0x599b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x599b4: str r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,52)?)?;
    // 0x599b8: b 0x599e8 <entry+0x41e8>  @ imm = #0x28
    c.emit(arm32::branch(C::Al,c.at as u32,0x599e8,false)?)?;
    // 0x599bc: ldr r1, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,276)?)?;
    // 0x599c0: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x599c4: ldr r1, [r1, #0x138]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,312)?)?;
    // 0x599c8: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x599cc: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x599d0: ldr r1, [pc, #0x170]        @ 0x4348 <entry+0x4348>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,368)?)?;
    // 0x599d4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x599d8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ad4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x599dc: ldr r1, [pc, #0x150]        @ 0x4334 <entry+0x4334>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,336)?)?;
    // 0x599e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x599e4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ad58
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x599e8: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x599ec: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x599f0: bne 0x59a48 <entry+0x4248>  @ imm = #0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59a48,false)?)?;
    // 0x599f4: str r4, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(4)?,84)?)?;
    // 0x599f8: add r1, r4, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,56)?)?;
    // 0x599fc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x59a00: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // 0x59a04: bl 0x72bc8 <entry+0x1d3c8> @ imm = #0x191bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x72bc8,true)?)?;
    // 0x59a08: ldr r2, [pc, #0x118]        @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,280)?)?;
    // 0x59a0c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59a10: str r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,88)?)?;
    // 0x59a14: mov r1, #5184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5184)?)?;
    // 0x59a18: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59a1c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58274
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59a20: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x59a24: ldr r1, [pc, #0x100]        @ 0x432c <entry+0x432c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,256)?)?;
    // 0x59a28: ldr r2, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,56)?)?;
    // 0x59a2c: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x59a30: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // 0x59a34: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x59a38: add r2, r4, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,84)?)?;
    // 0x59a3c: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x59a40: bl 0x2550 <entry+0xfffffffffffacd50> @ imm = #-0x574f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2550,true)?)?;
    // 0x59a44: b 0x59a70 <entry+0x4270>  @ imm = #0x24
    c.emit(arm32::branch(C::Al,c.at as u32,0x59a70,false)?)?;
    // 0x59a48: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x59a4c: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x56b58
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x59a50: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59a54: str r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // 0x59a58: beq 0x59b10 <entry+0x4310>  @ imm = #0xb0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59b10,false)?)?;
    // 0x59a5c: ldr r1, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,276)?)?;
    // 0x59a60: add r2, r1, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,200)?)?;
    // 0x59a64: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x59a68: ldr r1, [r1, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,324)?)?;
    // 0x59a6c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x59a70: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x59a74: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x59a78: bne 0x59a84 <entry+0x4284>  @ imm = #0x4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59a84,false)?)?;
    // 0x59a7c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x59a80: bl 0x2f928 <entry+0xfffffffffffda128> @ imm = #-0x2a160
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f928,true)?)?;
    // 0x59a84: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x59a88: ldr r1, [pc, #0xbc]         @ 0x434c <entry+0x434c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,188)?)?;
    // 0x59a8c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x59a90: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ae04
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x59a94: ldr r1, [pc, #0x98]         @ 0x4334 <entry+0x4334>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,152)?)?;
    // 0x59a98: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x59a9c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ae10
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x59aa0: ldr r0, [r4, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,672)?)?;
    // 0x59aa4: str r0, [r4, #0xcf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3320)?)?;
    // 0x59aa8: ldr r0, [r4, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,676)?)?;
    // 0x59aac: str r0, [r4, #0xcfc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3324)?)?;
    // 0x59ab0: ldrb r0, [r4, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3272)?)?;
    // 0x59ab4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59ab8: movne r0, r4
    c.emit(arm32::mov_register(C::Ne,R::R0,R::new(4)?))?;
    // 0x59abc: blne 0x59b50 <entry+0x4350>  @ imm = #0x8c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59b50,true)?)?;
    // 0x59ac0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59ac4: bl 0x59c88 <entry+0x4488>  @ imm = #0x1bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x59c88,true)?)?;
    // 0x59ac8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x59acc: strb r0, [r4, #0x47d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1149)?)?;
    // 0x59ad0: ldrb r0, [r4, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1148)?)?;
    // 0x59ad4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59ad8: bne 0x59ae8 <entry+0x42e8>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59ae8,false)?)?;
    // 0x59adc: ldr r0, [r4, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1244)?)?;
    // 0x59ae0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x59ae4: popne {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x59ae8: ldrb r1, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,990)?)?;
    // 0x59aec: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x59af0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x59af4: beq 0x59b08 <entry+0x4308>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59b08,false)?)?;
    // 0x59af8: ldr r1, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2744)?)?;
    // 0x59afc: ldrb r1, [r1, #0x4b6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,1206)?)?;
    // 0x59b00: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x59b04: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59b08: strb r0, [r4, #0x47d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1149)?)?;
    // 0x59b0c: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x59b10: mov r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,77)?)?;
    // 0x59b14: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x59b18: ldr r2, [pc, #0x8]          @ 0x4328 <entry+0x4328>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,8)?)?;
    // 0x59b1c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x59b20: pop {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x59b24: b 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5837c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,false)?)?;
 c.check(0x59b28)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_59B28:[u32;10]=[0x000ee470,0x0010ceac,0x000ee68a,0x000e2788,0xffffe918,0xffffe830,0xffffe848,0x000ee6be,0x000ee6f1,0x000ee724];
 for value in INLINE_DATA_59B28{c.emit(value)?;}
 // Executable A32 block 0x59b50..0x5a000.
 c.check(0x59b50)?;
    // 0x59b50: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x59b54: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x59b58: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x59b5c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x59b60: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x59b64: add r0, r4, #296
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,296)?)?;
    // 0x59b68: add r5, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,3072)?)?;
    // 0x59b6c: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x59b70: ldr r6, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,40)?)?;
    // 0x59b74: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x59b78: ldrb r8, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::R0,336)?)?;
    // 0x59b7c: ands r10, r8, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(10)?,R::new(8)?,1,true)?)?;
    // 0x59b80: ldrne r1, [r4, #0xd74]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,3444)?)?;
    // 0x59b84: ldrne r2, [r4, #0xd78]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(4)?,3448)?)?;
    // 0x59b88: strne r2, [r4, #0xd74]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::new(4)?,3444)?)?;
    // 0x59b8c: strne r1, [r4, #0xd78]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(4)?,3448)?)?;
    // 0x59b90: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x59b94: strb r1, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,336)?)?;
    // 0x59b98: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x59b9c: ldr r9, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,644)?)?;
    // 0x59ba0: str r1, [r4, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3372)?)?;
    // 0x59ba4: str r1, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,644)?)?;
    // 0x59ba8: strb r7, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R0,339)?)?;
    // 0x59bac: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59bb0: bl 0x54cfc <entry+0xfffffffffffff4fc> @ imm = #-0x4ebc
    c.emit(arm32::branch(C::Al,c.at as u32,0x54cfc,true)?)?;
    // 0x59bb4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59bb8: strb r7, [r4, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3272)?)?;
    // 0x59bbc: bl 0x59c88 <entry+0x4488>  @ imm = #0xc4
    c.emit(arm32::branch(C::Al,c.at as u32,0x59c88,true)?)?;
    // 0x59bc0: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // 0x59bc4: mov r6, sp
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::SP))?;
    // 0x59bc8: ldr r2, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,76)?)?;
    // 0x59bcc: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x59bd0: add r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    // 0x59bd4: sub r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x59bd8: add r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x59bdc: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x59be0: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x59be4: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x59be8: str r0, [r4, #0x948]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2376)?)?;
    // 0x59bec: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59bf0: bl 0x43ff0 <entry+0xfffffffffffee7f0> @ imm = #-0x15c08
    c.emit(arm32::branch(C::Al,c.at as u32,0x43ff0,true)?)?;
    // 0x59bf4: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x59bf8: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x59bfc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59c00: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x59c04: bl 0x43b50 <entry+0xfffffffffffee350> @ imm = #-0x160bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x43b50,true)?)?;
    // 0x59c08: add r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x59c0c: str r0, [r4, #0x94c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2380)?)?;
    // 0x59c10: ldr r0, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,716)?)?;
    // 0x59c14: str r0, [r4, #0xd48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3400)?)?;
    // 0x59c18: ldr r1, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,720)?)?;
    // 0x59c1c: str r1, [r4, #0xd4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3404)?)?;
    // 0x59c20: str r0, [r4, #0xd5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3420)?)?;
    // 0x59c24: mov r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // 0x59c28: str r1, [r4, #0xd60]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3424)?)?;
    // 0x59c2c: str r9, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,644)?)?;
    // 0x59c30: str r9, [r4, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,3372)?)?;
    // 0x59c34: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x59c38: str r6, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x59c3c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x59c40: ldr r1, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1392)?)?;
    // 0x59c44: bl 0x58130 <entry+0x2930>  @ imm = #-0x1b1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x58130,true)?)?;
    // 0x59c48: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x59c4c: bl 0x58214 <entry+0x2a14>  @ imm = #-0x1a40
    c.emit(arm32::branch(C::Al,c.at as u32,0x58214,true)?)?;
    // 0x59c50: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x59c54: ldrne r0, [r4, #0xd74]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,3444)?)?;
    // 0x59c58: ldrne r1, [r4, #0xd78]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,3448)?)?;
    // 0x59c5c: strne r1, [r4, #0xd74]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(4)?,3444)?)?;
    // 0x59c60: strne r0, [r4, #0xd78]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,3448)?)?;
    // 0x59c64: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x59c68: strb r8, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::R0,336)?)?;
    // 0x59c6c: strb r6, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R0,339)?)?;
    // 0x59c70: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59c74: bl 0x54cfc <entry+0xfffffffffffff4fc> @ imm = #-0x4f80
    c.emit(arm32::branch(C::Al,c.at as u32,0x54cfc,true)?)?;
    // 0x59c78: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59c7c: strb r0, [r4, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3272)?)?;
    // 0x59c80: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x59c84: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x59c88: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x59c8c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x59c90: sub sp, sp, #108
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,108)?)?;
    // 0x59c94: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x59c98: add r5, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,12)?)?;
    // 0x59c9c: ldr r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // 0x59ca0: add r10, r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(5)?,16)?)?;
    // 0x59ca4: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x59ca8: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x59cac: ldr r0, [r4, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,252)?)?;
    // 0x59cb0: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x59cb4: ldr r0, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,256)?)?;
    // 0x59cb8: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x59cbc: ldr r0, [r4, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,260)?)?;
    // 0x59cc0: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x59cc4: ldr r0, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,264)?)?;
    // 0x59cc8: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x59ccc: ldr r0, [r4, #0x10c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,268)?)?;
    // 0x59cd0: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x59cd4: ldr r0, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,240)?)?;
    // 0x59cd8: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x59cdc: ldr r0, [r4, #0xf4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,244)?)?;
    // 0x59ce0: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x59ce4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59ce8: bl 0x54854 <entry+0xfffffffffffff054> @ imm = #-0x549c
    c.emit(arm32::branch(C::Al,c.at as u32,0x54854,true)?)?;
    // 0x59cec: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x59cf0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x59cf4: bl 0x797ec <entry+0x23fec> @ imm = #0x1faf0
    c.emit(arm32::branch(C::Al,c.at as u32,0x797ec,true)?)?;
    // 0x59cf8: ldr r2, [r4, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1764)?)?;
    // 0x59cfc: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59d00: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x59d04: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24730
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x59d08: ldr r2, [pc, #0x938]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2360)?)?;
    // 0x59d0c: mov r1, #173
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,173)?)?;
    // 0x59d10: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59d14: str r0, [r4, #0x79c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1948)?)?;
    // 0x59d18: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59d1c: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x59d20: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58578
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59d24: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x59d28: str r10, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,4)?)?;
    // 0x59d2c: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x59d30: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x59d34: beq 0x59d50 <entry+0x4550>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59d50,false)?)?;
    // 0x59d38: ldr r0, [r4, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,940)?)?;
    // 0x59d3c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59d40: bne 0x59d50 <entry+0x4550>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59d50,false)?)?;
    // 0x59d44: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x59d48: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x59d4c: blo 0x59de0 <entry+0x45e0>  @ imm = #0x8c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x59de0,false)?)?;
    // 0x59d50: ldr r8, [r4, #0x5fc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,1532)?)?;
    // 0x59d54: ldr r2, [pc, #0x8ec]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2284)?)?;
    // 0x59d58: ldr r7, [r4, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1224)?)?;
    // 0x59d5c: ldr r6, [r4, #0x640]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1600)?)?;
    // 0x59d60: mov r1, #181
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,181)?)?;
    // 0x59d64: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x59d68: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x59d6c: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x59d70: cmp r8, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,17)?)?;
    // 0x59d74: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59d78: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x585d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59d7c: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x59d80: beq 0x59dd0 <entry+0x45d0>  @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59dd0,false)?)?;
    // 0x59d84: add r0, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,12)?)?;
    // 0x59d88: add r4, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::SP,44)?)?;
    // 0x59d8c: add r5, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,8)?)?;
    // 0x59d90: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x59d94: add r10, r0, #2272
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::R0,2272)?)?;
    // 0x59d98: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x59d9c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x59da0: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x59da4: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x247d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x59da8: ldr r2, [pc, #0x898]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2200)?)?;
    // 0x59dac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59db0: str r0, [r4], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x59db4: str r0, [r10], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::new(10)?,4)?)?;
    // 0x59db8: mov r1, #956
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,956)?)?;
    // 0x59dbc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59dc0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58618
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59dc4: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x59dc8: cmp r9, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x59dcc: bne 0x59d98 <entry+0x4598>  @ imm = #-0x3c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59d98,false)?)?;
    // 0x59dd0: ldr r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,8)?)?;
    // 0x59dd4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x59dd8: str r0, [r4, #0x614]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1556)?)?;
    // 0x59ddc: ldr r10, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,4)?)?;
    // 0x59de0: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x59de4: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x59de8: cmpne r0, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x59dec: bne 0x59f70 <entry+0x4770>  @ imm = #0x17c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59f70,false)?)?;
    // 0x59df0: ldr r2, [r4, #0x64c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1612)?)?;
    // 0x59df4: ldr r1, [r4, #0x610]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1552)?)?;
    // 0x59df8: ldr r0, [r4, #0x604]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1540)?)?;
    // 0x59dfc: mov r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // 0x59e00: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,255)?)?;
    // 0x59e04: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x59e08: lsr r2, r2, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsr,8)?)?;
    // 0x59e0c: orr r1, r3, r1, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,10,false)?)?;
    // 0x59e10: mul r3, r0, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    // 0x59e14: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59e18: add r2, r1, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x59e1c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x59e20: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x2484c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x59e24: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x59e28: str r0, [r4, #0x820]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2080)?)?;
    // 0x59e2c: ldr r1, [r4, #0x610]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1552)?)?;
    // 0x59e30: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x59e34: beq 0x59e64 <entry+0x4664>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59e64,false)?)?;
    // 0x59e38: add r1, r4, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,52)?)?;
    // 0x59e3c: add r2, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,256)?)?;
    // 0x59e40: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x59e44: add r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2048)?)?;
    // 0x59e48: str r2, [r1, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x59e4c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x59e50: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x59e54: ldr r3, [r4, #0x610]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1552)?)?;
    // 0x59e58: cmp r7, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x59e5c: blo 0x59e48 <entry+0x4648>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x59e48,false)?)?;
    // 0x59e60: b 0x59e68 <entry+0x4668>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x59e68,false)?)?;
    // 0x59e64: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x59e68: ldr r1, [r4, #0x604]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1540)?)?;
    // 0x59e6c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x59e70: beq 0x59ed4 <entry+0x46d4>  @ imm = #0x5c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59ed4,false)?)?;
    // 0x59e74: add r1, r3, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x59e78: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x59e7c: add r1, r0, r1, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,10,false)?)?;
    // 0x59e80: str r1, [r4, #0x824]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2084)?)?;
    // 0x59e84: ldr r1, [r4, #0x604]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1540)?)?;
    // 0x59e88: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x59e8c: blo 0x59ed4 <entry+0x46d4>  @ imm = #0x40
    c.emit(arm32::branch(C::Cc,c.at as u32,0x59ed4,false)?)?;
    // 0x59e90: add r1, r4, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,40)?)?;
    // 0x59e94: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x59e98: mov r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // 0x59e9c: add r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2048)?)?;
    // 0x59ea0: ldr r7, [r4, #0x610]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1552)?)?;
    // 0x59ea4: ldr r6, [r4, #0x64c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1612)?)?;
    // 0x59ea8: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x59eac: add r7, r7, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x59eb0: add r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,255)?)?;
    // 0x59eb4: add r7, r0, r7, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R0,R::new(7)?,Shift::Lsl,10,false)?)?;
    // 0x59eb8: lsr r6, r6, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(6)?,Shift::Lsr,8)?)?;
    // 0x59ebc: mla r5, r3, r6, r7
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(3)?,R::new(6)?,R::new(7)?)?)?;
    // 0x59ec0: add r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,256)?)?;
    // 0x59ec4: str r5, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(5)?,R::R1,4)?)?;
    // 0x59ec8: ldr r7, [r4, #0x604]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1540)?)?;
    // 0x59ecc: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x59ed0: blo 0x59ea0 <entry+0x46a0>  @ imm = #-0x38
    c.emit(arm32::branch(C::Cc,c.at as u32,0x59ea0,false)?)?;
    // 0x59ed4: ldr r0, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1528)?)?;
    // 0x59ed8: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x59edc: add r6, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,1)?)?;
    // 0x59ee0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59ee4: lsl r2, r6, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(6)?,Shift::Lsl,8)?)?;
    // 0x59ee8: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24914
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x59eec: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x59ef0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x59ef4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x59ef8: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // 0x59efc: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x59f00: str r5, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,32)?)?;
    // 0x59f04: beq 0x59f5c <entry+0x475c>  @ imm = #0x50
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59f5c,false)?)?;
    // 0x59f08: ldr r8, [pc, #0x738]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::PC,1848)?)?;
    // 0x59f0c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x59f10: bl 0x35c14 <entry+0xfffffffffffe0414> @ imm = #-0x24304
    c.emit(arm32::branch(C::Al,c.at as u32,0x35c14,true)?)?;
    // 0x59f14: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59f18: beq 0x5a630 <entry+0x4e30>  @ imm = #0x710
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5a630,false)?)?;
    // 0x59f1c: ldr r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // 0x59f20: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x59f24: add r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x59f28: str r1, [r0, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,116)?)?;
    // 0x59f2c: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x59f30: mov r1, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1004)?)?;
    // 0x59f34: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5878c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59f38: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // 0x59f3c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x59f40: add r5, r5, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,256)?)?;
    // 0x59f44: cmp r7, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x59f48: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x59f4c: ldr r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,32)?)?;
    // 0x59f50: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x59f54: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x59f58: blo 0x59f10 <entry+0x4710>  @ imm = #-0x50
    c.emit(arm32::branch(C::Cc,c.at as u32,0x59f10,false)?)?;
    // 0x59f5c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x59f60: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x59f64: mov r0, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,84)?)?;
    // 0x59f68: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // 0x59f6c: strh r1, [r4, r0]
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::new(4)?,R::R0,false)?)?;
    // 0x59f70: ldr r2, [r4, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1696)?)?;
    // 0x59f74: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59f78: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x59f7c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x249a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x59f80: ldr r7, [pc, #0x6c0]        @ 0x4e48 <entry+0x4e48>
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::PC,1728)?)?;
    // 0x59f84: mov r1, #251
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,251)?)?;
    // 0x59f88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59f8c: str r0, [r4, #0x7a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1952)?)?;
    // 0x59f90: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59f94: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x59f98: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x59f9c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x587f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59fa0: ldr r2, [r4, #0x6a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1700)?)?;
    // 0x59fa4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59fa8: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x59fac: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x249d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x59fb0: str r0, [r4, #0x7a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1956)?)?;
    // 0x59fb4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59fb8: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x59fbc: ldr r2, [r4, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1708)?)?;
    // 0x59fc0: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x249ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x59fc4: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x59fc8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59fcc: str r0, [r4, #0x7a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1960)?)?;
    // 0x59fd0: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x59fd4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x59fd8: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x59fdc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x58834
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x59fe0: ldr r0, [r4, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1708)?)?;
    // 0x59fe4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x59fe8: lsr r2, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsr,1)?)?;
    // 0x59fec: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x59ff0: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x24a1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x59ff4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x59ff8: str r0, [r4, #0x7ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1964)?)?;
    // 0x59ffc: mov r1, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1040)?)?;
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
