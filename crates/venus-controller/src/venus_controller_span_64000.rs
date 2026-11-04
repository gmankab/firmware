// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x64000,0x65000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x64000;
pub const END:usize=0x65000;
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
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x64000..0x643f8.
 c.check(0x64000)?;
    // 0x64000: movne r8, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(8)?,R::R0,2)?)?;
    // 0x64004: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x64008: moveq r8, r1
    c.emit(arm32::mov_register(C::Eq,R::new(8)?,R::R1))?;
    // 0x6400c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x64010: beq 0x64038 <entry+0xe838>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64038,false)?)?;
    // 0x64014: add r0, r4, #170
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,170)?)?;
    // 0x64018: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x6401c: add r1, r0, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2304)?)?;
    // 0x64020: ldr r0, [r7], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(7)?,4)?)?;
    // 0x64024: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x64028: strb r0, [r1], #1
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::R0,R::R1,1)?)?;
    // 0x6402c: ldrb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2454)?)?;
    // 0x64030: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x64034: blo 0x64020 <entry+0xe820>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x64020,false)?)?;
    // 0x64038: ldr r1, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2732)?)?;
    // 0x6403c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64040: beq 0x64050 <entry+0xe850>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64050,false)?)?;
    // 0x64044: ldr r1, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,644)?)?;
    // 0x64048: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x6404c: beq 0x64088 <entry+0xe888>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64088,false)?)?;
    // 0x64050: add r0, r4, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,640)?)?;
    // 0x64054: bl 0x5886c <entry+0x306c>  @ imm = #-0xb7f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x64058: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6405c: beq 0x64084 <entry+0xe884>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64084,false)?)?;
    // 0x64060: and r0, r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(8)?,255)?)?;
    // 0x64064: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x64068: b 0x6408c <entry+0xe88c>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x6408c,false)?)?;
    // 0x6406c: ldr r2, [pc, #0x3d0]        @ 0xec44 <entry+0xec44>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,976)?)?;
    // 0x64070: mov r1, #246
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,246)?)?;
    // 0x64074: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64078: orr r1, r1, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3584)?)?;
    // 0x6407c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x628d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x64080: b 0x643f0 <entry+0xebf0>  @ imm = #0x368
    c.emit(arm32::branch(C::Al,c.at as u32,0x643f0,false)?)?;
    // 0x64084: ldrb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2454)?)?;
    // 0x64088: and r1, r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(8)?,255)?)?;
    // 0x6408c: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x64090: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x64094: strb r0, [r4, #0x99a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2458)?)?;
    // 0x64098: ldr r1, [r2, #0x9cc]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R2,2508)?)?;
    // 0x6409c: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x640a0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x640a4: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x640a8: bne 0x640d4 <entry+0xe8d4>  @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x640d4,false)?)?;
    // 0x640ac: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x640b0: rsb r0, r0, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // 0x640b4: add r7, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,15)?)?;
    // 0x640b8: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x640bc: bl 0x2efc <entry+0xfffffffffffad6fc> @ imm = #-0x611c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2efc,true)?)?;
    // 0x640c0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x640c4: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x640c8: str r0, [r4, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2508)?)?;
    // 0x640cc: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x60ee8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x640d0: ldrb r0, [r4, #0x99a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2458)?)?;
    // 0x640d4: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x640d8: str r10, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,8)?)?;
    // 0x640dc: beq 0x6410c <entry+0xe90c>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6410c,false)?)?;
    // 0x640e0: add r0, r4, #2512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,2512)?)?;
    // 0x640e4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x640e8: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x640ec: ldr r3, [r4, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2508)?)?;
    // 0x640f0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x640f4: add r3, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x640f8: add r2, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,15)?)?;
    // 0x640fc: str r3, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x64100: ldrb r3, [r4, #0x99a]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,2458)?)?;
    // 0x64104: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x64108: bls 0x640ec <entry+0xe8ec>  @ imm = #-0x24
    c.emit(arm32::branch(C::Ls,c.at as u32,0x640ec,false)?)?;
    // 0x6410c: ldrb r0, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2454)?)?;
    // 0x64110: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64114: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x64118: beq 0x64284 <entry+0xea84>  @ imm = #0x164
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64284,false)?)?;
    // 0x6411c: ldr r10, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,4)?)?;
    // 0x64120: lsl lr, r6, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::LR,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x64124: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x64128: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6412c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x64130: ldrb r3, [r4, #0x468]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1128)?)?;
    // 0x64134: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x64138: cmp r3, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,12)?)?;
    // 0x6413c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x64140: beq 0x64158 <entry+0xe958>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64158,false)?)?;
    // 0x64144: ldr r1, [r5, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(5)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x64148: and r3, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R0,255)?)?;
    // 0x6414c: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x64150: bls 0x64160 <entry+0xe960>  @ imm = #0x8
    c.emit(arm32::branch(C::Ls,c.at as u32,0x64160,false)?)?;
    // 0x64154: and r3, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R1,255)?)?;
    // 0x64158: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x6415c: str r3, [r4, #0x534]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1332)?)?;
    // 0x64160: ldr r1, [r10], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // 0x64164: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x64168: beq 0x64270 <entry+0xea70>  @ imm = #0x100
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64270,false)?)?;
    // 0x6416c: strb r2, [r1, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,9)?)?;
    // 0x64170: ldr r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    // 0x64174: ldr r3, [r3, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x64178: strb r3, [r1, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,10)?)?;
    // 0x6417c: strb r2, [r1, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,2)?)?;
    // 0x64180: strb r2, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x64184: strb r2, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,1)?)?;
    // 0x64188: ldr r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,8)?)?;
    // 0x6418c: ldr r3, [r3, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x64190: strb r3, [r1, #0x8]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,8)?)?;
    // 0x64194: ldr r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // 0x64198: ldr r3, [r3, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x6419c: strb r3, [r1, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,3)?)?;
    // 0x641a0: ldr r3, [r5, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(5)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x641a4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x641a8: beq 0x64270 <entry+0xea70>  @ imm = #0xc0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64270,false)?)?;
    // 0x641ac: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x641b0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x641b4: add r12, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x641b8: strb r2, [r12, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R12,11)?)?;
    // 0x641bc: ldr r6, [r9, r3, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(9)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x641c0: strb r6, [r12, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R12,4)?)?;
    // 0x641c4: ldr r6, [r9, r3, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(9)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x641c8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x641cc: bge 0x64218 <entry+0xea18>  @ imm = #0x44
    c.emit(arm32::branch(C::Ge,c.at as u32,0x64218,false)?)?;
    // 0x641d0: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // 0x641d4: ldr r6, [r6, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x641d8: cmp r8, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x641dc: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x641e0: movlo r6, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x641e4: strb r6, [r12, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R12,11)?)?;
    // 0x641e8: ldrb r6, [r1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // 0x641ec: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x641f0: strb r6, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x641f4: ldr r6, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2732)?)?;
    // 0x641f8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x641fc: beq 0x64210 <entry+0xea10>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64210,false)?)?;
    // 0x64200: ldr r6, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,644)?)?;
    // 0x64204: cmp r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,8192)?)?;
    // 0x64208: addne r8, r8, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x6420c: b 0x64214 <entry+0xea14>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x64214,false)?)?;
    // 0x64210: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x64214: ldr r6, [r9, r3, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(9)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x64218: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x6421c: blt 0x64260 <entry+0xea60>  @ imm = #0x3c
    c.emit(arm32::branch(C::Lt,c.at as u32,0x64260,false)?)?;
    // 0x64220: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // 0x64224: ldr r6, [r6, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x64228: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x6422c: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x64230: strb r6, [r12, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R12,11)?)?;
    // 0x64234: ldrb r6, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,1)?)?;
    // 0x64238: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x6423c: strb r6, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,1)?)?;
    // 0x64240: ldr r6, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2732)?)?;
    // 0x64244: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x64248: beq 0x6425c <entry+0xea5c>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6425c,false)?)?;
    // 0x6424c: ldr r6, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,644)?)?;
    // 0x64250: cmp r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,8192)?)?;
    // 0x64254: addne r8, r8, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x64258: b 0x64260 <entry+0xea60>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x64260,false)?)?;
    // 0x6425c: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x64260: ldr r6, [r5, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(5)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x64264: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x64268: cmp r3, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x6426c: blo 0x641b4 <entry+0xe9b4>  @ imm = #-0xc0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x641b4,false)?)?;
    // 0x64270: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x64274: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x64278: add r9, r9, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::new(9)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x6427c: cmp r7, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x64280: bne 0x64130 <entry+0xe930>  @ imm = #-0x158
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64130,false)?)?;
    // 0x64284: ldr r10, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,0)?)?;
    // 0x64288: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x6428c: beq 0x643f0 <entry+0xebf0>  @ imm = #0x15c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x643f0,false)?)?;
    // 0x64290: ldrb r0, [r4, #0x33c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,828)?)?;
    // 0x64294: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x64298: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6429c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x642a0: beq 0x643b8 <entry+0xebb8>  @ imm = #0x110
    c.emit(arm32::branch(C::Eq,c.at as u32,0x643b8,false)?)?;
    // 0x642a4: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x642a8: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x642ac: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x642b0: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x642b4: strb r12, [r0, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R0,9)?)?;
    // 0x642b8: ldr r1, [r11, #-0x308]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-776)?)?;
    // 0x642bc: strb r1, [r0, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,10)?)?;
    // 0x642c0: strb r12, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R0,2)?)?;
    // 0x642c4: strb r12, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R0,0)?)?;
    // 0x642c8: strb r12, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R0,1)?)?;
    // 0x642cc: ldr r1, [r11, #-0x300]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-768)?)?;
    // 0x642d0: strb r1, [r0, #0x8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x642d4: ldr r1, [r11, #-0x310]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-784)?)?;
    // 0x642d8: strb r1, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,3)?)?;
    // 0x642dc: ldr r2, [r11, #-0x2f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-760)?)?;
    // 0x642e0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x642e4: beq 0x643a4 <entry+0xeba4>  @ imm = #0xb8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x643a4,false)?)?;
    // 0x642e8: ldr lr, [r11, #-0x310]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-784)?)?;
    // 0x642ec: sub r6, r11, #752
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,752)?)?;
    // 0x642f0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x642f4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x642f8: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x642fc: mov r9, lr
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::LR))?;
    // 0x64300: movne r9, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // 0x64304: add r1, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x64308: strb r12, [r1, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R1,11)?)?;
    // 0x6430c: ldr r3, [r6, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(6)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x64310: strb r3, [r1, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x64314: ldr r3, [r6, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(6)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x64318: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x6431c: bge 0x64360 <entry+0xeb60>  @ imm = #0x3c
    c.emit(arm32::branch(C::Ge,c.at as u32,0x64360,false)?)?;
    // 0x64320: cmp r5, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x64324: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x64328: movlo r3, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x6432c: strb r3, [r1, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,11)?)?;
    // 0x64330: ldrb r3, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // 0x64334: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x64338: strb r3, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x6433c: ldr r3, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2732)?)?;
    // 0x64340: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x64344: beq 0x64358 <entry+0xeb58>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64358,false)?)?;
    // 0x64348: ldr r3, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,644)?)?;
    // 0x6434c: cmp r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8192)?)?;
    // 0x64350: addne r5, r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x64354: b 0x6435c <entry+0xeb5c>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x6435c,false)?)?;
    // 0x64358: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x6435c: ldr r3, [r6, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(6)?,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x64360: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x64364: blt 0x64398 <entry+0xeb98>  @ imm = #0x2c
    c.emit(arm32::branch(C::Lt,c.at as u32,0x64398,false)?)?;
    // 0x64368: strb r9, [r1, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,11)?)?;
    // 0x6436c: ldrb r1, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,1)?)?;
    // 0x64370: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x64374: strb r1, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1)?)?;
    // 0x64378: ldr r1, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2732)?)?;
    // 0x6437c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64380: beq 0x64394 <entry+0xeb94>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64394,false)?)?;
    // 0x64384: ldr r1, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,644)?)?;
    // 0x64388: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x6438c: addne r5, r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x64390: b 0x64398 <entry+0xeb98>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x64398,false)?)?;
    // 0x64394: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x64398: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x6439c: cmp r7, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x643a0: blo 0x64304 <entry+0xeb04>  @ imm = #-0xa4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x64304,false)?)?;
    // 0x643a4: cmp r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,1)?)?;
    // 0x643a8: beq 0x643f0 <entry+0xebf0>  @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x643f0,false)?)?;
    // 0x643ac: ldrb r0, [r4, #0x33c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,828)?)?;
    // 0x643b0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x643b4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x643b8: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x643bc: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x643c0: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x643c4: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x643c8: ldr r4, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x643cc: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x643d0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x643d4: bl 0x471c <entry+0xfffffffffffaef1c> @ imm = #-0x5fcc0
    c.emit(arm32::branch(C::Al,c.at as u32,0x471c,true)?)?;
    // 0x643d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x643dc: strb r0, [r4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x643e0: strb r8, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,1)?)?;
    // 0x643e4: strb r0, [r4, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,11)?)?;
    // 0x643e8: strb r8, [r4, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,12)?)?;
    // 0x643ec: strb r0, [r4, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3)?)?;
    // 0x643f0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x643f4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x643f8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_643F8:[u32;20]=[0x000f198c,0x000f1a84,0x000f1aa4,0x000f1ac4,0x000f1ae4,0x000f1b04,0x000f1bc4,0x000f1a0c,0x000f1bd4,0x000f1be4,0x000f1bf4,0x000f1c04,0x000f1c14,0x000f1b24,0x000f1c24,0x000f1b44,0x000f1b64,0x000f1b84,0x000f1ba4,0x000f1748];
 for value in INLINE_DATA_643F8{c.emit(value)?;}
 // Executable A32 block 0x64448..0x65000.
 c.check(0x64448)?;
    // 0x64448: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x6444c: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x64450: ldrb r12, [r0, #0xd0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::R0,208)?)?;
    // 0x64454: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x64458: beq 0x6446c <entry+0xec6c>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6446c,false)?)?;
    // 0x6445c: ldrb r12, [r2, #0xa08]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::R2,2568)?)?;
    // 0x64460: ldr lr, [r2, #0xa2c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R2,2604)?)?;
    // 0x64464: cmp lr, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::R12,Shift::Lsl,0,false)?)?;
    // 0x64468: bhs 0x64478 <entry+0xec78>  @ imm = #0x8
    c.emit(arm32::branch(C::Cs,c.at as u32,0x64478,false)?)?;
    // 0x6446c: ldrb r12, [r2, #0xa07]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::R2,2567)?)?;
    // 0x64470: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x64474: beq 0x644bc <entry+0xecbc>  @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x644bc,false)?)?;
    // 0x64478: ldr r12, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,8)?)?;
    // 0x6447c: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x64480: beq 0x6449c <entry+0xec9c>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6449c,false)?)?;
    // 0x64484: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x64488: strb r12, [r3, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(3)?,4)?)?;
    // 0x6448c: strb r12, [r1, #0x4b0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R1,1200)?)?;
    // 0x64490: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x64494: strb r1, [r2, #0xa07]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,2567)?)?;
    // 0x64498: b 0x644a8 <entry+0xeca8>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x644a8,false)?)?;
    // 0x6449c: ldrb r1, [r2, #0xa07]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,2567)?)?;
    // 0x644a0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x644a4: beq 0x644cc <entry+0xeccc>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x644cc,false)?)?;
    // 0x644a8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x644ac: str r1, [r2, #0xa2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,2604)?)?;
    // 0x644b0: strb r1, [r0, #0xd0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,208)?)?;
    // 0x644b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x644b8: b 0x644c4 <entry+0xecc4>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x644c4,false)?)?;
    // 0x644bc: ldr r0, [r2, #0xa2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,2604)?)?;
    // 0x644c0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x644c4: str r0, [r2, #0xa2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,2604)?)?;
    // 0x644c8: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x644cc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x644d0: b 0x64494 <entry+0xec94>  @ imm = #-0x44
    c.emit(arm32::branch(C::Al,c.at as u32,0x64494,false)?)?;
    // 0x644d4: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x644d8: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x644dc: sub sp, sp, #188
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,188)?)?;
    // 0x644e0: ldr r4, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,44)?)?;
    // 0x644e4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x644e8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x644ec: add r3, r0, #92
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,92)?)?;
    // 0x644f0: add r7, r0, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,640)?)?;
    // 0x644f4: ldr r10, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(4)?,36)?)?;
    // 0x644f8: strb r1, [r4, #0x172]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,370)?)?;
    // 0x644fc: strb r1, [r4, #0x3e8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1000)?)?;
    // 0x64500: str r2, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1572)?)?;
    // 0x64504: strb r2, [r4, #0x63c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1596)?)?;
    // 0x64508: ldrb r1, [r10, #0x1ad]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,429)?)?;
    // 0x6450c: strb r1, [r4, #0x63d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1597)?)?;
    // 0x64510: str r2, [r10, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,1732)?)?;
    // 0x64514: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x64518: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x6451c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x64520: ldrbne r1, [r0, #0xc20]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::R0,3104)?)?;
    // 0x64524: tstne r1, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R1,2)?)?;
    // 0x64528: bne 0x64534 <entry+0xed34>  @ imm = #0x4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64534,false)?)?;
    // 0x6452c: strb r2, [r10, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1744)?)?;
    // 0x64530: b 0x64590 <entry+0xed90>  @ imm = #0x58
    c.emit(arm32::branch(C::Al,c.at as u32,0x64590,false)?)?;
    // 0x64534: ldr r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,476)?)?;
    // 0x64538: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6453c: beq 0x64590 <entry+0xed90>  @ imm = #0x4c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64590,false)?)?;
    // 0x64540: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x64544: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x64548: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x6454c: bl 0x65ae8 <entry+0x102e8> @ imm = #0x1594
    c.emit(arm32::branch(C::Al,c.at as u32,0x65ae8,true)?)?;
    // 0x64550: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64554: beq 0x64810 <entry+0xf010>  @ imm = #0x2b4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64810,false)?)?;
    // 0x64558: ldr r5, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,24)?)?;
    // 0x6455c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x64560: ldr r1, [r5, #0x304]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,772)?)?;
    // 0x64564: bl 0x65d6c <entry+0x1056c> @ imm = #0x1800
    c.emit(arm32::branch(C::Al,c.at as u32,0x65d6c,true)?)?;
    // 0x64568: ldr r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,92)?)?;
    // 0x6456c: ldr r1, [pc, #0xf7c]        @ 0xfcf0 <entry+0xfcf0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3964)?)?;
    // 0x64570: strb r0, [r5, #0x996]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,2454)?)?;
    // 0x64574: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x64578: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x6457c: ldrb r3, [r10, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,1744)?)?;
    // 0x64580: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x358f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x64584: ldr r1, [pc, #0xf68]        @ 0xfcf4 <entry+0xfcf4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3944)?)?;
    // 0x64588: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x6458c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x35900
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x64590: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x64594: ldr r0, [r0, #0x304]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,772)?)?;
    // 0x64598: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x6459c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x645a0: ldr r9, [r10, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,100)?)?;
    // 0x645a4: ldr r6, [r10, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,360)?)?;
    // 0x645a8: ldr r5, [r10, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,364)?)?;
    // 0x645ac: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x645b0: mvneq r9, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mvn,R::new(9)?,R::R0,0)?)?;
    // 0x645b4: bl 0x5886c <entry+0x306c>  @ imm = #-0xbd50
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x645b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x645bc: str r7, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,20)?)?;
    // 0x645c0: bne 0x645d0 <entry+0xedd0>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x645d0,false)?)?;
    // 0x645c4: ldrb r0, [r7, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,281)?)?;
    // 0x645c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x645cc: beq 0x64808 <entry+0xf008>  @ imm = #0x234
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64808,false)?)?;
    // 0x645d0: ldr r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,92)?)?;
    // 0x645d4: ldr r1, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,332)?)?;
    // 0x645d8: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x645dc: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x645e0: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x645e4: movne r8, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x645e8: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // 0x645ec: sub r5, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x645f0: ldrb r0, [r2, #0x40c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R2,1036)?)?;
    // 0x645f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x645f8: movne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x645fc: strb r0, [r4, #0x178]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,376)?)?;
    // 0x64600: ldrb r1, [r2, #0x40b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,1035)?)?;
    // 0x64604: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64608: orrne r0, r0, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,8)?)?;
    // 0x6460c: strbne r0, [r4, #0x178]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,376)?)?;
    // 0x64610: ldr r0, [r2, #0x424]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,1060)?)?;
    // 0x64614: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64618: beq 0x64644 <entry+0xee44>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64644,false)?)?;
    // 0x6461c: ldr r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,476)?)?;
    // 0x64620: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64624: beq 0x64668 <entry+0xee68>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64668,false)?)?;
    // 0x64628: ldr r7, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,20)?)?;
    // 0x6462c: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // 0x64630: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x64634: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x64638: bl 0x38ec4 <entry+0xfffffffffffe36c4> @ imm = #-0x2b77c
    c.emit(arm32::branch(C::Al,c.at as u32,0x38ec4,true)?)?;
    // 0x6463c: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x64640: b 0x64670 <entry+0xee70>  @ imm = #0x28
    c.emit(arm32::branch(C::Al,c.at as u32,0x64670,false)?)?;
    // 0x64644: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // 0x64648: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6464c: cmp r1, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x64650: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x64654: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64658: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x6465c: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x64660: and r6, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x64664: b 0x6466c <entry+0xee6c>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x6466c,false)?)?;
    // 0x64668: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x6466c: ldr r7, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,20)?)?;
    // 0x64670: add r0, r4, #368
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,368)?)?;
    // 0x64674: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x64678: ldrb r0, [r7, #0xa05]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,2565)?)?;
    // 0x6467c: ldr r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,24)?)?;
    // 0x64680: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64684: beq 0x646d0 <entry+0xeed0>  @ imm = #0x44
    c.emit(arm32::branch(C::Eq,c.at as u32,0x646d0,false)?)?;
    // 0x64688: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6468c: strb r0, [r4, #0x620]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1568)?)?;
    // 0x64690: ldrb r0, [r7, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,3104)?)?;
    // 0x64694: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64698: beq 0x646d0 <entry+0xeed0>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x646d0,false)?)?;
    // 0x6469c: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x646a0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x646a4: bl 0x39b5c <entry+0xfffffffffffe435c> @ imm = #-0x2ab50
    c.emit(arm32::branch(C::Al,c.at as u32,0x39b5c,true)?)?;
    // 0x646a8: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x646ac: beq 0x646d0 <entry+0xeed0>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x646d0,false)?)?;
    // 0x646b0: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x646b4: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x646b8: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // 0x646bc: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // 0x646c0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x646c4: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x646c8: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x646cc: bl 0x64448 <entry+0xec48>  @ imm = #-0x28c
    c.emit(arm32::branch(C::Al,c.at as u32,0x64448,true)?)?;
    // 0x646d0: ldr r1, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,416)?)?;
    // 0x646d4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x646d8: bne 0x64704 <entry+0xef04>  @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64704,false)?)?;
    // 0x646dc: cmn r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,2)?)?;
    // 0x646e0: beq 0x64704 <entry+0xef04>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64704,false)?)?;
    // 0x646e4: ldrb r0, [r10, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,3)?)?;
    // 0x646e8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x646ec: ldrbne r0, [r10, #0x4]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(10)?,4)?)?;
    // 0x646f0: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x646f4: beq 0x64704 <entry+0xef04>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64704,false)?)?;
    // 0x646f8: ldrb r0, [r10, #0xb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,11)?)?;
    // 0x646fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64700: beq 0x64844 <entry+0xf044>  @ imm = #0x13c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64844,false)?)?;
    // 0x64704: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x64708: str r8, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,400)?)?;
    // 0x6470c: ldr r0, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,644)?)?;
    // 0x64710: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x64714: bne 0x6472c <entry+0xef2c>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6472c,false)?)?;
    // 0x64718: ldr r0, [r7, #0xc50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,3152)?)?;
    // 0x6471c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64720: ldrbne r0, [r4, #0x178]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(4)?,376)?)?;
    // 0x64724: orrne r0, r0, #4
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,4)?)?;
    // 0x64728: strbne r0, [r4, #0x178]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,376)?)?;
    // 0x6472c: ldr r0, [r7, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,108)?)?;
    // 0x64730: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64734: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64738: beq 0x64c30 <entry+0xf430>  @ imm = #0x4f0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64c30,false)?)?;
    // 0x6473c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64740: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x64744: strb r0, [r10, #0x7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,7)?)?;
    // 0x64748: str r2, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,476)?)?;
    // 0x6474c: strb r0, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,378)?)?;
    // 0x64750: strb r0, [r4, #0x17b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,379)?)?;
    // 0x64754: ldr r3, [r10, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,136)?)?;
    // 0x64758: cmp r3, r9
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x6475c: beq 0x6477c <entry+0xef7c>  @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6477c,false)?)?;
    // 0x64760: cmn r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,2)?)?;
    // 0x64764: ldrbne r6, [r10, #0x3]
    c.emit(arm32::transfer_byte(C::Ne,true,R::new(6)?,R::new(10)?,3)?)?;
    // 0x64768: cmpne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x6476c: beq 0x6477c <entry+0xef7c>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6477c,false)?)?;
    // 0x64770: ldrb r6, [r10, #0xb]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(10)?,11)?)?;
    // 0x64774: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x64778: beq 0x64818 <entry+0xf018>  @ imm = #0x98
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64818,false)?)?;
    // 0x6477c: str r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,988)?)?;
    // 0x64780: strb r0, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,996)?)?;
    // 0x64784: strb r0, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,378)?)?;
    // 0x64788: strb r0, [r4, #0x17b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,379)?)?;
    // 0x6478c: str r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,356)?)?;
    // 0x64790: mov r0, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,19)?)?;
    // 0x64794: str r2, [r10, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,136)?)?;
    // 0x64798: strb r2, [r4, #0x171]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,369)?)?;
    // 0x6479c: str r0, [r4, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1008)?)?;
    // 0x647a0: ldrb r2, [r4, #0x350]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,848)?)?;
    // 0x647a4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x647a8: ldrbne r0, [r4, #0x178]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(4)?,376)?)?;
    // 0x647ac: orrne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,1)?)?;
    // 0x647b0: strbne r0, [r4, #0x178]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,376)?)?;
    // 0x647b4: ldr r0, [r7, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,280)?)?;
    // 0x647b8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x647bc: bne 0x647e0 <entry+0xefe0>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x647e0,false)?)?;
    // 0x647c0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x647c4: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x647c8: cmn r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,2)?)?;
    // 0x647cc: bne 0x64c30 <entry+0xf430>  @ imm = #0x45c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64c30,false)?)?;
    // 0x647d0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x647d4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x647d8: streq r0, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,1572)?)?;
    // 0x647dc: b 0x64c30 <entry+0xf430>  @ imm = #0x44c
    c.emit(arm32::branch(C::Al,c.at as u32,0x64c30,false)?)?;
    // 0x647e0: cmn r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,2)?)?;
    // 0x647e4: ldrbne r0, [r10, #0x3]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(10)?,3)?)?;
    // 0x647e8: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x647ec: bne 0x647fc <entry+0xeffc>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x647fc,false)?)?;
    // 0x647f0: ldrb r0, [r4, #0x178]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,376)?)?;
    // 0x647f4: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // 0x647f8: strb r0, [r4, #0x178]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,376)?)?;
    // 0x647fc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64800: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x64804: b 0x64c30 <entry+0xf430>  @ imm = #0x424
    c.emit(arm32::branch(C::Al,c.at as u32,0x64c30,false)?)?;
    // 0x64808: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x6480c: b 0x645e8 <entry+0xede8>  @ imm = #-0x22c
    c.emit(arm32::branch(C::Al,c.at as u32,0x645e8,false)?)?;
    // 0x64810: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x64814: b 0x65944 <entry+0x10144> @ imm = #0x1128
    c.emit(arm32::branch(C::Al,c.at as u32,0x65944,false)?)?;
    // 0x64818: add r0, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,1)?)?;
    // 0x6481c: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x64820: str r0, [r10, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,136)?)?;
    // 0x64824: ldr r0, [r10, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,360)?)?;
    // 0x64828: str r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,988)?)?;
    // 0x6482c: ldr r0, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,332)?)?;
    // 0x64830: strb r0, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,996)?)?;
    // 0x64834: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64838: strb r8, [r4, #0x171]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,369)?)?;
    // 0x6483c: str r0, [r4, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1008)?)?;
    // 0x64840: b 0x64c30 <entry+0xf430>  @ imm = #0x3e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x64c30,false)?)?;
    // 0x64844: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64848: strb r0, [r4, #0x171]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,369)?)?;
    // 0x6484c: ldr r0, [r10, #0x638]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1592)?)?;
    // 0x64850: ldr r7, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,20)?)?;
    // 0x64854: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x64858: ldrbeq r0, [r7, #0x119]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(7)?,281)?)?;
    // 0x6485c: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64860: beq 0x64994 <entry+0xf194>  @ imm = #0x12c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64994,false)?)?;
    // 0x64864: ldrb r0, [r7, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,1814)?)?;
    // 0x64868: ldr r9, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,92)?)?;
    // 0x6486c: cmp r9, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x64870: movhi r9, r0
    c.emit(arm32::mov_register(C::Hi,R::new(9)?,R::R0))?;
    // 0x64874: strhi r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Hi,false,R::R0,R::new(10)?,92)?)?;
    // 0x64878: cmp r9, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,2)?)?;
    // 0x6487c: blo 0x64940 <entry+0xf140>  @ imm = #0xbc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x64940,false)?)?;
    // 0x64880: ldrb r0, [r10, #0x1ad]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,429)?)?;
    // 0x64884: ldr r1, [r10, #0x6cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1740)?)?;
    // 0x64888: ldr r6, [r10, #0x6c8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,1736)?)?;
    // 0x6488c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64890: addeq r1, r10, #44
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::new(10)?,44)?)?;
    // 0x64894: addeq r6, r10, #12
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(6)?,R::new(10)?,12)?)?;
    // 0x64898: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x6489c: ldr r0, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,332)?)?;
    // 0x648a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x648a4: bne 0x648f4 <entry+0xf0f4>  @ imm = #0x48
    c.emit(arm32::branch(C::Ne,c.at as u32,0x648f4,false)?)?;
    // 0x648a8: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x648ac: bl 0x5886c <entry+0x306c>  @ imm = #-0xc048
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x648b0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x648b4: bne 0x648f4 <entry+0xf0f4>  @ imm = #0x38
    c.emit(arm32::branch(C::Ne,c.at as u32,0x648f4,false)?)?;
    // 0x648b8: ldrb r0, [r7, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,281)?)?;
    // 0x648bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x648c0: bne 0x648f4 <entry+0xf0f4>  @ imm = #0x2c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x648f4,false)?)?;
    // 0x648c4: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x648c8: and r1, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(9)?,255)?)?;
    // 0x648cc: sub r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x648d0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x648d4: bge 0x648f4 <entry+0xf0f4>  @ imm = #0x18
    c.emit(arm32::branch(C::Ge,c.at as u32,0x648f4,false)?)?;
    // 0x648d8: ldrb r1, [r10, #0x7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,7)?)?;
    // 0x648dc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x648e0: bne 0x648f4 <entry+0xf0f4>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x648f4,false)?)?;
    // 0x648e4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x648e8: strb r0, [r10, #0x8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,8)?)?;
    // 0x648ec: strb r1, [r10, #0x7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,7)?)?;
    // 0x648f0: strb r0, [r10, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,9)?)?;
    // 0x648f4: ldr r0, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,332)?)?;
    // 0x648f8: ldr r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,24)?)?;
    // 0x648fc: ldrb r8, [r6, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(8)?,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x64900: ldr r1, [r7, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,108)?)?;
    // 0x64904: cmp r1, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x64908: bls 0x64a3c <entry+0xf23c>  @ imm = #0x12c
    c.emit(arm32::branch(C::Ls,c.at as u32,0x64a3c,false)?)?;
    // 0x6490c: ldrb r0, [r10, #0x7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,7)?)?;
    // 0x64910: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64914: bne 0x649c0 <entry+0xf1c0>  @ imm = #0xa4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x649c0,false)?)?;
    // 0x64918: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x6491c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x64920: ldr r0, [r0, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x64924: ldr r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,52)?)?;
    // 0x64928: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x6492c: ldrne r0, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::R0,40)?)?;
    // 0x64930: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64934: beq 0x649c0 <entry+0xf1c0>  @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x649c0,false)?)?;
    // 0x64938: ldr r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,4)?)?;
    // 0x6493c: b 0x64b20 <entry+0xf320>  @ imm = #0x1dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x64b20,false)?)?;
    // 0x64940: ldr r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,356)?)?;
    // 0x64944: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x64948: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x6494c: str r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,988)?)?;
    // 0x64950: str r1, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,476)?)?;
    // 0x64954: ldr r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,24)?)?;
    // 0x64958: ldr r1, [r7, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,108)?)?;
    // 0x6495c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64960: beq 0x649fc <entry+0xf1fc>  @ imm = #0x94
    c.emit(arm32::branch(C::Eq,c.at as u32,0x649fc,false)?)?;
    // 0x64964: ldr r1, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,84)?)?;
    // 0x64968: cmp r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,17)?)?;
    // 0x6496c: bhi 0x64a04 <entry+0xf204>  @ imm = #0x90
    c.emit(arm32::branch(C::Hi,c.at as u32,0x64a04,false)?)?;
    // 0x64970: mov r3, #10752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10752)?)?;
    // 0x64974: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x64978: orr r3, r3, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,131072)?)?;
    // 0x6497c: tst r3, r2, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::R2,Shift::Lsl,R::R1,false)?)?;
    // 0x64980: beq 0x64a04 <entry+0xf204>  @ imm = #0x7c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64a04,false)?)?;
    // 0x64984: str r2, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,400)?)?;
    // 0x64988: ldr r0, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,332)?)?;
    // 0x6498c: strb r0, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,996)?)?;
    // 0x64990: b 0x64c24 <entry+0xf424>  @ imm = #0x28c
    c.emit(arm32::branch(C::Al,c.at as u32,0x64c24,false)?)?;
    // 0x64994: ldrb r0, [r4, #0x175]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,373)?)?;
    // 0x64998: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6499c: bne 0x64864 <entry+0xf064>  @ imm = #-0x140
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64864,false)?)?;
    // 0x649a0: ldrb r0, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,10)?)?;
    // 0x649a4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x649a8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x649ac: lsl r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x649b0: str r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,92)?)?;
    // 0x649b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x649b8: str r0, [r10, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1592)?)?;
    // 0x649bc: b 0x64864 <entry+0xf064>  @ imm = #-0x160
    c.emit(arm32::branch(C::Al,c.at as u32,0x64864,false)?)?;
    // 0x649c0: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x649c4: bl 0x5886c <entry+0x306c>  @ imm = #-0xc160
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x649c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x649cc: bne 0x649e0 <entry+0xf1e0>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x649e0,false)?)?;
    // 0x649d0: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x649d4: ldrb r0, [r0, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,281)?)?;
    // 0x649d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x649dc: beq 0x64a38 <entry+0xf238>  @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64a38,false)?)?;
    // 0x649e0: ldrb r0, [r10, #0x7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,7)?)?;
    // 0x649e4: ldr r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,4)?)?;
    // 0x649e8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x649ec: bne 0x64b20 <entry+0xf320>  @ imm = #0x12c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64b20,false)?)?;
    // 0x649f0: ldrb r0, [r10, #0x9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,9)?)?;
    // 0x649f4: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x649f8: b 0x64b1c <entry+0xf31c>  @ imm = #0x11c
    c.emit(arm32::branch(C::Al,c.at as u32,0x64b1c,false)?)?;
    // 0x649fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64a00: b 0x64c28 <entry+0xf428>  @ imm = #0x220
    c.emit(arm32::branch(C::Al,c.at as u32,0x64c28,false)?)?;
    // 0x64a04: cmp r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,12)?)?;
    // 0x64a08: bne 0x64a14 <entry+0xf214>  @ imm = #0x4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64a14,false)?)?;
    // 0x64a0c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64a10: b 0x64a30 <entry+0xf230>  @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x64a30,false)?)?;
    // 0x64a14: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x64a18: str r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,400)?)?;
    // 0x64a1c: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // 0x64a20: bl 0x81850 <entry+0x2c050> @ imm = #0x1ce28
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x64a24: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x64a28: bne 0x64988 <entry+0xf188>  @ imm = #-0xa8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64988,false)?)?;
    // 0x64a2c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64a30: str r0, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,400)?)?;
    // 0x64a34: b 0x64988 <entry+0xf188>  @ imm = #-0xb4
    c.emit(arm32::branch(C::Al,c.at as u32,0x64988,false)?)?;
    // 0x64a38: ldr r1, [r7, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,108)?)?;
    // 0x64a3c: ldrb r0, [r10, #0x7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,7)?)?;
    // 0x64a40: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64a44: beq 0x64a54 <entry+0xf254>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64a54,false)?)?;
    // 0x64a48: ldrb r2, [r10, #0x9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,9)?)?;
    // 0x64a4c: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x64a50: bhs 0x64adc <entry+0xf2dc>  @ imm = #0x84
    c.emit(arm32::branch(C::Cs,c.at as u32,0x64adc,false)?)?;
    // 0x64a54: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64a58: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64a5c: ldrne r2, [r7, #0x74]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(7)?,116)?)?;
    // 0x64a60: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x64a64: beq 0x64c28 <entry+0xf428>  @ imm = #0x1bc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64c28,false)?)?;
    // 0x64a68: ldr r2, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,332)?)?;
    // 0x64a6c: and r12, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R12,R::new(9)?,255)?)?;
    // 0x64a70: b 0x64ad0 <entry+0xf2d0>  @ imm = #0x58
    c.emit(arm32::branch(C::Al,c.at as u32,0x64ad0,false)?)?;
    // 0x64a74: ldrb r8, [r6, r2]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(8)?,R::new(6)?,R::R2,Shift::Lsl,0)?)?;
    // 0x64a78: cmp r1, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x64a7c: bls 0x64aa0 <entry+0xf2a0>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ls,c.at as u32,0x64aa0,false)?)?;
    // 0x64a80: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x64a84: ldr r5, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,0)?)?;
    // 0x64a88: ldr r5, [r5, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(5)?,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x64a8c: ldr r3, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,52)?)?;
    // 0x64a90: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x64a94: ldrne r3, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(5)?,40)?)?;
    // 0x64a98: cmpne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x64a9c: bne 0x65ac8 <entry+0x102c8> @ imm = #0x1024
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65ac8,false)?)?;
    // 0x64aa0: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x64aa4: bne 0x64ac8 <entry+0xf2c8>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64ac8,false)?)?;
    // 0x64aa8: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x64aac: ldr r3, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // 0x64ab0: ldr r5, [r3, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(3)?,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x64ab4: ldr r3, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,52)?)?;
    // 0x64ab8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x64abc: ldrne r3, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(5)?,40)?)?;
    // 0x64ac0: cmpne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x64ac4: beq 0x64938 <entry+0xf138>  @ imm = #-0x194
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64938,false)?)?;
    // 0x64ac8: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x64acc: str r2, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,332)?)?;
    // 0x64ad0: cmp r2, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // 0x64ad4: blo 0x64a74 <entry+0xf274>  @ imm = #-0x68
    c.emit(arm32::branch(C::Cc,c.at as u32,0x64a74,false)?)?;
    // 0x64ad8: b 0x64c28 <entry+0xf428>  @ imm = #0x148
    c.emit(arm32::branch(C::Al,c.at as u32,0x64c28,false)?)?;
    // 0x64adc: ldr r1, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,332)?)?;
    // 0x64ae0: and r3, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(9)?,255)?)?;
    // 0x64ae4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64ae8: b 0x64b00 <entry+0xf300>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x64b00,false)?)?;
    // 0x64aec: ldrb r8, [r6, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(8)?,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x64af0: cmp r2, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x64af4: bhi 0x64b0c <entry+0xf30c>  @ imm = #0x10
    c.emit(arm32::branch(C::Hi,c.at as u32,0x64b0c,false)?)?;
    // 0x64af8: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x64afc: str r1, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,332)?)?;
    // 0x64b00: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x64b04: blo 0x64aec <entry+0xf2ec>  @ imm = #-0x20
    c.emit(arm32::branch(C::Cc,c.at as u32,0x64aec,false)?)?;
    // 0x64b08: b 0x64c28 <entry+0xf428>  @ imm = #0x118
    c.emit(arm32::branch(C::Al,c.at as u32,0x64c28,false)?)?;
    // 0x64b0c: ldr r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,4)?)?;
    // 0x64b10: ldrb r0, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x64b14: str r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,988)?)?;
    // 0x64b18: sub r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,1)?)?;
    // 0x64b1c: strb r0, [r10, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,9)?)?;
    // 0x64b20: ldr r0, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,332)?)?;
    // 0x64b24: ldr r1, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,356)?)?;
    // 0x64b28: ldrb r2, [r3, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R2,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // 0x64b2c: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x64b30: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x64b34: str r1, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,988)?)?;
    // 0x64b38: strb r0, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,996)?)?;
    // 0x64b3c: ldr r1, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,84)?)?;
    // 0x64b40: cmp r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,11)?)?;
    // 0x64b44: bne 0x64b60 <entry+0xf360>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64b60,false)?)?;
    // 0x64b48: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x64b4c: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x64b50: str r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,400)?)?;
    // 0x64b54: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x64b58: strbeq r1, [r4, #0x172]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(4)?,370)?)?;
    // 0x64b5c: b 0x64b88 <entry+0xf388>  @ imm = #0x24
    c.emit(arm32::branch(C::Al,c.at as u32,0x64b88,false)?)?;
    // 0x64b60: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x64b64: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x64b68: and r2, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(9)?,255)?)?;
    // 0x64b6c: str r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,400)?)?;
    // 0x64b70: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x64b74: strbeq r1, [r4, #0x172]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(4)?,370)?)?;
    // 0x64b78: ldrb r1, [r3, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // 0x64b7c: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x64b80: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x64b84: streq r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,400)?)?;
    // 0x64b88: ldrb r1, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,378)?)?;
    // 0x64b8c: strb r1, [r4, #0x17b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,379)?)?;
    // 0x64b90: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x64b94: ldrb r0, [r3, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // 0x64b98: ldrb r1, [r1, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,281)?)?;
    // 0x64b9c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64ba0: beq 0x64bec <entry+0xf3ec>  @ imm = #0x44
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64bec,false)?)?;
    // 0x64ba4: mov r1, #920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,920)?)?;
    // 0x64ba8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x64bac: ldrsb r3, [r7, r1]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::new(3)?,R::new(7)?,R::R1)?)?;
    // 0x64bb0: sub r1, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(3)?,1)?)?;
    // 0x64bb4: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x64bb8: blt 0x64bd8 <entry+0xf3d8>  @ imm = #0x18
    c.emit(arm32::branch(C::Lt,c.at as u32,0x64bd8,false)?)?;
    // 0x64bbc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x64bc0: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x64bc4: tst r0, r3, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x64bc8: bne 0x64bd8 <entry+0xf3d8>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64bd8,false)?)?;
    // 0x64bcc: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x64bd0: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x64bd4: blt 0x64bc4 <entry+0xf3c4>  @ imm = #-0x18
    c.emit(arm32::branch(C::Lt,c.at as u32,0x64bc4,false)?)?;
    // 0x64bd8: sub r0, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x64bdc: strb r0, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,378)?)?;
    // 0x64be0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64be4: str r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,476)?)?;
    // 0x64be8: b 0x64c28 <entry+0xf428>  @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x64c28,false)?)?;
    // 0x64bec: ldrb r3, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,10)?)?;
    // 0x64bf0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x64bf4: sub r1, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(3)?,1)?)?;
    // 0x64bf8: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x64bfc: blo 0x64c1c <entry+0xf41c>  @ imm = #0x18
    c.emit(arm32::branch(C::Cc,c.at as u32,0x64c1c,false)?)?;
    // 0x64c00: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x64c04: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x64c08: tst r0, r3, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x64c0c: bne 0x64c1c <entry+0xf41c>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64c1c,false)?)?;
    // 0x64c10: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x64c14: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x64c18: blt 0x64c08 <entry+0xf408>  @ imm = #-0x18
    c.emit(arm32::branch(C::Lt,c.at as u32,0x64c08,false)?)?;
    // 0x64c1c: sub r0, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x64c20: strb r0, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,378)?)?;
    // 0x64c24: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64c28: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x64c2c: str r1, [r4, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1008)?)?;
    // 0x64c30: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x64c34: ldrb r1, [r1, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,345)?)?;
    // 0x64c38: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64c3c: beq 0x64c60 <entry+0xf460>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64c60,false)?)?;
    // 0x64c40: ldr r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,400)?)?;
    // 0x64c44: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x64c48: bne 0x64c60 <entry+0xf460>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64c60,false)?)?;
    // 0x64c4c: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x64c50: ldrb r1, [r1, #0x15a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,346)?)?;
    // 0x64c54: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x64c58: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x64c5c: str r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,400)?)?;
    // 0x64c60: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x64c64: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64c68: beq 0x65944 <entry+0x10144> @ imm = #0xcd4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65944,false)?)?;
    // 0x64c6c: ldr r1, [r7, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,1392)?)?;
    // 0x64c70: ldrb r0, [r7, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,3104)?)?;
    // 0x64c74: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x64c78: bne 0x64cc0 <entry+0xf4c0>  @ imm = #0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64cc0,false)?)?;
    // 0x64c7c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64c80: beq 0x64cdc <entry+0xf4dc>  @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64cdc,false)?)?;
    // 0x64c84: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x64c88: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x64c8c: bl 0x39b5c <entry+0xfffffffffffe435c> @ imm = #-0x2b138
    c.emit(arm32::branch(C::Al,c.at as u32,0x39b5c,true)?)?;
    // 0x64c90: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x64c94: bne 0x64cdc <entry+0xf4dc>  @ imm = #0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64cdc,false)?)?;
    // 0x64c98: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x64c9c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x64ca0: ldr r0, [r0, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x64ca4: ldr r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,40)?)?;
    // 0x64ca8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64cac: beq 0x64cdc <entry+0xf4dc>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64cdc,false)?)?;
    // 0x64cb0: ldr r0, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,52)?)?;
    // 0x64cb4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64cb8: bne 0x65944 <entry+0x10144> @ imm = #0xc84
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65944,false)?)?;
    // 0x64cbc: b 0x64cdc <entry+0xf4dc>  @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x64cdc,false)?)?;
    // 0x64cc0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64cc4: beq 0x64cdc <entry+0xf4dc>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64cdc,false)?)?;
    // 0x64cc8: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x64ccc: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x64cd0: bl 0x39b5c <entry+0xfffffffffffe435c> @ imm = #-0x2b17c
    c.emit(arm32::branch(C::Al,c.at as u32,0x39b5c,true)?)?;
    // 0x64cd4: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x64cd8: beq 0x65944 <entry+0x10144> @ imm = #0xc64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65944,false)?)?;
    // 0x64cdc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64ce0: mov r9, r4
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(4)?))?;
    // 0x64ce4: strb r0, [r10, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,11)?)?;
    // 0x64ce8: ldrb r0, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,378)?)?;
    // 0x64cec: strb r0, [r4, #0x1d4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,468)?)?;
    // 0x64cf0: ldr r1, [r9, #0x664]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::new(9)?,1636)?)?;
    // 0x64cf4: tst r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,8192)?)?;
    // 0x64cf8: ldrne r2, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(4)?,1772)?)?;
    // 0x64cfc: addne r2, r2, r0
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R2,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x64d00: strbne r2, [r4, #0x1d4]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::new(4)?,468)?)?;
    // 0x64d04: tst r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,256)?)?;
    // 0x64d08: beq 0x64d70 <entry+0xf570>  @ imm = #0x60
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64d70,false)?)?;
    // 0x64d0c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64d10: bne 0x64d70 <entry+0xf570>  @ imm = #0x58
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64d70,false)?)?;
    // 0x64d14: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64d18: str r0, [r10, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1592)?)?;
    // 0x64d1c: ldrb r0, [r4, #0x6e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1764)?)?;
    // 0x64d20: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x64d24: strb r0, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,10)?)?;
    // 0x64d28: ldr r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1636)?)?;
    // 0x64d2c: bic r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,256)?)?;
    // 0x64d30: str r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1636)?)?;
    // 0x64d34: ldr r0, [r7, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,744)?)?;
    // 0x64d38: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64d3c: beq 0x64d70 <entry+0xf570>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64d70,false)?)?;
    // 0x64d40: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x64d44: mov r1, #564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,564)?)?;
    // 0x64d48: mov r2, #568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,568)?)?;
    // 0x64d4c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x64d50: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x64d54: ldr r1, [r0, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x64d58: orr r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32)?)?;
    // 0x64d5c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x64d60: ldrb r1, [r4, #0x6e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1764)?)?;
    // 0x64d64: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x64d68: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x64d6c: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x64d70: ldr r0, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,400)?)?;
    // 0x64d74: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x64d78: beq 0x64da8 <entry+0xf5a8>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64da8,false)?)?;
    // 0x64d7c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x64d80: bne 0x64df4 <entry+0xf5f4>  @ imm = #0x6c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64df4,false)?)?;
    // 0x64d84: ldrb r0, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,996)?)?;
    // 0x64d88: ldrb r1, [r10, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1744)?)?;
    // 0x64d8c: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x64d90: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x64d94: ldr r0, [r0, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2508)?)?;
    // 0x64d98: ldrb r0, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,3)?)?;
    // 0x64d9c: strb r0, [r4, #0x3ee]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1006)?)?;
    // 0x64da0: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x64da4: b 0x64e00 <entry+0xf600>  @ imm = #0x54
    c.emit(arm32::branch(C::Al,c.at as u32,0x64e00,false)?)?;
    // 0x64da8: ldrb r0, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,996)?)?;
    // 0x64dac: ldrb r1, [r10, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1744)?)?;
    // 0x64db0: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x64db4: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x64db8: ldr r0, [r0, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2508)?)?;
    // 0x64dbc: ldrb r6, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R0,3)?)?;
    // 0x64dc0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64dc4: strb r6, [r4, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1005)?)?;
    // 0x64dc8: strb r0, [r4, #0x3ee]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1006)?)?;
    // 0x64dcc: ldr r5, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,84)?)?;
    // 0x64dd0: cmp r5, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,17)?)?;
    // 0x64dd4: bhi 0x65110 <entry+0xf910>  @ imm = #0x334
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65110,false)?)?;
    // 0x64dd8: mov r2, #33024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33024)?)?;
    // 0x64ddc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x64de0: orr r2, r2, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,196608)?)?;
    // 0x64de4: tst r2, r1, lsl r5
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::new(5)?,false)?)?;
    // 0x64de8: beq 0x65110 <entry+0xf910>  @ imm = #0x320
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65110,false)?)?;
    // 0x64dec: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x64df0: b 0x64e00 <entry+0xf600>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x64e00,false)?)?;
    // 0x64df4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64df8: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x64dfc: strb r0, [r4, #0x3ee]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1006)?)?;
    // 0x64e00: strb r6, [r4, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1005)?)?;
    // 0x64e04: ldr r5, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,84)?)?;
    // 0x64e08: cmp r5, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,17)?)?;
    // 0x64e0c: cmpne r5, #10
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(5)?,10)?)?;
    // 0x64e10: bne 0x64e2c <entry+0xf62c>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64e2c,false)?)?;
    // 0x64e14: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x64e18: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x64e1c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64e20: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64e24: strb r6, [r4, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1005)?)?;
    // 0x64e28: strb r0, [r4, #0x3ee]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1006)?)?;
    // 0x64e2c: ldr r2, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,420)?)?;
    // 0x64e30: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x64e34: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x64e38: bl 0x39970 <entry+0xfffffffffffe4170> @ imm = #-0x2b4d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x39970,true)?)?;
    // 0x64e3c: ldr r2, [pc, #0xc98]        @ 0x102dc <entry+0x102dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3224)?)?;
    // 0x64e40: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x64e44: mov r1, #428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,428)?)?;
    // 0x64e48: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x64e4c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x64e50: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x636a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x64e54: ldr r0, [r10, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,404)?)?;
    // 0x64e58: str r0, [r4, #0x368]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,872)?)?;
    // 0x64e5c: ldr r0, [r10, #0x198]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,408)?)?;
    // 0x64e60: str r0, [r4, #0x36c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,876)?)?;
    // 0x64e64: ldrb r0, [r7, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,3104)?)?;
    // 0x64e68: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x64e6c: beq 0x64ecc <entry+0xf6cc>  @ imm = #0x58
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64ecc,false)?)?;
    // 0x64e70: ldr r0, [r10, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1732)?)?;
    // 0x64e74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64e78: bne 0x64ecc <entry+0xf6cc>  @ imm = #0x4c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64ecc,false)?)?;
    // 0x64e7c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x64e80: str r1, [r4, #0x368]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,872)?)?;
    // 0x64e84: ldr r0, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,420)?)?;
    // 0x64e88: ldr r0, [r0, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,192)?)?;
    // 0x64e8c: str r0, [r4, #0x36c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,876)?)?;
    // 0x64e90: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64e94: ldr r2, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,2744)?)?;
    // 0x64e98: ldr r2, [r2, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,36)?)?;
    // 0x64e9c: blt 0x64eb4 <entry+0xf6b4>  @ imm = #0x10
    c.emit(arm32::branch(C::Lt,c.at as u32,0x64eb4,false)?)?;
    // 0x64ea0: asr r1, r2, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Asr,31)?)?;
    // 0x64ea4: add r1, r2, r1, lsr #28
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsr,28,false)?)?;
    // 0x64ea8: cmp r0, r1, asr #4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Asr,4,false)?)?;
    // 0x64eac: asrge r0, r1, #4
    c.emit(arm32::mov_shifted_register(C::Ge,R::R0,R::R1,Shift::Asr,4)?)?;
    // 0x64eb0: b 0x64ec8 <entry+0xf6c8>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x64ec8,false)?)?;
    // 0x64eb4: asr r3, r2, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Asr,31)?)?;
    // 0x64eb8: add r2, r2, r3, lsr #28
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsr,28,false)?)?;
    // 0x64ebc: sub r1, r1, r2, asr #4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Asr,4,false)?)?;
    // 0x64ec0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x64ec4: movle r0, r1
    c.emit(arm32::mov_register(C::Le,R::R0,R::R1))?;
    // 0x64ec8: str r0, [r4, #0x36c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,876)?)?;
    // 0x64ecc: ldr r5, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,32)?)?;
    // 0x64ed0: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x64ed4: beq 0x64f1c <entry+0xf71c>  @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64f1c,false)?)?;
    // 0x64ed8: ldr r0, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,420)?)?;
    // 0x64edc: ldr r1, [r0, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,176)?)?;
    // 0x64ee0: str r1, [r5, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,1496)?)?;
    // 0x64ee4: ldrd r0, r1, [r0, #184]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R0,R::R0,184)?)?;
    // 0x64ee8: str r0, [r5, #0x5e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1504)?)?;
    // 0x64eec: str r1, [r5, #0x5e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,1508)?)?;
    // 0x64ef0: ldr r0, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,416)?)?;
    // 0x64ef4: ldr r1, [r7, #0xcb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,3252)?)?;
    // 0x64ef8: bl 0x81850 <entry+0x2c050> @ imm = #0x1c950
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x64efc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64f00: beq 0x64f1c <entry+0xf71c>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64f1c,false)?)?;
    // 0x64f04: ldr r1, [r7, #0xcbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,3260)?)?;
    // 0x64f08: add r0, r5, #1504
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1504)?)?;
    // 0x64f0c: str r1, [r5, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,1496)?)?;
    // 0x64f10: ldr r2, [r7, #0xcc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,3264)?)?;
    // 0x64f14: ldr r3, [r7, #0xcc4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,3268)?)?;
    // 0x64f18: strd r2, r3, [r0]
    c.emit(arm32::transfer_doubleword(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x64f1c: ldr r0, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,412)?)?;
    // 0x64f20: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64f24: bne 0x64f58 <entry+0xf758>  @ imm = #0x2c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64f58,false)?)?;
    // 0x64f28: add r0, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,28)?)?;
    // 0x64f2c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x64f30: mov r2, #160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,160)?)?;
    // 0x64f34: bl 0x471c <entry+0xfffffffffffaef1c> @ imm = #-0x60820
    c.emit(arm32::branch(C::Al,c.at as u32,0x471c,true)?)?;
    // 0x64f38: ldr r0, [r7, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,744)?)?;
    // 0x64f3c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64f40: ldrne r2, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(4)?,32)?)?;
    // 0x64f44: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x64f48: beq 0x64f58 <entry+0xf758>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64f58,false)?)?;
    // 0x64f4c: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x64f50: add r1, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,28)?)?;
    // 0x64f54: bl 0x679e8 <entry+0x121e8> @ imm = #0x2a8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x679e8,true)?)?;
    // 0x64f58: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x64f5c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x64f60: bl 0x6358c <entry+0xdd8c>  @ imm = #-0x19dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x6358c,true)?)?;
    // 0x64f64: ldr r0, [r7, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,108)?)?;
    // 0x64f68: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64f6c: bne 0x64fa8 <entry+0xf7a8>  @ imm = #0x34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x64fa8,false)?)?;
    // 0x64f70: ldr r0, [r7, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,116)?)?;
    // 0x64f74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x64f78: beq 0x64fa8 <entry+0xf7a8>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64fa8,false)?)?;
    // 0x64f7c: ldr r1, [pc, #0xb5c]        @ 0x102e0 <entry+0x102e0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2908)?)?;
    // 0x64f80: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64f84: mvn r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1)?)?;
    // 0x64f88: strb r0, [r4, #0x170]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,368)?)?;
    // 0x64f8c: mvn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,1)?)?;
    // 0x64f90: str r0, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,416)?)?;
    // 0x64f94: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64f98: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3630c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x64f9c: ldr r1, [pc, #0xb34]        @ 0x102d8 <entry+0x102d8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2868)?)?;
    // 0x64fa0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x64fa4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x36318
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x64fa8: ldr r0, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,420)?)?;
    // 0x64fac: ldr r1, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,168)?)?;
    // 0x64fb0: str r1, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,416)?)?;
    // 0x64fb4: ldr r1, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,412)?)?;
    // 0x64fb8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64fbc: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x64fc0: strbne r1, [r4, #0x176]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(4)?,374)?)?;
    // 0x64fc4: ldr r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,52)?)?;
    // 0x64fc8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64fcc: beq 0x650ec <entry+0xf8ec>  @ imm = #0x118
    c.emit(arm32::branch(C::Eq,c.at as u32,0x650ec,false)?)?;
    // 0x64fd0: ldr r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,40)?)?;
    // 0x64fd4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x64fd8: beq 0x650fc <entry+0xf8fc>  @ imm = #0x11c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x650fc,false)?)?;
    // 0x64fdc: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x64fe0: ldrb r2, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,996)?)?;
    // 0x64fe4: strb r2, [r4, #0x3e5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,997)?)?;
    // 0x64fe8: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x64fec: ldr r9, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,400)?)?;
    // 0x64ff0: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x64ff4: ldrbne r1, [r10, #0x6d0]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::new(10)?,1744)?)?;
    // 0x64ff8: addne r1, r1, r2
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x64ffc: strb r1, [r4, #0x63c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1596)?)?;
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
