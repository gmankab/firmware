// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x61000,0x62000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x61000;
pub const END:usize=0x62000;
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
fn emit_halfword_postindexed(cond:C,rd:R,rn:R,offset:i32)->Result<u32,String>{
 if rd==rn || [rd,rn].contains(&R::PC) {return Err("invalid A32 postindexed halfword load operands".into())}
 Ok(arm32::store_halfword_postindexed(cond,rd,rn,offset)? | (1<<20))
}
fn emit_umull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 UMULL operands".into())}
 Ok(((cond as u32)<<28)|0x0080_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
fn emit_adc_shifted(cond:C,rd:R,rn:R,rm:R,shift:Shift,amount:u8,flags:bool)->Result<u32,String>{
 Ok(arm32::data_register(cond,Op::Add,rd,rn,rm,shift,amount,flags)? | (1<<21))
}
fn emit_smull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 SMULL operands".into())}
 Ok(((cond as u32)<<28)|0x00c0_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x61000..0x610d8.
 c.check(0x61000)?;
    // 0x61000: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x61004: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32378
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x61008: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x6100c: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x5880
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x61010: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x61014: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x61018: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x6101c: ldr r0, [pc, #0xbc]         @ 0xb8e0 <entry+0xb8e0>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,188)?)?;
    // 0x61020: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x61024: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x61028: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x6102c: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x61030: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x61034: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x61038: bne 0x61050 <entry+0xb850>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61050,false)?)?;
    // 0x6103c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x61040: bl 0x5a710 <entry+0x4f10>  @ imm = #-0x6938
    c.emit(arm32::branch(C::Al,c.at as u32,0x5a710,true)?)?;
    // 0x61044: bl 0x20a0 <entry+0xfffffffffffac8a0> @ imm = #-0x5efac
    c.emit(arm32::branch(C::Al,c.at as u32,0x20a0,true)?)?;
    // 0x61048: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // 0x6104c: b 0x610c0 <entry+0xb8c0>  @ imm = #0x6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x610c0,false)?)?;
    // 0x61050: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x61054: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x61058: beq 0x61094 <entry+0xb894>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x61094,false)?)?;
    // 0x6105c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x61060: ldr r1, [pc, #0x7c]         @ 0xb8e4 <entry+0xb8e4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,124)?)?;
    // 0x61064: ldr r2, [pc, #0x7c]         @ 0xb8e8 <entry+0xb8e8>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,124)?)?;
    // 0x61068: mov r3, #145
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,145)?)?;
    // 0x6106c: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x61070: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x61074: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x61078: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x323ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x6107c: ldr r1, [pc, #0x58]         @ 0xb8dc <entry+0xb8dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,88)?)?;
    // 0x61080: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x61084: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x323f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x61088: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x6108c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x61090: b 0x610bc <entry+0xb8bc>  @ imm = #0x24
    c.emit(arm32::branch(C::Al,c.at as u32,0x610bc,false)?)?;
    // 0x61094: ldr r1, [pc, #0x50]         @ 0xb8ec <entry+0xb8ec>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,80)?)?;
    // 0x61098: ldr r2, [pc, #0x48]         @ 0xb8e8 <entry+0xb8e8>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,72)?)?;
    // 0x6109c: mov r3, #140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,140)?)?;
    // 0x610a0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x610a4: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x610a8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3241c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x610ac: ldr r1, [pc, #0x28]         @ 0xb8dc <entry+0xb8dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,40)?)?;
    // 0x610b0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x610b4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32428
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x610b8: ldr r0, [pc, #0x30]         @ 0xb8f0 <entry+0xb8f0>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,48)?)?;
    // 0x610bc: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x610c0: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x610c4: add r1, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
    // 0x610c8: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x36858
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x610cc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x610d0: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x610d4: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x610d8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_610D8:[u32;7]=[0x000f11b5,0x000e2788,0x0022100a,0x000f121f,0x000f1202,0x000f11c7,0x01001002];
 for value in INLINE_DATA_610D8{c.emit(value)?;}
 // Executable A32 block 0x610f4..0x61294.
 c.check(0x610f4)?;
    // 0x610f4: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x610f8: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x610fc: sub sp, sp, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,2064)?)?;
    // 0x61100: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x61104: ldr r1, [pc, #0x4d8]        @ 0xbde4 <entry+0xbde4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1240)?)?;
    // 0x61108: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x6110c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x61110: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32484
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x61114: ldr r1, [pc, #0x4cc]        @ 0xbde8 <entry+0xbde8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1228)?)?;
    // 0x61118: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x6111c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32490
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x61120: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // 0x61124: ldr r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // 0x61128: bic r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,1)?)?;
    // 0x6112c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x61130: bne 0x61178 <entry+0xb978>  @ imm = #0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61178,false)?)?;
    // 0x61134: cmp r0, #536870912
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,536870912)?)?;
    // 0x61138: blo 0x611bc <entry+0xb9bc>  @ imm = #0x7c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x611bc,false)?)?;
    // 0x6113c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x61140: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x61144: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x61148: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x6114c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x61150: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x61154: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x61158: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x6115c: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x61160: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x367d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x61164: ldr r1, [pc, #0x488]        @ 0xbdf4 <entry+0xbdf4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1160)?)?;
    // 0x61168: ldr r2, [pc, #0x480]        @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1152)?)?;
    // 0x6116c: mov r3, #175
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,175)?)?;
    // 0x61170: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x61174: b 0x613a8 <entry+0xbba8>  @ imm = #0x22c
    c.emit(arm32::branch(C::Al,c.at as u32,0x613a8,false)?)?;
    // 0x61178: cmp r0, #1073741824
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1073741824)?)?;
    // 0x6117c: blo 0x61204 <entry+0xba04>  @ imm = #0x80
    c.emit(arm32::branch(C::Cc,c.at as u32,0x61204,false)?)?;
    // 0x61180: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x61184: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x61188: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x6118c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x61190: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x61194: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x61198: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x6119c: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x611a0: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x611a4: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x3681c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x611a8: ldr r1, [pc, #0x444]        @ 0xbdf4 <entry+0xbdf4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1092)?)?;
    // 0x611ac: ldr r2, [pc, #0x43c]        @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1084)?)?;
    // 0x611b0: mov r3, #196
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,196)?)?;
    // 0x611b4: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x611b8: b 0x613a8 <entry+0xbba8>  @ imm = #0x1e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x613a8,false)?)?;
    // 0x611bc: lsl r0, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,3)?)?;
    // 0x611c0: cmn r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,32)?)?;
    // 0x611c4: blo 0x6124c <entry+0xba4c>  @ imm = #0x80
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6124c,false)?)?;
    // 0x611c8: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x611cc: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x611d0: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x611d4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x611d8: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x611dc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x611e0: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x611e4: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x611e8: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x611ec: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x36864
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x611f0: ldr r1, [pc, #0x3f4]        @ 0xbdec <entry+0xbdec>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1012)?)?;
    // 0x611f4: ldr r2, [pc, #0x3f4]        @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1012)?)?;
    // 0x611f8: mov r3, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,183)?)?;
    // 0x611fc: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x61200: b 0x613a8 <entry+0xbba8>  @ imm = #0x1a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x613a8,false)?)?;
    // 0x61204: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x61208: cmn r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,32)?)?;
    // 0x6120c: blo 0x6124c <entry+0xba4c>  @ imm = #0x38
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6124c,false)?)?;
    // 0x61210: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x61214: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x61218: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x6121c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x61220: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x61224: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x61228: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x6122c: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x61230: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x61234: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x368ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x61238: ldr r1, [pc, #0x3ac]        @ 0xbdec <entry+0xbdec>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,940)?)?;
    // 0x6123c: ldr r2, [pc, #0x3ac]        @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,940)?)?;
    // 0x61240: mov r3, #204
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,204)?)?;
    // 0x61244: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x61248: b 0x613a8 <entry+0xbba8>  @ imm = #0x158
    c.emit(arm32::branch(C::Al,c.at as u32,0x613a8,false)?)?;
    // 0x6124c: add r7, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,32)?)?;
    // 0x61250: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x61254: cmp r0, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x61258: bne 0x612e8 <entry+0xbae8>  @ imm = #0x88
    c.emit(arm32::branch(C::Ne,c.at as u32,0x612e8,false)?)?;
    // 0x6125c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x61260: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x5ad4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x61264: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x61268: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6126c: ldrne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,0)?)?;
    // 0x61270: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x61274: bne 0x6132c <entry+0xbb2c>  @ imm = #0xb0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6132c,false)?)?;
    // 0x61278: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // 0x6127c: sub r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,2)?)?;
    // 0x61280: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x61284: bhi 0x6136c <entry+0xbb6c>  @ imm = #0xe0
    c.emit(arm32::branch(C::Hi,c.at as u32,0x6136c,false)?)?;
    // 0x61288: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x6128c: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x61290: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x61294)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_61294:[u32;7]=[0x000612b0,0x0006136c,0x000613c8,0x0006136c,0x0006140c,0x00061450,0x000614a4];
 for value in INLINE_DATA_61294{c.emit(value)?;}
 // Executable A32 block 0x612b0..0x615e4.
 c.check(0x612b0)?;
    // 0x612b0: ldr r1, [pc, #0x358]        @ 0xbe10 <entry+0xbe10>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,856)?)?;
    // 0x612b4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x612b8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3262c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x612bc: ldr r1, [pc, #0x324]        @ 0xbde8 <entry+0xbde8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,804)?)?;
    // 0x612c0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x612c4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32638
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x612c8: ldr r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // 0x612cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x612d0: beq 0x614e4 <entry+0xbce4>  @ imm = #0x20c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x614e4,false)?)?;
    // 0x612d4: add r0, r4, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,124)?)?;
    // 0x612d8: bl 0x7e7d4 <entry+0x28fd4> @ imm = #0x1d4f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e7d4,true)?)?;
    // 0x612dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x612e0: str r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // 0x612e4: b 0x614e4 <entry+0xbce4>  @ imm = #0x1f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x614e4,false)?)?;
    // 0x612e8: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x612ec: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x612f0: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x612f4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x612f8: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x612fc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x61300: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x61304: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x61308: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x6130c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x36984
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x61310: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x61314: ldr r1, [pc, #0x2dc]        @ 0xbdf8 <entry+0xbdf8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,732)?)?;
    // 0x61318: ldr r2, [pc, #0x2d0]        @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,720)?)?;
    // 0x6131c: mov r3, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,216)?)?;
    // 0x61320: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x61324: stm sp, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x61328: b 0x613a8 <entry+0xbba8>  @ imm = #0x78
    c.emit(arm32::branch(C::Al,c.at as u32,0x613a8,false)?)?;
    // 0x6132c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x61330: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x61334: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x61338: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x6133c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x61340: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x61344: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x61348: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x6134c: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x61350: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x369c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x61354: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x61358: ldr r1, [pc, #0x29c]        @ 0xbdfc <entry+0xbdfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,668)?)?;
    // 0x6135c: ldr r2, [pc, #0x28c]        @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,652)?)?;
    // 0x61360: mov r3, #121
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,121)?)?;
    // 0x61364: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x61368: b 0x613a4 <entry+0xbba4>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x613a4,false)?)?;
    // 0x6136c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x61370: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x61374: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x61378: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x6137c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x61380: orr r2, r2, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,131072)?)?;
    // 0x61384: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x61388: stm sp, {r2, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x6138c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x61390: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x61394: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x36a0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x61398: ldr r1, [pc, #0x27c]        @ 0xbe1c <entry+0xbe1c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,636)?)?;
    // 0x6139c: ldr r2, [pc, #0x24c]        @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,588)?)?;
    // 0x613a0: mov r3, #71
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,71)?)?;
    // 0x613a4: orr r3, r3, #9472
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9472)?)?;
    // 0x613a8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x613ac: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32720
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x613b0: ldr r1, [pc, #0x230]        @ 0xbde8 <entry+0xbde8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,560)?)?;
    // 0x613b4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x613b8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3272c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x613bc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x613c0: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x613c4: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x613c8: ldr r1, [pc, #0x23c]        @ 0xbe0c <entry+0xbe0c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,572)?)?;
    // 0x613cc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x613d0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32744
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x613d4: ldr r1, [pc, #0x20c]        @ 0xbde8 <entry+0xbde8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,524)?)?;
    // 0x613d8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x613dc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32750
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x613e0: ldr r0, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,240)?)?;
    // 0x613e4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x613e8: beq 0x614e4 <entry+0xbce4>  @ imm = #0xf4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x614e4,false)?)?;
    // 0x613ec: ldr r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,24)?)?;
    // 0x613f0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x613f4: bne 0x613fc <entry+0xbbfc>  @ imm = #0x0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x613fc,false)?)?;
    // 0x613f8: bl 0x20a0 <entry+0xfffffffffffac8a0> @ imm = #-0x5f360
    c.emit(arm32::branch(C::Al,c.at as u32,0x20a0,true)?)?;
    // 0x613fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61400: str r0, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,240)?)?;
    // 0x61404: str r0, [r4, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,244)?)?;
    // 0x61408: b 0x614e4 <entry+0xbce4>  @ imm = #0xd4
    c.emit(arm32::branch(C::Al,c.at as u32,0x614e4,false)?)?;
    // 0x6140c: ldr r1, [pc, #0x1f4]        @ 0xbe08 <entry+0xbe08>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,500)?)?;
    // 0x61410: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x61414: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32788
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x61418: ldr r1, [pc, #0x1c8]        @ 0xbde8 <entry+0xbde8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,456)?)?;
    // 0x6141c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x61420: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32794
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x61424: ldr r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // 0x61428: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6142c: beq 0x614e4 <entry+0xbce4>  @ imm = #0xb0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x614e4,false)?)?;
    // 0x61430: ldr r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,24)?)?;
    // 0x61434: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x61438: bne 0x61440 <entry+0xbc40>  @ imm = #0x0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61440,false)?)?;
    // 0x6143c: bl 0x20a0 <entry+0xfffffffffffac8a0> @ imm = #-0x5f3a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x20a0,true)?)?;
    // 0x61440: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61444: str r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,248)?)?;
    // 0x61448: str r0, [r4, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,252)?)?;
    // 0x6144c: b 0x614e4 <entry+0xbce4>  @ imm = #0x90
    c.emit(arm32::branch(C::Al,c.at as u32,0x614e4,false)?)?;
    // 0x61450: ldr r1, [pc, #0x1ac]        @ 0xbe04 <entry+0xbe04>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,428)?)?;
    // 0x61454: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x61458: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x327cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x6145c: ldr r1, [pc, #0x184]        @ 0xbde8 <entry+0xbde8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,388)?)?;
    // 0x61460: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x61464: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x327d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x61468: ldr r0, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,256)?)?;
    // 0x6146c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x61470: beq 0x614e4 <entry+0xbce4>  @ imm = #0x6c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x614e4,false)?)?;
    // 0x61474: ldr r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,24)?)?;
    // 0x61478: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x6147c: bne 0x61484 <entry+0xbc84>  @ imm = #0x0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61484,false)?)?;
    // 0x61480: bl 0x20a0 <entry+0xfffffffffffac8a0> @ imm = #-0x5f3e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x20a0,true)?)?;
    // 0x61484: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61488: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x6148c: mov r2, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,48)?)?;
    // 0x61490: str r0, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,256)?)?;
    // 0x61494: str r0, [r4, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,260)?)?;
    // 0x61498: add r0, r4, #2336
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,2336)?)?;
    // 0x6149c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x5e2b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x614a0: b 0x614e4 <entry+0xbce4>  @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x614e4,false)?)?;
    // 0x614a4: ldr r1, [pc, #0x154]        @ 0xbe00 <entry+0xbe00>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,340)?)?;
    // 0x614a8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x614ac: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32820
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x614b0: ldr r1, [pc, #0x130]        @ 0xbde8 <entry+0xbde8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,304)?)?;
    // 0x614b4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x614b8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3282c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x614bc: ldr r0, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,264)?)?;
    // 0x614c0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x614c4: beq 0x614e4 <entry+0xbce4>  @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x614e4,false)?)?;
    // 0x614c8: ldr r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,24)?)?;
    // 0x614cc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x614d0: bne 0x614d8 <entry+0xbcd8>  @ imm = #0x0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x614d8,false)?)?;
    // 0x614d4: bl 0x20a0 <entry+0xfffffffffffac8a0> @ imm = #-0x5f43c
    c.emit(arm32::branch(C::Al,c.at as u32,0x20a0,true)?)?;
    // 0x614d8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x614dc: str r0, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,264)?)?;
    // 0x614e0: str r0, [r4, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,268)?)?;
    // 0x614e4: ldr r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,24)?)?;
    // 0x614e8: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x614ec: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x614f0: bne 0x613bc <entry+0xbbbc>  @ imm = #-0x13c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x613bc,false)?)?;
    // 0x614f4: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // 0x614f8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x614fc: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x61500: ldr r0, [pc, #0x10c]        @ 0xbe14 <entry+0xbe14>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,268)?)?;
    // 0x61504: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x61508: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x6150c: str r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // 0x61510: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x61514: ldr r7, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,28)?)?;
    // 0x61518: cmp r7, #1073741824
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1073741824)?)?;
    // 0x6151c: str r7, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,32)?)?;
    // 0x61520: blo 0x61558 <entry+0xbd58>  @ imm = #0x30
    c.emit(arm32::branch(C::Cc,c.at as u32,0x61558,false)?)?;
    // 0x61524: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x61528: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x6152c: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x61530: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x61534: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x61538: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x6153c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x61540: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x61544: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x36bbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x61548: ldr r1, [pc, #0xc8]         @ 0xbe18 <entry+0xbe18>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,200)?)?;
    // 0x6154c: ldr r2, [pc, #0x9c]         @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,156)?)?;
    // 0x61550: mov r3, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,94)?)?;
    // 0x61554: b 0x61594 <entry+0xbd94>  @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x61594,false)?)?;
    // 0x61558: cmp r7, #508
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,508)?)?;
    // 0x6155c: blo 0x615b4 <entry+0xbdb4>  @ imm = #0x50
    c.emit(arm32::branch(C::Cc,c.at as u32,0x615b4,false)?)?;
    // 0x61560: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x61564: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x61568: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x6156c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x61570: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x61574: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x61578: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x6157c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x61580: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x61584: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x36bfc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x61588: ldr r1, [pc, #0x88]         @ 0xbe18 <entry+0xbe18>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,136)?)?;
    // 0x6158c: ldr r2, [pc, #0x5c]         @ 0xbdf0 <entry+0xbdf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,92)?)?;
    // 0x61590: mov r3, #102
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,102)?)?;
    // 0x61594: orr r3, r3, #9472
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9472)?)?;
    // 0x61598: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x6159c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32910
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x615a0: ldr r1, [pc, #0x40]         @ 0xbde8 <entry+0xbde8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,64)?)?;
    // 0x615a4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x615a8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3291c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x615ac: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x615b0: b 0x613bc <entry+0xbbbc>  @ imm = #-0x1fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x613bc,false)?)?;
    // 0x615b4: add r8, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,16)?)?;
    // 0x615b8: add r1, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,32)?)?;
    // 0x615bc: lsl r2, r7, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x615c0: add r0, r8, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,20)?)?;
    // 0x615c4: bl 0x471c <entry+0xfffffffffffaef1c> @ imm = #-0x5ceb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x471c,true)?)?;
    // 0x615c8: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x615cc: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x615d0: add r0, r0, r7, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x615d4: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x615d8: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x615dc: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x36d6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x615e0: b 0x613bc <entry+0xbbbc>  @ imm = #-0x22c
    c.emit(arm32::branch(C::Al,c.at as u32,0x613bc,false)?)?;
 c.check(0x615e4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_615E4:[u32;15]=[0x000f125e,0x000e2788,0x000f0837,0x000f126e,0x000f07de,0x000f1289,0x000f1408,0x000f1366,0x000f1340,0x000f131c,0x000f12f8,0x000f12de,0x0022100c,0x000f13b7,0x000f138c];
 for value in INLINE_DATA_615E4{c.emit(value)?;}
 // Executable A32 block 0x61620..0x6169c.
 c.check(0x61620)?;
    // 0x61620: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x61624: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x61628: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x6162c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x61630: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x61634: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5ea48
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x61638: ldr r0, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,112)?)?;
    // 0x6163c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x61640: beq 0x61668 <entry+0xbe68>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x61668,false)?)?;
    // 0x61644: ldr r1, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,92)?)?;
    // 0x61648: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x6164c: ldr r0, [r0, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,-4)?)?;
    // 0x61650: ldr r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,24)?)?;
    // 0x61654: orr r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32)?)?;
    // 0x61658: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x6165c: ldr r1, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // 0x61660: str r1, [r0, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,152)?)?;
    // 0x61664: b 0x6168c <entry+0xbe8c>  @ imm = #0x20
    c.emit(arm32::branch(C::Al,c.at as u32,0x6168c,false)?)?;
    // 0x61668: ldr r1, [pc, #0x2c]         @ 0xbe9c <entry+0xbe9c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,44)?)?;
    // 0x6166c: ldr r2, [pc, #0x2c]         @ 0xbea0 <entry+0xbea0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,44)?)?;
    // 0x61670: mov r3, #143
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,143)?)?;
    // 0x61674: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x61678: orr r3, r3, #9472
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9472)?)?;
    // 0x6167c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x329f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x61680: ldr r1, [pc, #0x1c]         @ 0xbea4 <entry+0xbea4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,28)?)?;
    // 0x61684: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x61688: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x329fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x6168c: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x61690: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5ea64
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x61694: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x61698: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x6169c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_6169C:[u32;3]=[0x000f1444,0x000f146b,0x000e2788];
 for value in INLINE_DATA_6169C{c.emit(value)?;}
 // Executable A32 block 0x616a8..0x617e8.
 c.check(0x616a8)?;
    // 0x616a8: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x616ac: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x616b0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x616b4: ldr r0, [r4, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2104)?)?;
    // 0x616b8: ldrb r0, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,336)?)?;
    // 0x616bc: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x616c0: beq 0x61734 <entry+0xbf34>  @ imm = #0x6c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x61734,false)?)?;
    // 0x616c4: ldr r2, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,76)?)?;
    // 0x616c8: ldr r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,80)?)?;
    // 0x616cc: ldrb r1, [r4, #0x71e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1822)?)?;
    // 0x616d0: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // 0x616d4: str r2, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,80)?)?;
    // 0x616d8: ldrb r2, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,348)?)?;
    // 0x616dc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x616e0: beq 0x61718 <entry+0xbf18>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x61718,false)?)?;
    // 0x616e4: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x616e8: rsb r1, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R1,0)?)?;
    // 0x616ec: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x616f0: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x616f4: ldr r1, [pc, #0xec]         @ 0xbfe8 <entry+0xbfe8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,236)?)?;
    // 0x616f8: add r0, r0, #191
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,191)?)?;
    // 0x616fc: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x61700: lsr r0, r3, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,7)?)?;
    // 0x61704: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x61708: lsl r0, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,6)?)?;
    // 0x6170c: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x61710: mvn r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,3)?)?;
    // 0x61714: and r0, r0, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x61718: add r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,127)?)?;
    // 0x6171c: bic r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,127)?)?;
    // 0x61720: str r0, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,668)?)?;
    // 0x61724: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x61728: bl 0x58900 <entry+0x3100>  @ imm = #-0x8e30
    c.emit(arm32::branch(C::Al,c.at as u32,0x58900,true)?)?;
    // 0x6172c: ldr r0, [r4, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2104)?)?;
    // 0x61730: ldrb r0, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,336)?)?;
    // 0x61734: ldr r1, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,668)?)?;
    // 0x61738: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x6173c: str r1, [r4, #0xa50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2640)?)?;
    // 0x61740: ldr r1, [r4, #0x86c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2156)?)?;
    // 0x61744: str r1, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2648)?)?;
    // 0x61748: ldr r1, [r4, #0x868]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2152)?)?;
    // 0x6174c: str r1, [r4, #0xa54]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2644)?)?;
    // 0x61750: ldr r1, [r4, #0x870]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2160)?)?;
    // 0x61754: str r1, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2652)?)?;
    // 0x61758: ldr r1, [r4, #0x874]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2164)?)?;
    // 0x6175c: str r1, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2656)?)?;
    // 0x61760: ldr r1, [r4, #0x878]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2168)?)?;
    // 0x61764: str r1, [r4, #0xa64]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2660)?)?;
    // 0x61768: ldr r1, [r4, #0x2ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,684)?)?;
    // 0x6176c: str r1, [r4, #0xa70]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2672)?)?;
    // 0x61770: ldr r1, [r4, #0x2b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,688)?)?;
    // 0x61774: str r1, [r4, #0xa74]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2676)?)?;
    // 0x61778: ldr r1, [r4, #0x84c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2124)?)?;
    // 0x6177c: str r1, [r4, #0xa68]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2664)?)?;
    // 0x61780: ldr r1, [r4, #0x850]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2128)?)?;
    // 0x61784: str r1, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2668)?)?;
    // 0x61788: popeq {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x6178c: ldr r1, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,76)?)?;
    // 0x61790: ldr r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,80)?)?;
    // 0x61794: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // 0x61798: str r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,80)?)?;
    // 0x6179c: ldrb r1, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,348)?)?;
    // 0x617a0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x617a4: beq 0x617d0 <entry+0xbfd0>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x617d0,false)?)?;
    // 0x617a8: ldr r0, [r4, #0x7dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2012)?)?;
    // 0x617ac: ldr r1, [pc, #0x34]         @ 0xbfe8 <entry+0xbfe8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,52)?)?;
    // 0x617b0: add r0, r0, #191
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,191)?)?;
    // 0x617b4: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x617b8: lsr r0, r3, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,7)?)?;
    // 0x617bc: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x617c0: lsl r0, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,6)?)?;
    // 0x617c4: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x617c8: mvn r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,3)?)?;
    // 0x617cc: and r0, r0, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x617d0: add r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,127)?)?;
    // 0x617d4: bic r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,127)?)?;
    // 0x617d8: str r0, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,668)?)?;
    // 0x617dc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x617e0: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x617e4: b 0x58900 <entry+0x3100>  @ imm = #-0x8eec
    c.emit(arm32::branch(C::Al,c.at as u32,0x58900,false)?)?;
 c.check(0x617e8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_617E8:[u32;1]=[0xaaaaaaab];
 for value in INLINE_DATA_617E8{c.emit(value)?;}
 // Executable A32 block 0x617ec..0x61988.
 c.check(0x617ec)?;
    // 0x617ec: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x617f0: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x617f4: ldr r8, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,44)?)?;
    // 0x617f8: ldr r4, [r8, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,36)?)?;
    // 0x617fc: ldr r1, [r4, #0x670]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1648)?)?;
    // 0x61800: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x61804: beq 0x6194c <entry+0xc14c>  @ imm = #0x140
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6194c,false)?)?;
    // 0x61808: ldr r1, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1672)?)?;
    // 0x6180c: add r2, r4, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x61810: ldr r2, [r2, #0x278]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,632)?)?;
    // 0x61814: cmn r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R2,1)?)?;
    // 0x61818: ldrne r5, [r4, #0x24c]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(5)?,R::new(4)?,588)?)?;
    // 0x6181c: cmpne r5, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x61820: beq 0x61870 <entry+0xc070>  @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x61870,false)?)?;
    // 0x61824: add r3, r4, #656
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,656)?)?;
    // 0x61828: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x6182c: ldr r7, [r3, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,8)?)?;
    // 0x61830: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // 0x61834: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x61838: cmpeq r7, r2
    c.emit(arm32::data_register(C::Eq,Op::Cmp,R::R0,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x6183c: beq 0x61854 <entry+0xc054>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x61854,false)?)?;
    // 0x61840: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x61844: add r3, r3, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,72)?)?;
    // 0x61848: cmp r6, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x6184c: blo 0x6182c <entry+0xc02c>  @ imm = #-0x28
    c.emit(arm32::branch(C::Cc,c.at as u32,0x6182c,false)?)?;
    // 0x61850: b 0x61870 <entry+0xc070>  @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x61870,false)?)?;
    // 0x61854: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x61858: str r1, [r3, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,8)?)?;
    // 0x6185c: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x61860: str r1, [r3, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,24)?)?;
    // 0x61864: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x61868: strb r1, [r3, #0x2a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,42)?)?;
    // 0x6186c: ldr r1, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1672)?)?;
    // 0x61870: ldr r2, [r8, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,972)?)?;
    // 0x61874: mov r3, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16711680)?)?;
    // 0x61878: orr r3, r3, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4278190080)?)?;
    // 0x6187c: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x61880: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x61884: str r1, [r8, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,972)?)?;
    // 0x61888: ldrb r0, [r0, #0x399]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,921)?)?;
    // 0x6188c: ldr r1, [r4, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,584)?)?;
    // 0x61890: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x61894: bne 0x618a8 <entry+0xc0a8>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x618a8,false)?)?;
    // 0x61898: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x6189c: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x618a0: bl 0x61960 <entry+0xc160>  @ imm = #0xb8
    c.emit(arm32::branch(C::Al,c.at as u32,0x61960,true)?)?;
    // 0x618a4: b 0x61934 <entry+0xc134>  @ imm = #0x88
    c.emit(arm32::branch(C::Al,c.at as u32,0x61934,false)?)?;
    // 0x618a8: add r0, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x618ac: add r2, r4, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x618b0: ldr r2, [r2, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,668)?)?;
    // 0x618b4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x618b8: bne 0x61934 <entry+0xc134>  @ imm = #0x74
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61934,false)?)?;
    // 0x618bc: ldr r2, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,76)?)?;
    // 0x618c0: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x618c4: bne 0x618fc <entry+0xc0fc>  @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x618fc,false)?)?;
    // 0x618c8: ldr r0, [r4, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1716)?)?;
    // 0x618cc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x618d0: ldreq r0, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(8)?,400)?)?;
    // 0x618d4: cmpeq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x618d8: bne 0x61898 <entry+0xc098>  @ imm = #-0x48
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61898,false)?)?;
    // 0x618dc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x618e0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x618e4: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x618e8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x618ec: bl 0x61960 <entry+0xc160>  @ imm = #0x6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x61960,true)?)?;
    // 0x618f0: str r5, [r4, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1716)?)?;
    // 0x618f4: ldr r1, [r4, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,584)?)?;
    // 0x618f8: b 0x61898 <entry+0xc098>  @ imm = #-0x68
    c.emit(arm32::branch(C::Al,c.at as u32,0x61898,false)?)?;
    // 0x618fc: add r1, r4, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,648)?)?;
    // 0x61900: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x61904: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x61908: str r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,16)?)?;
    // 0x6190c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x61910: ldr r0, [r4, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,584)?)?;
    // 0x61914: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x61918: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6191c: strb r2, [r0, #0x32]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,50)?)?;
    // 0x61920: ldr r0, [r4, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,584)?)?;
    // 0x61924: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x61928: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6192c: ldr r1, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1672)?)?;
    // 0x61930: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x61934: ldr r0, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1672)?)?;
    // 0x61938: ldr r1, [r8, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,988)?)?;
    // 0x6193c: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x61940: str r1, [r0, #0x278]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,632)?)?;
    // 0x61944: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61948: str r0, [r4, #0x670]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1648)?)?;
    // 0x6194c: ldr r0, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1656)?)?;
    // 0x61950: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x61954: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61958: strne r0, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1656)?)?;
    // 0x6195c: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x61960: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x61964: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x61968: ldr r5, [r0, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,368)?)?;
    // 0x6196c: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x61970: sub r2, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(4)?,1)?)?;
    // 0x61974: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // 0x61978: bhi 0x61a38 <entry+0xc238>  @ imm = #0xb8
    c.emit(arm32::branch(C::Hi,c.at as u32,0x61a38,false)?)?;
    // 0x6197c: lsl r2, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,2)?)?;
    // 0x61980: add r3, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::PC,0)?)?;
    // 0x61984: ldr pc, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
 c.check(0x61988)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_61988:[u32;6]=[0x000619a0,0x00061a18,0x00061a38,0x00061a54,0x00061a38,0x00061a5c];
 for value in INLINE_DATA_61988{c.emit(value)?;}
 // Executable A32 block 0x619a0..0x61ab8.
 c.check(0x619a0)?;
    // 0x619a0: ldr r2, [r0, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,88)?)?;
    // 0x619a4: ldr r7, [r5, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,308)?)?;
    // 0x619a8: add r1, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x619ac: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x619b0: sub r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R2,1)?)?;
    // 0x619b4: and r8, r7, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x619b8: add r7, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x619bc: mov r6, r7
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(7)?))?;
    // 0x619c0: ldr r0, [r6, #0x2a4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(6)?,676)?)?;
    // 0x619c4: cmp r0, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x619c8: subhi r0, r0, r2
    c.emit(arm32::data_register(C::Hi,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x619cc: ldr r2, [pc, #0xe4]         @ 0xc2b8 <entry+0xc2b8>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,228)?)?;
    // 0x619d0: strhi r0, [r6]
    c.emit(arm32::transfer_word(C::Hi,false,R::R0,R::new(6)?,0)?)?;
    // 0x619d4: cmp r8, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x619d8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x619dc: ldr r1, [r7, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,664)?)?;
    // 0x619e0: movgt r0, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x619e4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x619e8: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x619ec: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x619f0: mov r1, #58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,58)?)?;
    // 0x619f4: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x619f8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x60250
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x619fc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x61a00: str r9, [r7, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(7)?,664)?)?;
    // 0x61a04: strb r0, [r7, #0x2ba]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,698)?)?;
    // 0x61a08: sub r0, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,1)?)?;
    // 0x61a0c: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x61a10: sub r6, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x61a14: b 0x61a80 <entry+0xc280>  @ imm = #0x64
    c.emit(arm32::branch(C::Al,c.at as u32,0x61a80,false)?)?;
    // 0x61a18: add r1, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x61a1c: add r0, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x61a20: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x61a24: str r1, [r0, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,664)?)?;
    // 0x61a28: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x61a2c: ldr r6, [r0, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,680)?)?;
    // 0x61a30: str r1, [r0, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,680)?)?;
    // 0x61a34: b 0x61a80 <entry+0xc280>  @ imm = #0x44
    c.emit(arm32::branch(C::Al,c.at as u32,0x61a80,false)?)?;
    // 0x61a38: ldr r2, [pc, #0x78]         @ 0xc2b8 <entry+0xc2b8>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,120)?)?;
    // 0x61a3c: mov r1, #101
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,101)?)?;
    // 0x61a40: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61a44: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x61a48: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x61a4c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x602a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x61a50: b 0x61a80 <entry+0xc280>  @ imm = #0x28
    c.emit(arm32::branch(C::Al,c.at as u32,0x61a80,false)?)?;
    // 0x61a54: ldr r6, [r0, #0x680]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,1664)?)?;
    // 0x61a58: b 0x61a80 <entry+0xc280>  @ imm = #0x20
    c.emit(arm32::branch(C::Al,c.at as u32,0x61a80,false)?)?;
    // 0x61a5c: add r1, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x61a60: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x61a64: add r1, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x61a68: str r2, [r1, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,664)?)?;
    // 0x61a6c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x61a70: strb r2, [r1, #0x2ba]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,698)?)?;
    // 0x61a74: ldr r2, [r0, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,1672)?)?;
    // 0x61a78: str r2, [r1, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,680)?)?;
    // 0x61a7c: ldr r6, [r0, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,1672)?)?;
    // 0x61a80: ldr r0, [r5, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,332)?)?;
    // 0x61a84: ldr r2, [pc, #0x2c]         @ 0xc2b8 <entry+0xc2b8>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,44)?)?;
    // 0x61a88: add r0, r5, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x61a8c: str r4, [r0, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,368)?)?;
    // 0x61a90: str r6, [r0, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,372)?)?;
    // 0x61a94: ldr r0, [r5, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,332)?)?;
    // 0x61a98: add r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1)?)?;
    // 0x61a9c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61aa0: cmp r1, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,9)?)?;
    // 0x61aa4: str r1, [r5, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,332)?)?;
    // 0x61aa8: mov r1, #620
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,620)?)?;
    // 0x61aac: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x61ab0: pop {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x61ab4: b 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x6030c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,false)?)?;
 c.check(0x61ab8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_61AB8:[u32;1]=[0x000f1748];
 for value in INLINE_DATA_61AB8{c.emit(value)?;}
 // Executable A32 block 0x61abc..0x61d78.
 c.check(0x61abc)?;
    // 0x61abc: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x61ac0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x61ac4: add lr, r2, #452
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R2,452)?)?;
    // 0x61ac8: mov r12, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,10)?)?;
    // 0x61acc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x61ad0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x61ad4: ldr r5, [lr, r3, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::LR,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x61ad8: cmp r4, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x61adc: movlt r12, r3
    c.emit(arm32::mov_register(C::Lt,R::R12,R::new(3)?))?;
    // 0x61ae0: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x61ae4: add r4, r2, r12, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R2,R::R12,Shift::Lsl,2,false)?)?;
    // 0x61ae8: cmp r3, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,20)?)?;
    // 0x61aec: ldr r4, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,452)?)?;
    // 0x61af0: bne 0x61ad4 <entry+0xc2d4>  @ imm = #-0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61ad4,false)?)?;
    // 0x61af4: add lr, r2, #532
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R2,532)?)?;
    // 0x61af8: mov r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x61afc: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x61b00: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x61b04: ldr r6, [lr, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::LR,R::new(4)?,Shift::Lsl,2)?)?;
    // 0x61b08: cmp r5, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x61b0c: movlt r3, r4
    c.emit(arm32::mov_register(C::Lt,R::new(3)?,R::new(4)?))?;
    // 0x61b10: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x61b14: add r5, r2, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x61b18: cmp r4, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,10)?)?;
    // 0x61b1c: ldr r5, [r5, #0x214]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,532)?)?;
    // 0x61b20: bne 0x61b04 <entry+0xc304>  @ imm = #-0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61b04,false)?)?;
    // 0x61b24: ldr r6, [r2, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,448)?)?;
    // 0x61b28: add r3, r3, #251
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,251)?)?;
    // 0x61b2c: add r5, r12, #246
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R12,246)?)?;
    // 0x61b30: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x61b34: moveq r6, #10
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,10)?)?;
    // 0x61b38: streq r6, [r2, #0x1c0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,448)?)?;
    // 0x61b3c: ldrb r6, [r2, #0x1ac]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,428)?)?;
    // 0x61b40: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x61b44: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x61b48: addne r2, r5, #65280
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::new(5)?,65280)?)?;
    // 0x61b4c: addne r6, r3, #65280
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(6)?,R::new(3)?,65280)?)?;
    // 0x61b50: strh r2, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,0)?)?;
    // 0x61b54: strh r6, [r1]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::R1,0)?)?;
    // 0x61b58: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x61b5c: ldrb r3, [r1, #0xa0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,160)?)?;
    // 0x61b60: strb r3, [r0, #0x1ac]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,428)?)?;
    // 0x61b64: ldrb r2, [r1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,0)?)?;
    // 0x61b68: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x61b6c: bxne lr
    c.emit(arm32::bx(C::Ne,R::LR))?;
    // 0x61b70: ldrb r2, [r0, #0x1ad]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,429)?)?;
    // 0x61b74: orrs r3, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R2,R::new(3)?,Shift::Lsl,0,true)?)?;
    // 0x61b78: beq 0x61c60 <entry+0xc460>  @ imm = #0xe0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x61c60,false)?)?;
    // 0x61b7c: ldr r12, [r1, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R1,128)?)?;
    // 0x61b80: str r12, [r0, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,432)?)?;
    // 0x61b84: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x61b88: ldr r3, [r1, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,152)?)?;
    // 0x61b8c: rsbmi r12, r12, #0
    c.emit(arm32::data_immediate(C::Mi,Op::Rsb,R::R12,R::R12,0)?)?;
    // 0x61b90: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x61b94: str r3, [r0, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,436)?)?;
    // 0x61b98: rsbmi r3, r3, #0
    c.emit(arm32::data_immediate(C::Mi,Op::Rsb,R::new(3)?,R::new(3)?,0)?)?;
    // 0x61b9c: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x61ba0: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x61ba4: adds r2, r3, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R12,Shift::Lsl,0,true)?)?;
    // 0x61ba8: add lr, r0, #452
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R0,452)?)?;
    // 0x61bac: str r2, [r0, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,440)?)?;
    // 0x61bb0: asr r2, r3, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(3)?,Shift::Asr,31)?)?;
    // 0x61bb4: adc r2, r2, r12, asr #31
    c.emit(emit_adc_shifted(C::Al,R::R2,R::R2,R::R12,Shift::Asr,31,false)?)?;
    // 0x61bb8: add r12, r1, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R1,88)?)?;
    // 0x61bbc: str r2, [r0, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,444)?)?;
    // 0x61bc0: ldr r2, [r1, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,156)?)?;
    // 0x61bc4: ldr r3, [r1, #0xa4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,164)?)?;
    // 0x61bc8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x61bcc: str r3, [r0, #0x23c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,572)?)?;
    // 0x61bd0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x61bd4: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x61bd8: str r2, [r0, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,448)?)?;
    // 0x61bdc: add r2, r1, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,124)?)?;
    // 0x61be0: ldrh r4, [r2, #2]
    c.emit(arm32::load_halfword(C::Al,R::new(4)?,R::R2,2)?)?;
    // 0x61be4: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x61be8: sub r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,4)?)?;
    // 0x61bec: cmp r3, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,10)?)?;
    // 0x61bf0: str r4, [lr], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(4)?,R::LR,4)?)?;
    // 0x61bf4: bne 0x61be0 <entry+0xc3e0>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61be0,false)?)?;
    // 0x61bf8: add r2, r0, #492
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,492)?)?;
    // 0x61bfc: mov r3, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    // 0x61c00: ldrh r4, [r12], #4
    c.emit(emit_halfword_postindexed(C::Al,R::new(4)?,R::R12,4)?)?;
    // 0x61c04: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x61c08: cmp r3, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,20)?)?;
    // 0x61c0c: str r4, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(4)?,R::R2,4)?)?;
    // 0x61c10: bne 0x61c00 <entry+0xc400>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61c00,false)?)?;
    // 0x61c14: add lr, r0, #532
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R0,532)?)?;
    // 0x61c18: add r2, r1, #148
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,148)?)?;
    // 0x61c1c: add r12, r1, #132
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R1,132)?)?;
    // 0x61c20: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x61c24: ldrh r4, [r2, #2]
    c.emit(arm32::load_halfword(C::Al,R::new(4)?,R::R2,2)?)?;
    // 0x61c28: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x61c2c: sub r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,4)?)?;
    // 0x61c30: cmp r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,5)?)?;
    // 0x61c34: str r4, [lr], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(4)?,R::LR,4)?)?;
    // 0x61c38: bne 0x61c24 <entry+0xc424>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61c24,false)?)?;
    // 0x61c3c: add r2, r0, #552
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,552)?)?;
    // 0x61c40: mov r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x61c44: ldrh r4, [r12], #4
    c.emit(emit_halfword_postindexed(C::Al,R::new(4)?,R::R12,4)?)?;
    // 0x61c48: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x61c4c: cmp r3, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,10)?)?;
    // 0x61c50: str r4, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(4)?,R::R2,4)?)?;
    // 0x61c54: bne 0x61c44 <entry+0xc444>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61c44,false)?)?;
    // 0x61c58: ldrb r2, [r0, #0x1ad]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,429)?)?;
    // 0x61c5c: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x61c60: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x61c64: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x61c68: strbne r2, [r0, #0x6d8]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::R0,1752)?)?;
    // 0x61c6c: ldrbne r1, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::R1,1)?)?;
    // 0x61c70: strbne r1, [r0, #0x6d9]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::R0,1753)?)?;
    // 0x61c74: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x61c78: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x61c7c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x61c80: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x61c84: mov r0, #1760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1760)?)?;
    // 0x61c88: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x61c8c: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x5ed98
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x61c90: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x61c94: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x61c98: beq 0x61d58 <entry+0xc558>  @ imm = #0xb8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x61d58,false)?)?;
    // 0x61c9c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x61ca0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x61ca4: mov r2, #1760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1760)?)?;
    // 0x61ca8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x61cac: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x5eac8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x61cb0: ldr r0, [r5, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2104)?)?;
    // 0x61cb4: mov r1, #408
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,408)?)?;
    // 0x61cb8: ldrsb r1, [r0, r1]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R1,R::R0,R::R1)?)?;
    // 0x61cbc: str r1, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,404)?)?;
    // 0x61cc0: mov r1, #154
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,154)?)?;
    // 0x61cc4: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x61cc8: ldrsb r0, [r0, r1]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R0,R::R0,R::R1)?)?;
    // 0x61ccc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x61cd0: str r0, [r4, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,408)?)?;
    // 0x61cd4: str r6, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,368)?)?;
    // 0x61cd8: add r0, r4, #684
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,684)?)?;
    // 0x61cdc: mvn r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(6)?,R::R0,0)?)?;
    // 0x61ce0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x61ce4: strd r6, r7, [r0, #-4]
    c.emit(arm32::transfer_doubleword(C::Al,false,R::new(6)?,R::R0,-4)?)?;
    // 0x61ce8: add r0, r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,72)?)?;
    // 0x61cec: cmp r1, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,9)?)?;
    // 0x61cf0: bne 0x61ce0 <entry+0xc4e0>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61ce0,false)?)?;
    // 0x61cf4: add r0, r4, #308
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,308)?)?;
    // 0x61cf8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x61cfc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x61d00: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x61d04: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x61d08: str r1, [r0], #16
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x61d0c: cmp r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,6)?)?;
    // 0x61d10: bne 0x61d04 <entry+0xc504>  @ imm = #-0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61d04,false)?)?;
    // 0x61d14: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61d18: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x61d1c: str r0, [r4, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,596)?)?;
    // 0x61d20: ldrb r1, [r5, #0x9a0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2464)?)?;
    // 0x61d24: and r1, r2, r1, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsr,1,false)?)?;
    // 0x61d28: strb r1, [r4, #0x1ac]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,428)?)?;
    // 0x61d2c: ldrb r1, [r5, #0xbc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,188)?)?;
    // 0x61d30: strb r1, [r4, #0x1ad]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,429)?)?;
    // 0x61d34: ldr r1, [r5, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,284)?)?;
    // 0x61d38: str r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1732)?)?;
    // 0x61d3c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x61d40: strb r0, [r4, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1744)?)?;
    // 0x61d44: str r1, [r4, #0x6d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1748)?)?;
    // 0x61d48: mov r1, #728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,728)?)?;
    // 0x61d4c: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x61d50: strh r0, [r4, r1]
    c.emit(emit_halfword_register(C::Al,false,R::R0,R::new(4)?,R::R1,false)?)?;
    // 0x61d54: b 0x61d70 <entry+0xc570>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x61d70,false)?)?;
    // 0x61d58: ldr r2, [pc, #0x18]         @ 0xc578 <entry+0xc578>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,24)?)?;
    // 0x61d5c: mov r1, #141
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,141)?)?;
    // 0x61d60: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61d64: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x61d68: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x61d6c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x605c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x61d70: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x61d74: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x61d78)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_61D78:[u32;1]=[0x000f1748];
 for value in INLINE_DATA_61D78{c.emit(value)?;}
 // Executable A32 block 0x61d7c..0x61e34.
 c.check(0x61d7c)?;
    // 0x61d7c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x61d80: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x61d84: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x61d88: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x61d8c: mov r10, r2
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R2))?;
    // 0x61d90: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x61d94: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x61d98: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x61d9c: str r0, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,76)?)?;
    // 0x61da0: ldrb r0, [r9, #0x15f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,351)?)?;
    // 0x61da4: strb r0, [r10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x61da8: ldrb r0, [r9, #0x8ca]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2250)?)?;
    // 0x61dac: strb r0, [r10, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1)?)?;
    // 0x61db0: ldrb r0, [r9, #0x162]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,354)?)?;
    // 0x61db4: strb r0, [r10, #0x6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,6)?)?;
    // 0x61db8: ldr r0, [r9, #0x254]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,596)?)?;
    // 0x61dbc: str r0, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,80)?)?;
    // 0x61dc0: ldrb r0, [r9, #0x2e1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,737)?)?;
    // 0x61dc4: strb r0, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,10)?)?;
    // 0x61dc8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x61dcc: str r0, [r10, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,364)?)?;
    // 0x61dd0: str r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,356)?)?;
    // 0x61dd4: ldrb r0, [r9, #0xbc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,188)?)?;
    // 0x61dd8: strb r0, [r10, #0x1ad]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,429)?)?;
    // 0x61ddc: ldrb r0, [r8, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,1)?)?;
    // 0x61de0: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x61de4: ldrbne r0, [r8, #0x80]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(8)?,128)?)?;
    // 0x61de8: addne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,1)?)?;
    // 0x61dec: strbne r0, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(10)?,10)?)?;
    // 0x61df0: ldrb r0, [r9, #0x2d8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,728)?)?;
    // 0x61df4: lsl r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x61df8: str r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,88)?)?;
    // 0x61dfc: ldrsh r0, [r9, #116]
    c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::new(9)?,116)?)?;
    // 0x61e00: str r0, [r10, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,336)?)?;
    // 0x61e04: ldrsh r0, [r9, #118]
    c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::new(9)?,118)?)?;
    // 0x61e08: str r0, [r10, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,340)?)?;
    // 0x61e0c: ldrsh r0, [r9, #120]
    c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::new(9)?,120)?)?;
    // 0x61e10: str r0, [r10, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,344)?)?;
    // 0x61e14: ldrb r0, [r9, #0x1e8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,488)?)?;
    // 0x61e18: sub r1, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,8)?)?;
    // 0x61e1c: str r0, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,84)?)?;
    // 0x61e20: cmp r1, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,9)?)?;
    // 0x61e24: bhi 0x61ecc <entry+0xc6cc>  @ imm = #0xa0
    c.emit(arm32::branch(C::Hi,c.at as u32,0x61ecc,false)?)?;
    // 0x61e28: lsl r0, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x61e2c: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x61e30: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x61e34)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_61E34:[u32;10]=[0x00061e5c,0x00061f14,0x00061f14,0x00061f14,0x00061f14,0x00061f14,0x00061f14,0x00061f4c,0x00061f94,0x00061fe0];
 for value in INLINE_DATA_61E34{c.emit(value)?;}
 // Executable A32 block 0x61e5c..0x62000.
 c.check(0x61e5c)?;
    // 0x61e5c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x61e60: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x61e64: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x61e68: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x61e6c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x61e70: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x61e74: str r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,92)?)?;
    // 0x61e78: strb r1, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,10)?)?;
    // 0x61e7c: strb r2, [r10, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,12)?)?;
    // 0x61e80: strb r3, [r10, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(10)?,13)?)?;
    // 0x61e84: strb r7, [r10, #0x11]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(10)?,17)?)?;
    // 0x61e88: strb r7, [r10, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(10)?,14)?)?;
    // 0x61e8c: strb r6, [r10, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(10)?,16)?)?;
    // 0x61e90: strb r6, [r10, #0xf]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(10)?,15)?)?;
    // 0x61e94: strb r6, [r10, #0x13]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(10)?,19)?)?;
    // 0x61e98: strb r6, [r10, #0x12]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(10)?,18)?)?;
    // 0x61e9c: strb r0, [r10, #0x2c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,44)?)?;
    // 0x61ea0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x61ea4: strb r1, [r10, #0x2d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,45)?)?;
    // 0x61ea8: strb r0, [r10, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,46)?)?;
    // 0x61eac: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x61eb0: strb r7, [r10, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(10)?,47)?)?;
    // 0x61eb4: strb r3, [r10, #0x30]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(10)?,48)?)?;
    // 0x61eb8: strb r0, [r10, #0x31]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,49)?)?;
    // 0x61ebc: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x61ec0: strb r0, [r10, #0x32]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,50)?)?;
    // 0x61ec4: strb r2, [r10, #0x33]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,51)?)?;
    // 0x61ec8: b 0x62018 <entry+0xc818>  @ imm = #0x148
    c.emit(arm32::branch(C::Al,c.at as u32,0x62018,false)?)?;
    // 0x61ecc: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x61ed0: bne 0x61f14 <entry+0xc714>  @ imm = #0x3c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61f14,false)?)?;
    // 0x61ed4: ldr r0, [r9, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,284)?)?;
    // 0x61ed8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x61edc: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x61ee0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x61ee4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x61ee8: str r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,92)?)?;
    // 0x61eec: add r0, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,12)?)?;
    // 0x61ef0: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x5ed0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x61ef4: add r0, r10, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,44)?)?;
    // 0x61ef8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x61efc: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x61f00: strb r1, [r0], #1
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::R1,R::R0,1)?)?;
    // 0x61f04: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x61f08: cmp r7, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,32)?)?;
    // 0x61f0c: bne 0x61efc <entry+0xc6fc>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61efc,false)?)?;
    // 0x61f10: b 0x62018 <entry+0xc818>  @ imm = #0x100
    c.emit(arm32::branch(C::Al,c.at as u32,0x62018,false)?)?;
    // 0x61f14: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x61f18: add r0, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,12)?)?;
    // 0x61f1c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x61f20: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x61f24: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x61f28: str r7, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(10)?,92)?)?;
    // 0x61f2c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x5ed48
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x61f30: add r0, r10, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,44)?)?;
    // 0x61f34: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x61f38: strb r7, [r0], #1
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::new(7)?,R::R0,1)?)?;
    // 0x61f3c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x61f40: cmp r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,32)?)?;
    // 0x61f44: bne 0x61f34 <entry+0xc734>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x61f34,false)?)?;
    // 0x61f48: b 0x62018 <entry+0xc818>  @ imm = #0xc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x62018,false)?)?;
    // 0x61f4c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x61f50: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x61f54: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x61f58: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x61f5c: str r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,92)?)?;
    // 0x61f60: strb r1, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,10)?)?;
    // 0x61f64: strb r1, [r10, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,12)?)?;
    // 0x61f68: strb r2, [r10, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,13)?)?;
    // 0x61f6c: strh r3, [r10, #16]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::new(10)?,16)?)?;
    // 0x61f70: strh r3, [r10, #14]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::new(10)?,14)?)?;
    // 0x61f74: strh r3, [r10, #18]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::new(10)?,18)?)?;
    // 0x61f78: strb r0, [r10, #0x2c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,44)?)?;
    // 0x61f7c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x61f80: strb r0, [r10, #0x2d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,45)?)?;
    // 0x61f84: strb r2, [r10, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,46)?)?;
    // 0x61f88: strb r1, [r10, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,47)?)?;
    // 0x61f8c: str r3, [r10, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,48)?)?;
    // 0x61f90: b 0x62018 <entry+0xc818>  @ imm = #0x80
    c.emit(arm32::branch(C::Al,c.at as u32,0x62018,false)?)?;
    // 0x61f94: ldr r0, [r9, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,284)?)?;
    // 0x61f98: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x61f9c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x61fa0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x61fa4: str r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,92)?)?;
    // 0x61fa8: strb r1, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,10)?)?;
    // 0x61fac: str r2, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,12)?)?;
    // 0x61fb0: str r2, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,16)?)?;
    // 0x61fb4: ldr r2, [r9, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,284)?)?;
    // 0x61fb8: strb r2, [r10, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,12)?)?;
    // 0x61fbc: strb r0, [r10, #0x2c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,44)?)?;
    // 0x61fc0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x61fc4: strb r0, [r10, #0x2d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,45)?)?;
    // 0x61fc8: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x61fcc: strb r1, [r10, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,46)?)?;
    // 0x61fd0: strb r0, [r10, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,47)?)?;
    // 0x61fd4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x61fd8: strb r0, [r10, #0x30]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,48)?)?;
    // 0x61fdc: b 0x62018 <entry+0xc818>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x62018,false)?)?;
    // 0x61fe0: add r0, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,12)?)?;
    // 0x61fe4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x61fe8: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x61fec: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x61ff0: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x5ee0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x61ff4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x61ff8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x61ffc: strb r0, [r10, #0x2c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,44)?)?;
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
