// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x5b000,0x5c000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x5b000;
pub const END:usize=0x5c000;
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
 // Executable A32 block 0x5b000..0x5b018.
 c.check(0x5b000)?;
    // 0x5b000: str r1, [r4, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,380)?)?;
    // 0x5b004: str r6, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,384)?)?;
    // 0x5b008: str r2, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,388)?)?;
    // 0x5b00c: add r0, r4, #392
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,392)?)?;
    // 0x5b010: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x5b014: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x5b018)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5B018:[u32;1]=[0x000ee470];
 for value in INLINE_DATA_5B018{c.emit(value)?;}
 // Executable A32 block 0x5b01c..0x5b468.
 c.check(0x5b01c)?;
    // 0x5b01c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5b020: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x5b024: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5b028: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x5b02c: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5b030: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x5b034: ldr lr, [r0, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,3372)?)?;
    // 0x5b038: ldr r1, [r0, #0xd74]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,3444)?)?;
    // 0x5b03c: mov r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15)?)?;
    // 0x5b040: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x5b044: cmp lr, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,8192)?)?;
    // 0x5b048: moveq r3, #31
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,31)?)?;
    // 0x5b04c: moveq r2, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x5b050: add r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5b054: lsr r1, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::R2,false)?)?;
    // 0x5b058: lsl r1, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsl,R::R2,false)?)?;
    // 0x5b05c: lsr r2, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R1,Shift::Lsr,R::R2,false)?)?;
    // 0x5b060: cmp r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,12)?)?;
    // 0x5b064: bhi 0x5b080 <entry+0x5880>  @ imm = #0x14
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5b080,false)?)?;
    // 0x5b068: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5b06c: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x5b070: strb r1, [r0, #0xf64]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,3940)?)?;
    // 0x5b074: strb r3, [r0, #0xf70]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,3952)?)?;
    // 0x5b078: str r1, [r0, #0xd94]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,3476)?)?;
    // 0x5b07c: str r1, [r0, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1244)?)?;
    // 0x5b080: mov r1, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,78)?)?;
    // 0x5b084: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x5b088: ldrb r12, [r0, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R12,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x5b08c: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x5b090: bne 0x5b0bc <entry+0x58bc>  @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b0bc,false)?)?;
    // 0x5b094: ldrb r1, [r0, #0xe86]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,3718)?)?;
    // 0x5b098: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x5b09c: bne 0x5b0bc <entry+0x58bc>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b0bc,false)?)?;
    // 0x5b0a0: ldrb r3, [r0, #0xf70]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,3952)?)?;
    // 0x5b0a4: cmp r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // 0x5b0a8: bls 0x5b0d8 <entry+0x58d8>  @ imm = #0x28
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5b0d8,false)?)?;
    // 0x5b0ac: cmp r3, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,9)?)?;
    // 0x5b0b0: blo 0x5b0e0 <entry+0x58e0>  @ imm = #0x28
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5b0e0,false)?)?;
    // 0x5b0b4: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x5b0b8: b 0x5b0dc <entry+0x58dc>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b0dc,false)?)?;
    // 0x5b0bc: ldrb r3, [r0, #0xf70]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,3952)?)?;
    // 0x5b0c0: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x5b0c4: bhi 0x5b0d0 <entry+0x58d0>  @ imm = #0x4
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5b0d0,false)?)?;
    // 0x5b0c8: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x5b0cc: b 0x5b0dc <entry+0x58dc>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b0dc,false)?)?;
    // 0x5b0d0: cmp r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,5)?)?;
    // 0x5b0d4: blo 0x5b0e0 <entry+0x58e0>  @ imm = #0x4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5b0e0,false)?)?;
    // 0x5b0d8: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x5b0dc: strb r3, [r0, #0xf70]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,3952)?)?;
    // 0x5b0e0: ldrb r1, [r0, #0xe85]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,3717)?)?;
    // 0x5b0e4: mvn r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(4)?,R::R0,1)?)?;
    // 0x5b0e8: add r2, r4, r2, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::R2,Shift::Lsr,1,false)?)?;
    // 0x5b0ec: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5b0f0: moveq r3, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x5b0f4: strbeq r3, [r0, #0xf70]
    c.emit(arm32::transfer_byte(C::Eq,false,R::new(3)?,R::R0,3952)?)?;
    // 0x5b0f8: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x5b0fc: bhs 0x5b114 <entry+0x5914>  @ imm = #0x10
    c.emit(arm32::branch(C::Cs,c.at as u32,0x5b114,false)?)?;
    // 0x5b100: sub r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // 0x5b104: and r4, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(3)?,255)?)?;
    // 0x5b108: cmp r2, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x5b10c: blo 0x5b100 <entry+0x5900>  @ imm = #-0x14
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5b100,false)?)?;
    // 0x5b110: strb r3, [r0, #0xf70]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,3952)?)?;
    // 0x5b114: cmp lr, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,4096)?)?;
    // 0x5b118: cmpne lr, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::LR,2)?)?;
    // 0x5b11c: bne 0x5b140 <entry+0x5940>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b140,false)?)?;
    // 0x5b120: ldrb r2, [r0, #0xe74]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,3700)?)?;
    // 0x5b124: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5b128: beq 0x5b140 <entry+0x5940>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b140,false)?)?;
    // 0x5b12c: ldr r2, [r0, #0xd94]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,3476)?)?;
    // 0x5b130: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x5b134: moveq r2, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5b138: streq r2, [r0, #0xd94]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,3476)?)?;
    // 0x5b13c: streq r2, [r0, #0x4dc]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,1244)?)?;
    // 0x5b140: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x5b144: beq 0x5b15c <entry+0x595c>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b15c,false)?)?;
    // 0x5b148: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5b14c: andne r1, r3, #255
    c.emit(arm32::data_immediate(C::Ne,Op::And,R::R1,R::new(3)?,255)?)?;
    // 0x5b150: cmpne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5b154: bls 0x5b194 <entry+0x5994>  @ imm = #0x38
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5b194,false)?)?;
    // 0x5b158: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5b15c: ldrb r2, [r0, #0xe86]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,3718)?)?;
    // 0x5b160: and r12, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R12,R::new(3)?,255)?)?;
    // 0x5b164: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x5b168: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x5b16c: cmp r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,4)?)?;
    // 0x5b170: movlo r4, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x5b174: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x5b178: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x5b17c: tst r3, r4
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x5b180: bne 0x5b194 <entry+0x5994>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b194,false)?)?;
    // 0x5b184: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5b188: beq 0x5b194 <entry+0x5994>  @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b194,false)?)?;
    // 0x5b18c: cmp r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,2)?)?;
    // 0x5b190: pophs {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Cs,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5b194: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5b198: strb r1, [r0, #0xf64]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,3940)?)?;
    // 0x5b19c: ldr r2, [r0, #0xd94]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,3476)?)?;
    // 0x5b1a0: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x5b1a4: streq r1, [r0, #0xd94]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,3476)?)?;
    // 0x5b1a8: streq r1, [r0, #0x4dc]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,1244)?)?;
    // 0x5b1ac: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5b1b0: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5b1b4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x5b1b8: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x5b1bc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5b1c0: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x5b1c4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5b1c8: blo 0x5b1dc <entry+0x59dc>  @ imm = #0xc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5b1dc,false)?)?;
    // 0x5b1cc: ldr r1, [r4, #0xd94]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3476)?)?;
    // 0x5b1d0: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x5b1d4: bne 0x5b1e4 <entry+0x59e4>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b1e4,false)?)?;
    // 0x5b1d8: b 0x5b204 <entry+0x5a04>  @ imm = #0x24
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b204,false)?)?;
    // 0x5b1dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5b1e0: beq 0x5b204 <entry+0x5a04>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b204,false)?)?;
    // 0x5b1e4: ldr r1, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,668)?)?;
    // 0x5b1e8: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5b1ec: bhi 0x5b204 <entry+0x5a04>  @ imm = #0x10
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5b204,false)?)?;
    // 0x5b1f0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5b1f4: beq 0x5b274 <entry+0x5a74>  @ imm = #0x78
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b274,false)?)?;
    // 0x5b1f8: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x5b1fc: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x5b200: bne 0x5b274 <entry+0x5a74>  @ imm = #0x6c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b274,false)?)?;
    // 0x5b204: mov r3, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,23)?)?;
    // 0x5b208: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5b20c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b210: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5b214: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b218: str r3, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // 0x5b21c: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x5b220: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5b224: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5b228: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b22c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b230: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x308a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b234: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x5b238: ldr r1, [pc, #0x228]        @ 0x5c68 <entry+0x5c68>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,552)?)?;
    // 0x5b23c: ldr r2, [pc, #0x228]        @ 0x5c6c <entry+0x5c6c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,552)?)?;
    // 0x5b240: mov r3, #235
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,235)?)?;
    // 0x5b244: orr r3, r3, #5888
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5888)?)?;
    // 0x5b248: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5b24c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b250: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c5c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b254: ldr r1, [pc, #0x214]        @ 0x5c70 <entry+0x5c70>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,532)?)?;
    // 0x5b258: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b25c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c5d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b260: ldr r2, [pc, #0x20c]        @ 0x5c74 <entry+0x5c74>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,524)?)?;
    // 0x5b264: mov r1, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1004)?)?;
    // 0x5b268: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5b26c: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x5b270: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59ac8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5b274: ldrb r0, [r4, #0x399]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,921)?)?;
    // 0x5b278: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5b27c: bne 0x5b360 <entry+0x5b60>  @ imm = #0xdc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b360,false)?)?;
    // 0x5b280: ldrb r0, [r4, #0xe30]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3632)?)?;
    // 0x5b284: ldr r5, [pc, #0x1ec]        @ 0x5c78 <entry+0x5c78>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,492)?)?;
    // 0x5b288: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5b28c: beq 0x5b2f4 <entry+0x5af4>  @ imm = #0x60
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b2f4,false)?)?;
    // 0x5b290: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5b294: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b298: add r3, r5, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(5)?,10)?)?;
    // 0x5b29c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5b2a0: str r3, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // 0x5b2a4: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x5b2a8: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5b2ac: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5b2b0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b2b4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b2b8: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x30930
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b2bc: ldr r1, [pc, #0x1b8]        @ 0x5c7c <entry+0x5c7c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,440)?)?;
    // 0x5b2c0: ldr r2, [pc, #0x1a4]        @ 0x5c6c <entry+0x5c6c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,420)?)?;
    // 0x5b2c4: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x5b2c8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b2cc: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b2d0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c644
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b2d4: ldr r1, [pc, #0x194]        @ 0x5c70 <entry+0x5c70>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,404)?)?;
    // 0x5b2d8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b2dc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c650
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b2e0: ldr r2, [pc, #0x18c]        @ 0x5c74 <entry+0x5c74>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,396)?)?;
    // 0x5b2e4: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x5b2e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5b2ec: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x5b2f0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59b48
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5b2f4: ldr r0, [r4, #0xe34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3636)?)?;
    // 0x5b2f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5b2fc: beq 0x5b360 <entry+0x5b60>  @ imm = #0x5c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b360,false)?)?;
    // 0x5b300: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5b304: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b308: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x5b30c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5b310: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5b314: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5b318: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5b31c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b320: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b324: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x3099c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b328: ldr r1, [pc, #0x150]        @ 0x5c80 <entry+0x5c80>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,336)?)?;
    // 0x5b32c: ldr r2, [pc, #0x138]        @ 0x5c6c <entry+0x5c6c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,312)?)?;
    // 0x5b330: mov r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15)?)?;
    // 0x5b334: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b338: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b33c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c6b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b340: ldr r1, [pc, #0x128]        @ 0x5c70 <entry+0x5c70>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,296)?)?;
    // 0x5b344: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b348: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c6bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b34c: ldr r2, [pc, #0x120]        @ 0x5c74 <entry+0x5c74>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,288)?)?;
    // 0x5b350: mov r1, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2064)?)?;
    // 0x5b354: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5b358: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5b35c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59bb4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5b360: ldrb r0, [r4, #0x3f9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1017)?)?;
    // 0x5b364: ldr r5, [pc, #0x118]        @ 0x5c84 <entry+0x5c84>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,280)?)?;
    // 0x5b368: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x5b36c: beq 0x5b3f8 <entry+0x5bf8>  @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b3f8,false)?)?;
    // 0x5b370: cmp r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,17)?)?;
    // 0x5b374: bne 0x5b3e8 <entry+0x5be8>  @ imm = #0x6c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b3e8,false)?)?;
    // 0x5b378: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x5b37c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x5b380: bne 0x5b3f8 <entry+0x5bf8>  @ imm = #0x70
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b3f8,false)?)?;
    // 0x5b384: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5b388: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b38c: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x5b390: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5b394: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5b398: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5b39c: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5b3a0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b3a4: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b3a8: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x30a20
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b3ac: ldr r1, [pc, #0xd4]         @ 0x5c88 <entry+0x5c88>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,212)?)?;
    // 0x5b3b0: ldr r2, [pc, #0xb4]         @ 0x5c6c <entry+0x5c6c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,180)?)?;
    // 0x5b3b4: mov r3, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,28)?)?;
    // 0x5b3b8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b3bc: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b3c0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c734
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b3c4: ldr r1, [pc, #0xa4]         @ 0x5c70 <entry+0x5c70>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,164)?)?;
    // 0x5b3c8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b3cc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c740
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b3d0: ldr r2, [pc, #0x9c]         @ 0x5c74 <entry+0x5c74>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,156)?)?;
    // 0x5b3d4: mov r1, #29
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,29)?)?;
    // 0x5b3d8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5b3dc: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x5b3e0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59c38
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5b3e4: ldrb r0, [r4, #0x3f9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1017)?)?;
    // 0x5b3e8: cmp r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,18)?)?;
    // 0x5b3ec: ldreq r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,644)?)?;
    // 0x5b3f0: cmpeq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5b3f4: beq 0x5b400 <entry+0x5c00>  @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b400,false)?)?;
    // 0x5b3f8: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x5b3fc: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5b400: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5b404: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b408: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x5b40c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5b410: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x5b414: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5b418: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5b41c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b420: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b424: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x30a9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b428: ldr r1, [pc, #0x5c]         @ 0x5c8c <entry+0x5c8c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,92)?)?;
    // 0x5b42c: ldr r2, [pc, #0x38]         @ 0x5c6c <entry+0x5c6c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,56)?)?;
    // 0x5b430: mov r3, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,36)?)?;
    // 0x5b434: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b438: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b43c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c7b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b440: ldr r1, [pc, #0x28]         @ 0x5c70 <entry+0x5c70>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,40)?)?;
    // 0x5b444: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b448: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c7bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b44c: ldr r2, [pc, #0x20]         @ 0x5c74 <entry+0x5c74>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,32)?)?;
    // 0x5b450: mov r1, #37
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,37)?)?;
    // 0x5b454: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5b458: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x5b45c: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x5b460: pop {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5b464: b 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x59cbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,false)?)?;
 c.check(0x5b468)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5B468:[u32;10]=[0x000ee7a0,0x000ee7e0,0x000e2788,0x000ee470,0x0200501c,0x000ee7fc,0x000ee887,0x02205005,0x000ee901,0x000ee943];
 for value in INLINE_DATA_5B468{c.emit(value)?;}
 // Executable A32 block 0x5b490..0x5b5f0.
 c.check(0x5b490)?;
    // 0x5b490: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5b494: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x5b498: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x5b49c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5b4a0: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x5b4a4: cmp r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,12)?)?;
    // 0x5b4a8: bne 0x5b528 <entry+0x5d28>  @ imm = #0x78
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b528,false)?)?;
    // 0x5b4ac: ldr r1, [pc, #0x148]        @ 0x5dfc <entry+0x5dfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,328)?)?;
    // 0x5b4b0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5b4b4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c828
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b4b8: ldr r1, [pc, #0x138]        @ 0x5df8 <entry+0x5df8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,312)?)?;
    // 0x5b4bc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5b4c0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c834
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b4c4: ldr r0, [pc, #0x134]        @ 0x5e00 <entry+0x5e00>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,308)?)?;
    // 0x5b4c8: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x5b4cc: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x5b4d0: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x5b4d4: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x5b4d8: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x5b4dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5b4e0: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x5b4e4: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b4e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5b4ec: beq 0x5b578 <entry+0x5d78>  @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b578,false)?)?;
    // 0x5b4f0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b4f4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5b4f8: bne 0x5b59c <entry+0x5d9c>  @ imm = #0x9c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b59c,false)?)?;
    // 0x5b4fc: ldr r1, [pc, #0x104]        @ 0x5e08 <entry+0x5e08>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,260)?)?;
    // 0x5b500: ldr r2, [pc, #0xec]         @ 0x5df4 <entry+0x5df4>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,236)?)?;
    // 0x5b504: mov r3, #89
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,89)?)?;
    // 0x5b508: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b50c: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b510: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c884
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b514: ldr r1, [pc, #0xdc]         @ 0x5df8 <entry+0x5df8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,220)?)?;
    // 0x5b518: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b51c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c890
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b520: ldr r0, [pc, #0xe4]         @ 0x5e0c <entry+0x5e0c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,228)?)?;
    // 0x5b524: b 0x5b5d0 <entry+0x5dd0>  @ imm = #0xa4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b5d0,false)?)?;
    // 0x5b528: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5b52c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b530: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x5b534: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x5b538: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b53c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b540: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x5b544: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x5b548: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x5b54c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x30bc4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b550: ldr r1, [pc, #0x98]         @ 0x5df0 <entry+0x5df0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,152)?)?;
    // 0x5b554: ldr r2, [pc, #0x98]         @ 0x5df4 <entry+0x5df4>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,152)?)?;
    // 0x5b558: mov r3, #66
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,66)?)?;
    // 0x5b55c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b560: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b564: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c8d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b568: ldr r1, [pc, #0x88]         @ 0x5df8 <entry+0x5df8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,136)?)?;
    // 0x5b56c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b570: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c8e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b574: b 0x5b5e4 <entry+0x5de4>  @ imm = #0x68
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b5e4,false)?)?;
    // 0x5b578: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5b57c: bl 0x5b1b0 <entry+0x59b0>  @ imm = #-0x3d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b1b0,true)?)?;
    // 0x5b580: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5b584: bl 0x59638 <entry+0x3e38>  @ imm = #-0x1f54
    c.emit(arm32::branch(C::Al,c.at as u32,0x59638,true)?)?;
    // 0x5b588: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5b58c: bl 0x5b02c <entry+0x582c>  @ imm = #-0x568
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b02c,true)?)?;
    // 0x5b590: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5b594: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x5b598: b 0x5b5d4 <entry+0x5dd4>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b5d4,false)?)?;
    // 0x5b59c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b5a0: ldr r1, [pc, #0x5c]         @ 0x5e04 <entry+0x5e04>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,92)?)?;
    // 0x5b5a4: ldr r2, [pc, #0x48]         @ 0x5df4 <entry+0x5df4>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,72)?)?;
    // 0x5b5a8: mov r3, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,94)?)?;
    // 0x5b5ac: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b5b0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5b5b4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b5b8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c92c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b5bc: ldr r1, [pc, #0x34]         @ 0x5df8 <entry+0x5df8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,52)?)?;
    // 0x5b5c0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b5c4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c938
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b5c8: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x5b5cc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x5b5d0: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x5b5d4: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b5d8: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x5b5dc: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x30d6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x5b5e0: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x5b5e4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5b5e8: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x5b5ec: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x5b5f0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5B5F0:[u32;8]=[0x000ee986,0x000ee9c9,0x000e2788,0x000ee9e3,0x00221001,0x000eea17,0x000ee9f3,0x01001002];
 for value in INLINE_DATA_5B5F0{c.emit(value)?;}
 // Executable A32 block 0x5b610..0x5b778.
 c.check(0x5b610)?;
    // 0x5b610: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5b614: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x5b618: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x5b61c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5b620: ldr r1, [pc, #0x150]        @ 0x5f78 <entry+0x5f78>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,336)?)?;
    // 0x5b624: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5b628: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5b62c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c9a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b630: ldr r1, [pc, #0x144]        @ 0x5f7c <entry+0x5f7c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,324)?)?;
    // 0x5b634: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5b638: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2c9ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b63c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5b640: cmp r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,12)?)?;
    // 0x5b644: bne 0x5b69c <entry+0x5e9c>  @ imm = #0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b69c,false)?)?;
    // 0x5b648: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5b64c: bl 0x5b794 <entry+0x5f94>  @ imm = #0x140
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x5b650: ldr r5, [pc, #0x130]        @ 0x5f88 <entry+0x5f88>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,304)?)?;
    // 0x5b654: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x5b658: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x5b65c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x5b660: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x5b664: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x5b668: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5b66c: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x5b670: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b674: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5b678: bne 0x5b6ec <entry+0x5eec>  @ imm = #0x6c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b6ec,false)?)?;
    // 0x5b67c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5b680: bl 0x59604 <entry+0x3e04>  @ imm = #-0x2084
    c.emit(arm32::branch(C::Al,c.at as u32,0x59604,true)?)?;
    // 0x5b684: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x5b688: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x5b68c: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x5b690: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x5b694: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x58d70
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x5b698: b 0x5b75c <entry+0x5f5c>  @ imm = #0xbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b75c,false)?)?;
    // 0x5b69c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x5b6a0: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b6a4: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x5b6a8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x5b6ac: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b6b0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b6b4: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x5b6b8: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x5b6bc: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x5b6c0: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x30d38
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b6c4: ldr r1, [pc, #0xb4]         @ 0x5f80 <entry+0x5f80>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,180)?)?;
    // 0x5b6c8: ldr r2, [pc, #0xb4]         @ 0x5f84 <entry+0x5f84>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,180)?)?;
    // 0x5b6cc: mov r3, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,120)?)?;
    // 0x5b6d0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b6d4: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b6d8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ca4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b6dc: ldr r1, [pc, #0x98]         @ 0x5f7c <entry+0x5f7c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,152)?)?;
    // 0x5b6e0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b6e4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ca58
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b6e8: b 0x5b76c <entry+0x5f6c>  @ imm = #0x7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b76c,false)?)?;
    // 0x5b6ec: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b6f0: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x5b6f4: bne 0x5b724 <entry+0x5f24>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b724,false)?)?;
    // 0x5b6f8: ldr r1, [pc, #0x90]         @ 0x5f90 <entry+0x5f90>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,144)?)?;
    // 0x5b6fc: ldr r2, [pc, #0x80]         @ 0x5f84 <entry+0x5f84>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,128)?)?;
    // 0x5b700: mov r3, #146
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,146)?)?;
    // 0x5b704: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b708: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b70c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ca80
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b710: ldr r1, [pc, #0x64]         @ 0x5f7c <entry+0x5f7c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,100)?)?;
    // 0x5b714: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b718: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ca8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b71c: add r0, r5, #14548992
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,14548992)?)?;
    // 0x5b720: b 0x5b758 <entry+0x5f58>  @ imm = #0x30
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b758,false)?)?;
    // 0x5b724: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b728: ldr r1, [pc, #0x5c]         @ 0x5f8c <entry+0x5f8c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,92)?)?;
    // 0x5b72c: ldr r2, [pc, #0x50]         @ 0x5f84 <entry+0x5f84>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,80)?)?;
    // 0x5b730: mov r3, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,153)?)?;
    // 0x5b734: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b738: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5b73c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b740: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2cab4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b744: ldr r1, [pc, #0x30]         @ 0x5f7c <entry+0x5f7c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,48)?)?;
    // 0x5b748: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b74c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2cac0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b750: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x5b754: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x5b758: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x5b75c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5b760: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x5b764: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x30ef4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x5b768: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x5b76c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5b770: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x5b774: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x5b778)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5B778:[u32;7]=[0x000eea53,0x000e2788,0x000eea62,0x000eea8c,0x00221002,0x000eeace,0x000eea9e];
 for value in INLINE_DATA_5B778{c.emit(value)?;}
 // Executable A32 block 0x5b794..0x5c000.
 c.check(0x5b794)?;
    // 0x5b794: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5b798: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x5b79c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x5b7a0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b7a4: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x5b7a8: ldrne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b7ac: cmpne r0, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x5b7b0: beq 0x5b7c8 <entry+0x5fc8>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b7c8,false)?)?;
    // 0x5b7b4: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b7b8: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // 0x5b7bc: ldrne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,0)?)?;
    // 0x5b7c0: cmpne r0, #10
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,10)?)?;
    // 0x5b7c4: bne 0x5b7dc <entry+0x5fdc>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b7dc,false)?)?;
    // 0x5b7c8: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x5b7cc: bl 0x294c <entry+0xfffffffffffad14c> @ imm = #-0x58e88
    c.emit(arm32::branch(C::Al,c.at as u32,0x294c,true)?)?;
    // 0x5b7d0: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x5b7d4: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5b7d8: b 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x58ea4
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,false)?)?;
    // 0x5b7dc: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5b7e0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5b7e4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x5b7e8: sub sp, sp, #388
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,388)?)?;
    // 0x5b7ec: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x5b7f0: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x5b7f4: ldr r5, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,0)?)?;
    // 0x5b7f8: cmp r5, #19
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,19)?)?;
    // 0x5b7fc: bhi 0x5b838 <entry+0x6038>  @ imm = #0x34
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5b838,false)?)?;
    // 0x5b800: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5b804: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5b808: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x5b80c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5b810: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b814: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b818: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x5b81c: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x5b820: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x5b824: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x30e9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b828: ldr r1, [pc, #0xfe0]        @ 0x7010 <entry+0x7010>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4064)?)?;
    // 0x5b82c: ldr r2, [pc, #0xfe0]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,4064)?)?;
    // 0x5b830: mov r3, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,180)?)?;
    // 0x5b834: b 0x5b878 <entry+0x6078>  @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b878,false)?)?;
    // 0x5b838: mvn r0, r5
    c.emit(arm32::data_register(C::Al,Op::Mvn,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x5b83c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5b840: bls 0x5b89c <entry+0x609c>  @ imm = #0x54
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5b89c,false)?)?;
    // 0x5b844: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5b848: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5b84c: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x5b850: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5b854: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5b858: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b85c: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x5b860: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x5b864: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x5b868: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x30ee0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5b86c: ldr r1, [pc, #0xfcc]        @ 0x7040 <entry+0x7040>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4044)?)?;
    // 0x5b870: ldr r2, [pc, #0xf9c]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3996)?)?;
    // 0x5b874: mov r3, #187
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,187)?)?;
    // 0x5b878: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b87c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b880: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2cbf4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b884: ldr r1, [pc, #0xfb8]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4024)?)?;
    // 0x5b888: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b88c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2cc00
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b890: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x5b894: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x5b898: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x5b89c: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x5b8a0: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x114
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x5b8a4: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x5b8a8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5b8ac: beq 0x5b8c0 <entry+0x60c0>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5b8c0,false)?)?;
    // 0x5b8b0: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x5b8b4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x5b8b8: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x5b8bc: bne 0x5b8ec <entry+0x60ec>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5b8ec,false)?)?;
    // 0x5b8c0: ldr r1, [pc, #0xfc8]        @ 0x7090 <entry+0x7090>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4040)?)?;
    // 0x5b8c4: ldr r2, [pc, #0xf48]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3912)?)?;
    // 0x5b8c8: mov r3, #199
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,199)?)?;
    // 0x5b8cc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b8d0: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5b8d4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2cc48
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b8d8: ldr r1, [pc, #0xf64]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3940)?)?;
    // 0x5b8dc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5b8e0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2cc54
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5b8e4: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x5b8e8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5b8ec: ldr r8, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(7)?,12)?)?;
    // 0x5b8f0: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x5b8f4: beq 0x5bd30 <entry+0x6530>  @ imm = #0x434
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5bd30,false)?)?;
    // 0x5b8f8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x5b8fc: bne 0x5bd5c <entry+0x655c>  @ imm = #0x458
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5bd5c,false)?)?;
    // 0x5b900: add r0, r9, #296
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,296)?)?;
    // 0x5b904: ldr r2, [pc, #0xff4]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,4084)?)?;
    // 0x5b908: mov r1, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,216)?)?;
    // 0x5b90c: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // 0x5b910: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x5b914: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x5b918: add r0, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x5b91c: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x5b920: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5b924: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5a17c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5b928: add r0, r9, #976
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,976)?)?;
    // 0x5b92c: mov r4, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,20480)?)?;
    // 0x5b930: add r10, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(7)?,16)?)?;
    // 0x5b934: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5b938: orr r4, r4, #35651584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,35651584)?)?;
    // 0x5b93c: str r0, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-72)?)?;
    // 0x5b940: add r0, r9, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,84)?)?;
    // 0x5b944: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // 0x5b948: str r0, [r11, #-0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-168)?)?;
    // 0x5b94c: add r0, r9, #184
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,184)?)?;
    // 0x5b950: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5b954: str r0, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-68)?)?;
    // 0x5b958: add r0, r9, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,56)?)?;
    // 0x5b95c: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5b960: str r0, [r11, #-0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-160)?)?;
    // 0x5b964: add r0, r9, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,52)?)?;
    // 0x5b968: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5b96c: str r0, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-76)?)?;
    // 0x5b970: add r0, r9, #1072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1072)?)?;
    // 0x5b974: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5b978: str r0, [r11, #-0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-172)?)?;
    // 0x5b97c: add r0, r9, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,44)?)?;
    // 0x5b980: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5b984: str r0, [r11, #-0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-176)?)?;
    // 0x5b988: add r0, r9, #239
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,239)?)?;
    // 0x5b98c: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5b990: str r0, [r11, #-0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-180)?)?;
    // 0x5b994: add r0, r9, #250
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,250)?)?;
    // 0x5b998: add r0, r0, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3840)?)?;
    // 0x5b99c: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // 0x5b9a0: add r0, r9, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,20)?)?;
    // 0x5b9a4: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5b9a8: str r0, [sp, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,152)?)?;
    // 0x5b9ac: add r0, r9, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,24)?)?;
    // 0x5b9b0: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5b9b4: str r0, [sp, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,148)?)?;
    // 0x5b9b8: add r0, r9, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,28)?)?;
    // 0x5b9bc: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5b9c0: str r0, [sp, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,144)?)?;
    // 0x5b9c4: mov r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // 0x5b9c8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x5b9cc: add r0, r9, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5b9d0: str r0, [sp, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,140)?)?;
    // 0x5b9d4: add r0, r9, #26
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,26)?)?;
    // 0x5b9d8: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5b9dc: str r0, [sp, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,136)?)?;
    // 0x5b9e0: add r0, r9, #18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,18)?)?;
    // 0x5b9e4: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5b9e8: str r0, [sp, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,132)?)?;
    // 0x5b9ec: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x5b9f0: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5b9f4: str r0, [r11, #-0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-128)?)?;
    // 0x5b9f8: add r0, r9, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,36)?)?;
    // 0x5b9fc: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5ba00: str r0, [r11, #-0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-108)?)?;
    // 0x5ba04: add r0, r9, #34
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,34)?)?;
    // 0x5ba08: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5ba0c: str r0, [r11, #-0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-112)?)?;
    // 0x5ba10: add r0, r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,32)?)?;
    // 0x5ba14: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5ba18: str r0, [r11, #-0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-116)?)?;
    // 0x5ba1c: add r0, r9, #30
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,30)?)?;
    // 0x5ba20: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5ba24: str r0, [r11, #-0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-120)?)?;
    // 0x5ba28: add r0, r9, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1004)?)?;
    // 0x5ba2c: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5ba30: str r0, [r11, #-0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-184)?)?;
    // 0x5ba34: add r0, r9, #235
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,235)?)?;
    // 0x5ba38: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5ba3c: str r0, [r11, #-0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-188)?)?;
    // 0x5ba40: add r0, r9, #242
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,242)?)?;
    // 0x5ba44: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5ba48: str r0, [r11, #-0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-164)?)?;
    // 0x5ba4c: add r0, r9, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1000)?)?;
    // 0x5ba50: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5ba54: str r0, [r11, #-0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-192)?)?;
    // 0x5ba58: add r0, r9, #158
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,158)?)?;
    // 0x5ba5c: add r0, r0, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3328)?)?;
    // 0x5ba60: str r0, [r11, #-0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-136)?)?;
    // 0x5ba64: add r0, r9, #412
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,412)?)?;
    // 0x5ba68: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // 0x5ba6c: str r0, [r11, #-0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-140)?)?;
    // 0x5ba70: add r0, r9, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,60)?)?;
    // 0x5ba74: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5ba78: str r0, [r11, #-0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-196)?)?;
    // 0x5ba7c: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x5ba80: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5ba84: str r0, [r11, #-0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-200)?)?;
    // 0x5ba88: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x5ba8c: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5ba90: str r0, [r11, #-0xcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-204)?)?;
    // 0x5ba94: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x5ba98: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5ba9c: str r0, [sp, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,208)?)?;
    // 0x5baa0: add r0, r9, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,40)?)?;
    // 0x5baa4: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5baa8: str r0, [sp, #0xcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,204)?)?;
    // 0x5baac: add r0, r9, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,36)?)?;
    // 0x5bab0: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bab4: str r0, [sp, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,200)?)?;
    // 0x5bab8: add r0, r9, #1056
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1056)?)?;
    // 0x5babc: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bac0: str r0, [sp, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,196)?)?;
    // 0x5bac4: add r0, r9, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,28)?)?;
    // 0x5bac8: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bacc: str r0, [sp, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,192)?)?;
    // 0x5bad0: add r0, r9, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,24)?)?;
    // 0x5bad4: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bad8: str r0, [sp, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,188)?)?;
    // 0x5badc: add r0, r9, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,20)?)?;
    // 0x5bae0: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bae4: str r0, [sp, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,184)?)?;
    // 0x5bae8: add r0, r9, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1040)?)?;
    // 0x5baec: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5baf0: str r0, [sp, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,180)?)?;
    // 0x5baf4: add r0, r9, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1020)?)?;
    // 0x5baf8: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bafc: str r0, [r11, #-0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-132)?)?;
    // 0x5bb00: add r0, r9, #214
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,214)?)?;
    // 0x5bb04: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bb08: str r0, [r11, #-0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-80)?)?;
    // 0x5bb0c: add r0, r9, #213
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,213)?)?;
    // 0x5bb10: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bb14: str r0, [r11, #-0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-84)?)?;
    // 0x5bb18: add r0, r9, #227
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,227)?)?;
    // 0x5bb1c: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bb20: str r0, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-60)?)?;
    // 0x5bb24: add r0, r9, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,68)?)?;
    // 0x5bb28: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb2c: str r0, [sp, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,176)?)?;
    // 0x5bb30: add r0, r9, #136
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,136)?)?;
    // 0x5bb34: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb38: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x5bb3c: add r0, r9, #132
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,132)?)?;
    // 0x5bb40: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb44: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x5bb48: add r0, r9, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,124)?)?;
    // 0x5bb4c: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb50: str r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,100)?)?;
    // 0x5bb54: add r0, r9, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,72)?)?;
    // 0x5bb58: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb5c: str r0, [sp, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,120)?)?;
    // 0x5bb60: add r0, r9, #116
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,116)?)?;
    // 0x5bb64: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb68: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x5bb6c: add r0, r9, #1136
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1136)?)?;
    // 0x5bb70: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bb74: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x5bb78: add r0, r9, #120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,120)?)?;
    // 0x5bb7c: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb80: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x5bb84: add r0, r9, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,108)?)?;
    // 0x5bb88: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb8c: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x5bb90: add r0, r9, #100
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,100)?)?;
    // 0x5bb94: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bb98: str r0, [sp, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,116)?)?;
    // 0x5bb9c: add r0, r9, #1120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1120)?)?;
    // 0x5bba0: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bba4: str r0, [sp, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,112)?)?;
    // 0x5bba8: add r0, r9, #104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,104)?)?;
    // 0x5bbac: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bbb0: str r0, [sp, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,108)?)?;
    // 0x5bbb4: add r0, r9, #92
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,92)?)?;
    // 0x5bbb8: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bbbc: str r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,104)?)?;
    // 0x5bbc0: add r0, r9, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,60)?)?;
    // 0x5bbc4: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bbc8: str r0, [sp, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,160)?)?;
    // 0x5bbcc: add r0, r9, #225
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,225)?)?;
    // 0x5bbd0: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bbd4: str r0, [sp, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,156)?)?;
    // 0x5bbd8: add r0, r9, #238
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,238)?)?;
    // 0x5bbdc: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bbe0: str r0, [sp, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,172)?)?;
    // 0x5bbe4: add r0, r9, #992
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,992)?)?;
    // 0x5bbe8: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bbec: str r0, [sp, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,168)?)?;
    // 0x5bbf0: add r0, r9, #1008
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1008)?)?;
    // 0x5bbf4: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bbf8: str r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-64)?)?;
    // 0x5bbfc: add r0, r9, #222
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,222)?)?;
    // 0x5bc00: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bc04: str r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-48)?)?;
    // 0x5bc08: add r0, r9, #221
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,221)?)?;
    // 0x5bc0c: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bc10: str r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-52)?)?;
    // 0x5bc14: add r0, r9, #196
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,196)?)?;
    // 0x5bc18: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5120)?)?;
    // 0x5bc1c: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x5bc20: add r0, r9, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1016)?)?;
    // 0x5bc24: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bc28: str r0, [r11, #-0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-144)?)?;
    // 0x5bc2c: add r0, r9, #980
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,980)?)?;
    // 0x5bc30: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bc34: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x5bc38: add r0, r9, #210
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,210)?)?;
    // 0x5bc3c: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bc40: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // 0x5bc44: add r0, r9, #237
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,237)?)?;
    // 0x5bc48: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bc4c: str r0, [r11, #-0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-88)?)?;
    // 0x5bc50: add r0, r9, #206
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,206)?)?;
    // 0x5bc54: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bc58: str r0, [r11, #-0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-92)?)?;
    // 0x5bc5c: add r0, r9, #207
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,207)?)?;
    // 0x5bc60: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bc64: str r0, [r11, #-0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-96)?)?;
    // 0x5bc68: add r0, r9, #205
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,205)?)?;
    // 0x5bc6c: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bc70: str r0, [r11, #-0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-100)?)?;
    // 0x5bc74: add r0, r9, #972
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,972)?)?;
    // 0x5bc78: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bc7c: str r0, [r11, #-0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-104)?)?;
    // 0x5bc80: add r0, r9, #964
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,964)?)?;
    // 0x5bc84: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x5bc88: str r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-44)?)?;
    // 0x5bc8c: add r0, r9, #241
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,241)?)?;
    // 0x5bc90: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bc94: str r0, [r11, #-0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-148)?)?;
    // 0x5bc98: add r0, r9, #197
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,197)?)?;
    // 0x5bc9c: add r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
    // 0x5bca0: str r0, [sp, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,124)?)?;
    // 0x5bca4: add r0, r9, #456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,456)?)?;
    // 0x5bca8: str r0, [sp, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,128)?)?;
    // 0x5bcac: add r0, r9, #528
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,528)?)?;
    // 0x5bcb0: str r0, [r11, #-0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-152)?)?;
    // 0x5bcb4: add r0, r9, #3488
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,3488)?)?;
    // 0x5bcb8: str r0, [r11, #-0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-156)?)?;
    // 0x5bcbc: add r0, r9, #5184
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,5184)?)?;
    // 0x5bcc0: str r0, [sp, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,164)?)?;
    // 0x5bcc4: add r0, r9, #5248
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,5248)?)?;
    // 0x5bcc8: str r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,96)?)?;
    // 0x5bccc: add r0, r9, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,124)?)?;
    // 0x5bcd0: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // 0x5bcd4: add r0, r9, #600
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,600)?)?;
    // 0x5bcd8: str r0, [r11, #-0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-124)?)?;
    // 0x5bcdc: add r0, r9, #492
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,492)?)?;
    // 0x5bce0: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x5bce4: add r0, r9, #564
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,564)?)?;
    // 0x5bce8: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x5bcec: add r0, r9, #5312
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,5312)?)?;
    // 0x5bcf0: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x5bcf4: add r0, r9, #300
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,300)?)?;
    // 0x5bcf8: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x5bcfc: ldr r0, [pc, #0xfe4]        @ 0x74e8 <entry+0x74e8>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,4068)?)?;
    // 0x5bd00: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x5bd04: ldr r0, [pc, #0xfe0]        @ 0x74ec <entry+0x74ec>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,4064)?)?;
    // 0x5bd08: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x5bd0c: ldr r0, [pc, #0xfdc]        @ 0x74f0 <entry+0x74f0>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,4060)?)?;
    // 0x5bd10: str r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // 0x5bd14: ldr r0, [pc, #0xfd8]        @ 0x74f4 <entry+0x74f4>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,4056)?)?;
    // 0x5bd18: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x5bd1c: ldr r0, [pc, #0xfd4]        @ 0x74f8 <entry+0x74f8>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,4052)?)?;
    // 0x5bd20: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x5bd24: ldr r0, [pc, #0xfd0]        @ 0x74fc <entry+0x74fc>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,4048)?)?;
    // 0x5bd28: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // 0x5bd2c: b 0x5c9a0 <entry+0x71a0>  @ imm = #0xc6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c9a0,false)?)?;
    // 0x5bd30: ldr r1, [pc, #0xff8]        @ 0x7530 <entry+0x7530>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5bd34: ldr r2, [pc, #0xad8]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2776)?)?;
    // 0x5bd38: mov r3, #204
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,204)?)?;
    // 0x5bd3c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5bd40: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5bd44: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d0b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bd48: ldr r1, [pc, #0xaf4]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2804)?)?;
    // 0x5bd4c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5bd50: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d0c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bd54: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x5bd58: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5bd5c: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5bd60: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5bd64: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5bd68: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5bd6c: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x5bd70: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x5bd74: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x5bd78: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x313f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5bd7c: b 0x5b890 <entry+0x6090>  @ imm = #-0x4f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b890,false)?)?;
    // 0x5bd80: ldr r1, [pc, #0xfe8]        @ 0x7570 <entry+0x7570>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4072)?)?;
    // 0x5bd84: ldr r2, [pc, #0xa88]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2696)?)?;
    // 0x5bd88: mov r3, #157
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,157)?)?;
    // 0x5bd8c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5bd90: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5bd94: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7168)?)?;
    // 0x5bd98: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d10c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bd9c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5bda0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5bda4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d118
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bda8: ldr r2, [pc, #0xb50]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2896)?)?;
    // 0x5bdac: mov r1, #158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,158)?)?;
    // 0x5bdb0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5bdb4: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x5bdb8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5a610
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5bdbc: b 0x5c55c <entry+0x6d5c>  @ imm = #0x798
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c55c,false)?)?;
    // 0x5bdc0: mov r1, #308
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,308)?)?;
    // 0x5bdc4: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // 0x5bdc8: str r0, [r9, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x5bdcc: ldr r4, [pc, #0xa70]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,2672)?)?;
    // 0x5bdd0: ldr r1, [pc, #0xfd4]        @ 0x75ac <entry+0x75ac>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4052)?)?;
    // 0x5bdd4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bdd8: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5bddc: b 0x5c124 <entry+0x6924>  @ imm = #0x340
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c124,false)?)?;
    // 0x5bde0: ldr r5, [pc, #0xa5c]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,2652)?)?;
    // 0x5bde4: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // 0x5bde8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5bdec: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5bdf0: moveq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5bdf4: strb r0, [r9, #0xfdb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,4059)?)?;
    // 0x5bdf8: strb r1, [r9, #0xff8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,4088)?)?;
    // 0x5bdfc: ldr r0, [r11, #-0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-120)?)?;
    // 0x5be00: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x5be04: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x5be08: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x5be0c: strh r1, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::R0,0)?)?;
    // 0x5be10: ldr r0, [r11, #-0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-116)?)?;
    // 0x5be14: strh r2, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,0)?)?;
    // 0x5be18: ldr r0, [r11, #-0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-112)?)?;
    // 0x5be1c: strh r1, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::R0,0)?)?;
    // 0x5be20: ldr r1, [pc, #0xff8]        @ 0x7620 <entry+0x7620>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5be24: ldr r0, [r11, #-0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-108)?)?;
    // 0x5be28: strh r2, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,0)?)?;
    // 0x5be2c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5be30: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // 0x5be34: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d1a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5be38: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5be3c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5be40: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d1b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5be44: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5be48: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5be4c: beq 0x5c1d4 <entry+0x69d4>  @ imm = #0x380
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c1d4,false)?)?;
    // 0x5be50: ldr r1, [sp, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,136)?)?;
    // 0x5be54: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x5be58: strh r4, [r1]
    c.emit(arm32::store_halfword(C::Al,R::new(4)?,R::R1,0)?)?;
    // 0x5be5c: ldr r1, [sp, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,140)?)?;
    // 0x5be60: strh r4, [r1]
    c.emit(arm32::store_halfword(C::Al,R::new(4)?,R::R1,0)?)?;
    // 0x5be64: ldr r1, [r11, #-0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-128)?)?;
    // 0x5be68: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // 0x5be6c: str r4, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,4)?)?;
    // 0x5be70: ldr r1, [sp, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,144)?)?;
    // 0x5be74: strh r6, [r1]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::R1,0)?)?;
    // 0x5be78: ldr r1, [sp, #0x94]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,148)?)?;
    // 0x5be7c: strh r6, [r1]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::R1,0)?)?;
    // 0x5be80: ldr r1, [sp, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,152)?)?;
    // 0x5be84: strh r6, [r1]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::R1,0)?)?;
    // 0x5be88: beq 0x5beac <entry+0x66ac>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5beac,false)?)?;
    // 0x5be8c: ldr r3, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,56)?)?;
    // 0x5be90: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5be94: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5be98: ldr r7, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(7)?,R::R1,4)?)?;
    // 0x5be9c: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x5bea0: cmp r2, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,9)?)?;
    // 0x5bea4: strh r7, [r3], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(7)?,R::new(3)?,2)?)?;
    // 0x5bea8: bne 0x5be98 <entry+0x6698>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5be98,false)?)?;
    // 0x5beac: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x5beb0: beq 0x5bed4 <entry+0x66d4>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5bed4,false)?)?;
    // 0x5beb4: ldr r3, [r11, #-0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-128)?)?;
    // 0x5beb8: add r1, r10, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,48)?)?;
    // 0x5bebc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5bec0: ldr r7, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(7)?,R::R1,4)?)?;
    // 0x5bec4: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x5bec8: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // 0x5becc: strh r7, [r3], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(7)?,R::new(3)?,2)?)?;
    // 0x5bed0: bne 0x5bec0 <entry+0x66c0>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5bec0,false)?)?;
    // 0x5bed4: mov r4, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,20480)?)?;
    // 0x5bed8: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // 0x5bedc: orr r4, r4, #35651584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,35651584)?)?;
    // 0x5bee0: beq 0x5bf24 <entry+0x6724>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5bf24,false)?)?;
    // 0x5bee4: ldr r2, [sp, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,132)?)?;
    // 0x5bee8: add r0, r10, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,60)?)?;
    // 0x5beec: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5bef0: ldrb r3, [r0], #4
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(3)?,R::R0,4)?)?;
    // 0x5bef4: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x5bef8: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x5befc: strh r3, [r2], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(3)?,R::R2,2)?)?;
    // 0x5bf00: bne 0x5bef0 <entry+0x66f0>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5bef0,false)?)?;
    // 0x5bf04: ldr r1, [sp, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,132)?)?;
    // 0x5bf08: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5bf0c: ldrh r2, [r1]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::R1,0)?)?;
    // 0x5bf10: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5bf14: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x5bf18: strh r2, [r1, #12]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R1,12)?)?;
    // 0x5bf1c: add r1, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // 0x5bf20: bne 0x5bf0c <entry+0x670c>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5bf0c,false)?)?;
    // 0x5bf24: add r7, r10, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(10)?,20)?)?;
    // 0x5bf28: ldr r2, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,12)?)?;
    // 0x5bf2c: ldr r3, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,16)?)?;
    // 0x5bf30: ldm r7, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5bf34: ldr r6, [r10, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,32)?)?;
    // 0x5bf38: stm sp, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5bf3c: ldr r1, [pc, #0xf9c]        @ 0x76e0 <entry+0x76e0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3996)?)?;
    // 0x5bf40: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bf44: str r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // 0x5bf48: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d2bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bf4c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bf50: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5bf54: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d2c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bf58: add r7, r10, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(10)?,44)?)?;
    // 0x5bf5c: ldr r2, [r10, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,36)?)?;
    // 0x5bf60: ldr r3, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,40)?)?;
    // 0x5bf64: ldm r7, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5bf68: ldr r6, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,56)?)?;
    // 0x5bf6c: stm sp, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5bf70: ldr r1, [pc, #0xff8]        @ 0x7770 <entry+0x7770>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5bf74: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bf78: str r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // 0x5bf7c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d2f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bf80: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bf84: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5bf88: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d2fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bf8c: add r7, r10, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(10)?,68)?)?;
    // 0x5bf90: ldr r2, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,60)?)?;
    // 0x5bf94: ldr r3, [r10, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,64)?)?;
    // 0x5bf98: ldm r7, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5bf9c: ldr r6, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,80)?)?;
    // 0x5bfa0: stm sp, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5bfa4: ldr r1, [pc, #0xfc8]        @ 0x7774 <entry+0x7774>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4040)?)?;
    // 0x5bfa8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bfac: str r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // 0x5bfb0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d324
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bfb4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bfb8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5bfbc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d330
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bfc0: ldr r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,16)?)?;
    // 0x5bfc4: b 0x5e4c4 <entry+0x8cc4>  @ imm = #0x24f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4c4,false)?)?;
    // 0x5bfc8: ldr r1, [pc, #0xfa8]        @ 0x7778 <entry+0x7778>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4008)?)?;
    // 0x5bfcc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bfd0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d344
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bfd4: ldr r4, [pc, #0x868]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,2152)?)?;
    // 0x5bfd8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5bfdc: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5bfe0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d354
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5bfe4: mov r1, #227
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,227)?)?;
    // 0x5bfe8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5bfec: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5bff0: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x5bff4: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5a84c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5bff8: ldmib r10, {r2, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5bffc: ldr r1, [pc, #0xf78]        @ 0x777c <entry+0x777c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3960)?)?;
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
