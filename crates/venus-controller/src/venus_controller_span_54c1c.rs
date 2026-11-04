// SPDX-License-Identifier: GPL-3.0-only
//! Map codec input source modes into a compact set of hardware controller
//! status bits; normalize the following small codec boolean state fields.
//! 4 addressed words following the first genuine function return are data.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x54c1c;
pub const END:usize=0x54cfc;
pub const FILE_OFFSET:usize=START+0x4000;
const MODE_ISSUE:u32=0x000e_e470;
const MODE_A:u32=0x000f_16e0;
const MODE_B:u32=0x000f_16a0;
const MODE_C:u32=0x000f_1660;
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,addr:usize)->Result<(),String>{
   if self.at!=addr{return Err(format!("Venus controller at 0x{:x} != 0x{addr:x}",self.at));}Ok(())
 }
 #[inline(never)]fn emit(&mut self,w:u32)->Result<(),String>{
   if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0{return Err("outside Agent04 owned range".into());}
   self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
 }
 fn literal(&mut self,cond:C,rd:R,target:usize)->Result<(),String>{
   let off=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"invalid PC-relative pointer")?;
   self.emit(arm32::transfer_word(cond,true,rd,R::PC,off)?)
 }
}
#[inline(never)]fn install_codec_mode_flags(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x54c1c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x54c20: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x54c24: sub r6, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R1,1)?)?;
    // 0x54c28: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x54c2c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x54c30: cmp r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,4)?)?;
    // 0x54c34: blo 0x54c50 <entry+0x130c>  @ imm = #0x14
    c.emit(arm32::branch(C::Cc,c.at as u32,0x54c50,false)?)?;
    // 0x54c38: ldr r2, [pc, #0x7c]         @ 0x1378 <entry+0x1378>
    c.literal(C::Al,R::R2,0x54cbc)?;
    // 0x54c3c: mov r1, #584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,584)?)?;
    // 0x54c40: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x54c44: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x54c48: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x54c4c: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x534a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x54c50: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x54c54: tst r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,4)?)?;
    // 0x54c58: strb r6, [r4, #0x2b7]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,695)?)?;
    // 0x54c5c: and r0, r0, r5, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(5)?,Shift::Lsl,1,false)?)?;
    // 0x54c60: orr r1, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x54c64: addne r1, r1, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,8)?)?;
    // 0x54c68: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // 0x54c6c: bhs 0x54c8c <entry+0x1348>  @ imm = #0x18
    c.emit(arm32::branch(C::Cs,c.at as u32,0x54c8c,false)?)?;
    // 0x54c70: ldr r0, [pc, #0x48]         @ 0x137c <entry+0x137c>
    c.literal(C::Al,R::R0,0x54cc0)?;
    // 0x54c74: ldr r2, [pc, #0x48]         @ 0x1380 <entry+0x1380>
    c.literal(C::Al,R::R2,0x54cc4)?;
    // 0x54c78: ldr r3, [pc, #0x48]         @ 0x1384 <entry+0x1384>
    c.literal(C::Al,R::new(3)?,0x54cc8)?;
    // 0x54c7c: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x54c80: ldr r2, [r2, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R1,Shift::Lsl,2)?)?;
    // 0x54c84: ldr r5, [r3, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(3)?,R::R1,Shift::Lsl,2)?)?;
    // 0x54c88: b 0x54cac <entry+0x1368>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x54cac,false)?)?;
    // 0x54c8c: ldr r2, [pc, #0x28]         @ 0x1378 <entry+0x1378>
    c.literal(C::Al,R::R2,0x54cbc)?;
    // 0x54c90: mov r1, #135
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,135)?)?;
    // 0x54c94: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x54c98: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x54c9c: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x54ca0: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x534f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x54ca4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x54ca8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x54cac: orr r1, r2, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x54cb0: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x54cb4: strb r0, [r4, #0x2c0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,704)?)?;
    // 0x54cb8: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

 c.check(0x54cbc)?;
 for word in [MODE_ISSUE,MODE_A,MODE_B,MODE_C] {c.emit(word)?;}
 c.check(0x54ccc)
}
#[inline(never)]fn clear_codec_mode_state(c:&mut Code)->Result<(),String>{
 c.check(0x54ccc)?;
    // 0x54ccc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x54cd0: strb r1, [r0, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x54cd4: ldrb r2, [r0, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,990)?)?;
    // 0x54cd8: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x54cdc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x54ce0: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x54ce4: strb r2, [r0, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,13)?)?;
    // 0x54ce8: ldr r2, [r0, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,280)?)?;
    // 0x54cec: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x54cf0: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x54cf4: strb r1, [r0, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,14)?)?;
    // 0x54cf8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;

 c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new(); install_codec_mode_flags(&mut c)?;
 clear_codec_mode_state(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn mode_tables_are_explicit(){
  let b=fragment().unwrap();assert_eq!(b.len(),END-START);
  for (pos,v) in [(0x54cbc,MODE_ISSUE),(0x54cc0,MODE_A),(0x54cc4,MODE_B),(0x54cc8,MODE_C)]{
   let i=pos-START;assert_eq!(u32::from_le_bytes(b[i..i+4].try_into().unwrap()),v);
  }
 }
 #[test]fn private_reference_matches_if_provided(){
  let b=fragment().unwrap();
  if let Ok(path)=std::env::var("NABU_VENUS_REFERENCE").or_else(|_| std::env::var("AGENT04_PRIVATE_VENUS")){
   let image=std::fs::read(path).unwrap();let refb=&image[FILE_OFFSET..FILE_OFFSET+b.len()];
   if let Some(i)=b.iter().zip(refb).position(|(a,c)|a!=c){panic!("A32 mismatch at 0x{:x}",START+i);}
  }
  if let Ok(out)=std::env::var("AGENT04_EMIT_FRAGMENT"){std::fs::write(out,b).unwrap()}
 }
}
