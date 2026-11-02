// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x5f000,0x60000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x5f000;
pub const END:usize=0x60000;
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
 // Executable A32 block 0x5f000..0x5fd80.
 c.check(0x5f000)?;
    // 0x5f000: add r0, r4, #804
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,804)?)?;
    // 0x5f004: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x5f008: add r0, r4, #748
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,748)?)?;
    // 0x5f00c: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x5f010: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x5f014: b 0x5f064 <entry+0x9864>  @ imm = #0x48
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f064,false)?)?;
    // 0x5f018: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5f01c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5f020: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x5f024: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x5f028: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5f02c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5f030: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x5f034: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x5f038: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x5f03c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x346b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5f040: ldr r1, [pc, #0xd38]        @ 0xa580 <entry+0xa580>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3384)?)?;
    // 0x5f044: ldr r2, [pc, #0xd38]        @ 0xa584 <entry+0xa584>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3384)?)?;
    // 0x5f048: mov r3, #163
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,163)?)?;
    // 0x5f04c: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x5f050: b 0x5fd60 <entry+0xa560>  @ imm = #0xd08
    c.emit(arm32::branch(C::Al,c.at as u32,0x5fd60,false)?)?;
    // 0x5f054: ldrb r0, [r4, #0x401]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1025)?)?;
    // 0x5f058: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5f05c: bne 0x5fa6c <entry+0xa26c>  @ imm = #0xa08
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5fa6c,false)?)?;
    // 0x5f060: b 0x5fae4 <entry+0xa2e4>  @ imm = #0xa7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5fae4,false)?)?;
    // 0x5f064: ldr r0, [r3], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(3)?,4)?)?;
    // 0x5f068: cmp r0, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // 0x5f06c: ble 0x5f15c <entry+0x995c>  @ imm = #0xe8
    c.emit(arm32::branch(C::Le,c.at as u32,0x5f15c,false)?)?;
    // 0x5f070: ldr r1, [pc, #0xd1c]        @ 0xa594 <entry+0xa594>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3356)?)?;
    // 0x5f074: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5f078: beq 0x5f1b0 <entry+0x99b0>  @ imm = #0x130
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f1b0,false)?)?;
    // 0x5f07c: ldr r1, [pc, #0xd14]        @ 0xa598 <entry+0xa598>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3348)?)?;
    // 0x5f080: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5f084: beq 0x5f1d0 <entry+0x99d0>  @ imm = #0x144
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f1d0,false)?)?;
    // 0x5f088: ldr r1, [pc, #0xd00]        @ 0xa590 <entry+0xa590>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3328)?)?;
    // 0x5f08c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5f090: bne 0x5fd28 <entry+0xa528>  @ imm = #0xc90
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5fd28,false)?)?;
    // 0x5f094: str r3, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,88)?)?;
    // 0x5f098: ldr r1, [pc, #0xcfc]        @ 0xa59c <entry+0xa59c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3324)?)?;
    // 0x5f09c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5f0a0: ldr r2, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    // 0x5f0a4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x30418
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5f0a8: ldr r1, [pc, #0xcd8]        @ 0xa588 <entry+0xa588>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3288)?)?;
    // 0x5f0ac: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5f0b0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x30424
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5f0b4: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5f0b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5f0bc: beq 0x5f220 <entry+0x9a20>  @ imm = #0x15c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f220,false)?)?;
    // 0x5f0c0: ldr r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,92)?)?;
    // 0x5f0c4: mov r5, r7
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(7)?))?;
    // 0x5f0c8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5f0cc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5f0d0: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x5f0d4: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x5f0d8: ldr r6, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,80)?)?;
    // 0x5f0dc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x5f0e0: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x5f0e4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x5f0e8: str r1, [r5, #0x4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R1,R::new(5)?,4)?)?;
    // 0x5f0ec: ldr r0, [r4, #0x628]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1576)?)?;
    // 0x5f0f0: str r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,4)?)?;
    // 0x5f0f4: str r2, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,8)?)?;
    // 0x5f0f8: ldr r0, [r4, #0x5f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1520)?)?;
    // 0x5f0fc: str r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // 0x5f100: ldr r0, [r4, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1524)?)?;
    // 0x5f104: str r0, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,20)?)?;
    // 0x5f108: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x5f10c: str r1, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,24)?)?;
    // 0x5f110: str r0, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,28)?)?;
    // 0x5f114: ldrb r0, [r4, #0x57b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1403)?)?;
    // 0x5f118: str r5, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,84)?)?;
    // 0x5f11c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5f120: bne 0x5f2c8 <entry+0x9ac8>  @ imm = #0x1a0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f2c8,false)?)?;
    // 0x5f124: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x5f128: str r0, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,36)?)?;
    // 0x5f12c: ldr r0, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1512)?)?;
    // 0x5f130: str r0, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,40)?)?;
    // 0x5f134: str r2, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,44)?)?;
    // 0x5f138: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x5f13c: ldr r2, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1528)?)?;
    // 0x5f140: str r2, [r7, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,48)?)?;
    // 0x5f144: str r2, [r7, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,56)?)?;
    // 0x5f148: str r1, [r7, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,60)?)?;
    // 0x5f14c: str r0, [r7, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,64)?)?;
    // 0x5f150: add r1, r7, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,68)?)?;
    // 0x5f154: mov r0, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,96)?)?;
    // 0x5f158: b 0x5f2d4 <entry+0x9ad4>  @ imm = #0x174
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f2d4,false)?)?;
    // 0x5f15c: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x5f160: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5f164: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5f168: beq 0x5f1f8 <entry+0x99f8>  @ imm = #0x88
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f1f8,false)?)?;
    // 0x5f16c: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x5f170: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5f174: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5f178: bne 0x5fd28 <entry+0xa528>  @ imm = #0xba8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5fd28,false)?)?;
    // 0x5f17c: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x5f180: add r5, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(7)?,4)?)?;
    // 0x5f184: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x5f188: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x5f18c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x5f190: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x5f194: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x5f198: ldr r0, [r4, #0xd30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3376)?)?;
    // 0x5f19c: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x5f1a0: ldr r0, [r4, #0xd34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3380)?)?;
    // 0x5f1a4: str r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x5f1a8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5f1ac: b 0x5fcfc <entry+0xa4fc>  @ imm = #0xb48
    c.emit(arm32::branch(C::Al,c.at as u32,0x5fcfc,false)?)?;
    // 0x5f1b0: ldr r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,92)?)?;
    // 0x5f1b4: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // 0x5f1b8: str r0, [r7], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x5f1bc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5f1c0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x5f1c4: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x5f1c8: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // 0x5f1cc: b 0x5f218 <entry+0x9a18>  @ imm = #0x44
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f218,false)?)?;
    // 0x5f1d0: ldr r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,92)?)?;
    // 0x5f1d4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5f1d8: str r0, [r7], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x5f1dc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x5f1e0: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x5f1e4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x5f1e8: ldrb r0, [lr]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x5f1ec: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x5f1f0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5f1f4: b 0x5f218 <entry+0x9a18>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f218,false)?)?;
    // 0x5f1f8: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // 0x5f1fc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x5f200: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x5f204: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x5f208: str r2, [r7, #0x4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,4)?)?;
    // 0x5f20c: ldr r0, [r4, #0xdb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3508)?)?;
    // 0x5f210: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x5f214: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5f218: mov r5, r7
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(7)?))?;
    // 0x5f21c: b 0x5fcfc <entry+0xa4fc>  @ imm = #0xad8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5fcfc,false)?)?;
    // 0x5f220: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5f224: bl 0x54cfc <entry+0xfffffffffffff4fc> @ imm = #-0xa530
    c.emit(arm32::branch(C::Al,c.at as u32,0x54cfc,true)?)?;
    // 0x5f228: ldr r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,92)?)?;
    // 0x5f22c: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5f230: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x5f234: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x5f238: ldr r1, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,80)?)?;
    // 0x5f23c: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x5f240: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x5f244: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5f248: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f24c: str r5, [r1, #0x4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(5)?,R::R1,4)?)?;
    // 0x5f250: ldr r0, [r4, #0x628]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1576)?)?;
    // 0x5f254: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x5f258: str r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,8)?)?;
    // 0x5f25c: ldr r0, [r4, #0x5f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1520)?)?;
    // 0x5f260: str r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,84)?)?;
    // 0x5f264: str r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x5f268: ldr r1, [r4, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1524)?)?;
    // 0x5f26c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5f270: moveq r1, r0
    c.emit(arm32::mov_register(C::Eq,R::R1,R::R0))?;
    // 0x5f274: streq r0, [r4, #0x5f4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,1524)?)?;
    // 0x5f278: str r1, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,24)?)?;
    // 0x5f27c: str r5, [r7, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,28)?)?;
    // 0x5f280: ldr r2, [r4, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1276)?)?;
    // 0x5f284: str r2, [r7, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,32)?)?;
    // 0x5f288: ldrb r2, [r4, #0x57c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1404)?)?;
    // 0x5f28c: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x5f290: bne 0x5f3ec <entry+0x9bec>  @ imm = #0x154
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f3ec,false)?)?;
    // 0x5f294: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x5f298: add r6, r7, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,68)?)?;
    // 0x5f29c: str r2, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,36)?)?;
    // 0x5f2a0: ldr r2, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1504)?)?;
    // 0x5f2a4: str r2, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,40)?)?;
    // 0x5f2a8: str r3, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,44)?)?;
    // 0x5f2ac: str r0, [r7, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // 0x5f2b0: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x5f2b4: str r1, [r7, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,56)?)?;
    // 0x5f2b8: str r5, [r7, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,60)?)?;
    // 0x5f2bc: str r0, [r7, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,64)?)?;
    // 0x5f2c0: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x5f2c4: b 0x5f3f4 <entry+0x9bf4>  @ imm = #0x128
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f3f4,false)?)?;
    // 0x5f2c8: ldr r2, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1528)?)?;
    // 0x5f2cc: add r1, r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,32)?)?;
    // 0x5f2d0: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x5f2d4: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x5f2d8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5f2dc: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x5f2e0: mov r12, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,8192)?)?;
    // 0x5f2e4: ldr r3, [r4, #0x63c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1596)?)?;
    // 0x5f2e8: orr r12, r12, #2097152
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,2097152)?)?;
    // 0x5f2ec: stmib r1, {r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5f2f0: str r2, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,12)?)?;
    // 0x5f2f4: str r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,20)?)?;
    // 0x5f2f8: str r7, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,24)?)?;
    // 0x5f2fc: ldr r2, [r4, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1224)?)?;
    // 0x5f300: str r2, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,28)?)?;
    // 0x5f304: ldr r2, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,56)?)?;
    // 0x5f308: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x5f30c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5f310: beq 0x5f33c <entry+0x9b3c>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f33c,false)?)?;
    // 0x5f314: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x5f318: add r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x5f31c: str r3, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,32)?)?;
    // 0x5f320: add r3, r1, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,36)?)?;
    // 0x5f324: stm r3, {r2, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R2.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5f328: str r7, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,52)?)?;
    // 0x5f32c: str r7, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,56)?)?;
    // 0x5f330: str r7, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,60)?)?;
    // 0x5f334: add r1, r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,64)?)?;
    // 0x5f338: b 0x5f340 <entry+0x9b40>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f340,false)?)?;
    // 0x5f33c: add r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,32)?)?;
    // 0x5f340: ldr r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,60)?)?;
    // 0x5f344: ldr r3, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,88)?)?;
    // 0x5f348: mov lr, r8
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(8)?))?;
    // 0x5f34c: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x5f350: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5f354: beq 0x5f388 <entry+0x9b88>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f388,false)?)?;
    // 0x5f358: mov r5, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x5f35c: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x5f360: add r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x5f364: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // 0x5f368: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,4)?)?;
    // 0x5f36c: str r8, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,8)?)?;
    // 0x5f370: str r7, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,12)?)?;
    // 0x5f374: str r7, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,20)?)?;
    // 0x5f378: str r7, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,24)?)?;
    // 0x5f37c: str r7, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,28)?)?;
    // 0x5f380: add r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,32)?)?;
    // 0x5f384: b 0x5f38c <entry+0x9b8c>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f38c,false)?)?;
    // 0x5f388: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x5f38c: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x5f390: ldr r5, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,84)?)?;
    // 0x5f394: add r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
    // 0x5f398: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x5f39c: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,64)?)?;
    // 0x5f3a0: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x5f3a4: stmib r1, {r2, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5f3a8: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x5f3ac: str r7, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,12)?)?;
    // 0x5f3b0: str r7, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,20)?)?;
    // 0x5f3b4: str r7, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,24)?)?;
    // 0x5f3b8: str r7, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,28)?)?;
    // 0x5f3bc: str r2, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,32)?)?;
    // 0x5f3c0: ldr r2, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,68)?)?;
    // 0x5f3c4: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x5f3c8: str r2, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,36)?)?;
    // 0x5f3cc: str r8, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,40)?)?;
    // 0x5f3d0: str r7, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,44)?)?;
    // 0x5f3d4: str r7, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,52)?)?;
    // 0x5f3d8: str r7, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,56)?)?;
    // 0x5f3dc: str r7, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,60)?)?;
    // 0x5f3e0: mov r8, lr
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::LR))?;
    // 0x5f3e4: ldr lr, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,72)?)?;
    // 0x5f3e8: b 0x5fcf8 <entry+0xa4f8>  @ imm = #0x908
    c.emit(arm32::branch(C::Al,c.at as u32,0x5fcf8,false)?)?;
    // 0x5f3ec: add r6, r7, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,36)?)?;
    // 0x5f3f0: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x5f3f4: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x5f3f8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x5f3fc: ldrb r0, [r4, #0x413]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1043)?)?;
    // 0x5f400: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5f404: bne 0x5f438 <entry+0x9c38>  @ imm = #0x2c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f438,false)?)?;
    // 0x5f408: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // 0x5f40c: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5f410: ldr r0, [r4, #0x63c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1596)?)?;
    // 0x5f414: stmib r6, {r0, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R0.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5f418: ldr r0, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1572)?)?;
    // 0x5f41c: lsl r1, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsl,1)?)?;
    // 0x5f420: str r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,12)?)?;
    // 0x5f424: ldr r2, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1528)?)?;
    // 0x5f428: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x5f42c: lslhi r2, r0, #1
    c.emit(arm32::mov_shifted_register(C::Hi,R::R2,R::R0,Shift::Lsl,1)?)?;
    // 0x5f430: str r2, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,20)?)?;
    // 0x5f434: b 0x5f4e8 <entry+0x9ce8>  @ imm = #0xac
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f4e8,false)?)?;
    // 0x5f438: ldrb r0, [r4, #0x57b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1403)?)?;
    // 0x5f43c: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5f440: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5f444: bne 0x5f4cc <entry+0x9ccc>  @ imm = #0x80
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f4cc,false)?)?;
    // 0x5f448: ldr r0, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1512)?)?;
    // 0x5f44c: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // 0x5f450: ldr r6, [pc, #0x948]        @ 0xa5a0 <entry+0xa5a0>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,2376)?)?;
    // 0x5f454: mov r1, #242
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,242)?)?;
    // 0x5f458: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x5f45c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5f460: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5f464: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5f468: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5dcc0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5f46c: ldr r0, [r4, #0x5ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1516)?)?;
    // 0x5f470: mov r1, #243
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,243)?)?;
    // 0x5f474: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5f478: mov r6, r7
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(7)?))?;
    // 0x5f47c: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x5f480: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5f484: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5f488: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5dce0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5f48c: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x5f490: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x5f494: mov r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4096)?)?;
    // 0x5f498: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x5f49c: ldr r0, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1512)?)?;
    // 0x5f4a0: stmib r6, {r0, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R0.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5f4a4: ldr r0, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1528)?)?;
    // 0x5f4a8: str r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // 0x5f4ac: str r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // 0x5f4b0: str r5, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,24)?)?;
    // 0x5f4b4: str r1, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,28)?)?;
    // 0x5f4b8: add r6, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,32)?)?;
    // 0x5f4bc: ldr r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // 0x5f4c0: add r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,32)?)?;
    // 0x5f4c4: str r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,52)?)?;
    // 0x5f4c8: b 0x5f4d0 <entry+0x9cd0>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5f4d0,false)?)?;
    // 0x5f4cc: ldr r0, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1528)?)?;
    // 0x5f4d0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x5f4d4: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // 0x5f4d8: ldr r1, [r4, #0x63c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1596)?)?;
    // 0x5f4dc: stmib r6, {r1, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R1.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5f4e0: str r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // 0x5f4e4: str r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // 0x5f4e8: str r5, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,24)?)?;
    // 0x5f4ec: mov r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,256)?)?;
    // 0x5f4f0: sub r1, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,40)?)?;
    // 0x5f4f4: ldr r0, [r4, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1224)?)?;
    // 0x5f4f8: str r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,28)?)?;
    // 0x5f4fc: sub r0, r11, #52
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,52)?)?;
    // 0x5f500: str r5, [r11, #-0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-84)?)?;
    // 0x5f504: str r3, [r11, #-0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-88)?)?;
    // 0x5f508: str r5, [r11, #-0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-80)?)?;
    // 0x5f50c: str r5, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-72)?)?;
    // 0x5f510: str r3, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-76)?)?;
    // 0x5f514: str r5, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-68)?)?;
    // 0x5f518: str r5, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-60)?)?;
    // 0x5f51c: str r3, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-64)?)?;
    // 0x5f520: str r5, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-56)?)?;
    // 0x5f524: stm r0, {r3, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5f528: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f52c: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f530: ldr r0, [r4, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1764)?)?;
    // 0x5f534: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f538: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f53c: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x29fbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f540: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x5f544: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5f548: beq 0x5f564 <entry+0x9d64>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f564,false)?)?;
    // 0x5f54c: ldr r0, [r4, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,940)?)?;
    // 0x5f550: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5f554: bne 0x5f564 <entry+0x9d64>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f564,false)?)?;
    // 0x5f558: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x5f55c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5f560: blo 0x5f588 <entry+0x9d88>  @ imm = #0x20
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5f588,false)?)?;
    // 0x5f564: ldr r0, [r4, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1224)?)?;
    // 0x5f568: sub r1, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,40)?)?;
    // 0x5f56c: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x5f570: ldr r0, [r4, #0x5fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1532)?)?;
    // 0x5f574: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f578: ldr r0, [r4, #0x640]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1600)?)?;
    // 0x5f57c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f580: sub r0, r11, #88
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,88)?)?;
    // 0x5f584: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a004
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f588: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x5f58c: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x5f590: cmpne r0, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x5f594: bne 0x5f5f0 <entry+0x9df0>  @ imm = #0x54
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f5f0,false)?)?;
    // 0x5f598: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x5f59c: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x5f5a0: sub r8, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,40)?)?;
    // 0x5f5a4: movne r0, #3840
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3840)?)?;
    // 0x5f5a8: str r0, [r4, #0x64c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1612)?)?;
    // 0x5f5ac: str r1, [r4, #0x604]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1540)?)?;
    // 0x5f5b0: str r1, [r4, #0x610]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1552)?)?;
    // 0x5f5b4: mov r1, #12544
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12544)?)?;
    // 0x5f5b8: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f5bc: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x5f5c0: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f5c4: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f5c8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f5cc: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f5d0: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a050
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f5d4: str r7, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-40)?)?;
    // 0x5f5d8: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f5dc: ldr r0, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1528)?)?;
    // 0x5f5e0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5f5e4: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f5e8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f5ec: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a06c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f5f0: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x5f5f4: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f5f8: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f5fc: sub r8, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,40)?)?;
    // 0x5f600: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f604: add r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x5f608: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x5f60c: add r0, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,32)?)?;
    // 0x5f610: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x5f614: ldr r0, [r4, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1696)?)?;
    // 0x5f618: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f61c: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f620: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a0a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f624: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f628: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f62c: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f630: ldr r0, [r4, #0x6a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1700)?)?;
    // 0x5f634: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f638: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f63c: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a0bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f640: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f644: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f648: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f64c: ldr r0, [r4, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1708)?)?;
    // 0x5f650: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f654: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f658: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a0d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f65c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5f660: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f664: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f668: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f66c: ldr r0, [r4, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1708)?)?;
    // 0x5f670: lsr r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,1)?)?;
    // 0x5f674: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f678: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f67c: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a0fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f680: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f684: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f688: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f68c: ldr r0, [r4, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1752)?)?;
    // 0x5f690: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f694: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f698: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a118
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f69c: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f6a0: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f6a4: ldr r0, [r4, #0x764]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1892)?)?;
    // 0x5f6a8: ldr r1, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,664)?)?;
    // 0x5f6ac: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x5f6b0: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f6b4: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f6b8: str r2, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-40)?)?;
    // 0x5f6bc: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a13c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f6c0: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f6c4: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f6c8: ldr r0, [r4, #0x768]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1896)?)?;
    // 0x5f6cc: ldr r1, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,664)?)?;
    // 0x5f6d0: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x5f6d4: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f6d8: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f6dc: str r2, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-40)?)?;
    // 0x5f6e0: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a160
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f6e4: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f6e8: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f6ec: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f6f0: ldr r0, [r4, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1756)?)?;
    // 0x5f6f4: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f6f8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f6fc: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a17c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f700: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f704: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f708: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x5f70c: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x5f710: ldr r0, [r4, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1760)?)?;
    // 0x5f714: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f718: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f71c: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a19c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f720: ldr r8, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,40)?)?;
    // 0x5f724: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f728: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5f72c: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f730: tst r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,1)?)?;
    // 0x5f734: sub r1, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,40)?)?;
    // 0x5f738: ldrne r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,664)?)?;
    // 0x5f73c: lslne r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::R0,R::R0,Shift::Lsl,1)?)?;
    // 0x5f740: strne r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::R11,-36)?)?;
    // 0x5f744: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x5f748: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x5f74c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5f750: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x5f754: subne r0, r0, #56
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::R0,R::R0,56)?)?;
    // 0x5f758: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x5f75c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f760: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f764: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a1e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f768: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x5f76c: add r8, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    // 0x5f770: cmp r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4)?)?;
    // 0x5f774: bne 0x5f72c <entry+0x9f2c>  @ imm = #-0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f72c,false)?)?;
    // 0x5f778: mov r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,256)?)?;
    // 0x5f77c: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5f780: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x5f784: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x5f788: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f78c: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f790: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x5f794: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x5f798: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5f79c: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x5f7a0: addne r0, r1, #1024
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R1,1024)?)?;
    // 0x5f7a4: sub r1, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,40)?)?;
    // 0x5f7a8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x5f7ac: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f7b0: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f7b4: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a234
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f7b8: ldrb r0, [r4, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // 0x5f7bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5f7c0: beq 0x5f7f0 <entry+0x9ff0>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f7f0,false)?)?;
    // 0x5f7c4: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x5f7c8: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x5f7cc: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5f7d0: bne 0x5f7f0 <entry+0x9ff0>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f7f0,false)?)?;
    // 0x5f7d4: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f7d8: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f7dc: sub r1, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,40)?)?;
    // 0x5f7e0: ldr r0, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1776)?)?;
    // 0x5f7e4: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f7e8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f7ec: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a26c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f7f0: mov r0, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20480)?)?;
    // 0x5f7f4: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f7f8: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f7fc: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f800: sub r5, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,40)?)?;
    // 0x5f804: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f808: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5f80c: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a28c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f810: mov r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16)?)?;
    // 0x5f814: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5f818: mov r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2048)?)?;
    // 0x5f81c: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f820: str r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-40)?)?;
    // 0x5f824: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f828: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f82c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5f830: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a2b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f834: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x5f838: mov r7, r5
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(5)?))?;
    // 0x5f83c: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x5f840: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5f844: mov r6, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,256)?)?;
    // 0x5f848: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f84c: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f850: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f854: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f858: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a2d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f85c: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f860: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f864: ldr r0, [r4, #0x608]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1544)?)?;
    // 0x5f868: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f86c: ldr r0, [r4, #0x69c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1692)?)?;
    // 0x5f870: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f874: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f878: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a2f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f87c: mov r8, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,10)?)?;
    // 0x5f880: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f884: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f888: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // 0x5f88c: ldrsb r0, [r4, r8]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R0,R::new(4)?,R::new(8)?)?)?;
    // 0x5f890: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f894: ldr r0, [r4, #0x684]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1668)?)?;
    // 0x5f898: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f89c: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f8a0: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a320
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f8a4: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f8a8: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f8ac: ldrsb r0, [r4, r8]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R0,R::new(4)?,R::new(8)?)?)?;
    // 0x5f8b0: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f8b4: ldr r0, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1672)?)?;
    // 0x5f8b8: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f8bc: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f8c0: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a340
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f8c4: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f8c8: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f8cc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f8d0: ldr r0, [r4, #0x68c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1676)?)?;
    // 0x5f8d4: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f8d8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f8dc: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a35c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f8e0: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f8e4: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f8e8: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f8ec: ldr r0, [r4, #0x690]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1680)?)?;
    // 0x5f8f0: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f8f4: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f8f8: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a378
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f8fc: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f900: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5f904: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f908: ldr r0, [r4, #0x694]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1684)?)?;
    // 0x5f90c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f910: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f914: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a394
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f918: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x5f91c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x5f920: bne 0x5f93c <entry+0xa13c>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f93c,false)?)?;
    // 0x5f924: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f928: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f92c: ldr r0, [r4, #0xb80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2944)?)?;
    // 0x5f930: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f934: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f938: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a3b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f93c: mov r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,32)?)?;
    // 0x5f940: str r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // 0x5f944: ldrb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,991)?)?;
    // 0x5f948: ldrb r1, [r4, #0x3e0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,992)?)?;
    // 0x5f94c: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5f950: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f954: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // 0x5f958: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f95c: ldr r0, [r4, #0x654]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1620)?)?;
    // 0x5f960: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f964: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f968: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a3e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f96c: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f970: ldrb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,991)?)?;
    // 0x5f974: ldrb r1, [r4, #0x3e0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,992)?)?;
    // 0x5f978: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5f97c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f980: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5f984: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f988: ldr r0, [r4, #0x658]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1624)?)?;
    // 0x5f98c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f990: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f994: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a414
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f998: str r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // 0x5f99c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f9a0: ldrb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,991)?)?;
    // 0x5f9a4: add r0, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,6)?)?;
    // 0x5f9a8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f9ac: ldr r0, [r4, #0x650]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1616)?)?;
    // 0x5f9b0: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f9b4: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5f9b8: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a438
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f9bc: ldrb r0, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3104)?)?;
    // 0x5f9c0: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5f9c4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5f9c8: beq 0x5fa58 <entry+0xa258>  @ imm = #0x88
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5fa58,false)?)?;
    // 0x5f9cc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5f9d0: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5f9d4: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5f9d8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5f9dc: mov r0, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,716)?)?;
    // 0x5f9e0: orr r0, r0, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,15360)?)?;
    // 0x5f9e4: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5f9e8: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x5f9ec: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a46c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5f9f0: ldrb r0, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3104)?)?;
    // 0x5f9f4: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x5f9f8: beq 0x5fa20 <entry+0xa220>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5fa20,false)?)?;
    // 0x5f9fc: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x5fa00: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fa04: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x5fa08: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x5fa0c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5fa10: ldr r0, [r4, #0xc24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3108)?)?;
    // 0x5fa14: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fa18: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // 0x5fa1c: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a49c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fa20: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5fa24: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5fa28: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fa2c: ldr r0, [r4, #0xc28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3112)?)?;
    // 0x5fa30: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fa34: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fa38: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a4b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fa3c: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5fa40: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5fa44: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fa48: ldr r0, [r4, #0xc2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3116)?)?;
    // 0x5fa4c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fa50: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fa54: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a4d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fa58: ldr r0, [r4, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1256)?)?;
    // 0x5fa5c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5fa60: ldrbeq r0, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(4)?,1156)?)?;
    // 0x5fa64: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5fa68: beq 0x5f054 <entry+0x9854>  @ imm = #-0xa1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5f054,false)?)?;
    // 0x5fa6c: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5fa70: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fa74: ldrb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,991)?)?;
    // 0x5fa78: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5fa7c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5fa80: ldr r0, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1656)?)?;
    // 0x5fa84: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fa88: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fa8c: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a50c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fa90: mov r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,8)?)?;
    // 0x5fa94: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5fa98: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fa9c: str r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // 0x5faa0: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5faa4: ldr r0, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2668)?)?;
    // 0x5faa8: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // 0x5faac: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fab0: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fab4: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a534
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fab8: str r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // 0x5fabc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fac0: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5fac4: ldrb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,991)?)?;
    // 0x5fac8: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5facc: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5fad0: ldr r0, [r4, #0x4ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1260)?)?;
    // 0x5fad4: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // 0x5fad8: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fadc: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fae0: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a560
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fae4: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5fae8: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5faec: ldr r0, [r4, #0xd04]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3332)?)?;
    // 0x5faf0: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5faf4: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5faf8: str r6, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-40)?)?;
    // 0x5fafc: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a57c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fb00: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x5fb04: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x5fb08: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5fb0c: beq 0x5fb78 <entry+0xa378>  @ imm = #0x64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5fb78,false)?)?;
    // 0x5fb10: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x5fb14: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x5fb18: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fb1c: ldr r0, [r4, #0x704]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1796)?)?;
    // 0x5fb20: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fb24: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fb28: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a5a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fb2c: ldr r0, [r4, #0x708]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1800)?)?;
    // 0x5fb30: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fb34: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fb38: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fb3c: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a5bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fb40: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x5fb44: ldrb r0, [r0, #0x154]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,340)?)?;
    // 0x5fb48: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5fb4c: blo 0x5fb78 <entry+0xa378>  @ imm = #0x24
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5fb78,false)?)?;
    // 0x5fb50: ldr r0, [r4, #0x70c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1804)?)?;
    // 0x5fb54: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fb58: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fb5c: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fb60: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a5e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fb64: ldr r0, [r4, #0x710]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1808)?)?;
    // 0x5fb68: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5fb6c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5fb70: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5fb74: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2a5f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x5fb78: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x5fb7c: ldr r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // 0x5fb80: bl 0x62268 <entry+0xca68>  @ imm = #0x26e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x62268,true)?)?;
    // 0x5fb84: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x5fb88: ldr r2, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // 0x5fb8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5fb90: beq 0x5fbd4 <entry+0xa3d4>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5fbd4,false)?)?;
    // 0x5fb94: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // 0x5fb98: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x5fb9c: str r1, [r2, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,32)?)?;
    // 0x5fba0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5fba4: str r0, [r2, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,36)?)?;
    // 0x5fba8: str r1, [r2, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,40)?)?;
    // 0x5fbac: ldr r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // 0x5fbb0: str r1, [r2, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,44)?)?;
    // 0x5fbb4: str r1, [r2, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,52)?)?;
    // 0x5fbb8: str r5, [r2, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,56)?)?;
    // 0x5fbbc: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // 0x5fbc0: str r1, [r2, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,60)?)?;
    // 0x5fbc4: add r2, r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,64)?)?;
    // 0x5fbc8: ldr r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // 0x5fbcc: add r1, r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,64)?)?;
    // 0x5fbd0: str r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,76)?)?;
    // 0x5fbd4: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,56)?)?;
    // 0x5fbd8: mov r12, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,8192)?)?;
    // 0x5fbdc: orr r12, r12, #2097152
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,2097152)?)?;
    // 0x5fbe0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5fbe4: ldr r0, [r11, #-0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-88)?)?;
    // 0x5fbe8: ldr r3, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,88)?)?;
    // 0x5fbec: ldr r6, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,80)?)?;
    // 0x5fbf0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5fbf4: beq 0x5fc40 <entry+0xa440>  @ imm = #0x44
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5fc40,false)?)?;
    // 0x5fbf8: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x5fbfc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5fc00: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x5fc04: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x5fc08: str r0, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,4)?)?;
    // 0x5fc0c: str r7, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,8)?)?;
    // 0x5fc10: ldr r1, [r11, #-0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-84)?)?;
    // 0x5fc14: str r1, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,12)?)?;
    // 0x5fc18: str r1, [r2, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,20)?)?;
    // 0x5fc1c: str lr, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R2,24)?)?;
    // 0x5fc20: ldr r1, [r11, #-0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-80)?)?;
    // 0x5fc24: str r1, [r2, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,28)?)?;
    // 0x5fc28: add r2, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,32)?)?;
    // 0x5fc2c: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,76)?)?;
    // 0x5fc30: ldr r5, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,84)?)?;
    // 0x5fc34: add r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,32)?)?;
    // 0x5fc38: str r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,76)?)?;
    // 0x5fc3c: b 0x5fc4c <entry+0xa44c>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5fc4c,false)?)?;
    // 0x5fc40: ldr r5, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,84)?)?;
    // 0x5fc44: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5fc48: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x5fc4c: ldr r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,60)?)?;
    // 0x5fc50: mov r8, r2
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R2))?;
    // 0x5fc54: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5fc58: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x5fc5c: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x5fc60: ldr r0, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-76)?)?;
    // 0x5fc64: stmib r2, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5fc68: ldr r1, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-72)?)?;
    // 0x5fc6c: str r1, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,12)?)?;
    // 0x5fc70: str r1, [r2, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,20)?)?;
    // 0x5fc74: str lr, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R2,24)?)?;
    // 0x5fc78: ldr r1, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-68)?)?;
    // 0x5fc7c: str r1, [r2, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,28)?)?;
    // 0x5fc80: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // 0x5fc84: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5fc88: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5fc8c: str r0, [r2, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,32)?)?;
    // 0x5fc90: ldr r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-64)?)?;
    // 0x5fc94: str r0, [r2, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,36)?)?;
    // 0x5fc98: str r7, [r2, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,40)?)?;
    // 0x5fc9c: ldr r1, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-60)?)?;
    // 0x5fca0: str r1, [r2, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,44)?)?;
    // 0x5fca4: str r1, [r2, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,52)?)?;
    // 0x5fca8: str lr, [r2, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R2,56)?)?;
    // 0x5fcac: ldr r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-56)?)?;
    // 0x5fcb0: str r1, [r2, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,60)?)?;
    // 0x5fcb4: ldr r2, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,68)?)?;
    // 0x5fcb8: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x5fcbc: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x5fcc0: str r0, [r8, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,64)?)?;
    // 0x5fcc4: ldr r0, [r4, #0xae8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2792)?)?;
    // 0x5fcc8: str r0, [r8, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,68)?)?;
    // 0x5fccc: str r7, [r8, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,72)?)?;
    // 0x5fcd0: ldr r0, [r4, #0x608]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1544)?)?;
    // 0x5fcd4: str r0, [r8, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,76)?)?;
    // 0x5fcd8: str r0, [r8, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,80)?)?;
    // 0x5fcdc: str r0, [r8, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,84)?)?;
    // 0x5fce0: str lr, [r8, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,88)?)?;
    // 0x5fce4: str r1, [r8, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,92)?)?;
    // 0x5fce8: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,76)?)?;
    // 0x5fcec: ldr lr, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,72)?)?;
    // 0x5fcf0: ldr r8, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,36)?)?;
    // 0x5fcf4: add r0, r0, #96
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,96)?)?;
    // 0x5fcf8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x5fcfc: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x5fd00: add r7, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x5fd04: subs r10, r10, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::new(10)?,R::new(10)?,1,true)?)?;
    // 0x5fd08: add r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5fd0c: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // 0x5fd10: bne 0x5f064 <entry+0x9864>  @ imm = #-0xcb4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5f064,false)?)?;
    // 0x5fd14: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5fd18: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x5fd1c: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x354ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x5fd20: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x5fd24: b 0x5fd74 <entry+0xa574>  @ imm = #0x48
    c.emit(arm32::branch(C::Al,c.at as u32,0x5fd74,false)?)?;
    // 0x5fd28: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5fd2c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5fd30: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x5fd34: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x5fd38: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5fd3c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5fd40: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x5fd44: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x5fd48: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x5fd4c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x353c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5fd50: ldr r1, [pc, #0x4c]         @ 0xa5a4 <entry+0xa5a4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,76)?)?;
    // 0x5fd54: ldr r2, [pc, #0x28]         @ 0xa584 <entry+0xa584>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,40)?)?;
    // 0x5fd58: mov r3, #238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,238)?)?;
    // 0x5fd5c: orr r3, r3, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8448)?)?;
    // 0x5fd60: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5fd64: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x310d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5fd68: ldr r1, [pc, #0x18]         @ 0xa588 <entry+0xa588>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,24)?)?;
    // 0x5fd6c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5fd70: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x310e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5fd74: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5fd78: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x5fd7c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x5fd80)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5FD80:[u32;10]=[0x000f0967,0x000f0996,0x000e2788,0x00221009,0x00202001,0x00202003,0x00202002,0x000f09ae,0x000ee470,0x000f09fe];
 for value in INLINE_DATA_5FD80{c.emit(value)?;}
 // Executable A32 block 0x5fda8..0x60000.
 c.check(0x5fda8)?;
    // 0x5fda8: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5fdac: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x5fdb0: sub sp, sp, #96
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,96)?)?;
    // 0x5fdb4: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5fdb8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5fdbc: bl 0x26754 <entry+0xfffffffffffd0f54> @ imm = #-0x39670
    c.emit(arm32::branch(C::Al,c.at as u32,0x26754,true)?)?;
    // 0x5fdc0: mov r6, r5
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(5)?))?;
    // 0x5fdc4: ldr r1, [pc, #0x3ac]        @ 0xa978 <entry+0xa978>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,940)?)?;
    // 0x5fdc8: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x5fdcc: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x5fdd0: ldr r3, [r6, #0x34]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(3)?,R::new(6)?,52)?)?;
    // 0x5fdd4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31148
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5fdd8: ldr r1, [pc, #0x39c]        @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,924)?)?;
    // 0x5fddc: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x5fde0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31154
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5fde4: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5fde8: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x5fdec: bne 0x5ff5c <entry+0xa75c>  @ imm = #0x168
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5ff5c,false)?)?;
    // 0x5fdf0: ldr r1, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,24)?)?;
    // 0x5fdf4: tst r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // 0x5fdf8: bne 0x5fe18 <entry+0xa618>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5fe18,false)?)?;
    // 0x5fdfc: ldr r0, [r4, #0x628]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1576)?)?;
    // 0x5fe00: ldr r2, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,36)?)?;
    // 0x5fe04: ldr r3, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,44)?)?;
    // 0x5fe08: subs r7, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::R2,R::R0,Shift::Lsl,0,true)?)?;
    // 0x5fe0c: blo 0x60010 <entry+0xa810>  @ imm = #0x1fc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x60010,false)?)?;
    // 0x5fe10: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x5fe14: bhi 0x60010 <entry+0xa810>  @ imm = #0x1f4
    c.emit(arm32::branch(C::Hi,c.at as u32,0x60010,false)?)?;
    // 0x5fe18: tst r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,64)?)?;
    // 0x5fe1c: bne 0x5ffb8 <entry+0xa7b8>  @ imm = #0x194
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5ffb8,false)?)?;
    // 0x5fe20: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5fe24: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x4698
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x5fe28: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5fe2c: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x5fe30: ldrne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,0)?)?;
    // 0x5fe34: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5fe38: bne 0x600dc <entry+0xa8dc>  @ imm = #0x29c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x600dc,false)?)?;
    // 0x5fe3c: ldr r0, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,24)?)?;
    // 0x5fe40: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x5fe44: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x5fe48: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x5fe4c: and r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,1)?)?;
    // 0x5fe50: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5fe54: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5fe58: bl 0x5ed74 <entry+0x9574>  @ imm = #-0x10ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x5ed74,true)?)?;
    // 0x5fe5c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5fe60: bne 0x6006c <entry+0xa86c>  @ imm = #0x204
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6006c,false)?)?;
    // 0x5fe64: ldr r6, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,16)?)?;
    // 0x5fe68: ldr r7, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,20)?)?;
    // 0x5fe6c: ldr r1, [pc, #0x324]        @ 0xa998 <entry+0xa998>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,804)?)?;
    // 0x5fe70: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5fe74: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5fe78: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x5fe7c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x311f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5fe80: ldr r1, [pc, #0x2f4]        @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,756)?)?;
    // 0x5fe84: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5fe88: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x311fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5fe8c: mov r1, #2032
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2032)?)?;
    // 0x5fe90: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5fe94: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5fe98: ldr r3, [r0, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x5fe9c: ldr r1, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,24)?)?;
    // 0x5fea0: ldr r12, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,4)?)?;
    // 0x5fea4: subs r2, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(7)?,R::new(3)?,Shift::Lsl,0,true)?)?;
    // 0x5fea8: and r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,1)?)?;
    // 0x5feac: sbcs r2, r6, r12
    c.emit(arm32::data_register(C::Al,Op::Sbc,R::R2,R::new(6)?,R::R12,Shift::Lsl,0,true)?)?;
    // 0x5feb0: bhs 0x5fedc <entry+0xa6dc>  @ imm = #0x24
    c.emit(arm32::branch(C::Cs,c.at as u32,0x5fedc,false)?)?;
    // 0x5feb4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5feb8: bne 0x5fedc <entry+0xa6dc>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5fedc,false)?)?;
    // 0x5febc: mov r2, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1016)?)?;
    // 0x5fec0: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x5fec4: ldr r2, [r4, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x5fec8: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x5fecc: beq 0x5fedc <entry+0xa6dc>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5fedc,false)?)?;
    // 0x5fed0: ldrb r2, [r4, #0x34a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,842)?)?;
    // 0x5fed4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5fed8: beq 0x60164 <entry+0xa964>  @ imm = #0x284
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60164,false)?)?;
    // 0x5fedc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5fee0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5fee4: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x5fee8: moveq r9, r6
    c.emit(arm32::mov_register(C::Eq,R::new(9)?,R::new(6)?))?;
    // 0x5feec: moveq r2, r7
    c.emit(arm32::mov_register(C::Eq,R::R2,R::new(7)?))?;
    // 0x5fef0: beq 0x5ff0c <entry+0xa70c>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5ff0c,false)?)?;
    // 0x5fef4: subs r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,true)?)?;
    // 0x5fef8: sbcs r3, r6, r12
    c.emit(arm32::data_register(C::Al,Op::Sbc,R::new(3)?,R::new(6)?,R::R12,Shift::Lsl,0,true)?)?;
    // 0x5fefc: movlo r2, #0
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5ff00: movlo r1, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5ff04: movlo r9, #0
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x5ff08: strlo r2, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Cc,false,R::R2,R::new(5)?,40)?)?;
    // 0x5ff0c: mov r3, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1016)?)?;
    // 0x5ff10: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x5ff14: str r1, [r4, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x5ff18: stm r0, {r2, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(9)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5ff1c: ldr r1, [pc, #0x280]        @ 0xa9a4 <entry+0xa9a4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,640)?)?;
    // 0x5ff20: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5ff24: ldr r2, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,100)?)?;
    // 0x5ff28: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3129c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ff2c: ldr r1, [pc, #0x248]        @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,584)?)?;
    // 0x5ff30: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5ff34: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x312a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ff38: add r1, r4, #92
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,92)?)?;
    // 0x5ff3c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5ff40: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x5ff44: bl 0x39490 <entry+0xfffffffffffe3c90> @ imm = #-0x26abc
    c.emit(arm32::branch(C::Al,c.at as u32,0x39490,true)?)?;
    // 0x5ff48: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x5ff4c: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x5ff50: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5d62c
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x5ff54: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x5ff58: b 0x60158 <entry+0xa958>  @ imm = #0x1f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x60158,false)?)?;
    // 0x5ff5c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5ff60: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5ff64: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x5ff68: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5ff6c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5ff70: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5ff74: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x5ff78: str r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x5ff7c: str r6, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // 0x5ff80: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x355f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5ff84: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5ff88: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // 0x5ff8c: ldr r2, [pc, #0x1f0]        @ 0xa984 <entry+0xa984>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,496)?)?;
    // 0x5ff90: mov r3, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,18)?)?;
    // 0x5ff94: orr r3, r3, #8704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8704)?)?;
    // 0x5ff98: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5ff9c: ldr r1, [pc, #0x1dc]        @ 0xa980 <entry+0xa980>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,476)?)?;
    // 0x5ffa0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ffa4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31318
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ffa8: ldr r1, [pc, #0x1cc]        @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,460)?)?;
    // 0x5ffac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ffb0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31324
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ffb4: b 0x60158 <entry+0xa958>  @ imm = #0x19c
    c.emit(arm32::branch(C::Al,c.at as u32,0x60158,false)?)?;
    // 0x5ffb8: ldr r0, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,56)?)?;
    // 0x5ffbc: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x5ffc0: orr r1, r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3840)?)?;
    // 0x5ffc4: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ffc8: beq 0x60098 <entry+0xa898>  @ imm = #0xc8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60098,false)?)?;
    // 0x5ffcc: ldr r1, [pc, #0x1b4]        @ 0xa988 <entry+0xa988>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,436)?)?;
    // 0x5ffd0: ldr r2, [pc, #0x1ac]        @ 0xa984 <entry+0xa984>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,428)?)?;
    // 0x5ffd4: mov r3, #548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,548)?)?;
    // 0x5ffd8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5ffdc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5ffe0: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x5ffe4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31358
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ffe8: ldr r1, [pc, #0x18c]        @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,396)?)?;
    // 0x5ffec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5fff0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31364
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5fff4: ldr r2, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,56)?)?;
    // 0x5fff8: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5fffc: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
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
