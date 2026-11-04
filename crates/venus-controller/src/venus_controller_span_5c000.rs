// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x5c000,0x5d000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x5c000;
pub const END:usize=0x5d000;
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
fn emit_umull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 UMULL operands".into())}
 Ok(((cond as u32)<<28)|0x0080_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x5c000..0x5c1b4.
 c.check(0x5c000)?;
    // 0x5c000: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c004: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x5c008: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d37c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c00c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c010: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c014: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d388
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c018: ldr r1, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,20)?)?;
    // 0x5c01c: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5c020: ldr r3, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,16)?)?;
    // 0x5c024: ldr r2, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,24)?)?;
    // 0x5c028: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x5c02c: ldr r1, [pc, #0xfd8]        @ 0x780c <entry+0x780c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4056)?)?;
    // 0x5c030: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x5c034: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c038: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d3ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c03c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c040: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c044: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d3b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c048: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c04c: orr r0, r0, r7, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,4,false)?)?;
    // 0x5c050: add r5, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5c054: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5c058: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5c05c: blo 0x5c134 <entry+0x6934>  @ imm = #0xd0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5c134,false)?)?;
    // 0x5c060: ldr r1, [r10, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,36)?)?;
    // 0x5c064: ldr r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,28)?)?;
    // 0x5c068: ldr r3, [r10, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,32)?)?;
    // 0x5c06c: ldr r2, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,40)?)?;
    // 0x5c070: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x5c074: ldr r1, [pc, #0xfd4]        @ 0x7850 <entry+0x7850>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4052)?)?;
    // 0x5c078: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x5c07c: b 0x5c120 <entry+0x6920>  @ imm = #0x9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c120,false)?)?;
    // 0x5c080: ldr r1, [pc, #0xfcc]        @ 0x7854 <entry+0x7854>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4044)?)?;
    // 0x5c084: ldr r2, [pc, #0x788]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1928)?)?;
    // 0x5c088: mov r3, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,25)?)?;
    // 0x5c08c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c090: orr r3, r3, #7424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7424)?)?;
    // 0x5c094: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d408
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c098: ldr r4, [pc, #0x7a4]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,1956)?)?;
    // 0x5c09c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c0a0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c0a4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d418
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c0a8: mov r1, #26
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,26)?)?;
    // 0x5c0ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c0b0: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5c0b4: orr r1, r1, #7424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7424)?)?;
    // 0x5c0b8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5a910
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c0bc: ldr r7, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,8)?)?;
    // 0x5c0c0: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // 0x5c0c4: ldr r1, [pc, #0xf8c]        @ 0x7858 <entry+0x7858>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3980)?)?;
    // 0x5c0c8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c0cc: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x5c0d0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d444
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c0d4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c0d8: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c0dc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d450
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c0e0: ldr r2, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,12)?)?;
    // 0x5c0e4: ldr r3, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,16)?)?;
    // 0x5c0e8: ldr r1, [pc, #0xfe8]        @ 0x78d8 <entry+0x78d8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4072)?)?;
    // 0x5c0ec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c0f0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d464
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c0f4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c0f8: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c0fc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d470
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c100: add r0, r5, r7, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x5c104: add r5, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,8)?)?;
    // 0x5c108: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5c10c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5c110: blo 0x5c134 <entry+0x6934>  @ imm = #0x1c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5c134,false)?)?;
    // 0x5c114: ldr r2, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,20)?)?;
    // 0x5c118: ldr r3, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,24)?)?;
    // 0x5c11c: ldr r1, [pc, #0xfb8]        @ 0x78dc <entry+0x78dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4024)?)?;
    // 0x5c120: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c124: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d498
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c128: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c12c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c130: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d4a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c134: mov r4, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,20480)?)?;
    // 0x5c138: orr r4, r4, #35651584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,35651584)?)?;
    // 0x5c13c: mov r10, r5
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(5)?))?;
    // 0x5c140: b 0x5e4c4 <entry+0x8cc4>  @ imm = #0x237c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4c4,false)?)?;
    // 0x5c144: cmp r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,255)?)?;
    // 0x5c148: bne 0x5c798 <entry+0x6f98>  @ imm = #0x648
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c798,false)?)?;
    // 0x5c14c: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5c150: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x59564
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5c154: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5c158: str r0, [r9, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,300)?)?;
    // 0x5c15c: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5c160: orr r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // 0x5c164: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5c168: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5c16c: str r0, [r9, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,364)?)?;
    // 0x5c170: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5c174: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x59548
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5c178: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5c17c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5c180: str r0, [r9, #0xd8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3468)?)?;
    // 0x5c184: ldr r0, [sp, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,124)?)?;
    // 0x5c188: strb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x5c18c: ldr r2, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,8)?)?;
    // 0x5c190: b 0x5c798 <entry+0x6f98>  @ imm = #0x600
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c798,false)?)?;
    // 0x5c194: mvn r0, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,33554432)?)?;
    // 0x5c198: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // 0x5c19c: add r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5c1a0: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x5c1a4: bhi 0x5c8d4 <entry+0x70d4>  @ imm = #0x728
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5c8d4,false)?)?;
    // 0x5c1a8: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x5c1ac: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x5c1b0: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x5c1b4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5C1B4:[u32;6]=[0x0005c7f0,0x0005c1cc,0x0005c1cc,0x0005c904,0x0005c818,0x0005c848];
 for value in INLINE_DATA_5C1B4{c.emit(value)?;}
 // Executable A32 block 0x5c1cc..0x5c810.
 c.check(0x5c1cc)?;
    // 0x5c1cc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5c1d0: b 0x5c81c <entry+0x701c>  @ imm = #0x644
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c81c,false)?)?;
    // 0x5c1d4: ldr r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,16)?)?;
    // 0x5c1d8: mov r4, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,20480)?)?;
    // 0x5c1dc: orr r4, r4, #35651584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,35651584)?)?;
    // 0x5c1e0: b 0x5e4c4 <entry+0x8cc4>  @ imm = #0x22dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4c4,false)?)?;
    // 0x5c1e4: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5c1e8: ldr r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
    // 0x5c1ec: mov r10, r3
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(3)?))?;
    // 0x5c1f0: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5c1f4: ldr r3, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x5c1f8: ldr r1, [pc, #0xfc0]        @ 0x79c0 <entry+0x79c0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4032)?)?;
    // 0x5c1fc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c200: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d574
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c204: b 0x5e4b8 <entry+0x8cb8>  @ imm = #0x22ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b8,false)?)?;
    // 0x5c208: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5c20c: ldr r5, [pc, #0x630]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,1584)?)?;
    // 0x5c210: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5c214: str r0, [r9, #0xd74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3444)?)?;
    // 0x5c218: ldr r1, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,12)?)?;
    // 0x5c21c: orr r2, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5c220: str r1, [r9, #0xd78]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3448)?)?;
    // 0x5c224: tst r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,1)?)?;
    // 0x5c228: beq 0x5c290 <entry+0x6a90>  @ imm = #0x60
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c290,false)?)?;
    // 0x5c22c: bic r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,1)?)?;
    // 0x5c230: bic r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,1)?)?;
    // 0x5c234: ldr r2, [pc, #0x5d8]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1496)?)?;
    // 0x5c238: mov r3, #2336
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2336)?)?;
    // 0x5c23c: str r0, [r9, #0xd74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3444)?)?;
    // 0x5c240: str r1, [r9, #0xd78]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3448)?)?;
    // 0x5c244: ldr r1, [pc, #0xfdc]        @ 0x7a28 <entry+0x7a28>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4060)?)?;
    // 0x5c248: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c24c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5c250: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x5c254: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5c258: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c25c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d5d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c260: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c264: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5c268: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d5dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c26c: mov r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,8)?)?;
    // 0x5c270: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x5c274: b 0x5c290 <entry+0x6a90>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c290,false)?)?;
    // 0x5c278: ldr r2, [pc, #0x680]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1664)?)?;
    // 0x5c27c: mov r1, #292
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,292)?)?;
    // 0x5c280: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c284: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5c288: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x5c28c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5aae4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c290: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5c294: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5c298: ldr r1, [pc, #0xf8c]        @ 0x7a2c <entry+0x7a2c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3980)?)?;
    // 0x5c29c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5c2a0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c2a4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d618
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c2a8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c2ac: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5c2b0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d624
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c2b4: mov r10, r7
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(7)?))?;
    // 0x5c2b8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x5c2bc: beq 0x5e4c4 <entry+0x8cc4>  @ imm = #0x2200
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5e4c4,false)?)?;
    // 0x5c2c0: b 0x5e510 <entry+0x8d10>  @ imm = #0x2248
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
    // 0x5c2c4: ldrh r0, [r10, #10]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(10)?,10)?)?;
    // 0x5c2c8: str r0, [r9, #0xd88]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3464)?)?;
    // 0x5c2cc: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5c2d0: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x596e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5c2d4: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5c2d8: str r0, [r9, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,332)?)?;
    // 0x5c2dc: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5c2e0: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // 0x5c2e4: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5c2e8: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5c2ec: str r0, [r9, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,352)?)?;
    // 0x5c2f0: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5c2f4: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x596c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5c2f8: ldr r4, [pc, #0x544]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,1348)?)?;
    // 0x5c2fc: b 0x5c314 <entry+0x6b14>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c314,false)?)?;
    // 0x5c300: ldr r2, [pc, #0x5f8]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1528)?)?;
    // 0x5c304: mov r1, #452
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,452)?)?;
    // 0x5c308: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c30c: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x5c310: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5ab68
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c314: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5c318: ldr r1, [pc, #0xfb8]        @ 0x7ad8 <entry+0x7ad8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4024)?)?;
    // 0x5c31c: b 0x5c8b0 <entry+0x70b0>  @ imm = #0x58c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c8b0,false)?)?;
    // 0x5c320: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5c324: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5c328: ldr r1, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,100)?)?;
    // 0x5c32c: ldr r3, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,8)?)?;
    // 0x5c330: ldr r4, [pc, #0x50c]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,1292)?)?;
    // 0x5c334: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5c338: ldr r1, [pc, #0xf9c]        @ 0x7adc <entry+0x7adc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3996)?)?;
    // 0x5c33c: b 0x5c8b0 <entry+0x70b0>  @ imm = #0x56c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c8b0,false)?)?;
    // 0x5c340: ldr r2, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-56)?)?;
    // 0x5c344: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5c348: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5c34c: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x5c350: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5c354: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x5c358: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5c35c: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5c360: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5c364: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5c368: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x319e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5c36c: ldr r1, [pc, #0xf6c]        @ 0x7ae0 <entry+0x7ae0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3948)?)?;
    // 0x5c370: ldr r2, [pc, #0x49c]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1180)?)?;
    // 0x5c374: mov r3, #226
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,226)?)?;
    // 0x5c378: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c37c: orr r3, r3, #6656
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6656)?)?;
    // 0x5c380: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d6f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c384: ldr r1, [pc, #0x4b8]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1208)?)?;
    // 0x5c388: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c38c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d700
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c390: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5c394: ldr r1, [pc, #0xfe8]        @ 0x7b84 <entry+0x7b84>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4072)?)?;
    // 0x5c398: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0x2110
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
    // 0x5c39c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5c3a0: strb r0, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3568)?)?;
    // 0x5c3a4: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5c3a8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // 0x5c3ac: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5c3b0: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5c3b4: str r0, [r9, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,372)?)?;
    // 0x5c3b8: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5c3bc: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x59790
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5c3c0: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5c3c4: ldr r1, [pc, #0xfbc]        @ 0x7b88 <entry+0x7b88>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4028)?)?;
    // 0x5c3c8: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0x20e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
    // 0x5c3cc: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5c3d0: ldr r4, [pc, #0x46c]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,1132)?)?;
    // 0x5c3d4: str r0, [r9, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1524)?)?;
    // 0x5c3d8: b 0x5c8a8 <entry+0x70a8>  @ imm = #0x4c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c8a8,false)?)?;
    // 0x5c3dc: ldr r1, [pc, #0xfa8]        @ 0x7b8c <entry+0x7b8c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4008)?)?;
    // 0x5c3e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c3e4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d758
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c3e8: ldr r4, [pc, #0x454]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,1108)?)?;
    // 0x5c3ec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c3f0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c3f4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d768
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c3f8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c3fc: mov r1, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    // 0x5c400: b 0x5c89c <entry+0x709c>  @ imm = #0x494
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c89c,false)?)?;
    // 0x5c404: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5c408: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5c40c: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x5c410: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5c414: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x5c418: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5c41c: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5c420: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5c424: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5c428: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x31aa0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5c42c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5c430: ldr r1, [pc, #0xfbc]        @ 0x7bf4 <entry+0x7bf4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4028)?)?;
    // 0x5c434: ldr r2, [pc, #0x3d8]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,984)?)?;
    // 0x5c438: mov r3, #3104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3104)?)?;
    // 0x5c43c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5c440: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5c444: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c448: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d7bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c44c: ldr r4, [pc, #0x3f0]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,1008)?)?;
    // 0x5c450: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c454: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c458: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d7cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c45c: b 0x5c8a8 <entry+0x70a8>  @ imm = #0x444
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c8a8,false)?)?;
    // 0x5c460: ldr r1, [pc, #0xfd4]        @ 0x7c3c <entry+0x7c3c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4052)?)?;
    // 0x5c464: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c468: str r2, [r9, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,1592)?)?;
    // 0x5c46c: str r2, [r9, #0x63c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,1596)?)?;
    // 0x5c470: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d7e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c474: b 0x5c7a8 <entry+0x6fa8>  @ imm = #0x32c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c7a8,false)?)?;
    // 0x5c478: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5c47c: str r1, [r9, #0xd50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3408)?)?;
    // 0x5c480: strb r1, [r9, #0xe62]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,3682)?)?;
    // 0x5c484: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x5c488: blo 0x5c4a8 <entry+0x6ca8>  @ imm = #0x18
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5c4a8,false)?)?;
    // 0x5c48c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x5c490: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5c494: strb r2, [r9, #0xe83]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,3715)?)?;
    // 0x5c498: ldr r3, [pc, #0xff8]        @ 0x7c98 <entry+0x7c98>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,4088)?)?;
    // 0x5c49c: mvneq r3, #16384
    c.emit(arm32::data_immediate(C::Eq,Op::Mvn,R::new(3)?,R::R0,16384)?)?;
    // 0x5c4a0: add r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5c4a4: b 0x5c4b0 <entry+0x6cb0>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c4b0,false)?)?;
    // 0x5c4a8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5c4ac: strb r2, [r9, #0xe83]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,3715)?)?;
    // 0x5c4b0: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x5c4b4: mvn r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,2147483648)?)?;
    // 0x5c4b8: strb r2, [r9, #0xe84]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,3716)?)?;
    // 0x5c4bc: moveq r1, #25
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,25)?)?;
    // 0x5c4c0: cmp r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,10)?)?;
    // 0x5c4c4: sublo r1, r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x5c4c8: cmn r1, #-2147483647
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,2147483649)?)?;
    // 0x5c4cc: str r1, [r9, #0xdb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3508)?)?;
    // 0x5c4d0: str r1, [r9, #0xdb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3512)?)?;
    // 0x5c4d4: bne 0x5c540 <entry+0x6d40>  @ imm = #0x64
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c540,false)?)?;
    // 0x5c4d8: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5c4dc: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5c4e0: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x5c4e4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5c4e8: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x5c4ec: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5c4f0: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5c4f4: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5c4f8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5c4fc: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x31b74
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5c500: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5c504: ldr r1, [pc, #0xfe0]        @ 0x7cec <entry+0x7cec>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4064)?)?;
    // 0x5c508: ldr r2, [pc, #0x304]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,772)?)?;
    // 0x5c50c: mov r3, #148
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,148)?)?;
    // 0x5c510: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7168)?)?;
    // 0x5c514: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5c518: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c51c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d890
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c520: ldr r1, [pc, #0x31c]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,796)?)?;
    // 0x5c524: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c528: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d89c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c52c: mov r1, #149
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,149)?)?;
    // 0x5c530: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c534: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x5c538: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x5c53c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5ad94
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c540: ldr r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // 0x5c544: ldr r1, [r9, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1392)?)?;
    // 0x5c548: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5c54c: bl 0x58130 <entry+0x2930>  @ imm = #-0x4424
    c.emit(arm32::branch(C::Al,c.at as u32,0x58130,true)?)?;
    // 0x5c550: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5c554: bl 0x58214 <entry+0x2a14>  @ imm = #-0x4348
    c.emit(arm32::branch(C::Al,c.at as u32,0x58214,true)?)?;
    // 0x5c558: ldr r4, [pc, #0x2e4]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,740)?)?;
    // 0x5c55c: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5c560: ldr r1, [pc, #0xff0]        @ 0x7d58 <entry+0x7d58>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4080)?)?;
    // 0x5c564: b 0x5c8b0 <entry+0x70b0>  @ imm = #0x344
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c8b0,false)?)?;
    // 0x5c568: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c56c: strb r0, [r9, #0xdc4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3524)?)?;
    // 0x5c570: b 0x5c5b8 <entry+0x6db8>  @ imm = #0x40
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c5b8,false)?)?;
    // 0x5c574: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5c578: strb r0, [r9, #0xdc4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3524)?)?;
    // 0x5c57c: b 0x5c5b8 <entry+0x6db8>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c5b8,false)?)?;
    // 0x5c580: ldr r1, [pc, #0xff4]        @ 0x7d7c <entry+0x7d7c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x5c584: ldr r2, [pc, #0x288]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,648)?)?;
    // 0x5c588: mov r3, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,98)?)?;
    // 0x5c58c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c590: orr r3, r3, #6912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6912)?)?;
    // 0x5c594: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d908
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c598: ldr r1, [pc, #0x2a4]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,676)?)?;
    // 0x5c59c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c5a0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d914
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c5a4: ldr r2, [pc, #0x354]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,852)?)?;
    // 0x5c5a8: mov r1, #99
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,99)?)?;
    // 0x5c5ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c5b0: orr r1, r1, #6912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6912)?)?;
    // 0x5c5b4: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5ae0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c5b8: ldr r3, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,8)?)?;
    // 0x5c5bc: ldr r1, [pc, #0xfe4]        @ 0x7da8 <entry+0x7da8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4068)?)?;
    // 0x5c5c0: str r3, [r9, #0xdc8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,3528)?)?;
    // 0x5c5c4: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5c5c8: str r0, [r9, #0xdcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3532)?)?;
    // 0x5c5cc: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // 0x5c5d0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5c5d4: b 0x5c77c <entry+0x6f7c>  @ imm = #0x1a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c77c,false)?)?;
    // 0x5c5d8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x5c5dc: str r1, [r9, #0xd94]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3476)?)?;
    // 0x5c5e0: str r0, [r9, #0xd98]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3480)?)?;
    // 0x5c5e4: b 0x5c62c <entry+0x6e2c>  @ imm = #0x40
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c62c,false)?)?;
    // 0x5c5e8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c5ec: str r0, [r9, #0xd94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3476)?)?;
    // 0x5c5f0: b 0x5c62c <entry+0x6e2c>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c62c,false)?)?;
    // 0x5c5f4: ldr r1, [pc, #0xff4]        @ 0x7df0 <entry+0x7df0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x5c5f8: ldr r2, [pc, #0x214]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,532)?)?;
    // 0x5c5fc: mov r3, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,648)?)?;
    // 0x5c600: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c604: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x5c608: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d97c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c60c: ldr r1, [pc, #0x230]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,560)?)?;
    // 0x5c610: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c614: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d988
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c618: ldr r2, [pc, #0x2e0]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,736)?)?;
    // 0x5c61c: mov r1, #137
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,137)?)?;
    // 0x5c620: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c624: orr r1, r1, #6656
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6656)?)?;
    // 0x5c628: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5ae80
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c62c: ldr r0, [r9, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,280)?)?;
    // 0x5c630: ldr r3, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,8)?)?;
    // 0x5c634: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5c638: bne 0x5c654 <entry+0x6e54>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c654,false)?)?;
    // 0x5c63c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x5c640: mov r10, r6
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(6)?))?;
    // 0x5c644: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c648: moveq r3, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x5c64c: streq r0, [r9, #0xd98]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,3480)?)?;
    // 0x5c650: b 0x5c658 <entry+0x6e58>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c658,false)?)?;
    // 0x5c654: mov r10, r6
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(6)?))?;
    // 0x5c658: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5c65c: ldr r1, [pc, #0xfe0]        @ 0x7e44 <entry+0x7e44>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4064)?)?;
    // 0x5c660: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c664: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d9d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c668: b 0x5e480 <entry+0x8c80>  @ imm = #0x1e10
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e480,false)?)?;
    // 0x5c66c: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5c670: ldr r4, [pc, #0x1cc]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,460)?)?;
    // 0x5c674: str r0, [r9, #0x5f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1520)?)?;
    // 0x5c678: str r0, [r9, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1524)?)?;
    // 0x5c67c: b 0x5c70c <entry+0x6f0c>  @ imm = #0x88
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c70c,false)?)?;
    // 0x5c680: ldr r1, [pc, #0xd04]        @ 0x7b8c <entry+0x7b8c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3332)?)?;
    // 0x5c684: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c688: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2d9fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c68c: ldr r4, [pc, #0x1b0]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,432)?)?;
    // 0x5c690: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c694: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c698: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2da0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c69c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c6a0: mov r1, #79
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,79)?)?;
    // 0x5c6a4: ldr r2, [pc, #0x254]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,596)?)?;
    // 0x5c6a8: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x5c6ac: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5af04
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c6b0: b 0x5c70c <entry+0x6f0c>  @ imm = #0x54
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c70c,false)?)?;
    // 0x5c6b4: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5c6b8: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5c6bc: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x5c6c0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5c6c4: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x5c6c8: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5c6cc: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5c6d0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5c6d4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5c6d8: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x31d50
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5c6dc: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5c6e0: ldr r1, [pc, #0xd0c]        @ 0x7bf4 <entry+0x7bf4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3340)?)?;
    // 0x5c6e4: ldr r2, [pc, #0x128]        @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,296)?)?;
    // 0x5c6e8: mov r3, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,93)?)?;
    // 0x5c6ec: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,7168)?)?;
    // 0x5c6f0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5c6f4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c6f8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2da6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c6fc: ldr r4, [pc, #0x140]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,320)?)?;
    // 0x5c700: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c704: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c708: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2da7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c70c: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5c710: ldr r1, [pc, #0xfd0]        @ 0x7ee8 <entry+0x7ee8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4048)?)?;
    // 0x5c714: b 0x5c8b0 <entry+0x70b0>  @ imm = #0x194
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c8b0,false)?)?;
    // 0x5c718: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c71c: b 0x5c750 <entry+0x6f50>  @ imm = #0x2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c750,false)?)?;
    // 0x5c720: ldr r2, [pc, #0x1d8]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,472)?)?;
    // 0x5c724: mov r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
    // 0x5c728: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5c72c: str r0, [r9, #0xdbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3516)?)?;
    // 0x5c730: orr r1, r1, #6912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6912)?)?;
    // 0x5c734: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c738: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5af90
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c73c: ldr r0, [r9, #0xdc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3520)?)?;
    // 0x5c740: b 0x5c75c <entry+0x6f5c>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c75c,false)?)?;
    // 0x5c744: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x5c748: b 0x5c750 <entry+0x6f50>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c750,false)?)?;
    // 0x5c74c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c750: str r0, [r9, #0xdbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3516)?)?;
    // 0x5c754: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5c758: str r0, [r9, #0xdc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3520)?)?;
    // 0x5c75c: ldr r1, [r9, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,3372)?)?;
    // 0x5c760: add r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3)?)?;
    // 0x5c764: bic r2, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R0,3)?)?;
    // 0x5c768: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x5c76c: ldr r1, [pc, #0xfc8]        @ 0x7f3c <entry+0x7f3c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4040)?)?;
    // 0x5c770: lsreq r2, r0, #2
    c.emit(arm32::mov_shifted_register(C::Eq,R::R2,R::R0,Shift::Lsr,2)?)?;
    // 0x5c774: str r2, [r9, #0xdc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,3520)?)?;
    // 0x5c778: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5c77c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c780: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2daf4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c784: ldr r1, [pc, #0xb8]         @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,184)?)?;
    // 0x5c788: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c78c: b 0x5c7b0 <entry+0x6fb0>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c7b0,false)?)?;
    // 0x5c790: str r0, [r9, #0xd8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3468)?)?;
    // 0x5c794: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5c798: ldr r3, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x5c79c: ldr r1, [pc, #0xf9c]        @ 0x7f40 <entry+0x7f40>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3996)?)?;
    // 0x5c7a0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c7a4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2db18
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c7a8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c7ac: ldr r1, [pc, #0x90]         @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,144)?)?;
    // 0x5c7b0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2db24
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c7b4: b 0x5c8cc <entry+0x70cc>  @ imm = #0x110
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c8cc,false)?)?;
    // 0x5c7b8: ldr r4, [pc, #0x84]         @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,132)?)?;
    // 0x5c7bc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5c7c0: str r0, [r9, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1528)?)?;
    // 0x5c7c4: bne 0x5c8a8 <entry+0x70a8>  @ imm = #0xdc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c8a8,false)?)?;
    // 0x5c7c8: ldr r1, [r9, #0x63c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1596)?)?;
    // 0x5c7cc: ldr r2, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,68)?)?;
    // 0x5c7d0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x5c7d4: bl 0x7e544 <entry+0x28d44> @ imm = #0x21d68
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e544,true)?)?;
    // 0x5c7d8: ldr r0, [r9, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,124)?)?;
    // 0x5c7dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5c7e0: beq 0x5c894 <entry+0x7094>  @ imm = #0xac
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c894,false)?)?;
    // 0x5c7e4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5c7e8: bl 0x804f4 <entry+0x2acf4> @ imm = #0x23d04
    c.emit(arm32::branch(C::Al,c.at as u32,0x804f4,true)?)?;
    // 0x5c7ec: b 0x5c8a8 <entry+0x70a8>  @ imm = #0xb4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c8a8,false)?)?;
    // 0x5c7f0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5c7f4: str r0, [r9, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3472)?)?;
    // 0x5c7f8: ldrb r0, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,3568)?)?;
    // 0x5c7fc: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5c800: beq 0x5c85c <entry+0x705c>  @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c85c,false)?)?;
    // 0x5c804: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c808: strb r0, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3568)?)?;
    // 0x5c80c: b 0x5c85c <entry+0x705c>  @ imm = #0x48
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c85c,false)?)?;
 c.check(0x5c810)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5C810:[u32;2]=[0x000eeb01,0x000eeb65];
 for value in INLINE_DATA_5C810{c.emit(value)?;}
 // Executable A32 block 0x5c818..0x5c840.
 c.check(0x5c818)?;
    // 0x5c818: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c81c: str r0, [r9, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3472)?)?;
    // 0x5c820: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x5c824: strb r0, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3568)?)?;
    // 0x5c828: ldr r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-44)?)?;
    // 0x5c82c: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x5c830: cmp r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,255)?)?;
    // 0x5c834: bne 0x5c878 <entry+0x7078>  @ imm = #0x3c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c878,false)?)?;
    // 0x5c838: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c83c: b 0x5c870 <entry+0x7070>  @ imm = #0x2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c870,false)?)?;
 c.check(0x5c840)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5C840:[u32;2]=[0x000eeb7d,0x000e2788];
 for value in INLINE_DATA_5C840{c.emit(value)?;}
 // Executable A32 block 0x5c848..0x5c890.
 c.check(0x5c848)?;
    // 0x5c848: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c84c: str r0, [r9, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3472)?)?;
    // 0x5c850: ldrb r1, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,3568)?)?;
    // 0x5c854: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x5c858: strbne r0, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(9)?,3568)?)?;
    // 0x5c85c: ldr r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-44)?)?;
    // 0x5c860: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x5c864: cmp r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,255)?)?;
    // 0x5c868: bne 0x5c878 <entry+0x7078>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c878,false)?)?;
    // 0x5c86c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c870: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // 0x5c874: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5c878: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c87c: strb r0, [r9, #0xdf2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3570)?)?;
    // 0x5c880: ldr r1, [pc, #0xfe4]        @ 0x806c <entry+0x806c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4068)?)?;
    // 0x5c884: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c888: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x5c88c: b 0x5e47c <entry+0x8c7c>  @ imm = #0x1be8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e47c,false)?)?;
 c.check(0x5c890)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5C890:[u32;1]=[0x000eebb0];
 for value in INLINE_DATA_5C890{c.emit(value)?;}
 // Executable A32 block 0x5c894..0x5c900.
 c.check(0x5c894)?;
    // 0x5c894: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c898: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x5c89c: ldr r2, [pc, #0x5c]         @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,92)?)?;
    // 0x5c8a0: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x5c8a4: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5b0fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c8a8: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5c8ac: ldr r1, [pc, #0xfbc]        @ 0x8070 <entry+0x8070>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4028)?)?;
    // 0x5c8b0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c8b4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2dc28
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c8b8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5c8bc: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5c8c0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2dc34
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c8c4: mov r4, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,20480)?)?;
    // 0x5c8c8: orr r4, r4, #35651584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,35651584)?)?;
    // 0x5c8cc: mov r10, r6
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(6)?))?;
    // 0x5c8d0: b 0x5e4c4 <entry+0x8cc4>  @ imm = #0x1bec
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4c4,false)?)?;
    // 0x5c8d4: ldr r1, [pc, #0xf98]        @ 0x8074 <entry+0x8074>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3992)?)?;
    // 0x5c8d8: ldr r2, [pc, #0xf98]        @ 0x8078 <entry+0x8078>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3992)?)?;
    // 0x5c8dc: mov r3, #157
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,157)?)?;
    // 0x5c8e0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c8e4: orr r3, r3, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // 0x5c8e8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2dc5c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c8ec: ldr r1, [pc, #-0xb0]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,-176)?)?;
    // 0x5c8f0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c8f4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2dc68
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c8f8: mov r1, #158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,158)?)?;
    // 0x5c8fc: b 0x5c938 <entry+0x7138>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c938,false)?)?;
 c.check(0x5c900)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5C900:[u32;1]=[0x000ee470];
 for value in INLINE_DATA_5C900{c.emit(value)?;}
 // Executable A32 block 0x5c904..0x5c9e0.
 c.check(0x5c904)?;
    // 0x5c904: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c908: str r0, [r9, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3472)?)?;
    // 0x5c90c: b 0x5c87c <entry+0x707c>  @ imm = #-0x98
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c87c,false)?)?;
    // 0x5c910: ldr r1, [pc, #0xf5c]        @ 0x8074 <entry+0x8074>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3932)?)?;
    // 0x5c914: ldr r2, [pc, #0xf5c]        @ 0x8078 <entry+0x8078>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3932)?)?;
    // 0x5c918: mov r3, #876
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,876)?)?;
    // 0x5c91c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c920: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x5c924: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2dc98
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c928: ldr r1, [pc, #-0xec]        @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,-236)?)?;
    // 0x5c92c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5c930: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2dca4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5c934: mov r1, #109
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,109)?)?;
    // 0x5c938: ldr r2, [pc, #-0x40]        @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,-64)?)?;
    // 0x5c93c: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x5c940: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c944: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5b19c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5c948: b 0x5c880 <entry+0x7080>  @ imm = #-0xd0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c880,false)?)?;
    // 0x5c94c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c950: b 0x5c958 <entry+0x7158>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c958,false)?)?;
    // 0x5c954: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5c958: str r0, [r9, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3472)?)?;
    // 0x5c95c: mov r0, #248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,248)?)?;
    // 0x5c960: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x5c964: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5c968: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x5c96c: strb r0, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3568)?)?;
    // 0x5c970: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c974: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // 0x5c978: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5c97c: b 0x5c87c <entry+0x707c>  @ imm = #-0x108
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c87c,false)?)?;
    // 0x5c980: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c984: str r0, [r9, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3472)?)?;
    // 0x5c988: strb r0, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3568)?)?;
    // 0x5c98c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5c990: b 0x5c87c <entry+0x707c>  @ imm = #-0x11c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c87c,false)?)?;
    // 0x5c994: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5c998: mov r1, #66
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,66)?)?;
    // 0x5c99c: b 0x5c6a4 <entry+0x6ea4>  @ imm = #-0x300
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c6a4,false)?)?;
    // 0x5c9a0: mov r5, r10
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(10)?))?;
    // 0x5c9a4: mov r1, #24576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24576)?)?;
    // 0x5c9a8: ldr r0, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x5c9ac: orr r1, r1, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,33554432)?)?;
    // 0x5c9b0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5c9b4: bgt 0x5caf0 <entry+0x72f0>  @ imm = #0x134
    c.emit(arm32::branch(C::Gt,c.at as u32,0x5caf0,false)?)?;
    // 0x5c9b8: ldr r1, [pc, #0xfe0]        @ 0x81a0 <entry+0x81a0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4064)?)?;
    // 0x5c9bc: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5c9c0: ble 0x5cbe0 <entry+0x73e0>  @ imm = #0x218
    c.emit(arm32::branch(C::Le,c.at as u32,0x5cbe0,false)?)?;
    // 0x5c9c4: ldr r1, [pc, #0xfd8]        @ 0x81a4 <entry+0x81a4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4056)?)?;
    // 0x5c9c8: add r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5c9cc: cmp r1, #55
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,55)?)?;
    // 0x5c9d0: bhi 0x5d010 <entry+0x7810>  @ imm = #0x638
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5d010,false)?)?;
    // 0x5c9d4: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x5c9d8: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // 0x5c9dc: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
 c.check(0x5c9e0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5C9E0:[u32;56]=[0x0005cac0,0x0005dc78,0x0005dcb8,0x0005dcf4,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005dd70,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005dde8,0x0005de3c,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005de88,0x0005e4e0,0x0005dea4,0x0005decc,0x0005def0,0x0005df14,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005df40,0x0005dfac,0x0005e02c,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005e08c,0x0005e0b0,0x0005e4e0,0x0005e0d4,0x0005e100,0x0005e188,0x0005e4e0,0x0005e1ac,0x0005e4e0,0x0005e4e0,0x0005e1d8,0x0005e27c,0x0005e4e0,0x0005e2ac,0x0005e4e0,0x0005e364,0x0005e4e0,0x0005e390,0x0005e3c0,0x0005e4e0,0x0005e3ec,0x0005e438,0x0005e45c];
 for value in INLINE_DATA_5C9E0{c.emit(value)?;}
 // Executable A32 block 0x5cac0..0x5cb14.
 c.check(0x5cac0)?;
    // 0x5cac0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5cac4: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5cac8: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5cacc: blo 0x5e710 <entry+0x8f10>  @ imm = #0x1c3c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e710,false)?)?;
    // 0x5cad0: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5cad4: ldr r1, [r11, #-0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-196)?)?;
    // 0x5cad8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5cadc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5cae0: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5cae4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5cae8: ldr r1, [pc, #0xff0]        @ 0x82e0 <entry+0x82e0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4080)?)?;
    // 0x5caec: b 0x5e478 <entry+0x8c78>  @ imm = #0x1984
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
    // 0x5caf0: cmp r0, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x5caf4: bgt 0x5ccbc <entry+0x74bc>  @ imm = #0x1c0
    c.emit(arm32::branch(C::Gt,c.at as u32,0x5ccbc,false)?)?;
    // 0x5caf8: ldr r1, [pc, #0xfe4]        @ 0x82e4 <entry+0x82e4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4068)?)?;
    // 0x5cafc: add r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cb00: cmp r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,20)?)?;
    // 0x5cb04: bhi 0x5d05c <entry+0x785c>  @ imm = #0x550
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5d05c,false)?)?;
    // 0x5cb08: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x5cb0c: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // 0x5cb10: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
 c.check(0x5cb14)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5CB14:[u32;21]=[0x0005cb68,0x0005d6ec,0x0005d744,0x0005d7d8,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005d824,0x0005d87c,0x0005d8e0,0x0005d944,0x0005e4e0,0x0005d9a8,0x0005d9e0,0x0005da2c,0x0005da84,0x0005dae8,0x0005db28,0x0005dbb4,0x0005dbf0,0x0005dc4c];
 for value in INLINE_DATA_5CB14{c.emit(value)?;}
 // Executable A32 block 0x5cb68..0x5cc1c.
 c.check(0x5cb68)?;
    // 0x5cb68: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5cb6c: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5cb70: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5cb74: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cb78: blo 0x5e718 <entry+0x8f18>  @ imm = #0x1b98
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e718,false)?)?;
    // 0x5cb7c: ldr r2, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,8)?)?;
    // 0x5cb80: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // 0x5cb84: bhi 0x5c144 <entry+0x6944>  @ imm = #-0xa48
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5c144,false)?)?;
    // 0x5cb88: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5cb8c: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x59fa0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5cb90: ldmib r10, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5cb94: ldr r2, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,88)?)?;
    // 0x5cb98: str r0, [r2, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,2)?)?;
    // 0x5cb9c: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5cba0: orr r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // 0x5cba4: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5cba8: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5cbac: str r0, [r9, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,364)?)?;
    // 0x5cbb0: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5cbb4: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x59f88
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5cbb8: ldmib r10, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R0.bit()|R::R2.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5cbbc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5cbc0: beq 0x5c790 <entry+0x6f90>  @ imm = #-0x438
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c790,false)?)?;
    // 0x5cbc4: ldr r1, [r9, #0xd8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,3468)?)?;
    // 0x5cbc8: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5cbcc: str r0, [r9, #0xd8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3468)?)?;
    // 0x5cbd0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5cbd4: ldr r1, [sp, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,124)?)?;
    // 0x5cbd8: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5cbdc: b 0x5c798 <entry+0x6f98>  @ imm = #-0x44c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c798,false)?)?;
    // 0x5cbe0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5cbe4: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x5cbe8: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cbec: bgt 0x5cd54 <entry+0x7554>  @ imm = #0x160
    c.emit(arm32::branch(C::Gt,c.at as u32,0x5cd54,false)?)?;
    // 0x5cbf0: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x5cbf4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5cbf8: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cbfc: bgt 0x5ce24 <entry+0x7624>  @ imm = #0x220
    c.emit(arm32::branch(C::Gt,c.at as u32,0x5ce24,false)?)?;
    // 0x5cc00: mvn r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,4096)?)?;
    // 0x5cc04: add r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cc08: cmp r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,10)?)?;
    // 0x5cc0c: bhi 0x5e4e0 <entry+0x8ce0>  @ imm = #0x18cc
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5e4e0,false)?)?;
    // 0x5cc10: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x5cc14: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // 0x5cc18: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
 c.check(0x5cc1c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5CC1C:[u32;11]=[0x0005cc48,0x0005d2e4,0x0005d390,0x0005e4e0,0x0005d3f8,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005d440];
 for value in INLINE_DATA_5CC1C{c.emit(value)?;}
 // Executable A32 block 0x5cc48..0x5cce8.
 c.check(0x5cc48)?;
    // 0x5cc48: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5cc4c: add r1, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,16)?)?;
    // 0x5cc50: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x5cc54: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cc58: blo 0x5e5ac <entry+0x8dac>  @ imm = #0x194c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e5ac,false)?)?;
    // 0x5cc5c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5cc60: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5cc64: beq 0x5c208 <entry+0x6a08>  @ imm = #-0xa64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c208,false)?)?;
    // 0x5cc68: ldr r5, [pc, #-0x42c]       @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,-1068)?)?;
    // 0x5cc6c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5cc70: bne 0x5c278 <entry+0x6a78>  @ imm = #-0xa00
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c278,false)?)?;
    // 0x5cc74: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5cc78: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5cc7c: str r0, [r9, #0xd48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3400)?)?;
    // 0x5cc80: ldr r1, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,12)?)?;
    // 0x5cc84: str r1, [r9, #0xd4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3404)?)?;
    // 0x5cc88: str r6, [r9, #0xd54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,3412)?)?;
    // 0x5cc8c: str r6, [r9, #0xd58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,3416)?)?;
    // 0x5cc90: str r0, [r9, #0xd5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3420)?)?;
    // 0x5cc94: str r1, [r9, #0xd60]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3424)?)?;
    // 0x5cc98: ldr r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // 0x5cc9c: ldr r1, [r9, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1392)?)?;
    // 0x5cca0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5cca4: bl 0x58130 <entry+0x2930>  @ imm = #-0x4b7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x58130,true)?)?;
    // 0x5cca8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x5ccac: mov r4, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,20480)?)?;
    // 0x5ccb0: orr r4, r4, #35651584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,35651584)?)?;
    // 0x5ccb4: bl 0x58214 <entry+0x2a14>  @ imm = #-0x4aa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x58214,true)?)?;
    // 0x5ccb8: b 0x5c290 <entry+0x6a90>  @ imm = #-0xa30
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c290,false)?)?;
    // 0x5ccbc: mov r1, #28672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28672)?)?;
    // 0x5ccc0: orr r1, r1, #50331648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,50331648)?)?;
    // 0x5ccc4: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ccc8: bgt 0x5ce74 <entry+0x7674>  @ imm = #0x1a4
    c.emit(arm32::branch(C::Gt,c.at as u32,0x5ce74,false)?)?;
    // 0x5cccc: ldr r1, [pc, #0xfe0]        @ 0x84b4 <entry+0x84b4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4064)?)?;
    // 0x5ccd0: add r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ccd4: cmp r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,11)?)?;
    // 0x5ccd8: bhi 0x5e4e0 <entry+0x8ce0>  @ imm = #0x1800
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5e4e0,false)?)?;
    // 0x5ccdc: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x5cce0: add r2, pc, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,24)?)?;
    // 0x5cce4: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
 c.check(0x5cce8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5CCE8:[u32;19]=[0x03007001,0x02205001,0x02006002,0x02005001,0x00201002,0x00201001,0x0005cd34,0x0005e4e0,0x0005d4f0,0x0005d510,0x0005d530,0x0005d55c,0x0005d580,0x0005d5ac,0x0005e4e0,0x0005d5cc,0x0005d5f4,0x0005d624,0x000eebe2];
 for value in INLINE_DATA_5CCE8{c.emit(value)?;}
 // Executable A32 block 0x5cd34..0x5cd70.
 c.check(0x5cd34)?;
    // 0x5cd34: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5cd38: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5cd3c: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5cd40: blo 0x5e5b4 <entry+0x8db4>  @ imm = #0x186c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e5b4,false)?)?;
    // 0x5cd44: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5cd48: ldr r0, [r11, #-0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-104)?)?;
    // 0x5cd4c: ldr r1, [pc, #0xfe4]        @ 0x8538 <entry+0x8538>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4068)?)?;
    // 0x5cd50: b 0x5e4ac <entry+0x8cac>  @ imm = #0x1754
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4ac,false)?)?;
    // 0x5cd54: ldr r1, [pc, #0xfe0]        @ 0x853c <entry+0x853c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4064)?)?;
    // 0x5cd58: add r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cd5c: cmp r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,13)?)?;
    // 0x5cd60: bhi 0x5cee4 <entry+0x76e4>  @ imm = #0x17c
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5cee4,false)?)?;
    // 0x5cd64: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x5cd68: add r2, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,4)?)?;
    // 0x5cd6c: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
 c.check(0x5cd70)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5CD70:[u32;16]=[0x000ef7bb,0x0005cdb0,0x0005d0e0,0x0005e4e0,0x0005e4e0,0x0005e4e0,0x0005d1c4,0x0005e4e0,0x0005e4e0,0x0005d208,0x0005e4e0,0x0005e4e0,0x0005d230,0x0005e4e0,0x0005d2b8,0x000eec10];
 for value in INLINE_DATA_5CD70{c.emit(value)?;}
 // Executable A32 block 0x5cdb0..0x5ce20.
 c.check(0x5cdb0)?;
    // 0x5cdb0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5cdb4: add r1, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,16)?)?;
    // 0x5cdb8: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5cdbc: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cdc0: blo 0x5e55c <entry+0x8d5c>  @ imm = #0x1794
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e55c,false)?)?;
    // 0x5cdc4: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5cdc8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5cdcc: beq 0x5c3cc <entry+0x6bcc>  @ imm = #-0xa08
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c3cc,false)?)?;
    // 0x5cdd0: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x5cdd4: beq 0x5c3dc <entry+0x6bdc>  @ imm = #-0xa00
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c3dc,false)?)?;
    // 0x5cdd8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5cddc: bne 0x5c404 <entry+0x6c04>  @ imm = #-0x9e0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c404,false)?)?;
    // 0x5cde0: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5cde4: ldr r2, [r9, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1528)?)?;
    // 0x5cde8: ldr r1, [r9, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,124)?)?;
    // 0x5cdec: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5cdf0: bhs 0x5c7b8 <entry+0x6fb8>  @ imm = #-0x640
    c.emit(arm32::branch(C::Cs,c.at as u32,0x5c7b8,false)?)?;
    // 0x5cdf4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5cdf8: beq 0x5ce14 <entry+0x7614>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5ce14,false)?)?;
    // 0x5cdfc: ldr r5, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,68)?)?;
    // 0x5ce00: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5ce04: bl 0x7e7d4 <entry+0x28fd4> @ imm = #0x219c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e7d4,true)?)?;
    // 0x5ce08: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5ce0c: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x5ce10: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5ce14: ldr r4, [pc, #-0x5d8]       @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,-1496)?)?;
    // 0x5ce18: str r0, [r9, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1528)?)?;
    // 0x5ce1c: b 0x5c7c8 <entry+0x6fc8>  @ imm = #-0x65c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c7c8,false)?)?;
 c.check(0x5ce20)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5CE20:[u32;1]=[0x000f01b7];
 for value in INLINE_DATA_5CE20{c.emit(value)?;}
 // Executable A32 block 0x5ce24..0x5cee0.
 c.check(0x5ce24)?;
    // 0x5ce24: mov r1, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    // 0x5ce28: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5ce2c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ce30: bgt 0x5cf80 <entry+0x7780>  @ imm = #0x148
    c.emit(arm32::branch(C::Gt,c.at as u32,0x5cf80,false)?)?;
    // 0x5ce34: mov r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
    // 0x5ce38: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5ce3c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ce40: beq 0x5d49c <entry+0x7c9c>  @ imm = #0x654
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5d49c,false)?)?;
    // 0x5ce44: mov r1, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,21)?)?;
    // 0x5ce48: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5ce4c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ce50: bne 0x5e4e0 <entry+0x8ce0>  @ imm = #0x1688
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5e4e0,false)?)?;
    // 0x5ce54: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5ce58: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5ce5c: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5ce60: blo 0x5e5bc <entry+0x8dbc>  @ imm = #0x1754
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e5bc,false)?)?;
    // 0x5ce64: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5ce68: ldr r0, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-68)?)?;
    // 0x5ce6c: ldr r1, [pc, #0xfc4]        @ 0x8638 <entry+0x8638>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4036)?)?;
    // 0x5ce70: b 0x5d048 <entry+0x7848>  @ imm = #0x1d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5d048,false)?)?;
    // 0x5ce74: ldr r1, [pc, #-0x194]       @ 0x74e8 <entry+0x74e8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,-404)?)?;
    // 0x5ce78: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ce7c: beq 0x5d648 <entry+0x7e48>  @ imm = #0x7c4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5d648,false)?)?;
    // 0x5ce80: ldr r1, [pc, #0xff8]        @ 0x8680 <entry+0x8680>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5ce84: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ce88: beq 0x5d674 <entry+0x7e74>  @ imm = #0x7e4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5d674,false)?)?;
    // 0x5ce8c: ldr r1, [pc, #0xff0]        @ 0x8684 <entry+0x8684>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4080)?)?;
    // 0x5ce90: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ce94: bne 0x5e4e0 <entry+0x8ce0>  @ imm = #0x1644
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5e4e0,false)?)?;
    // 0x5ce98: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5ce9c: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5cea0: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5cea4: blo 0x5e5c8 <entry+0x8dc8>  @ imm = #0x171c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e5c8,false)?)?;
    // 0x5cea8: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5ceac: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5a2c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5ceb0: ldr r2, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,100)?)?;
    // 0x5ceb4: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x5ceb8: ldr r0, [r11, #-0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-124)?)?;
    // 0x5cebc: bl 0x35ee4 <entry+0xfffffffffffe06e4> @ imm = #-0x26fe0
    c.emit(arm32::branch(C::Al,c.at as u32,0x35ee4,true)?)?;
    // 0x5cec0: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5cec4: orr r0, r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,131072)?)?;
    // 0x5cec8: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5cecc: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5ced0: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5a2a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5ced4: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5ced8: ldr r1, [pc, #0xfe8]        @ 0x86c8 <entry+0x86c8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4072)?)?;
    // 0x5cedc: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0x15cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
 c.check(0x5cee0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5CEE0:[u32;1]=[0x000f021e];
 for value in INLINE_DATA_5CEE0{c.emit(value)?;}
 // Executable A32 block 0x5cee4..0x5cf70.
 c.check(0x5cee4)?;
    // 0x5cee4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x5cee8: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x5ceec: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cef0: beq 0x5d6bc <entry+0x7ebc>  @ imm = #0x7c4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5d6bc,false)?)?;
    // 0x5cef4: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x5cef8: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x5cefc: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cf00: bne 0x5e4e0 <entry+0x8ce0>  @ imm = #0x15d8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5e4e0,false)?)?;
    // 0x5cf04: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5cf08: add r1, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,8)?)?;
    // 0x5cf0c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5cf10: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cf14: blo 0x5e5d4 <entry+0x8dd4>  @ imm = #0x16b8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e5d4,false)?)?;
    // 0x5cf18: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // 0x5cf1c: lsr r1, r2, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Lsr,16)?)?;
    // 0x5cf20: str r1, [r9, #0xcb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3256)?)?;
    // 0x5cf24: ldr r0, [r9, #0xd88]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3464)?)?;
    // 0x5cf28: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5cf2c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5cf30: lsrhi r0, r2, #16
    c.emit(arm32::mov_shifted_register(C::Hi,R::R0,R::R2,Shift::Lsr,16)?)?;
    // 0x5cf34: cmp r0, #61
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,61)?)?;
    // 0x5cf38: blo 0x5cf64 <entry+0x7764>  @ imm = #0x24
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5cf64,false)?)?;
    // 0x5cf3c: ldrb r1, [r9, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,3104)?)?;
    // 0x5cf40: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x5cf44: and r1, r1, #253
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,253)?)?;
    // 0x5cf48: strb r1, [r9, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,3104)?)?;
    // 0x5cf4c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5cf50: str r1, [r9, #0xec0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3776)?)?;
    // 0x5cf54: ldr r1, [pc, #0xfdc]        @ 0x8738 <entry+0x8738>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4060)?)?;
    // 0x5cf58: umull r3, r7, r0, r1
    c.emit(emit_umull(C::Al,R::new(3)?,R::new(7)?,R::R0,R::R1)?)?;
    // 0x5cf5c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5cf60: add r1, r0, r7, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(7)?,Shift::Lsr,5,false)?)?;
    // 0x5cf64: str r1, [r9, #0xcb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3252)?)?;
    // 0x5cf68: ldr r1, [pc, #0xfcc]        @ 0x873c <entry+0x873c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4044)?)?;
    // 0x5cf6c: b 0x5d818 <entry+0x8018>  @ imm = #0x8a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5d818,false)?)?;
 c.check(0x5cf70)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5CF70:[u32;4]=[0x000f0274,0x000f02ca,0x000ef8e6,0x000ef931];
 for value in INLINE_DATA_5CF70{c.emit(value)?;}
 // Executable A32 block 0x5cf80..0x5d000.
 c.check(0x5cf80)?;
    // 0x5cf80: mov r1, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23)?)?;
    // 0x5cf84: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x5cf88: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cf8c: beq 0x5d4c8 <entry+0x7cc8>  @ imm = #0x534
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5d4c8,false)?)?;
    // 0x5cf90: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5cf94: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x5cf98: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cf9c: bne 0x5e4e0 <entry+0x8ce0>  @ imm = #0x153c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5e4e0,false)?)?;
    // 0x5cfa0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5cfa4: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5cfa8: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5cfac: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5cfb0: blo 0x5e5e4 <entry+0x8de4>  @ imm = #0x162c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e5e4,false)?)?;
    // 0x5cfb4: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5cfb8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5cfbc: beq 0x5c2c4 <entry+0x6ac4>  @ imm = #-0xd00
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c2c4,false)?)?;
    // 0x5cfc0: ldr r4, [pc, #-0x784]       @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,-1924)?)?;
    // 0x5cfc4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5cfc8: bne 0x5c300 <entry+0x6b00>  @ imm = #-0xcd0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c300,false)?)?;
    // 0x5cfcc: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5cfd0: lsr r1, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsr,16)?)?;
    // 0x5cfd4: str r0, [r9, #0xd84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3460)?)?;
    // 0x5cfd8: str r1, [r9, #0xd88]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3464)?)?;
    // 0x5cfdc: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5cfe0: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5a3f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5cfe4: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5cfe8: str r0, [r9, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,332)?)?;
    // 0x5cfec: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5cff0: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // 0x5cff4: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5cff8: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5cffc: str r0, [r9, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,352)?)?;
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
