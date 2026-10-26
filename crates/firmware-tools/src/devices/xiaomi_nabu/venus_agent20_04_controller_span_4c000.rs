// SPDX-License-Identifier: GPL-3.0-only
//! Continue the Venus controller stream and codec parameter writer through
//! runtime [0x4c000, 0x4d000). Every executable word is assembled from typed
//! A32 operations, without copying OEM opcode literals or filling unknown
//! ranges with zeroes. The parent function continues beyond the fragment.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
pub const START:usize=0x4c000;
pub const END:usize=0x4d000;
const CONTROLLER_LOG_POINTER:u32=0x000e_e028; // PC referenced by eleven LDRs, branch-skipped
struct Code{at:usize, bytes:Vec<u8>}
impl Code{
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,a:usize)->Result<(),String>{
  if self.at!=a{return Err(format!("Venus runtime {:#x} expected {a:#x}",self.at));}Ok(())
 }
 #[inline(never)]
 fn emit(&mut self,w:u32)->Result<(),String>{
  if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0{return Err("outside Agent04 area".into());}
  self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
 }
 fn branch(&mut self,c:C,target:usize)->Result<(),String>{
  self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
 }
 fn literal(&mut self,c:C,rd:R,target:usize)->Result<(),String>{
  let offset=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"literal address overflow")?;
  self.emit(arm32::transfer_word(c,true,rd,R::PC,offset)?)
 }
}
fn emit_controller_code(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x4c000: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c004: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c008: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c00c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c010: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c014: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c018: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c01c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c020: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c024: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c028: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c02c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c030: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c034: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c038: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c03c: ldr r2, [pc, #0xb2c]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c040: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c044: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c048: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c04c: add r0, r8, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,24)?)?;
    // 0x4c050: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c054: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c058: mov r1, #372
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,372)?)?;
    // 0x4c05c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c060: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c064: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4c068: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c06c: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4a8c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c070: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c074: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x4c078: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c07c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4c080: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c084: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c088: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c08c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c090: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c094: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c098: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c09c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c0a0: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c0a4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c0a8: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c0ac: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c0b0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c0b4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c0b8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c0bc: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c0c0: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c0c4: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c0c8: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c0cc: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c0d0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c0d4: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c0d8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c0dc: ldr r2, [pc, #0xa8c]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c0e0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c0e4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c0e8: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c0ec: add r0, r8, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,28)?)?;
    // 0x4c0f0: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c0f4: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c0f8: mov r1, #117
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,117)?)?;
    // 0x4c0fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c100: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c104: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4c108: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c10c: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4a964
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c110: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c114: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c118: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x4c11c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c120: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c124: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c128: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c12c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c130: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c134: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c138: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c13c: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c140: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c144: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c148: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c14c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c150: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c154: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c158: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c15c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c160: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c164: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c168: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c16c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c170: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c174: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c178: ldr r2, [pc, #0x9f0]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c17c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c180: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c184: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c188: add r0, r8, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,32)?)?;
    // 0x4c18c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c190: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c194: mov r1, #118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,118)?)?;
    // 0x4c198: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c19c: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c1a0: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4c1a4: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c1a8: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4aa00
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c1ac: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c1b0: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x4c1b4: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c1b8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4c1bc: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c1c0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c1c4: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c1c8: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c1cc: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c1d0: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c1d4: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c1d8: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c1dc: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c1e0: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c1e4: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c1e8: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c1ec: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c1f0: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c1f4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c1f8: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c1fc: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c200: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c204: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c208: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c20c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c210: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c214: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c218: ldr r2, [pc, #0x950]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c21c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c220: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c224: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c228: add r0, r8, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,36)?)?;
    // 0x4c22c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c230: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c234: mov r1, #119
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,119)?)?;
    // 0x4c238: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c23c: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c240: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4c244: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c248: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4aaa0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c24c: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c250: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c254: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x4c258: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c25c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c260: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c264: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c268: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c26c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c270: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c274: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c278: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c27c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c280: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c284: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c288: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c28c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c290: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c294: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c298: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c29c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c2a0: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c2a4: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c2a8: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c2ac: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c2b0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c2b4: ldr r2, [pc, #0x8b4]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c2b8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c2bc: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c2c0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c2c4: add r0, r8, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,40)?)?;
    // 0x4c2c8: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c2cc: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c2d0: mov r1, #376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,376)?)?;
    // 0x4c2d4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c2d8: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c2dc: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4c2e0: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c2e4: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4ab3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c2e8: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c2ec: mov r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // 0x4c2f0: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c2f4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4c2f8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c2fc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c300: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c304: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c308: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c30c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c310: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c314: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c318: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c31c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c320: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c324: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c328: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c32c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c330: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c334: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c338: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c33c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c340: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c344: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c348: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c34c: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c350: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c354: ldr r2, [pc, #0x814]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c358: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c35c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c360: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c364: add r0, r8, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,44)?)?;
    // 0x4c368: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c36c: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c370: mov r1, #121
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,121)?)?;
    // 0x4c374: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c378: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c37c: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4c380: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c384: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4abdc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c388: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c38c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c390: mov r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // 0x4c394: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c398: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c39c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c3a0: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c3a4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c3a8: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c3ac: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c3b0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c3b4: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c3b8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c3bc: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c3c0: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c3c4: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c3c8: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c3cc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c3d0: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c3d4: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c3d8: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c3dc: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c3e0: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c3e4: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c3e8: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c3ec: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c3f0: ldr r2, [pc, #0x778]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c3f4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c3f8: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c3fc: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c400: add r0, r8, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,48)?)?;
    // 0x4c404: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c408: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c40c: mov r1, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,122)?)?;
    // 0x4c410: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c414: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c418: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4c41c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c420: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4ac78
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c424: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c428: mov r2, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,104)?)?;
    // 0x4c42c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c430: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4c434: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c438: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c43c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c440: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c444: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c448: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c44c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c450: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c454: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c458: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c45c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c460: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c464: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c468: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c46c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c470: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c474: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c478: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c47c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c480: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c484: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c488: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c48c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c490: ldr r2, [pc, #0x6d8]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c494: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c498: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c49c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c4a0: add r0, r8, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,52)?)?;
    // 0x4c4a4: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c4a8: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c4ac: mov r1, #123
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,123)?)?;
    // 0x4c4b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c4b4: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c4b8: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4c4bc: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c4c0: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4ad18
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c4c4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c4c8: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c4cc: mov r2, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,104)?)?;
    // 0x4c4d0: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c4d4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c4d8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c4dc: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c4e0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c4e4: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c4e8: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c4ec: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c4f0: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c4f4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c4f8: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c4fc: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c500: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c504: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c508: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c50c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c510: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c514: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c518: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c51c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c520: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c524: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c528: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c52c: ldr r2, [pc, #0x63c]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c530: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c534: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c538: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c53c: add r0, r8, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,56)?)?;
    // 0x4c540: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c544: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c548: mov r1, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,380)?)?;
    // 0x4c54c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c550: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c554: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4c558: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c55c: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4adb4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c560: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c564: mov r2, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,120)?)?;
    // 0x4c568: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c56c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4c570: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c574: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c578: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c57c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c580: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c584: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c588: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c58c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c590: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c594: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c598: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c59c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c5a0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c5a4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c5a8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c5ac: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c5b0: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c5b4: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c5b8: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c5bc: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c5c0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c5c4: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c5c8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c5cc: ldr r2, [pc, #0x59c]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c5d0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c5d4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c5d8: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c5dc: add r0, r8, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,60)?)?;
    // 0x4c5e0: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c5e4: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c5e8: mov r1, #125
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,125)?)?;
    // 0x4c5ec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c5f0: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4c5f4: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4c5f8: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c5fc: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4ae54
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c600: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4c604: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4c608: mov r2, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,120)?)?;
    // 0x4c60c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4c610: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c614: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c618: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4c61c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4c620: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4c624: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4c628: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4c62c: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4c630: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4c634: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4c638: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4c63c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4c640: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4c644: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4c648: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c64c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c650: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4c654: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4c658: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4c65c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4c660: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4c664: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4c668: ldr r2, [pc, #0x500]        @ 0xb70 <entry+0xb70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4c66c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c670: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c674: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4c678: add r0, r8, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,64)?)?;
    // 0x4c67c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4c680: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4c684: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4c688: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4c68c: mov r1, #126
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,126)?)?;
    // 0x4c690: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4c694: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4c698: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4aef0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4c69c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4c6a0: mov lr, r6
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(6)?))?;
    // 0x4c6a4: mov r12, #251
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,251)?)?;
    // 0x4c6a8: str r1, [sp, #0x3f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1012)?)?;
    // 0x4c6ac: orr r12, r12, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,1024)?)?;
    // 0x4c6b0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4c6b4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x4c6b8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4c6bc: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4c6c0: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4c6c4: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4c6c8: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x4c6cc: orr r2, r2, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,15360)?)?;
    // 0x4c6d0: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x4c6d4: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x4c6d8: mov r2, #129
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,129)?)?;
    // 0x4c6dc: ldr r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x4c6e0: add r1, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(3)?,1)?)?;
    // 0x4c6e4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c6e8: add r1, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,8)?)?;
    // 0x4c6ec: str r1, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x4c6f0: str r2, [sp, #0x3fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1020)?)?;
    // 0x4c6f4: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4c6f8: ldreq r7, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R1,0)?)?;
    // 0x4c6fc: ldr r9, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,88)?)?;
    // 0x4c700: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c704: streq r7, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R1,0)?)?;
    // 0x4c708: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x4c70c: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x4c710: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x4c714: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x4c718: add r2, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,2)?)?;
    // 0x4c71c: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4c720: add r2, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,16)?)?;
    // 0x4c724: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x4c728: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x4c72c: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x4c730: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c734: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x4c738: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x4c73c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4c740: add r1, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(3)?,3)?)?;
    // 0x4c744: str r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // 0x4c748: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4c74c: add r1, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,24)?)?;
    // 0x4c750: str r1, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x4c754: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4c758: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4c75c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4c760: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x4c764: mov r1, #3152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3152)?)?;
    // 0x4c768: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // 0x4c76c: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x4c770: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x4c774: str r2, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,28)?)?;
    // 0x4c778: add r2, r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,4)?)?;
    // 0x4c77c: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4c780: add r2, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,32)?)?;
    // 0x4c784: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x4c788: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x4c78c: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x4c790: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c794: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x4c798: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x4c79c: add r2, r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,5)?)?;
    // 0x4c7a0: add r1, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,40)?)?;
    // 0x4c7a4: str r6, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,36)?)?;
    // 0x4c7a8: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4c7ac: str r1, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x4c7b0: ldr r7, [r9, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,2744)?)?;
    // 0x4c7b4: strb r6, [r7, #0x4fa]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(7)?,1274)?)?;
    // 0x4c7b8: ldrsb r5, [r7, r12]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::new(5)?,R::new(7)?,R::R12)?)?;
    // 0x4c7bc: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x4c7c0: beq 0x4c7f8 <entry+0x7f8>   @ imm = #0x30
    c.branch(C::Eq,0x4c7f8)?;
    // 0x4c7c4: cmp r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8)?)?;
    // 0x4c7c8: bgt 0x4c7f8 <entry+0x7f8>   @ imm = #0x28
    c.branch(C::Gt,0x4c7f8)?;
    // 0x4c7cc: ldr r3, [r7, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,1276)?)?;
    // 0x4c7d0: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x4c7d4: beq 0x4c8e0 <entry+0x8e0>   @ imm = #0x104
    c.branch(C::Eq,0x4c8e0)?;
    // 0x4c7d8: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x4c7dc: bne 0x4ca20 <entry+0xa20>   @ imm = #0x23c
    c.branch(C::Ne,0x4ca20)?;
    // 0x4c7e0: ldr r3, [r7, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,1284)?)?;
    // 0x4c7e4: cmp r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,6)?)?;
    // 0x4c7e8: ldreq r3, [r7, #0x500]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(7)?,1280)?)?;
    // 0x4c7ec: cmpeq r3, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x4c7f0: bne 0x4ca20 <entry+0xa20>   @ imm = #0x228
    c.branch(C::Ne,0x4ca20)?;
    // 0x4c7f4: b 0x4c93c <entry+0x93c>   @ imm = #0x140
    c.branch(C::Al,0x4c93c)?;
    // 0x4c7f8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4c7fc: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4c800: orr r2, r2, #45568
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,45568)?)?;
    // 0x4c804: str r2, [sp, #0x43c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1084)?)?;
    // 0x4c808: ldreq r7, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R1,0)?)?;
    // 0x4c80c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4c810: streq r7, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R1,0)?)?;
    // 0x4c814: mov r7, #3168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3168)?)?;
    // 0x4c818: orr r7, r7, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,12288)?)?;
    // 0x4c81c: str r7, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,40)?)?;
    // 0x4c820: str r2, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,44)?)?;
    // 0x4c824: add r2, r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,6)?)?;
    // 0x4c828: add r7, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,48)?)?;
    // 0x4c82c: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4c830: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4c834: str r7, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::LR,0)?)?;
    // 0x4c838: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x4c83c: orr r2, r2, #24832
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,24832)?)?;
    // 0x4c840: str r2, [sp, #0x440]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1088)?)?;
    // 0x4c844: ldreq r6, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(7)?,0)?)?;
    // 0x4c848: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x4c84c: streq r6, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(7)?,0)?)?;
    // 0x4c850: mov r7, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,100)?)?;
    // 0x4c854: orr r7, r7, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,15360)?)?;
    // 0x4c858: str r7, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,48)?)?;
    // 0x4c85c: str r2, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,52)?)?;
    // 0x4c860: add r2, r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,7)?)?;
    // 0x4c864: add r7, r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,56)?)?;
    // 0x4c868: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4c86c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4c870: str r7, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::LR,0)?)?;
    // 0x4c874: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x4c878: orr r2, r2, #20992
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,20992)?)?;
    // 0x4c87c: str r2, [sp, #0x444]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1092)?)?;
    // 0x4c880: ldreq r6, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(7)?,0)?)?;
    // 0x4c884: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x4c888: streq r6, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(7)?,0)?)?;
    // 0x4c88c: mov r7, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,104)?)?;
    // 0x4c890: orr r7, r7, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,15360)?)?;
    // 0x4c894: str r7, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,56)?)?;
    // 0x4c898: str r2, [r0, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,60)?)?;
    // 0x4c89c: add r2, r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,8)?)?;
    // 0x4c8a0: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4c8a4: add r7, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,64)?)?;
    // 0x4c8a8: orr r3, r3, #29184
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,29184)?)?;
    // 0x4c8ac: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4c8b0: str r7, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::LR,0)?)?;
    // 0x4c8b4: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x4c8b8: str r3, [sp, #0x448]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,1096)?)?;
    // 0x4c8bc: ldreq r6, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(7)?,0)?)?;
    // 0x4c8c0: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x4c8c4: streq r6, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(7)?,0)?)?;
    // 0x4c8c8: mov r7, #108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,108)?)?;
    // 0x4c8cc: orr r7, r7, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,15360)?)?;
    // 0x4c8d0: str r7, [r0, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,64)?)?;
    // 0x4c8d4: add r7, r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,68)?)?;
    // 0x4c8d8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x4c8dc: b 0x4ca0c <entry+0xa0c>   @ imm = #0x128
    c.branch(C::Al,0x4ca0c)?;
    // 0x4c8e0: ldr r4, [r7, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,1284)?)?;
    // 0x4c8e4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4c8e8: cmp r4, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,11)?)?;
    // 0x4c8ec: bne 0x4c900 <entry+0x900>   @ imm = #0xc
    c.branch(C::Ne,0x4c900)?;
    // 0x4c8f0: ldr r4, [r7, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,1280)?)?;
    // 0x4c8f4: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x4c8f8: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4c8fc: strbeq r3, [r7, #0x4fa]
    c.emit(arm32::transfer_byte(C::Eq,false,R::new(3)?,R::new(7)?,1274)?)?;
    // 0x4c900: ldr r4, [r7, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,1284)?)?;
    // 0x4c904: cmp r4, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,5)?)?;
    // 0x4c908: bne 0x4c91c <entry+0x91c>   @ imm = #0xc
    c.branch(C::Ne,0x4c91c)?;
    // 0x4c90c: ldr r4, [r7, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,1280)?)?;
    // 0x4c910: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x4c914: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4c918: strbeq r3, [r7, #0x4fa]
    c.emit(arm32::transfer_byte(C::Eq,false,R::new(3)?,R::new(7)?,1274)?)?;
    // 0x4c91c: ldr r4, [r7, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,1284)?)?;
    // 0x4c920: cmp r4, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,7)?)?;
    // 0x4c924: ldreq r4, [r7, #0x500]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(7)?,1280)?)?;
    // 0x4c928: cmpeq r4, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x4c92c: beq 0x4c93c <entry+0x93c>   @ imm = #0x8
    c.branch(C::Eq,0x4c93c)?;
    // 0x4c930: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x4c934: bne 0x4c944 <entry+0x944>   @ imm = #0x8
    c.branch(C::Ne,0x4c944)?;
    // 0x4c938: b 0x4ca20 <entry+0xa20>   @ imm = #0xe0
    c.branch(C::Al,0x4ca20)?;
    // 0x4c93c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4c940: strb r3, [r7, #0x4fa]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(7)?,1274)?)?;
    // 0x4c944: add r3, sp, #300
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,300)?)?;
    // 0x4c948: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x4c94c: add r3, r3, #784
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,784)?)?;
    // 0x4c950: add r5, r3, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x4c954: str r4, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,-4)?)?;
    // 0x4c958: ldrsb r5, [r7, r12]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::new(5)?,R::new(7)?,R::R12)?)?;
    // 0x4c95c: ldr r6, [r7, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,1280)?)?;
    // 0x4c960: add r5, r3, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x4c964: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x4c968: ldr r4, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,-4)?)?;
    // 0x4c96c: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x4c970: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4c974: bic r4, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(4)?,R::new(4)?,16)?)?;
    // 0x4c978: orr r6, r4, r6, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,4,false)?)?;
    // 0x4c97c: str r6, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,-4)?)?;
    // 0x4c980: mov r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3840)?)?;
    // 0x4c984: ldr r6, [r7, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,1276)?)?;
    // 0x4c988: and r6, r5, r6, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,8,false)?)?;
    // 0x4c98c: ldrsb r5, [r7, r12]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::new(5)?,R::new(7)?,R::R12)?)?;
    // 0x4c990: add r5, r3, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x4c994: ldr r4, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,-4)?)?;
    // 0x4c998: bic r4, r4, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(4)?,R::new(4)?,3840)?)?;
    // 0x4c99c: orr r6, r4, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4c9a0: str r6, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,-4)?)?;
    // 0x4c9a4: mov r5, #61440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,61440)?)?;
    // 0x4c9a8: ldr r6, [r7, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,1284)?)?;
    // 0x4c9ac: and r6, r5, r6, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,12,false)?)?;
    // 0x4c9b0: ldrsb r5, [r7, r12]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::new(5)?,R::new(7)?,R::R12)?)?;
    // 0x4c9b4: add r5, r3, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x4c9b8: ldr r4, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,-4)?)?;
    // 0x4c9bc: bic r4, r4, #61440
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(4)?,R::new(4)?,61440)?)?;
    // 0x4c9c0: orr r6, r4, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4c9c4: str r6, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,-4)?)?;
    // 0x4c9c8: ldrsb r6, [r7, r12]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::new(6)?,R::new(7)?,R::R12)?)?;
    // 0x4c9cc: add r6, r3, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(3)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x4c9d0: ldr r5, [r6, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,-4)?)?;
    // 0x4c9d4: bic r5, r5, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,196608)?)?;
    // 0x4c9d8: str r5, [r6, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,-4)?)?;
    // 0x4c9dc: ldrsb r6, [r7, r12]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::new(6)?,R::new(7)?,R::R12)?)?;
    // 0x4c9e0: mov r7, #92
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,92)?)?;
    // 0x4c9e4: orr r7, r7, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,15360)?)?;
    // 0x4c9e8: add r3, r3, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x4c9ec: add r7, r7, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x4c9f0: ldreq r6, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R1,0)?)?;
    // 0x4c9f4: ldr r3, [r3, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,-4)?)?;
    // 0x4c9f8: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x4c9fc: streq r6, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R1,0)?)?;
    // 0x4ca00: str r7, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,40)?)?;
    // 0x4ca04: add r7, r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,44)?)?;
    // 0x4ca08: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x4ca0c: add r1, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x4ca10: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x4ca14: str r3, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,0)?)?;
    // 0x4ca18: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4ca1c: str r1, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x4ca20: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x4ca24: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4ca28: str r0, [sp, #0x47c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1148)?)?;
    // 0x4ca2c: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4ca30: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4ca34: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x4ca38: mov r2, #3232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3232)?)?;
    // 0x4ca3c: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x4ca40: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x4ca44: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x4ca48: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x4ca4c: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x4ca50: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4ca54: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x4ca58: str r1, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x4ca5c: ldrb r1, [r9, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,3272)?)?;
    // 0x4ca60: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4ca64: beq 0x4cb60 <entry+0xb60>   @ imm = #0xf4
    c.branch(C::Eq,0x4cb60)?;
    // 0x4ca68: ldr r1, [r9, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2744)?)?;
    // 0x4ca6c: ldrb r1, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,339)?)?;
    // 0x4ca70: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4ca74: bne 0x4cb60 <entry+0xb60>   @ imm = #0xe4
    c.branch(C::Ne,0x4cb60)?;
    // 0x4ca78: ldr r0, [r9, #0xccc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3276)?)?;
    // 0x4ca7c: ldr r1, [r9, #0xcd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,3288)?)?;
    // 0x4ca80: ldr r5, [r9, #0xcdc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,3292)?)?;
    // 0x4ca84: ldr r10, [r9, #0xce0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,3296)?)?;
    // 0x4ca88: sub r6, r11, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,36)?)?;
    // 0x4ca8c: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x4ca90: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x4ca94: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4ca98: str lr, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,16)?)?;
    // 0x4ca9c: mov r4, lr
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::LR))?;
    // 0x4caa0: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x4caa4: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x4caa8: add r7, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4caac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4cab0: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x4cab4: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x4cab8: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x4cabc: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xea98
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4cac0: ldr r0, [r9, #0xce4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3300)?)?;
    // 0x4cac4: add r1, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4cac8: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x4cacc: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x4cad0: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x4cad4: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x4cad8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4cadc: add r1, r1, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x4cae0: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x4cae4: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x4cae8: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4caec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4caf0: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xeacc
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4caf4: ldr r0, [r9, #0xccc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3276)?)?;
    // 0x4caf8: ldr r1, [r9, #0xcd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,3288)?)?;
    // 0x4cafc: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x4cb00: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x4cb04: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x4cb08: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x4cb0c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4cb10: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4cb14: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4cb18: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x4cb1c: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xeaf8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4cb20: ldr r0, [r9, #0xccc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3276)?)?;
    // 0x4cb24: ldr r1, [r9, #0xcd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,3288)?)?;
    // 0x4cb28: ldr r2, [r9, #0xcdc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,3292)?)?;
    // 0x4cb2c: ldr r3, [r9, #0xce0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,3296)?)?;
    // 0x4cb30: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x4cb34: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x4cb38: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x4cb3c: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4cb40: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x4cb44: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4cb48: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x4cb4c: stmib sp, {r0, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(3)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4cb50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4cb54: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4cb58: bl 0x3e02c <entry+0xffffffffffff202c> @ imm = #-0xeb34
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4cb5c: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x4cb60: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x4cb64: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x4cb68: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x4cb6c: b 0x476ac <entry+0xffffffffffffb6ac> @ imm = #-0x54c8
    c.branch(C::Al,0x476ac)?;
    // 0x4cb70: non-executable logging-string pointer, not ANDEQ.
    c.check(0x4cb70)?;
    c.emit(CONTROLLER_LOG_POINTER)?;
    // 0x4cb74: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x4cb78: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x4cb7c: sub sp, sp, #188
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,188)?)?;
    // 0x4cb80: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x4cb84: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x4cb88: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x4cb8c: ldr r1, [r0, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,696)?)?;
    // 0x4cb90: ldr lr, [r0, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,692)?)?;
    // 0x4cb94: ldr r10, [r0, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,716)?)?;
    // 0x4cb98: ldr r8, [r0, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,720)?)?;
    // 0x4cb9c: str r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,104)?)?;
    // 0x4cba0: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x4cba4: mov r12, r1
    c.emit(arm32::mov_register(C::Al,R::R12,R::R1))?;
    // 0x4cba8: ldrb r2, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,336)?)?;
    // 0x4cbac: ands r2, r2, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R2,1,true)?)?;
    // 0x4cbb0: movne r12, lr
    c.emit(arm32::mov_register(C::Ne,R::R12,R::LR))?;
    // 0x4cbb4: movne lr, r1
    c.emit(arm32::mov_register(C::Ne,R::LR,R::R1))?;
    // 0x4cbb8: lsl r1, r10, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(10)?,Shift::Lsl,3)?)?;
    // 0x4cbbc: cmp r1, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // 0x4cbc0: str r12, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,96)?)?;
    // 0x4cbc4: str lr, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,76)?)?;
    // 0x4cbc8: lslhs r1, r8, #3
    c.emit(arm32::mov_shifted_register(C::Cs,R::R1,R::new(8)?,Shift::Lsl,3)?)?;
    // 0x4cbcc: cmphs r1, r12
    c.emit(arm32::data_register(C::Cs,Op::Cmp,R::R0,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // 0x4cbd0: bhs 0x4cbf8 <entry+0xbf8>   @ imm = #0x20
    c.branch(C::Cs,0x4cbf8)?;
    // 0x4cbd4: ldr r2, [pc, #0xd20]        @ 0x18fc <entry+0x18fc>
    c.literal(C::Al,R::R2,0x4d8fc)?;
    // 0x4cbd8: mov r1, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    // 0x4cbdc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4cbe0: orr r1, r1, #10752
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10752)?)?;
    // 0x4cbe4: bl 0x17b0 <entry+0xfffffffffffb57b0> @ imm = #-0x4b43c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4cbe8: ldr r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,104)?)?;
    // 0x4cbec: ldr lr, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,76)?)?;
    // 0x4cbf0: ldr r12, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,96)?)?;
    // 0x4cbf4: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x4cbf8: ldrb r2, [r0, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,325)?)?;
    // 0x4cbfc: ldrb r3, [r0, #0x13e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,318)?)?;
    // 0x4cc00: cmp lr, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x4cc04: ldrb r7, [r0, #0x143]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R0,323)?)?;
    // 0x4cc08: ldrb r1, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,339)?)?;
    // 0x4cc0c: str r8, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,72)?)?;
    // 0x4cc10: and r6, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::R2,2)?)?;
    // 0x4cc14: and r4, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::R2,1)?)?;
    // 0x4cc18: movne r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x4cc1c: lsr r5, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Lsr,1)?)?;
    // 0x4cc20: cmp r12, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x4cc24: str r6, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,60)?)?;
    // 0x4cc28: movne r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x4cc2c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4cc30: str r4, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,48)?)?;
    // 0x4cc34: str r5, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,52)?)?;
    // 0x4cc38: orr r6, r4, r5, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(5)?,Shift::Lsl,1,false)?)?;
    // 0x4cc3c: mov r5, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,24)?)?;
    // 0x4cc40: and r3, r5, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x4cc44: mov r5, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x4cc48: and r7, r5, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(5)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x4cc4c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x4cc50: orr r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4cc54: orreq r7, r3, #96
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(7)?,R::new(3)?,96)?)?;
    // 0x4cc58: cmp r12, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x4cc5c: orr r3, r7, r6, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(6)?,Shift::Lsl,7,false)?)?;
    // 0x4cc60: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x4cc64: movne r7, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x4cc68: orr r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4cc6c: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x4cc70: str r7, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,56)?)?;
    // 0x4cc74: blo 0x4ccb0 <entry+0xcb0>   @ imm = #0x34
    c.branch(C::Cc,0x4ccb0)?;
    // 0x4cc78: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x4cc7c: bic r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x4cc80: and r4, r7, r2, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(7)?,R::R2,Shift::Lsr,4,false)?)?;
    // 0x4cc84: and r7, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R2,32)?)?;
    // 0x4cc88: orr r6, r4, r7, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(7)?,Shift::Lsr,4,false)?)?;
    // 0x4cc8c: str r4, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,80)?)?;
    // 0x4cc90: lsr r4, r7, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(7)?,Shift::Lsr,5)?)?;
    // 0x4cc94: and r7, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R2,8)?)?;
    // 0x4cc98: and r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,4)?)?;
    // 0x4cc9c: orr r3, r3, r6, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,9,false)?)?;
    // 0x4cca0: str r4, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,84)?)?;
    // 0x4cca4: lsr r8, r7, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::new(7)?,Shift::Lsr,3)?)?;
    // 0x4cca8: lsr r5, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R2,Shift::Lsr,2)?)?;
    // 0x4ccac: b 0x4ccf4 <entry+0xcf4>   @ imm = #0x40
    c.branch(C::Al,0x4ccf4)?;
    // 0x4ccb0: rsb r2, r8, r8, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R2,R::new(8)?,R::new(8)?,Shift::Lsl,3,false)?)?;
    // 0x4ccb4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x4ccb8: cmp r12, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4ccbc: rsb r2, r10, r10, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R2,R::new(10)?,R::new(10)?,Shift::Lsl,3,false)?)?;
    // 0x4ccc0: movhi r4, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x4ccc4: cmp lr, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4ccc8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4cccc: movhi r2, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4ccd0: cmp r12, r8, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(8)?,Shift::Lsl,2,false)?)?;
    // 0x4ccd4: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x4ccd8: str r4, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,84)?)?;
    // 0x4ccdc: str r2, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,80)?)?;
    // 0x4cce0: orr r2, r2, r4, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(4)?,Shift::Lsl,1,false)?)?;
    // 0x4cce4: movhi r8, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x4cce8: cmp lr, r10, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::new(10)?,Shift::Lsl,2,false)?)?;
    // 0x4ccec: orr r3, r3, r2, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,9,false)?)?;
    // 0x4ccf0: movhi r5, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x4ccf4: ldrb r0, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,336)?)?;
    // 0x4ccf8: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // 0x4ccfc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x4cd00: and r0, r2, r0, lsl #11
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,11,false)?)?;
    // 0x4cd04: lsr r2, lr, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::LR,Shift::Lsr,8)?)?;
    // 0x4cd08: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x4cd0c: bic r2, r3, #30720
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::new(3)?,30720)?)?;
    // 0x4cd10: movhi r7, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x4cd14: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4cd18: orr r2, r2, r7, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,12,false)?)?;
    // 0x4cd1c: ldr r7, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,0)?)?;
    // 0x4cd20: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4cd24: orrne r0, r0, #16384
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,16384)?)?;
    // 0x4cd28: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x4cd2c: ldreq r1, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(7)?,0)?)?;
    // 0x4cd30: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4cd34: streq r1, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(7)?,0)?)?;
    // 0x4cd38: mov r1, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2064)?)?;
    // 0x4cd3c: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x4cd40: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // 0x4cd44: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x4cd48: str r9, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,100)?)?;
    // 0x4cd4c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x4cd50: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // 0x4cd54: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4cd58: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4cd5c: add r0, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,8)?)?;
    // 0x4cd60: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x4cd64: ldr r2, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,104)?)?;
    // 0x4cd68: ldrb r1, [r2, #0x3db]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,987)?)?;
    // 0x4cd6c: ldr r2, [r2, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,2744)?)?;
    // 0x4cd70: ldr r3, [r2, #0x520]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,1312)?)?;
    // 0x4cd74: cmp r3, #25
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,25)?)?;
    // 0x4cd78: beq 0x4cd84 <entry+0xd84>   @ imm = #0x4
    c.branch(C::Eq,0x4cd84)?;
    // 0x4cd7c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4cd80: beq 0x4d8e4 <entry+0x18e4>  @ imm = #0xb5c
    c.branch(C::Eq,0x4d8e4)?;
    // 0x4cd84: and r3, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R1,1)?)?;
    // 0x4cd88: ldr r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x4cd8c: ldr r6, [pc, #0xfc4]        @ 0x1d58 <entry+0x1d58>
    c.literal(C::Al,R::new(6)?,0x4dd58)?;
    // 0x4cd90: ldr r1, [r1, #0x30c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,780)?)?;
    // 0x4cd94: and r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,15)?)?;
    // 0x4cd98: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x4cd9c: addlo r6, r6, r1
    c.emit(arm32::data_register(C::Cc,Op::Add,R::new(6)?,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4cda0: ldrb r1, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x4cda4: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x4cda8: and r1, r6, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(6)?,R::R1,Shift::Lsl,1,false)?)?;
    // 0x4cdac: orr r4, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4cdb0: ldrb r3, [r2, #0x143]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,323)?)?;
    // 0x4cdb4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x4cdb8: beq 0x4cdc4 <entry+0xdc4>   @ imm = #0x4
    c.branch(C::Eq,0x4cdc4)?;
    // 0x4cdbc: orr r4, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,16)?)?;
    // 0x4cdc0: b 0x4cde4 <entry+0xde4>   @ imm = #0x1c
    c.branch(C::Al,0x4cde4)?;
    // 0x4cdc4: ldr r3, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,72)?)?;
    // 0x4cdc8: cmp r12, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x4cdcc: lslls r3, r10, #1
    c.emit(arm32::mov_shifted_register(C::Ls,R::new(3)?,R::new(10)?,Shift::Lsl,1)?)?;
    // 0x4cdd0: cmpls lr, r3
    c.emit(arm32::data_register(C::Ls,Op::Cmp,R::R0,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4cdd4: bhi 0x4cdbc <entry+0xdbc>   @ imm = #-0x20
    c.branch(C::Hi,0x4cdbc)?;
    // 0x4cdd8: ldrb r2, [r2, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,336)?)?;
    // 0x4cddc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4cde0: orrne r4, r4, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,16)?)?;
    // 0x4cde4: str r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,24)?)?;
    // 0x4cde8: ldr r4, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,104)?)?;
    // 0x4cdec: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x4cdf0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4cdf4: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x4cdf8: orr r6, r6, #16128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,16128)?)?;
    // 0x4cdfc: ldr r2, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,716)?)?;
    // 0x4ce00: ldr r3, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,720)?)?;
    // 0x4ce04: and r2, r2, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4ce08: mov r6, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,16711680)?)?;
    // 0x4ce0c: orr r6, r6, #1056964608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1056964608)?)?;
    // 0x4ce10: and r3, r6, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x4ce14: add r6, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,16)?)?;
    // 0x4ce18: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4ce1c: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x4ce20: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4ce24: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x4ce28: mov r0, #10432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10432)?)?;
    // 0x4ce2c: str r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x4ce30: str r2, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,12)?)?;
    // 0x4ce34: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4ce38: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x4ce3c: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // 0x4ce40: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4ce44: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x4ce48: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x4ce4c: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x4ce50: str r5, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,92)?)?;
    // 0x4ce54: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x4ce58: lsl r0, r5, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsl,6)?)?;
    // 0x4ce5c: orr r3, r0, r8, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::new(8)?,Shift::Lsl,7,false)?)?;
    // 0x4ce60: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x4ce64: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4ce68: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x4ce6c: ldrb r5, [r0, #0x151]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R0,337)?)?;
    // 0x4ce70: and r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4ce74: orr r3, r3, r5, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,25,false)?)?;
    // 0x4ce78: ldrb r5, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,988)?)?;
    // 0x4ce7c: and r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4ce80: orr r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4ce84: orr r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4ce88: orr r1, r1, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,2,false)?)?;
    // 0x4ce8c: bne 0x4ce9c <entry+0xe9c>   @ imm = #0x8
    c.branch(C::Ne,0x4ce9c)?;
    // 0x4ce90: ldr r2, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,92)?)?;
    // 0x4ce94: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x4ce98: bne 0x4cea4 <entry+0xea4>   @ imm = #0x4
    c.branch(C::Ne,0x4cea4)?;
    // 0x4ce9c: orr r5, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,32)?)?;
    // 0x4cea0: b 0x4cea8 <entry+0xea8>   @ imm = #0x0
    c.branch(C::Al,0x4cea8)?;
    // 0x4cea4: orr r5, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,16)?)?;
    // 0x4cea8: ldrb r2, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,339)?)?;
    // 0x4ceac: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x4ceb0: mov r3, #1044480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1044480)?)?;
    // 0x4ceb4: orr r4, r4, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4261412864)?)?;
    // 0x4ceb8: orr r3, r3, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,32505856)?)?;
    // 0x4cebc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4cec0: beq 0x4cf04 <entry+0xf04>   @ imm = #0x3c
    c.branch(C::Eq,0x4cf04)?;
    // 0x4cec4: ldrb r0, [r0, #0x154]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,340)?)?;
    // 0x4cec8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x4cecc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x4ced0: orrhi r0, r1, #48
    c.emit(arm32::data_immediate(C::Hi,Op::Orr,R::R0,R::R1,48)?)?;
    // 0x4ced4: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x4ced8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4cedc: ldr r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x4cee0: movne r5, r0
    c.emit(arm32::mov_register(C::Ne,R::new(5)?,R::R0))?;
    // 0x4cee4: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x4cee8: movne r5, r0
    c.emit(arm32::mov_register(C::Ne,R::new(5)?,R::R0))?;
    // 0x4ceec: and r0, r5, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4cef0: ldr r1, [r1, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,716)?)?;
    // 0x4cef4: and r1, r3, r1, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,12,false)?)?;
    // 0x4cef8: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4cefc: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x4cf00: b 0x4cf58 <entry+0xf58>   @ imm = #0x50
    c.branch(C::Al,0x4cf58)?;
    // 0x4cf04: ldr r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,104)?)?;
    // 0x4cf08: ldr r4, [r0, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,928)?)?;
    // 0x4cf0c: ldr r9, [r0, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R0,716)?)?;
    // 0x4cf10: add r0, r10, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4cf14: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x4cf18: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x4cf1c: blx 0x83c4c <entry+0x37c4c> @ imm = #0x36d28
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x4cf20: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4cf24: mov r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3840)?)?;
    // 0x4cf28: ldr lr, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,76)?)?;
    // 0x4cf2c: ldr r12, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,96)?)?;
    // 0x4cf30: orr r1, r1, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4261412864)?)?;
    // 0x4cf34: and r0, r2, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,8,false)?)?;
    // 0x4cf38: and r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4cf3c: cmp r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    // 0x4cf40: orr r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4cf44: mov r0, #1044480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1044480)?)?;
    // 0x4cf48: movne r9, r4
    c.emit(arm32::mov_register(C::Ne,R::new(9)?,R::new(4)?))?;
    // 0x4cf4c: orr r0, r0, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32505856)?)?;
    // 0x4cf50: and r0, r0, r9, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,12,false)?)?;
    // 0x4cf54: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4cf58: str r8, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,88)?)?;
    // 0x4cf5c: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x4cf60: add r4, r7, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,32)?)?;
    // 0x4cf64: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x4cf68: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4cf6c: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x4cf70: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x4cf74: rsb r3, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(3)?,R::R1,0)?)?;
    // 0x4cf78: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x4cf7c: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4cf80: str r1, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,16)?)?;
    // 0x4cf84: str r0, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,20)?)?;
    // 0x4cf88: add r1, r7, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,24)?)?;
    // 0x4cf8c: ldr r8, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,68)?)?;
    // 0x4cf90: ldr r9, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,100)?)?;
    // 0x4cf94: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4cf98: add r0, r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,3)?)?;
    // 0x4cf9c: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4cfa0: str r1, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x4cfa4: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4cfa8: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4cfac: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x4cfb0: and r0, lr, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::LR,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4cfb4: orr r0, r0, r12, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R12,Shift::Lsl,16,false)?)?;
    // 0x4cfb8: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4cfbc: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x4cfc0: ldr r1, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,88)?)?;
    // 0x4cfc4: rsb r1, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R1,0)?)?;
    // 0x4cfc8: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // 0x4cfcc: mov r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // 0x4cfd0: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4cfd4: str r1, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,24)?)?;
    // 0x4cfd8: str r0, [r7, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,28)?)?;
    // 0x4cfdc: add r0, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,4)?)?;
    // 0x4cfe0: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x4cfe4: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x4cfe8: str r4, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,0)?)?;
    // 0x4cfec: str r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,28)?)?;
    // 0x4cff0: str r10, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,40)?)?;
    // 0x4cff4: ldr r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,80)?)?;
    // 0x4cff8: rsb r0, r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,18)?)?;
    // 0x4cffc: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
 c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();emit_controller_code(&mut c)?;Ok(c.bytes)
}
#[cfg(test)] mod tests {
 use super::*;
 #[test] fn verified_typed_controller_span_is_complete(){
  assert_eq!(fragment().unwrap().len(),END-START);
  let bytes=fragment().unwrap();
  let at=0x4cb70-START;
  assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),CONTROLLER_LOG_POINTER);
  assert_eq!(u32::from_le_bytes(bytes[0x4cb6c-START..0x4cb70-START].try_into().unwrap()),arm32::branch(C::Al,0x4cb6c,0x476ac,false).unwrap());
 }
}
