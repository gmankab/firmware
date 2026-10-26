// SPDX-License-Identifier: GPL-3.0-only
//! Start of Venus codec-format controller command path, runtime
//! [0x53000,0x53400). The first function returns at 0x53250, followed by
//! four literal words with different semantics; next function continues.
//! Executable ARM32 instructions generated from typed architectural operands.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x53000;
pub const END:usize=0x53400;
const CONTROLLER_RETURN_PTR:u32=0x00053264;
const CONTROLLER_GLOBAL:u32=0x0010ceac;
const CODEC_FRAME_FORMAT_A:u32=0x0401100d;
const CODEC_FRAME_FORMAT_B:u32=0x04021009;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,x:usize)->Result<(),String>{if x!=self.at{return Err(format!("Agent04 A32 {:#x} expected {x:#x}",self.at));}Ok(())}
 #[inline(never)]fn emit(&mut self,w:u32)->Result<(),String>{
  if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0{return Err("Agent04 range or alignment".into())}
  self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
 }
 fn literal(&mut self,cond:C,rd:R,target:usize)->Result<(),String>{
  let d=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"literal target overflow")?;
  self.emit(arm32::transfer_word(cond,true,rd,R::PC,d)?)
 }
}

/// Reconstruct controller code in [0x53000,0x53264).
#[inline(never)]fn finalize_format_output(c:&mut Code)->Result<(),String>{
 c.check(0x53000)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,68)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,332)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,31)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,11264)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,8)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,404)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,10240)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(9)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,52)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,4026531841)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
        c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsr,4,false)?)?;
        c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsr,4,false)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,348)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
        c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R2,Shift::Lsl,4,false)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,5,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2400)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,636)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,60)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,3140)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(7)?,R::new(3)?,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,9856)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(7)?,R::new(3)?,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,652)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9216)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,248)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2288)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(8)?,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,3144)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,364)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,388)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,88)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,40)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,392)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(9)?,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,36)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x1a1ec,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,76)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,80)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,40)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x52984,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,552)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,15360)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3600)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
        c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3105)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
        c.emit(arm32::branch(C::Lt,c.at as u32,0x531cc,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,520)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsr,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x531bc,false)?)?;
        c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(9)?,3105)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,1)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(9)?,R::new(6)?,Shift::Lsl,2,false)?)?;
        c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(9)?,3105)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2976)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
        c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::branch(C::Lt,c.at as u32,0x5319c,false)?)?;
        c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,3104)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x53248,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,564)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,15360)?)?;
        c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,3105)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,540)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,15360)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
        c.emit(arm32::branch(C::Lt,c.at as u32,0x53248,false)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,38400)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x53238,false)?)?;
        c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,3105)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,1)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(9)?,R::new(7)?,Shift::Lsl,2,false)?)?;
        c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,3105)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,2976)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(3)?,255)?)?;
        c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::branch(C::Hi,c.at as u32,0x53218,false)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2080,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
        c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(CONTROLLER_RETURN_PTR)?;
    c.emit(CONTROLLER_GLOBAL)?;
    c.emit(CODEC_FRAME_FORMAT_A)?;
    c.emit(CODEC_FRAME_FORMAT_B)?;
 c.check(0x53264)?;
 Ok(())
}

/// Reconstruct controller code in [0x53264,0x53400).
#[inline(never)]fn start_indexed_codec_controller(c:&mut Code)->Result<(),String>{
 c.check(0x53264)?;
        c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,40)?)?;
        c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,692)?)?;
        c.literal(C::Al,R::R1,0x53708)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3760)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R1,24)?)?;
        c.literal(C::Al,R::R1,0x5370c)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(7)?,92)?)?;
        c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x53324,false)?)?;
        c.literal(C::Al,R::R1,0x53710)?;
        c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x53630,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,644)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7808)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
        c.literal(C::Al,R::new(3)?,0x53714)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,4,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,168)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
        c.literal(C::Al,R::R1,0x53734)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
        c.literal(C::Al,R::R1,0x5371c)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,696)?)?;
        c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
        c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x53420,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,2744)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,400)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,404)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x53438,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,652)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,648)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
        c.literal(C::Al,R::new(3)?,0x53714)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,4,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,168)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
        c.literal(C::Al,R::R1,0x53718)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
        c.literal(C::Al,R::new(5)?,0x5371c)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,696)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
        c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.literal(C::Al,R::R1,0x53720)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
        c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,192)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
        c.literal(C::Al,R::R1,0x5372c)?;
        c.literal(C::Al,R::R0,0x53724)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,200)?)?;
        c.literal(C::Al,R::R0,0x53728)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,204)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
        c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x39b9c,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,-168)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,140)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x7fd1c,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,280)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x53638,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,16)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,276)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,256)?)?;
 c.check(0x53400)?;
 Ok(())
}

pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();finalize_format_output(&mut c)?;start_indexed_codec_controller(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn format_output_retains_literal_pool(){
  let b=fragment().unwrap();assert_eq!(b.len(),END-START);
  assert_eq!(u32::from_le_bytes(b[596..600].try_into().unwrap()),CONTROLLER_RETURN_PTR);
  assert_eq!(u32::from_le_bytes(b[600..604].try_into().unwrap()),CONTROLLER_GLOBAL);
  assert_eq!(u32::from_le_bytes(b[604..608].try_into().unwrap()),CODEC_FRAME_FORMAT_A);
  assert_eq!(u32::from_le_bytes(b[608..612].try_into().unwrap()),CODEC_FRAME_FORMAT_B);
 }
}
