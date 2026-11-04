// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x63000,0x64000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x63000;
pub const END:usize=0x64000;
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
fn emit_halfword_register(cond:C,load:bool,rd:R,base:R,index:R,writeback:bool)->Result<u32,String>{
 if [rd,base,index].contains(&R::PC)||(load&&rd==base&&writeback){return Err("unsafe A32 indexed halfword transfer".into())}
 Ok(((cond as u32)<<28)|0x0180_00b0|(if load{1<<20}else{0})|(if writeback{1<<21}else{0})|(base.number()<<16)|(rd.number()<<12)|index.number())
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x63000..0x63588.
 c.check(0x63000)?;
    // 0x63000: bne 0x6302c <entry+0xd82c>  @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6302c,false)?)?;
    // 0x63004: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x63008: mov r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,127)?)?;
    // 0x6300c: mov r9, r12
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R12))?;
    // 0x63010: sub r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::new(9)?,1)?)?;
    // 0x63014: tst r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,255)?)?;
    // 0x63018: beq 0x6302c <entry+0xd82c>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6302c,false)?)?;
    // 0x6301c: and r7, r2, r1, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R2,R::R1,Shift::Lsr,1,false)?)?;
    // 0x63020: and r1, r0, r1, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsr,1,false)?)?;
    // 0x63024: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x63028: beq 0x63010 <entry+0xd810>  @ imm = #-0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63010,false)?)?;
    // 0x6302c: and r1, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(9)?,255)?)?;
    // 0x63030: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x63034: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x63038: cmp r12, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R1,Shift::Lsl,0,false)?)?;
    // 0x6303c: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x63040: movne r12, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x63044: b 0x62e30 <entry+0xd630>  @ imm = #-0x21c
    c.emit(arm32::branch(C::Al,c.at as u32,0x62e30,false)?)?;
    // 0x63048: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x6304c: strb r0, [r5, #0x175]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,373)?)?;
    // 0x63050: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x63054: strb r0, [r5, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,4)?)?;
    // 0x63058: b 0x63164 <entry+0xd964>  @ imm = #0x104
    c.emit(arm32::branch(C::Al,c.at as u32,0x63164,false)?)?;
    // 0x6305c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63060: strb r0, [r5, #0x175]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,373)?)?;
    // 0x63064: strb r0, [r5, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,4)?)?;
    // 0x63068: b 0x63164 <entry+0xd964>  @ imm = #0xf4
    c.emit(arm32::branch(C::Al,c.at as u32,0x63164,false)?)?;
    // 0x6306c: ldr r1, [r6, #0x458]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,1112)?)?;
    // 0x63070: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x63074: ldrne r1, [r6, #0x460]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(6)?,1120)?)?;
    // 0x63078: cmpne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x6307c: beq 0x630bc <entry+0xd8bc>  @ imm = #0x38
    c.emit(arm32::branch(C::Eq,c.at as u32,0x630bc,false)?)?;
    // 0x63080: ldr r8, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,12)?)?;
    // 0x63084: ldr r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,8)?)?;
    // 0x63088: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x6308c: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x63090: bl 0x761e0 <entry+0x209e0> @ imm = #0x13148
    c.emit(arm32::branch(C::Al,c.at as u32,0x761e0,true)?)?;
    // 0x63094: ldr r1, [r5, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,948)?)?;
    // 0x63098: ldr r2, [r5, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,952)?)?;
    // 0x6309c: ldr r0, [r5, #0x704]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1796)?)?;
    // 0x630a0: bl 0x71828 <entry+0x1c028> @ imm = #0xe780
    c.emit(arm32::branch(C::Al,c.at as u32,0x71828,true)?)?;
    // 0x630a4: ldr r0, [r5, #0x704]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1796)?)?;
    // 0x630a8: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x630ac: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x630b0: bl 0x71978 <entry+0x1c178> @ imm = #0xe8c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x71978,true)?)?;
    // 0x630b4: ldr r12, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,16)?)?;
    // 0x630b8: b 0x63160 <entry+0xd960>  @ imm = #0xa0
    c.emit(arm32::branch(C::Al,c.at as u32,0x63160,false)?)?;
    // 0x630bc: ldr r1, [r6, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,2732)?)?;
    // 0x630c0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x630c4: beq 0x63158 <entry+0xd958>  @ imm = #0x8c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63158,false)?)?;
    // 0x630c8: add r0, r12, r10, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::new(10)?,Shift::Lsl,2,false)?)?;
    // 0x630cc: ldr r0, [r0, #0x150]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,336)?)?;
    // 0x630d0: b 0x63160 <entry+0xd960>  @ imm = #0x88
    c.emit(arm32::branch(C::Al,c.at as u32,0x63160,false)?)?;
    // 0x630d4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x630d8: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x630dc: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x630e0: b 0x62e28 <entry+0xd628>  @ imm = #-0x2c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x62e28,false)?)?;
    // 0x630e4: ldr r7, [r6, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,924)?)?;
    // 0x630e8: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x630ec: beq 0x63148 <entry+0xd948>  @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63148,false)?)?;
    // 0x630f0: add r10, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(7)?,1)?)?;
    // 0x630f4: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x630f8: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x630fc: bl 0x81850 <entry+0x2c050> @ imm = #0x1e74c
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x63100: and r0, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R1,255)?)?;
    // 0x63104: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x63108: bne 0x6356c <entry+0xdd6c>  @ imm = #0x45c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6356c,false)?)?;
    // 0x6310c: ldr r2, [pc, #0x474]        @ 0xdd88 <entry+0xdd88>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1140)?)?;
    // 0x63110: sub r9, r9, r8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(9)?,R::new(9)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x63114: mov r1, #227
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,227)?)?;
    // 0x63118: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6311c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x63120: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x63124: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2560)?)?;
    // 0x63128: movgt r0, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x6312c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x61984
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x63130: cmp r9, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x63134: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x63138: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x6313c: sublo r7, r9, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Sub,R::new(7)?,R::new(9)?,1)?)?;
    // 0x63140: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x63144: b 0x63580 <entry+0xdd80>  @ imm = #0x434
    c.emit(arm32::branch(C::Al,c.at as u32,0x63580,false)?)?;
    // 0x63148: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x6314c: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x63150: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x63154: b 0x62e28 <entry+0xd628>  @ imm = #-0x334
    c.emit(arm32::branch(C::Al,c.at as u32,0x62e28,false)?)?;
    // 0x63158: add r0, r0, r10, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(10)?,Shift::Lsl,2,false)?)?;
    // 0x6315c: ldr r0, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,108)?)?;
    // 0x63160: strb r0, [r5, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,380)?)?;
    // 0x63164: ldrb r0, [r5, #0x175]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,373)?)?;
    // 0x63168: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x6316c: bne 0x63180 <entry+0xd980>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63180,false)?)?;
    // 0x63170: ldrb r0, [r5, #0x172]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,370)?)?;
    // 0x63174: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x63178: bne 0x6320c <entry+0xda0c>  @ imm = #0x8c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6320c,false)?)?;
    // 0x6317c: b 0x63200 <entry+0xda00>  @ imm = #0x7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x63200,false)?)?;
    // 0x63180: ldr r0, [r5, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,400)?)?;
    // 0x63184: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x63188: bne 0x63200 <entry+0xda00>  @ imm = #0x70
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63200,false)?)?;
    // 0x6318c: ldrb r0, [r12, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R12,3)?)?;
    // 0x63190: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x63194: bne 0x631c8 <entry+0xd9c8>  @ imm = #0x2c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x631c8,false)?)?;
    // 0x63198: ldr r0, [r5, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1760)?)?;
    // 0x6319c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x631a0: beq 0x631b0 <entry+0xd9b0>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x631b0,false)?)?;
    // 0x631a4: subs r0, r0, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R0,R::R0,1,true)?)?;
    // 0x631a8: str r0, [r5, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1760)?)?;
    // 0x631ac: bne 0x631bc <entry+0xd9bc>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x631bc,false)?)?;
    // 0x631b0: ldr r0, [r5, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1636)?)?;
    // 0x631b4: bic r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,16)?)?;
    // 0x631b8: str r0, [r5, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1636)?)?;
    // 0x631bc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x631c0: strb r0, [r12, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R12,3)?)?;
    // 0x631c4: b 0x63200 <entry+0xda00>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x63200,false)?)?;
    // 0x631c8: ldrb r0, [r12, #0x4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R12,4)?)?;
    // 0x631cc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x631d0: bne 0x63200 <entry+0xda00>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63200,false)?)?;
    // 0x631d4: ldr r0, [r5, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1760)?)?;
    // 0x631d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x631dc: beq 0x631ec <entry+0xd9ec>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x631ec,false)?)?;
    // 0x631e0: subs r0, r0, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R0,R::R0,1,true)?)?;
    // 0x631e4: str r0, [r5, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1760)?)?;
    // 0x631e8: bne 0x631f8 <entry+0xd9f8>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x631f8,false)?)?;
    // 0x631ec: ldr r0, [r5, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1636)?)?;
    // 0x631f0: bic r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,16)?)?;
    // 0x631f4: str r0, [r5, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1636)?)?;
    // 0x631f8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x631fc: strb r0, [r12, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R12,4)?)?;
    // 0x63200: ldr r0, [r12, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,332)?)?;
    // 0x63204: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x63208: str r0, [r12, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,332)?)?;
    // 0x6320c: ldrb r2, [r5, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,380)?)?;
    // 0x63210: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63214: add r1, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x63218: lsr r3, r1, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R1,Shift::Lsr,3)?)?;
    // 0x6321c: cmp r0, r1, lsr #9
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsr,9,false)?)?;
    // 0x63220: movne r3, #63
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,63)?)?;
    // 0x63224: strb r3, [r5, #0x33f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(5)?,831)?)?;
    // 0x63228: ldrb r1, [r6, #0x33b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,827)?)?;
    // 0x6322c: strb r1, [r5, #0x340]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,832)?)?;
    // 0x63230: ldr r1, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,644)?)?;
    // 0x63234: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x63238: bne 0x6325c <entry+0xda5c>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6325c,false)?)?;
    // 0x6323c: ldrb r0, [r6, #0x338]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,824)?)?;
    // 0x63240: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x63244: blo 0x63264 <entry+0xda64>  @ imm = #0x18
    c.emit(arm32::branch(C::Cc,c.at as u32,0x63264,false)?)?;
    // 0x63248: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6324c: strb r0, [r5, #0x33e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,830)?)?;
    // 0x63250: strb r0, [r5, #0x33f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,831)?)?;
    // 0x63254: strb r0, [r5, #0x340]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,832)?)?;
    // 0x63258: b 0x63278 <entry+0xda78>  @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x63278,false)?)?;
    // 0x6325c: strb r0, [r5, #0x33e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,830)?)?;
    // 0x63260: b 0x63278 <entry+0xda78>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x63278,false)?)?;
    // 0x63264: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // 0x63268: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x6326c: ldrb r1, [r1, #0xba]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,186)?)?;
    // 0x63270: moveq r1, r0
    c.emit(arm32::mov_register(C::Eq,R::R1,R::R0))?;
    // 0x63274: strb r1, [r5, #0x33e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,830)?)?;
    // 0x63278: ldr r0, [r6, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2744)?)?;
    // 0x6327c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63280: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x63284: ldr r0, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,56)?)?;
    // 0x63288: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6328c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63290: strbne r0, [r5, #0x33e]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(5)?,830)?)?;
    // 0x63294: strbne r0, [r5, #0x33f]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(5)?,831)?)?;
    // 0x63298: strbne r0, [r5, #0x340]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(5)?,832)?)?;
    // 0x6329c: mov r0, #126
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,126)?)?;
    // 0x632a0: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x632a4: strh r1, [r3, r0]!
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::new(3)?,R::R0,true)?)?;
    // 0x632a8: strh r1, [r3, #14]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(3)?,14)?)?;
    // 0x632ac: strh r1, [r3, #12]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(3)?,12)?)?;
    // 0x632b0: strh r1, [r3, #10]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(3)?,10)?)?;
    // 0x632b4: strh r1, [r3, #8]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(3)?,8)?)?;
    // 0x632b8: strh r1, [r3, #6]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(3)?,6)?)?;
    // 0x632bc: strh r1, [r3, #4]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(3)?,4)?)?;
    // 0x632c0: strh r1, [r3, #2]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(3)?,2)?)?;
    // 0x632c4: ldr r1, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,644)?)?;
    // 0x632c8: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // 0x632cc: beq 0x6332c <entry+0xdb2c>  @ imm = #0x58
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6332c,false)?)?;
    // 0x632d0: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x632d4: bne 0x633a0 <entry+0xdba0>  @ imm = #0xc4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x633a0,false)?)?;
    // 0x632d8: ldrb r7, [r6, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,1148)?)?;
    // 0x632dc: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x632e0: bne 0x633b0 <entry+0xdbb0>  @ imm = #0xc8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x633b0,false)?)?;
    // 0x632e4: ldr r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,12)?)?;
    // 0x632e8: ldrb r7, [r7, #0x215]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,533)?)?;
    // 0x632ec: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x632f0: bne 0x633b0 <entry+0xdbb0>  @ imm = #0xb8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x633b0,false)?)?;
    // 0x632f4: ldrb r7, [r6, #0x484]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,1156)?)?;
    // 0x632f8: strh r2, [r3]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(3)?,0)?)?;
    // 0x632fc: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // 0x63300: bne 0x63554 <entry+0xdd54>  @ imm = #0x24c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63554,false)?)?;
    // 0x63304: ldr r2, [r6, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,2744)?)?;
    // 0x63308: mov r7, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,384)?)?;
    // 0x6330c: ldr r3, [r2, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,236)?)?;
    // 0x63310: strh r3, [r5, r7]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,false)?)?;
    // 0x63314: mov r7, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,130)?)?;
    // 0x63318: ldr r3, [r2, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,240)?)?;
    // 0x6331c: orr r7, r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,256)?)?;
    // 0x63320: strh r3, [r5, r7]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,false)?)?;
    // 0x63324: ldr r2, [r2, #0xf4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,244)?)?;
    // 0x63328: b 0x633d8 <entry+0xdbd8>  @ imm = #0xa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x633d8,false)?)?;
    // 0x6332c: ldrb r7, [r6, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,1148)?)?;
    // 0x63330: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x63334: beq 0x6349c <entry+0xdc9c>  @ imm = #0x160
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6349c,false)?)?;
    // 0x63338: ldrb r2, [r5, #0x25e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,606)?)?;
    // 0x6333c: strh r2, [r3]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(3)?,0)?)?;
    // 0x63340: mov r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,384)?)?;
    // 0x63344: ldrb r2, [r5, #0x25f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,607)?)?;
    // 0x63348: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x6334c: mov r3, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,130)?)?;
    // 0x63350: ldrb r2, [r5, #0x260]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,608)?)?;
    // 0x63354: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x63358: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x6335c: mov r3, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,388)?)?;
    // 0x63360: ldrb r2, [r5, #0x261]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,609)?)?;
    // 0x63364: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63368: mov r3, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,134)?)?;
    // 0x6336c: ldrb r2, [r5, #0x262]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,610)?)?;
    // 0x63370: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x63374: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63378: mov r3, #392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,392)?)?;
    // 0x6337c: ldrb r2, [r5, #0x263]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,611)?)?;
    // 0x63380: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63384: mov r3, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,138)?)?;
    // 0x63388: ldrb r2, [r5, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,612)?)?;
    // 0x6338c: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x63390: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63394: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x63398: ldrb r2, [r5, #0x265]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,613)?)?;
    // 0x6339c: b 0x633dc <entry+0xdbdc>  @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x633dc,false)?)?;
    // 0x633a0: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // 0x633a4: ldrb r2, [r2, #0x215]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,533)?)?;
    // 0x633a8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x633ac: beq 0x633e4 <entry+0xdbe4>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x633e4,false)?)?;
    // 0x633b0: ldrb r2, [r5, #0x25e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,606)?)?;
    // 0x633b4: strh r2, [r3]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(3)?,0)?)?;
    // 0x633b8: mov r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,384)?)?;
    // 0x633bc: ldrb r2, [r5, #0x25f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,607)?)?;
    // 0x633c0: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x633c4: mov r3, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,130)?)?;
    // 0x633c8: ldrb r2, [r5, #0x260]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,608)?)?;
    // 0x633cc: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x633d0: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x633d4: ldrb r2, [r5, #0x261]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,609)?)?;
    // 0x633d8: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x633dc: add r3, r5, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x633e0: strh r2, [r3, r0]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(3)?,R::R0,false)?)?;
    // 0x633e4: add r2, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x633e8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x633ec: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // 0x633f0: beq 0x6341c <entry+0xdc1c>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6341c,false)?)?;
    // 0x633f4: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x633f8: bne 0x63448 <entry+0xdc48>  @ imm = #0x48
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63448,false)?)?;
    // 0x633fc: add r7, r5, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(5)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x63400: add r4, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x63404: ldrsh r7, [r2]
    c.emit(arm32::load_signed_halfword(C::Al,R::new(7)?,R::R2,0)?)?;
    // 0x63408: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x6340c: blt 0x63434 <entry+0xdc34>  @ imm = #0x20
    c.emit(arm32::branch(C::Lt,c.at as u32,0x63434,false)?)?;
    // 0x63410: cmp r7, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,127)?)?;
    // 0x63414: movgt r7, #127
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(7)?,R::R0,127)?)?;
    // 0x63418: b 0x63444 <entry+0xdc44>  @ imm = #0x24
    c.emit(arm32::branch(C::Al,c.at as u32,0x63444,false)?)?;
    // 0x6341c: ldrsh r7, [r2]
    c.emit(arm32::load_signed_halfword(C::Al,R::new(7)?,R::R2,0)?)?;
    // 0x63420: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x63424: blt 0x6343c <entry+0xdc3c>  @ imm = #0x10
    c.emit(arm32::branch(C::Lt,c.at as u32,0x6343c,false)?)?;
    // 0x63428: cmp r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,255)?)?;
    // 0x6342c: movgt r7, #255
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(7)?,R::R0,255)?)?;
    // 0x63430: b 0x63440 <entry+0xdc40>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x63440,false)?)?;
    // 0x63434: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x63438: b 0x63444 <entry+0xdc44>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x63444,false)?)?;
    // 0x6343c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x63440: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x63444: strh r7, [r4]
    c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::new(4)?,0)?)?;
    // 0x63448: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x6344c: add r2, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,2)?)?;
    // 0x63450: cmp r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8)?)?;
    // 0x63454: bne 0x633ec <entry+0xdbec>  @ imm = #-0x70
    c.emit(arm32::branch(C::Ne,c.at as u32,0x633ec,false)?)?;
    // 0x63458: ldr r0, [r6, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2744)?)?;
    // 0x6345c: ldrb r0, [r0, #0x498]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1176)?)?;
    // 0x63460: strb r0, [r5, #0x342]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,834)?)?;
    // 0x63464: ldr r1, [r6, #0xd04]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,3332)?)?;
    // 0x63468: ldr r0, [r12, #0x6c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,1728)?)?;
    // 0x6346c: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x63470: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x63474: movhi r0, #0
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63478: strhi r0, [r12, #0x6c0]
    c.emit(arm32::transfer_word(C::Hi,false,R::R0,R::R12,1728)?)?;
    // 0x6347c: add r1, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x63480: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x63484: ldr r1, [r1, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,76)?)?;
    // 0x63488: str r1, [r5, #0x518]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,1304)?)?;
    // 0x6348c: str r0, [r12, #0x6c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,1728)?)?;
    // 0x63490: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63494: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x63498: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x6349c: ldrb r7, [r6, #0x484]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,1156)?)?;
    // 0x634a0: strh r2, [r3]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(3)?,0)?)?;
    // 0x634a4: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // 0x634a8: bne 0x63510 <entry+0xdd10>  @ imm = #0x60
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63510,false)?)?;
    // 0x634ac: ldr r2, [r6, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,2744)?)?;
    // 0x634b0: mov r7, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,384)?)?;
    // 0x634b4: ldr r3, [r2, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,236)?)?;
    // 0x634b8: strh r3, [r5, r7]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,false)?)?;
    // 0x634bc: mov r7, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,130)?)?;
    // 0x634c0: ldr r3, [r2, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,240)?)?;
    // 0x634c4: orr r7, r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,256)?)?;
    // 0x634c8: strh r3, [r5, r7]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,false)?)?;
    // 0x634cc: mov r7, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,388)?)?;
    // 0x634d0: ldr r3, [r2, #0xf4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,244)?)?;
    // 0x634d4: strh r3, [r5, r7]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,false)?)?;
    // 0x634d8: mov r7, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,134)?)?;
    // 0x634dc: ldr r3, [r2, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,248)?)?;
    // 0x634e0: orr r7, r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,256)?)?;
    // 0x634e4: strh r3, [r5, r7]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,false)?)?;
    // 0x634e8: mov r7, #392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,392)?)?;
    // 0x634ec: ldr r3, [r2, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,252)?)?;
    // 0x634f0: strh r3, [r5, r7]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,false)?)?;
    // 0x634f4: mov r7, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,138)?)?;
    // 0x634f8: ldr r3, [r2, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,256)?)?;
    // 0x634fc: orr r7, r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,256)?)?;
    // 0x63500: strh r3, [r5, r7]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,false)?)?;
    // 0x63504: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x63508: ldr r2, [r2, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,260)?)?;
    // 0x6350c: b 0x633dc <entry+0xdbdc>  @ imm = #-0x138
    c.emit(arm32::branch(C::Al,c.at as u32,0x633dc,false)?)?;
    // 0x63510: mov r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,384)?)?;
    // 0x63514: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63518: mov r3, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,130)?)?;
    // 0x6351c: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x63520: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63524: mov r3, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,388)?)?;
    // 0x63528: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x6352c: mov r3, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,134)?)?;
    // 0x63530: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x63534: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63538: mov r3, #392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,392)?)?;
    // 0x6353c: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63540: mov r3, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,138)?)?;
    // 0x63544: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x63548: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x6354c: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x63550: b 0x633dc <entry+0xdbdc>  @ imm = #-0x17c
    c.emit(arm32::branch(C::Al,c.at as u32,0x633dc,false)?)?;
    // 0x63554: mov r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,384)?)?;
    // 0x63558: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x6355c: mov r3, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,130)?)?;
    // 0x63560: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x63564: strh r2, [r5, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?,false)?)?;
    // 0x63568: b 0x633d8 <entry+0xdbd8>  @ imm = #-0x198
    c.emit(arm32::branch(C::Al,c.at as u32,0x633d8,false)?)?;
    // 0x6356c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x63570: mov r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,2)?)?;
    // 0x63574: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x63578: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x6357c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x63580: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x63584: b 0x62e30 <entry+0xd630>  @ imm = #-0x75c
    c.emit(arm32::branch(C::Al,c.at as u32,0x62e30,false)?)?;
 c.check(0x63588)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_63588:[u32;1]=[0x000f1748];
 for value in INLINE_DATA_63588{c.emit(value)?;}
 // Executable A32 block 0x6358c..0x636f8.
 c.check(0x6358c)?;
    // 0x6358c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x63590: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x63594: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x63598: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x6359c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x635a0: ldrb r0, [r5, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,281)?)?;
    // 0x635a4: ldr r7, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,36)?)?;
    // 0x635a8: ldr r6, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,420)?)?;
    // 0x635ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x635b0: beq 0x635c8 <entry+0xddc8>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x635c8,false)?)?;
    // 0x635b4: mov r0, #280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,280)?)?;
    // 0x635b8: ldr r1, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,988)?)?;
    // 0x635bc: ldrsb r2, [r5, r0]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R2,R::new(5)?,R::R0)?)?;
    // 0x635c0: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x635c4: bl 0x39410 <entry+0xfffffffffffe3c10> @ imm = #-0x2a1bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x39410,true)?)?;
    // 0x635c8: ldr r0, [r6, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,140)?)?;
    // 0x635cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x635d0: beq 0x635f4 <entry+0xddf4>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x635f4,false)?)?;
    // 0x635d4: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // 0x635d8: and r0, r1, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,6,false)?)?;
    // 0x635dc: ldr r1, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1636)?)?;
    // 0x635e0: bic r1, r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,64)?)?;
    // 0x635e4: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x635e8: str r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1636)?)?;
    // 0x635ec: ldr r0, [r6, #0x94]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,148)?)?;
    // 0x635f0: str r0, [r4, #0x6d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1744)?)?;
    // 0x635f4: ldr r0, [r6, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,144)?)?;
    // 0x635f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x635fc: beq 0x6362c <entry+0xde2c>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6362c,false)?)?;
    // 0x63600: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // 0x63604: and r0, r1, r0, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,7,false)?)?;
    // 0x63608: ldr r1, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1636)?)?;
    // 0x6360c: bic r1, r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,128)?)?;
    // 0x63610: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x63614: add r1, r6, #156
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,156)?)?;
    // 0x63618: str r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1636)?)?;
    // 0x6361c: add r0, r4, #724
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,724)?)?;
    // 0x63620: ldm r1, {r2, r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63624: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x63628: stm r0, {r2, r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x6362c: ldr r0, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,420)?)?;
    // 0x63630: ldr r0, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    // 0x63634: str r0, [r4, #0x6bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1724)?)?;
    // 0x63638: str r0, [r4, #0x6c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1728)?)?;
    // 0x6363c: ldrb r0, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,378)?)?;
    // 0x63640: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x63644: beq 0x63654 <entry+0xde54>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63654,false)?)?;
    // 0x63648: ldrb r0, [r5, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,281)?)?;
    // 0x6364c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x63650: beq 0x636b8 <entry+0xdeb8>  @ imm = #0x60
    c.emit(arm32::branch(C::Eq,c.at as u32,0x636b8,false)?)?;
    // 0x63654: ldr r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1636)?)?;
    // 0x63658: tst r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,64)?)?;
    // 0x6365c: beq 0x636bc <entry+0xdebc>  @ imm = #0x58
    c.emit(arm32::branch(C::Eq,c.at as u32,0x636bc,false)?)?;
    // 0x63660: ldr r0, [r4, #0x6d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1744)?)?;
    // 0x63664: str r0, [r7, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1672)?)?;
    // 0x63668: ldrb r1, [r4, #0x171]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,369)?)?;
    // 0x6366c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x63670: beq 0x636a0 <entry+0xdea0>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x636a0,false)?)?;
    // 0x63674: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63678: ldr r2, [pc, #0x7c]         @ 0xdefc <entry+0xdefc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,124)?)?;
    // 0x6367c: mov r3, #1200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1200)?)?;
    // 0x63680: str r1, [r7, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,1672)?)?;
    // 0x63684: ldr r1, [pc, #0x6c]         @ 0xdef8 <entry+0xdef8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,108)?)?;
    // 0x63688: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x6368c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x63690: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x34a04
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x63694: ldr r1, [pc, #0x64]         @ 0xdf00 <entry+0xdf00>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,100)?)?;
    // 0x63698: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x6369c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x34a10
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x636a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x636a4: str r0, [r7, #0x670]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1648)?)?;
    // 0x636a8: ldr r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1636)?)?;
    // 0x636ac: bic r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,64)?)?;
    // 0x636b0: str r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1636)?)?;
    // 0x636b4: b 0x636bc <entry+0xdebc>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x636bc,false)?)?;
    // 0x636b8: ldr r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1636)?)?;
    // 0x636bc: tst r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,128)?)?;
    // 0x636c0: beq 0x636f0 <entry+0xdef0>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x636f0,false)?)?;
    // 0x636c4: ldr r0, [r4, #0x6d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1748)?)?;
    // 0x636c8: str r0, [r7, #0x68c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1676)?)?;
    // 0x636cc: ldr r0, [r4, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1752)?)?;
    // 0x636d0: str r0, [r7, #0x694]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1684)?)?;
    // 0x636d4: ldr r0, [r4, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1756)?)?;
    // 0x636d8: str r0, [r7, #0x698]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1688)?)?;
    // 0x636dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x636e0: str r0, [r7, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1656)?)?;
    // 0x636e4: ldr r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1636)?)?;
    // 0x636e8: bic r0, r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,128)?)?;
    // 0x636ec: str r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1636)?)?;
    // 0x636f0: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x636f4: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x636f8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_636F8:[u32;3]=[0x000f184b,0x000f1893,0x000e2788];
 for value in INLINE_DATA_636F8{c.emit(value)?;}
 // Executable A32 block 0x63704..0x63834.
 c.check(0x63704)?;
    // 0x63704: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x63708: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x6370c: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x63710: ldr r1, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,52)?)?;
    // 0x63714: ldrd r6, r7, [r1, #16]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::new(6)?,R::R1,16)?)?;
    // 0x63718: ldrb r1, [r4, #0x6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,6)?)?;
    // 0x6371c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x63720: bne 0x63738 <entry+0xdf38>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63738,false)?)?;
    // 0x63724: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63728: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x6372c: str r6, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,292)?)?;
    // 0x63730: str r0, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,296)?)?;
    // 0x63734: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63738: ldr r0, [r0, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,84)?)?;
    // 0x6373c: cmp r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,17)?)?;
    // 0x63740: bhi 0x637b4 <entry+0xdfb4>  @ imm = #0x6c
    c.emit(arm32::branch(C::Hi,c.at as u32,0x637b4,false)?)?;
    // 0x63744: mov r2, #260
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,260)?)?;
    // 0x63748: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x6374c: orr r2, r2, #98304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,98304)?)?;
    // 0x63750: tst r2, r1, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x63754: beq 0x63794 <entry+0xdf94>  @ imm = #0x38
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63794,false)?)?;
    // 0x63758: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x6375c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x63760: beq 0x63788 <entry+0xdf88>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63788,false)?)?;
    // 0x63764: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x63768: ldr r1, [r4, #0x26c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,620)?)?;
    // 0x6376c: sub r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x63770: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x63774: beq 0x63784 <entry+0xdf84>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63784,false)?)?;
    // 0x63778: ldr r2, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,300)?)?;
    // 0x6377c: sub r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x63780: blx 0x83c4c <entry+0x2e44c> @ imm = #0x204c4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x63784: str r0, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,296)?)?;
    // 0x63788: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x6378c: str r6, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,292)?)?;
    // 0x63790: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63794: cmp r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,17)?)?;
    // 0x63798: bne 0x637b4 <entry+0xdfb4>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x637b4,false)?)?;
    // 0x6379c: ldr r8, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,32)?)?;
    // 0x637a0: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // 0x637a4: bne 0x637c0 <entry+0xdfc0>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x637c0,false)?)?;
    // 0x637a8: ldr r2, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,296)?)?;
    // 0x637ac: ldr r5, [r4, #0x26c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,620)?)?;
    // 0x637b0: b 0x63808 <entry+0xe008>  @ imm = #0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x63808,false)?)?;
    // 0x637b4: ldr r0, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,288)?)?;
    // 0x637b8: sub r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x637bc: b 0x63728 <entry+0xdf28>  @ imm = #-0x9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x63728,false)?)?;
    // 0x637c0: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x637c4: add r1, r4, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,280)?)?;
    // 0x637c8: ldr r5, [r4, #0x26c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,620)?)?;
    // 0x637cc: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x637d0: addne r1, r4, #288
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::new(4)?,288)?)?;
    // 0x637d4: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x637d8: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x637dc: sub r2, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x637e0: beq 0x63804 <entry+0xe004>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63804,false)?)?;
    // 0x637e4: add r1, r4, #300
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,300)?)?;
    // 0x637e8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x637ec: addne r1, r4, #40
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::new(4)?,40)?)?;
    // 0x637f0: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x637f4: sub r1, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x637f8: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x637fc: blx 0x83c4c <entry+0x2e44c> @ imm = #0x20448
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x63800: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x63804: str r2, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,296)?)?;
    // 0x63808: ldr r1, [pc, #0x24]         @ 0xe034 <entry+0xe034>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,36)?)?;
    // 0x6380c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x63810: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x63814: str r6, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,292)?)?;
    // 0x63818: str r8, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,36)?)?;
    // 0x6381c: str r5, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,40)?)?;
    // 0x63820: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x34b94
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x63824: ldr r1, [pc, #0xc]          @ 0xe038 <entry+0xe038>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,12)?)?;
    // 0x63828: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x6382c: pop {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63830: b 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x34ba4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,false)?)?;
 c.check(0x63834)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_63834:[u32;2]=[0x000f18a5,0x000e2788];
 for value in INLINE_DATA_63834{c.emit(value)?;}
 // Executable A32 block 0x6383c..0x63b00.
 c.check(0x6383c)?;
    // 0x6383c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x63840: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x63844: sub sp, sp, #788
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,788)?)?;
    // 0x63848: sub sp, sp, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,1024)?)?;
    // 0x6384c: ldr r1, [pc, #0xba4]        @ 0xebf8 <entry+0xebf8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2980)?)?;
    // 0x63850: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x63854: sub r0, r11, #192
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,192)?)?;
    // 0x63858: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x6385c: bl 0x471c <entry+0xfffffffffffaef1c> @ imm = #-0x5f148
    c.emit(arm32::branch(C::Al,c.at as u32,0x471c,true)?)?;
    // 0x63860: ldr r0, [pc, #0xb94]        @ 0xebfc <entry+0xebfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2964)?)?;
    // 0x63864: sub r1, r11, #224
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,224)?)?;
    // 0x63868: ldr r9, [pc, #0xba4]        @ 0xec14 <entry+0xec14>
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::PC,2980)?)?;
    // 0x6386c: mov lr, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,2)?)?;
    // 0x63870: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x63874: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x63878: ldr r10, [pc, #0xbc0]       @ 0xec40 <entry+0xec40>
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::PC,3008)?)?;
    // 0x6387c: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63880: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63884: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63888: ldr r0, [pc, #0xb70]        @ 0xec00 <entry+0xec00>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2928)?)?;
    // 0x6388c: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63890: sub r1, r11, #256
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,256)?)?;
    // 0x63894: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63898: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x6389c: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638a0: ldr r0, [pc, #0xb5c]        @ 0xec04 <entry+0xec04>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2908)?)?;
    // 0x638a4: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638a8: sub r1, r11, #288
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,288)?)?;
    // 0x638ac: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x638b0: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x638b4: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638b8: ldr r0, [pc, #0xb48]        @ 0xec08 <entry+0xec08>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2888)?)?;
    // 0x638bc: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638c0: sub r1, r11, #320
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,320)?)?;
    // 0x638c4: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x638c8: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x638cc: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638d0: ldr r0, [pc, #0xb34]        @ 0xec0c <entry+0xec0c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2868)?)?;
    // 0x638d4: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638d8: sub r1, r11, #352
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,352)?)?;
    // 0x638dc: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x638e0: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x638e4: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638e8: ldr r0, [pc, #0xb20]        @ 0xec10 <entry+0xec10>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2848)?)?;
    // 0x638ec: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638f0: sub r1, r11, #368
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,368)?)?;
    // 0x638f4: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638f8: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x638fc: str lr, [r11, #-0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-372)?)?;
    // 0x63900: str r8, [r11, #-0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-380)?)?;
    // 0x63904: str r12, [r11, #-0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R11,-388)?)?;
    // 0x63908: str r8, [r11, #-0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-396)?)?;
    // 0x6390c: str lr, [r11, #-0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-404)?)?;
    // 0x63910: str r8, [r11, #-0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-376)?)?;
    // 0x63914: str r8, [r11, #-0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-384)?)?;
    // 0x63918: str r8, [r11, #-0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-392)?)?;
    // 0x6391c: str lr, [r11, #-0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-400)?)?;
    // 0x63920: str r8, [r11, #-0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-408)?)?;
    // 0x63924: sub r7, r11, #456
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,456)?)?;
    // 0x63928: ldm r9!, {r0, r1, r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(9)?,R::R0.bit()|R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x6392c: stm r7!, {r0, r1, r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(7)?,R::R0.bit()|R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63930: ldm r9, {r0, r1, r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(9)?,R::R0.bit()|R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63934: sub r9, r11, #568
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,568)?)?;
    // 0x63938: stm r7, {r0, r1, r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(7)?,R::R0.bit()|R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x6393c: ldr r0, [pc, #0xad4]        @ 0xec18 <entry+0xec18>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2772)?)?;
    // 0x63940: sub r1, r11, #472
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,472)?)?;
    // 0x63944: ldm r0, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63948: ldr r0, [pc, #0xacc]        @ 0xec1c <entry+0xec1c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2764)?)?;
    // 0x6394c: stm r1, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63950: sub r1, r11, #488
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,488)?)?;
    // 0x63954: ldm r0, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63958: ldr r0, [pc, #0xac0]        @ 0xec20 <entry+0xec20>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2752)?)?;
    // 0x6395c: stm r1, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63960: sub r1, r11, #504
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,504)?)?;
    // 0x63964: ldm r0, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63968: ldr r0, [pc, #0xab4]        @ 0xec24 <entry+0xec24>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2740)?)?;
    // 0x6396c: stm r1, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63970: sub r1, r11, #520
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,520)?)?;
    // 0x63974: ldm r0, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63978: ldr r0, [pc, #0xaa8]        @ 0xec28 <entry+0xec28>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2728)?)?;
    // 0x6397c: stm r1, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63980: sub r1, r11, #536
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,536)?)?;
    // 0x63984: ldm r0, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63988: ldr r0, [pc, #0xa9c]        @ 0xec2c <entry+0xec2c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2716)?)?;
    // 0x6398c: stm r1, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63990: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x63994: ldm r0!, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63998: stm r1!, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x6399c: ldm r0, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x639a0: ldr r0, [pc, #0xa88]        @ 0xec30 <entry+0xec30>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2696)?)?;
    // 0x639a4: stm r1, {r2, r3, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x639a8: sub r5, r11, #584
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,584)?)?;
    // 0x639ac: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x639b0: ldr r0, [pc, #0xa7c]        @ 0xec34 <entry+0xec34>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2684)?)?;
    // 0x639b4: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x639b8: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x639bc: sub r1, r11, #616
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,616)?)?;
    // 0x639c0: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x639c4: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x639c8: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x639cc: ldr r0, [pc, #0xa64]        @ 0xec38 <entry+0xec38>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2660)?)?;
    // 0x639d0: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x639d4: sub r1, r11, #648
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,648)?)?;
    // 0x639d8: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x639dc: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x639e0: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x639e4: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x639e8: ldr r0, [pc, #0xa4c]        @ 0xec3c <entry+0xec3c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,2636)?)?;
    // 0x639ec: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x639f0: sub r1, r11, #680
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,680)?)?;
    // 0x639f4: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x639f8: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x639fc: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63a00: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63a04: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63a08: sub r7, r11, #712
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,712)?)?;
    // 0x63a0c: ldm r10!, {r0, r2, r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63a10: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x63a14: stm r1!, {r0, r2, r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x63a18: ldm r10, {r0, r2, r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63a1c: add r10, sp, #664
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,664)?)?;
    // 0x63a20: stm r1, {r0, r2, r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x63a24: mvn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,1)?)?;
    // 0x63a28: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x63a2c: mov r2, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,384)?)?;
    // 0x63a30: str r8, [r11, #-0x2f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-756)?)?;
    // 0x63a34: str lr, [r11, #-0x2fc]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-764)?)?;
    // 0x63a38: str r8, [r11, #-0x304]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-772)?)?;
    // 0x63a3c: str r8, [r11, #-0x30c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-780)?)?;
    // 0x63a40: str lr, [r11, #-0x314]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-788)?)?;
    // 0x63a44: str lr, [r11, #-0x2d4]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-724)?)?;
    // 0x63a48: str r8, [r11, #-0x2d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-728)?)?;
    // 0x63a4c: str r8, [r11, #-0x2dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-732)?)?;
    // 0x63a50: str lr, [r11, #-0x2e0]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-736)?)?;
    // 0x63a54: str r8, [r11, #-0x2e4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-740)?)?;
    // 0x63a58: str r12, [r11, #-0x2e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R11,-744)?)?;
    // 0x63a5c: str r8, [r11, #-0x2f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-760)?)?;
    // 0x63a60: str r8, [r11, #-0x300]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-768)?)?;
    // 0x63a64: str r8, [r11, #-0x308]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-776)?)?;
    // 0x63a68: str r8, [r11, #-0x310]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-784)?)?;
    // 0x63a6c: str r8, [r11, #-0x318]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-792)?)?;
    // 0x63a70: str r0, [r11, #-0x2cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-716)?)?;
    // 0x63a74: str r0, [r11, #-0x2ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-748)?)?;
    // 0x63a78: str r1, [r11, #-0x2d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-720)?)?;
    // 0x63a7c: str r1, [r11, #-0x2f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-752)?)?;
    // 0x63a80: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x63a84: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63a88: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x608a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x63a8c: add r6, sp, #536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::SP,536)?)?;
    // 0x63a90: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63a94: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x63a98: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x63a9c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x608b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x63aa0: add r0, sp, #408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,408)?)?;
    // 0x63aa4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63aa8: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x63aac: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x608c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x63ab0: add r0, sp, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,280)?)?;
    // 0x63ab4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63ab8: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x63abc: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x608d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x63ac0: add r8, sp, #152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,152)?)?;
    // 0x63ac4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63ac8: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x63acc: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x63ad0: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x608ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x63ad4: add r0, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,24)?)?;
    // 0x63ad8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63adc: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x63ae0: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x608fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x63ae4: ldrb r2, [r4, #0x468]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1128)?)?;
    // 0x63ae8: sub r0, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,8)?)?;
    // 0x63aec: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // 0x63af0: bhi 0x63b78 <entry+0xe378>  @ imm = #0x80
    c.emit(arm32::branch(C::Hi,c.at as u32,0x63b78,false)?)?;
    // 0x63af4: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x63af8: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x63afc: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x63b00)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_63B00:[u32;10]=[0x00063bb4,0x00063b28,0x00063b28,0x00063be8,0x00063dd0,0x00063b44,0x00063b44,0x00063e04,0x00063e38,0x00063e28];
 for value in INLINE_DATA_63B00{c.emit(value)?;}
 // Executable A32 block 0x63b28..0x64000.
 c.check(0x63b28)?;
    // 0x63b28: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x63b2c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x63b30: sub r10, r11, #616
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R11,616)?)?;
    // 0x63b34: strb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2454)?)?;
    // 0x63b38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63b3c: strb r0, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2457)?)?;
    // 0x63b40: b 0x63f90 <entry+0xe790>  @ imm = #0x448
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f90,false)?)?;
    // 0x63b44: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63b48: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x63b4c: sub r9, r11, #720
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,720)?)?;
    // 0x63b50: sub r5, r11, #724
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,724)?)?;
    // 0x63b54: sub r10, r11, #728
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R11,728)?)?;
    // 0x63b58: sub r7, r11, #740
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,740)?)?;
    // 0x63b5c: strb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2454)?)?;
    // 0x63b60: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63b64: strb r0, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2457)?)?;
    // 0x63b68: sub r0, r11, #732
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,732)?)?;
    // 0x63b6c: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63b70: sub r0, r11, #736
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,736)?)?;
    // 0x63b74: b 0x63f8c <entry+0xe78c>  @ imm = #0x410
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f8c,false)?)?;
    // 0x63b78: ldr r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
    // 0x63b7c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x63b80: beq 0x63b28 <entry+0xe328>  @ imm = #-0x60
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63b28,false)?)?;
    // 0x63b84: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x63b88: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63b8c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x63b90: bne 0x63b28 <entry+0xe328>  @ imm = #-0x70
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63b28,false)?)?;
    // 0x63b94: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63b98: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x63b9c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x63ba0: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x63ba4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x63ba8: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63bac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63bb0: b 0x63f8c <entry+0xe78c>  @ imm = #0x3d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f8c,false)?)?;
    // 0x63bb4: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x63bb8: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x63bbc: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x63bc0: sub r9, r11, #192
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,192)?)?;
    // 0x63bc4: sub r5, r11, #224
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,224)?)?;
    // 0x63bc8: sub r10, r11, #256
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R11,256)?)?;
    // 0x63bcc: sub r7, r11, #352
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,352)?)?;
    // 0x63bd0: strb r2, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2454)?)?;
    // 0x63bd4: strb r0, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2457)?)?;
    // 0x63bd8: sub r0, r11, #288
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,288)?)?;
    // 0x63bdc: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63be0: sub r0, r11, #320
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,320)?)?;
    // 0x63be4: b 0x63f8c <entry+0xe78c>  @ imm = #0x3a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f8c,false)?)?;
    // 0x63be8: ldrb r1, [r4, #0x561]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1377)?)?;
    // 0x63bec: ldrb r0, [r4, #0x399]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,921)?)?;
    // 0x63bf0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x63bf4: bhi 0x63c04 <entry+0xe404>  @ imm = #0x8
    c.emit(arm32::branch(C::Hi,c.at as u32,0x63c04,false)?)?;
    // 0x63bf8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x63bfc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63c00: bne 0x63f38 <entry+0xe738>  @ imm = #0x330
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63f38,false)?)?;
    // 0x63c04: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x63c08: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x63c0c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x63c10: lsl r1, r6, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::new(6)?,Shift::Lsl,R::R1,false)?)?;
    // 0x63c14: strb r1, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2454)?)?;
    // 0x63c18: bne 0x63c30 <entry+0xe430>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63c30,false)?)?;
    // 0x63c1c: mov r0, #920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,920)?)?;
    // 0x63c20: ldrsb r0, [r4, r0]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R0,R::new(4)?,R::R0)?)?;
    // 0x63c24: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x63c28: lsl r0, r6, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(6)?,Shift::Lsl,R::R0,false)?)?;
    // 0x63c2c: strb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2454)?)?;
    // 0x63c30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63c34: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x63c38: add r5, sp, #536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,536)?)?;
    // 0x63c3c: mvn r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,3)?)?;
    // 0x63c40: add r9, sp, #664
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::SP,664)?)?;
    // 0x63c44: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x63c48: strb r0, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2457)?)?;
    // 0x63c4c: sub r0, r11, #64
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,64)?)?;
    // 0x63c50: mvn r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(4)?,R::R0,0)?)?;
    // 0x63c54: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x63c58: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x63c5c: add r0, sp, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,280)?)?;
    // 0x63c60: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63c64: add r0, sp, #152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,152)?)?;
    // 0x63c68: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x63c6c: add r7, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R2,1)?)?;
    // 0x63c70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63c74: tst r7, r6, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(7)?,R::new(6)?,Shift::Lsl,R::R0,false)?)?;
    // 0x63c78: bne 0x63c90 <entry+0xe490>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63c90,false)?)?;
    // 0x63c7c: add r3, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,1)?)?;
    // 0x63c80: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x63c84: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // 0x63c88: blt 0x63c74 <entry+0xe474>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Lt,c.at as u32,0x63c74,false)?)?;
    // 0x63c8c: b 0x63c94 <entry+0xe494>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x63c94,false)?)?;
    // 0x63c90: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x63c94: rsb r0, r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(3)?,5)?)?;
    // 0x63c98: sub r3, r11, #64
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R11,64)?)?;
    // 0x63c9c: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x63ca0: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x63ca4: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x63ca8: strb r0, [r3, r2]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x63cac: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x63cb0: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x63cb4: str r12, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,0)?)?;
    // 0x63cb8: rsb r0, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,5)?)?;
    // 0x63cbc: bic lr, r7, r3, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Bic,R::LR,R::new(7)?,R::new(3)?,Shift::Lsl,R::R0,false)?)?;
    // 0x63cc0: add r10, r4, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(10)?,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x63cc4: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x63cc8: beq 0x63d08 <entry+0xe508>  @ imm = #0x38
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63d08,false)?)?;
    // 0x63ccc: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x63cd0: blt 0x63d64 <entry+0xe564>  @ imm = #0x8c
    c.emit(arm32::branch(C::Lt,c.at as u32,0x63d64,false)?)?;
    // 0x63cd4: ldr r0, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,-4)?)?;
    // 0x63cd8: sub r3, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(8)?,1)?)?;
    // 0x63cdc: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x63ce0: bhs 0x63d64 <entry+0xe564>  @ imm = #0x7c
    c.emit(arm32::branch(C::Cs,c.at as u32,0x63d64,false)?)?;
    // 0x63ce4: add r0, r1, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x63ce8: ldr r0, [r9, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,2)?)?;
    // 0x63cec: add r3, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x63cf0: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x63cf4: blt 0x63d24 <entry+0xe524>  @ imm = #0x28
    c.emit(arm32::branch(C::Lt,c.at as u32,0x63d24,false)?)?;
    // 0x63cf8: ldr r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,16)?)?;
    // 0x63cfc: add r6, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x63d00: ldrb r6, [r6, r2]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(6)?,R::new(6)?,R::R2,Shift::Lsl,0)?)?;
    // 0x63d04: b 0x63d28 <entry+0xe528>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x63d28,false)?)?;
    // 0x63d08: add r0, r1, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // 0x63d0c: add r12, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,1)?)?;
    // 0x63d10: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x63d14: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x63d18: str r12, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,0)?)?;
    // 0x63d1c: str r10, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,16)?)?;
    // 0x63d20: b 0x63cc4 <entry+0xe4c4>  @ imm = #-0x64
    c.emit(arm32::branch(C::Al,c.at as u32,0x63cc4,false)?)?;
    // 0x63d24: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x63d28: cmp lr, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x63d2c: beq 0x63d64 <entry+0xe564>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63d64,false)?)?;
    // 0x63d30: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x63d34: beq 0x63d4c <entry+0xe54c>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63d4c,false)?)?;
    // 0x63d38: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x63d3c: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x63d40: ldrb r3, [r3, r2]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x63d44: cmp r6, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x63d48: beq 0x63d64 <entry+0xe564>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63d64,false)?)?;
    // 0x63d4c: add r3, r1, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // 0x63d50: add r12, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,1)?)?;
    // 0x63d54: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x63d58: add r3, r9, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(9)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x63d5c: str r12, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,0)?)?;
    // 0x63d60: str r0, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,16)?)?;
    // 0x63d64: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x63d68: cmp r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,4)?)?;
    // 0x63d6c: bne 0x63cc4 <entry+0xe4c4>  @ imm = #-0xb0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63cc4,false)?)?;
    // 0x63d70: ldr r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,8)?)?;
    // 0x63d74: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x63d78: add r3, sp, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,280)?)?;
    // 0x63d7c: add r10, sp, #408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,408)?)?;
    // 0x63d80: add r1, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3)?)?;
    // 0x63d84: add r5, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4)?)?;
    // 0x63d88: cmp r7, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,32)?)?;
    // 0x63d8c: bic r0, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x63d90: str r7, [r10, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,2)?)?;
    // 0x63d94: str r0, [r3, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // 0x63d98: add r0, sp, #152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,152)?)?;
    // 0x63d9c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x63da0: str r6, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x63da4: add r0, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,24)?)?;
    // 0x63da8: str r3, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x63dac: sub r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(4)?,1)?)?;
    // 0x63db0: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x63db4: bne 0x63c6c <entry+0xe46c>  @ imm = #-0x150
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63c6c,false)?)?;
    // 0x63db8: ldr r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,4)?)?;
    // 0x63dbc: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x63dc0: add r5, sp, #536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,536)?)?;
    // 0x63dc4: add r7, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::SP,24)?)?;
    // 0x63dc8: ldrb r2, [r4, #0x468]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1128)?)?;
    // 0x63dcc: b 0x63f90 <entry+0xe790>  @ imm = #0x1bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f90,false)?)?;
    // 0x63dd0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63dd4: mov r9, r10
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(10)?))?;
    // 0x63dd8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63ddc: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x63de0: sub r7, r11, #744
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,744)?)?;
    // 0x63de4: mov r5, r6
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(6)?))?;
    // 0x63de8: add r10, sp, #408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,408)?)?;
    // 0x63dec: strb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2454)?)?;
    // 0x63df0: strb r0, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2457)?)?;
    // 0x63df4: add r0, sp, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,280)?)?;
    // 0x63df8: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x63dfc: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63e00: b 0x63f90 <entry+0xe790>  @ imm = #0x188
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f90,false)?)?;
    // 0x63e04: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x63e08: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x63e0c: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x63e10: sub r9, r11, #456
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,456)?)?;
    // 0x63e14: sub r5, r11, #472
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,472)?)?;
    // 0x63e18: strb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2454)?)?;
    // 0x63e1c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x63e20: strb r0, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2457)?)?;
    // 0x63e24: b 0x63f78 <entry+0xe778>  @ imm = #0x14c
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f78,false)?)?;
    // 0x63e28: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x63e2c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63e30: mov r2, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,17)?)?;
    // 0x63e34: b 0x63e50 <entry+0xe650>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x63e50,false)?)?;
    // 0x63e38: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x63e3c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x63e40: bne 0x63e7c <entry+0xe67c>  @ imm = #0x34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63e7c,false)?)?;
    // 0x63e44: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x63e48: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x63e4c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63e50: strb r1, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2454)?)?;
    // 0x63e54: strb r0, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2457)?)?;
    // 0x63e58: sub r0, r11, #384
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,384)?)?;
    // 0x63e5c: sub r7, r11, #408
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,408)?)?;
    // 0x63e60: sub r10, r11, #400
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R11,400)?)?;
    // 0x63e64: sub r5, r11, #376
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,376)?)?;
    // 0x63e68: sub r9, r11, #368
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,368)?)?;
    // 0x63e6c: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x63e70: sub r0, r11, #392
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,392)?)?;
    // 0x63e74: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63e78: b 0x63f90 <entry+0xe790>  @ imm = #0x110
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f90,false)?)?;
    // 0x63e7c: ldr r5, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,924)?)?;
    // 0x63e80: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63e84: sub r8, r11, #472
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,472)?)?;
    // 0x63e88: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
    // 0x63e8c: mvn r2, r5
    c.emit(arm32::data_register(C::Al,Op::Mvn,R::R2,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x63e90: and r7, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R0,255)?)?;
    // 0x63e94: strb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2454)?)?;
    // 0x63e98: strb r5, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2457)?)?;
    // 0x63e9c: str r1, [r11, #-0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-424)?)?;
    // 0x63ea0: str r2, [r11, #-0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-456)?)?;
    // 0x63ea4: str r1, [r11, #-0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-452)?)?;
    // 0x63ea8: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x63eac: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x63eb0: str r0, [r11, #-0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-488)?)?;
    // 0x63eb4: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x63eb8: blo 0x63f70 <entry+0xe770>  @ imm = #0xb0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x63f70,false)?)?;
    // 0x63ebc: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x63ec0: mvn r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,3)?)?;
    // 0x63ec4: movls r0, #2
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x63ec8: add r2, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x63ecc: sub r0, r11, #504
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,504)?)?;
    // 0x63ed0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63ed4: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63ed8: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x63edc: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x60cf8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x63ee0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63ee4: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x63ee8: sub r9, r11, #456
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,456)?)?;
    // 0x63eec: sub r10, r11, #488
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R11,488)?)?;
    // 0x63ef0: add r2, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x63ef4: str r0, [r10, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R0,Shift::Lsl,2)?)?;
    // 0x63ef8: str r1, [r9, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(9)?,R::R2,Shift::Lsl,2)?)?;
    // 0x63efc: add r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x63f00: add r2, r9, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(9)?,R::R2,Shift::Lsl,2,false)?)?;
    // 0x63f04: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x63f08: str r1, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,4)?)?;
    // 0x63f0c: add r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,1)?)?;
    // 0x63f10: mvn r1, r0
    c.emit(arm32::data_register(C::Al,Op::Mvn,R::R1,R::R0,R::R0,Shift::Lsl,0,false)?)?;
    // 0x63f14: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x63f18: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x63f1c: blo 0x63ef0 <entry+0xe6f0>  @ imm = #-0x34
    c.emit(arm32::branch(C::Cc,c.at as u32,0x63ef0,false)?)?;
    // 0x63f20: sub r0, r11, #520
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,520)?)?;
    // 0x63f24: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x63f28: sub r7, r11, #536
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,536)?)?;
    // 0x63f2c: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x63f30: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x63f34: b 0x63f8c <entry+0xe78c>  @ imm = #0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f8c,false)?)?;
    // 0x63f38: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x63f3c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63f40: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // 0x63f44: sub r7, r11, #792
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,792)?)?;
    // 0x63f48: sub r10, r11, #768
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R11,768)?)?;
    // 0x63f4c: sub r5, r11, #760
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,760)?)?;
    // 0x63f50: sub r9, r11, #752
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,752)?)?;
    // 0x63f54: strb r1, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2454)?)?;
    // 0x63f58: strb r0, [r4, #0x999]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2457)?)?;
    // 0x63f5c: sub r0, r11, #784
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,784)?)?;
    // 0x63f60: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x63f64: sub r0, r11, #776
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,776)?)?;
    // 0x63f68: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63f6c: b 0x63f90 <entry+0xe790>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x63f90,false)?)?;
    // 0x63f70: sub r9, r11, #456
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,456)?)?;
    // 0x63f74: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x63f78: sub r0, r11, #504
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,504)?)?;
    // 0x63f7c: sub r10, r11, #488
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R11,488)?)?;
    // 0x63f80: sub r7, r11, #536
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,536)?)?;
    // 0x63f84: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x63f88: sub r0, r11, #520
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,520)?)?;
    // 0x63f8c: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x63f90: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63f94: cmp r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,10)?)?;
    // 0x63f98: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x63f9c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x63fa0: strb r0, [r4, #0x9a8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2472)?)?;
    // 0x63fa4: beq 0x6406c <entry+0xe86c>  @ imm = #0xc0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6406c,false)?)?;
    // 0x63fa8: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x63fac: ldrne r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::SP,12)?)?;
    // 0x63fb0: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x63fb4: beq 0x6406c <entry+0xe86c>  @ imm = #0xb0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6406c,false)?)?;
    // 0x63fb8: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x63fbc: cmpne r10, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x63fc0: beq 0x6406c <entry+0xe86c>  @ imm = #0xa4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6406c,false)?)?;
    // 0x63fc4: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x63fc8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x63fcc: beq 0x6406c <entry+0xe86c>  @ imm = #0x98
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6406c,false)?)?;
    // 0x63fd0: ldrb r2, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2454)?)?;
    // 0x63fd4: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x63fd8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x63fdc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x63fe0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x63fe4: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x63fe8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x63fec: movhi r1, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x63ff0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x63ff4: movhi r3, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x63ff8: orrs r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,true)?)?;
    // 0x63ffc: ldrb r3, [r4, #0x33c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,828)?)?;
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
