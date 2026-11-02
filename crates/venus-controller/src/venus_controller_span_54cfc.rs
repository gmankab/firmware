// SPDX-License-Identifier: GPL-3.0-only
//! Codec controller resource policy, indexed 16-bit sample fields and
//! platform parameter marshaling. This is a partial larger function, not
//! a complete return. Architecture A32 instruction fields are synthesized
//! from typed operands; private original bytes remain outside source control.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_controller_format_registers::signed_byte_register as load_signed_byte_register;
pub const START:usize=0x54cfc;
pub const END:usize=0x55400;
pub const FILE_OFFSET:usize=0x4000+START;

/// Public A32 indexed halfword load/store with optional preindex writeback.
/// Register-offset mode is distinct from immediate-offset halfword transfers.
fn transfer_halfword_register(cond:C,load:bool,rd:R,base:R,off:R,writeback:bool)->Result<u32,String>{
 if [base,off,rd].contains(&R::PC) || (writeback && load && rd==base){
   return Err("unsafe A32 halfword register-indexed addressing".into());
 }
 Ok(((cond as u32)<<28)|0x0180_00b0|(if load{1<<20}else{0})|
   (if writeback{1<<21}else{0})|(base.number()<<16)|(rd.number()<<12)|off.number())
}

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,a:usize)->Result<(),String>{if self.at!=a{return Err(format!("codec mode 0x{:x} != {a:#x}",self.at))}Ok(())}
 #[inline(never)]fn emit(&mut self,word:u32)->Result<(),String>{
  if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0{return Err("Agent04 address boundary".into())}
  self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
 }
 fn literal(&mut self,cond:C,dst:R,target:usize)->Result<(),String>{
   let off=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"invalid literal range")?;
   self.emit(arm32::transfer_word(cond,true,dst,R::PC,off)?)
 }
}

/// Select codec format and source buffers, copying numeric state into the
/// controller's operation register map.
#[inline(never)]fn configure_controller_resource_policy(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x54cfc: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x54d00: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x54d04: sub sp, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,44)?)?;
    // 0x54d08: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x54d0c: ldr r1, [r4, #0xab0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2736)?)?;
    // 0x54d10: add r0, r4, #664
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,664)?)?;
    // 0x54d14: add r5, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,3072)?)?;
    // 0x54d18: add r0, r4, #296
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,296)?)?;
    // 0x54d1c: add r6, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,3072)?)?;
    // 0x54d20: str r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,32)?)?;
    // 0x54d24: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x54d28: beq 0x54dac <entry+0x1468>  @ imm = #0x7c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54dac,false)?)?;
    // 0x54d2c: mov r2, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,668)?)?;
    // 0x54d30: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x54d34: orr r2, r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // 0x54d38: bl 0x471c <entry+0xfffffffffffb0dd8> @ imm = #-0x50624
    c.emit(arm32::branch(C::Al,c.at as u32,0x471c,true)?)?;
    // 0x54d3c: ldr r1, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1392)?)?;
    // 0x54d40: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x54d44: bl 0x58130 <entry+0x47ec>  @ imm = #0x33e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x58130,true)?)?;
    // 0x54d48: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x54d4c: bl 0x58214 <entry+0x48d0>  @ imm = #0x34c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x58214,true)?)?;
    // 0x54d50: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x54d54: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x54d58: str r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2732)?)?;
    // 0x54d5c: mov r0, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,230)?)?;
    // 0x54d60: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x54d64: strb r7, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(7)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x54d68: ldrb r0, [r4, #0xfdd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,4061)?)?;
    // 0x54d6c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54d70: bne 0x54d8c <entry+0x1448>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54d8c,false)?)?;
    // 0x54d74: ldr r0, [r4, #0xfe0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4064)?)?;
    // 0x54d78: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54d7c: bne 0x54d8c <entry+0x1448>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54d8c,false)?)?;
    // 0x54d80: ldr r0, [r4, #0xfe4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4068)?)?;
    // 0x54d84: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54d88: beq 0x568bc <entry+0x2f78>  @ imm = #0x1b2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x568bc,false)?)?;
    // 0x54d8c: mov r0, #200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,200)?)?;
    // 0x54d90: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x54d94: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x54d98: str r7, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x54d9c: ldr r0, [r3, #0x284]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(3)?,644)?)?;
    // 0x54da0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x54da4: beq 0x5501c <entry+0x16d8>  @ imm = #0x270
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5501c,false)?)?;
    // 0x54da8: b 0x5504c <entry+0x1708>  @ imm = #0x29c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5504c,false)?)?;
    // 0x54dac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x54db0: str r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2732)?)?;
    // 0x54db4: ldr r1, [r4, #0xd74]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3444)?)?;
    // 0x54db8: ldr r2, [r4, #0xe48]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,3656)?)?;
    // 0x54dbc: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x54dc0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x54dc4: movlo r2, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x54dc8: strb r2, [r4, #0xe74]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,3700)?)?;
    // 0x54dcc: mov r2, #241
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,241)?)?;
    // 0x54dd0: orr r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4864)?)?;
    // 0x54dd4: ldrb r2, [r4, r2]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R2,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x54dd8: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x54ddc: bne 0x54df0 <entry+0x14ac>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54df0,false)?)?;
    // 0x54de0: mov r1, #188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,188)?)?;
    // 0x54de4: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x54de8: str r0, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x54dec: b 0x54e30 <entry+0x14ec>  @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x54e30,false)?)?;
    // 0x54df0: ldr r2, [r4, #0xd78]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,3448)?)?;
    // 0x54df4: mul r3, r2, r1
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // 0x54df8: ldr r1, [pc, #0xf54]        @ 0x2410 <entry+0x2410>
    c.literal(C::Al,R::R1,0x55d54)?;
    // 0x54dfc: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x54e00: mov r1, #188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,188)?)?;
    // 0x54e04: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x54e08: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x54e0c: str r0, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x54e10: bhs 0x54e30 <entry+0x14ec>  @ imm = #0x18
    c.emit(arm32::branch(C::Cs,c.at as u32,0x54e30,false)?)?;
    // 0x54e14: mov r0, #163
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,163)?)?;
    // 0x54e18: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x54e1c: str r0, [r4, #0xd18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3352)?)?;
    // 0x54e20: ldrb r1, [r4, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3568)?)?;
    // 0x54e24: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x54e28: movne r1, #4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x54e2c: b 0x54e48 <entry+0x1504>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x54e48,false)?)?;
    // 0x54e30: mov r0, #320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,320)?)?;
    // 0x54e34: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x54e38: str r0, [r4, #0xd18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3352)?)?;
    // 0x54e3c: ldrb r0, [r4, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3568)?)?;
    // 0x54e40: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x54e44: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x54e48: ldr r2, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,644)?)?;
    // 0x54e4c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x54e50: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x54e54: bl 0x38be4 <entry+0xfffffffffffe52a0> @ imm = #-0x1c278
    c.emit(arm32::branch(C::Al,c.at as u32,0x38be4,true)?)?;
    // 0x54e58: ldrb r8, [r4, #0xdf8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(4)?,3576)?)?;
    // 0x54e5c: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x54e60: bne 0x54ea0 <entry+0x155c>  @ imm = #0x38
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54ea0,false)?)?;
    // 0x54e64: ldr r0, [r4, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3372)?)?;
    // 0x54e68: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x54e6c: beq 0x54ea0 <entry+0x155c>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54ea0,false)?)?;
    // 0x54e70: ldr r0, [r4, #0xd48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3400)?)?;
    // 0x54e74: cmp r0, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3840)?)?;
    // 0x54e78: bhi 0x54ea0 <entry+0x155c>  @ imm = #0x20
    c.emit(arm32::branch(C::Hi,c.at as u32,0x54ea0,false)?)?;
    // 0x54e7c: mov r0, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,221)?)?;
    // 0x54e80: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x54e84: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x54e88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54e8c: bne 0x54ea0 <entry+0x155c>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54ea0,false)?)?;
    // 0x54e90: mov r0, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3648)?)?;
    // 0x54e94: ldrsb r0, [r4, r0]
    c.emit(load_signed_byte_register(C::Al,R::R0,R::new(4)?,R::R0)?)?;
    // 0x54e98: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x54e9c: blt 0x56b84 <entry+0x3240>  @ imm = #0x1ce0
    c.emit(arm32::branch(C::Lt,c.at as u32,0x56b84,false)?)?;
    // 0x54ea0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x54ea4: strb r7, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3104)?)?;
    // 0x54ea8: str r7, [r4, #0xec0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,3776)?)?;
    // 0x54eac: strb r7, [r4, #0x494]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1172)?)?;
    // 0x54eb0: ldrb r0, [r4, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3568)?)?;
    // 0x54eb4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x54eb8: bne 0x54f50 <entry+0x160c>  @ imm = #0x90
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54f50,false)?)?;
    // 0x54ebc: mov r0, #193
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,193)?)?;
    // 0x54ec0: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x54ec4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x54ec8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x54ecc: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x54ed0: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x54ed4: strb r7, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(7)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x54ed8: mov r0, #194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,194)?)?;
    // 0x54edc: strb r7, [r4, #0xc85]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3205)?)?;
    // 0x54ee0: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x54ee4: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x54ee8: strb r0, [r4, #0xc86]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3206)?)?;
    // 0x54eec: mov r0, #195
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,195)?)?;
    // 0x54ef0: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x54ef4: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x54ef8: strb r0, [r4, #0xc88]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3208)?)?;
    // 0x54efc: add r0, r4, #140
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,140)?)?;
    // 0x54f00: strb r6, [r4, #0xc87]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,3207)?)?;
    // 0x54f04: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // 0x54f08: bl 0x31ec <entry+0xfffffffffffaf8a8> @ imm = #-0x51d24
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x54f0c: mov r0, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,221)?)?;
    // 0x54f10: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x54f14: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x54f18: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54f1c: bne 0x54f40 <entry+0x15fc>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54f40,false)?)?;
    // 0x54f20: mov r0, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3648)?)?;
    // 0x54f24: ldrsb r0, [r4, r0]
    c.emit(load_signed_byte_register(C::Al,R::R0,R::new(4)?,R::R0)?)?;
    // 0x54f28: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54f2c: bgt 0x54f40 <entry+0x15fc>  @ imm = #0xc
    c.emit(arm32::branch(C::Gt,c.at as u32,0x54f40,false)?)?;
    // 0x54f30: ldr r0, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1392)?)?;
    // 0x54f34: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x54f38: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x54f3c: movne r7, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x54f40: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x54f44: strb r7, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3104)?)?;
    // 0x54f48: str r6, [r4, #0xec0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,3776)?)?;
    // 0x54f4c: strb r0, [r4, #0x494]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1172)?)?;
    // 0x54f50: ldr r0, [r4, #0xd78]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3448)?)?;
    // 0x54f54: cmp r0, #320
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,320)?)?;
    // 0x54f58: movls r0, #0
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x54f5c: strls r0, [r4, #0xec0]
    c.emit(arm32::transfer_word(C::Ls,false,R::R0,R::new(4)?,3776)?)?;
    // 0x54f60: ldr r0, [r4, #0xcb8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3256)?)?;
    // 0x54f64: ldr r1, [r4, #0xd88]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3464)?)?;
    // 0x54f68: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x54f6c: movhi r1, r0
    c.emit(arm32::mov_register(C::Hi,R::R1,R::R0))?;
    // 0x54f70: cmp r1, #61
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,61)?)?;
    // 0x54f74: andhs r0, r7, #253
    c.emit(arm32::data_immediate(C::Cs,Op::And,R::R0,R::new(7)?,253)?)?;
    // 0x54f78: strbhs r0, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Cs,false,R::R0,R::new(4)?,3104)?)?;
    // 0x54f7c: movhs r0, #0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x54f80: strhs r0, [r4, #0xec0]
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,3776)?)?;
    // 0x54f84: ldr r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
    // 0x54f88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54f8c: ldrne r0, [r4, #0xe48]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,3656)?)?;
    // 0x54f90: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54f94: bne 0x54fb8 <entry+0x1674>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54fb8,false)?)?;
    // 0x54f98: ldr r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,32)?)?;
    // 0x54f9c: ldr r1, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,280)?)?;
    // 0x54fa0: lsl r0, r8, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(8)?,Shift::Lsl,24)?)?;
    // 0x54fa4: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x54fa8: asr r2, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Asr,24)?)?;
    // 0x54fac: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x54fb0: bl 0x37be8 <entry+0xfffffffffffe42a4> @ imm = #-0x1d3d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x37be8,true)?)?;
    // 0x54fb4: b 0x54fc8 <entry+0x1684>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x54fc8,false)?)?;
    // 0x54fb8: ldr r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,32)?)?;
    // 0x54fbc: ldr r1, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,280)?)?;
    // 0x54fc0: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x54fc4: bl 0x37180 <entry+0xfffffffffffe383c> @ imm = #-0x1de4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x37180,true)?)?;
    // 0x54fc8: ldr r1, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1392)?)?;
    // 0x54fcc: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x54fd0: bl 0x58130 <entry+0x47ec>  @ imm = #0x3158
    c.emit(arm32::branch(C::Al,c.at as u32,0x58130,true)?)?;
    // 0x54fd4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x54fd8: bl 0x58214 <entry+0x48d0>  @ imm = #0x3234
    c.emit(arm32::branch(C::Al,c.at as u32,0x58214,true)?)?;
    // 0x54fdc: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x54fe0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x54fe4: bne 0x55048 <entry+0x1704>  @ imm = #0x5c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55048,false)?)?;
    // 0x54fe8: mov r0, #242
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,242)?)?;
    // 0x54fec: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x54ff0: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x54ff4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54ff8: bne 0x55018 <entry+0x16d4>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55018,false)?)?;
    // 0x54ffc: ldr r1, [r4, #0xd30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3376)?)?;

 c.check(0x55000)
}

/// Serialize byte/halfword device metadata, including packed source mode
/// flags and early position controls. Later function bytes remain unclaimed.
#[inline(never)]fn encode_controller_property_fields(c:&mut Code)->Result<(),String>{
 c.check(0x55000)?;
    // 0x55000: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x55004: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x55008: beq 0x55014 <entry+0x16d0>  @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55014,false)?)?;
    // 0x5500c: cmp r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,64)?)?;
    // 0x55010: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x55014: strb r0, [r4, #0xe8c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3724)?)?;
    // 0x55018: add r3, r4, #644
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,644)?)?;
    // 0x5501c: ldr r1, [r4, #0xe44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3652)?)?;
    // 0x55020: add r0, r4, #996
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,996)?)?;
    // 0x55024: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x55028: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5502c: moveq r1, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x55030: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x55034: strb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x55038: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5503c: strb r1, [r0, #0x410]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R1,R::R0,1040)?)?;
    // 0x55040: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x55044: b 0x55078 <entry+0x1734>  @ imm = #0x2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x55078,false)?)?;
    // 0x55048: add r3, r4, #644
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,644)?)?;
    // 0x5504c: mov r1, #996
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,996)?)?;
    // 0x55050: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x55054: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x55058: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5505c: strb r2, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R2,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x55060: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x55064: strb r2, [r1, #0x410]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R2,R::R1,1040)?)?;
    // 0x55068: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x5506c: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x55070: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x55074: bne 0x5507c <entry+0x1738>  @ imm = #0x0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5507c,false)?)?;
    // 0x55078: ldrb r1, [r4, #0xe95]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3733)?)?;
    // 0x5507c: mov r0, #972
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,972)?)?;
    // 0x55080: strb r1, [r4, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1005)?)?;
    // 0x55084: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x55088: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5508c: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x55090: mov r1, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,68)?)?;
    // 0x55094: ands r0, r0, #255
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,255,true)?)?;
    // 0x55098: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5509c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x550a0: str r0, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x550a4: str r6, [r4, #0xab4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,2740)?)?;
    // 0x550a8: str r5, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,2744)?)?;
    // 0x550ac: ldr r0, [r4, #0xab0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2736)?)?;
    // 0x550b0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x550b4: beq 0x550e0 <entry+0x179c>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x550e0,false)?)?;
    // 0x550b8: ldrb r0, [r4, #0xe94]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3732)?)?;
    // 0x550bc: mov r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // 0x550c0: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x550c4: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x550c8: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x550cc: subhi r1, r1, #3
    c.emit(arm32::data_immediate(C::Hi,Op::Sub,R::R1,R::R1,3)?)?;
    // 0x550d0: str r1, [r4, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x550d4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x550d8: strb r1, [r4, #0xfde]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,4062)?)?;
    // 0x550dc: b 0x550e4 <entry+0x17a0>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x550e4,false)?)?;
    // 0x550e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x550e4: strb r0, [r4, #0xc84]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3204)?)?;
    // 0x550e8: mov r0, #200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,200)?)?;
    // 0x550ec: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x550f0: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x550f4: str r0, [r4, #0xd1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3356)?)?;
    // 0x550f8: ldrb r0, [r4, #0xe68]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3688)?)?;
    // 0x550fc: strb r0, [r4, #0x3c0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,960)?)?;
    // 0x55100: ldrb r0, [r4, #0xe69]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3689)?)?;
    // 0x55104: strb r0, [r4, #0x3c1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,961)?)?;
    // 0x55108: ldrb r0, [r4, #0xe6a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3690)?)?;
    // 0x5510c: strb r0, [r4, #0x3c2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,962)?)?;
    // 0x55110: ldrb r0, [r4, #0xe67]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3687)?)?;
    // 0x55114: strb r0, [r4, #0x3bf]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,959)?)?;
    // 0x55118: ldrb r0, [r4, #0xe6b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3691)?)?;
    // 0x5511c: strb r0, [r4, #0x3c3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,963)?)?;
    // 0x55120: ldrb r0, [r4, #0xe6c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3692)?)?;
    // 0x55124: strb r0, [r4, #0x3c4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,964)?)?;
    // 0x55128: ldrb r0, [r4, #0xe6d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3693)?)?;
    // 0x5512c: strb r0, [r4, #0x3c5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,965)?)?;
    // 0x55130: ldrb r0, [r4, #0xe6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3694)?)?;
    // 0x55134: strb r0, [r4, #0x3c6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,966)?)?;
    // 0x55138: ldrb r0, [r4, #0xe6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3695)?)?;
    // 0x5513c: strb r0, [r4, #0x3c7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,967)?)?;
    // 0x55140: ldrb r0, [r4, #0xe70]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3696)?)?;
    // 0x55144: strb r0, [r4, #0x3c8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,968)?)?;
    // 0x55148: ldrb r0, [r4, #0xe71]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3697)?)?;
    // 0x5514c: strb r0, [r4, #0x3c9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,969)?)?;
    // 0x55150: ldrb r0, [r4, #0xe72]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3698)?)?;
    // 0x55154: strb r0, [r4, #0x3ca]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,970)?)?;
    // 0x55158: ldr r0, [r4, #0xd28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3368)?)?;
    // 0x5515c: str r0, [r4, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,640)?)?;
    // 0x55160: ldr r0, [r4, #0xd3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3388)?)?;
    // 0x55164: str r0, [r4, #0x294]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,660)?)?;
    // 0x55168: ldr r0, [r4, #0xd40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3392)?)?;
    // 0x5516c: str r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,664)?)?;
    // 0x55170: ldr r0, [r4, #0xd44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3396)?)?;
    // 0x55174: str r0, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,668)?)?;
    // 0x55178: ldr r0, [r4, #0xe48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3656)?)?;
    // 0x5517c: str r0, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,928)?)?;
    // 0x55180: str r0, [r4, #0x3a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,932)?)?;
    // 0x55184: ldr r0, [r4, #0xe50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3664)?)?;
    // 0x55188: str r0, [r4, #0x3a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,936)?)?;
    // 0x5518c: ldr r0, [r4, #0xe54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3668)?)?;
    // 0x55190: str r0, [r4, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,940)?)?;
    // 0x55194: ldr r0, [r4, #0xe58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3672)?)?;
    // 0x55198: str r0, [r4, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,944)?)?;
    // 0x5519c: ldr r0, [r4, #0xe5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3676)?)?;
    // 0x551a0: str r0, [r4, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,948)?)?;
    // 0x551a4: ldrb r0, [r4, #0xe60]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3680)?)?;
    // 0x551a8: strb r0, [r4, #0x3b8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,952)?)?;
    // 0x551ac: ldrb r0, [r4, #0xe62]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3682)?)?;
    // 0x551b0: strb r0, [r4, #0x3ba]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,954)?)?;
    // 0x551b4: ldrb r0, [r4, #0xe61]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3681)?)?;
    // 0x551b8: strb r0, [r4, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,953)?)?;
    // 0x551bc: ldrb r0, [r4, #0xe73]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3699)?)?;
    // 0x551c0: strb r0, [r4, #0x3cb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,971)?)?;
    // 0x551c4: ldrb r0, [r4, #0xde2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3554)?)?;
    // 0x551c8: strb r0, [r4, #0x33a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,826)?)?;
    // 0x551cc: ldrb r0, [r4, #0xde3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3555)?)?;
    // 0x551d0: strb r0, [r4, #0x33b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,827)?)?;
    // 0x551d4: ldrb r0, [r4, #0xe63]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3683)?)?;
    // 0x551d8: strb r0, [r4, #0x3bb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,955)?)?;
    // 0x551dc: ldrb r0, [r4, #0xe65]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3685)?)?;
    // 0x551e0: strb r0, [r4, #0x3bd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,957)?)?;
    // 0x551e4: ldrb r0, [r4, #0xe64]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3684)?)?;
    // 0x551e8: strb r0, [r4, #0x3bc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,956)?)?;
    // 0x551ec: ldrb r0, [r4, #0xe66]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3686)?)?;
    // 0x551f0: strb r0, [r4, #0x3be]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,958)?)?;
    // 0x551f4: mov r0, #1008
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1008)?)?;
    // 0x551f8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x551fc: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55200: strb r0, [r4, #0x41c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1052)?)?;
    // 0x55204: ldr r0, [r4, #0xd50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3408)?)?;
    // 0x55208: str r0, [r4, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,680)?)?;
    // 0x5520c: ldr r0, [r4, #0xd48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3400)?)?;
    // 0x55210: str r0, [r4, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,672)?)?;
    // 0x55214: ldr r0, [r4, #0xd4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3404)?)?;
    // 0x55218: str r0, [r4, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,676)?)?;
    // 0x5521c: ldr r0, [r4, #0xd74]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3444)?)?;
    // 0x55220: str r0, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,716)?)?;
    // 0x55224: ldr r0, [r4, #0xd78]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3448)?)?;
    // 0x55228: str r0, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,720)?)?;
    // 0x5522c: ldrb r0, [r4, #0xe91]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3729)?)?;
    // 0x55230: strb r0, [r4, #0x3e9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1001)?)?;
    // 0x55234: ldrb r0, [r4, #0xe85]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3717)?)?;
    // 0x55238: strb r0, [r4, #0x3dd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,989)?)?;
    // 0x5523c: ldrb r0, [r4, #0xe86]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3718)?)?;
    // 0x55240: strb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,990)?)?;
    // 0x55244: ldr r0, [r4, #0xdb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3508)?)?;
    // 0x55248: str r0, [r4, #0x30c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,780)?)?;
    // 0x5524c: ldr r0, [r4, #0xdb8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3512)?)?;
    // 0x55250: str r0, [r4, #0x310]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,784)?)?;
    // 0x55254: ldr r0, [r4, #0xd54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3412)?)?;
    // 0x55258: str r0, [r4, #0x2ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,684)?)?;
    // 0x5525c: ldr r0, [r4, #0xd58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3416)?)?;
    // 0x55260: str r0, [r4, #0x2b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,688)?)?;
    // 0x55264: ldr r0, [r4, #0xd5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3420)?)?;
    // 0x55268: str r0, [r4, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,692)?)?;
    // 0x5526c: ldr r0, [r4, #0xd60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3424)?)?;
    // 0x55270: str r0, [r4, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,696)?)?;
    // 0x55274: ldr r0, [r4, #0xe44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3652)?)?;
    // 0x55278: str r0, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,924)?)?;
    // 0x5527c: ldr r0, [r4, #0xab0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2736)?)?;
    // 0x55280: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55284: beq 0x55320 <entry+0x19dc>  @ imm = #0x94
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55320,false)?)?;
    // 0x55288: ldr r0, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,0)?)?;
    // 0x5528c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x55290: bne 0x55320 <entry+0x19dc>  @ imm = #0x88
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55320,false)?)?;
    // 0x55294: ldrb r0, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,988)?)?;
    // 0x55298: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5529c: beq 0x55320 <entry+0x19dc>  @ imm = #0x7c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55320,false)?)?;
    // 0x552a0: mov r0, #412
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,412)?)?;
    // 0x552a4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x552a8: orr r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // 0x552ac: ldrh r0, [r1, r0]!
    c.emit(transfer_halfword_register(C::Al,true,R::R0,R::R1,R::R0,true)?)?;
    // 0x552b0: add r0, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,12)?)?;
    // 0x552b4: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x552b8: mov r0, #158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,158)?)?;
    // 0x552bc: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x552c0: orr r0, r0, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3328)?)?;
    // 0x552c4: ldrh r0, [r1, r0]!
    c.emit(transfer_halfword_register(C::Al,true,R::R0,R::R1,R::R0,true)?)?;
    // 0x552c8: add r0, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,12)?)?;
    // 0x552cc: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x552d0: mov r0, #3488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3488)?)?;
    // 0x552d4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x552d8: ldrh r0, [r1, r0]!
    c.emit(transfer_halfword_register(C::Al,true,R::R0,R::R1,R::R0,true)?)?;
    // 0x552dc: add r0, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,12)?)?;
    // 0x552e0: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x552e4: ldrb r0, [r1, #0x1d7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,471)?)?;
    // 0x552e8: add r0, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,12)?)?;
    // 0x552ec: strb r0, [r1, #0x1d7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,471)?)?;
    // 0x552f0: add r0, r1, #560
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,560)?)?;
    // 0x552f4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x552f8: ldrb r2, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,3)?)?;
    // 0x552fc: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x55300: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x55304: add r2, r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,12)?)?;
    // 0x55308: strb r2, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,3)?)?;
    // 0x5530c: ldrb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x55310: add r2, r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,12)?)?;
    // 0x55314: strb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x55318: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5531c: bne 0x552f8 <entry+0x19b4>  @ imm = #-0x2c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x552f8,false)?)?;
    // 0x55320: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x55324: ldrb r9, [r6, #0xd9c]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::new(9)?,R::new(6)?,3484)?)?;
    // 0x55328: str r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // 0x5532c: ldr r8, [pc, #0xfd0]        @ 0x29c0 <entry+0x29c0>
    c.literal(C::Al,R::new(8)?,0x56304)?;
    // 0x55330: strh r9, [r6, r8]
    c.emit(transfer_halfword_register(C::Al,false,R::new(9)?,R::new(6)?,R::new(8)?,false)?)?;
    // 0x55334: ldrb r0, [r6, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,2)?)?;
    // 0x55338: ldr r2, [pc, #0xfc8]        @ 0x29c4 <entry+0x29c4>
    c.literal(C::Al,R::R2,0x56308)?;
    // 0x5533c: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x55340: strh r0, [r6, r2]
    c.emit(transfer_halfword_register(C::Al,false,R::R0,R::new(6)?,R::R2,false)?)?;
    // 0x55344: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x55348: ldrb r0, [r6, #0x4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // 0x5534c: ldr r7, [pc, #0xfb8]        @ 0x29c8 <entry+0x29c8>
    c.literal(C::Al,R::new(7)?,0x5630c)?;
    // 0x55350: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x55354: strh r0, [r6, r7]
    c.emit(transfer_halfword_register(C::Al,false,R::R0,R::new(6)?,R::new(7)?,false)?)?;
    // 0x55358: add r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x5535c: ldrb r0, [r6, #0x6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,6)?)?;
    // 0x55360: strb r0, [r6, #-0xaa2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,-2722)?)?;
    // 0x55364: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // 0x55368: str r0, [r6, #-0xa98]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,-2712)?)?;
    // 0x5536c: ldr r0, [r6, #0x690]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1680)?)?;
    // 0x55370: str r0, [r6, #-0x944]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,-2372)?)?;
    // 0x55374: ldr r0, [r6, #0x694]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1684)?)?;
    // 0x55378: str r0, [r6, #-0x940]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,-2368)?)?;
    // 0x5537c: ldr r0, [r6, #0x698]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1688)?)?;
    // 0x55380: str r0, [r6, #-0x93c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,-2364)?)?;
    // 0x55384: ldrb r1, [r6, #0x234]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,564)?)?;
    // 0x55388: ldrb r12, [r6, #0x237]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(6)?,567)?)?;
    // 0x5538c: ldrb r10, [r6, #0x238]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(10)?,R::new(6)?,568)?)?;
    // 0x55390: ldrb lr, [r6, #0x239]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(6)?,569)?)?;
    // 0x55394: strb r1, [r6, #-0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,-2454)?)?;
    // 0x55398: ldrb r0, [r6, #0x235]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,565)?)?;
    // 0x5539c: strb r0, [r6, #-0x995]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,-2453)?)?;
    // 0x553a0: ldrb r3, [r6, #0x236]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,566)?)?;
    // 0x553a4: strb r3, [r6, #-0x994]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(6)?,-2452)?)?;
    // 0x553a8: ldrb r5, [r6, #0x5c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,92)?)?;
    // 0x553ac: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x553b0: movne r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x553b4: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x553b8: moveq r12, r5
    c.emit(arm32::mov_register(C::Eq,R::R12,R::new(5)?))?;
    // 0x553bc: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x553c0: moveq r10, r5
    c.emit(arm32::mov_register(C::Eq,R::new(10)?,R::new(5)?))?;
    // 0x553c4: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x553c8: strb r12, [r6, #-0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(6)?,-2457)?)?;
    // 0x553cc: movne r5, lr
    c.emit(arm32::mov_register(C::Ne,R::new(5)?,R::LR))?;
    // 0x553d0: strb r10, [r6, #-0x998]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(6)?,-2456)?)?;
    // 0x553d4: add lr, r6, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x553d8: strb r5, [r6, #-0x997]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(6)?,-2455)?)?;
    // 0x553dc: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x553e0: str r7, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,28)?)?;
    // 0x553e4: ldr r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // 0x553e8: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x553ec: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x553f0: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x553f4: bne 0x55438 <entry+0x1af4>  @ imm = #0x3c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55438,false)?)?;
    // 0x553f8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x553fc: mov r8, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,127)?)?;

 c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();configure_controller_resource_policy(&mut c)?;
 encode_controller_property_fields(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn checked_indexed_halfword_modes(){
   assert_eq!(transfer_halfword_register(C::Al,true,R::R0,R::R1,R::R0,true).unwrap(),0xe1b1_00b0);
   assert_eq!(transfer_halfword_register(C::Al,false,R::new(9).unwrap(),R::new(6).unwrap(),R::new(8).unwrap(),false).unwrap(),0xe186_90b8);
   assert!(transfer_halfword_register(C::Al,true,R::R0,R::R0,R::R1,true).is_err());
   assert!(transfer_halfword_register(C::Al,false,R::R1,R::PC,R::R2,false).is_err());
 }
 #[test]fn partial_codec_policy_is_adjacent(){assert_eq!(fragment().unwrap().len(),END-START);}
 #[test]fn compare_private_firmware_and_emit_when_requested(){
   let b=fragment().unwrap();
   if let Ok(path)=std::env::var("AGENT04_PRIVATE_VENUS"){
     let fw=std::fs::read(path).unwrap();let original=&fw[FILE_OFFSET..FILE_OFFSET+b.len()];
     if let Some(i)=b.iter().zip(original).position(|(a,c)|a!=c){panic!("byte mismatch at runtime {:#x}: {:02x} != {:02x}",START+i,b[i],original[i]);}
   }
   if let Ok(path)=std::env::var("AGENT04_EMIT_FRAGMENT"){std::fs::write(path,b).unwrap();}
 }
}
