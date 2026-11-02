// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 typed instruction reconstruction, runtime
//! address `[0x55800,0x56000)`. This is a *partial function* continuation.
//! Every instruction is encoded from public A32 ISA fields; original MBN bytes
//! are used only by the optional external verifier, never by the emitter.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x55800;
pub const END:usize=0x56000;
pub const FILE_OFFSET:usize=0x4000+START;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,a:usize)->Result<(),String>{
   if self.at!=a {return Err(format!("Agent04 expected 0x{a:x}, reached 0x{:x}",self.at))}Ok(())
 }
 fn emit(&mut self,w:u32)->Result<(),String>{
   if self.at<START || self.at>=END || self.at>=0x68000 || self.at&3!=0 {return Err(format!("Agent04 invalid A32 offset: 0x{:x}",self.at))}
   self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
 }
}
/// A32 positive register-indexed byte load/store with pre-index writeback.
fn emit_byte_register_writeback(cond:C,load:bool,rd:R,rn:R,rm:R,
    shift:Shift,amount:u8)->Result<u32,String>{
 if rn==R::PC || rm==R::PC || (load && rd==rn){return Err("invalid A32 indexed byte writeback".into())}
 Ok(arm32::transfer_byte_register(cond,load,rd,rn,rm,shift,amount)? | (1<<21))
}
/// A32 LDRH/STRH positive register offset with optional preindex writeback.
fn emit_halfword_register(cond:C,load:bool,rd:R,base:R,off:R,writeback:bool)->Result<u32,String>{
 if [rd,base,off].contains(&R::PC) || (writeback && load && rd==base){return Err("unsafe halfword register indexing".into())}
 Ok(((cond as u32)<<28)|0x0180_00b0|(if load{1<<20}else{0})|(if writeback{1<<21}else{0})|(base.number()<<16)|(rd.number()<<12)|off.number())
}

/// Map active codec status, format identifiers and source controller fields.
fn copy_active_codec_properties(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x55800: ldr r0, [r4, #0xe3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3644)?)?;
    // 0x55804: str r0, [r4, #0x394]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,916)?)?;
    // 0x55808: mov r0, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,992)?)?;
    // 0x5580c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x55810: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55814: strb r0, [r4, #0x40c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1036)?)?;
    // 0x55818: mov r0, #214
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,214)?)?;
    // 0x5581c: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55820: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55824: strb r0, [r4, #0x402]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1026)?)?;
    // 0x55828: mov r0, #213
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,213)?)?;
    // 0x5582c: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55830: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55834: strb r0, [r4, #0x401]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1025)?)?;
    // 0x55838: mov r0, #237
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,237)?)?;
    // 0x5583c: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55840: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55844: strb r0, [r4, #0x419]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1049)?)?;
    // 0x55848: mov r0, #210
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,210)?)?;
    // 0x5584c: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55850: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55854: strb r0, [r4, #0x3fe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1022)?)?;
    // 0x55858: mov r0, #980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,980)?)?;
    // 0x5585c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x55860: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55864: strb r0, [r4, #0x400]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1024)?)?;
    // 0x55868: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x5586c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x55870: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55874: str r0, [r4, #0x440]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1088)?)?;
    // 0x55878: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x5587c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x55880: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55884: str r0, [r4, #0x444]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1092)?)?;
    // 0x55888: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x5588c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x55890: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55894: str r0, [r4, #0x448]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1096)?)?;
    // 0x55898: mov r0, #1056
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1056)?)?;
    // 0x5589c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x558a0: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x558a4: str r0, [r4, #0x44c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1100)?)?;
    // 0x558a8: mov r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,36)?)?;
    // 0x558ac: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x558b0: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x558b4: str r0, [r4, #0x450]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1104)?)?;
    // 0x558b8: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x558bc: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x558c0: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x558c4: str r0, [r4, #0x454]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1108)?)?;
    // 0x558c8: mov r0, #238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,238)?)?;
    // 0x558cc: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x558d0: ldrb r0, [r3, r0]!
    c.emit(emit_byte_register_writeback(C::Al,true,R::R0,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // 0x558d4: strb r0, [r3, #-0xfd4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,-4052)?)?;
    // 0x558d8: ldr r0, [r3, #0x1e]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,30)?)?;
    // 0x558dc: str r0, [r3, #-0xfb6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,-4022)?)?;
    // 0x558e0: ldr r0, [r3, #0x1a]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,26)?)?;
    // 0x558e4: and r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,1)?)?;
    // 0x558e8: str r0, [r3, #-0xfba]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,-4026)?)?;
    // 0x558ec: ldrb r0, [r3, #-0x46e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-1134)?)?;
    // 0x558f0: strb r0, [r3, #-0xf6a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,-3946)?)?;
    // 0x558f4: ldrb r0, [r3, #-0x56a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-1386)?)?;
    // 0x558f8: ldr r1, [pc, #0xfb0]        @ 0x10b0 <entry+0x10b0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4016)?)?;
    // 0x558fc: strb r0, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x55900: ldrb r0, [r3, #-0x56b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-1387)?)?;
    // 0x55904: ldr r1, [pc, #0xfa8]        @ 0x10b4 <entry+0x10b4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4008)?)?;
    // 0x55908: strb r0, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x5590c: ldrb r0, [r3, #-0x56d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-1389)?)?;
    // 0x55910: ldr r1, [pc, #0xfa0]        @ 0x10b8 <entry+0x10b8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4000)?)?;
    // 0x55914: strb r0, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x55918: ldrb r0, [r3, #-0x57a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-1402)?)?;
    // 0x5591c: ldr r1, [pc, #0xff4]        @ 0x1118 <entry+0x1118>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x55920: strb r0, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x55924: ldrb r0, [r3, #-0x579]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-1401)?)?;
    // 0x55928: ldr r1, [pc, #0xfec]        @ 0x111c <entry+0x111c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4076)?)?;
    // 0x5592c: strb r0, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x55930: ldrb r0, [r3, #-0x5f5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-1525)?)?;
    // 0x55934: ldr r1, [pc, #0xff4]        @ 0x1130 <entry+0x1130>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x55938: strb r0, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x5593c: ldrb r0, [r3, #-0xb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-11)?)?;
    // 0x55940: strb r0, [r3, #-0xfdf]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,-4063)?)?;
    // 0x55944: ldr r2, [r3, #-0x5aa]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,-1450)?)?;
    // 0x55948: ldrb r0, [r3, #-0x60a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,-1546)?)?;
    // 0x5594c: ldr r1, [pc, #0xff0]        @ 0x1144 <entry+0x1144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4080)?)?;
    // 0x55950: str r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,28)?)?;
    // 0x55954: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x55958: moveq r0, r2
    c.emit(arm32::mov_register(C::Eq,R::R0,R::R2))?;
    // 0x5595c: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x55960: strb r0, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x55964: blo 0x5598c <entry+0x18c>   @ imm = #0x20
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5598c,false)?)?;
    // 0x55968: ldr r1, [pc, #0xfd8]        @ 0x1148 <entry+0x1148>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4056)?)?;
    // 0x5596c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x55970: strb r0, [r4, #0x33c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,828)?)?;
    // 0x55974: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x55978: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x26cec
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5597c: ldr r1, [pc, #0xff4]        @ 0x1178 <entry+0x1178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x55980: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x55984: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x26cf8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x55988: ldrb r0, [r4, #0x33c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,828)?)?;
    // 0x5598c: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x55990: beq 0x559b8 <entry+0x1b8>   @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x559b8,false)?)?;
    // 0x55994: ldr r0, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1392)?)?;
    // 0x55998: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5599c: bne 0x559b8 <entry+0x1b8>   @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x559b8,false)?)?;
    // 0x559a0: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x559a4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x559a8: strb r0, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3104)?)?;
    // 0x559ac: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x559b0: str r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,40)?)?;
    // 0x559b4: strb r1, [r4, #0x494]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1172)?)?;
    // 0x559b8: ldrb r0, [r4, #0xe74]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3700)?)?;
    // 0x559bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x559c0: bne 0x559d4 <entry+0x1d4>   @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x559d4,false)?)?;
    // 0x559c4: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x559c8: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x559cc: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x559d0: streq r0, [r4, #0xec0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,3776)?)?;
    // 0x559d4: mov r0, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,124)?)?;
    // 0x559d8: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x559dc: mov r9, r4
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(4)?))?;
    // 0x559e0: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x559e4: ldr r3, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x559e8: mov r0, #5248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5248)?)?;
    // 0x559ec: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x559f0: str r3, [r4, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1268)?)?;
    // 0x559f4: ldr r10, [r9, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(10)?,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x559f8: str r10, [r9, #-0xf88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(9)?,-3976)?)?;
    // 0x559fc: ldr r0, [r9, #-0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,-24)?)?;
    // 0x55a00: str r0, [r9, #-0xf84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,-3972)?)?;
    // 0x55a04: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x55a08: str r0, [r9, #-0xf80]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,-3968)?)?;
    // 0x55a0c: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // 0x55a10: str r1, [r9, #-0xf7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,-3964)?)?;
    // 0x55a14: ldr r2, [r9, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,-8)?)?;
    // 0x55a18: str r2, [r9, #-0xf78]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,-3960)?)?;
    // 0x55a1c: ldr r2, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,12)?)?;
    // 0x55a20: str r2, [r9, #-0xf74]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,-3956)?)?;
    // 0x55a24: ldr r2, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,16)?)?;
    // 0x55a28: str r2, [r9, #-0xf70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,-3952)?)?;
    // 0x55a2c: ldr r2, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,20)?)?;
    // 0x55a30: str r2, [r9, #-0xf6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,-3948)?)?;
    // 0x55a34: ldr r2, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,24)?)?;
    // 0x55a38: str r2, [r9, #-0xf68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,-3944)?)?;
    // 0x55a3c: ldr r2, [r9, #-0x738]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,-1848)?)?;
    // 0x55a40: str r2, [r9, #-0x9e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,-2536)?)?;
    // 0x55a44: ldr r2, [r9, #-0x734]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,-1844)?)?;
    // 0x55a48: str r2, [r9, #-0x9e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,-2532)?)?;
    // 0x55a4c: str r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,36)?)?;
    // 0x55a50: str r3, [r9, #-0x9c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,-2500)?)?;
    // 0x55a54: str r0, [r9, #-0x9c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,-2496)?)?;
    // 0x55a58: str r10, [r9, #-0x9bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(9)?,-2492)?)?;
    // 0x55a5c: str r1, [r9, #-0x9b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,-2488)?)?;
    // 0x55a60: ldr r0, [pc, #0xff8]        @ 0x1260 <entry+0x1260>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,4088)?)?;
    // 0x55a64: ldr r0, [r9, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55a68: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x55a6c: bne 0x55a80 <entry+0x280>   @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55a80,false)?)?;
    // 0x55a70: add r0, r4, #2336
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,2336)?)?;
    // 0x55a74: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x55a78: mov r2, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,48)?)?;
    // 0x55a7c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x52898
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x55a80: mov r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,60)?)?;
    // 0x55a84: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x55a88: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55a8c: strb r0, [r4, #0x579]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1401)?)?;
    // 0x55a90: ldr r7, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,0)?)?;
    // 0x55a94: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x55a98: cmpne r7, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x55a9c: bne 0x55ab8 <entry+0x2b8>   @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55ab8,false)?)?;
    // 0x55aa0: mov r1, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1016)?)?;
    // 0x55aa4: mov r2, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1000)?)?;
    // 0x55aa8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x55aac: ldr r1, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x55ab0: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // 0x55ab4: str r3, [r4, #0x424]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1060)?)?;
    // 0x55ab8: ldr r1, [r4, #0xab0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2736)?)?;
    // 0x55abc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55ac0: beq 0x55ad8 <entry+0x2d8>   @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55ad8,false)?)?;
    // 0x55ac4: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x55ac8: bne 0x55ad8 <entry+0x2d8>   @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55ad8,false)?)?;
    // 0x55acc: ldr r1, [r4, #0xe9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3740)?)?;
    // 0x55ad0: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x55ad4: b 0x55af8 <entry+0x2f8>   @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x55af8,false)?)?;
    // 0x55ad8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x55adc: cmp r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x55ae0: beq 0x55af8 <entry+0x2f8>   @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55af8,false)?)?;
    // 0x55ae4: ldr r1, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2744)?)?;
    // 0x55ae8: ldrb r2, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,339)?)?;
    // 0x55aec: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x55af0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x55af4: beq 0x567d4 <entry+0xfd4>   @ imm = #0xcd8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x567d4,false)?)?;
    // 0x55af8: strb r1, [r4, #0x468]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1128)?)?;
    // 0x55afc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x55b00: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x55b04: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x55b08: strb r1, [r4, #0x413]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1043)?)?;
    // 0x55b0c: ldrb r0, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x55b10: strb r0, [r4, #0x57a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1402)?)?;
    // 0x55b14: ldr r8, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,2732)?)?;
    // 0x55b18: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x55b1c: beq 0x55b40 <entry+0x340>   @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55b40,false)?)?;
    // 0x55b20: ldr r0, [r4, #0x304]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,772)?)?;
    // 0x55b24: ldr r5, [r4, #0xe44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,3652)?)?;
    // 0x55b28: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x55b2c: add r1, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // 0x55b30: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2e114
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x55b34: mul r1, r0, r5
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(5)?)?)?;
    // 0x55b38: str r0, [r4, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,336)?)?;
    // 0x55b3c: str r1, [r4, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,340)?)?;
    // 0x55b40: ldr r0, [r4, #0x474]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1140)?)?;
    // 0x55b44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55b48: bne 0x55b5c <entry+0x35c>   @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55b5c,false)?)?;
    // 0x55b4c: ldr r0, [r4, #0x46c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1132)?)?;
    // 0x55b50: str r0, [r4, #0x474]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1140)?)?;
    // 0x55b54: ldr r0, [r4, #0xeb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3760)?)?;
    // 0x55b58: str r0, [r4, #0xeb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3768)?)?;
    // 0x55b5c: ldr r0, [r4, #0x478]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1144)?)?;
    // 0x55b60: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55b64: bne 0x55b78 <entry+0x378>   @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55b78,false)?)?;
    // 0x55b68: ldr r0, [r4, #0x470]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1136)?)?;
    // 0x55b6c: str r0, [r4, #0x478]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1144)?)?;
    // 0x55b70: ldr r0, [r4, #0xeb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3764)?)?;
    // 0x55b74: str r0, [r4, #0xebc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3772)?)?;
    // 0x55b78: mov r0, #404
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,404)?)?;
    // 0x55b7c: mov r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1280)?)?;
    // 0x55b80: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x55b84: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // 0x55b88: strh r1, [r4, r0]
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::new(4)?,R::R0,false)?)?;
    // 0x55b8c: ldr r0, [r4, #0xed0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3792)?)?;
    // 0x55b90: strb r0, [r4, #0x998]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2456)?)?;
    // 0x55b94: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x55b98: strb r0, [r4, #0x99b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2459)?)?;
    // 0x55b9c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x55ba0: beq 0x55bac <entry+0x3ac>   @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55bac,false)?)?;
    // 0x55ba4: cmp r7, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,16384)?)?;
    // 0x55ba8: movne r0, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x55bac: strb r0, [r4, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2462)?)?;
    // 0x55bb0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x55bb4: ldr r0, [r4, #0xea0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3744)?)?;
    // 0x55bb8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55bbc: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x55bc0: moveq r1, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x55bc4: moveq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x55bc8: strb r1, [r4, #0x99c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2460)?)?;
    // 0x55bcc: strb r0, [r4, #0x9a2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2466)?)?;
    // 0x55bd0: ldr r0, [r4, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,744)?)?;
    // 0x55bd4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55bd8: beq 0x55ce4 <entry+0x4e4>   @ imm = #0x104
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55ce4,false)?)?;
    // 0x55bdc: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x55be0: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x55be4: moveq r3, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x55be8: cmp r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x55bec: beq 0x55c3c <entry+0x43c>   @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55c3c,false)?)?;
    // 0x55bf0: ldrb r1, [r4, #0x348]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,840)?)?;
    // 0x55bf4: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x55bf8: bhs 0x55c3c <entry+0x43c>   @ imm = #0x3c
    c.emit(arm32::branch(C::Cs,c.at as u32,0x55c3c,false)?)?;
    // 0x55bfc: ldr r1, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2744)?)?;
 c.check(0x55c00)
}

/// Continue controller validation, conditional record updates and diagnostics.
fn normalize_active_codec_properties(c:&mut Code)->Result<(),String>{
 c.check(0x55c00)?;
    // 0x55c00: mov r5, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,15)?)?;
    // 0x55c04: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x55c08: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x55c0c: moveq r5, #31
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,31)?)?;
    // 0x55c10: ldrb r6, [r1, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,224)?)?;
    // 0x55c14: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x55c18: bne 0x55c44 <entry+0x444>   @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55c44,false)?)?;
    // 0x55c1c: ldr r6, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,716)?)?;
    // 0x55c20: add r6, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x55c24: lsr r6, r6, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(6)?,R::R0,R::new(6)?,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x55c28: lsl r6, r6, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(6)?,R::R0,R::new(6)?,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x55c2c: lsr r3, r6, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::new(6)?,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x55c30: cmp r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,13)?)?;
    // 0x55c34: movhs r2, #1
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x55c38: b 0x55c44 <entry+0x444>   @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x55c44,false)?)?;
    // 0x55c3c: ldr r1, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2744)?)?;
    // 0x55c40: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x55c44: strb r2, [r4, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1148)?)?;
    // 0x55c48: strb r2, [r1, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,204)?)?;
    // 0x55c4c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x55c50: ldrb r2, [r4, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,3568)?)?;
    // 0x55c54: strb r3, [r4, #0x493]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1171)?)?;
    // 0x55c58: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x55c5c: bls 0x55cdc <entry+0x4dc>   @ imm = #0x78
    c.emit(arm32::branch(C::Ls,c.at as u32,0x55cdc,false)?)?;
    // 0x55c60: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x55c64: strb r6, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1173)?)?;
    // 0x55c68: strb r6, [r1, #0x498]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,1176)?)?;
    // 0x55c6c: ldr r5, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,716)?)?;
    // 0x55c70: cmp r5, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1280)?)?;
    // 0x55c74: movlo r3, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x55c78: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x55c7c: strb r3, [r4, #0x497]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1175)?)?;
    // 0x55c80: mov r3, #2560
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2560)?)?;
    // 0x55c84: orr r3, r3, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65536)?)?;
    // 0x55c88: str r3, [r4, #0x49c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1180)?)?;
    // 0x55c8c: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x55c90: strb r3, [r4, #0x498]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1176)?)?;
    // 0x55c94: strb r6, [r4, #0x496]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1174)?)?;
    // 0x55c98: bne 0x55ce4 <entry+0x4e4>   @ imm = #0x44
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55ce4,false)?)?;
    // 0x55c9c: ldrb r2, [r4, #0x3f0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1008)?)?;
    // 0x55ca0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x55ca4: beq 0x55ce4 <entry+0x4e4>   @ imm = #0x38
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55ce4,false)?)?;
    // 0x55ca8: ldr r2, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,716)?)?;
    // 0x55cac: cmp r2, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1920)?)?;
    // 0x55cb0: bhi 0x55ce4 <entry+0x4e4>   @ imm = #0x2c
    c.emit(arm32::branch(C::Hi,c.at as u32,0x55ce4,false)?)?;
    // 0x55cb4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x55cb8: strb r2, [r4, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1148)?)?;
    // 0x55cbc: strb r2, [r1, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,204)?)?;
    // 0x55cc0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x55cc4: strb r2, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1173)?)?;
    // 0x55cc8: strb r2, [r4, #0x496]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1174)?)?;
    // 0x55ccc: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x55cd0: strb r2, [r1, #0xdd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,221)?)?;
    // 0x55cd4: strb r2, [r1, #0xde]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,222)?)?;
    // 0x55cd8: b 0x55ce4 <entry+0x4e4>   @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x55ce4,false)?)?;
    // 0x55cdc: strb r3, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1173)?)?;
    // 0x55ce0: strb r3, [r4, #0x496]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1174)?)?;
    // 0x55ce4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x55ce8: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x55cec: strb r1, [r4, #0xb78]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2936)?)?;
    // 0x55cf0: bne 0x55d94 <entry+0x594>   @ imm = #0x9c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55d94,false)?)?;
    // 0x55cf4: ldrb r1, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1173)?)?;
    // 0x55cf8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55cfc: bne 0x55d18 <entry+0x518>   @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55d18,false)?)?;
    // 0x55d00: ldrb r1, [r4, #0xf64]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3940)?)?;
    // 0x55d04: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55d08: bne 0x55d18 <entry+0x518>   @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55d18,false)?)?;
    // 0x55d0c: ldrb r1, [r4, #0xf80]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3968)?)?;
    // 0x55d10: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x55d14: bne 0x55d94 <entry+0x594>   @ imm = #0x78
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55d94,false)?)?;
    // 0x55d18: ldrb r1, [r4, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,972)?)?;
    // 0x55d1c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55d20: bne 0x55d30 <entry+0x530>   @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55d30,false)?)?;
    // 0x55d24: ldr r1, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,664)?)?;
    // 0x55d28: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x55d2c: blo 0x55d94 <entry+0x594>   @ imm = #0x60
    c.emit(arm32::branch(C::Cc,c.at as u32,0x55d94,false)?)?;
    // 0x55d30: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55d34: beq 0x55d94 <entry+0x594>   @ imm = #0x58
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55d94,false)?)?;
    // 0x55d38: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x55d3c: beq 0x55d58 <entry+0x558>   @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55d58,false)?)?;
    // 0x55d40: ldr r0, [r4, #0xd90]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3472)?)?;
    // 0x55d44: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x55d48: and r0, r1, r0, lsr #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsr,7,false)?)?;
    // 0x55d4c: strb r0, [r4, #0xb78]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2936)?)?;
    // 0x55d50: b 0x55d94 <entry+0x594>   @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x55d94,false)?)?;
    // 0x55d54: andseq lr, pc, r1
    c.emit(arm32::data_register(C::Eq,Op::And,R::LR,R::PC,R::R1,Shift::Lsl,0,true)?)?;
    // 0x55d58: ldrb r0, [r4, #0x404]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1028)?)?;
    // 0x55d5c: cmp r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,24)?)?;
    // 0x55d60: bhi 0x55d94 <entry+0x594>   @ imm = #0x2c
    c.emit(arm32::branch(C::Hi,c.at as u32,0x55d94,false)?)?;
    // 0x55d64: mov r1, #888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,888)?)?;
    // 0x55d68: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x55d6c: cmp r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,17)?)?;
    // 0x55d70: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x55d74: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x55d78: strh r2, [r4, r1]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(4)?,R::R1,false)?)?;
    // 0x55d7c: blo 0x55d94 <entry+0x594>   @ imm = #0x10
    c.emit(arm32::branch(C::Cc,c.at as u32,0x55d94,false)?)?;
    // 0x55d80: cmp r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x55d84: cmpne r0, #18
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,18)?)?;
    // 0x55d88: bne 0x55d90 <entry+0x590>   @ imm = #0x0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55d90,false)?)?;
    // 0x55d8c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x55d90: strb r0, [r4, #0xb79]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2937)?)?;
    // 0x55d94: ldrb r0, [r4, #0xdf2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3570)?)?;
    // 0x55d98: strb r0, [r4, #0x34a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,842)?)?;
    // 0x55d9c: ldrb r1, [r4, #0xdf8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3576)?)?;
    // 0x55da0: strb r1, [r4, #0x350]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,848)?)?;
    // 0x55da4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55da8: ldr r2, [pc, #0xebc]        @ 0x146c <entry+0x146c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3772)?)?;
    // 0x55dac: ldr r8, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-32)?)?;
    // 0x55db0: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x55db4: beq 0x55e90 <entry+0x690>   @ imm = #0xd4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55e90,false)?)?;
    // 0x55db8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55dbc: bne 0x55e38 <entry+0x638>   @ imm = #0x74
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55e38,false)?)?;
    // 0x55dc0: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // 0x55dc4: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x55dc8: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x55dcc: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x55dd0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x55dd4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x55dd8: stm sp, {r2, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x55ddc: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x55de0: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x55de4: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x2b45c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x55de8: ldr r1, [pc, #0xe80]        @ 0x1470 <entry+0x1470>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3712)?)?;
    // 0x55dec: ldr r2, [pc, #0xe80]        @ 0x1474 <entry+0x1474>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3712)?)?;
    // 0x55df0: mov r3, #29
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,29)?)?;
    // 0x55df4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x55df8: orr r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3072)?)?;
    // 0x55dfc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x27170
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x55e00: ldr r1, [pc, #0xb70]        @ 0x1178 <entry+0x1178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2928)?)?;
    // 0x55e04: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x55e08: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2717c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x55e0c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x55e10: strb r0, [r4, #0x34a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,842)?)?;
    // 0x55e14: str r5, [r4, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,3472)?)?;
    // 0x55e18: strb r5, [r4, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,3568)?)?;
    // 0x55e1c: ldrb r0, [r4, #0x350]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,848)?)?;
    // 0x55e20: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55e24: beq 0x55e90 <entry+0x690>   @ imm = #0x64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55e90,false)?)?;
    // 0x55e28: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x55e2c: ldr r10, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,0)?)?;
    // 0x55e30: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x55e34: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x55e38: ldr r1, [r4, #0xd74]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3444)?)?;
    // 0x55e3c: ldr r5, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,36)?)?;
    // 0x55e40: ldr r7, [r4, #0xd78]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,3448)?)?;
    // 0x55e44: add r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x55e48: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x55e4c: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2ddf8
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x55e50: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x55e54: add r0, r10, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x55e58: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x55e5c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x55e60: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2dde4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x55e64: ldr r1, [pc, #0xfa4]        @ 0x1610 <entry+0x1610>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4004)?)?;
    // 0x55e68: mul r2, r0, r6
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::new(6)?)?)?;
    // 0x55e6c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x55e70: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x55e74: str r2, [r4, #0xd14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,3348)?)?;
    // 0x55e78: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // 0x55e7c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x271f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x55e80: ldr r1, [pc, #0xaf0]        @ 0x1178 <entry+0x1178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2800)?)?;
    // 0x55e84: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x55e88: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x271fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x55e8c: b 0x55e98 <entry+0x698>   @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x55e98,false)?)?;
    // 0x55e90: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x55e94: str r0, [r4, #0xd14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3348)?)?;
    // 0x55e98: ldrb r0, [r4, #0xdf2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3570)?)?;
    // 0x55e9c: mov r2, #2560
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2560)?)?;
    // 0x55ea0: orr r2, r2, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65536)?)?;
    // 0x55ea4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55ea8: beq 0x55f04 <entry+0x704>   @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55f04,false)?)?;
    // 0x55eac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x55eb0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x55eb4: strb r0, [r4, #0x493]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1171)?)?;
    // 0x55eb8: strb r1, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1173)?)?;
    // 0x55ebc: strb r1, [r4, #0x496]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1174)?)?;
    // 0x55ec0: ldr r1, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,716)?)?;
    // 0x55ec4: cmp r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1280)?)?;
    // 0x55ec8: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x55ecc: strb r0, [r4, #0x497]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1175)?)?;
    // 0x55ed0: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x55ed4: str r2, [r4, #0x49c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1180)?)?;
    // 0x55ed8: strb r0, [r4, #0x498]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1176)?)?;
    // 0x55edc: ldr r0, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,292)?)?;
    // 0x55ee0: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x532f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x55ee4: ldr r0, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,296)?)?;
    // 0x55ee8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x55eec: str r0, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,296)?)?;
    // 0x55ef0: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x55ef4: bl 0x58814 <entry+0x3014>  @ imm = #0x2918
    c.emit(arm32::branch(C::Al,c.at as u32,0x58814,true)?)?;
    // 0x55ef8: str r0, [r4, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,436)?)?;
    // 0x55efc: ldr r0, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,292)?)?;
    // 0x55f00: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x532d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x55f04: ldr r9, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,2744)?)?;
    // 0x55f08: ldrb r0, [r9, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,224)?)?;
    // 0x55f0c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55f10: beq 0x55f24 <entry+0x724>   @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55f24,false)?)?;
    // 0x55f14: ldrb r1, [r4, #0x34a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,842)?)?;
    // 0x55f18: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55f1c: bne 0x55f30 <entry+0x730>   @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55f30,false)?)?;
    // 0x55f20: b 0x55fe4 <entry+0x7e4>   @ imm = #0xbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x55fe4,false)?)?;
    // 0x55f24: ldr r1, [r4, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,744)?)?;
    // 0x55f28: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55f2c: beq 0x55f14 <entry+0x714>   @ imm = #-0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55f14,false)?)?;
    // 0x55f30: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x55f34: strb r1, [r9, #0xe1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,225)?)?;
    // 0x55f38: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x55f3c: strb r1, [r9, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,217)?)?;
    // 0x55f40: strb r1, [r9, #0xdc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,220)?)?;
    // 0x55f44: ldrb r2, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1173)?)?;
    // 0x55f48: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x55f4c: movne r1, #10
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,10)?)?;
    // 0x55f50: strbne r1, [r9, #0xdd]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(9)?,221)?)?;
    // 0x55f54: strbne r1, [r9, #0xde]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(9)?,222)?)?;
    // 0x55f58: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x55f5c: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x55f60: bne 0x55f8c <entry+0x78c>   @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55f8c,false)?)?;
    // 0x55f64: ldrb r3, [r4, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1148)?)?;
    // 0x55f68: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x55f6c: beq 0x55f8c <entry+0x78c>   @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55f8c,false)?)?;
    // 0x55f70: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x55f74: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x55f78: strb r2, [r9, #0xda]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,218)?)?;
    // 0x55f7c: strb r2, [r9, #0xdb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,219)?)?;
    // 0x55f80: strb r3, [r9, #0xdd]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(9)?,221)?)?;
    // 0x55f84: strb r2, [r9, #0xde]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,222)?)?;
    // 0x55f88: b 0x55fcc <entry+0x7cc>   @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x55fcc,false)?)?;
    // 0x55f8c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x55f90: beq 0x55fa0 <entry+0x7a0>   @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55fa0,false)?)?;
    // 0x55f94: ldrb r2, [r4, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1148)?)?;
    // 0x55f98: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x55f9c: beq 0x55fb0 <entry+0x7b0>   @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55fb0,false)?)?;
    // 0x55fa0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x55fa4: strb r2, [r9, #0xda]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,218)?)?;
    // 0x55fa8: strb r2, [r9, #0xdb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,219)?)?;
    // 0x55fac: b 0x55fcc <entry+0x7cc>   @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x55fcc,false)?)?;
    // 0x55fb0: ldr r2, [pc, #0xff8]        @ 0x17b0 <entry+0x17b0>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,4088)?)?;
    // 0x55fb4: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // 0x55fb8: strb r2, [r3, #0xd9]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,217)?)?;
    // 0x55fbc: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x55fc0: strb r2, [r3, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(3)?,3)?)?;
    // 0x55fc4: strb r2, [r3, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(3)?,2)?)?;
    // 0x55fc8: strb r2, [r3, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(3)?,1)?)?;
    // 0x55fcc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x55fd0: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x55fd4: str r2, [r9, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,300)?)?;
    // 0x55fd8: str r2, [r9, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,304)?)?;
    // 0x55fdc: str r2, [r9, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,308)?)?;
    // 0x55fe0: strbne r2, [r4, #0x497]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::new(4)?,1175)?)?;
    // 0x55fe4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x55fe8: bne 0x55ffc <entry+0x7fc>   @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55ffc,false)?)?;
    // 0x55fec: ldrb r0, [r4, #0x497]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1175)?)?;
    // 0x55ff0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55ff4: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x55ff8: strbne r0, [r4, #0x497]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,1175)?)?;
    // 0x55ffc: add r0, r4, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,640)?)?;
 c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();copy_active_codec_properties(&mut c)?;
 normalize_active_codec_properties(&mut c)?;
 Ok(c.bytes)
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn controller_fragment_is_contiguous(){assert_eq!(fragment().unwrap().len(),END-START);}
 #[test]fn strict_register_transfer_checks(){
 assert!(emit_byte_register_writeback(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0).is_err());
 assert!(emit_halfword_register(C::Al,true,R::R0,R::R0,R::R1,true).is_err());
 }
 #[test]fn exact_private_reference_when_present(){
  if let Ok(path)=std::env::var("AGENT04_PRIVATE_VENUS"){
   let bytes=std::fs::read(path).unwrap();let frag=fragment().unwrap();
   assert_eq!(&frag[..],&bytes[FILE_OFFSET..FILE_OFFSET+frag.len()]);
  }
 }
}
