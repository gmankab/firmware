// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x62000,0x63000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x62000;
pub const END:usize=0x63000;
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
fn emit_signed_halfword_register(cond:C,rd:R,base:R,index:R,writeback:bool)->Result<u32,String>{
 if [rd,base,index].contains(&R::PC)||(writeback&&rd==base){return Err("invalid A32 indexed signed halfword operands".into())}
 Ok(((cond as u32)<<28)|0x0190_00f0|(if writeback{1<<21}else{0})|(base.number()<<16)|(rd.number()<<12)|index.number())
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x62000..0x62264.
 c.check(0x62000)?;
    // 0x62000: strb r0, [r10, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,13)?)?;
    // 0x62004: strb r4, [r10, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(10)?,14)?)?;
    // 0x62008: strb r1, [r10, #0x2d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,45)?)?;
    // 0x6200c: strb r0, [r10, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,46)?)?;
    // 0x62010: str r0, [r10, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1732)?)?;
    // 0x62014: str r1, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,92)?)?;
    // 0x62018: ldr r0, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,84)?)?;
    // 0x6201c: cmp r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,11)?)?;
    // 0x62020: bne 0x6203c <entry+0xc83c>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6203c,false)?)?;
    // 0x62024: ldrb r0, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,10)?)?;
    // 0x62028: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6202c: subne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x62030: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x62034: lslne r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Ne,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x62038: strne r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(10)?,92)?)?;
    // 0x6203c: ldr r0, [r9, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,132)?)?;
    // 0x62040: mov r1, #408
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,408)?)?;
    // 0x62044: mov r2, #154
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,154)?)?;
    // 0x62048: mvn r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,0)?)?;
    // 0x6204c: orr r2, r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,256)?)?;
    // 0x62050: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62054: moveq r0, #12
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x62058: streq r0, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(10)?,84)?)?;
    // 0x6205c: ldrb r0, [r9, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,281)?)?;
    // 0x62060: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62064: ldrbne r0, [r9, #0x716]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,1814)?)?;
    // 0x62068: strne r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(10)?,92)?)?;
    // 0x6206c: ldr r0, [r8, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,48)?)?;
    // 0x62070: str r0, [r10, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,100)?)?;
    // 0x62074: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62078: str r0, [r10, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,136)?)?;
    // 0x6207c: strh r0, [r10, #4]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(10)?,4)?)?;
    // 0x62080: strh r0, [r10, #2]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(10)?,2)?)?;
    // 0x62084: str r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,108)?)?;
    // 0x62088: str r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,112)?)?;
    // 0x6208c: str r0, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,116)?)?;
    // 0x62090: str r0, [r10, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,120)?)?;
    // 0x62094: ldr r0, [r9, #0x388]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,904)?)?;
    // 0x62098: str r0, [r10, #0x24c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,588)?)?;
    // 0x6209c: ldr r0, [r9, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2104)?)?;
    // 0x620a0: ldrsh r1, [r0, r1]
    c.emit(emit_signed_halfword_register(C::Al,R::R1,R::R0,R::R1,false)?)?;
    // 0x620a4: str r1, [r10, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,404)?)?;
    // 0x620a8: ldrsh r2, [r0, r2]
    c.emit(emit_signed_halfword_register(C::Al,R::R2,R::R0,R::R2,false)?)?;
    // 0x620ac: str r2, [r10, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,408)?)?;
    // 0x620b0: ldrb r7, [r9, #0x15d]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,349)?)?;
    // 0x620b4: ldr r2, [r9, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,556)?)?;
    // 0x620b8: cmp r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // 0x620bc: bne 0x620d0 <entry+0xc8d0>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x620d0,false)?)?;
    // 0x620c0: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x620c4: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x620c8: sub r2, r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,5)?)?;
    // 0x620cc: b 0x620dc <entry+0xc8dc>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x620dc,false)?)?;
    // 0x620d0: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x620d4: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x620d8: sub r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,4)?)?;
    // 0x620dc: rsble r3, r3, #0
    c.emit(arm32::data_immediate(C::Le,Op::Rsb,R::new(3)?,R::new(3)?,0)?)?;
    // 0x620e0: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x620e4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x620e8: add r6, r10, #656
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(10)?,656)?)?;
    // 0x620ec: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x620f0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x620f4: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x620f8: streq r1, [r10, #0x194]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(10)?,404)?)?;
    // 0x620fc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62100: rsbmi r1, r1, #0
    c.emit(arm32::data_immediate(C::Mi,Op::Rsb,R::R1,R::R1,0)?)?;
    // 0x62104: sub r3, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x62108: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x6210c: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x62110: movls r3, r1
    c.emit(arm32::mov_register(C::Ls,R::new(3)?,R::R1))?;
    // 0x62114: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x62118: movhs r1, r2
    c.emit(arm32::mov_register(C::Cs,R::R1,R::R2))?;
    // 0x6211c: str r3, [r10, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,372)?)?;
    // 0x62120: str r1, [r10, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,376)?)?;
    // 0x62124: ldr r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,24)?)?;
    // 0x62128: str r1, [r10, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,380)?)?;
    // 0x6212c: ldr r0, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,28)?)?;
    // 0x62130: str r0, [r10, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,392)?)?;
    // 0x62134: ldr r0, [r9, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,908)?)?;
    // 0x62138: str r0, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1296)?)?;
    // 0x6213c: str r7, [r10, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(10)?,1300)?)?;
    // 0x62140: ldrb r0, [r10, #0x518]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1304)?)?;
    // 0x62144: strb r0, [r10, #0x526]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1318)?)?;
    // 0x62148: strb r7, [r10, #0x519]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(10)?,1305)?)?;
    // 0x6214c: strb r7, [r10, #0x518]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(10)?,1304)?)?;
    // 0x62150: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x62154: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x62158: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x6215c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x5ef78
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x62160: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x62164: strb r5, [r6, #0x2a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(6)?,42)?)?;
    // 0x62168: strb r7, [r6, #0x20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(6)?,32)?)?;
    // 0x6216c: add r6, r6, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,72)?)?;
    // 0x62170: cmp r4, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,9)?)?;
    // 0x62174: bne 0x62150 <entry+0xc950>  @ imm = #-0x2c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62150,false)?)?;
    // 0x62178: add r0, r10, #572
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,572)?)?;
    // 0x6217c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x62180: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x62184: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x62188: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x6218c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x5efa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x62190: add r0, r10, #1328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,1328)?)?;
    // 0x62194: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x62198: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x6219c: str r6, [r0, #-0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,-8)?)?;
    // 0x621a0: str r6, [r0, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,-4)?)?;
    // 0x621a4: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x621a8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x621ac: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x621b0: bne 0x62198 <entry+0xc998>  @ imm = #-0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62198,false)?)?;
    // 0x621b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x621b8: str r0, [r10, #0x66c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1644)?)?;
    // 0x621bc: str r0, [r10, #0x668]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1640)?)?;
    // 0x621c0: ldr r1, [r9, #0x110]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,272)?)?;
    // 0x621c4: str r1, [r10, #0x680]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1664)?)?;
    // 0x621c8: ldr r1, [r9, #0x10c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,268)?)?;
    // 0x621cc: str r1, [r10, #0x684]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1668)?)?;
    // 0x621d0: ldr r1, [r9, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,276)?)?;
    // 0x621d4: str r1, [r10, #0x69c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1692)?)?;
    // 0x621d8: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x621dc: str r0, [r10, #0x67c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1660)?)?;
    // 0x621e0: str r0, [r10, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1656)?)?;
    // 0x621e4: str r0, [r10, #0x674]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1652)?)?;
    // 0x621e8: str r0, [r10, #0x670]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1648)?)?;
    // 0x621ec: str r1, [r10, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1672)?)?;
    // 0x621f0: str r0, [r10, #0x68c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1676)?)?;
    // 0x621f4: str r1, [r10, #0x690]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1680)?)?;
    // 0x621f8: str r0, [r10, #0x694]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1684)?)?;
    // 0x621fc: str r0, [r10, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1696)?)?;
    // 0x62200: str r1, [r10, #0x6a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1700)?)?;
    // 0x62204: str r0, [r10, #0x6a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1704)?)?;
    // 0x62208: str r1, [r10, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1708)?)?;
    // 0x6220c: str r0, [r10, #0x6b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1712)?)?;
    // 0x62210: str r0, [r10, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1716)?)?;
    // 0x62214: str r0, [r10, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1592)?)?;
    // 0x62218: str r0, [r10, #0x6c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1728)?)?;
    // 0x6221c: str r1, [r10, #0x278]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,632)?)?;
    // 0x62220: str r1, [r10, #0x27c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,636)?)?;
    // 0x62224: str r1, [r10, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,640)?)?;
    // 0x62228: str r1, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,644)?)?;
    // 0x6222c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x62230: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x62234: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x62238: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x6223c: ldr r2, [pc, #0x20]         @ 0xca64 <entry+0xca64>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,32)?)?;
    // 0x62240: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x62244: mov r1, #222
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,222)?)?;
    // 0x62248: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x6224c: orr r1, r1, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    // 0x62250: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x62254: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x60aac
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x62258: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x6225c: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x62260: b 0x2f1c <entry+0xfffffffffffad71c> @ imm = #-0x5f34c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f1c,false)?)?;
 c.check(0x62264)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_62264:[u32;1]=[0x000f1748];
 for value in INLINE_DATA_62264{c.emit(value)?;}
 // Executable A32 block 0x62268..0x623c4.
 c.check(0x62268)?;
    // 0x62268: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x6226c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x62270: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x62274: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x62278: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x6227c: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x62280: ldrb r1, [r5, #0x139]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,313)?)?;
    // 0x62284: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62288: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x6228c: movne r1, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,4096)?)?;
    // 0x62290: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x62294: ldr r1, [r5, #0x868]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2152)?)?;
    // 0x62298: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x6229c: ldr r1, [r5, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2104)?)?;
    // 0x622a0: ldrb r1, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,339)?)?;
    // 0x622a4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x622a8: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // 0x622ac: strne r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::SP,4)?)?;
    // 0x622b0: ldr r0, [r5, #0x388]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,904)?)?;
    // 0x622b4: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x622b8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x622bc: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2cd3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x622c0: ldrb r0, [r5, #0xa48]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2632)?)?;
    // 0x622c4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x622c8: beq 0x622f0 <entry+0xcaf0>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x622f0,false)?)?;
    // 0x622cc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x622d0: bl 0x616a8 <entry+0xbea8>  @ imm = #-0xc30
    c.emit(arm32::branch(C::Al,c.at as u32,0x616a8,true)?)?;
    // 0x622d4: ldr r0, [r5, #0xa54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2644)?)?;
    // 0x622d8: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // 0x622dc: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x622e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x622e4: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x622e8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x622ec: bl 0x35588 <entry+0xfffffffffffdfd88> @ imm = #-0x2cd6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35588,true)?)?;
    // 0x622f0: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x622f4: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x622f8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x622fc: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x62300: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x62304: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x62308: mov r6, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,256)?)?;
    // 0x6230c: mov r10, r2
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R2))?;
    // 0x62310: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x62314: mov r5, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,256)?)?;
    // 0x62318: ldrb r0, [r7, #0x139]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,313)?)?;
    // 0x6231c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62320: ldrb r0, [r7, #0xa48]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,2632)?)?;
    // 0x62324: movne r6, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,4096)?)?;
    // 0x62328: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6232c: beq 0x62358 <entry+0xcb58>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62358,false)?)?;
    // 0x62330: ldr r2, [r7, #0xa54]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,2644)?)?;
    // 0x62334: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x62338: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x6233c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x2cd68
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x62340: ldr r2, [pc, #0x7c]         @ 0xcbc4 <entry+0xcbc4>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,124)?)?;
    // 0x62344: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62348: str r0, [r7, #0xa4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,2636)?)?;
    // 0x6234c: mov r1, #2592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2592)?)?;
    // 0x62350: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x62354: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x60bac
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x62358: ldr r1, [r7, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,2104)?)?;
    // 0x6235c: ldr r0, [r7, #0x868]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,2152)?)?;
    // 0x62360: ldr r4, [r7, #0x388]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,904)?)?;
    // 0x62364: ldrb r1, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,339)?)?;
    // 0x62368: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x6236c: moveq r5, r0
    c.emit(arm32::mov_register(C::Eq,R::new(5)?,R::R0))?;
    // 0x62370: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x62374: beq 0x623bc <entry+0xcbbc>  @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x623bc,false)?)?;
    // 0x62378: add r8, r8, #684
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,684)?)?;
    // 0x6237c: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x62380: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x62384: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x62388: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x6238c: bl 0x355dc <entry+0xfffffffffffdfddc> @ imm = #-0x2cdb8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x62390: ldr r2, [pc, #0x2c]         @ 0xcbc4 <entry+0xcbc4>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,44)?)?;
    // 0x62394: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x62398: mov r1, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,46)?)?;
    // 0x6239c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x623a0: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2560)?)?;
    // 0x623a4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x623a8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x60c00
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x623ac: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x623b0: str r7, [r8], #72
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(7)?,R::new(8)?,72)?)?;
    // 0x623b4: cmp r9, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x623b8: bne 0x62380 <entry+0xcb80>  @ imm = #-0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62380,false)?)?;
    // 0x623bc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x623c0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x623c4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_623C4:[u32;1]=[0x000f1748];
 for value in INLINE_DATA_623C4{c.emit(value)?;}
 // Executable A32 block 0x623c8..0x62bf4.
 c.check(0x623c8)?;
    // 0x623c8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x623cc: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x623d0: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x623d4: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x623d8: ldr r2, [pc, #0x814]        @ 0xd3f4 <entry+0xd3f4>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2068)?)?;
    // 0x623dc: ldr r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,44)?)?;
    // 0x623e0: ldr r4, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,36)?)?;
    // 0x623e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x623e8: ldr r9, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,368)?)?;
    // 0x623ec: ldrb r1, [r9, #0x5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,5)?)?;
    // 0x623f0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x623f4: mov r1, #95
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,95)?)?;
    // 0x623f8: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x623fc: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2560)?)?;
    // 0x62400: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x60c58
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x62404: ldr r8, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,368)?)?;
    // 0x62408: ldr r0, [r8, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,32)?)?;
    // 0x6240c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62410: bne 0x62440 <entry+0xcc40>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62440,false)?)?;
    // 0x62414: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x62418: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x6241c: strb r0, [r1, #0x264]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::R1,612)?)?;
    // 0x62420: strb r0, [r1, #0x4]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x62424: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x62428: strb r0, [r1, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,3)?)?;
    // 0x6242c: strb r0, [r1, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,2)?)?;
    // 0x62430: strb r0, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,1)?)?;
    // 0x62434: strb r0, [r1, #-0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,-1)?)?;
    // 0x62438: strb r0, [r1, #-0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,-2)?)?;
    // 0x6243c: strb r0, [r1, #-0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,-3)?)?;
    // 0x62440: ldr r2, [r4, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,588)?)?;
    // 0x62444: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x62448: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x6244c: beq 0x62490 <entry+0xcc90>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62490,false)?)?;
    // 0x62450: add r0, r4, #612
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,612)?)?;
    // 0x62454: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x62458: mvn r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,0)?)?;
    // 0x6245c: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x62460: ldrb r5, [r5, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(5)?,1)?)?;
    // 0x62464: cmp r3, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x62468: beq 0x6247c <entry+0xcc7c>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6247c,false)?)?;
    // 0x6246c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x62470: cmp r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,6)?)?;
    // 0x62474: ble 0x6245c <entry+0xcc5c>  @ imm = #-0x20
    c.emit(arm32::branch(C::Le,c.at as u32,0x6245c,false)?)?;
    // 0x62478: b 0x6248c <entry+0xcc8c>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x6248c,false)?)?;
    // 0x6247c: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x62480: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x62484: blo 0x62458 <entry+0xcc58>  @ imm = #-0x34
    c.emit(arm32::branch(C::Cc,c.at as u32,0x62458,false)?)?;
    // 0x62488: b 0x62490 <entry+0xcc90>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x62490,false)?)?;
    // 0x6248c: mov r1, r3
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // 0x62490: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x62494: str r1, [r0, #0x248]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R1,R::R0,584)?)?;
    // 0x62498: add r1, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x6249c: ldr r2, [r8, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,48)?)?;
    // 0x624a0: add r1, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x624a4: str r2, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,72)?)?;
    // 0x624a8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x624ac: strb r1, [r0, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x624b0: ldr r1, [r0, #-0xd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,-216)?)?;
    // 0x624b4: ldr r2, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,32)?)?;
    // 0x624b8: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x624bc: beq 0x624f4 <entry+0xccf4>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x624f4,false)?)?;
    // 0x624c0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x624c4: beq 0x6256c <entry+0xcd6c>  @ imm = #0xa0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6256c,false)?)?;
    // 0x624c8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x624cc: bne 0x627a8 <entry+0xcfa8>  @ imm = #0x2d4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x627a8,false)?)?;
    // 0x624d0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x624d4: strb r0, [r1, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,637)?)?;
    // 0x624d8: strb r0, [r1, #0x27e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,638)?)?;
    // 0x624dc: str r0, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,64)?)?;
    // 0x624e0: str r0, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,68)?)?;
    // 0x624e4: str r0, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,348)?)?;
    // 0x624e8: str r0, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,352)?)?;
    // 0x624ec: str r0, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1656)?)?;
    // 0x624f0: b 0x627a8 <entry+0xcfa8>  @ imm = #0x2b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x627a8,false)?)?;
    // 0x624f4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x624f8: strb r0, [r1, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,637)?)?;
    // 0x624fc: strb r0, [r1, #0x27e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,638)?)?;
    // 0x62500: add r0, r4, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,648)?)?;
    // 0x62504: ldrb r3, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,612)?)?;
    // 0x62508: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x6250c: add r3, r0, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62510: ldr r3, [r3, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,36)?)?;
    // 0x62514: str r3, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,64)?)?;
    // 0x62518: ldrb r3, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,613)?)?;
    // 0x6251c: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62520: add r3, r0, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62524: ldr r3, [r3, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,36)?)?;
    // 0x62528: str r3, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,68)?)?;
    // 0x6252c: ldrb r3, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,613)?)?;
    // 0x62530: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62534: add r3, r0, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62538: ldr r3, [r3, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,44)?)?;
    // 0x6253c: str r3, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,72)?)?;
    // 0x62540: ldrb r3, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,612)?)?;
    // 0x62544: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62548: add r3, r0, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x6254c: ldr r3, [r3, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,8)?)?;
    // 0x62550: str r3, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,348)?)?;
    // 0x62554: ldrb r3, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,613)?)?;
    // 0x62558: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x6255c: add r0, r0, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62560: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x62564: str r0, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,352)?)?;
    // 0x62568: b 0x627a8 <entry+0xcfa8>  @ imm = #0x238
    c.emit(arm32::branch(C::Al,c.at as u32,0x627a8,false)?)?;
    // 0x6256c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x62570: strb r0, [r1, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,637)?)?;
    // 0x62574: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62578: strb r0, [r1, #0x27e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,638)?)?;
    // 0x6257c: str r0, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,68)?)?;
    // 0x62580: ldr r3, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,644)?)?;
    // 0x62584: cmp r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // 0x62588: bne 0x625e4 <entry+0xcde4>  @ imm = #0x54
    c.emit(arm32::branch(C::Ne,c.at as u32,0x625e4,false)?)?;
    // 0x6258c: ldr r12, [r10, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(10)?,1236)?)?;
    // 0x62590: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // 0x62594: cmp r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // 0x62598: bne 0x625b8 <entry+0xcdb8>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x625b8,false)?)?;
    // 0x6259c: ldrb r0, [r10, #0x561]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1377)?)?;
    // 0x625a0: ldr r3, [pc, #0x650]        @ 0xd3f8 <entry+0xd3f8>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,1616)?)?;
    // 0x625a4: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x625a8: add r0, r3, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,1,false)?)?;
    // 0x625ac: ldrb r3, [r1, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,10)?)?;
    // 0x625b0: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x625b4: ldrb lr, [r0, #-0x6]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::R0,-6)?)?;
    // 0x625b8: ldr r0, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1656)?)?;
    // 0x625bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x625c0: ldrne r0, [r10, #0x38c]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(10)?,908)?)?;
    // 0x625c4: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x625c8: beq 0x62668 <entry+0xce68>  @ imm = #0x98
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62668,false)?)?;
    // 0x625cc: cmp r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // 0x625d0: bne 0x62650 <entry+0xce50>  @ imm = #0x78
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62650,false)?)?;
    // 0x625d4: ldrb r0, [r10, #0x561]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1377)?)?;
    // 0x625d8: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x625dc: movls lr, #2
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::LR,R::R0,2)?)?;
    // 0x625e0: b 0x62660 <entry+0xce60>  @ imm = #0x78
    c.emit(arm32::branch(C::Al,c.at as u32,0x62660,false)?)?;
    // 0x625e4: ldr r3, [r10, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,924)?)?;
    // 0x625e8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x625ec: beq 0x62600 <entry+0xce00>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62600,false)?)?;
    // 0x625f0: ldrb r3, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,613)?)?;
    // 0x625f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x625f8: cmp r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,255)?)?;
    // 0x625fc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x62600: add r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x62604: add r7, r4, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,648)?)?;
    // 0x62608: ldrb r3, [r0, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,612)?)?;
    // 0x6260c: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62610: add r3, r7, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62614: ldr r3, [r3, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,36)?)?;
    // 0x62618: str r3, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,64)?)?;
    // 0x6261c: ldrb r3, [r0, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,612)?)?;
    // 0x62620: add r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62624: add r3, r7, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62628: ldr r3, [r3, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,44)?)?;
    // 0x6262c: str r3, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,72)?)?;
    // 0x62630: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x62634: ldrb r0, [r0, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,612)?)?;
    // 0x62638: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6263c: add r0, r7, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x62640: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x62644: str r0, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,348)?)?;
    // 0x62648: str r3, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,352)?)?;
    // 0x6264c: b 0x627a8 <entry+0xcfa8>  @ imm = #0x154
    c.emit(arm32::branch(C::Al,c.at as u32,0x627a8,false)?)?;
    // 0x62650: ldr r0, [r4, #0x68c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1676)?)?;
    // 0x62654: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x62658: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6265c: movne lr, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::LR,R::R0,2)?)?;
    // 0x62660: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62664: str r0, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1656)?)?;
    // 0x62668: add r0, r4, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x6266c: ldrb r3, [r0, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,612)?)?;
    // 0x62670: add r5, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x62674: add r3, r4, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,648)?)?;
    // 0x62678: add r5, r3, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x6267c: ldr r5, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,36)?)?;
    // 0x62680: str r5, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,64)?)?;
    // 0x62684: ldrb r5, [r0, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R0,612)?)?;
    // 0x62688: add r5, r5, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x6268c: add r5, r3, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x62690: ldr r5, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,44)?)?;
    // 0x62694: str r5, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,72)?)?;
    // 0x62698: strb lr, [r1, #0x3ae]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::R1,942)?)?;
    // 0x6269c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x626a0: ldrb r0, [r0, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,612)?)?;
    // 0x626a4: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x626a8: add r0, r3, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x626ac: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x626b0: str r0, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,348)?)?;
    // 0x626b4: str r5, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,352)?)?;
    // 0x626b8: ldr r0, [r10, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2732)?)?;
    // 0x626bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x626c0: beq 0x62798 <entry+0xcf98>  @ imm = #0xd0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62798,false)?)?;
    // 0x626c4: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x626c8: ldrb r0, [r0, #0x4e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1252)?)?;
    // 0x626cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x626d0: beq 0x62930 <entry+0xd130>  @ imm = #0x258
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62930,false)?)?;
    // 0x626d4: ldrb r0, [r1, #0x424]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,1060)?)?;
    // 0x626d8: ldrb r7, [r1, #0x425]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R1,1061)?)?;
    // 0x626dc: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x626e0: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x626e4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x626e8: add r6, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,255)?)?;
    // 0x626ec: strb r5, [r1, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R1,637)?)?;
    // 0x626f0: add r5, r4, #612
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,612)?)?;
    // 0x626f4: strb r7, [r1, #0x27e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R1,638)?)?;
    // 0x626f8: lsl r7, r6, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(6)?,Shift::Lsl,24)?)?;
    // 0x626fc: ldrb r0, [r5, r7, asr #24]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(5)?,R::new(7)?,Shift::Asr,24)?)?;
    // 0x62700: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x62704: add r0, r3, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x62708: ldr lr, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,36)?)?;
    // 0x6270c: str lr, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,64)?)?;
    // 0x62710: ldrb r12, [r5, r7, asr #24]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R12,R::new(5)?,R::new(7)?,Shift::Asr,24)?)?;
    // 0x62714: add r0, r12, r12, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::R12,Shift::Lsl,3,false)?)?;
    // 0x62718: add r0, r3, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6271c: ldr r0, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,44)?)?;
    // 0x62720: str r0, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,72)?)?;
    // 0x62724: strb r6, [r1, #0x3ae]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,942)?)?;
    // 0x62728: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x6272c: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // 0x62730: strb r0, [r1, #0x3af]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,943)?)?;
    // 0x62734: lsl r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,24)?)?;
    // 0x62738: ldrb r7, [r5, r7, asr #24]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(7)?,R::new(5)?,R::new(7)?,Shift::Asr,24)?)?;
    // 0x6273c: add r7, r7, r7, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x62740: add r7, r3, r7, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x62744: ldr r7, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,8)?)?;
    // 0x62748: str r7, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,348)?)?;
    // 0x6274c: ldrb r7, [r5, r0, asr #24]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(7)?,R::new(5)?,R::R0,Shift::Asr,24)?)?;
    // 0x62750: add r7, r7, r7, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x62754: add r7, r3, r7, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x62758: ldr r7, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,36)?)?;
    // 0x6275c: str r7, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,68)?)?;
    // 0x62760: cmp lr, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x62764: ldrb r0, [r5, r0, asr #24]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Asr,24)?)?;
    // 0x62768: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6276c: add r0, r3, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x62770: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x62774: str r0, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,352)?)?;
    // 0x62778: bne 0x627a8 <entry+0xcfa8>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x627a8,false)?)?;
    // 0x6277c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x62780: strb r0, [r1, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,637)?)?;
    // 0x62784: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62788: str r0, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,68)?)?;
    // 0x6278c: str r0, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,352)?)?;
    // 0x62790: strb r0, [r1, #0x3ae]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,942)?)?;
    // 0x62794: b 0x627a8 <entry+0xcfa8>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x627a8,false)?)?;
    // 0x62798: cmp r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // 0x6279c: moveq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x627a0: strbeq lr, [r1, #0x3ae]
    c.emit(arm32::transfer_byte(C::Eq,false,R::LR,R::R1,942)?)?;
    // 0x627a4: strbeq r0, [r1, #0x3af]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::R1,943)?)?;
    // 0x627a8: ldrb r0, [r1, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,637)?)?;
    // 0x627ac: strb r0, [r1, #0x3ad]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,941)?)?;
    // 0x627b0: strb r2, [r1, #0x3ac]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,940)?)?;
    // 0x627b4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x627b8: ldr r3, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,368)?)?;
    // 0x627bc: ldr r1, [r4, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,584)?)?;
    // 0x627c0: ldr r0, [r3, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,32)?)?;
    // 0x627c4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x627c8: beq 0x62814 <entry+0xd014>  @ imm = #0x44
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62814,false)?)?;
    // 0x627cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x627d0: bne 0x62a50 <entry+0xd250>  @ imm = #0x278
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62a50,false)?)?;
    // 0x627d4: strb r1, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // 0x627d8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x627dc: ldr r0, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,644)?)?;
    // 0x627e0: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x627e4: strbeq r1, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(4)?,613)?)?;
    // 0x627e8: strbeq r1, [r4, #0x266]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(4)?,614)?)?;
    // 0x627ec: moveq r2, #7
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x627f0: ldr r0, [r4, #0x670]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1648)?)?;
    // 0x627f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x627f8: beq 0x62a50 <entry+0xd250>  @ imm = #0x250
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62a50,false)?)?;
    // 0x627fc: ldr r0, [r10, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,908)?)?;
    // 0x62800: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62804: beq 0x62a50 <entry+0xd250>  @ imm = #0x244
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62a50,false)?)?;
    // 0x62808: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6280c: str r0, [r4, #0x670]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1648)?)?;
    // 0x62810: b 0x62a50 <entry+0xd250>  @ imm = #0x238
    c.emit(arm32::branch(C::Al,c.at as u32,0x62a50,false)?)?;
    // 0x62814: ldrb r0, [r3, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,2)?)?;
    // 0x62818: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6281c: beq 0x62a50 <entry+0xd250>  @ imm = #0x22c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62a50,false)?)?;
    // 0x62820: ldr r0, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,644)?)?;
    // 0x62824: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x62828: bne 0x62a50 <entry+0xd250>  @ imm = #0x220
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62a50,false)?)?;
    // 0x6282c: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x62830: ldrb r2, [r0, #0x4e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,1252)?)?;
    // 0x62834: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x62838: beq 0x6290c <entry+0xd10c>  @ imm = #0xcc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6290c,false)?)?;
    // 0x6283c: ldrb r12, [r0, #0x4e5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::R0,1253)?)?;
    // 0x62840: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x62844: strb r12, [r3, #0x424]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(3)?,1060)?)?;
    // 0x62848: cmp r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,2)?)?;
    // 0x6284c: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x62850: ldrb r0, [r0, #0x4e6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1254)?)?;
    // 0x62854: cmpeq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62858: strb r0, [r3, #0x425]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,1061)?)?;
    // 0x6285c: bne 0x62880 <entry+0xd080>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62880,false)?)?;
    // 0x62860: ldrb r0, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,612)?)?;
    // 0x62864: mov r2, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,21)?)?;
    // 0x62868: strb r0, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,613)?)?;
    // 0x6286c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x62870: strb r1, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // 0x62874: strb r1, [r4, #0x266]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,614)?)?;
    // 0x62878: strb r0, [r3, #0x427]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,1063)?)?;
    // 0x6287c: ldrb r12, [r3, #0x424]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(3)?,1060)?)?;
    // 0x62880: cmp r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // 0x62884: bne 0x628ac <entry+0xd0ac>  @ imm = #0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x628ac,false)?)?;
    // 0x62888: ldrb r0, [r3, #0x425]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,1061)?)?;
    // 0x6288c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x62890: bne 0x628ec <entry+0xd0ec>  @ imm = #0x54
    c.emit(arm32::branch(C::Ne,c.at as u32,0x628ec,false)?)?;
    // 0x62894: ldrb r0, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,612)?)?;
    // 0x62898: mov r2, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,21)?)?;
    // 0x6289c: strb r0, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,613)?)?;
    // 0x628a0: strb r1, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // 0x628a4: strb r1, [r4, #0x266]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,614)?)?;
    // 0x628a8: ldrb r12, [r3, #0x424]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(3)?,1060)?)?;
    // 0x628ac: cmp r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,3)?)?;
    // 0x628b0: bne 0x628e0 <entry+0xd0e0>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x628e0,false)?)?;
    // 0x628b4: ldrb r0, [r3, #0x425]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,1061)?)?;
    // 0x628b8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x628bc: bne 0x62a50 <entry+0xd250>  @ imm = #0x18c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62a50,false)?)?;
    // 0x628c0: ldrb r0, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,612)?)?;
    // 0x628c4: mov r2, #131
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,131)?)?;
    // 0x628c8: strb r0, [r4, #0x266]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,614)?)?;
    // 0x628cc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x628d0: strb r1, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // 0x628d4: strb r1, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,613)?)?;
    // 0x628d8: strb r0, [r3, #0x427]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,1063)?)?;
    // 0x628dc: ldrb r12, [r3, #0x424]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(3)?,1060)?)?;
    // 0x628e0: cmp r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // 0x628e4: bne 0x62a50 <entry+0xd250>  @ imm = #0x164
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62a50,false)?)?;
    // 0x628e8: ldrb r0, [r3, #0x425]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,1061)?)?;
    // 0x628ec: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x628f0: bne 0x62a50 <entry+0xd250>  @ imm = #0x158
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62a50,false)?)?;
    // 0x628f4: ldrb r0, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,612)?)?;
    // 0x628f8: mov r2, #131
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,131)?)?;
    // 0x628fc: strb r0, [r4, #0x266]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,614)?)?;
    // 0x62900: strb r1, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // 0x62904: strb r1, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,613)?)?;
    // 0x62908: b 0x62a50 <entry+0xd250>  @ imm = #0x140
    c.emit(arm32::branch(C::Al,c.at as u32,0x62a50,false)?)?;
    // 0x6290c: ldrb r0, [r10, #0x3e2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,994)?)?;
    // 0x62910: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62914: beq 0x629d8 <entry+0xd1d8>  @ imm = #0xbc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x629d8,false)?)?;
    // 0x62918: ldrb r0, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,612)?)?;
    // 0x6291c: mov r2, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,21)?)?;
    // 0x62920: strb r0, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,613)?)?;
    // 0x62924: strb r1, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // 0x62928: strb r1, [r4, #0x266]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,614)?)?;
    // 0x6292c: b 0x62a50 <entry+0xd250>  @ imm = #0x11c
    c.emit(arm32::branch(C::Al,c.at as u32,0x62a50,false)?)?;
    // 0x62930: ldrb r0, [r10, #0x3e2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,994)?)?;
    // 0x62934: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62938: beq 0x627a8 <entry+0xcfa8>  @ imm = #-0x198
    c.emit(arm32::branch(C::Eq,c.at as u32,0x627a8,false)?)?;
    // 0x6293c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x62940: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x62944: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x62948: strb r0, [r1, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,637)?)?;
    // 0x6294c: strb r12, [r1, #0x27e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R1,638)?)?;
    // 0x62950: ldrb r0, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,612)?)?;
    // 0x62954: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x62958: add r0, r3, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6295c: ldr r7, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,36)?)?;
    // 0x62960: str r7, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,64)?)?;
    // 0x62964: ldrb r0, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,612)?)?;
    // 0x62968: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x6296c: add r0, r3, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x62970: ldr r0, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,44)?)?;
    // 0x62974: str r0, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,72)?)?;
    // 0x62978: strb r12, [r1, #0x3ae]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R1,942)?)?;
    // 0x6297c: strb lr, [r1, #0x3af]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::R1,943)?)?;
    // 0x62980: ldrb r5, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,612)?)?;
    // 0x62984: add r5, r5, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x62988: add r5, r3, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x6298c: ldr r5, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,8)?)?;
    // 0x62990: str r5, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,348)?)?;
    // 0x62994: ldrb r5, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,613)?)?;
    // 0x62998: add r5, r5, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x6299c: add r5, r3, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x629a0: ldr r5, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,36)?)?;
    // 0x629a4: str r5, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,68)?)?;
    // 0x629a8: cmp r7, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x629ac: ldrb r0, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,613)?)?;
    // 0x629b0: add r0, r0, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x629b4: add r0, r3, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x629b8: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x629bc: str r0, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,352)?)?;
    // 0x629c0: bne 0x627a8 <entry+0xcfa8>  @ imm = #-0x220
    c.emit(arm32::branch(C::Ne,c.at as u32,0x627a8,false)?)?;
    // 0x629c4: strb lr, [r1, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::R1,637)?)?;
    // 0x629c8: str r12, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,68)?)?;
    // 0x629cc: str r12, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,352)?)?;
    // 0x629d0: strb r12, [r1, #0x3ae]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R1,942)?)?;
    // 0x629d4: b 0x627a8 <entry+0xcfa8>  @ imm = #-0x234
    c.emit(arm32::branch(C::Al,c.at as u32,0x627a8,false)?)?;
    // 0x629d8: strb r1, [r4, #0x264]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // 0x629dc: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x629e0: ldr r0, [r10, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1236)?)?;
    // 0x629e4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x629e8: bne 0x62a24 <entry+0xd224>  @ imm = #0x34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62a24,false)?)?;
    // 0x629ec: ldrb r0, [r10, #0x561]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1377)?)?;
    // 0x629f0: ldr r2, [pc, #0x204]        @ 0xd3fc <entry+0xd3fc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,516)?)?;
    // 0x629f4: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x629f8: add r0, r2, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // 0x629fc: ldrb r2, [r3, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(3)?,10)?)?;
    // 0x62a00: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x62a04: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x62a08: ldrb r0, [r0, #-0x6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,-6)?)?;
    // 0x62a0c: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x62a10: strbne r1, [r4, #0x265]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(4)?,613)?)?;
    // 0x62a14: movne r2, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x62a18: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // 0x62a1c: orrne r2, r2, #4
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::R2,4)?)?;
    // 0x62a20: strbne r1, [r4, #0x266]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(4)?,614)?)?;
    // 0x62a24: ldr r0, [r4, #0x670]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1648)?)?;
    // 0x62a28: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62a2c: ldrne r0, [r10, #0x38c]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(10)?,908)?)?;
    // 0x62a30: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62a34: beq 0x62a50 <entry+0xd250>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62a50,false)?)?;
    // 0x62a38: ldr r0, [r10, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1236)?)?;
    // 0x62a3c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62a40: bne 0x62bdc <entry+0xd3dc>  @ imm = #0x194
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62bdc,false)?)?;
    // 0x62a44: ldrb r0, [r3, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,10)?)?;
    // 0x62a48: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62a4c: beq 0x62be8 <entry+0xd3e8>  @ imm = #0x194
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62be8,false)?)?;
    // 0x62a50: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x62a54: str r2, [r8, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,948)?)?;
    // 0x62a58: strb r0, [r8, #0x3b1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(8)?,945)?)?;
    // 0x62a5c: add r0, r4, #26
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,26)?)?;
    // 0x62a60: add r1, r0, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1280)?)?;
    // 0x62a64: ldr r0, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,368)?)?;
    // 0x62a68: ldrb r2, [r0, #0x1dd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,477)?)?;
    // 0x62a6c: ldrb r7, [r0, #0x1df]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R0,479)?)?;
    // 0x62a70: add r3, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x62a74: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x62a78: add r3, r1, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x62a7c: moveq r1, r3
    c.emit(arm32::mov_register(C::Eq,R::R1,R::new(3)?))?;
    // 0x62a80: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x62a84: moveq r1, r3
    c.emit(arm32::mov_register(C::Eq,R::R1,R::new(3)?))?;
    // 0x62a88: ldr r3, [r0, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,68)?)?;
    // 0x62a8c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x62a90: bne 0x62aa0 <entry+0xd2a0>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62aa0,false)?)?;
    // 0x62a94: ldr r7, [r0, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,64)?)?;
    // 0x62a98: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x62a9c: beq 0x62af8 <entry+0xd2f8>  @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62af8,false)?)?;
    // 0x62aa0: add r2, r4, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x62aa4: add r7, r4, #352
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,352)?)?;
    // 0x62aa8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x62aac: ldrb r2, [r2, #0x518]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,1304)?)?;
    // 0x62ab0: addeq r7, r4, #348
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(7)?,R::new(4)?,348)?)?;
    // 0x62ab4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x62ab8: beq 0x62b04 <entry+0xd304>  @ imm = #0x44
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62b04,false)?)?;
    // 0x62abc: ldr r3, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,0)?)?;
    // 0x62ac0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x62ac4: ldrb r5, [r1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R1,0)?)?;
    // 0x62ac8: add r5, r4, r5, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,4,false)?)?;
    // 0x62acc: ldr r6, [r5, #0x528]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,1320)?)?;
    // 0x62ad0: cmp r6, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x62ad4: beq 0x62aec <entry+0xd2ec>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62aec,false)?)?;
    // 0x62ad8: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x62adc: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x62ae0: cmp r7, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x62ae4: blt 0x62ac4 <entry+0xd2c4>  @ imm = #-0x28
    c.emit(arm32::branch(C::Lt,c.at as u32,0x62ac4,false)?)?;
    // 0x62ae8: b 0x62b04 <entry+0xd304>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x62b04,false)?)?;
    // 0x62aec: add r1, r5, #308
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,308)?)?;
    // 0x62af0: add r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // 0x62af4: b 0x62afc <entry+0xd2fc>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x62afc,false)?)?;
    // 0x62af8: add r1, r0, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,76)?)?;
    // 0x62afc: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x62b00: str r1, [r0, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,72)?)?;
    // 0x62b04: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x62b08: mov r7, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4278190080)?)?;
    // 0x62b0c: ldr r1, [r0, #0x248]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,584)?)?;
    // 0x62b10: add r2, r1, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x62b14: add r0, r0, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,3,false)?)?;
    // 0x62b18: ldr r2, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,100)?)?;
    // 0x62b1c: str r2, [r9, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,56)?)?;
    // 0x62b20: str r1, [r9, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,60)?)?;
    // 0x62b24: ldr r0, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,108)?)?;
    // 0x62b28: str r0, [r9, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,76)?)?;
    // 0x62b2c: ldr r1, [r9, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,72)?)?;
    // 0x62b30: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62b34: add r1, r10, #984
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,984)?)?;
    // 0x62b38: streq r0, [r9, #0x48]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,72)?)?;
    // 0x62b3c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x62b40: add r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // 0x62b44: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // 0x62b48: ldrsb r0, [r10, r0]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R0,R::new(10)?,R::R0)?)?;
    // 0x62b4c: ldr r2, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x62b50: add r7, r7, r0, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::R0,Shift::Lsl,24,false)?)?;
    // 0x62b54: asr r7, r7, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Asr,24)?)?;
    // 0x62b58: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x62b5c: str r2, [r9, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,84)?)?;
    // 0x62b60: add r2, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(10)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x62b64: ldr r2, [r2, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,2032)?)?;
    // 0x62b68: str r2, [r9, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,88)?)?;
    // 0x62b6c: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x62b70: add r3, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,255)?)?;
    // 0x62b74: strb r3, [r10, #0x809]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(10)?,2057)?)?;
    // 0x62b78: ldrblt r3, [r10, #0x80a]
    c.emit(arm32::transfer_byte(C::Lt,true,R::new(3)?,R::new(10)?,2058)?)?;
    // 0x62b7c: addlt r3, r3, #255
    c.emit(arm32::data_immediate(C::Lt,Op::Add,R::new(3)?,R::new(3)?,255)?)?;
    // 0x62b80: strblt r3, [r10, #0x809]
    c.emit(arm32::transfer_byte(C::Lt,false,R::new(3)?,R::new(10)?,2057)?)?;
    // 0x62b84: lsl r3, r3, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,24)?)?;
    // 0x62b88: ldr r1, [r1, r3, asr #22]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(3)?,Shift::Asr,22)?)?;
    // 0x62b8c: str r1, [r9, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,80)?)?;
    // 0x62b90: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // 0x62b94: str r0, [r4, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,596)?)?;
    // 0x62b98: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // 0x62b9c: strb r0, [r10, #0x808]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,2056)?)?;
    // 0x62ba0: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x62ba4: lsl r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,24)?)?;
    // 0x62ba8: ldrsb r1, [r10, r1]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R1,R::new(10)?,R::R1)?)?;
    // 0x62bac: asr r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,24)?)?;
    // 0x62bb0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x62bb4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x62bb8: movge r0, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62bbc: strbge r0, [r10, #0x808]
    c.emit(arm32::transfer_byte(C::Ge,false,R::R0,R::new(10)?,2056)?)?;
    // 0x62bc0: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x62bc4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x62bc8: str r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // 0x62bcc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x62bd0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x62bd4: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x62bd8: b 0x62c00 <entry+0xd400>  @ imm = #0x20
    c.emit(arm32::branch(C::Al,c.at as u32,0x62c00,false)?)?;
    // 0x62bdc: ldr r0, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1672)?)?;
    // 0x62be0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62be4: bne 0x62808 <entry+0xd008>  @ imm = #-0x3e4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62808,false)?)?;
    // 0x62be8: orr r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4)?)?;
    // 0x62bec: strb r1, [r4, #0x266]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,614)?)?;
    // 0x62bf0: b 0x62808 <entry+0xd008>  @ imm = #-0x3f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x62808,false)?)?;
 c.check(0x62bf4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_62BF4:[u32;3]=[0x000f1748,0x000f1a3c,0x000f1a60];
 for value in INLINE_DATA_62BF4{c.emit(value)?;}
 // Executable A32 block 0x62c00..0x62dcc.
 c.check(0x62c00)?;
    // 0x62c00: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x62c04: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x62c08: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x62c0c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x62c10: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x62c14: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x62c18: beq 0x62c54 <entry+0xd454>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62c54,false)?)?;
    // 0x62c1c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62c20: beq 0x62c7c <entry+0xd47c>  @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62c7c,false)?)?;
    // 0x62c24: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62c28: bne 0x62ce8 <entry+0xd4e8>  @ imm = #0xb8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62ce8,false)?)?;
    // 0x62c2c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62c30: str r0, [r5, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,412)?)?;
    // 0x62c34: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // 0x62c38: str r0, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,416)?)?;
    // 0x62c3c: str r0, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,508)?)?;
    // 0x62c40: str r0, [r5, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,420)?)?;
    // 0x62c44: str r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,512)?)?;
    // 0x62c48: str r0, [r5, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,424)?)?;
    // 0x62c4c: str r0, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,516)?)?;
    // 0x62c50: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x62c54: ldr r0, [r5, #0x240]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,576)?)?;
    // 0x62c58: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62c5c: bne 0x62c70 <entry+0xd470>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62c70,false)?)?;
    // 0x62c60: ldr r1, [r5, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,412)?)?;
    // 0x62c64: str r1, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,512)?)?;
    // 0x62c68: ldr r1, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,416)?)?;
    // 0x62c6c: str r1, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,516)?)?;
    // 0x62c70: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x62c74: str r0, [r5, #0x240]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,576)?)?;
    // 0x62c78: b 0x62d38 <entry+0xd538>  @ imm = #0xb8
    c.emit(arm32::branch(C::Al,c.at as u32,0x62d38,false)?)?;
    // 0x62c7c: ldrb r0, [r4, #0x27d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,637)?)?;
    // 0x62c80: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x62c84: ldreq r1, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,76)?)?;
    // 0x62c88: cmpeq r1, #8192
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x62c8c: beq 0x62d28 <entry+0xd528>  @ imm = #0x94
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62d28,false)?)?;
    // 0x62c90: ldrb r1, [r4, #0x9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,9)?)?;
    // 0x62c94: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x62c98: ldreq r1, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,76)?)?;
    // 0x62c9c: cmpeq r1, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x62ca0: beq 0x62d48 <entry+0xd548>  @ imm = #0xa0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62d48,false)?)?;
    // 0x62ca4: ldr r2, [pc, #0x120]        @ 0xd5cc <entry+0xd5cc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,288)?)?;
    // 0x62ca8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62cac: mov r1, #57
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,57)?)?;
    // 0x62cb0: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62cb4: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x62cb8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x61510
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x62cbc: ldr r0, [r5, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,412)?)?;
    // 0x62cc0: str r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,512)?)?;
    // 0x62cc4: ldr r0, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,416)?)?;
    // 0x62cc8: str r0, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,516)?)?;
    // 0x62ccc: ldr r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,504)?)?;
    // 0x62cd0: str r0, [r5, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,412)?)?;
    // 0x62cd4: ldr r0, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,508)?)?;
    // 0x62cd8: str r0, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,416)?)?;
    // 0x62cdc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62ce0: str r0, [r5, #0x240]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,576)?)?;
    // 0x62ce4: b 0x62d6c <entry+0xd56c>  @ imm = #0x80
    c.emit(arm32::branch(C::Al,c.at as u32,0x62d6c,false)?)?;
    // 0x62ce8: ldr r0, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,76)?)?;
    // 0x62cec: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x62cf0: bne 0x62d14 <entry+0xd514>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62d14,false)?)?;
    // 0x62cf4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62cf8: str r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,512)?)?;
    // 0x62cfc: str r0, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,516)?)?;
    // 0x62d00: ldr r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,504)?)?;
    // 0x62d04: str r0, [r5, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,412)?)?;
    // 0x62d08: ldr r0, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,508)?)?;
    // 0x62d0c: str r0, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,416)?)?;
    // 0x62d10: b 0x62d6c <entry+0xd56c>  @ imm = #0x54
    c.emit(arm32::branch(C::Al,c.at as u32,0x62d6c,false)?)?;
    // 0x62d14: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62d18: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // 0x62d1c: str r0, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,508)?)?;
    // 0x62d20: str r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,512)?)?;
    // 0x62d24: b 0x62d68 <entry+0xd568>  @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x62d68,false)?)?;
    // 0x62d28: ldr r0, [r5, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,412)?)?;
    // 0x62d2c: str r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,512)?)?;
    // 0x62d30: ldr r0, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,416)?)?;
    // 0x62d34: str r0, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,516)?)?;
    // 0x62d38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62d3c: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // 0x62d40: str r0, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,508)?)?;
    // 0x62d44: b 0x62d6c <entry+0xd56c>  @ imm = #0x20
    c.emit(arm32::branch(C::Al,c.at as u32,0x62d6c,false)?)?;
    // 0x62d48: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62d4c: str r0, [r5, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,412)?)?;
    // 0x62d50: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // 0x62d54: str r0, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,416)?)?;
    // 0x62d58: str r0, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,508)?)?;
    // 0x62d5c: str r0, [r5, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,420)?)?;
    // 0x62d60: str r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,512)?)?;
    // 0x62d64: str r0, [r5, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,424)?)?;
    // 0x62d68: str r0, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,516)?)?;
    // 0x62d6c: ldr r0, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,76)?)?;
    // 0x62d70: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x62d74: popne {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x62d78: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x62d7c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x62d80: ldrbne r0, [r4, #0x9]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(4)?,9)?)?;
    // 0x62d84: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x62d88: bne 0x62da4 <entry+0xd5a4>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62da4,false)?)?;
    // 0x62d8c: ldr r1, [r5, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,376)?)?;
    // 0x62d90: ldr r0, [r5, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,388)?)?;
    // 0x62d94: cmp r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x62d98: lslge r0, r1, #2
    c.emit(arm32::mov_shifted_register(C::Ge,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x62d9c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x62da0: b 0x62db8 <entry+0xd5b8>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x62db8,false)?)?;
    // 0x62da4: ldr r1, [r5, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,372)?)?;
    // 0x62da8: ldr r0, [r5, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,384)?)?;
    // 0x62dac: cmp r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x62db0: lslge r0, r1, #2
    c.emit(arm32::mov_shifted_register(C::Ge,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x62db4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x62db8: str r0, [r4, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,496)?)?;
    // 0x62dbc: add r0, r5, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x62dc0: ldr r0, [r0, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,392)?)?;
    // 0x62dc4: str r0, [r4, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,500)?)?;
    // 0x62dc8: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x62dcc)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_62DCC:[u32;1]=[0x000f1748];
 for value in INLINE_DATA_62DCC{c.emit(value)?;}
 // Executable A32 block 0x62dd0..0x63000.
 c.check(0x62dd0)?;
    // 0x62dd0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x62dd4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x62dd8: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // 0x62ddc: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x62de0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62de4: ldr r1, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,644)?)?;
    // 0x62de8: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x62dec: bne 0x63494 <entry+0xdc94>  @ imm = #0x6a0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63494,false)?)?;
    // 0x62df0: ldr r5, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,44)?)?;
    // 0x62df4: ldr r0, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,416)?)?;
    // 0x62df8: ldr r3, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,36)?)?;
    // 0x62dfc: cmn r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,2)?)?;
    // 0x62e00: beq 0x62e18 <entry+0xd618>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62e18,false)?)?;
    // 0x62e04: ldrb r0, [r3, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,3)?)?;
    // 0x62e08: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62e0c: ldrbne r0, [r3, #0x4]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(3)?,4)?)?;
    // 0x62e10: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62e14: bne 0x62f70 <entry+0xd770>  @ imm = #0x154
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62f70,false)?)?;
    // 0x62e18: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x62e1c: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x62e20: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x62e24: str r10, [r3, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,332)?)?;
    // 0x62e28: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x62e2c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x62e30: ldr r1, [r6, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,108)?)?;
    // 0x62e34: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62e38: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62e3c: beq 0x63494 <entry+0xdc94>  @ imm = #0x650
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63494,false)?)?;
    // 0x62e40: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62e44: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x62e48: strb r0, [r3, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,11)?)?;
    // 0x62e4c: bhi 0x62e5c <entry+0xd65c>  @ imm = #0x8
    c.emit(arm32::branch(C::Hi,c.at as u32,0x62e5c,false)?)?;
    // 0x62e50: ldr r2, [r6, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,116)?)?;
    // 0x62e54: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x62e58: subne r7, r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::new(7)?,R::R1,1)?)?;
    // 0x62e5c: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x62e60: bls 0x63494 <entry+0xdc94>  @ imm = #0x62c
    c.emit(arm32::branch(C::Ls,c.at as u32,0x63494,false)?)?;
    // 0x62e64: add r1, r5, #368
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,368)?)?;
    // 0x62e68: add r0, r6, #92
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,92)?)?;
    // 0x62e6c: add r8, r6, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(6)?,640)?)?;
    // 0x62e70: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x62e74: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x62e78: mov r7, r12
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R12))?;
    // 0x62e7c: ldr r2, [r5, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,420)?)?;
    // 0x62e80: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x62e84: bl 0x39970 <entry+0xfffffffffffe4170> @ imm = #-0x2951c
    c.emit(arm32::branch(C::Al,c.at as u32,0x39970,true)?)?;
    // 0x62e88: ldr r2, [pc, #0x6f8]        @ 0xdd88 <entry+0xdd88>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1784)?)?;
    // 0x62e8c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62e90: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x62e94: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62e98: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2816)?)?;
    // 0x62e9c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x616f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x62ea0: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x62ea4: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x62ea8: str r8, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,12)?)?;
    // 0x62eac: bl 0x6358c <entry+0xdd8c>  @ imm = #0x6d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x6358c,true)?)?;
    // 0x62eb0: ldr r12, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,16)?)?;
    // 0x62eb4: str r10, [r5, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,400)?)?;
    // 0x62eb8: strb r4, [r5, #0x171]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(5)?,369)?)?;
    // 0x62ebc: strb r7, [r5, #0x172]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(5)?,370)?)?;
    // 0x62ec0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x62ec4: ldr r0, [r12, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,332)?)?;
    // 0x62ec8: strb r0, [r5, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,996)?)?;
    // 0x62ecc: ldrb r0, [r5, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,378)?)?;
    // 0x62ed0: strb r0, [r5, #0x17b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,379)?)?;
    // 0x62ed4: strb r9, [r5, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(5)?,378)?)?;
    // 0x62ed8: ldr r0, [r12, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,104)?)?;
    // 0x62edc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62ee0: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x62ee4: strb r1, [r5, #0x176]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,374)?)?;
    // 0x62ee8: ldr r0, [r6, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,108)?)?;
    // 0x62eec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62ef0: bne 0x62f08 <entry+0xd708>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62f08,false)?)?;
    // 0x62ef4: ldr r0, [r6, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,116)?)?;
    // 0x62ef8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62efc: ldrne r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::SP,8)?)?;
    // 0x62f00: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x62f04: strbne r0, [r1]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::R1,0)?)?;
    // 0x62f08: ldr r0, [r5, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,420)?)?;
    // 0x62f0c: ldr r1, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,168)?)?;
    // 0x62f10: str r1, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,416)?)?;
    // 0x62f14: ldr r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,52)?)?;
    // 0x62f18: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62f1c: beq 0x63048 <entry+0xd848>  @ imm = #0x124
    c.emit(arm32::branch(C::Eq,c.at as u32,0x63048,false)?)?;
    // 0x62f20: ldr r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,40)?)?;
    // 0x62f24: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62f28: beq 0x6305c <entry+0xd85c>  @ imm = #0x12c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6305c,false)?)?;
    // 0x62f2c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x62f30: strb r1, [r5, #0x175]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,373)?)?;
    // 0x62f34: ldr r1, [r6, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,744)?)?;
    // 0x62f38: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62f3c: beq 0x6306c <entry+0xd86c>  @ imm = #0x128
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6306c,false)?)?;
    // 0x62f40: ldr r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,8)?)?;
    // 0x62f44: mov r0, r12
    c.emit(arm32::mov_register(C::Al,R::R0,R::R12))?;
    // 0x62f48: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x62f4c: bl 0x63704 <entry+0xdf04>  @ imm = #0x7b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x63704,true)?)?;
    // 0x62f50: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x62f54: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x62f58: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x62f5c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x62f60: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x62f64: bl 0x6b990 <entry+0x16190> @ imm = #0x8a24
    c.emit(arm32::branch(C::Al,c.at as u32,0x6b990,true)?)?;
    // 0x62f68: ldr r12, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,16)?)?;
    // 0x62f6c: b 0x63164 <entry+0xd964>  @ imm = #0x1f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x63164,false)?)?;
    // 0x62f70: ldr r9, [r6, #0x304]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(6)?,772)?)?;
    // 0x62f74: ldr r8, [r3, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(3)?,332)?)?;
    // 0x62f78: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x62f7c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x62f80: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x62f84: bl 0x81850 <entry+0x2c050> @ imm = #0x1e8c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x62f88: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x62f8c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62f90: beq 0x62e18 <entry+0xd618>  @ imm = #-0x180
    c.emit(arm32::branch(C::Eq,c.at as u32,0x62e18,false)?)?;
    // 0x62f94: ldrb r0, [r3, #0xb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,11)?)?;
    // 0x62f98: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x62f9c: bne 0x62e18 <entry+0xd618>  @ imm = #-0x18c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x62e18,false)?)?;
    // 0x62fa0: ldrb r1, [r6, #0x468]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,1128)?)?;
    // 0x62fa4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x62fa8: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x62fac: movhi r1, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x62fb0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x62fb4: beq 0x630d4 <entry+0xd8d4>  @ imm = #0x118
    c.emit(arm32::branch(C::Eq,c.at as u32,0x630d4,false)?)?;
    // 0x62fb8: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x62fbc: beq 0x630e4 <entry+0xd8e4>  @ imm = #0x120
    c.emit(arm32::branch(C::Eq,c.at as u32,0x630e4,false)?)?;
    // 0x62fc0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x62fc4: bne 0x63494 <entry+0xdc94>  @ imm = #0x4c8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63494,false)?)?;
    // 0x62fc8: ldr r0, [r6, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1236)?)?;
    // 0x62fcc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x62fd0: bne 0x63148 <entry+0xd948>  @ imm = #0x170
    c.emit(arm32::branch(C::Ne,c.at as u32,0x63148,false)?)?;
    // 0x62fd4: ldrb r0, [r3, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,10)?)?;
    // 0x62fd8: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x62fdc: sub r12, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R12,R::R0,1)?)?;
    // 0x62fe0: tst r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R12,255)?)?;
    // 0x62fe4: mov r9, r12
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R12))?;
    // 0x62fe8: beq 0x6302c <entry+0xd82c>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6302c,false)?)?;
    // 0x62fec: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x62ff0: mov r9, r12
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R12))?;
    // 0x62ff4: add r1, r1, r10, lsl r12
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R1,R::R1,R::new(10)?,Shift::Lsl,R::R12,false)?)?;
    // 0x62ff8: and r1, r1, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x62ffc: ands r2, r1, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R1,1,true)?)?;
 c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();emit_controller_packet(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests{use super::*;
 #[test]fn chunk_len_and_bounds(){assert_eq!(fragment().unwrap().len(),END-START);assert!(END<=0x68000);}
 #[test]fn original_firmware_byte_comparison_when_present(){
  if let Ok(path)=std::env::var("NABU_VENUS_REFERENCE").or_else(|_| std::env::var("AGENT04_PRIVATE_VENUS")){
   let reference=std::fs::read(path).unwrap();let generated=fragment().unwrap();
   assert_eq!(&generated[..],&reference[FILE_OFFSET..FILE_OFFSET+generated.len()]);
  }
 }
}
