// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x56800,0x57000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x56800;
pub const END:usize=0x57000;
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
 // Executable A32 block 0x56800..0x568b0.
 c.check(0x56800)?;
    // 0x56800: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x56804: beq 0x56928 <entry+0x1128>  @ imm = #0x11c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56928,false)?)?;
    // 0x56808: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5680c: str r1, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,924)?)?;
    // 0x56810: mov r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
    // 0x56814: b 0x55af8 <entry+0x2f8>   @ imm = #-0xd24
    c.emit(arm32::branch(C::Al,c.at as u32,0x55af8,false)?)?;
    // 0x56818: ldr r1, [r4, #0xd30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3376)?)?;
    // 0x5681c: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x56820: moveq r1, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x56824: str r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,648)?)?;
    // 0x56828: ldrb r1, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,988)?)?;
    // 0x5682c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x56830: moveq r1, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x56834: streq r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,648)?)?;
    // 0x56838: ldr r1, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2732)?)?;
    // 0x5683c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x56840: beq 0x5694c <entry+0x114c>  @ imm = #0x104
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5694c,false)?)?;
    // 0x56844: str r0, [r4, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,652)?)?;
    // 0x56848: b 0x56d1c <entry+0x151c>  @ imm = #0x4cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x56d1c,false)?)?;
    // 0x5684c: ldr r2, [pc, #0xc2c]        @ 0x1c80 <entry+0x1c80>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3116)?)?;
    // 0x56850: mov r1, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,46)?)?;
    // 0x56854: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56858: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x5685c: b 0x56d18 <entry+0x1518>  @ imm = #0x4b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x56d18,false)?)?;
    // 0x56860: ldrb r2, [r4, #0x3d9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,985)?)?;
    // 0x56864: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56868: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5686c: bne 0x56720 <entry+0xf20>   @ imm = #-0x154
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56720,false)?)?;
    // 0x56870: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x56874: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // 0x56878: bl 0x38a44 <entry+0xfffffffffffe3244> @ imm = #-0x1de3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x38a44,true)?)?;
    // 0x5687c: b 0x5671c <entry+0xf1c>   @ imm = #-0x168
    c.emit(arm32::branch(C::Al,c.at as u32,0x5671c,false)?)?;
    // 0x56880: cmp r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,16)?)?;
    // 0x56884: beq 0x56a10 <entry+0x1210>  @ imm = #0x184
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56a10,false)?)?;
    // 0x56888: cmp r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,32)?)?;
    // 0x5688c: beq 0x56a24 <entry+0x1224>  @ imm = #0x190
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56a24,false)?)?;
    // 0x56890: cmp r5, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,64)?)?;
    // 0x56894: bne 0x569a4 <entry+0x11a4>  @ imm = #0x108
    c.emit(arm32::branch(C::Ne,c.at as u32,0x569a4,false)?)?;
    // 0x56898: mov r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,100)?)?;
    // 0x5689c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x568a0: str r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,648)?)?;
    // 0x568a4: strb r2, [r4, #0x55e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1374)?)?;
    // 0x568a8: strb r2, [r4, #0x55f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1375)?)?;
    // 0x568ac: b 0x56a38 <entry+0x1238>  @ imm = #0x184
    c.emit(arm32::branch(C::Al,c.at as u32,0x56a38,false)?)?;
 c.check(0x568b0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_568B0:[u32;3]=[0xffffefee,0xffffefed,0xffffefeb];
 for value in INLINE_DATA_568B0{c.emit(value)?;}
 // Executable A32 block 0x568bc..0x56918.
 c.check(0x568bc)?;
    // 0x568bc: ldr r0, [r4, #0xd90]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3472)?)?;
    // 0x568c0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x568c4: beq 0x54d8c <entry+0xfffffffffffff58c> @ imm = #-0x1b40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54d8c,false)?)?;
    // 0x568c8: bl 0x2f8d0 <entry+0xfffffffffffda0d0> @ imm = #-0x27000
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f8d0,true)?)?;
    // 0x568cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x568d0: beq 0x568f8 <entry+0x10f8>  @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x568f8,false)?)?;
    // 0x568d4: ldrb r0, [r4, #0xfe8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,4072)?)?;
    // 0x568d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x568dc: bne 0x54d8c <entry+0xfffffffffffff58c> @ imm = #-0x1b58
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54d8c,false)?)?;
    // 0x568e0: ldr r0, [r4, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3372)?)?;
    // 0x568e4: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x568e8: beq 0x54d8c <entry+0xfffffffffffff58c> @ imm = #-0x1b64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54d8c,false)?)?;
    // 0x568ec: ldrb r0, [r4, #0xe84]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3716)?)?;
    // 0x568f0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x568f4: bne 0x54d8c <entry+0xfffffffffffff58c> @ imm = #-0x1b70
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54d8c,false)?)?;
    // 0x568f8: bl 0x2f8d0 <entry+0xfffffffffffda0d0> @ imm = #-0x27030
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f8d0,true)?)?;
    // 0x568fc: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x56900: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56904: bne 0x54d8c <entry+0xfffffffffffff58c> @ imm = #-0x1b80
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54d8c,false)?)?;
    // 0x56908: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x5690c: bl 0x3701c <entry+0xfffffffffffe181c> @ imm = #-0x1f8f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3701c,true)?)?;
    // 0x56910: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x56914: b 0x54d8c <entry+0xfffffffffffff58c> @ imm = #-0x1b90
    c.emit(arm32::branch(C::Al,c.at as u32,0x54d8c,false)?)?;
 c.check(0x56918)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56918:[u32;2]=[0xffffefde,0xffffefdf];
 for value in INLINE_DATA_56918{c.emit(value)?;}
 // Executable A32 block 0x56920..0x56930.
 c.check(0x56920)?;
    // 0x56920: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x56924: str r1, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,924)?)?;
    // 0x56928: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x5692c: b 0x55af8 <entry+0x2f8>   @ imm = #-0xe3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x55af8,false)?)?;
 c.check(0x56930)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56930:[u32;1]=[0xffffef63];
 for value in INLINE_DATA_56930{c.emit(value)?;}
 // Executable A32 block 0x56934..0x56944.
 c.check(0x56934)?;
    // 0x56934: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x56938: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x5693c: moveq r1, #15
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x56940: b 0x55af8 <entry+0x2f8>   @ imm = #-0xe50
    c.emit(arm32::branch(C::Al,c.at as u32,0x55af8,false)?)?;
 c.check(0x56944)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56944:[u32;2]=[0xffffef4e,0x000ee4ac];
 for value in INLINE_DATA_56944{c.emit(value)?;}
 // Executable A32 block 0x5694c..0x56978.
 c.check(0x5694c)?;
    // 0x5694c: cmp r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,127)?)?;
    // 0x56950: bgt 0x569ac <entry+0x11ac>  @ imm = #0x54
    c.emit(arm32::branch(C::Gt,c.at as u32,0x569ac,false)?)?;
    // 0x56954: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x56958: bgt 0x56b48 <entry+0x1348>  @ imm = #0x1e8
    c.emit(arm32::branch(C::Gt,c.at as u32,0x56b48,false)?)?;
    // 0x5695c: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x56960: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // 0x56964: bhi 0x56bd0 <entry+0x13d0>  @ imm = #0x264
    c.emit(arm32::branch(C::Hi,c.at as u32,0x56bd0,false)?)?;
    // 0x56968: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x5696c: add r2, pc, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,8)?)?;
    // 0x56970: mov r0, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,30)?)?;
    // 0x56974: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
 c.check(0x56978)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56978:[u32;9]=[0x000e2788,0x00056844,0x0005699c,0x00056bd0,0x000580a0,0x00056bd0,0x00056bd0,0x00056bd0,0x000580a8];
 for value in INLINE_DATA_56978{c.emit(value)?;}
 // Executable A32 block 0x5699c..0x56a60.
 c.check(0x5699c)?;
    // 0x5699c: mov r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,60)?)?;
    // 0x569a0: b 0x56844 <entry+0x1044>  @ imm = #-0x164
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x569a4: mov r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,100)?)?;
    // 0x569a8: b 0x56a00 <entry+0x1200>  @ imm = #0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x56a00,false)?)?;
    // 0x569ac: cmp r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1024)?)?;
    // 0x569b0: bge 0x56b68 <entry+0x1368>  @ imm = #0x1b0
    c.emit(arm32::branch(C::Ge,c.at as u32,0x56b68,false)?)?;
    // 0x569b4: cmp r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,128)?)?;
    // 0x569b8: beq 0x580b0 <entry+0x28b0>  @ imm = #0x16f0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x580b0,false)?)?;
    // 0x569bc: cmp r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    // 0x569c0: beq 0x56bd0 <entry+0x13d0>  @ imm = #0x208
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56bd0,false)?)?;
    // 0x569c4: cmp r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,512)?)?;
    // 0x569c8: bne 0x56bd0 <entry+0x13d0>  @ imm = #0x200
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56bd0,false)?)?;
    // 0x569cc: mov r0, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,156)?)?;
    // 0x569d0: b 0x56844 <entry+0x1044>  @ imm = #-0x194
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x569d4: mov r2, #196
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,196)?)?;
    // 0x569d8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x569dc: ldr r2, [r4, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x569e0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x569e4: beq 0x55af8 <entry+0x2f8>   @ imm = #-0xef4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55af8,false)?)?;
    // 0x569e8: ldrb r2, [r4, #0xe8a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,3722)?)?;
    // 0x569ec: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // 0x569f0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x569f4: movne r1, #13
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x569f8: b 0x55af8 <entry+0x2f8>   @ imm = #-0xf08
    c.emit(arm32::branch(C::Al,c.at as u32,0x55af8,false)?)?;
    // 0x569fc: mov r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,77)?)?;
    // 0x56a00: str r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,648)?)?;
    // 0x56a04: b 0x56a38 <entry+0x1238>  @ imm = #0x2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x56a38,false)?)?;
    // 0x56a08: mov r1, #118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,118)?)?;
    // 0x56a0c: b 0x56a14 <entry+0x1214>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x56a14,false)?)?;
    // 0x56a10: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // 0x56a14: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x56a18: str r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,648)?)?;
    // 0x56a1c: str r2, [r4, #0x56c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1388)?)?;
    // 0x56a20: b 0x56a38 <entry+0x1238>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x56a38,false)?)?;
    // 0x56a24: mov r1, #66
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,66)?)?;
    // 0x56a28: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x56a2c: str r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,648)?)?;
    // 0x56a30: strb r2, [r4, #0x55b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1371)?)?;
    // 0x56a34: strb r2, [r4, #0x55c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1372)?)?;
    // 0x56a38: cmp r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1024)?)?;
    // 0x56a3c: bge 0x56a8c <entry+0x128c>  @ imm = #0x48
    c.emit(arm32::branch(C::Ge,c.at as u32,0x56a8c,false)?)?;
    // 0x56a40: cmp r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,31)?)?;
    // 0x56a44: bgt 0x56ab8 <entry+0x12b8>  @ imm = #0x6c
    c.emit(arm32::branch(C::Gt,c.at as u32,0x56ab8,false)?)?;
    // 0x56a48: sub r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R0,1)?)?;
    // 0x56a4c: cmp r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,7)?)?;
    // 0x56a50: bhi 0x56bc0 <entry+0x13c0>  @ imm = #0x168
    c.emit(arm32::branch(C::Hi,c.at as u32,0x56bc0,false)?)?;
    // 0x56a54: lsl r0, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x56a58: add r2, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,4)?)?;
    // 0x56a5c: ldr pc, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R2,Shift::Lsl,0)?)?;
 c.check(0x56a60)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56A60:[u32;9]=[0xffffec98,0x00056a84,0x00056bd8,0x00056b40,0x00056bf0,0x00056b40,0x00056b40,0x00056b40,0x00056bf8];
 for value in INLINE_DATA_56A60{c.emit(value)?;}
 // Executable A32 block 0x56a84..0x56c6c.
 c.check(0x56a84)?;
    // 0x56a84: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x56a88: b 0x56c44 <entry+0x1444>  @ imm = #0x1b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56a8c: cmp r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32768)?)?;
    // 0x56a90: bge 0x56ae0 <entry+0x12e0>  @ imm = #0x48
    c.emit(arm32::branch(C::Ge,c.at as u32,0x56ae0,false)?)?;
    // 0x56a94: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x56a98: blt 0x56b04 <entry+0x1304>  @ imm = #0x64
    c.emit(arm32::branch(C::Lt,c.at as u32,0x56b04,false)?)?;
    // 0x56a9c: beq 0x56c00 <entry+0x1400>  @ imm = #0x15c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c00,false)?)?;
    // 0x56aa0: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x56aa4: beq 0x56c08 <entry+0x1408>  @ imm = #0x15c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c08,false)?)?;
    // 0x56aa8: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x56aac: bne 0x56b40 <entry+0x1340>  @ imm = #0x8c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56b40,false)?)?;
    // 0x56ab0: mov r0, #50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,50)?)?;
    // 0x56ab4: b 0x56c44 <entry+0x1444>  @ imm = #0x188
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56ab8: cmp r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,127)?)?;
    // 0x56abc: ble 0x56b1c <entry+0x131c>  @ imm = #0x58
    c.emit(arm32::branch(C::Le,c.at as u32,0x56b1c,false)?)?;
    // 0x56ac0: cmp r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,128)?)?;
    // 0x56ac4: beq 0x56c10 <entry+0x1410>  @ imm = #0x144
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c10,false)?)?;
    // 0x56ac8: cmp r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    // 0x56acc: beq 0x56c18 <entry+0x1418>  @ imm = #0x144
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c18,false)?)?;
    // 0x56ad0: cmp r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,512)?)?;
    // 0x56ad4: bne 0x56b40 <entry+0x1340>  @ imm = #0x64
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56b40,false)?)?;
    // 0x56ad8: mov r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,31)?)?;
    // 0x56adc: b 0x56c44 <entry+0x1444>  @ imm = #0x160
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56ae0: cmp r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,131072)?)?;
    // 0x56ae4: blt 0x56b34 <entry+0x1334>  @ imm = #0x48
    c.emit(arm32::branch(C::Lt,c.at as u32,0x56b34,false)?)?;
    // 0x56ae8: beq 0x56c20 <entry+0x1420>  @ imm = #0x130
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c20,false)?)?;
    // 0x56aec: cmp r0, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,262144)?)?;
    // 0x56af0: beq 0x56c28 <entry+0x1428>  @ imm = #0x130
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c28,false)?)?;
    // 0x56af4: cmp r0, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,524288)?)?;
    // 0x56af8: bne 0x56b40 <entry+0x1340>  @ imm = #0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56b40,false)?)?;
    // 0x56afc: mov r0, #62
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,62)?)?;
    // 0x56b00: b 0x56c44 <entry+0x1444>  @ imm = #0x13c
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56b04: cmp r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1024)?)?;
    // 0x56b08: beq 0x56c30 <entry+0x1430>  @ imm = #0x120
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c30,false)?)?;
    // 0x56b0c: cmp r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2048)?)?;
    // 0x56b10: bne 0x56b40 <entry+0x1340>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56b40,false)?)?;
    // 0x56b14: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x56b18: b 0x56c44 <entry+0x1444>  @ imm = #0x124
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56b1c: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x56b20: beq 0x56c38 <entry+0x1438>  @ imm = #0x110
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c38,false)?)?;
    // 0x56b24: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x56b28: bne 0x56b40 <entry+0x1340>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56b40,false)?)?;
    // 0x56b2c: mov r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,21)?)?;
    // 0x56b30: b 0x56c44 <entry+0x1444>  @ imm = #0x10c
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56b34: cmp r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32768)?)?;
    // 0x56b38: beq 0x56c40 <entry+0x1440>  @ imm = #0x100
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56c40,false)?)?;
    // 0x56b3c: cmp r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,65536)?)?;
    // 0x56b40: mov r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // 0x56b44: b 0x56c44 <entry+0x1444>  @ imm = #0xf8
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56b48: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x56b4c: beq 0x580b8 <entry+0x28b8>  @ imm = #0x1564
    c.emit(arm32::branch(C::Eq,c.at as u32,0x580b8,false)?)?;
    // 0x56b50: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x56b54: beq 0x580c0 <entry+0x28c0>  @ imm = #0x1564
    c.emit(arm32::branch(C::Eq,c.at as u32,0x580c0,false)?)?;
    // 0x56b58: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x56b5c: bne 0x56bd0 <entry+0x13d0>  @ imm = #0x6c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56bd0,false)?)?;
    // 0x56b60: mov r0, #123
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,123)?)?;
    // 0x56b64: b 0x56844 <entry+0x1044>  @ imm = #-0x328
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x56b68: beq 0x580c8 <entry+0x28c8>  @ imm = #0x1558
    c.emit(arm32::branch(C::Eq,c.at as u32,0x580c8,false)?)?;
    // 0x56b6c: cmp r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2048)?)?;
    // 0x56b70: beq 0x580d0 <entry+0x28d0>  @ imm = #0x1558
    c.emit(arm32::branch(C::Eq,c.at as u32,0x580d0,false)?)?;
    // 0x56b74: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x56b78: bne 0x56bd0 <entry+0x13d0>  @ imm = #0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56bd0,false)?)?;
    // 0x56b7c: mov r0, #186
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,186)?)?;
    // 0x56b80: b 0x56844 <entry+0x1044>  @ imm = #-0x344
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x56b84: mov r0, #5312
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5312)?)?;
    // 0x56b88: ldrb r1, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x56b8c: ldrb r0, [r4, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3568)?)?;
    // 0x56b90: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x56b94: bne 0x58048 <entry+0x2848>  @ imm = #0x14ac
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58048,false)?)?;
    // 0x56b98: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x56b9c: bne 0x58050 <entry+0x2850>  @ imm = #0x14ac
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58050,false)?)?;
    // 0x56ba0: ldr r0, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1392)?)?;
    // 0x56ba4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x56ba8: bne 0x580d8 <entry+0x28d8>  @ imm = #0x1528
    c.emit(arm32::branch(C::Ne,c.at as u32,0x580d8,false)?)?;
    // 0x56bac: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x56bb0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x56bb4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56bb8: strb r7, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3104)?)?;
    // 0x56bbc: b 0x580f0 <entry+0x28f0>  @ imm = #0x152c
    c.emit(arm32::branch(C::Al,c.at as u32,0x580f0,false)?)?;
    // 0x56bc0: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x56bc4: bne 0x56b40 <entry+0x1340>  @ imm = #-0x8c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56b40,false)?)?;
    // 0x56bc8: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x56bcc: b 0x56c44 <entry+0x1444>  @ imm = #0x70
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56bd0: mov r0, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,153)?)?;
    // 0x56bd4: b 0x56844 <entry+0x1044>  @ imm = #-0x398
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x56bd8: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x56bdc: cmp r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,100)?)?;
    // 0x56be0: str r0, [r4, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,652)?)?;
    // 0x56be4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56be8: strbne r0, [r4, #0x55d]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,1373)?)?;
    // 0x56bec: b 0x56c48 <entry+0x1448>  @ imm = #0x54
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c48,false)?)?;
    // 0x56bf0: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x56bf4: b 0x56c44 <entry+0x1444>  @ imm = #0x48
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56bf8: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x56bfc: b 0x56c44 <entry+0x1444>  @ imm = #0x40
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c00: mov r0, #41
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,41)?)?;
    // 0x56c04: b 0x56c44 <entry+0x1444>  @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c08: mov r0, #42
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,42)?)?;
    // 0x56c0c: b 0x56c44 <entry+0x1444>  @ imm = #0x30
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c10: mov r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // 0x56c14: b 0x56c44 <entry+0x1444>  @ imm = #0x28
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c18: mov r0, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,30)?)?;
    // 0x56c1c: b 0x56c44 <entry+0x1444>  @ imm = #0x20
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c20: mov r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,60)?)?;
    // 0x56c24: b 0x56c44 <entry+0x1444>  @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c28: mov r0, #61
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,61)?)?;
    // 0x56c2c: b 0x56c44 <entry+0x1444>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c30: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x56c34: b 0x56c44 <entry+0x1444>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c38: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x56c3c: b 0x56c44 <entry+0x1444>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x56c44,false)?)?;
    // 0x56c40: mov r0, #51
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,51)?)?;
    // 0x56c44: str r0, [r4, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,652)?)?;
    // 0x56c48: ldrb r0, [r4, #0x348]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,840)?)?;
    // 0x56c4c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x56c50: bne 0x56c78 <entry+0x1478>  @ imm = #0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56c78,false)?)?;
    // 0x56c54: ldr r0, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2652)?)?;
    // 0x56c58: cmp r0, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x56c5c: ble 0x56c78 <entry+0x1478>  @ imm = #0x14
    c.emit(arm32::branch(C::Le,c.at as u32,0x56c78,false)?)?;
    // 0x56c60: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56c64: strb r0, [r4, #0x328]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,808)?)?;
    // 0x56c68: b 0x56cb8 <entry+0x14b8>  @ imm = #0x48
    c.emit(arm32::branch(C::Al,c.at as u32,0x56cb8,false)?)?;
 c.check(0x56c6c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56C6C:[u32;3]=[0x02005004,0x000ee4dc,0x000ee51b];
 for value in INLINE_DATA_56C6C{c.emit(value)?;}
 // Executable A32 block 0x56c78..0x56e10.
 c.check(0x56c78)?;
    // 0x56c78: ldrb r0, [r4, #0xdd0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3536)?)?;
    // 0x56c7c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56c80: strb r0, [r4, #0x328]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,808)?)?;
    // 0x56c84: beq 0x56cb8 <entry+0x14b8>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56cb8,false)?)?;
    // 0x56c88: ldrb r0, [r4, #0xdd1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3537)?)?;
    // 0x56c8c: strb r0, [r4, #0x329]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,809)?)?;
    // 0x56c90: ldr r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
    // 0x56c94: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56c98: bne 0x56cb8 <entry+0x14b8>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56cb8,false)?)?;
    // 0x56c9c: ldr r2, [pc, #0x7dc]        @ 0x1c80 <entry+0x1c80>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2012)?)?;
    // 0x56ca0: cmp r1, #66
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,66)?)?;
    // 0x56ca4: mov r1, #210
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,210)?)?;
    // 0x56ca8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56cac: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56cb0: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x56cb4: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5550c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56cb8: ldr r1, [r4, #0x4f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1264)?)?;
    // 0x56cbc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56cc0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x56cc4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x56cc8: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x56ccc: strb r1, [r4, #0x560]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1376)?)?;
    // 0x56cd0: bne 0x56cf0 <entry+0x14f0>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56cf0,false)?)?;
    // 0x56cd4: ldr r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,648)?)?;
    // 0x56cd8: ldr r2, [pc, #0x7a0]        @ 0x1c80 <entry+0x1c80>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1952)?)?;
    // 0x56cdc: cmp r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,100)?)?;
    // 0x56ce0: mov r1, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,221)?)?;
    // 0x56ce4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56ce8: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x56cec: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x55544
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56cf0: ldrb r0, [r4, #0x3e6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,998)?)?;
    // 0x56cf4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x56cf8: bne 0x56d1c <entry+0x151c>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56d1c,false)?)?;
    // 0x56cfc: ldr r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,648)?)?;
    // 0x56d00: ldr r2, [pc, #0x778]        @ 0x1c80 <entry+0x1c80>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1912)?)?;
    // 0x56d04: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56d08: cmp r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,100)?)?;
    // 0x56d0c: mov r1, #226
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,226)?)?;
    // 0x56d10: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56d14: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x56d18: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x55570
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56d1c: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x56d20: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x56d24: cmpne r0, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x56d28: beq 0x56e14 <entry+0x1614>  @ imm = #0xe4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56e14,false)?)?;
    // 0x56d2c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x56d30: bne 0x56ec4 <entry+0x16c4>  @ imm = #0x18c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56ec4,false)?)?;
    // 0x56d34: ldrb r0, [r4, #0xe8a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3722)?)?;
    // 0x56d38: mov r1, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1000)?)?;
    // 0x56d3c: strb r0, [r4, #0x3e2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,994)?)?;
    // 0x56d40: str r1, [r4, #0x4d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1240)?)?;
    // 0x56d44: ldrb r1, [r4, #0xe8c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3724)?)?;
    // 0x56d48: strb r1, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,996)?)?;
    // 0x56d4c: mov r1, #223
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,223)?)?;
    // 0x56d50: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x56d54: ldrb r1, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x56d58: strb r1, [r4, #0x40b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1035)?)?;
    // 0x56d5c: ldrb r1, [r4, #0xe30]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3632)?)?;
    // 0x56d60: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x56d64: beq 0x56ea4 <entry+0x16a4>  @ imm = #0x138
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56ea4,false)?)?;
    // 0x56d68: ldrb r0, [r4, #0x561]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1377)?)?;
    // 0x56d6c: ldr r7, [pc, #0x70c]        @ 0x1c80 <entry+0x1c80>
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::PC,1804)?)?;
    // 0x56d70: mov r1, #201
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,201)?)?;
    // 0x56d74: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x56d78: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x56d7c: cmp r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,10)?)?;
    // 0x56d80: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56d84: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x56d88: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56d8c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x555e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56d90: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56d94: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x56d98: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x56d9c: strb r0, [r4, #0x559]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1369)?)?;
    // 0x56da0: strb r0, [r4, #0x55a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1370)?)?;
    // 0x56da4: ldrb r0, [r4, #0x561]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1377)?)?;
    // 0x56da8: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x56dac: addhi r1, r0, #251
    c.emit(arm32::data_immediate(C::Hi,Op::Add,R::R1,R::R0,251)?)?;
    // 0x56db0: strb r1, [r4, #0x558]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1368)?)?;
    // 0x56db4: mov r1, #207
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,207)?)?;
    // 0x56db8: ldrb r0, [r4, #0x3e2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,994)?)?;
    // 0x56dbc: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x56dc0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56dc4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56dc8: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56dcc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x55624
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56dd0: ldrb r0, [r4, #0x40e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1038)?)?;
    // 0x56dd4: mov r1, #3536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3536)?)?;
    // 0x56dd8: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x56ddc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56de0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56de4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56de8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x55640
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56dec: ldrb r0, [r4, #0x558]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1368)?)?;
    // 0x56df0: mov r1, #209
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,209)?)?;
    // 0x56df4: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x56df8: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x56dfc: cmp r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,13)?)?;
    // 0x56e00: movlo r6, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x56e04: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x56e08: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x55660
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56e0c: b 0x56ec4 <entry+0x16c4>  @ imm = #0xb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x56ec4,false)?)?;
 c.check(0x56e10)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56E10:[u32;1]=[0x000ee536];
 for value in INLINE_DATA_56E10{c.emit(value)?;}
 // Executable A32 block 0x56e14..0x56fb0.
 c.check(0x56e14)?;
    // 0x56e14: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56e18: mov r1, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1000)?)?;
    // 0x56e1c: ldr r2, [pc, #0x65c]        @ 0x1c80 <entry+0x1c80>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1628)?)?;
    // 0x56e20: strb r0, [r4, #0x40e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1038)?)?;
    // 0x56e24: str r1, [r4, #0x4d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1240)?)?;
    // 0x56e28: ldr r1, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2732)?)?;
    // 0x56e2c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x56e30: ldrbne r1, [r4, #0xdfa]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::new(4)?,3578)?)?;
    // 0x56e34: strbne r1, [r4, #0x561]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(4)?,1377)?)?;
    // 0x56e38: ldrb r1, [r4, #0xe30]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3632)?)?;
    // 0x56e3c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x56e40: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x56e44: str r1, [r4, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1236)?)?;
    // 0x56e48: ldrb r1, [r4, #0x3e6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,998)?)?;
    // 0x56e4c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x56e50: mov r1, #239
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,239)?)?;
    // 0x56e54: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56e58: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x56e5c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x556b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56e60: ldr r0, [r4, #0xe34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3636)?)?;
    // 0x56e64: str r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,908)?)?;
    // 0x56e68: ldr r0, [r4, #0xe38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3640)?)?;
    // 0x56e6c: str r0, [r4, #0x390]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,912)?)?;
    // 0x56e70: ldr r0, [r4, #0xe3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3644)?)?;
    // 0x56e74: str r0, [r4, #0x394]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,916)?)?;
    // 0x56e78: ldrb r0, [r4, #0xe8a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3722)?)?;
    // 0x56e7c: strb r0, [r4, #0x3e2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,994)?)?;
    // 0x56e80: mov r0, #239
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,239)?)?;
    // 0x56e84: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x56e88: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x56e8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56e90: strb r0, [r4, #0x41b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1051)?)?;
    // 0x56e94: ldrne r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,2744)?)?;
    // 0x56e98: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x56e9c: strbne r1, [r0, #0x4e0]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::R0,1248)?)?;
    // 0x56ea0: b 0x56ec4 <entry+0x16c4>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x56ec4,false)?)?;
    // 0x56ea4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x56ea8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x56eac: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56eb0: strb r1, [r4, #0x558]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1368)?)?;
    // 0x56eb4: moveq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x56eb8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x56ebc: strb r0, [r4, #0x559]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1369)?)?;
    // 0x56ec0: strb r1, [r4, #0x55a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1370)?)?;
    // 0x56ec4: ldrb r0, [r4, #0x419]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1049)?)?;
    // 0x56ec8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x56ecc: ldreq r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,2744)?)?;
    // 0x56ed0: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x56ed4: strbeq r1, [r0, #0x143]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::R0,323)?)?;
    // 0x56ed8: mov r0, #1012
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1012)?)?;
    // 0x56edc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x56ee0: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x56ee4: str r0, [r4, #0x420]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1056)?)?;
    // 0x56ee8: ldr r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
    // 0x56eec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56ef0: beq 0x56f34 <entry+0x1734>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56f34,false)?)?;
    // 0x56ef4: ldrb r0, [r4, #0xf80]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3968)?)?;
    // 0x56ef8: strb r0, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1156)?)?;
    // 0x56efc: ldr r0, [r4, #0xfbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4028)?)?;
    // 0x56f00: str r0, [r4, #0x488]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1160)?)?;
    // 0x56f04: ldr r0, [r4, #0xfc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4032)?)?;
    // 0x56f08: str r0, [r4, #0x48c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1164)?)?;
    // 0x56f0c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56f10: str r0, [r4, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1256)?)?;
    // 0x56f14: ldrb r0, [r4, #0x3dd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,989)?)?;
    // 0x56f18: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x56f1c: bne 0x56fb4 <entry+0x17b4>  @ imm = #0x90
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56fb4,false)?)?;
    // 0x56f20: ldrb r0, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1156)?)?;
    // 0x56f24: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x56f28: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56f2c: strbeq r0, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::new(4)?,1156)?)?;
    // 0x56f30: b 0x56fb4 <entry+0x17b4>  @ imm = #0x7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x56fb4,false)?)?;
    // 0x56f34: ldr r0, [r4, #0xdbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3516)?)?;
    // 0x56f38: add r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1)?)?;
    // 0x56f3c: str r1, [r4, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1256)?)?;
    // 0x56f40: ldr r2, [r4, #0xdc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,3520)?)?;
    // 0x56f44: str r2, [r4, #0x4ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1260)?)?;
    // 0x56f48: ldr r1, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2668)?)?;
    // 0x56f4c: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x56f50: strhi r1, [r4, #0x4ec]
    c.emit(arm32::transfer_word(C::Hi,false,R::R1,R::new(4)?,1260)?)?;
    // 0x56f54: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56f58: beq 0x57164 <entry+0x1964>  @ imm = #0x204
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57164,false)?)?;
    // 0x56f5c: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x56f60: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x56f64: strb r0, [r4, #0xf80]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3968)?)?;
    // 0x56f68: strb r0, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1156)?)?;
    // 0x56f6c: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x56f70: str r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,52)?)?;
    // 0x56f74: ldrb r0, [r4, #0x3dd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,989)?)?;
    // 0x56f78: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x56f7c: bne 0x56fb4 <entry+0x17b4>  @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56fb4,false)?)?;
    // 0x56f80: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56f84: strb r0, [r4, #0x3dd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,989)?)?;
    // 0x56f88: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x56f8c: ldrb r1, [r4, #0x328]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,808)?)?;
    // 0x56f90: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x56f94: ldrne r1, [r8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(8)?,0)?)?;
    // 0x56f98: cmpne r1, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x56f9c: beq 0x56fa8 <entry+0x17a8>  @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56fa8,false)?)?;
    // 0x56fa0: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x56fa4: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56fa8: strb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,990)?)?;
    // 0x56fac: b 0x56f14 <entry+0x1714>  @ imm = #-0xa0
    c.emit(arm32::branch(C::Al,c.at as u32,0x56f14,false)?)?;
 c.check(0x56fb0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56FB0:[u32;1]=[0x0a0a0a0a];
 for value in INLINE_DATA_56FB0{c.emit(value)?;}
 // Executable A32 block 0x56fb4..0x57000.
 c.check(0x56fb4)?;
    // 0x56fb4: ldr r0, [r4, #0x4b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1204)?)?;
    // 0x56fb8: mov r2, #812
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,812)?)?;
    // 0x56fbc: mul r1, r0, r2
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::R2)?)?;
    // 0x56fc0: cmp r0, #3600
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3600)?)?;
    // 0x56fc4: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x56fc8: str r1, [r4, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2724)?)?;
    // 0x56fcc: bhi 0x57018 <entry+0x1818>  @ imm = #0x44
    c.emit(arm32::branch(C::Hi,c.at as u32,0x57018,false)?)?;
    // 0x56fd0: add r0, r1, #576
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,576)?)?;
    // 0x56fd4: add r0, r0, #999424
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,999424)?)?;
    // 0x56fd8: str r0, [r4, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2724)?)?;
    // 0x56fdc: ldr r2, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2744)?)?;
    // 0x56fe0: ldrb r2, [r2, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,325)?)?;
    // 0x56fe4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x56fe8: beq 0x57018 <entry+0x1818>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57018,false)?)?;
    // 0x56fec: ldr r2, [r4, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,672)?)?;
    // 0x56ff0: ldr r3, [r4, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,676)?)?;
    // 0x56ff4: ldr r7, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,716)?)?;
    // 0x56ff8: ldr r6, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,720)?)?;
    // 0x56ffc: mul r5, r6, r7
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::new(6)?,R::new(7)?)?)?;
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
