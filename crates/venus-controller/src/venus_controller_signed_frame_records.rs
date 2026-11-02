// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller record packing, signed halfword buffer checks and
//! frame-completion callbacks. The inline six-word diagnostic/capability
//! table at 0x4dd54–0x4dd6c is PC referenced and follows a real function
//! return; it is not executed as ARM instructions. All executable code is
//! recreated from typed ARM32 operand encoders.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
pub const START:usize=0x4d000;
pub const END:usize=0x4e000;
const EMBEDDED_LOG_POINTER:u32=0x000e_e028;
const CONTROLLER_DIAGNOSTICS:[u32;6]=[
 0x000e_e028,0x000e_e45a,0x0003_ffff,
 0x000e_e41c,0x000e_e3f0,0x000e_e408,
];
/// ADC of zero adds the carry flag to a word; avoid misleading ADD syntax.
/// This A32 immediate-zero form has the unrotated imm8 field equal to zero.
fn add_with_carry_zero(cond:C,rd:R,rn:R)->Result<u32,String>{
 if rd==R::PC || rn==R::PC {return Err("invalid ADC-zero register".into());}
 Ok(((cond as u32)<<28)|0x02a0_0000|(rn.number()<<16)|(rd.number()<<12))
}
/// Immediate LDRSH post-indexed; signed halfword and base are distinct.
fn load_signed_halfword_postindexed(cond:C,rd:R,base:R,n:i32)->Result<u32,String>{
 if [rd,base].contains(&R::PC)||rd==base||n.unsigned_abs()>255{
  return Err("invalid LDRSH postindexed operand".into());
 }
 let m=n.unsigned_abs();Ok(((cond as u32)<<28)|0x0050_00f0|
  if n>=0{1<<23}else{0}|(base.number()<<16)|(rd.number()<<12)|((m&0xf0)<<4)|(m&0x0f))
}
/// Immediate LDRH post-indexed; useful for the original 16-bit size walker.
fn load_halfword_postindexed(cond:C,rd:R,base:R,n:i32)->Result<u32,String>{
 if [rd,base].contains(&R::PC)||rd==base||n.unsigned_abs()>255{
  return Err("invalid LDRH postindexed operand".into());
 }
 let m=n.unsigned_abs();Ok(((cond as u32)<<28)|0x0050_00b0|
  if n>=0{1<<23}else{0}|(base.number()<<16)|(rd.number()<<12)|((m&0xf0)<<4)|(m&0x0f))
}
struct Code {at:usize,bytes:Vec<u8>}
impl Code {
 fn new()->Self {Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,a:usize)->Result<(),String>{
  if self.at!=a{return Err(format!("Venus controller A32 at {:#x} expected {a:#x}",self.at));}Ok(())
 }
 #[inline(never)]
 fn emit(&mut self,w:u32)->Result<(),String>{
  if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0 {return Err("outside Agent 04 range".into());}
  self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
 }
 fn branch(&mut self,c:C,target:usize)->Result<(),String>{
  self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
 }
 fn literal(&mut self,c:C,rd:R,target:usize)->Result<(),String>{
  let d=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"literal displacement overflow")?;
  self.emit(arm32::transfer_word(c,true,rd,R::PC,d)?)
 }
}
fn serialize_signed_frame_records(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x4d000: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4d004: lsl r0, lr, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::LR,Shift::Lsl,R::R0,false)?)?;
    // 0x4d008: blx 0x83c4c <entry+0x37c4c> @ imm = #0x36c3c
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x4d00c: ldr r5, [pc, #0xd48]        @ 0x1d5c <entry+0x1d5c>
    c.literal(C::Al,R::new(5)?,0x4dd5c)?;
    // 0x4d010: mov r10, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1048576)?)?;
    // 0x4d014: tst r0, r5
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4d018: add r6, r5, #1835008
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(5)?,1835008)?)?;
    // 0x4d01c: addne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4d020: and r1, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4d024: cmp r1, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1048576)?)?;
    // 0x4d028: movhs r1, r10
    c.emit(arm32::mov_register(C::Cs,R::R1,R::new(10)?))?;
    // 0x4d02c: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x4d030: ldreq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,0)?)?;
    // 0x4d034: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x4d038: streq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,0)?)?;
    // 0x4d03c: mov r0, #2080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2080)?)?;
    // 0x4d040: add r4, r7, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,40)?)?;
    // 0x4d044: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x4d048: str r0, [r7, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,32)?)?;
    // 0x4d04c: add r0, r8, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,5)?)?;
    // 0x4d050: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x4d054: str r1, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,36)?)?;
    // 0x4d058: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4d05c: str r4, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,0)?)?;
    // 0x4d060: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x4d064: ldr r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
    // 0x4d068: add r0, r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,18)?)?;
    // 0x4d06c: sub r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d070: ldr r1, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,96)?)?;
    // 0x4d074: lsl r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x4d078: ldr r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // 0x4d07c: blx 0x83c4c <entry+0x37c4c> @ imm = #0x36bc8
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x4d080: tst r0, r5
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4d084: mov r3, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,134)?)?;
    // 0x4d088: mov r2, #392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,392)?)?;
    // 0x4d08c: addne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4d090: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x4d094: and r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4d098: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x4d09c: cmp r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1048576)?)?;
    // 0x4d0a0: orr r6, r6, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,768)?)?;
    // 0x4d0a4: movlo r10, r0
    c.emit(arm32::mov_register(C::Cc,R::new(10)?,R::R0))?;
    // 0x4d0a8: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x4d0ac: ldreq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,0)?)?;
    // 0x4d0b0: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x4d0b4: streq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,0)?)?;
    // 0x4d0b8: mov r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // 0x4d0bc: orr r0, r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    // 0x4d0c0: str r0, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,40)?)?;
    // 0x4d0c4: add r0, r8, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,6)?)?;
    // 0x4d0c8: str r10, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,44)?)?;
    // 0x4d0cc: str r10, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,44)?)?;
    // 0x4d0d0: add r10, r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(7)?,64)?)?;
    // 0x4d0d4: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4d0d8: add r0, r7, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,48)?)?;
    // 0x4d0dc: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x4d0e0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4d0e4: ldr r5, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,104)?)?;
    // 0x4d0e8: ldr r4, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,76)?)?;
    // 0x4d0ec: ldr r1, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2744)?)?;
    // 0x4d0f0: ldrh r2, [r1, r2]
    c.emit(super::venus_controller_packets::load_halfword_register(C::Al,R::R2,R::R1,R::R2)?)?;
    // 0x4d0f4: ldrh r1, [r1, r3]
    c.emit(super::venus_controller_packets::load_halfword_register(C::Al,R::R1,R::R1,R::new(3)?)?)?;
    // 0x4d0f8: mov r3, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,138)?)?;
    // 0x4d0fc: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x4d100: and r2, r2, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4d104: and r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4d108: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x4d10c: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4d110: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4d114: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4d118: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // 0x4d11c: mov r2, #396
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,396)?)?;
    // 0x4d120: orr r0, r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    // 0x4d124: str r0, [r7, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // 0x4d128: add r0, r8, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,7)?)?;
    // 0x4d12c: str r1, [r7, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,52)?)?;
    // 0x4d130: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4d134: add r0, r7, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,56)?)?;
    // 0x4d138: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x4d13c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4d140: ldr r1, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2744)?)?;
    // 0x4d144: ldrh r2, [r1, r2]
    c.emit(super::venus_controller_packets::load_halfword_register(C::Al,R::R2,R::R1,R::R2)?)?;
    // 0x4d148: ldrh r1, [r1, r3]
    c.emit(super::venus_controller_packets::load_halfword_register(C::Al,R::R1,R::R1,R::new(3)?)?)?;
    // 0x4d14c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d150: and r2, r2, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4d154: and r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4d158: ldr r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // 0x4d15c: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x4d160: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4d164: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4d168: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4d16c: mov r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,60)?)?;
    // 0x4d170: orr r0, r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    // 0x4d174: str r0, [r7, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,56)?)?;
    // 0x4d178: add r0, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,8)?)?;
    // 0x4d17c: str r1, [r7, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,60)?)?;
    // 0x4d180: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4d184: str r10, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(9)?,0)?)?;
    // 0x4d188: ldr r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,80)?)?;
    // 0x4d18c: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x4d190: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4d194: ldr r1, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,644)?)?;
    // 0x4d198: mov r5, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,22)?)?;
    // 0x4d19c: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x4d1a0: moveq r5, #23
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,23)?)?;
    // 0x4d1a4: add r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4d1a8: sub r2, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R0,32)?)?;
    // 0x4d1ac: rsb r1, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R0,32)?)?;
    // 0x4d1b0: lsl r0, r4, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(4)?,Shift::Lsl,R::R0,false)?)?;
    // 0x4d1b4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4d1b8: lsr r1, r4, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::new(4)?,Shift::Lsr,R::R1,false)?)?;
    // 0x4d1bc: movge r0, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4d1c0: lslge r1, r4, r2
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R1,R::R0,R::new(4)?,Shift::Lsl,R::R2,false)?)?;
    // 0x4d1c4: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x4d1c8: adds r0, r0, r6, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsr,1,true)?)?;
    // 0x4d1cc: adc r1, r1, #0
    c.emit(add_with_carry_zero(C::Al,R::R1,R::R1)?)?;
    // 0x4d1d0: bl 0x8187c <entry+0x3587c> @ imm = #0x346a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x8187c,true)?)?;
    // 0x4d1d4: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x4d1d8: ldr r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,84)?)?;
    // 0x4d1dc: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // 0x4d1e0: ldr r3, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,96)?)?;
    // 0x4d1e4: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4d1e8: add r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4d1ec: rsb r1, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R0,32)?)?;
    // 0x4d1f0: sub r2, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R0,32)?)?;
    // 0x4d1f4: lsl r0, r3, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(3)?,Shift::Lsl,R::R0,false)?)?;
    // 0x4d1f8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4d1fc: lsr r1, r3, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::new(3)?,Shift::Lsr,R::R1,false)?)?;
    // 0x4d200: lslge r1, r3, r2
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R1,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x4d204: ldr r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,72)?)?;
    // 0x4d208: movge r0, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4d20c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d210: adds r0, r0, r2, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsr,1,true)?)?;
    // 0x4d214: adc r1, r1, #0
    c.emit(add_with_carry_zero(C::Al,R::R1,R::R1)?)?;
    // 0x4d218: bl 0x8187c <entry+0x3587c> @ imm = #0x3465c
    c.emit(arm32::branch(C::Al,c.at as u32,0x8187c,true)?)?;
    // 0x4d21c: tst r10, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(10)?,31)?)?;
    // 0x4d220: bic r2, r4, #-134217728
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::new(4)?,4160749568)?)?;
    // 0x4d224: mov r4, r8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // 0x4d228: bic r0, r0, #-134217728
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,4160749568)?)?;
    // 0x4d22c: add r5, r7, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(7)?,80)?)?;
    // 0x4d230: lsr r12, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R12,R::new(6)?,Shift::Lsr,1)?)?;
    // 0x4d234: ldreq r1, [r10]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(10)?,0)?)?;
    // 0x4d238: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4d23c: streq r1, [r10]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(10)?,0)?)?;
    // 0x4d240: mov r1, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,152)?)?;
    // 0x4d244: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4d248: str r1, [r7, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,64)?)?;
    // 0x4d24c: add r1, r8, #9
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,9)?)?;
    // 0x4d250: str r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,72)?)?;
    // 0x4d254: str r2, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,68)?)?;
    // 0x4d258: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x4d25c: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x4d260: add r1, r7, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,72)?)?;
    // 0x4d264: str r1, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x4d268: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4d26c: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x4d270: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4d274: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x4d278: mov r1, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,156)?)?;
    // 0x4d27c: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4d280: str r1, [r7, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,72)?)?;
    // 0x4d284: add r1, r4, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,10)?)?;
    // 0x4d288: str r0, [r7, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,76)?)?;
    // 0x4d28c: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x4d290: str r5, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(9)?,0)?)?;
    // 0x4d294: ldr r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x4d298: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,76)?)?;
    // 0x4d29c: ldr r1, [r8, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,928)?)?;
    // 0x4d2a0: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x4d2a4: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x4d2a8: mul r6, r1, r10
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::R1,R::new(10)?)?)?;
    // 0x4d2ac: rsb r1, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R0,32)?)?;
    // 0x4d2b0: sub r3, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R0,32)?)?;
    // 0x4d2b4: lsr r1, r6, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::new(6)?,Shift::Lsr,R::R1,false)?)?;
    // 0x4d2b8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x4d2bc: lslge r1, r6, r3
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R1,R::R0,R::new(6)?,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x4d2c0: lsl r3, r6, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::new(6)?,Shift::Lsl,R::R0,false)?)?;
    // 0x4d2c4: movge r3, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d2c8: adds r0, r3, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R12,Shift::Lsl,0,true)?)?;
    // 0x4d2cc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d2d0: adc r1, r1, #0
    c.emit(add_with_carry_zero(C::Al,R::R1,R::R1)?)?;
    // 0x4d2d4: bl 0x8187c <entry+0x3587c> @ imm = #0x345a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x8187c,true)?)?;
    // 0x4d2d8: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x4d2dc: bic r0, r0, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,2147483648)?)?;
    // 0x4d2e0: ldreq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,0)?)?;
    // 0x4d2e4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4d2e8: streq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(5)?,0)?)?;
    // 0x4d2ec: mov r1, #188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,188)?)?;
    // 0x4d2f0: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4d2f4: str r1, [r7, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,80)?)?;
    // 0x4d2f8: str r0, [r7, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,84)?)?;
    // 0x4d2fc: add r0, r4, #11
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,11)?)?;
    // 0x4d300: mov r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16)?)?;
    // 0x4d304: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4d308: add r0, r7, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,88)?)?;
    // 0x4d30c: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x4d310: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // 0x4d314: ldr r2, [r8, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,644)?)?;
    // 0x4d318: ldr r1, [r8, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,716)?)?;
    // 0x4d31c: ldr r3, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,92)?)?;
    // 0x4d320: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x4d324: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // 0x4d328: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x4d32c: moveq r4, #32
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,32)?)?;
    // 0x4d330: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d334: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x4d338: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x4d33c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4d340: ldr r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,80)?)?;
    // 0x4d344: lsr r0, r10, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(10)?,Shift::Lsr,R::R0,false)?)?;
    // 0x4d348: bl 0x40a2c <entry+0xffffffffffff4a2c> @ imm = #-0xc924
    c.emit(arm32::branch(C::Al,c.at as u32,0x40a2c,true)?)?;
    // 0x4d34c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x4d350: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x4d354: ldr r1, [r8, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,720)?)?;
    // 0x4d358: ldr r2, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,84)?)?;
    // 0x4d35c: ldr r3, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,88)?)?;
    // 0x4d360: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x4d364: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x4d368: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x4d36c: ldr r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x4d370: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4d374: ldr r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,96)?)?;
    // 0x4d378: lsr r0, r0, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::R2,false)?)?;
    // 0x4d37c: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d380: bl 0x40a2c <entry+0xffffffffffff4a2c> @ imm = #-0xc95c
    c.emit(arm32::branch(C::Al,c.at as u32,0x40a2c,true)?)?;
    // 0x4d384: ldr r1, [r8, #0x30c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,780)?)?;
    // 0x4d388: mov r4, #86
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,86)?)?;
    // 0x4d38c: orr r4, r4, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,256)?)?;
    // 0x4d390: and r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,15)?)?;
    // 0x4d394: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x4d398: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x4d39c: mov r1, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,480)?)?;
    // 0x4d3a0: andhi r0, r0, #2
    c.emit(arm32::data_immediate(C::Hi,Op::And,R::R0,R::R0,2)?)?;
    // 0x4d3a4: andhi r5, r5, #2
    c.emit(arm32::data_immediate(C::Hi,Op::And,R::new(5)?,R::new(5)?,2)?)?;
    // 0x4d3a8: orr r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x4d3ac: and r0, r1, r0, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,5,false)?)?;
    // 0x4d3b0: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x4d3b4: bic r1, r1, #480
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,480)?)?;
    // 0x4d3b8: orr r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d3bc: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x4d3c0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4d3c4: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4d3c8: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4d3cc: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4d3d0: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x4d3d4: orr r2, r2, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,10240)?)?;
    // 0x4d3d8: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x4d3dc: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x4d3e0: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x4d3e4: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x4d3e8: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x4d3ec: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x4d3f0: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x4d3f4: ldr r0, [r8, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2744)?)?;
    // 0x4d3f8: ldrsh r0, [r0, r4]
    c.emit(super::venus_controller_packets::load_signed_halfword_register(C::Al,R::R0,R::R0,R::new(4)?)?)?;
    // 0x4d3fc: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x4d400: blt 0x4d420 <entry+0x1420>  @ imm = #0x18
    c.branch(C::Lt,0x4d420)?;
    // 0x4d404: ldr r2, [pc, #0x948]        @ 0x1d54 <entry+0x1d54>
    c.literal(C::Al,R::R2,0x4dd54)?;
    // 0x4d408: mov r1, #740
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,740)?)?;
    // 0x4d40c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4d410: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4d414: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4bc6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4d418: ldr r0, [r8, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2744)?)?;
    // 0x4d41c: ldrh r0, [r0, r4]
    c.emit(super::venus_controller_packets::load_halfword_register(C::Al,R::R0,R::R0,R::new(4)?)?)?;
    // 0x4d420: ldr r1, [pc, #0x938]        @ 0x1d60 <entry+0x1d60>
    c.literal(C::Al,R::R1,0x4dd60)?;
    // 0x4d424: lsl r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
    // 0x4d428: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x4d42c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d430: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x4d434: ldr r5, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x4d438: ldr r0, [r5, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,240)?)?;
    // 0x4d43c: ldr r1, [r5, #0x10c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,268)?)?;
    // 0x4d440: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d444: bne 0x4d484 <entry+0x1484>  @ imm = #0x38
    c.branch(C::Ne,0x4d484)?;
    // 0x4d448: ldr r0, [r5, #0xf4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,244)?)?;
    // 0x4d44c: ldr r1, [r5, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,264)?)?;
    // 0x4d450: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d454: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d458: bne 0x4d484 <entry+0x1484>  @ imm = #0x24
    c.branch(C::Ne,0x4d484)?;
    // 0x4d45c: ldr r0, [r5, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,248)?)?;
    // 0x4d460: ldr r1, [r5, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,260)?)?;
    // 0x4d464: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d468: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d46c: bne 0x4d484 <entry+0x1484>  @ imm = #0x10
    c.branch(C::Ne,0x4d484)?;
    // 0x4d470: ldr r0, [r5, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,252)?)?;
    // 0x4d474: ldr r1, [r5, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,256)?)?;
    // 0x4d478: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d47c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d480: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4d484: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,60)?)?;
    // 0x4d488: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,56)?)?;
    // 0x4d48c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x4d490: mov r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,255)?)?;
    // 0x4d494: sub r6, r11, #92
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,92)?)?;
    // 0x4d498: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x4d49c: str r3, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,84)?)?;
    // 0x4d4a0: strh r9, [r11, #-56]
    c.emit(arm32::store_halfword(C::Al,R::new(9)?,R::R11,-56)?)?;
    // 0x4d4a4: str r5, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,92)?)?;
    // 0x4d4a8: orr r10, r10, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,256)?)?;
    // 0x4d4ac: orr r0, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // 0x4d4b0: str r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,96)?)?;
    // 0x4d4b4: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // 0x4d4b8: movne r0, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x4d4bc: str r0, [r11, #-0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-92)?)?;
    // 0x4d4c0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x4d4c4: strh r0, [r11, #-104]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R11,-104)?)?;
    // 0x4d4c8: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x4d4cc: sub r4, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,56)?)?;
    // 0x4d4d0: sub r8, r11, #104
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,104)?)?;
    // 0x4d4d4: add r1, r5, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,12)?)?;
    // 0x4d4d8: stm sp, {r0, r7, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(7)?.bit()|R::new(10)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d4dc: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d4e0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d4e4: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x4d4e8: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1eaa0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d4ec: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x4d4f0: add r1, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,8)?)?;
    // 0x4d4f4: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d4f8: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x4d4fc: stm sp, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d500: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4d504: orr r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
    // 0x4d508: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4d50c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d510: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1eac8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d514: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x4d518: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x4d51c: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d520: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x4d524: stm sp, {r0, r7, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(7)?.bit()|R::new(10)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d528: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d52c: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1eae4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d530: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x4d534: add r5, r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,32)?)?;
    // 0x4d538: cmp r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,8)?)?;
    // 0x4d53c: bne 0x4d4c8 <entry+0x14c8>  @ imm = #-0x7c
    c.branch(C::Ne,0x4d4c8)?;
    // 0x4d540: ldr r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
    // 0x4d544: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4d548: sub r5, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,56)?)?;
    // 0x4d54c: sub r7, r11, #104
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,104)?)?;
    // 0x4d550: mov r9, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,9)?)?;
    // 0x4d554: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x4d558: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x4d55c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x4d560: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x4d564: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // 0x4d568: str r8, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,4)?)?;
    // 0x4d56c: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x4d570: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x4d574: str r10, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,8)?)?;
    // 0x4d578: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4d57c: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x4d580: movne r0, #240
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,240)?)?;
    // 0x4d584: add r4, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4d588: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d58c: add r1, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,12)?)?;
    // 0x4d590: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1eb48
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d594: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x4d598: add r1, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,8)?)?;
    // 0x4d59c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x4d5a0: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x4d5a4: stm sp, {r0, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d5a8: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4d5ac: orr r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
    // 0x4d5b0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4d5b4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d5b8: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1eb70
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d5bc: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d5c0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x4d5c4: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x4d5c8: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x4d5cc: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // 0x4d5d0: stmib sp, {r8, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(8)?.bit()|R::new(10)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4d5d4: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1eb8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d5d8: ldr r4, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,100)?)?;
    // 0x4d5dc: ldr r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x4d5e0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x4d5e4: mov r2, #10304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10304)?)?;
    // 0x4d5e8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x4d5ec: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x4d5f0: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4d5f4: ldr r7, [r6], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(7)?,R::new(6)?,4)?)?;
    // 0x4d5f8: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x4d5fc: ldreq r5, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::new(3)?,0)?)?;
    // 0x4d600: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4d604: streq r5, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::new(3)?,0)?)?;
    // 0x4d608: stm r3, {r2, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R2.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d60c: add r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    // 0x4d610: add r3, r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,8)?)?;
    // 0x4d614: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x4d618: ldr r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-32)?)?;
    // 0x4d61c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4d620: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x4d624: bne 0x4d5f0 <entry+0x15f0>  @ imm = #-0x3c
    c.branch(C::Ne,0x4d5f0)?;
    // 0x4d628: add r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
    // 0x4d62c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x4d630: mov r4, #344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,344)?)?;
    // 0x4d634: ldr r0, [r8, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2744)?)?;
    // 0x4d638: ldrsh r0, [r0, r4]
    c.emit(super::venus_controller_packets::load_signed_halfword_register(C::Al,R::R0,R::R0,R::new(4)?)?)?;
    // 0x4d63c: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x4d640: blt 0x4d660 <entry+0x1660>  @ imm = #0x18
    c.branch(C::Lt,0x4d660)?;
    // 0x4d644: ldr r2, [pc, #0x708]        @ 0x1d54 <entry+0x1d54>
    c.literal(C::Al,R::R2,0x4dd54)?;
    // 0x4d648: mov r1, #780
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,780)?)?;
    // 0x4d64c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4d650: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4d654: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4beac
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4d658: ldr r0, [r8, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2744)?)?;
    // 0x4d65c: ldrh r0, [r0, r4]
    c.emit(super::venus_controller_packets::load_halfword_register(C::Al,R::R0,R::R0,R::new(4)?)?)?;
    // 0x4d660: ldr r1, [pc, #0x6f8]        @ 0x1d60 <entry+0x1d60>
    c.literal(C::Al,R::R1,0x4dd60)?;
    // 0x4d664: lsl r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
    // 0x4d668: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x4d66c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d670: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x4d674: ldr r5, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x4d678: ldr r0, [r5, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,240)?)?;
    // 0x4d67c: ldr r1, [r5, #0x10c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,268)?)?;
    // 0x4d680: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d684: bne 0x4d6c4 <entry+0x16c4>  @ imm = #0x38
    c.branch(C::Ne,0x4d6c4)?;
    // 0x4d688: ldr r0, [r5, #0xf4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,244)?)?;
    // 0x4d68c: ldr r1, [r5, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,264)?)?;
    // 0x4d690: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d694: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d698: bne 0x4d6c4 <entry+0x16c4>  @ imm = #0x24
    c.branch(C::Ne,0x4d6c4)?;
    // 0x4d69c: ldr r0, [r5, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,248)?)?;
    // 0x4d6a0: ldr r1, [r5, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,260)?)?;
    // 0x4d6a4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d6a8: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d6ac: bne 0x4d6c4 <entry+0x16c4>  @ imm = #0x10
    c.branch(C::Ne,0x4d6c4)?;
    // 0x4d6b0: ldr r0, [r5, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,252)?)?;
    // 0x4d6b4: ldr r1, [r5, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,256)?)?;
    // 0x4d6b8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4d6bc: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4d6c0: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4d6c4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x4d6c8: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // 0x4d6cc: mov r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,255)?)?;
    // 0x4d6d0: sub r6, r11, #92
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,92)?)?;
    // 0x4d6d4: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // 0x4d6d8: str r3, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,84)?)?;
    // 0x4d6dc: strh r7, [r11, #-56]
    c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::R11,-56)?)?;
    // 0x4d6e0: str r5, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,92)?)?;
    // 0x4d6e4: movne r0, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x4d6e8: orr r10, r10, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,256)?)?;
    // 0x4d6ec: str r0, [r11, #-0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-92)?)?;
    // 0x4d6f0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x4d6f4: strh r0, [r11, #-104]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R11,-104)?)?;
    // 0x4d6f8: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x4d6fc: sub r4, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,56)?)?;
    // 0x4d700: sub r8, r11, #104
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,104)?)?;
    // 0x4d704: add r1, r5, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,12)?)?;
    // 0x4d708: stm sp, {r0, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(9)?.bit()|R::new(10)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d70c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d710: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d714: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x4d718: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1ecd0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d71c: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x4d720: add r1, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,8)?)?;
    // 0x4d724: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d728: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x4d72c: stm sp, {r0, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(9)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d730: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4d734: orr r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
    // 0x4d738: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4d73c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d740: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1ecf8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d744: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x4d748: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x4d74c: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d750: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x4d754: stm sp, {r0, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(9)?.bit()|R::new(10)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d758: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d75c: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1ed14
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d760: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4d764: add r5, r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,32)?)?;
    // 0x4d768: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // 0x4d76c: bne 0x4d6f8 <entry+0x16f8>  @ imm = #-0x7c
    c.branch(C::Ne,0x4d6f8)?;
    // 0x4d770: ldr r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
    // 0x4d774: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4d778: sub r5, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,56)?)?;
    // 0x4d77c: sub r7, r11, #104
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,104)?)?;
    // 0x4d780: mov r9, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,9)?)?;
    // 0x4d784: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x4d788: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x4d78c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x4d790: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x4d794: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // 0x4d798: str r8, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,4)?)?;
    // 0x4d79c: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x4d7a0: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x4d7a4: str r10, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,8)?)?;
    // 0x4d7a8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4d7ac: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x4d7b0: movne r0, #240
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,240)?)?;
    // 0x4d7b4: add r4, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4d7b8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d7bc: add r1, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,12)?)?;
    // 0x4d7c0: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1ed78
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d7c4: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x4d7c8: add r1, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,8)?)?;
    // 0x4d7cc: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x4d7d0: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x4d7d4: stm sp, {r0, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d7d8: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4d7dc: orr r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
    // 0x4d7e0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4d7e4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d7e8: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1eda0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d7ec: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d7f0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x4d7f4: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x4d7f8: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x4d7fc: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // 0x4d800: stmib sp, {r8, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(8)?.bit()|R::new(10)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4d804: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1edbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d808: ldr r10, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,100)?)?;
    // 0x4d80c: ldr r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x4d810: mov r2, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2144)?)?;
    // 0x4d814: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x4d818: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // 0x4d81c: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x4d820: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x4d824: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4d828: ldr r5, [r6], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(5)?,R::new(6)?,4)?)?;
    // 0x4d82c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x4d830: ldreq r4, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(3)?,0)?)?;
    // 0x4d834: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x4d838: streq r4, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(3)?,0)?)?;
    // 0x4d83c: stm r3, {r2, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R2.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d840: add r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    // 0x4d844: add r3, r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,8)?)?;
    // 0x4d848: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x4d84c: ldr r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // 0x4d850: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4d854: str r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // 0x4d858: bne 0x4d824 <entry+0x1824>  @ imm = #-0x3c
    c.branch(C::Ne,0x4d824)?;
    // 0x4d85c: add r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
    // 0x4d860: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x4d864: ldr r4, [r8, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,2744)?)?;
    // 0x4d868: ldrb r0, [r4, #0x143]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,323)?)?;
    // 0x4d86c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x4d870: bne 0x4da18 <entry+0x1a18>  @ imm = #0x1a0
    c.branch(C::Ne,0x4da18)?;
    // 0x4d874: sub r6, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,56)?)?;
    // 0x4d878: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x4d87c: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x4d880: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x4d884: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d888: bl 0x31ec <entry+0xfffffffffffb71ec> @ imm = #-0x4a6a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x4d88c: ldrb r0, [r4, #0x160]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,352)?)?;
    // 0x4d890: strh r5, [r11, #-106]
    c.emit(arm32::store_halfword(C::Al,R::new(5)?,R::R11,-106)?)?;
    // 0x4d894: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4d898: beq 0x4d900 <entry+0x1900>  @ imm = #0x60
    c.branch(C::Eq,0x4d900)?;
    // 0x4d89c: add r0, r4, #98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,98)?)?;
    // 0x4d8a0: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x4d8a4: sub r2, r11, #92
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,92)?)?;
    // 0x4d8a8: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x4d8ac: ldrsh r3, [r0], #2
    c.emit(load_signed_halfword_postindexed(C::Al,R::new(3)?,R::R0,2)?)?;
    // 0x4d8b0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x4d8b4: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x4d8b8: str r3, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R2,4)?)?;
    // 0x4d8bc: blt 0x4d8ac <entry+0x18ac>  @ imm = #-0x18
    c.branch(C::Lt,0x4d8ac)?;
    // 0x4d8c0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4d8c4: add r1, r4, #372
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,372)?)?;
    // 0x4d8c8: strh r0, [r11, #-106]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R11,-106)?)?;
    // 0x4d8cc: ldrsh r2, [r1], #2
    c.emit(load_signed_halfword_postindexed(C::Al,R::R2,R::R1,2)?)?;
    // 0x4d8d0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4d8d4: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x4d8d8: str r2, [r7], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R2,R::new(7)?,4)?)?;
    // 0x4d8dc: bne 0x4d8cc <entry+0x18cc>  @ imm = #-0x18
    c.branch(C::Ne,0x4d8cc)?;
    // 0x4d8e0: b 0x4d944 <entry+0x1944>  @ imm = #0x5c
    c.branch(C::Al,0x4d944)?;
    // 0x4d8e4: ldr r3, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,104)?)?;
    // 0x4d8e8: ldrb r6, [r3, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,988)?)?;
    // 0x4d8ec: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4d8f0: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x4d8f4: andne r3, r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::And,R::new(3)?,R::R1,1)?)?;
    // 0x4d8f8: b 0x4cd88 <entry+0xd88>   @ imm = #-0xb78
    c.branch(C::Al,0x4cd88)?;
    // 0x4d8fc is skipped by the unconditional branch at 0x4d8f8;
    // it is a logging-address literal, not ANDEQ / LSR #32.
    c.check(0x4d8fc)?;
    c.emit(EMBEDDED_LOG_POINTER)?;
    // 0x4d900: ldr r2, [pc, #0x45c]        @ 0x1d64 <entry+0x1d64>
    c.literal(C::Al,R::R2,0x4dd64)?;
    // 0x4d904: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x4d908: sub r1, r11, #92
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,92)?)?;
    // 0x4d90c: ldrh r3, [r2], #2
    c.emit(load_halfword_postindexed(C::Al,R::new(3)?,R::R2,2)?)?;
    // 0x4d910: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4d914: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x4d918: str r3, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x4d91c: blt 0x4d90c <entry+0x190c>  @ imm = #-0x18
    c.branch(C::Lt,0x4d90c)?;
    // 0x4d920: ldr r1, [pc, #0x440]        @ 0x1d68 <entry+0x1d68>
    c.literal(C::Al,R::R1,0x4dd68)?;
    // 0x4d924: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4d928: sub r2, r11, #104
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,104)?)?;
    // 0x4d92c: strh r0, [r11, #-106]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R11,-106)?)?;
    // 0x4d930: ldrh r3, [r1], #2
    c.emit(load_halfword_postindexed(C::Al,R::new(3)?,R::R1,2)?)?;
    // 0x4d934: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4d938: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x4d93c: str r3, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R2,4)?)?;
    // 0x4d940: bne 0x4d930 <entry+0x1930>  @ imm = #-0x18
    c.branch(C::Ne,0x4d930)?;
    // 0x4d944: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4d948: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x4d94c: sub r4, r11, #106
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,106)?)?;
    // 0x4d950: add r5, sp, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,108)?)?;
    // 0x4d954: mov r8, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,9)?)?;
    // 0x4d958: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // 0x4d95c: orr r0, r0, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1792)?)?;
    // 0x4d960: strh r7, [r11, #-106]
    c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::R11,-106)?)?;
    // 0x4d964: strh r7, [sp, #108]
    c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::SP,108)?)?;
    // 0x4d968: stm sp, {r1, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d96c: sub r1, r11, #92
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,92)?)?;
    // 0x4d970: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d974: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x4d978: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4d97c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d980: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1ef38
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d984: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x4d988: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x4d98c: sub r1, r11, #104
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,104)?)?;
    // 0x4d990: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4d994: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x4d998: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x4d99c: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4d9a0: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x4d9a4: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4d9a8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4d9ac: bl 0x2ea50 <entry+0xfffffffffffe2a50> @ imm = #-0x1ef64
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
    // 0x4d9b0: ldrsh r0, [r11, #-106]
    c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::R11,-106)?)?;
    // 0x4d9b4: ldrsh r2, [sp, #108]
    c.emit(arm32::load_signed_halfword(C::Al,R::R2,R::SP,108)?)?;
    // 0x4d9b8: ldr r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x4d9bc: ldr r1, [r6, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
    // 0x4d9c0: rsb r2, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R2,R::R2,32)?)?;
    // 0x4d9c4: lsl r1, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsl,R::R2,false)?)?;
    // 0x4d9c8: str r1, [r6, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
    // 0x4d9cc: mov r1, #10368
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10368)?)?;
    // 0x4d9d0: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x4d9d4: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x4d9d8: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x4d9dc: ldr r3, [r6], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::new(6)?,4)?)?;
    // 0x4d9e0: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4d9e4: ldreq r5, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R2,0)?)?;
    // 0x4d9e8: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4d9ec: streq r5, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R2,0)?)?;
    // 0x4d9f0: stm r2, {r1, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R1.bit()|R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4d9f4: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // 0x4d9f8: add r2, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,8)?)?;
    // 0x4d9fc: cmp r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4)?)?;
    // 0x4da00: ldr r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // 0x4da04: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4da08: str r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // 0x4da0c: bne 0x4d9d8 <entry+0x19d8>  @ imm = #-0x3c
    c.branch(C::Ne,0x4d9d8)?;
    // 0x4da10: add r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x4da14: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x4da18: ldr r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,96)?)?;
    // 0x4da1c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x4da20: bne 0x4da40 <entry+0x1a40>  @ imm = #0x18
    c.branch(C::Ne,0x4da40)?;
    // 0x4da24: ldr r0, [r8, #0x858]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2136)?)?;
    // 0x4da28: ldr r2, [pc, #0x324]        @ 0x1d54 <entry+0x1d54>
    c.literal(C::Al,R::R2,0x4dd54)?;
    // 0x4da2c: mov r1, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,93)?)?;
    // 0x4da30: orr r1, r1, #11008
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,11008)?)?;
    // 0x4da34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4da38: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4da3c: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4c294
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4da40: ldr r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,88)?)?;
    // 0x4da44: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x4da48: bne 0x4da68 <entry+0x1a68>  @ imm = #0x18
    c.branch(C::Ne,0x4da68)?;
    // 0x4da4c: ldr r0, [r8, #0x868]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2152)?)?;
    // 0x4da50: ldr r2, [pc, #0x2fc]        @ 0x1d54 <entry+0x1d54>
    c.literal(C::Al,R::R2,0x4dd54)?;
    // 0x4da54: mov r1, #95
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,95)?)?;
    // 0x4da58: orr r1, r1, #11008
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,11008)?)?;
    // 0x4da5c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4da60: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4da64: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4c2bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4da68: ldr r0, [r8, #0x858]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2136)?)?;
    // 0x4da6c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4da70: beq 0x4dad4 <entry+0x1ad4>  @ imm = #0x5c
    c.branch(C::Eq,0x4dad4)?;
    // 0x4da74: ldr r1, [r8, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1780)?)?;
    // 0x4da78: sub r4, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
    // 0x4da7c: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x4da80: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4da84: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4da88: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4da8c: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x4da90: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4da94: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4da98: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4da9c: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x4daa0: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfa7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4daa4: ldr r0, [r8, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1780)?)?;
    // 0x4daa8: ldr r1, [r8, #0x85c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2140)?)?;
    // 0x4daac: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4dab0: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x4dab4: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4dab8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4dabc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dac0: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x4dac4: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4dac8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4dacc: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x4dad0: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfaac
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4dad4: ldr r0, [r8, #0x868]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2152)?)?;
    // 0x4dad8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4dadc: beq 0x4db40 <entry+0x1b40>  @ imm = #0x5c
    c.branch(C::Eq,0x4db40)?;
    // 0x4dae0: ldr r1, [r8, #0x6fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1788)?)?;
    // 0x4dae4: sub r4, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
    // 0x4dae8: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x4daec: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4daf0: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x4daf4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4daf8: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x4dafc: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4db00: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4db04: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4db08: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x4db0c: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfae8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4db10: ldr r0, [r8, #0x6fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1788)?)?;
    // 0x4db14: ldr r1, [r8, #0x86c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2156)?)?;
    // 0x4db18: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4db1c: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x4db20: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4db24: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x4db28: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4db2c: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x4db30: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4db34: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4db38: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x4db3c: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfb18
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4db40: ldr r0, [r8, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,664)?)?;
    // 0x4db44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4db48: beq 0x4dc3c <entry+0x1c3c>  @ imm = #0xec
    c.branch(C::Eq,0x4dc3c)?;
    // 0x4db4c: add r4, r8, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,2144)?)?;
    // 0x4db50: mov r5, #67
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,67)?)?;
    // 0x4db54: sub r9, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,32)?)?;
    // 0x4db58: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x4db5c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x4db60: sub r6, r5, #67
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(5)?,67)?)?;
    // 0x4db64: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4db68: beq 0x4dbc0 <entry+0x1bc0>  @ imm = #0x50
    c.branch(C::Eq,0x4dbc0)?;
    // 0x4db6c: ldr r1, [r8, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1784)?)?;
    // 0x4db70: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4db74: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x4db78: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x4db7c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x4db80: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4db84: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4db88: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4db8c: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x4db90: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfb6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4db94: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x4db98: ldr r1, [r8, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1784)?)?;
    // 0x4db9c: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4dba0: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x4dba4: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x4dba8: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x4dbac: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dbb0: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4dbb4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4dbb8: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x4dbbc: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfb98
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4dbc0: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x4dbc4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4dbc8: beq 0x4dc20 <entry+0x1c20>  @ imm = #0x50
    c.branch(C::Eq,0x4dc20)?;
    // 0x4dbcc: ldr r1, [r8, #0x700]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1792)?)?;
    // 0x4dbd0: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4dbd4: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x4dbd8: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x4dbdc: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x4dbe0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dbe4: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4dbe8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4dbec: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x4dbf0: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfbcc
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4dbf4: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x4dbf8: ldr r1, [r8, #0x700]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1792)?)?;
    // 0x4dbfc: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4dc00: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x4dc04: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x4dc08: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x4dc0c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dc10: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4dc14: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x4dc18: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x4dc1c: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfbf8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4dc20: ldr r1, [r8, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,664)?)?;
    // 0x4dc24: sub r2, r5, #66
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(5)?,66)?)?;
    // 0x4dc28: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
    // 0x4dc2c: add r4, r4, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,32)?)?;
    // 0x4dc30: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x4dc34: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4dc38: blo 0x4db5c <entry+0x1b5c>  @ imm = #-0xe4
    c.branch(C::Cc,0x4db5c)?;
    // 0x4dc3c: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x4dc40: mov r2, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,182)?)?;
    // 0x4dc44: orr r2, r2, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3328)?)?;
    // 0x4dc48: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4dc4c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4dc50: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4dc54: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4dc58: mov r1, #436
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,436)?)?;
    // 0x4dc5c: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x4dc60: stm r0, {r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4dc64: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x4dc68: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x4dc6c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x4dc70: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x4dc74: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x4dc78: ldr r0, [r8, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2432)?)?;
    // 0x4dc7c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4dc80: beq 0x4dce0 <entry+0x1ce0>  @ imm = #0x58
    c.branch(C::Eq,0x4dce0)?;
    // 0x4dc84: ldr r1, [r8, #0x704]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1796)?)?;
    // 0x4dc88: sub r4, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
    // 0x4dc8c: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x4dc90: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4dc94: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4dc98: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dc9c: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x4dca0: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4dca4: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4dca8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4dcac: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x4dcb0: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfc8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4dcb4: ldr r0, [r8, #0x984]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2436)?)?;
    // 0x4dcb8: ldr r1, [r8, #0x708]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1800)?)?;
    // 0x4dcbc: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4dcc0: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x4dcc4: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4dcc8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4dccc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dcd0: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4dcd4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4dcd8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x4dcdc: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfcb8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4dce0: ldr r0, [r8, #0x988]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2440)?)?;
    // 0x4dce4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4dce8: beq 0x4dd48 <entry+0x1d48>  @ imm = #0x58
    c.branch(C::Eq,0x4dd48)?;
    // 0x4dcec: ldr r1, [r8, #0x70c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1804)?)?;
    // 0x4dcf0: sub r4, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
    // 0x4dcf4: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x4dcf8: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4dcfc: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x4dd00: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dd04: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x4dd08: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4dd0c: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4dd10: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4dd14: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x4dd18: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfcf4
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4dd1c: ldr r0, [r8, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2444)?)?;
    // 0x4dd20: ldr r1, [r8, #0x710]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1808)?)?;
    // 0x4dd24: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x4dd28: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x4dd2c: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4dd30: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x4dd34: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dd38: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4dd3c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4dd40: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x4dd44: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xfd20
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4dd48: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x4dd4c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x4dd50: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x4dd54)?;
 for literal in CONTROLLER_DIAGNOSTICS {c.emit(literal)?;}
 c.check(0x4dd6c)?;
    // 0x4dd6c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x4dd70: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x4dd74: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x4dd78: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x4dd7c: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x4dd80: str r10, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,32)?)?;
    // 0x4dd84: ldr r4, [r9, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,664)?)?;
    // 0x4dd88: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x4dd8c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x4dd90: beq 0x4dfc4 <entry+0x1fc4>  @ imm = #0x22c
    c.branch(C::Eq,0x4dfc4)?;
    // 0x4dd94: ldr r5, [r9, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,944)?)?;
    // 0x4dd98: mov r12, #364
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,364)?)?;
    // 0x4dd9c: mov r2, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,360)?)?;
    // 0x4dda0: mov lr, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,16)?)?;
    // 0x4dda4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4dda8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4ddac: orr r12, r12, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,14336)?)?;
    // 0x4ddb0: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x4ddb4: orr lr, lr, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,4096)?)?;
    // 0x4ddb8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x4ddbc: beq 0x4de9c <entry+0x1e9c>  @ imm = #0xd8
    c.branch(C::Eq,0x4de9c)?;
    // 0x4ddc0: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x4ddc4: lsl r7, r3, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(3)?,Shift::Lsl,16)?)?;
    // 0x4ddc8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4ddcc: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x4ddd0: ldr r4, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,0)?)?;
    // 0x4ddd4: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x4ddd8: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x4dddc: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x4dde0: ldreq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,0)?)?;
    // 0x4dde4: orr r5, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(3)?,4096)?)?;
    // 0x4dde8: orr r3, r3, lr
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x4ddec: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x4ddf0: streq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,0)?)?;
    // 0x4ddf4: str r12, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,0)?)?;
    // 0x4ddf8: str r5, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,4)?)?;
    // 0x4ddfc: ldr r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,32)?)?;
    // 0x4de00: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
    // 0x4de04: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4de08: add r0, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,8)?)?;
    // 0x4de0c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4de10: ldreq r8, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(8)?,R::R0,0)?)?;
    // 0x4de14: andeq r1, r8, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::new(8)?,1)?)?;
    // 0x4de18: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4de1c: add r0, r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,2)?)?;
    // 0x4de20: str r2, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,8)?)?;
    // 0x4de24: str r10, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,12)?)?;
    // 0x4de28: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4de2c: add r0, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,16)?)?;
    // 0x4de30: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4de34: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4de38: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4de3c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4de40: add r0, r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,3)?)?;
    // 0x4de44: str r12, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,16)?)?;
    // 0x4de48: str r3, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,20)?)?;
    // 0x4de4c: and r3, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(6)?,255)?)?;
    // 0x4de50: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4de54: add r0, r4, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,24)?)?;
    // 0x4de58: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4de5c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4de60: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4de64: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4de68: add r0, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,4)?)?;
    // 0x4de6c: str r2, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,24)?)?;
    // 0x4de70: str r10, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,28)?)?;
    // 0x4de74: add r4, r4, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,32)?)?;
    // 0x4de78: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4de7c: ldr r5, [r9, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,944)?)?;
    // 0x4de80: cmp r5, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4de84: bhi 0x4ddd4 <entry+0x1dd4>  @ imm = #-0xb8
    c.branch(C::Hi,0x4ddd4)?;
    // 0x4de88: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x4de8c: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // 0x4de90: ldr r4, [r9, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,664)?)?;
    // 0x4de94: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x4de98: b 0x4dea0 <entry+0x1ea0>  @ imm = #0x0
    c.branch(C::Al,0x4dea0)?;
    // 0x4de9c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x4dea0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4dea4: and r3, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R0,255)?)?;
    // 0x4dea8: cmp r4, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4deac: bhi 0x4ddb8 <entry+0x1db8>  @ imm = #-0xfc
    c.branch(C::Hi,0x4ddb8)?;
    // 0x4deb0: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x4deb4: beq 0x4dfc4 <entry+0x1fc4>  @ imm = #0x108
    c.branch(C::Eq,0x4dfc4)?;
    // 0x4deb8: ldr r5, [r9, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,944)?)?;
    // 0x4debc: mov r8, #356
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,356)?)?;
    // 0x4dec0: mov r2, #2400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2400)?)?;
    // 0x4dec4: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // 0x4dec8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4decc: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x4ded0: orr r8, r8, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,14336)?)?;
    // 0x4ded4: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x4ded8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x4dedc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x4dee0: beq 0x4dfb4 <entry+0x1fb4>  @ imm = #0xcc
    c.branch(C::Eq,0x4dfb4)?;
    // 0x4dee4: ldr r6, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // 0x4dee8: lsl r7, r3, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(3)?,Shift::Lsl,16)?)?;
    // 0x4deec: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4def0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x4def4: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x4def8: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x4defc: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x4df00: ldreq r0, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(6)?,0)?)?;
    // 0x4df04: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x4df08: streq r0, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(6)?,0)?)?;
    // 0x4df0c: str r8, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,0)?)?;
    // 0x4df10: str r3, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,4)?)?;
    // 0x4df14: orr r3, r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,16)?)?;
    // 0x4df18: ldr r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,32)?)?;
    // 0x4df1c: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
    // 0x4df20: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4df24: add r0, r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,8)?)?;
    // 0x4df28: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4df2c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4df30: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4df34: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4df38: add r0, r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,2)?)?;
    // 0x4df3c: str r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,8)?)?;
    // 0x4df40: str lr, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(6)?,12)?)?;
    // 0x4df44: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4df48: add r0, r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,16)?)?;
    // 0x4df4c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4df50: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4df54: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4df58: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4df5c: add r0, r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,3)?)?;
    // 0x4df60: str r8, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,16)?)?;
    // 0x4df64: str r3, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,20)?)?;
    // 0x4df68: and r3, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,255)?)?;
    // 0x4df6c: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4df70: add r0, r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,24)?)?;
    // 0x4df74: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4df78: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4df7c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4df80: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4df84: add r0, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,4)?)?;
    // 0x4df88: str r2, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,24)?)?;
    // 0x4df8c: str lr, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(6)?,28)?)?;
    // 0x4df90: add r6, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,32)?)?;
    // 0x4df94: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x4df98: ldr r5, [r9, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,944)?)?;
    // 0x4df9c: cmp r5, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4dfa0: bhi 0x4def4 <entry+0x1ef4>  @ imm = #-0xb4
    c.branch(C::Hi,0x4def4)?;
    // 0x4dfa4: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x4dfa8: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x4dfac: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x4dfb0: ldr r4, [r9, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,664)?)?;
    // 0x4dfb4: add r12, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,1)?)?;
    // 0x4dfb8: and r3, r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R12,255)?)?;
    // 0x4dfbc: cmp r4, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4dfc0: bhi 0x4ded8 <entry+0x1ed8>  @ imm = #-0xf0
    c.branch(C::Hi,0x4ded8)?;
    // 0x4dfc4: ldr r0, [r9, #0x694]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1684)?)?;
    // 0x4dfc8: ldr r3, [r9, #0x7c8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,1992)?)?;
    // 0x4dfcc: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x4dfd0: add r6, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::SP,32)?)?;
    // 0x4dfd4: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x4dfd8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x4dfdc: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x4dfe0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x4dfe4: str r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x4dfe8: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x4dfec: stm sp, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4dff0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4dff4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4dff8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4dffc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();serialize_signed_frame_records(&mut c)?;Ok(c.bytes)
}
#[cfg(test)] mod tests {
 use super::*;
 #[test] fn signed_halfword_and_inline_diagnostics_are_checked() {
  assert_eq!(add_with_carry_zero(C::Al,R::R1,R::R1).unwrap(),0xe2a1_1000);
  assert_eq!(load_signed_halfword_postindexed(C::Al,R::new(3).unwrap(),R::R0,2).unwrap(),0xe0d0_30f2);
  assert_eq!(load_halfword_postindexed(C::Al,R::new(3).unwrap(),R::R2,2).unwrap(),0xe0d2_30b2);
  assert!(add_with_carry_zero(C::Al,R::PC,R::R1).is_err());
  assert!(load_halfword_postindexed(C::Al,R::R0,R::R0,2).is_err());
  let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
  let at=0x4d8fc-START;
  assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),EMBEDDED_LOG_POINTER);
  for (i,literal) in CONTROLLER_DIAGNOSTICS.iter().enumerate(){
   let offset=0x4dd54-START+i*4;
   assert_eq!(u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap()),*literal);
  }
 }
}
