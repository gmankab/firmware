// SPDX-License-Identifier: GPL-3.0-only
//! Completion of the indexed codec controller and source-plane status scan.
//! ARM A32 architectural encoders generate executable instructions; literal
//! pools after function returns remain separate named data values.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x53400;
pub const END:usize=0x53944;
const CONTROLLER_GLOBAL:u32=0x0010ceac;
const MODE_A:u32=0x0401100d;
const MODE_B:u32=0x0401100f;
const HANDLER_OFFSET:u32=0x00105894;
const CODEC_LOG_A:u32=0x000ee38b;
const LOG_CONTEXT:u32=0x000e2788;
const TABLE_DISP_A:u32=0xffffea8c;
const TABLE_DISP_B:u32=0xffffea94;
const TABLE_DISP_C:u32=0xffffea98;
const CODEC_LOG_B:u32=0x000ee3bd;
const CODEC_LOG_C:u32=0x000ee028;
const CODEC_LOG_D:u32=0x000ee358;
const RECIPROCAL_TEN:u32=0xcccccccd;
const PLANE_GLOBAL:u32=0x0010ceac;
const PLANE_RECIPROCAL:u32=0xcccccccd;
const PLANE_FORMAT:u32=0x0401100f;

fn unsigned_long_multiply(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe ARM32 UMULL operands".into())}
 Ok(((cond as u32)<<28)|0x0080_0090|(hi.number()<<16)|
   (lo.number()<<12)|(rs.number()<<8)|rm.number())
}
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,x:usize)->Result<(),String>{if x!=self.at{return Err(format!("Agent04 A32 {:#x} expected {x:#x}",self.at));}Ok(())}
 #[inline(never)]fn emit(&mut self,w:u32)->Result<(),String>{
  if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0{return Err("Agent04 range/alignment".into())}
  self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
 }
 fn literal(&mut self,cond:C,rd:R,target:usize)->Result<(),String>{
  let d=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"literal target overflow")?;
  self.emit(arm32::transfer_word(cond,true,rd,R::PC,d)?)
 }
}

/// Controller logic and separate data pools in [0x53400,0x5373c).
#[inline(never)]fn publish_indexed_codec_state(c:&mut Code)->Result<(),String>{
 c.check(0x53400)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x80928,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31)?)?;
        c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15616)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x53628,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2704)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,3260)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2708)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2712)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,3268)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,3264)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,176)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
        c.emit(arm32::transfer_doubleword(C::Al,false,R::R2,R::new(6)?,184)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
        c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,208)?)?;
        c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,3205)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x534d4,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2720)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
        c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,16)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2724)?)?;
        c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,16)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2728)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,16)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
        c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,16)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
        c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,3206)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,3248)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,168)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,140)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x53944,true)?)?;
        c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,208)?)?;
        c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,3104)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x53538,false)?)?;
        c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x39b9c,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3600)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,140)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x7fd1c,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,280)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x53638,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,16)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,276)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,256)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x80928,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,251)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x53620,false)?)?;
        c.literal(C::Al,R::R0,0x5370c)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,168)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x535cc,false)?)?;
        c.literal(C::Al,R::R1,0x53738)?;
        c.emit(unsigned_long_multiply(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(3)?,Shift::Lsr,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(7)?,3120)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,2,false)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R0,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
        c.emit(unsigned_long_multiply(C::Al,R::new(6)?,R::new(5)?,R::R0,R::R1)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(5)?,Shift::Lsr,2)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,108)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(3)?,R::R0,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1600)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3616)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3240)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,3040)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3224)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x5364c,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,3044)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,108)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,18)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x53650,false)?)?;
        c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x39b9c,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3600)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,140)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x7fd1c,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,280)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x53638,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,16)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,276)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,256)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x80928,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,179)?)?;
        c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
        c.literal(C::Al,R::R2,0x53730)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
        c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,44)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
        c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x292c,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1600)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2200)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::R1,Shift::Lsl,2,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,2976)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(3)?,Shift::Lsl,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2184)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
        c.emit(arm32::data_register(C::Ne,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,2,false)?)?;
        c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(4)?,1600)?)?;
        c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::R2,2976)?)?;
        c.emit(arm32::transfer_word_register(C::Ne,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R1,2)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R2,Shift::Lsl,2,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,2412)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1660)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1676)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,2976)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,2980)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,2984)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,2988)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,2992)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,3040)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,2992)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,2,false)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2014,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,225)?)?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
        c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
        c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
        c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
        c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0xd790,false)?)?;
    c.emit(CONTROLLER_GLOBAL)?;
    c.emit(MODE_A)?;
    c.emit(MODE_B)?;
    c.emit(HANDLER_OFFSET)?;
    c.emit(CODEC_LOG_A)?;
    c.emit(LOG_CONTEXT)?;
    c.emit(TABLE_DISP_A)?;
    c.emit(TABLE_DISP_B)?;
    c.emit(TABLE_DISP_C)?;
    c.emit(CODEC_LOG_B)?;
    c.emit(CODEC_LOG_C)?;
    c.emit(CODEC_LOG_D)?;
    c.emit(RECIPROCAL_TEN)?;
 c.check(0x5373c)?;
 Ok(())
}

/// Controller logic and separate data pools in [0x5373c,0x53944).
#[inline(never)]fn scan_codec_plane_counts(c:&mut Code)->Result<(),String>{
 c.check(0x5373c)?;
        c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
        c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,108)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x53930,false)?)?;
        c.literal(C::Al,R::R0,0x53938)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,92)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,24)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,225)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4864)?)?;
        c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
        c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
        c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x39b10,true)?)?;
        c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
        c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::SP,24)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
        c.emit(arm32::branch(C::Ne,c.at as u32,0x53920,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,140)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,20)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,16)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x7f974,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,16)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x53930,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,532)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,1312)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,52)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,2976)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,15360)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(3)?,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,954)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x53804,false)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,2772)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,2980)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x53808,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,608)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(6)?,R::R2,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
        c.emit(arm32::branch(C::Eq,c.at as u32,0x53884,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,312)?)?;
        c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
        c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(9)?,168)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,11264)?)?;
        c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R12,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
        c.literal(C::Al,R::new(3)?,0x5393c)?;
        c.emit(unsigned_long_multiply(C::Al,R::new(4)?,R::new(5)?,R::LR,R::new(3)?)?)?;
        c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(5)?,Shift::Lsr,2)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,2,false)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(8)?,2976)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(8)?,R::new(3)?,Shift::Lsl,2,false)?)?;
        c.emit(arm32::mov_register(C::Al,R::new(4)?,R::LR))?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,3120)?)?;
        c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(5)?,R::new(4)?,R::R1,Shift::Lsl,2)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R12,R::R0,R::new(5)?,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,4)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(5)?,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,8)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::LR,R::R1,Shift::Lsl,2)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,2)?)?;
        c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(6)?,8192)?)?;
        c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::new(4)?,2976)?)?;
        c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,2)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,2,false)?)?;
        c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
        c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x2014,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,-232)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,328)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
        c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,336)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,692)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,15360)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::new(9)?,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
        c.literal(C::Al,R::R0,0x53940)?;
        c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
        c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
        c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,8)?)?;
        c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x39b9c,true)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
        c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
        c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
        c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,2)?)?;
        c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
        c.emit(arm32::branch(C::Al,c.at as u32,0x1b164,true)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,24)?)?;
        c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
        c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
        c.emit(arm32::branch(C::Cc,c.at as u32,0x53784,false)?)?;
        c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
        c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(PLANE_GLOBAL)?;
    c.emit(PLANE_RECIPROCAL)?;
    c.emit(PLANE_FORMAT)?;
 c.check(0x53944)?;
 Ok(())
}

pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();publish_indexed_codec_state(&mut c)?;scan_codec_plane_counts(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn indexed_pool_and_reciprocals_have_fixed_locations(){
  let b=fragment().unwrap();assert_eq!(b.len(),END-START);
  assert_eq!(u32::from_le_bytes(b[776..780].try_into().unwrap()),CONTROLLER_GLOBAL);
  assert_eq!(u32::from_le_bytes(b[780..784].try_into().unwrap()),MODE_A);
  assert_eq!(u32::from_le_bytes(b[784..788].try_into().unwrap()),MODE_B);
  assert_eq!(u32::from_le_bytes(b[788..792].try_into().unwrap()),HANDLER_OFFSET);
  assert_eq!(u32::from_le_bytes(b[792..796].try_into().unwrap()),CODEC_LOG_A);
  assert_eq!(u32::from_le_bytes(b[796..800].try_into().unwrap()),LOG_CONTEXT);
  assert_eq!(u32::from_le_bytes(b[800..804].try_into().unwrap()),TABLE_DISP_A);
  assert_eq!(u32::from_le_bytes(b[804..808].try_into().unwrap()),TABLE_DISP_B);
  assert_eq!(u32::from_le_bytes(b[808..812].try_into().unwrap()),TABLE_DISP_C);
  assert_eq!(u32::from_le_bytes(b[812..816].try_into().unwrap()),CODEC_LOG_B);
  assert_eq!(u32::from_le_bytes(b[816..820].try_into().unwrap()),CODEC_LOG_C);
  assert_eq!(u32::from_le_bytes(b[820..824].try_into().unwrap()),CODEC_LOG_D);
  assert_eq!(u32::from_le_bytes(b[824..828].try_into().unwrap()),RECIPROCAL_TEN);
  assert_eq!(u32::from_le_bytes(b[1336..1340].try_into().unwrap()),PLANE_GLOBAL);
  assert_eq!(u32::from_le_bytes(b[1340..1344].try_into().unwrap()),PLANE_RECIPROCAL);
  assert_eq!(u32::from_le_bytes(b[1344..1348].try_into().unwrap()),PLANE_FORMAT);
 }
 #[test]fn umull_registers_encode_distinct_result_halves(){
  assert_eq!(unsigned_long_multiply(C::Al,R::R2,R::new(3).unwrap(),R::R0,R::R1).unwrap(),0xe083_2190);
  assert!(unsigned_long_multiply(C::Al,R::R2,R::R2,R::R0,R::R1).is_err());
 }
}
