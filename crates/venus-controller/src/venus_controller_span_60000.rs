// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x60000,0x61000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x60000;
pub const END:usize=0x61000;
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
 // Executable A32 block 0x60000..0x60178.
 c.check(0x60000)?;
    // 0x60000: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x60004: mov r3, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,22)?)?;
    // 0x60008: stm sp, {r2, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x6000c: b 0x60058 <entry+0xa858>  @ imm = #0x44
    c.emit(arm32::branch(C::Al,c.at as u32,0x60058,false)?)?;
    // 0x60010: stm sp, {r3, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x60014: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x60018: ldr r1, [pc, #0x188]        @ 0xa9a8 <entry+0xa9a8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,392)?)?;
    // 0x6001c: ldr r2, [pc, #0x160]        @ 0xa984 <entry+0xa984>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,352)?)?;
    // 0x60020: mov r3, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,25)?)?;
    // 0x60024: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x60028: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x6002c: orr r3, r3, #8704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8704)?)?;
    // 0x60030: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x313a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60034: ldr r1, [pc, #0x140]        @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,320)?)?;
    // 0x60038: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x6003c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x313b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60040: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x60044: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60048: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x6004c: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x60050: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x60054: str r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x60058: str r6, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // 0x6005c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x60060: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x60064: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x356dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x60068: b 0x60158 <entry+0xa958>  @ imm = #0xe8
    c.emit(arm32::branch(C::Al,c.at as u32,0x60158,false)?)?;
    // 0x6006c: ldr r1, [pc, #0x120]        @ 0xa994 <entry+0xa994>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,288)?)?;
    // 0x60070: ldr r2, [pc, #0x10c]        @ 0xa984 <entry+0xa984>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,268)?)?;
    // 0x60074: mov r3, #101
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,101)?)?;
    // 0x60078: orr r3, r3, #8704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8704)?)?;
    // 0x6007c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60080: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x313f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60084: ldr r1, [pc, #0xf0]         @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,240)?)?;
    // 0x60088: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x6008c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31400
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60090: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x60094: b 0x60104 <entry+0xa904>  @ imm = #0x68
    c.emit(arm32::branch(C::Al,c.at as u32,0x60104,false)?)?;
    // 0x60098: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6009c: bne 0x5fe20 <entry+0xa620>  @ imm = #-0x284
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5fe20,false)?)?;
    // 0x600a0: ldr r1, [pc, #0xe4]         @ 0xa98c <entry+0xa98c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,228)?)?;
    // 0x600a4: ldr r2, [pc, #0xd8]         @ 0xa984 <entry+0xa984>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,216)?)?;
    // 0x600a8: mov r3, #47
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,47)?)?;
    // 0x600ac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x600b0: orr r3, r3, #8704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8704)?)?;
    // 0x600b4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31428
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x600b8: ldr r1, [pc, #0xbc]         @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,188)?)?;
    // 0x600bc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x600c0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31434
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x600c4: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x600c8: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x600cc: ldr r2, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,56)?)?;
    // 0x600d0: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x600d4: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x600d8: b 0x60008 <entry+0xa808>  @ imm = #-0xd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x60008,false)?)?;
    // 0x600dc: ldr r1, [pc, #0xac]         @ 0xa990 <entry+0xa990>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,172)?)?;
    // 0x600e0: ldr r2, [pc, #0x9c]         @ 0xa984 <entry+0xa984>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,156)?)?;
    // 0x600e4: mov r3, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,106)?)?;
    // 0x600e8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x600ec: orr r3, r3, #8704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8704)?)?;
    // 0x600f0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31464
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x600f4: ldr r1, [pc, #0x80]         @ 0xa97c <entry+0xa97c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,128)?)?;
    // 0x600f8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x600fc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31470
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60100: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x60104: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x60108: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x6010c: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x60110: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x60114: ldr r0, [pc, #0x84]         @ 0xa9a0 <entry+0xa9a0>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,132)?)?;
    // 0x60118: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x6011c: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x60120: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x60124: ldr r0, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,52)?)?;
    // 0x60128: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x6012c: mov r0, #80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,80)?)?;
    // 0x60130: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x60134: ldr r0, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,40)?)?;
    // 0x60138: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x6013c: ldr r0, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x60140: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x60144: ldr r0, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,48)?)?;
    // 0x60148: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x6014c: str r6, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,92)?)?;
    // 0x60150: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60154: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x358e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x60158: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x6015c: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x60160: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x60164: ldr r1, [pc, #0x30]         @ 0xa99c <entry+0xa99c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,48)?)?;
    // 0x60168: ldr r2, [pc, #0x14]         @ 0xa984 <entry+0xa984>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,20)?)?;
    // 0x6016c: mov r3, #608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,608)?)?;
    // 0x60170: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x60174: b 0x6007c <entry+0xa87c>  @ imm = #-0x100
    c.emit(arm32::branch(C::Al,c.at as u32,0x6007c,false)?)?;
 c.check(0x60178)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_60178:[u32;13]=[0x000f0a35,0x000e2788,0x000f0a4b,0x000f0a7c,0x000f0aff,0x000f0b51,0x000f0c0a,0x000f0be3,0x000f0b82,0x000f0bbe,0x00221007,0x000f0bac,0x000f0a91];
 for value in INLINE_DATA_60178{c.emit(value)?;}
 // Executable A32 block 0x601ac..0x604f8.
 c.check(0x601ac)?;
    // 0x601ac: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x601b0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x601b4: sub sp, sp, #96
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,96)?)?;
    // 0x601b8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x601bc: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x601c0: ldr r6, [r4, #0x63c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1596)?)?;
    // 0x601c4: bl 0x26754 <entry+0xfffffffffffd0f54> @ imm = #-0x39a78
    c.emit(arm32::branch(C::Al,c.at as u32,0x26754,true)?)?;
    // 0x601c8: ldr r3, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,32)?)?;
    // 0x601cc: ldr r1, [pc, #0x324]        @ 0xacf8 <entry+0xacf8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,804)?)?;
    // 0x601d0: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x601d4: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x601d8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3154c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x601dc: ldr r1, [pc, #0x318]        @ 0xacfc <entry+0xacfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,792)?)?;
    // 0x601e0: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x601e4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31558
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x601e8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x601ec: cmp r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,44)?)?;
    // 0x601f0: bne 0x6026c <entry+0xaa6c>  @ imm = #0x74
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6026c,false)?)?;
    // 0x601f4: ldr r0, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,20)?)?;
    // 0x601f8: ldr r1, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,16)?)?;
    // 0x601fc: subs r2, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R0,R::new(6)?,Shift::Lsl,0,true)?)?;
    // 0x60200: blo 0x602c8 <entry+0xaac8>  @ imm = #0xc0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x602c8,false)?)?;
    // 0x60204: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x60208: bhi 0x602c8 <entry+0xaac8>  @ imm = #0xb8
    c.emit(arm32::branch(C::Hi,c.at as u32,0x602c8,false)?)?;
    // 0x6020c: ldrb r0, [r4, #0x57b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1403)?)?;
    // 0x60210: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x60214: beq 0x6032c <entry+0xab2c>  @ imm = #0x110
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6032c,false)?)?;
    // 0x60218: ldr r1, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,36)?)?;
    // 0x6021c: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x60220: orr r0, r0, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3840)?)?;
    // 0x60224: tst r1, r0
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x60228: beq 0x60314 <entry+0xab14>  @ imm = #0xe4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60314,false)?)?;
    // 0x6022c: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x60230: ldr r1, [pc, #0x2d0]        @ 0xad08 <entry+0xad08>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,720)?)?;
    // 0x60234: ldr r2, [pc, #0x2c8]        @ 0xad04 <entry+0xad04>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,712)?)?;
    // 0x60238: mov r3, #161
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,161)?)?;
    // 0x6023c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60240: orr r3, r3, #8704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8704)?)?;
    // 0x60244: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x315b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60248: ldr r1, [pc, #0x2ac]        @ 0xacfc <entry+0xacfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,684)?)?;
    // 0x6024c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60250: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x315c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60254: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x60258: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x6025c: ldr r2, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,36)?)?;
    // 0x60260: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x60264: mov r3, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,22)?)?;
    // 0x60268: b 0x60498 <entry+0xac98>  @ imm = #0x228
    c.emit(arm32::branch(C::Al,c.at as u32,0x60498,false)?)?;
    // 0x6026c: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x60270: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60274: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x60278: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x6027c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x60280: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x60284: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x60288: str r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x6028c: str r6, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // 0x60290: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x35908
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x60294: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x60298: mov r1, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,44)?)?;
    // 0x6029c: ldr r2, [pc, #0x260]        @ 0xad04 <entry+0xad04>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,608)?)?;
    // 0x602a0: mov r3, #143
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,143)?)?;
    // 0x602a4: orr r3, r3, #8704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8704)?)?;
    // 0x602a8: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x602ac: ldr r1, [pc, #0x24c]        @ 0xad00 <entry+0xad00>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,588)?)?;
    // 0x602b0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x602b4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31628
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x602b8: ldr r1, [pc, #0x23c]        @ 0xacfc <entry+0xacfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,572)?)?;
    // 0x602bc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x602c0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31634
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x602c4: b 0x604ac <entry+0xacac>  @ imm = #0x1e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x604ac,false)?)?;
    // 0x602c8: stm sp, {r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x602cc: ldr r1, [pc, #0x248]        @ 0xad1c <entry+0xad1c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,584)?)?;
    // 0x602d0: ldr r2, [pc, #0x22c]        @ 0xad04 <entry+0xad04>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,556)?)?;
    // 0x602d4: mov r3, #150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,150)?)?;
    // 0x602d8: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x602dc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x602e0: str r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // 0x602e4: orr r3, r3, #8704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8704)?)?;
    // 0x602e8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3165c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x602ec: ldr r1, [pc, #0x208]        @ 0xacfc <entry+0xacfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,520)?)?;
    // 0x602f0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x602f4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31668
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x602f8: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x602fc: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60300: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x60304: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x60308: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x6030c: str r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x60310: b 0x6049c <entry+0xac9c>  @ imm = #0x184
    c.emit(arm32::branch(C::Al,c.at as u32,0x6049c,false)?)?;
    // 0x60314: ldr r0, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1512)?)?;
    // 0x60318: ldr r2, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,40)?)?;
    // 0x6031c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x60320: beq 0x60458 <entry+0xac58>  @ imm = #0x130
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60458,false)?)?;
    // 0x60324: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x60328: blo 0x60458 <entry+0xac58>  @ imm = #0x128
    c.emit(arm32::branch(C::Cc,c.at as u32,0x60458,false)?)?;
    // 0x6032c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60330: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x4ba4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x60334: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60338: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x6033c: ldrne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,0)?)?;
    // 0x60340: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x60344: bne 0x603bc <entry+0xabbc>  @ imm = #0x70
    c.emit(arm32::branch(C::Ne,c.at as u32,0x603bc,false)?)?;
    // 0x60348: ldr r2, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1512)?)?;
    // 0x6034c: ldr r1, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1592)?)?;
    // 0x60350: ldr r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,124)?)?;
    // 0x60354: ldr r3, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,32)?)?;
    // 0x60358: ldrb lr, [r4, #0x57b]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(4)?,1403)?)?;
    // 0x6035c: ldr r12, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(5)?,28)?)?;
    // 0x60360: ldr r6, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,36)?)?;
    // 0x60364: stm sp, {r6, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x60368: str r12, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,8)?)?;
    // 0x6036c: bl 0x7edc8 <entry+0x295c8> @ imm = #0x1ea54
    c.emit(arm32::branch(C::Al,c.at as u32,0x7edc8,true)?)?;
    // 0x60370: ldr r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // 0x60374: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x60378: str r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // 0x6037c: ldrb r0, [r4, #0x350]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,848)?)?;
    // 0x60380: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x60384: beq 0x603b0 <entry+0xabb0>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x603b0,false)?)?;
    // 0x60388: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x6038c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x60390: bne 0x603a8 <entry+0xaba8>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x603a8,false)?)?;
    // 0x60394: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60398: orr r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // 0x6039c: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x603a0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x603a4: bne 0x604b8 <entry+0xacb8>  @ imm = #0x10c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x604b8,false)?)?;
    // 0x603a8: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x603ac: b 0x604bc <entry+0xacbc>  @ imm = #0x108
    c.emit(arm32::branch(C::Al,c.at as u32,0x604bc,false)?)?;
    // 0x603b0: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x603b4: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x603b8: b 0x604c0 <entry+0xacc0>  @ imm = #0x100
    c.emit(arm32::branch(C::Al,c.at as u32,0x604c0,false)?)?;
    // 0x603bc: ldr r0, [pc, #0x14c]        @ 0xad10 <entry+0xad10>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,332)?)?;
    // 0x603c0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x603c4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x603c8: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x603cc: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x603d0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x603d4: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x603d8: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x603dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x603e0: ldrne r1, [r4, #0x63c]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,1596)?)?;
    // 0x603e4: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // 0x603e8: str r6, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,68)?)?;
    // 0x603ec: str r6, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,60)?)?;
    // 0x603f0: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x603f4: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x603f8: mov r1, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,76)?)?;
    // 0x603fc: str r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,32)?)?;
    // 0x60400: str r6, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,36)?)?;
    // 0x60404: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x60408: str r6, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,44)?)?;
    // 0x6040c: str r6, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,72)?)?;
    // 0x60410: ldr r1, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,28)?)?;
    // 0x60414: str r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,76)?)?;
    // 0x60418: str r6, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,52)?)?;
    // 0x6041c: str r6, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,48)?)?;
    // 0x60420: str r6, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,80)?)?;
    // 0x60424: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x60428: add r1, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,20)?)?;
    // 0x6042c: ldr r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,36)?)?;
    // 0x60430: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x60434: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60438: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x35bc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x6043c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60440: ldr r1, [pc, #0xcc]         @ 0xad14 <entry+0xad14>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,204)?)?;
    // 0x60444: ldr r2, [pc, #0xb8]         @ 0xad04 <entry+0xad04>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,184)?)?;
    // 0x60448: mov r3, #764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,764)?)?;
    // 0x6044c: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x60450: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x60454: b 0x602b0 <entry+0xaab0>  @ imm = #-0x1ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x602b0,false)?)?;
    // 0x60458: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x6045c: ldr r1, [pc, #0xa8]         @ 0xad0c <entry+0xad0c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,168)?)?;
    // 0x60460: ldr r2, [pc, #0x9c]         @ 0xad04 <entry+0xad04>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,156)?)?;
    // 0x60464: mov r3, #684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,684)?)?;
    // 0x60468: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x6046c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60470: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x60474: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x317e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60478: ldr r1, [pc, #0x7c]         @ 0xacfc <entry+0xacfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,124)?)?;
    // 0x6047c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60480: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x317f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60484: ldr r2, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,36)?)?;
    // 0x60488: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x6048c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60490: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x60494: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x60498: stm sp, {r2, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x6049c: str r6, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // 0x604a0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x604a4: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x604a8: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x35b20
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x604ac: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x604b0: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x604b4: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x604b8: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x604bc: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x604c0: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5db9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x604c4: mov r0, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1020)?)?;
    // 0x604c8: ldr r1, [pc, #0x48]         @ 0xad18 <entry+0xad18>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,72)?)?;
    // 0x604cc: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x604d0: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x604d4: ldr r2, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x604d8: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // 0x604dc: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x604e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x604e4: ldr r3, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,32)?)?;
    // 0x604e8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3185c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x604ec: ldr r1, [pc, #0x8]          @ 0xacfc <entry+0xacfc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,8)?)?;
    // 0x604f0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x604f4: b 0x602c0 <entry+0xaac0>  @ imm = #-0x23c
    c.emit(arm32::branch(C::Al,c.at as u32,0x602c0,false)?)?;
 c.check(0x604f8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_604F8:[u32;10]=[0x000f0c30,0x000e2788,0x000f0c46,0x000f0c8f,0x000f0d11,0x000f0d62,0x00221008,0x000f0df1,0x000f0dba,0x000f0ca3];
 for value in INLINE_DATA_604F8{c.emit(value)?;}
 // Executable A32 block 0x60520..0x60720.
 c.check(0x60520)?;
    // 0x60520: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x60524: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x60528: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x6052c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x60530: ldr r1, [pc, #0x1e8]        @ 0xaf20 <entry+0xaf20>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,488)?)?;
    // 0x60534: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x60538: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x6053c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x318b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60540: ldr r1, [pc, #0x1dc]        @ 0xaf24 <entry+0xaf24>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,476)?)?;
    // 0x60544: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60548: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x318bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x6054c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x60550: cmp r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,12)?)?;
    // 0x60554: bne 0x60650 <entry+0xae50>  @ imm = #0xf4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60650,false)?)?;
    // 0x60558: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x6055c: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x4dd0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x60560: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x60564: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x60568: ldr r0, [pc, #0x1c0]        @ 0xaf30 <entry+0xaf30>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,448)?)?;
    // 0x6056c: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x60570: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x60574: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x60578: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x6057c: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x60580: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60584: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x60588: beq 0x606a0 <entry+0xaea0>  @ imm = #0x110
    c.emit(arm32::branch(C::Eq,c.at as u32,0x606a0,false)?)?;
    // 0x6058c: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x60590: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x60594: bne 0x606cc <entry+0xaecc>  @ imm = #0x130
    c.emit(arm32::branch(C::Ne,c.at as u32,0x606cc,false)?)?;
    // 0x60598: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6059c: strb r0, [r4, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x605a0: ldrb r1, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,990)?)?;
    // 0x605a4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x605a8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x605ac: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x605b0: strb r1, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,13)?)?;
    // 0x605b4: ldr r1, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,280)?)?;
    // 0x605b8: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x605bc: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x605c0: strb r0, [r4, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,14)?)?;
    // 0x605c4: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x605c8: bl 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x5dc94
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,true)?)?;
    // 0x605cc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x605d0: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x605d4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x605d8: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x605dc: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // 0x605e0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x605e4: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5dcc0
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x605e8: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x605ec: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x605f0: bne 0x60600 <entry+0xae00>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60600,false)?)?;
    // 0x605f4: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x605f8: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x605fc: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5dcd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60600: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x60604: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60608: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5dce4
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x6060c: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60610: bl 0x294c <entry+0xfffffffffffad14c> @ imm = #-0x5dccc
    c.emit(arm32::branch(C::Al,c.at as u32,0x294c,true)?)?;
    // 0x60614: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60618: bl 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x5dce4
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,true)?)?;
    // 0x6061c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60620: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x60624: beq 0x60704 <entry+0xaf04>  @ imm = #0xd8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60704,false)?)?;
    // 0x60628: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x6062c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x60630: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x60634: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x60638: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5dd14
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x6063c: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60640: bl 0x294c <entry+0xfffffffffffad14c> @ imm = #-0x5dcfc
    c.emit(arm32::branch(C::Al,c.at as u32,0x294c,true)?)?;
    // 0x60644: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60648: bl 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x5dd14
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,true)?)?;
    // 0x6064c: b 0x60704 <entry+0xaf04>  @ imm = #0xb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x60704,false)?)?;
    // 0x60650: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x60654: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60658: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x6065c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x60660: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x60664: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x60668: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x6066c: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x60670: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x60674: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x35cec
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x60678: ldr r1, [pc, #0xa8]         @ 0xaf28 <entry+0xaf28>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,168)?)?;
    // 0x6067c: ldr r2, [pc, #0xa8]         @ 0xaf2c <entry+0xaf2c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,168)?)?;
    // 0x60680: mov r3, #784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,784)?)?;
    // 0x60684: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60688: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x6068c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31a00
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60690: ldr r1, [pc, #0x8c]         @ 0xaf24 <entry+0xaf24>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,140)?)?;
    // 0x60694: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60698: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31a0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x6069c: b 0x60714 <entry+0xaf14>  @ imm = #0x70
    c.emit(arm32::branch(C::Al,c.at as u32,0x60714,false)?)?;
    // 0x606a0: ldr r1, [pc, #0x8c]         @ 0xaf34 <entry+0xaf34>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,140)?)?;
    // 0x606a4: ldr r2, [pc, #0x80]         @ 0xaf2c <entry+0xaf2c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,128)?)?;
    // 0x606a8: mov r3, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,63)?)?;
    // 0x606ac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x606b0: orr r3, r3, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8960)?)?;
    // 0x606b4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31a28
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x606b8: ldr r1, [pc, #0x64]         @ 0xaf24 <entry+0xaf24>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,100)?)?;
    // 0x606bc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x606c0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31a34
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x606c4: ldr r0, [pc, #0x6c]         @ 0xaf38 <entry+0xaf38>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,108)?)?;
    // 0x606c8: b 0x60700 <entry+0xaf00>  @ imm = #0x30
    c.emit(arm32::branch(C::Al,c.at as u32,0x60700,false)?)?;
    // 0x606cc: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x606d0: ldr r1, [pc, #0x64]         @ 0xaf3c <entry+0xaf3c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,100)?)?;
    // 0x606d4: ldr r2, [pc, #0x50]         @ 0xaf2c <entry+0xaf2c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,80)?)?;
    // 0x606d8: mov r3, #836
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,836)?)?;
    // 0x606dc: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x606e0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x606e4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x606e8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31a5c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x606ec: ldr r1, [pc, #0x30]         @ 0xaf24 <entry+0xaf24>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,48)?)?;
    // 0x606f0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x606f4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31a68
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x606f8: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x606fc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x60700: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x60704: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60708: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x6070c: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x35e9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x60710: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x60714: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60718: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x6071c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x60720)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_60720:[u32;8]=[0x000f0e1a,0x000e2788,0x000f0e1f,0x000f0e48,0x00221003,0x000f0e59,0x01001002,0x000f0e85];
 for value in INLINE_DATA_60720{c.emit(value)?;}
 // Executable A32 block 0x60740..0x60974.
 c.check(0x60740)?;
    // 0x60740: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x60744: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x60748: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x6074c: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x60750: ldr r1, [pc, #0x21c]        @ 0xb174 <entry+0xb174>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,540)?)?;
    // 0x60754: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x60758: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x6075c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31ad0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60760: ldr r1, [pc, #0x210]        @ 0xb178 <entry+0xb178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,528)?)?;
    // 0x60764: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60768: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31adc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x6076c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60770: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x60774: bne 0x607d0 <entry+0xafd0>  @ imm = #0x54
    c.emit(arm32::branch(C::Ne,c.at as u32,0x607d0,false)?)?;
    // 0x60778: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x6077c: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x4ff0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x60780: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x60784: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x60788: ldrne r0, [r5]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,0)?)?;
    // 0x6078c: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x60790: bne 0x6082c <entry+0xb02c>  @ imm = #0x94
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6082c,false)?)?;
    // 0x60794: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x60798: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x6079c: orr r1, r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16777216)?)?;
    // 0x607a0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x607a4: beq 0x60864 <entry+0xb064>  @ imm = #0xb8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60864,false)?)?;
    // 0x607a8: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x607ac: orr r1, r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16777216)?)?;
    // 0x607b0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x607b4: beq 0x6086c <entry+0xb06c>  @ imm = #0xb0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6086c,false)?)?;
    // 0x607b8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x607bc: orr r1, r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16777216)?)?;
    // 0x607c0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x607c4: bne 0x6087c <entry+0xb07c>  @ imm = #0xb0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6087c,false)?)?;
    // 0x607c8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x607cc: b 0x60870 <entry+0xb070>  @ imm = #0x9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x60870,false)?)?;
    // 0x607d0: ldr r1, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,20)?)?;
    // 0x607d4: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x607d8: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x607dc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x607e0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x607e4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x607e8: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x607ec: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x607f0: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x607f4: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x35e6c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x607f8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x607fc: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x60800: ldr r2, [pc, #0x178]        @ 0xb180 <entry+0xb180>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,376)?)?;
    // 0x60804: mov r3, #91
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,91)?)?;
    // 0x60808: orr r3, r3, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8960)?)?;
    // 0x6080c: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x60810: ldr r1, [pc, #0x164]        @ 0xb17c <entry+0xb17c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,356)?)?;
    // 0x60814: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60818: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31b8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x6081c: ldr r1, [pc, #0x154]        @ 0xb178 <entry+0xb178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,340)?)?;
    // 0x60820: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60824: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31b98
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60828: b 0x60968 <entry+0xb168>  @ imm = #0x138
    c.emit(arm32::branch(C::Al,c.at as u32,0x60968,false)?)?;
    // 0x6082c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x60830: ldr r1, [pc, #0x14c]        @ 0xb184 <entry+0xb184>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,332)?)?;
    // 0x60834: ldr r2, [pc, #0x144]        @ 0xb180 <entry+0xb180>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,324)?)?;
    // 0x60838: mov r3, #133
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,133)?)?;
    // 0x6083c: orr r3, r3, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8960)?)?;
    // 0x60840: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x60844: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60848: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31bbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x6084c: ldr r1, [pc, #0x124]        @ 0xb178 <entry+0xb178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,292)?)?;
    // 0x60850: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60854: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31bc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60858: mov r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,6)?)?;
    // 0x6085c: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x60860: b 0x6092c <entry+0xb12c>  @ imm = #0xc4
    c.emit(arm32::branch(C::Al,c.at as u32,0x6092c,false)?)?;
    // 0x60864: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x60868: b 0x60870 <entry+0xb070>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x60870,false)?)?;
    // 0x6086c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60870: str r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,8)?)?;
    // 0x60874: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x60878: b 0x608ac <entry+0xb0ac>  @ imm = #0x2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x608ac,false)?)?;
    // 0x6087c: ldr r1, [pc, #0x104]        @ 0xb188 <entry+0xb188>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,260)?)?;
    // 0x60880: ldr r2, [pc, #0xf8]         @ 0xb180 <entry+0xb180>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,248)?)?;
    // 0x60884: mov r3, #111
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,111)?)?;
    // 0x60888: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x6088c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60890: orr r3, r3, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8960)?)?;
    // 0x60894: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31c08
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60898: ldr r1, [pc, #0xd8]         @ 0xb178 <entry+0xb178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,216)?)?;
    // 0x6089c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x608a0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31c14
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x608a4: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x608a8: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x608ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x608b0: strb r0, [r5, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // 0x608b4: ldrb r1, [r5, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,990)?)?;
    // 0x608b8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x608bc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x608c0: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x608c4: strb r1, [r5, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,13)?)?;
    // 0x608c8: ldr r1, [r5, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,280)?)?;
    // 0x608cc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x608d0: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x608d4: strb r0, [r5, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,14)?)?;
    // 0x608d8: ldr r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,36)?)?;
    // 0x608dc: bl 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x5dfa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,true)?)?;
    // 0x608e0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x608e4: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x608e8: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x608ec: moveq r1, #9
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,9)?)?;
    // 0x608f0: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x608f4: ldr r0, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,56)?)?;
    // 0x608f8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x608fc: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5dfd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60900: ldrb r0, [r5, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,990)?)?;
    // 0x60904: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x60908: bne 0x60918 <entry+0xb118>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60918,false)?)?;
    // 0x6090c: ldr r0, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,52)?)?;
    // 0x60910: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60914: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5dff0
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60918: ldr r0, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,48)?)?;
    // 0x6091c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60920: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5dffc
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60924: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x60928: beq 0x60964 <entry+0xb164>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60964,false)?)?;
    // 0x6092c: ldr r0, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,20)?)?;
    // 0x60930: add r1, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,12)?)?;
    // 0x60934: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x60938: ldr r0, [pc, #0x4c]         @ 0xb18c <entry+0xb18c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,76)?)?;
    // 0x6093c: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x60940: str r6, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,24)?)?;
    // 0x60944: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x60948: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x6094c: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x60950: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x60954: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x60958: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x360e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x6095c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x60960: b 0x60968 <entry+0xb168>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x60968,false)?)?;
    // 0x60964: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x60968: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x6096c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x60970: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x60974)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_60974:[u32;7]=[0x000f0eb8,0x000e2788,0x000f0ec6,0x000f0ef9,0x000f0f3d,0x000f0f0b,0x00221006];
 for value in INLINE_DATA_60974{c.emit(value)?;}
 // Executable A32 block 0x60990..0x60b90.
 c.check(0x60990)?;
    // 0x60990: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x60994: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x60998: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x6099c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x609a0: ldr r1, [pc, #0x1e8]        @ 0xb390 <entry+0xb390>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,488)?)?;
    // 0x609a4: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x609a8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x609ac: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31d20
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x609b0: ldr r1, [pc, #0x1dc]        @ 0xb394 <entry+0xb394>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,476)?)?;
    // 0x609b4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x609b8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31d2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x609bc: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x609c0: cmp r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,12)?)?;
    // 0x609c4: bne 0x60aa8 <entry+0xb2a8>  @ imm = #0xdc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60aa8,false)?)?;
    // 0x609c8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x609cc: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x5240
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x609d0: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x609d4: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x609d8: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x609dc: ldr r0, [pc, #0x1bc]        @ 0xb3a0 <entry+0xb3a0>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,444)?)?;
    // 0x609e0: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x609e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x609e8: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x609ec: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x609f0: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x609f4: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // 0x609f8: beq 0x60a0c <entry+0xb20c>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60a0c,false)?)?;
    // 0x609fc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x60a00: beq 0x60b04 <entry+0xb304>  @ imm = #0xfc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60b04,false)?)?;
    // 0x60a04: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x60a08: bne 0x60b34 <entry+0xb334>  @ imm = #0x124
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60b34,false)?)?;
    // 0x60a0c: strb r0, [r4, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x60a10: ldrb r1, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,990)?)?;
    // 0x60a14: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x60a18: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x60a1c: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x60a20: strb r1, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,13)?)?;
    // 0x60a24: ldr r1, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,280)?)?;
    // 0x60a28: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x60a2c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x60a30: strb r0, [r4, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,14)?)?;
    // 0x60a34: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60a38: bl 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x5e104
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,true)?)?;
    // 0x60a3c: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x60a40: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x60a44: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // 0x60a48: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x60a4c: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e128
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60a50: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x60a54: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x60a58: bne 0x60a68 <entry+0xb268>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60a68,false)?)?;
    // 0x60a5c: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x60a60: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60a64: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e140
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60a68: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x60a6c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60a70: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e14c
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60a74: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60a78: bl 0x294c <entry+0xfffffffffffad14c> @ imm = #-0x5e134
    c.emit(arm32::branch(C::Al,c.at as u32,0x294c,true)?)?;
    // 0x60a7c: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60a80: bl 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x5e14c
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,true)?)?;
    // 0x60a84: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x60a88: ldr r2, [pc, #0x11c]        @ 0xb3ac <entry+0xb3ac>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,284)?)?;
    // 0x60a8c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x60a90: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x60a94: mov r1, #206
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,206)?)?;
    // 0x60a98: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x60a9c: orr r1, r1, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8960)?)?;
    // 0x60aa0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5f2f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x60aa4: b 0x60b74 <entry+0xb374>  @ imm = #0xc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x60b74,false)?)?;
    // 0x60aa8: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x60aac: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60ab0: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x60ab4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x60ab8: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x60abc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x60ac0: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x60ac4: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x60ac8: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x60acc: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x36144
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x60ad0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x60ad4: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x60ad8: ldr r2, [pc, #0xbc]         @ 0xb39c <entry+0xb39c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,188)?)?;
    // 0x60adc: mov r3, #169
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,169)?)?;
    // 0x60ae0: orr r3, r3, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8960)?)?;
    // 0x60ae4: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x60ae8: ldr r1, [pc, #0xa8]         @ 0xb398 <entry+0xb398>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,168)?)?;
    // 0x60aec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60af0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31e64
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60af4: ldr r1, [pc, #0x98]         @ 0xb394 <entry+0xb394>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,152)?)?;
    // 0x60af8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60afc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31e70
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60b00: b 0x60b84 <entry+0xb384>  @ imm = #0x7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x60b84,false)?)?;
    // 0x60b04: ldr r1, [pc, #0x98]         @ 0xb3a4 <entry+0xb3a4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,152)?)?;
    // 0x60b08: ldr r2, [pc, #0x8c]         @ 0xb39c <entry+0xb39c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,140)?)?;
    // 0x60b0c: mov r3, #210
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,210)?)?;
    // 0x60b10: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60b14: orr r3, r3, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8960)?)?;
    // 0x60b18: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31e8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60b1c: ldr r1, [pc, #0x70]         @ 0xb394 <entry+0xb394>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,112)?)?;
    // 0x60b20: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60b24: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31e98
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60b28: ldr r0, [pc, #0x78]         @ 0xb3a8 <entry+0xb3a8>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,120)?)?;
    // 0x60b2c: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x60b30: b 0x60b74 <entry+0xb374>  @ imm = #0x3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x60b74,false)?)?;
    // 0x60b34: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60b38: ldr r1, [pc, #0x70]         @ 0xb3b0 <entry+0xb3b0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,112)?)?;
    // 0x60b3c: ldr r2, [pc, #0x58]         @ 0xb39c <entry+0xb39c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,88)?)?;
    // 0x60b40: mov r3, #214
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,214)?)?;
    // 0x60b44: orr r3, r3, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8960)?)?;
    // 0x60b48: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x60b4c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60b50: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31ec4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60b54: ldr r1, [pc, #0x38]         @ 0xb394 <entry+0xb394>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,56)?)?;
    // 0x60b58: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60b5c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31ed0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60b60: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x60b64: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x60b68: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x60b6c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x60b70: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x60b74: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60b78: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x60b7c: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x3630c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x60b80: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x60b84: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60b88: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x60b8c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x60b90)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_60B90:[u32;9]=[0x000f0f71,0x000e2788,0x000f0f81,0x000f0fbd,0x00221004,0x000f0fd1,0x01001002,0x000ee470,0x000f1002];
 for value in INLINE_DATA_60B90{c.emit(value)?;}
 // Executable A32 block 0x60bb4..0x60d50.
 c.check(0x60bb4)?;
    // 0x60bb4: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x60bb8: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x60bbc: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x60bc0: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x60bc4: ldr r1, [pc, #0x184]        @ 0xb550 <entry+0xb550>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,388)?)?;
    // 0x60bc8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x60bcc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60bd0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31f44
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60bd4: ldr r1, [pc, #0x178]        @ 0xb554 <entry+0xb554>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,376)?)?;
    // 0x60bd8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60bdc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x31f50
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60be0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x60be4: cmp r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,12)?)?;
    // 0x60be8: bne 0x60c68 <entry+0xb468>  @ imm = #0x78
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60c68,false)?)?;
    // 0x60bec: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60bf0: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x5464
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x60bf4: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x60bf8: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x60bfc: ldr r0, [pc, #0x15c]        @ 0xb560 <entry+0xb560>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,348)?)?;
    // 0x60c00: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x60c04: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x60c08: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x60c0c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x60c10: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x60c14: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60c18: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x60c1c: bne 0x60cc4 <entry+0xb4c4>  @ imm = #0xa0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60cc4,false)?)?;
    // 0x60c20: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x60c24: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x60c28: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x60c2c: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x60c30: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e30c
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60c34: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x60c38: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60c3c: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e318
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60c40: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x60c44: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x60c48: bne 0x60c58 <entry+0xb458>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60c58,false)?)?;
    // 0x60c4c: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x60c50: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60c54: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e330
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60c58: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // 0x60c5c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x60c60: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e33c
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60c64: b 0x60d34 <entry+0xb534>  @ imm = #0xc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x60d34,false)?)?;
    // 0x60c68: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x60c6c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60c70: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x60c74: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x60c78: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x60c7c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x60c80: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x60c84: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x60c88: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x60c8c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x36304
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x60c90: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x60c94: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x60c98: ldr r2, [pc, #0xbc]         @ 0xb55c <entry+0xb55c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,188)?)?;
    // 0x60c9c: mov r3, #241
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,241)?)?;
    // 0x60ca0: orr r3, r3, #8960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8960)?)?;
    // 0x60ca4: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x60ca8: ldr r1, [pc, #0xa8]         @ 0xb558 <entry+0xb558>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,168)?)?;
    // 0x60cac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60cb0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32024
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60cb4: ldr r1, [pc, #0x98]         @ 0xb554 <entry+0xb554>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,152)?)?;
    // 0x60cb8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60cbc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32030
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60cc0: b 0x60d44 <entry+0xb544>  @ imm = #0x7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x60d44,false)?)?;
    // 0x60cc4: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60cc8: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x60ccc: bne 0x60cfc <entry+0xb4fc>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60cfc,false)?)?;
    // 0x60cd0: ldr r1, [pc, #0x90]         @ 0xb568 <entry+0xb568>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,144)?)?;
    // 0x60cd4: ldr r2, [pc, #0x80]         @ 0xb55c <entry+0xb55c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,128)?)?;
    // 0x60cd8: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x60cdc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60ce0: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x60ce4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32058
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60ce8: ldr r1, [pc, #0x64]         @ 0xb554 <entry+0xb554>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,100)?)?;
    // 0x60cec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60cf0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32064
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60cf4: ldr r0, [pc, #0x70]         @ 0xb56c <entry+0xb56c>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,112)?)?;
    // 0x60cf8: b 0x60d30 <entry+0xb530>  @ imm = #0x30
    c.emit(arm32::branch(C::Al,c.at as u32,0x60d30,false)?)?;
    // 0x60cfc: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60d00: ldr r1, [pc, #0x5c]         @ 0xb564 <entry+0xb564>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,92)?)?;
    // 0x60d04: ldr r2, [pc, #0x50]         @ 0xb55c <entry+0xb55c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,80)?)?;
    // 0x60d08: mov r3, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,18)?)?;
    // 0x60d0c: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x60d10: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x60d14: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60d18: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3208c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60d1c: ldr r1, [pc, #0x30]         @ 0xb554 <entry+0xb554>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,48)?)?;
    // 0x60d20: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60d24: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32098
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60d28: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x60d2c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x60d30: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x60d34: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x60d38: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x60d3c: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x364cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x60d40: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x60d44: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60d48: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x60d4c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x60d50)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_60D50:[u32;8]=[0x000f1038,0x000e2788,0x000f103f,0x000f107a,0x00221005,0x000f10c0,0x000f108d,0x01001002];
 for value in INLINE_DATA_60D50{c.emit(value)?;}
 // Executable A32 block 0x60d70..0x60e44.
 c.check(0x60d70)?;
    // 0x60d70: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x60d74: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x60d78: sub sp, sp, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,40)?)?;
    // 0x60d7c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x60d80: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x60d84: ldr r1, [pc, #0xb8]         @ 0xb644 <entry+0xb644>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,184)?)?;
    // 0x60d88: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60d8c: ldr r6, [r5, #0x81c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2076)?)?;
    // 0x60d90: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32104
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60d94: ldr r1, [pc, #0xac]         @ 0xb648 <entry+0xb648>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,172)?)?;
    // 0x60d98: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60d9c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32110
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60da0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60da4: cmp r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x60da8: bne 0x60de0 <entry+0xb5e0>  @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60de0,false)?)?;
    // 0x60dac: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x60db0: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x5624
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x60db4: add r7, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::SP,16)?)?;
    // 0x60db8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x60dbc: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x60dc0: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x60dc4: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x60dc8: bl 0x593a8 <entry+0x3ba8>  @ imm = #-0x7a28
    c.emit(arm32::branch(C::Al,c.at as u32,0x593a8,true)?)?;
    // 0x60dcc: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x60dd0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x60dd4: bl 0x2a878 <entry+0xfffffffffffd5078> @ imm = #-0x36564
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a878,true)?)?;
    // 0x60dd8: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x60ddc: b 0x60e38 <entry+0xb638>  @ imm = #0x54
    c.emit(arm32::branch(C::Al,c.at as u32,0x60e38,false)?)?;
    // 0x60de0: ldr r1, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,20)?)?;
    // 0x60de4: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x60de8: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x60dec: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x60df0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x60df4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x60df8: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x60dfc: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x60e00: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x60e04: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x3647c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x60e08: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60e0c: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x60e10: ldr r2, [pc, #0x38]         @ 0xb650 <entry+0xb650>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,56)?)?;
    // 0x60e14: mov r3, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,44)?)?;
    // 0x60e18: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x60e1c: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x60e20: ldr r1, [pc, #0x24]         @ 0xb64c <entry+0xb64c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,36)?)?;
    // 0x60e24: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60e28: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3219c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60e2c: ldr r1, [pc, #0x14]         @ 0xb648 <entry+0xb648>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,20)?)?;
    // 0x60e30: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60e34: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x321a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60e38: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x60e3c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x60e40: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x60e44)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_60E44:[u32;4]=[0x000ebfe9,0x000e2788,0x000f10f5,0x000f1145];
 for value in INLINE_DATA_60E44{c.emit(value)?;}
 // Executable A32 block 0x60e54..0x60f44.
 c.check(0x60e54)?;
    // 0x60e54: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x60e58: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x60e5c: ldr r1, [pc, #0xe0]         @ 0xb744 <entry+0xb744>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,224)?)?;
    // 0x60e60: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x60e64: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60e68: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x321dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60e6c: ldr r1, [pc, #0xd4]         @ 0xb748 <entry+0xb748>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,212)?)?;
    // 0x60e70: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60e74: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x321e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60e78: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60e7c: bl 0x5b794 <entry+0x5f94>  @ imm = #-0x56f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5b794,true)?)?;
    // 0x60e80: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60e84: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x60e88: beq 0x60f34 <entry+0xb734>  @ imm = #0xa4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60f34,false)?)?;
    // 0x60e8c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x60e90: strb r5, [r4, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,12)?)?;
    // 0x60e94: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x60e98: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x60e9c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x60ea0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x60ea4: strb r0, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,13)?)?;
    // 0x60ea8: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x60eac: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x60eb0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x60eb4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x60eb8: strb r0, [r4, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,14)?)?;
    // 0x60ebc: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60ec0: bl 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x5e58c
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,true)?)?;
    // 0x60ec4: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x60ec8: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x60ecc: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // 0x60ed0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x60ed4: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e5b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60ed8: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x60edc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x60ee0: bne 0x60ef0 <entry+0xb6f0>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x60ef0,false)?)?;
    // 0x60ee4: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x60ee8: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60eec: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e5c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60ef0: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x60ef4: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x60ef8: bl 0x292c <entry+0xfffffffffffad12c> @ imm = #-0x5e5d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x292c,true)?)?;
    // 0x60efc: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60f00: bl 0x294c <entry+0xfffffffffffad14c> @ imm = #-0x5e5bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x294c,true)?)?;
    // 0x60f04: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x60f08: bl 0x293c <entry+0xfffffffffffad13c> @ imm = #-0x5e5d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x293c,true)?)?;
    // 0x60f0c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60f10: ldr r2, [pc, #0x34]         @ 0xb74c <entry+0xb74c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,52)?)?;
    // 0x60f14: mov r1, #86
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,86)?)?;
    // 0x60f18: orr r1, r1, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9216)?)?;
    // 0x60f1c: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x60f20: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x60f24: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x60f28: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5f780
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x60f2c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60f30: bl 0x5a710 <entry+0x4f10>  @ imm = #-0x6828
    c.emit(arm32::branch(C::Al,c.at as u32,0x5a710,true)?)?;
    // 0x60f34: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60f38: bl 0x5a968 <entry+0x5168>  @ imm = #-0x65d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5a968,true)?)?;
    // 0x60f3c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x60f40: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x60f44)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_60F44:[u32;3]=[0x000f115b,0x000e2788,0x000ee470];
 for value in INLINE_DATA_60F44{c.emit(value)?;}
 // Executable A32 block 0x60f50..0x60fd0.
 c.check(0x60f50)?;
    // 0x60f50: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x60f54: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x60f58: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x60f5c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x60f60: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60f64: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x60f68: beq 0x60fa4 <entry+0xb7a4>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x60fa4,false)?)?;
    // 0x60f6c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x60f70: ldr r1, [pc, #0x58]         @ 0xb7d0 <entry+0xb7d0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,88)?)?;
    // 0x60f74: ldr r2, [pc, #0x58]         @ 0xb7d4 <entry+0xb7d4>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,88)?)?;
    // 0x60f78: mov r3, #105
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,105)?)?;
    // 0x60f7c: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x60f80: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x60f84: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60f88: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x322fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60f8c: ldr r1, [pc, #0x44]         @ 0xb7d8 <entry+0xb7d8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,68)?)?;
    // 0x60f90: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x60f94: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32308
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60f98: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x60f9c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x60fa0: b 0x60fc8 <entry+0xb7c8>  @ imm = #0x20
    c.emit(arm32::branch(C::Al,c.at as u32,0x60fc8,false)?)?;
    // 0x60fa4: ldr r1, [pc, #0x30]         @ 0xb7dc <entry+0xb7dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,48)?)?;
    // 0x60fa8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60fac: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x32320
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60fb0: ldr r1, [pc, #0x20]         @ 0xb7d8 <entry+0xb7d8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,32)?)?;
    // 0x60fb4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60fb8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3232c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60fbc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x60fc0: bl 0x5a968 <entry+0x5168>  @ imm = #-0x6660
    c.emit(arm32::branch(C::Al,c.at as u32,0x5a968,true)?)?;
    // 0x60fc4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x60fc8: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x60fcc: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x60fd0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_60FD0:[u32;4]=[0x000f116a,0x000f119b,0x000e2788,0x000f11a8];
 for value in INLINE_DATA_60FD0{c.emit(value)?;}
 // Executable A32 block 0x60fe0..0x61000.
 c.check(0x60fe0)?;
    // 0x60fe0: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x60fe4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x60fe8: sub sp, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,24)?)?;
    // 0x60fec: ldr r1, [pc, #0xe4]         @ 0xb8d8 <entry+0xb8d8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,228)?)?;
    // 0x60ff0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x60ff4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x60ff8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x3236c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x60ffc: ldr r1, [pc, #0xd8]         @ 0xb8dc <entry+0xb8dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,216)?)?;
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
