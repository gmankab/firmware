// SPDX-License-Identifier: GPL-3.0-only
//! Codec controller property initializer: serializes a fixed roster of
//! platform codec control fields, reference pointer slots and default values.
//! ARM A32 operations are independently derived from typed register operands;
//! the trailing mask belongs to a literal slot after a real function return.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_controller_format_registers::signed_byte_register as load_signed_byte_register;

pub const START:usize=0x54028;
pub const END:usize=0x54854;
pub const FILE_OFFSET:usize=0x4000+START;
const CODEC_FIELD_MASK:u32=0x001f_a3ff;
struct Code{at:usize,bytes:Vec<u8>}
impl Code {
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,expected:usize)->Result<(),String>{
    if self.at!=expected{return Err(format!("A32 codec initializer 0x{:x} != {expected:#x}",self.at));}Ok(())
 }
 #[inline(never)] fn emit(&mut self,value:u32)->Result<(),String>{
   if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0{return Err("outside Agent04 reserved A32 region".into());}
   self.bytes.extend_from_slice(&value.to_le_bytes());self.at+=4;Ok(())
 }
 fn literal(&mut self,condition:C,rd:R,target:usize)->Result<(),String>{
  let off=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"literal displacement out of range")?;
  self.emit(arm32::transfer_word(condition,true,rd,R::PC,off)?)
 }
}

/// Assign core controller properties, sampled format dimensions and tables.
#[inline(never)]fn write_default_controller_properties(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x54028: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5402c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x54030: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // 0x54034: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x54038: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x5403c: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x54040: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x54044: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // 0x54048: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x5404c: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x54050: mov r3, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,21)?)?;
    // 0x54054: mov r12, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,8)?)?;
    // 0x54058: mov r7, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,17)?)?;
    // 0x5405c: mov r9, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,21)?)?;
    // 0x54060: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x54064: str r8, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,4)?)?;
    // 0x54068: add r1, r5, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,80)?)?;
    // 0x5406c: add lr, r4, #252
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(4)?,252)?)?;
    // 0x54070: ldr r0, [r5, #0xa78]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2680)?)?;
    // 0x54074: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x54078: ldr r0, [r5, #0xa7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2684)?)?;
    // 0x5407c: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x54080: add r0, r5, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,76)?)?;
    // 0x54084: ldr r2, [r5, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2104)?)?;
    // 0x54088: ldrb r2, [r2, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,336)?)?;
    // 0x5408c: ands r2, r2, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R2,1,true)?)?;
    // 0x54090: mov r2, r1
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // 0x54094: moveq r2, r0
    c.emit(arm32::mov_register(C::Eq,R::R2,R::R0))?;
    // 0x54098: moveq r0, r1
    c.emit(arm32::mov_register(C::Eq,R::R0,R::R1))?;
    // 0x5409c: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x540a0: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x540a4: str r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,16)?)?;
    // 0x540a8: mov r2, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,22)?)?;
    // 0x540ac: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x540b0: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // 0x540b4: mov r0, #91
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,91)?)?;
    // 0x540b8: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x540bc: ldrsb r0, [r5, r0]
    c.emit(load_signed_byte_register(C::Al,R::R0,R::new(5)?,R::R0)?)?;
    // 0x540c0: str r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // 0x540c4: ldrb r0, [r5, #0x138]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,312)?)?;
    // 0x540c8: str r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,36)?)?;
    // 0x540cc: mov r0, #320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,320)?)?;
    // 0x540d0: str r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // 0x540d4: ldr r0, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,40)?)?;
    // 0x540d8: str r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,52)?)?;
    // 0x540dc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x540e0: str r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // 0x540e4: ldrb r0, [r5, #0x142]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,322)?)?;
    // 0x540e8: str r0, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,72)?)?;
    // 0x540ec: ldrb r0, [r5, #0x141]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,321)?)?;
    // 0x540f0: str r0, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,60)?)?;
    // 0x540f4: ldrb r0, [r5, #0x140]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,320)?)?;
    // 0x540f8: str r0, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,64)?)?;
    // 0x540fc: mov r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,63)?)?;
    // 0x54100: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x54104: ldrsb r0, [r5, r0]
    c.emit(load_signed_byte_register(C::Al,R::R0,R::new(5)?,R::R0)?)?;
    // 0x54108: str r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // 0x5410c: ldrb r0, [r5, #0x13b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,315)?)?;
    // 0x54110: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // 0x54114: ldrb r0, [r5, #0x13d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,317)?)?;
    // 0x54118: str r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // 0x5411c: ldrb r0, [r5, #0x13c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,316)?)?;
    // 0x54120: str r0, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,84)?)?;
    // 0x54124: ldrb r0, [r5, #0x13e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,318)?)?;
    // 0x54128: str r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,88)?)?;
    // 0x5412c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x54130: str r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,92)?)?;
    // 0x54134: str r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,96)?)?;
    // 0x54138: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5413c: str r10, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,100)?)?;
    // 0x54140: str r10, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,104)?)?;
    // 0x54144: str r6, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,108)?)?;
    // 0x54148: str r8, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,112)?)?;
    // 0x5414c: str r8, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,116)?)?;
    // 0x54150: str r0, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,120)?)?;
    // 0x54154: str r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,124)?)?;
    // 0x54158: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x5415c: str r8, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,220)?)?;
    // 0x54160: str r10, [r4, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,224)?)?;
    // 0x54164: str r0, [r4, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // 0x54168: str r3, [r4, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,232)?)?;
    // 0x5416c: str r6, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,236)?)?;
    // 0x54170: str r2, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,240)?)?;
    // 0x54174: str r1, [r4, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,244)?)?;
    // 0x54178: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // 0x5417c: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x54180: mov r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // 0x54184: mov r6, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,9)?)?;
    // 0x54188: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x5418c: str r12, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,248)?)?;
    // 0x54190: mov r12, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,14)?)?;
    // 0x54194: stm lr, {r1, r2, r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::LR,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x54198: str r0, [r4, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,268)?)?;
    // 0x5419c: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x541a0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x541a4: str r12, [r4, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,272)?)?;
    // 0x541a8: mov r3, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,19)?)?;
    // 0x541ac: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x541b0: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x541b4: mov r12, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,6)?)?;
    // 0x541b8: mov lr, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,15)?)?;
    // 0x541bc: str r2, [r0, #0x114]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R2,R::R0,276)?)?;
    // 0x541c0: mov r2, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,18)?)?;
    // 0x541c4: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x541c8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x541cc: str r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,280)?)?;
    // 0x541d0: str r7, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,284)?)?;
    // 0x541d4: str r2, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,288)?)?;
    // 0x541d8: str r3, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,292)?)?;
    // 0x541dc: str r1, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,296)?)?;
    // 0x541e0: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x541e4: str r6, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,300)?)?;
    // 0x541e8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x541ec: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x541f0: mov r3, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    // 0x541f4: mov r7, r4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // 0x541f8: str r1, [r4, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,304)?)?;
    // 0x541fc: str r12, [r4, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,308)?)?;
    // 0x54200: str r8, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,404)?)?;
    // 0x54204: str r10, [r4, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,408)?)?;
    // 0x54208: str r6, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,412)?)?;
    // 0x5420c: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x54210: mov r1, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    // 0x54214: str r9, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,416)?)?;
    // 0x54218: mov r12, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,9)?)?;
    // 0x5421c: mov r9, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,6)?)?;
    // 0x54220: str r6, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,420)?)?;
    // 0x54224: str r1, [r4, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,424)?)?;
    // 0x54228: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x5422c: str r1, [r4, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,428)?)?;
    // 0x54230: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x54234: str r1, [r4, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,432)?)?;
    // 0x54238: str lr, [r4, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,436)?)?;
    // 0x5423c: str r0, [r4, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,440)?)?;
    // 0x54240: mov lr, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,7)?)?;
    // 0x54244: mov r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,17)?)?;
    // 0x54248: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x5424c: str lr, [r4, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,444)?)?;
    // 0x54250: str r2, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,448)?)?;
    // 0x54254: str r12, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,452)?)?;
    // 0x54258: str r3, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,456)?)?;
    // 0x5425c: str r0, [r7, #0x1cc]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,460)?)?;
    // 0x54260: mov r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,18)?)?;
    // 0x54264: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x54268: str r0, [r4, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,464)?)?;
    // 0x5426c: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x54270: mov r7, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,19)?)?;
    // 0x54274: str r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,468)?)?;
    // 0x54278: str r1, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,472)?)?;
    // 0x5427c: str r7, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,476)?)?;
    // 0x54280: mov r7, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,20)?)?;
    // 0x54284: str r7, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,480)?)?;
    // 0x54288: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x5428c: str r7, [r4, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,484)?)?;
    // 0x54290: mov r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,5)?)?;
    // 0x54294: str r7, [r4, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,488)?)?;
    // 0x54298: str r9, [r4, #0x1ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,492)?)?;
    // 0x5429c: str r8, [r4, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,496)?)?;
    // 0x542a0: str r10, [r4, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,500)?)?;
    // 0x542a4: str r7, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,504)?)?;
    // 0x542a8: mov r7, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,21)?)?;
    // 0x542ac: mov r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,16)?)?;
    // 0x542b0: str r7, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,508)?)?;
    // 0x542b4: str r6, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,512)?)?;
    // 0x542b8: mov r6, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,22)?)?;
    // 0x542bc: mov r7, r4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // 0x542c0: str r6, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,516)?)?;
    // 0x542c4: str r2, [r4, #0x208]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,520)?)?;
    // 0x542c8: str r12, [r4, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,524)?)?;
    // 0x542cc: str r3, [r4, #0x210]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,528)?)?;
    // 0x542d0: str lr, [r4, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,532)?)?;
    // 0x542d4: str r0, [r4, #0x218]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,536)?)?;
    // 0x542d8: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x542dc: str r1, [r4, #0x21c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,540)?)?;
    // 0x542e0: mov r12, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,17)?)?;
    // 0x542e4: mov r1, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,18)?)?;
    // 0x542e8: mov r3, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,19)?)?;
    // 0x542ec: mov lr, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3)?)?;
    // 0x542f0: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x542f4: mov r6, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,22)?)?;
    // 0x542f8: str r0, [r4, #0x220]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,544)?)?;
    // 0x542fc: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x54300: str r0, [r4, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,548)?)?;
    // 0x54304: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x54308: str r0, [r7, #0x228]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,552)?)?;
    // 0x5430c: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x54310: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x54314: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x54318: str r9, [r4, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,556)?)?;
    // 0x5431c: str r12, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,560)?)?;
    // 0x54320: str r1, [r4, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,564)?)?;
    // 0x54324: str r3, [r4, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,568)?)?;
    // 0x54328: str r0, [r4, #0x23c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,572)?)?;
    // 0x5432c: str lr, [r4, #0x240]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,576)?)?;
    // 0x54330: str r7, [r4, #0x244]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,580)?)?;
    // 0x54334: str r2, [r4, #0x248]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,584)?)?;
    // 0x54338: str r8, [r4, #0x24c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,588)?)?;
    // 0x5433c: str r10, [r4, #0x250]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,592)?)?;
    // 0x54340: str r2, [r4, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,596)?)?;
    // 0x54344: mov r2, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,21)?)?;
    // 0x54348: str r2, [r4, #0x258]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,600)?)?;
    // 0x5434c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x54350: str r2, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,604)?)?;
    // 0x54354: str r6, [r4, #0x260]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,608)?)?;
    // 0x54358: str r1, [r4, #0x264]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // 0x5435c: str r3, [r4, #0x268]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,616)?)?;
    // 0x54360: str r0, [r4, #0x26c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,620)?)?;
    // 0x54364: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x54368: str r12, [r4, #0x270]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,624)?)?;
    // 0x5436c: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // 0x54370: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x54374: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x54378: str r0, [r4, #0x274]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,628)?)?;
    // 0x5437c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x54380: str r0, [r4, #0x278]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,632)?)?;
    // 0x54384: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x54388: str r0, [r4, #0x27c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,636)?)?;
    // 0x5438c: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x54390: str r1, [r4, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,640)?)?;
    // 0x54394: str r0, [r6, #0x284]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(6)?,644)?)?;
    // 0x54398: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x5439c: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x543a0: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x543a4: add r1, r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,16)?)?;
    // 0x543a8: str r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // 0x543ac: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x543b0: str r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // 0x543b4: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x543b8: stm r1, {r0, r9, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(9)?.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x543bc: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x543c0: add r1, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,32)?)?;
    // 0x543c4: str r7, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,28)?)?;
    // 0x543c8: mov r7, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,100)?)?;
    // 0x543cc: stm r1, {r0, r8, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(8)?.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x543d0: mov r0, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,250)?)?;
    // 0x543d4: str r10, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,44)?)?;
    // 0x543d8: str r10, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,48)?)?;
    // 0x543dc: str r10, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,52)?)?;
    // 0x543e0: str r10, [r6, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,56)?)?;
    // 0x543e4: str r10, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,60)?)?;
    // 0x543e8: str r10, [r6, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,64)?)?;
    // 0x543ec: str r10, [r6, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,68)?)?;
    // 0x543f0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x543f4: str r0, [r6, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,72)?)?;
    // 0x543f8: add r0, r6, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,80)?)?;
    // 0x543fc: str r7, [r6, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,76)?)?;
 c.check(0x54400)
}

/// Populate the remaining controller register fields, including exact
/// hardware codec IDs and default padding/capability flags.
#[inline(never)]fn write_remaining_controller_properties(c:&mut Code)->Result<(),String>{
 c.check(0x54400)?;
    // 0x54400: bl 0x31ec <entry+0xfffffffffffaf8a8> @ imm = #-0x5121c
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x54404: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x54408: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5440c: mov r2, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // 0x54410: mov r9, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,2)?)?;
    // 0x54414: str r0, [r6, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,116)?)?;
    // 0x54418: mov r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,18)?)?;
    // 0x5441c: str r8, [r6, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,120)?)?;
    // 0x54420: str r8, [r6, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,124)?)?;
    // 0x54424: str r8, [r6, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,128)?)?;
    // 0x54428: str r8, [r6, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,132)?)?;
    // 0x5442c: str r0, [r6, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,136)?)?;
    // 0x54430: str r0, [r6, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,140)?)?;
    // 0x54434: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x54438: str r0, [r6, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,144)?)?;
    // 0x5443c: str r0, [r6, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,148)?)?;
    // 0x54440: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x54444: str r10, [r6, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,152)?)?;
    // 0x54448: str r8, [r6, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,156)?)?;
    // 0x5444c: str r8, [r6, #-0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,-596)?)?;
    // 0x54450: str r8, [r6, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,160)?)?;
    // 0x54454: str r8, [r6, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,164)?)?;
    // 0x54458: str r10, [r6, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,168)?)?;
    // 0x5445c: str r8, [r6, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,172)?)?;
    // 0x54460: str r8, [r6, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,176)?)?;
    // 0x54464: str r0, [r6, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,180)?)?;
    // 0x54468: str r0, [r6, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,184)?)?;
    // 0x5446c: add r0, r6, #188
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,188)?)?;
    // 0x54470: bl 0x31ec <entry+0xfffffffffffaf8a8> @ imm = #-0x5128c
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x54474: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x54478: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x5447c: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x54480: str r1, [r6, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,216)?)?;
    // 0x54484: str r7, [r6, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,220)?)?;
    // 0x54488: str r7, [r6, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,224)?)?;
    // 0x5448c: str r8, [r6, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,228)?)?;
    // 0x54490: str r0, [r6, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,232)?)?;
    // 0x54494: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x54498: str r0, [r6, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,236)?)?;
    // 0x5449c: str r0, [r6, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,240)?)?;
    // 0x544a0: mov r0, #424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,424)?)?;
    // 0x544a4: str r8, [r6, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,244)?)?;
    // 0x544a8: str r2, [r6, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,248)?)?;
    // 0x544ac: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x544b0: str r8, [r6, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,252)?)?;
    // 0x544b4: str r8, [r6, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,256)?)?;
    // 0x544b8: str r8, [r6, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,260)?)?;
    // 0x544bc: str r1, [r6, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,264)?)?;
    // 0x544c0: str r1, [r6, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,268)?)?;
    // 0x544c4: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x544c8: orr r0, r0, #24576
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,24576)?)?;
    // 0x544cc: str r2, [r6, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,272)?)?;
    // 0x544d0: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x544d4: str r0, [r6, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,276)?)?;
    // 0x544d8: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x544dc: str r8, [r6, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,280)?)?;
    // 0x544e0: str r8, [r6, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,284)?)?;
    // 0x544e4: str r10, [r6, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,288)?)?;
    // 0x544e8: str r9, [r6, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(6)?,292)?)?;
    // 0x544ec: str r10, [r6, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,296)?)?;
    // 0x544f0: str r10, [r6, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,300)?)?;
    // 0x544f4: str r9, [r6, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(6)?,304)?)?;
    // 0x544f8: str r10, [r6, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,308)?)?;
    // 0x544fc: str r10, [r6, #0x138]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,312)?)?;
    // 0x54500: str r1, [r6, #0x13c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,316)?)?;
    // 0x54504: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x54508: str r0, [r6, #0x140]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,320)?)?;
    // 0x5450c: add r0, r6, #324
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,324)?)?;
    // 0x54510: bl 0x31ec <entry+0xfffffffffffaf8a8> @ imm = #-0x5132c
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x54514: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x54518: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x5451c: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x54520: str r0, [r6, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,348)?)?;
    // 0x54524: str r8, [r6, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,352)?)?;
    // 0x54528: str r1, [r6, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,356)?)?;
    // 0x5452c: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x54530: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x54534: mul r0, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::R2)?)?;
    // 0x54538: cmp r0, #921600
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,921600)?)?;
    // 0x5453c: bhi 0x54554 <entry+0xc10>   @ imm = #0x10
    c.emit(arm32::branch(C::Hi,c.at as u32,0x54554,false)?)?;
    // 0x54540: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // 0x54544: ldr r2, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,12)?)?;
    // 0x54548: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // 0x5454c: cmp r0, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x54550: bhs 0x54564 <entry+0xc20>   @ imm = #0xc
    c.emit(arm32::branch(C::Cs,c.at as u32,0x54564,false)?)?;
    // 0x54554: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x54558: str r0, [r4, #0x324]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,804)?)?;
    // 0x5455c: str r8, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,980)?)?;
    // 0x54560: str r8, [r4, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,984)?)?;
    // 0x54564: ldrb r0, [r5, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,347)?)?;
    // 0x54568: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5456c: beq 0x54598 <entry+0xc54>   @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54598,false)?)?;
    // 0x54570: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x54574: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x54578: str r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // 0x5457c: str r7, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,84)?)?;
    // 0x54580: bl 0x2f8ac <entry+0xfffffffffffdbf68> @ imm = #-0x24cdc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f8ac,true)?)?;
    // 0x54584: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x54588: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5458c: movne r1, #14
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x54590: str r1, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,88)?)?;
    // 0x54594: str r7, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,68)?)?;
    // 0x54598: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x5459c: ldr r2, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,12)?)?;
    // 0x545a0: mul r1, r0, r2
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::R2)?)?;
    // 0x545a4: ldr r2, [pc, #0x2a4]        @ 0xf0c <entry+0xf0c>
    c.literal(C::Al,R::R2,0x54850)?;
    // 0x545a8: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x545ac: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x545b0: bhi 0x545e0 <entry+0xc9c>   @ imm = #0x28
    c.emit(arm32::branch(C::Hi,c.at as u32,0x545e0,false)?)?;
    // 0x545b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x545b8: cmp r0, r1, lsr #16
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsr,16,false)?)?;
    // 0x545bc: beq 0x545e0 <entry+0xc9c>   @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x545e0,false)?)?;
    // 0x545c0: lsr r2, r1, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsr,16)?)?;
    // 0x545c4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x545c8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x545cc: lsr r3, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsr,1)?)?;
    // 0x545d0: cmp r1, r2, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsr,1,false)?)?;
    // 0x545d4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x545d8: mov r2, r3
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // 0x545dc: bne 0x545cc <entry+0xc88>   @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x545cc,false)?)?;
    // 0x545e0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x545e4: str r1, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,972)?)?;
    // 0x545e8: str r0, [r4, #0x3d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,976)?)?;
    // 0x545ec: str r0, [r5, #0xa30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2608)?)?;
    // 0x545f0: ldrb r0, [r5, #0x9a0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2464)?)?;
    // 0x545f4: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x545f8: strne r1, [r4, #0x334]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(4)?,820)?)?;
    // 0x545fc: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x54600: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54604: beq 0x54848 <entry+0xf04>   @ imm = #0x23c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54848,false)?)?;
    // 0x54608: ldrb r0, [r5, #0x142]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,322)?)?;
    // 0x5460c: mov r1, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,48)?)?;
    // 0x54610: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x54614: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x54618: mov r9, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,14)?)?;
    // 0x5461c: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x54620: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x54624: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // 0x54628: mov r5, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,12)?)?;
    // 0x5462c: mov lr, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,13)?)?;
    // 0x54630: mov r8, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,9)?)?;
    // 0x54634: str r0, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,72)?)?;
    // 0x54638: mov r0, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,96)?)?;
    // 0x5463c: str r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,92)?)?;
    // 0x54640: str r1, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,96)?)?;
    // 0x54644: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x54648: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5464c: str r10, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,104)?)?;
    // 0x54650: str r10, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,108)?)?;
    // 0x54654: str r7, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,120)?)?;
    // 0x54658: str r9, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,124)?)?;
    // 0x5465c: str r12, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,220)?)?;
    // 0x54660: str r10, [r4, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,224)?)?;
    // 0x54664: str r3, [r4, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,228)?)?;
    // 0x54668: str r2, [r4, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,232)?)?;
    // 0x5466c: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x54670: str r1, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,236)?)?;
    // 0x54674: str r5, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,240)?)?;
    // 0x54678: str r0, [r4, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,244)?)?;
    // 0x5467c: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x54680: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x54684: mov r5, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,5)?)?;
    // 0x54688: str lr, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,248)?)?;
    // 0x5468c: str r8, [r4, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,252)?)?;
    // 0x54690: str r9, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,256)?)?;
    // 0x54694: str r7, [r4, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,260)?)?;
    // 0x54698: str r1, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,264)?)?;
    // 0x5469c: str r0, [r4, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,268)?)?;
    // 0x546a0: str r5, [r4, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,272)?)?;
    // 0x546a4: str r12, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,404)?)?;
    // 0x546a8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x546ac: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x546b0: bl 0x31ec <entry+0xfffffffffffaf8a8> @ imm = #-0x514cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x546b4: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x546b8: str r10, [r4, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,408)?)?;
    // 0x546bc: add r1, r4, #424
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,424)?)?;
    // 0x546c0: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x546c4: str r0, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,412)?)?;
    // 0x546c8: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x546cc: str r0, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,416)?)?;
    // 0x546d0: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x546d4: str r0, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,420)?)?;
    // 0x546d8: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x546dc: stm r1, {r0, r8, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(8)?.bit()|R::new(9)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x546e0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x546e4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x546e8: str r0, [r4, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,436)?)?;
    // 0x546ec: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x546f0: str r0, [r4, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,440)?)?;
    // 0x546f4: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x546f8: str r7, [r4, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,444)?)?;
    // 0x546fc: str r0, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,448)?)?;
    // 0x54700: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x54704: str r0, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,452)?)?;
    // 0x54708: str r5, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,456)?)?;
    // 0x5470c: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x54710: bl 0x31ec <entry+0xfffffffffffaf8a8> @ imm = #-0x5152c
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x54714: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x54718: str r10, [r4, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,500)?)?;
    // 0x5471c: add r1, r4, #508
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,508)?)?;
    // 0x54720: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x54724: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // 0x54728: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x5472c: stm r1, {r0, r8, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(8)?.bit()|R::new(9)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x54730: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x54734: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x54738: str r0, [r4, #0x208]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,520)?)?;
    // 0x5473c: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x54740: str r0, [r4, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,524)?)?;
    // 0x54744: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x54748: str r0, [r4, #0x210]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,528)?)?;
    // 0x5474c: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x54750: str r0, [r4, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,532)?)?;
    // 0x54754: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x54758: str r7, [r4, #0x218]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,536)?)?;
    // 0x5475c: str r0, [r4, #0x21c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,540)?)?;
    // 0x54760: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x54764: str r0, [r4, #0x220]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,544)?)?;
    // 0x54768: str r5, [r4, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,548)?)?;
    // 0x5476c: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x54770: bl 0x31ec <entry+0xfffffffffffaf8a8> @ imm = #-0x5158c
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x54774: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x54778: str r10, [r4, #0x250]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,592)?)?;
    // 0x5477c: str r8, [r4, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,596)?)?;
    // 0x54780: str r9, [r4, #0x258]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,600)?)?;
    // 0x54784: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x54788: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x5478c: mov r8, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,7)?)?;
    // 0x54790: mov r9, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3)?)?;
    // 0x54794: str r0, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,604)?)?;
    // 0x54798: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x5479c: str r0, [r4, #0x260]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,608)?)?;
    // 0x547a0: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x547a4: str r0, [r4, #0x264]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,612)?)?;
    // 0x547a8: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x547ac: str r0, [r4, #0x268]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,616)?)?;
    // 0x547b0: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x547b4: str r0, [r4, #0x26c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,620)?)?;
    // 0x547b8: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x547bc: str r0, [r4, #0x270]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,624)?)?;
    // 0x547c0: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x547c4: str r7, [r4, #0x274]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,628)?)?;
    // 0x547c8: str r0, [r4, #0x278]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,632)?)?;
    // 0x547cc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x547d0: str r0, [r4, #0x27c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,636)?)?;
    // 0x547d4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x547d8: str r5, [r4, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,640)?)?;
    // 0x547dc: bl 0x31ec <entry+0xfffffffffffaf8a8> @ imm = #-0x515f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x547e0: mov r1, #420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,420)?)?;
    // 0x547e4: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x547e8: mov r2, #500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,500)?)?;
    // 0x547ec: str r0, [r4, #0x2ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,684)?)?;
    // 0x547f0: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x547f4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x547f8: str r7, [r4, #0x2b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,688)?)?;
    // 0x547fc: str r7, [r4, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,692)?)?;
    // 0x54800: str r8, [r4, #0x2c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,704)?)?;
    // 0x54804: str r0, [r4, #0x2c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,708)?)?;
    // 0x54808: str r5, [r4, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,712)?)?;
    // 0x5480c: str r1, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,716)?)?;
    // 0x54810: str r2, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,720)?)?;
    // 0x54814: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x54818: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x5481c: str r9, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,928)?)?;
    // 0x54820: str r10, [r4, #0x338]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,824)?)?;
    // 0x54824: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x54828: str r2, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,980)?)?;
    // 0x5482c: str r2, [r4, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,984)?)?;
    // 0x54830: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x54834: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x54838: str r2, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,988)?)?;
    // 0x5483c: str r0, [r4, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,992)?)?;
    // 0x54840: str r2, [r4, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,996)?)?;
    // 0x54844: str r1, [r4, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1000)?)?;
    // 0x54848: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x5484c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x54850)?;
 c.emit(CODEC_FIELD_MASK)?;
 c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();write_default_controller_properties(&mut c)?;
 write_remaining_controller_properties(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn controller_initialization_includes_exact_mask_word(){
   let b=fragment().unwrap();assert_eq!(b.len(),END-START);
   assert_eq!(u32::from_le_bytes(b[0x54850-START..].try_into().unwrap()),CODEC_FIELD_MASK);
 }
 #[test]fn check_private_firmware_when_available(){
  let b=fragment().unwrap();
  if let Ok(path)=std::env::var("NABU_VENUS_REFERENCE").or_else(|_| std::env::var("AGENT04_PRIVATE_VENUS")){
    let ref_bytes=std::fs::read(path).unwrap();
    let orig=&ref_bytes[FILE_OFFSET..FILE_OFFSET+b.len()];
    if let Some(i)=b.iter().zip(orig).position(|(a,c)|a!=c){
      panic!("byte mismatch at {:#x}: {:02x} vs {:02x}",START+i,b[i],orig[i]);
    }
  }
  if let Ok(path)=std::env::var("AGENT04_EMIT_FRAGMENT"){
    std::fs::write(path,b).unwrap();
  }
 }
}
