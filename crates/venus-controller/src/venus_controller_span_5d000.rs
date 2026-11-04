// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x5d000,0x5e000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x5d000;
pub const END:usize=0x5e000;
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
 // Executable A32 block 0x5d000..0x5d00c.
 c.check(0x5d000)?;
    // 0x5d000: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d004: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5a3d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5d008: b 0x5c314 <entry+0x6b14>  @ imm = #-0xcfc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c314,false)?)?;
 c.check(0x5d00c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D00C:[u32;1]=[0x000ef98b];
 for value in INLINE_DATA_5D00C{c.emit(value)?;}
 // Executable A32 block 0x5d010..0x5d050.
 c.check(0x5d010)?;
    // 0x5d010: ldr r1, [pc, #0xf8c]        @ 0x87a4 <entry+0x87a4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3980)?)?;
    // 0x5d014: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d018: beq 0x5e490 <entry+0x8c90>  @ imm = #0x1470
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5e490,false)?)?;
    // 0x5d01c: ldr r1, [pc, #0xf84]        @ 0x87a8 <entry+0x87a8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3972)?)?;
    // 0x5d020: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d024: bne 0x5e4e0 <entry+0x8ce0>  @ imm = #0x14b4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5e4e0,false)?)?;
    // 0x5d028: ldr r6, [pc, #0xf78]        @ 0x87a8 <entry+0x87a8>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,3960)?)?;
    // 0x5d02c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d030: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d034: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d038: blo 0x5e510 <entry+0x8d10>  @ imm = #0x14d0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e510,false)?)?;
    // 0x5d03c: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d040: ldr r0, [sp, #0xa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,160)?)?;
    // 0x5d044: ldr r1, [pc, #0xfd8]        @ 0x8824 <entry+0x8824>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4056)?)?;
    // 0x5d048: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5d04c: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0x145c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
 c.check(0x5d050)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D050:[u32;3]=[0x000efa31,0x000efad7,0x000efb25];
 for value in INLINE_DATA_5D050{c.emit(value)?;}
 // Executable A32 block 0x5d05c..0x5d0d8.
 c.check(0x5d05c)?;
    // 0x5d05c: ldr r1, [pc, #0xfc4]        @ 0x8828 <entry+0x8828>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4036)?)?;
    // 0x5d060: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d064: bne 0x5e4e0 <entry+0x8ce0>  @ imm = #0x1474
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5e4e0,false)?)?;
    // 0x5d068: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d06c: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5d070: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5d074: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d078: blo 0x5e55c <entry+0x8d5c>  @ imm = #0x14dc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e55c,false)?)?;
    // 0x5d07c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d080: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5d084: beq 0x5c66c <entry+0x6e6c>  @ imm = #-0xa20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c66c,false)?)?;
    // 0x5d088: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x5d08c: beq 0x5c680 <entry+0x6e80>  @ imm = #-0xa14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c680,false)?)?;
    // 0x5d090: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5d094: bne 0x5c6b4 <entry+0x6eb4>  @ imm = #-0x9e8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c6b4,false)?)?;
    // 0x5d098: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d09c: ldr r4, [pc, #-0x860]       @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,-2144)?)?;
    // 0x5d0a0: str r0, [r9, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1528)?)?;
    // 0x5d0a4: ldr r1, [r9, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,124)?)?;
    // 0x5d0a8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5d0ac: bne 0x5c70c <entry+0x6f0c>  @ imm = #-0x9a8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c70c,false)?)?;
    // 0x5d0b0: ldr r1, [r9, #0x63c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1596)?)?;
    // 0x5d0b4: ldr r2, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,68)?)?;
    // 0x5d0b8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x5d0bc: bl 0x7e544 <entry+0x28d44> @ imm = #0x21480
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e544,true)?)?;
    // 0x5d0c0: ldr r0, [r9, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,124)?)?;
    // 0x5d0c4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5d0c8: beq 0x5c994 <entry+0x7194>  @ imm = #-0x73c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c994,false)?)?;
    // 0x5d0cc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5d0d0: bl 0x804f4 <entry+0x2acf4> @ imm = #0x2341c
    c.emit(arm32::branch(C::Al,c.at as u32,0x804f4,true)?)?;
    // 0x5d0d4: b 0x5c70c <entry+0x6f0c>  @ imm = #-0x9d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c70c,false)?)?;
 c.check(0x5d0d8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D0D8:[u32;2]=[0x000efb73,0x000efbe1];
 for value in INLINE_DATA_5D0D8{c.emit(value)?;}
 // Executable A32 block 0x5d0e0..0x5d1c0.
 c.check(0x5d0e0)?;
    // 0x5d0e0: cmn r5, #24
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(5)?,24)?)?;
    // 0x5d0e4: bhs 0x5e564 <entry+0x8d64>  @ imm = #0x1478
    c.emit(arm32::branch(C::Cs,c.at as u32,0x5e564,false)?)?;
    // 0x5d0e8: mvn r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,24)?)?;
    // 0x5d0ec: sub r2, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x5d0f0: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d0f4: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x5d0f8: cmp r1, r2, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsr,4,false)?)?;
    // 0x5d0fc: bhi 0x5e5f4 <entry+0x8df4>  @ imm = #0x14f0
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5e5f4,false)?)?;
    // 0x5d100: ldr r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // 0x5d104: add r1, r10, r1, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(10)?,R::R1,Shift::Lsl,4,false)?)?;
    // 0x5d108: add r1, r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,28)?)?;
    // 0x5d10c: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d110: blo 0x5e618 <entry+0x8e18>  @ imm = #0x1500
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e618,false)?)?;
    // 0x5d114: ldr r6, [pc, #-0x81c]       @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,-2076)?)?;
    // 0x5d118: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5d11c: mov r1, #7360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7360)?)?;
    // 0x5d120: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d124: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5d128: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5b980
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5d12c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d130: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5d134: bne 0x5bfc8 <entry+0x67c8>  @ imm = #-0x1174
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5bfc8,false)?)?;
    // 0x5d138: ldr r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,24)?)?;
    // 0x5d13c: ldr r1, [sp, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,108)?)?;
    // 0x5d140: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d144: ldr r0, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,16)?)?;
    // 0x5d148: ldr r1, [sp, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,112)?)?;
    // 0x5d14c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d150: ldr r0, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,20)?)?;
    // 0x5d154: ldr r1, [sp, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,116)?)?;
    // 0x5d158: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d15c: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5d160: ldr r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x5d164: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d168: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d16c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5d170: blo 0x5d1a4 <entry+0x79a4>  @ imm = #0x2c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5d1a4,false)?)?;
    // 0x5d174: ldr r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,40)?)?;
    // 0x5d178: ldr r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // 0x5d17c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d180: ldr r0, [r10, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,32)?)?;
    // 0x5d184: ldr r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // 0x5d188: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d18c: ldr r0, [r10, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,36)?)?;
    // 0x5d190: ldr r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // 0x5d194: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d198: ldr r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,28)?)?;
    // 0x5d19c: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // 0x5d1a0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d1a4: ldr r1, [sp, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,120)?)?;
    // 0x5d1a8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d1ac: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d1b0: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x5d1b4: bl 0x58214 <entry+0x2a14>  @ imm = #-0x4fa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x58214,true)?)?;
    // 0x5d1b8: ldr r4, [pc, #-0x97c]       @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,-2428)?)?;
    // 0x5d1bc: b 0x5bff8 <entry+0x67f8>  @ imm = #-0x11cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5bff8,false)?)?;
 c.check(0x5d1c0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D1C0:[u32;1]=[0x000ef0bb];
 for value in INLINE_DATA_5D1C0{c.emit(value)?;}
 // Executable A32 block 0x5d1c4..0x5d228.
 c.check(0x5d1c4)?;
    // 0x5d1c4: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d1c8: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5d1cc: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x5d1d0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d1d4: blo 0x5e578 <entry+0x8d78>  @ imm = #0x139c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e578,false)?)?;
    // 0x5d1d8: ldr r2, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,8)?)?;
    // 0x5d1dc: ldr r0, [pc, #0xfec]        @ 0x89d0 <entry+0x89d0>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,4076)?)?;
    // 0x5d1e0: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5d1e4: beq 0x5c1e4 <entry+0x69e4>  @ imm = #-0x1008
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c1e4,false)?)?;
    // 0x5d1e8: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x5d1ec: mov r10, r3
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(3)?))?;
    // 0x5d1f0: orr r0, r0, #117440512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,117440512)?)?;
    // 0x5d1f4: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5d1f8: ldreq r0, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d1fc: ldreq r1, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::SP,80)?)?;
    // 0x5d200: strbeq r0, [r1]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::R1,0)?)?;
    // 0x5d204: b 0x5c1f4 <entry+0x69f4>  @ imm = #-0x1018
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c1f4,false)?)?;
    // 0x5d208: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d20c: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d210: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d214: blo 0x5e584 <entry+0x8d84>  @ imm = #0x1368
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e584,false)?)?;
    // 0x5d218: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d21c: ldr r0, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-60)?)?;
    // 0x5d220: ldr r1, [pc, #0xfac]        @ 0x89d4 <entry+0x89d4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4012)?)?;
    // 0x5d224: b 0x5e4ac <entry+0x8cac>  @ imm = #0x1280
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4ac,false)?)?;
 c.check(0x5d228)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D228:[u32;2]=[0x000eec4d,0x000eec95];
 for value in INLINE_DATA_5D228{c.emit(value)?;}
 // Executable A32 block 0x5d230..0x5d2d8.
 c.check(0x5d230)?;
    // 0x5d230: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d234: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5d238: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5d23c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d240: blo 0x5e590 <entry+0x8d90>  @ imm = #0x1348
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e590,false)?)?;
    // 0x5d244: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d248: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5d24c: bne 0x5c8cc <entry+0x70cc>  @ imm = #-0x988
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c8cc,false)?)?;
    // 0x5d250: ldr r2, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,8)?)?;
    // 0x5d254: ldr r0, [r9, #0x638]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1592)?)?;
    // 0x5d258: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5d25c: bhs 0x5c460 <entry+0x6c60>  @ imm = #-0xe04
    c.emit(arm32::branch(C::Cs,c.at as u32,0x5c460,false)?)?;
    // 0x5d260: ldr r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,60)?)?;
    // 0x5d264: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x5d268: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // 0x5d26c: mov r3, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,23)?)?;
    // 0x5d270: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x5d274: sub r2, r2, #2031616
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,2031616)?)?;
    // 0x5d278: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x5d27c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5d280: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x5d284: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x5d288: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x5d28c: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x32904
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x5d290: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d294: ldr r1, [r9, #0x638]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1592)?)?;
    // 0x5d298: ldr r2, [pc, #-0xa8c]       @ 0x7014 <entry+0x7014>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,-2700)?)?;
    // 0x5d29c: mov r3, #8000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8000)?)?;
    // 0x5d2a0: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5d2a4: ldr r1, [pc, #0xff8]        @ 0x8aa4 <entry+0x8aa4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5d2a8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5d2ac: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2e620
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5d2b0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5d2b4: b 0x5c7ac <entry+0x6fac>  @ imm = #-0xb10
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c7ac,false)?)?;
    // 0x5d2b8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d2bc: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d2c0: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d2c4: blo 0x5e5a0 <entry+0x8da0>  @ imm = #0x12d4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e5a0,false)?)?;
    // 0x5d2c8: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d2cc: ldr r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-64)?)?;
    // 0x5d2d0: ldr r1, [pc, #0xfd0]        @ 0x8aa8 <entry+0x8aa8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4048)?)?;
    // 0x5d2d4: b 0x5e4ac <entry+0x8cac>  @ imm = #0x11d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4ac,false)?)?;
 c.check(0x5d2d8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D2D8:[u32;3]=[0x000eedf9,0x000f002f,0x000ef1fb];
 for value in INLINE_DATA_5D2D8{c.emit(value)?;}
 // Executable A32 block 0x5d2e4..0x5d384.
 c.check(0x5d2e4)?;
    // 0x5d2e4: cmn r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(5)?,16)?)?;
    // 0x5d2e8: bhs 0x5e620 <entry+0x8e20>  @ imm = #0x1330
    c.emit(arm32::branch(C::Cs,c.at as u32,0x5e620,false)?)?;
    // 0x5d2ec: mvn r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,16)?)?;
    // 0x5d2f0: sub r2, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x5d2f4: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d2f8: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x5d2fc: cmp r1, r2, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsr,3,false)?)?;
    // 0x5d300: bhi 0x5e634 <entry+0x8e34>  @ imm = #0x132c
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5e634,false)?)?;
    // 0x5d304: ldr r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // 0x5d308: add r1, r10, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(10)?,R::R1,Shift::Lsl,3,false)?)?;
    // 0x5d30c: add r1, r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,20)?)?;
    // 0x5d310: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d314: blo 0x5e658 <entry+0x8e58>  @ imm = #0x133c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e658,false)?)?;
    // 0x5d318: ldr r6, [pc, #-0xa20]       @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,-2592)?)?;
    // 0x5d31c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5d320: mov r1, #264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,264)?)?;
    // 0x5d324: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d328: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x5d32c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x5d330: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5bb88
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5d334: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d338: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5d33c: bne 0x5c080 <entry+0x6880>  @ imm = #-0x12c4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c080,false)?)?;
    // 0x5d340: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5d344: ldr r1, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,100)?)?;
    // 0x5d348: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d34c: ldr r0, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,16)?)?;
    // 0x5d350: ldr r1, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,96)?)?;
    // 0x5d354: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d358: ldr r7, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,8)?)?;
    // 0x5d35c: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x5d360: blo 0x5d37c <entry+0x7b7c>  @ imm = #0x14
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5d37c,false)?)?;
    // 0x5d364: ldr r0, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,20)?)?;
    // 0x5d368: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x5d36c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d370: ldr r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,24)?)?;
    // 0x5d374: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // 0x5d378: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d37c: ldr r4, [pc, #-0xb40]       @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,-2880)?)?;
    // 0x5d380: b 0x5c0c0 <entry+0x68c0>  @ imm = #-0x12c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c0c0,false)?)?;
 c.check(0x5d384)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D384:[u32;3]=[0x000ef239,0x000eedc4,0x000ef5e1];
 for value in INLINE_DATA_5D384{c.emit(value)?;}
 // Executable A32 block 0x5d390..0x5d3f4.
 c.check(0x5d390)?;
    // 0x5d390: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d394: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5d398: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5d39c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d3a0: blo 0x5e660 <entry+0x8e60>  @ imm = #0x12b8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e660,false)?)?;
    // 0x5d3a4: ldr r1, [pc, #0xfe0]        @ 0x8b8c <entry+0x8b8c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4064)?)?;
    // 0x5d3a8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5d3ac: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2e720
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5d3b0: ldr r4, [pc, #-0xb74]       @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,-2932)?)?;
    // 0x5d3b4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5d3b8: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x5d3bc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2e730
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5d3c0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d3c4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5d3c8: bne 0x5bd80 <entry+0x6580>  @ imm = #-0x1650
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5bd80,false)?)?;
    // 0x5d3cc: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d3d0: ldr r5, [pc, #-0xad8]       @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,-2776)?)?;
    // 0x5d3d4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5d3d8: str r1, [r9, #0xd50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3408)?)?;
    // 0x5d3dc: strb r1, [r9, #0xe62]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,3682)?)?;
    // 0x5d3e0: cmp r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32768)?)?;
    // 0x5d3e4: blo 0x5c478 <entry+0x6c78>  @ imm = #-0xf74
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5c478,false)?)?;
    // 0x5d3e8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5d3ec: sub r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,32768)?)?;
    // 0x5d3f0: b 0x5c47c <entry+0x6c7c>  @ imm = #-0xf7c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c47c,false)?)?;
 c.check(0x5d3f4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D3F4:[u32;1]=[0x000ef60b];
 for value in INLINE_DATA_5D3F4{c.emit(value)?;}
 // Executable A32 block 0x5d3f8..0x5d43c.
 c.check(0x5d3f8)?;
    // 0x5d3f8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d3fc: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5d400: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5d404: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d408: blo 0x5e66c <entry+0x8e6c>  @ imm = #0x125c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e66c,false)?)?;
    // 0x5d40c: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5d410: ldr r1, [r9, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,3372)?)?;
    // 0x5d414: str r0, [r9, #0xd30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3376)?)?;
    // 0x5d418: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x5d41c: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d420: bne 0x5bdc0 <entry+0x65c0>  @ imm = #-0x1668
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5bdc0,false)?)?;
    // 0x5d424: bic r1, r0, #-268435456
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R0,4026531840)?)?;
    // 0x5d428: lsr r0, r0, #28
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,28)?)?;
    // 0x5d42c: str r1, [r9, #0xd34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,3380)?)?;
    // 0x5d430: mov r1, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,156)?)?;
    // 0x5d434: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x5d438: b 0x5bdc8 <entry+0x65c8>  @ imm = #-0x1678
    c.emit(arm32::branch(C::Al,c.at as u32,0x5bdc8,false)?)?;
 c.check(0x5d43c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D43C:[u32;1]=[0x000f0547];
 for value in INLINE_DATA_5D43C{c.emit(value)?;}
 // Executable A32 block 0x5d440..0x5d498.
 c.check(0x5d440)?;
    // 0x5d440: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d444: add r1, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,8)?)?;
    // 0x5d448: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5d44c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d450: blo 0x5e674 <entry+0x8e74>  @ imm = #0x121c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e674,false)?)?;
    // 0x5d454: ldr r7, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,4)?)?;
    // 0x5d458: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x5d45c: ldr r2, [pc, #-0xb64]       @ 0x7100 <entry+0x7100>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,-2916)?)?;
    // 0x5d460: mov r1, #195
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,195)?)?;
    // 0x5d464: orr r1, r1, #6912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6912)?)?;
    // 0x5d468: cmp r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,16)?)?;
    // 0x5d46c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x5d470: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x5d474: cmp r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // 0x5d478: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5d47c: orr r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x5d480: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x5bcd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5d484: ldr r1, [pc, #0xfcc]        @ 0x8c58 <entry+0x8c58>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4044)?)?;
    // 0x5d488: strb r5, [r9, #0xe95]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(9)?,3733)?)?;
    // 0x5d48c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5d490: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x5d494: b 0x5c470 <entry+0x6c70>  @ imm = #-0x102c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c470,false)?)?;
 c.check(0x5d498)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D498:[u32;1]=[0xffffc000];
 for value in INLINE_DATA_5D498{c.emit(value)?;}
 // Executable A32 block 0x5d49c..0x5d4ec.
 c.check(0x5d49c)?;
    // 0x5d49c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d4a0: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d4a4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d4a8: blo 0x5e67c <entry+0x8e7c>  @ imm = #0x11cc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e67c,false)?)?;
    // 0x5d4ac: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d4b0: ldr r1, [pc, #0xfd4]        @ 0x8c8c <entry+0x8c8c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4052)?)?;
    // 0x5d4b4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5d4b8: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x5d4bc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d4c0: str r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4)?)?;
    // 0x5d4c4: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0xfe4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
    // 0x5d4c8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d4cc: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d4d0: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d4d4: blo 0x5e684 <entry+0x8e84>  @ imm = #0x11a8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e684,false)?)?;
    // 0x5d4d8: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d4dc: ldr r1, [pc, #0xff0]        @ 0x8cd4 <entry+0x8cd4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4080)?)?;
    // 0x5d4e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5d4e4: str r2, [r9, #0xd40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,3392)?)?;
    // 0x5d4e8: b 0x5e4b4 <entry+0x8cb4>  @ imm = #0xfc4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b4,false)?)?;
 c.check(0x5d4ec)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D4EC:[u32;1]=[0x000ef782];
 for value in INLINE_DATA_5D4EC{c.emit(value)?;}
 // Executable A32 block 0x5d4f0..0x5d558.
 c.check(0x5d4f0)?;
    // 0x5d4f0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d4f4: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d4f8: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d4fc: blo 0x5e68c <entry+0x8e8c>  @ imm = #0x1188
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e68c,false)?)?;
    // 0x5d500: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d504: ldr r0, [r11, #-0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-92)?)?;
    // 0x5d508: ldr r1, [pc, #0xfc8]        @ 0x8cd8 <entry+0x8cd8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4040)?)?;
    // 0x5d50c: b 0x5e4ac <entry+0x8cac>  @ imm = #0xf98
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4ac,false)?)?;
    // 0x5d510: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d514: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d518: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d51c: blo 0x5e698 <entry+0x8e98>  @ imm = #0x1174
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e698,false)?)?;
    // 0x5d520: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d524: ldr r0, [r11, #-0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-88)?)?;
    // 0x5d528: ldr r1, [pc, #0xfac]        @ 0x8cdc <entry+0x8cdc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4012)?)?;
    // 0x5d52c: b 0x5e4ac <entry+0x8cac>  @ imm = #0xf78
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4ac,false)?)?;
    // 0x5d530: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d534: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d538: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d53c: blo 0x5e6a4 <entry+0x8ea4>  @ imm = #0x1160
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e6a4,false)?)?;
    // 0x5d540: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d544: ldr r1, [r11, #-0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-100)?)?;
    // 0x5d548: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d54c: ldr r1, [pc, #0xff8]        @ 0x8d4c <entry+0x8d4c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5d550: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x5d554: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0xf54
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
 c.check(0x5d558)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D558:[u32;1]=[0x000ef850];
 for value in INLINE_DATA_5D558{c.emit(value)?;}
 // Executable A32 block 0x5d55c..0x5d57c.
 c.check(0x5d55c)?;
    // 0x5d55c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d560: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d564: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d568: blo 0x5e6b0 <entry+0x8eb0>  @ imm = #0x1140
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e6b0,false)?)?;
    // 0x5d56c: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d570: ldr r0, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-72)?)?;
    // 0x5d574: ldr r1, [pc, #0xff8]        @ 0x8d74 <entry+0x8d74>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5d578: b 0x5e4ac <entry+0x8cac>  @ imm = #0xf2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4ac,false)?)?;
 c.check(0x5d57c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D57C:[u32;1]=[0x000ef38c];
 for value in INLINE_DATA_5D57C{c.emit(value)?;}
 // Executable A32 block 0x5d580..0x5d5a8.
 c.check(0x5d580)?;
    // 0x5d580: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d584: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d588: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d58c: blo 0x5e6bc <entry+0x8ebc>  @ imm = #0x1128
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e6bc,false)?)?;
    // 0x5d590: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d594: ldr r1, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-76)?)?;
    // 0x5d598: and r0, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R2,255)?)?;
    // 0x5d59c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d5a0: ldr r1, [pc, #0xff4]        @ 0x8d9c <entry+0x8d9c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x5d5a4: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0xf04
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
 c.check(0x5d5a8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D5A8:[u32;1]=[0x000ef3c1];
 for value in INLINE_DATA_5D5A8{c.emit(value)?;}
 // Executable A32 block 0x5d5ac..0x5d5f0.
 c.check(0x5d5ac)?;
    // 0x5d5ac: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d5b0: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d5b4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d5b8: blo 0x5e6c8 <entry+0x8ec8>  @ imm = #0x1108
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e6c8,false)?)?;
    // 0x5d5bc: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d5c0: ldr r0, [r11, #-0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-84)?)?;
    // 0x5d5c4: ldr r1, [pc, #0xff8]        @ 0x8dc4 <entry+0x8dc4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5d5c8: b 0x5d5e8 <entry+0x7de8>  @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x5d5e8,false)?)?;
    // 0x5d5cc: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d5d0: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d5d4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d5d8: blo 0x5e6d8 <entry+0x8ed8>  @ imm = #0x10f8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e6d8,false)?)?;
    // 0x5d5dc: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d5e0: ldr r0, [r11, #-0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-80)?)?;
    // 0x5d5e4: ldr r1, [pc, #0xff4]        @ 0x8de0 <entry+0x8de0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x5d5e8: strb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5d5ec: b 0x5d614 <entry+0x7e14>  @ imm = #0x20
    c.emit(arm32::branch(C::Al,c.at as u32,0x5d614,false)?)?;
 c.check(0x5d5f0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D5F0:[u32;1]=[0x000ef0fc];
 for value in INLINE_DATA_5D5F0{c.emit(value)?;}
 // Executable A32 block 0x5d5f4..0x5d644.
 c.check(0x5d5f4)?;
    // 0x5d5f4: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d5f8: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d5fc: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d600: blo 0x5e6e4 <entry+0x8ee4>  @ imm = #0x10dc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e6e4,false)?)?;
    // 0x5d604: ldr r1, [pc, #0xfe4]        @ 0x8df0 <entry+0x8df0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4068)?)?;
    // 0x5d608: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5d60c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5d610: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x5d614: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d618: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2e98c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5d61c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d620: b 0x5e4bc <entry+0x8cbc>  @ imm = #0xe94
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4bc,false)?)?;
    // 0x5d624: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d628: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d62c: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d630: blo 0x5e6f0 <entry+0x8ef0>  @ imm = #0x10b8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e6f0,false)?)?;
    // 0x5d634: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d638: ldr r0, [r11, #-0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-96)?)?;
    // 0x5d63c: ldr r1, [pc, #0xfec]        @ 0x8e30 <entry+0x8e30>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4076)?)?;
    // 0x5d640: b 0x5e4ac <entry+0x8cac>  @ imm = #0xe64
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4ac,false)?)?;
 c.check(0x5d644)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D644:[u32;1]=[0x000ef122];
 for value in INLINE_DATA_5D644{c.emit(value)?;}
 // Executable A32 block 0x5d648..0x5d6e8.
 c.check(0x5d648)?;
    // 0x5d648: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d64c: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5d650: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5d654: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d658: blo 0x5e6fc <entry+0x8efc>  @ imm = #0x109c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e6fc,false)?)?;
    // 0x5d65c: ldmib r10, {r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5d660: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x5d664: bl 0x54c1c <entry+0xfffffffffffff41c> @ imm = #-0x8a50
    c.emit(arm32::branch(C::Al,c.at as u32,0x54c1c,true)?)?;
    // 0x5d668: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5d66c: ldr r1, [pc, #0xff4]        @ 0x8e68 <entry+0x8e68>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x5d670: b 0x5dba4 <entry+0x83a4>  @ imm = #0x52c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5dba4,false)?)?;
    // 0x5d674: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d678: add r1, r10, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,84)?)?;
    // 0x5d67c: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x5d680: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d684: blo 0x5e704 <entry+0x8f04>  @ imm = #0x1078
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e704,false)?)?;
    // 0x5d688: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5d68c: str r0, [r9, #0xff4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4084)?)?;
    // 0x5d690: ldr r3, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,8)?)?;
    // 0x5d694: and r1, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(3)?,1)?)?;
    // 0x5d698: strb r1, [r9, #0xff8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,4088)?)?;
    // 0x5d69c: sub r1, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,5)?)?;
    // 0x5d6a0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5d6a4: bhi 0x5bde0 <entry+0x65e0>  @ imm = #-0x18cc
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5bde0,false)?)?;
    // 0x5d6a8: ldr r5, [pc, #-0xe6c]       @ 0x7044 <entry+0x7044>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,-3692)?)?;
    // 0x5d6ac: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d6b0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5d6b4: strb r0, [r9, #0xfdb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,4059)?)?;
    // 0x5d6b8: b 0x5bdfc <entry+0x65fc>  @ imm = #-0x18c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5bdfc,false)?)?;
    // 0x5d6bc: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d6c0: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d6c4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d6c8: blo 0x5e550 <entry+0x8d50>  @ imm = #0xe80
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e550,false)?)?;
    // 0x5d6cc: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d6d0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5d6d4: bne 0x5e550 <entry+0x8d50>  @ imm = #0xe74
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5e550,false)?)?;
    // 0x5d6d8: ldr r1, [pc, #0xff4]        @ 0x8ed4 <entry+0x8ed4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x5d6dc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5d6e0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x5d6e4: b 0x5e4b4 <entry+0x8cb4>  @ imm = #0xdc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b4,false)?)?;
 c.check(0x5d6e8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D6E8:[u32;1]=[0x000ef6bf];
 for value in INLINE_DATA_5D6E8{c.emit(value)?;}
 // Executable A32 block 0x5d6ec..0x5d73c.
 c.check(0x5d6ec)?;
    // 0x5d6ec: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d6f0: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d6f4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d6f8: blo 0x5e724 <entry+0x8f24>  @ imm = #0x1024
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e724,false)?)?;
    // 0x5d6fc: ldr r0, [r9, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3372)?)?;
    // 0x5d700: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x5d704: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5d708: bne 0x5c340 <entry+0x6b40>  @ imm = #-0x13d0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c340,false)?)?;
    // 0x5d70c: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d710: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5ab24
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5d714: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d718: str r0, [r9, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,344)?)?;
    // 0x5d71c: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5d720: orr r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // 0x5d724: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5d728: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5d72c: str r0, [r9, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,360)?)?;
    // 0x5d730: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d734: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5ab08
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5d738: b 0x5c390 <entry+0x6b90>  @ imm = #-0x13b0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c390,false)?)?;
 c.check(0x5d73c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D73C:[u32;2]=[0x000ef2a4,0x000eecd6];
 for value in INLINE_DATA_5D73C{c.emit(value)?;}
 // Executable A32 block 0x5d744..0x5d86c.
 c.check(0x5d744)?;
    // 0x5d744: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d748: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5d74c: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5d750: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d754: blo 0x5e72c <entry+0x8f2c>  @ imm = #0xfd0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e72c,false)?)?;
    // 0x5d758: ldr r0, [r11, #-0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-144)?)?;
    // 0x5d75c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x5d760: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5d764: bne 0x5d7cc <entry+0x7fcc>  @ imm = #0x60
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5d7cc,false)?)?;
    // 0x5d768: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d76c: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5ab80
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5d770: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d774: str r0, [r9, #0xe44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3652)?)?;
    // 0x5d778: ldr r1, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // 0x5d77c: cmp r1, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,268435456)?)?;
    // 0x5d780: mvnhs r1, #-268435456
    c.emit(arm32::data_immediate(C::Cs,Op::Mvn,R::R1,R::R0,4026531840)?)?;
    // 0x5d784: strhs r1, [r5]
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(5)?,0)?)?;
    // 0x5d788: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5d78c: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5d790: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x5d794: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5d798: str r2, [r9, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,340)?)?;
    // 0x5d79c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d7a0: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,76)?)?;
    // 0x5d7a4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5d7a8: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5d7ac: orr r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2)?)?;
    // 0x5d7b0: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5d7b4: ldr r2, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,100)?)?;
    // 0x5d7b8: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x5d7bc: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // 0x5d7c0: bl 0x35ee4 <entry+0xfffffffffffe06e4> @ imm = #-0x278e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x35ee4,true)?)?;
    // 0x5d7c4: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d7c8: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5ab9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5d7cc: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5d7d0: ldr r1, [pc, #0xff0]        @ 0x8fc8 <entry+0x8fc8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4080)?)?;
    // 0x5d7d4: b 0x5c7a0 <entry+0x6fa0>  @ imm = #-0x103c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c7a0,false)?)?;
    // 0x5d7d8: ldrb r0, [r9, #0xe41]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,3649)?)?;
    // 0x5d7dc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5d7e0: beq 0x5c13c <entry+0x693c>  @ imm = #-0x16ac
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c13c,false)?)?;
    // 0x5d7e4: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d7e8: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5abfc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5d7ec: ldr r2, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,100)?)?;
    // 0x5d7f0: ldr r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,92)?)?;
    // 0x5d7f4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5d7f8: bl 0x35ee4 <entry+0xfffffffffffe06e4> @ imm = #-0x2791c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35ee4,true)?)?;
    // 0x5d7fc: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5d800: orr r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16)?)?;
    // 0x5d804: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5d808: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d80c: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5abe0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5d810: ldr r2, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,100)?)?;
    // 0x5d814: ldr r1, [pc, #0xff8]        @ 0x9014 <entry+0x9014>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4088)?)?;
    // 0x5d818: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5d81c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2eb90
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5d820: b 0x5dbac <entry+0x83ac>  @ imm = #0x384
    c.emit(arm32::branch(C::Al,c.at as u32,0x5dbac,false)?)?;
    // 0x5d824: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d828: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d82c: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d830: blo 0x5e738 <entry+0x8f38>  @ imm = #0xf00
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e738,false)?)?;
    // 0x5d834: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d838: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5ac4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5d83c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d840: str r0, [r9, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,348)?)?;
    // 0x5d844: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5d848: orr r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32)?)?;
    // 0x5d84c: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5d850: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5d854: str r0, [r9, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,380)?)?;
    // 0x5d858: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d85c: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5ac30
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5d860: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d864: ldr r1, [pc, #0xff4]        @ 0x9060 <entry+0x9060>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4084)?)?;
    // 0x5d868: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0xc40
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
 c.check(0x5d86c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D86C:[u32;4]=[0x000eee76,0x000ef675,0x000f14d9,0x000f14fc];
 for value in INLINE_DATA_5D86C{c.emit(value)?;}
 // Executable A32 block 0x5d87c..0x5d9a0.
 c.check(0x5d87c)?;
    // 0x5d87c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d880: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d884: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d888: blo 0x5e744 <entry+0x8f44>  @ imm = #0xeb4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e744,false)?)?;
    // 0x5d88c: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d890: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5aca4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5d894: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5d898: orr r1, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,64)?)?;
    // 0x5d89c: str r1, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,296)?)?;
    // 0x5d8a0: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x5d8a4: str r1, [r9, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,404)?)?;
    // 0x5d8a8: ldr r2, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,100)?)?;
    // 0x5d8ac: str r2, [r9, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,384)?)?;
    // 0x5d8b0: ldr r2, [r9, #0xe38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,3640)?)?;
    // 0x5d8b4: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x5d8b8: bicge r0, r0, #64
    c.emit(arm32::data_immediate(C::Ge,Op::Bic,R::R0,R::R0,64)?)?;
    // 0x5d8bc: strge r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Ge,false,R::R0,R::new(9)?,296)?)?;
    // 0x5d8c0: movge r0, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5d8c4: strge r0, [r9, #0x180]
    c.emit(arm32::transfer_word(C::Ge,false,R::R0,R::new(9)?,384)?)?;
    // 0x5d8c8: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d8cc: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5aca0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5d8d0: ldr r3, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,100)?)?;
    // 0x5d8d4: ldr r2, [r9, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,404)?)?;
    // 0x5d8d8: ldr r1, [pc, #0xff0]        @ 0x90d0 <entry+0x90d0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4080)?)?;
    // 0x5d8dc: b 0x5c1fc <entry+0x69fc>  @ imm = #-0x16e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c1fc,false)?)?;
    // 0x5d8e0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d8e4: add r1, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,16)?)?;
    // 0x5d8e8: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5d8ec: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5d8f0: blo 0x5e750 <entry+0x8f50>  @ imm = #0xe58
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e750,false)?)?;
    // 0x5d8f4: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d8f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5d8fc: beq 0x5c320 <entry+0x6b20>  @ imm = #-0x15e4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c320,false)?)?;
    // 0x5d900: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d904: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5ad18
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5d908: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5d90c: orr r0, r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,128)?)?;
    // 0x5d910: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5d914: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5d918: str r0, [r9, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,408)?)?;
    // 0x5d91c: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5d920: str r0, [r9, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,412)?)?;
    // 0x5d924: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5d928: str r0, [r9, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,416)?)?;
    // 0x5d92c: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5d930: str r0, [r9, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,388)?)?;
    // 0x5d934: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d938: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5ad0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5d93c: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // 0x5d940: b 0x5c324 <entry+0x6b24>  @ imm = #-0x1624
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c324,false)?)?;
    // 0x5d944: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d948: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d94c: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d950: blo 0x5e75c <entry+0x8f5c>  @ imm = #0xe04
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e75c,false)?)?;
    // 0x5d954: ldrb r0, [r9, #0xe30]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,3632)?)?;
    // 0x5d958: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5d95c: beq 0x5d994 <entry+0x8194>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5d994,false)?)?;
    // 0x5d960: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d964: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5ad78
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5d968: ldr r2, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,100)?)?;
    // 0x5d96c: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x5d970: ldr r0, [sp, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,128)?)?;
    // 0x5d974: bl 0x35ee4 <entry+0xfffffffffffe06e4> @ imm = #-0x27a98
    c.emit(arm32::branch(C::Al,c.at as u32,0x35ee4,true)?)?;
    // 0x5d978: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5d97c: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x5d980: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5d984: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5d988: str r0, [r9, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,392)?)?;
    // 0x5d98c: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d990: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5ad64
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5d994: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5d998: ldr r1, [pc, #0xfdc]        @ 0x917c <entry+0x917c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4060)?)?;
    // 0x5d99c: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0xb0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
 c.check(0x5d9a0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5D9A0:[u32;2]=[0x00202001,0xfdffafff];
 for value in INLINE_DATA_5D9A0{c.emit(value)?;}
 // Executable A32 block 0x5d9a8..0x5dae0.
 c.check(0x5d9a8)?;
    // 0x5d9a8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d9ac: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d9b0: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d9b4: blo 0x5e768 <entry+0x8f68>  @ imm = #0xdac
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e768,false)?)?;
    // 0x5d9b8: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5d9bc: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5add0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5d9c0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d9c4: cmp r0, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1000)?)?;
    // 0x5d9c8: str r0, [r9, #0x148]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,328)?)?;
    // 0x5d9cc: beq 0x5c39c <entry+0x6b9c>  @ imm = #-0x1638
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c39c,false)?)?;
    // 0x5d9d0: cmp r0, #500
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,500)?)?;
    // 0x5d9d4: bne 0x5c3a4 <entry+0x6ba4>  @ imm = #-0x1638
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c3a4,false)?)?;
    // 0x5d9d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5d9dc: b 0x5c3a0 <entry+0x6ba0>  @ imm = #-0x1644
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c3a0,false)?)?;
    // 0x5d9e0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5d9e4: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5d9e8: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5d9ec: blo 0x5e774 <entry+0x8f74>  @ imm = #0xd80
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e774,false)?)?;
    // 0x5d9f0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5d9f4: ldr r5, [r11, #-0x94]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-148)?)?;
    // 0x5d9f8: strb r0, [r5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x5d9fc: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5da00: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5ae14
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5da04: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5da08: orr r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32768)?)?;
    // 0x5da0c: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5da10: ldrb r0, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5da14: str r0, [r9, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,440)?)?;
    // 0x5da18: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5da1c: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5adf0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5da20: ldrb r2, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5da24: ldr r1, [pc, #0xfb0]        @ 0x91dc <entry+0x91dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4016)?)?;
    // 0x5da28: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0xa80
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
    // 0x5da2c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5da30: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5da34: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5da38: blo 0x5e780 <entry+0x8f80>  @ imm = #0xd40
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e780,false)?)?;
    // 0x5da3c: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5da40: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5ae54
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5da44: ldr r2, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,100)?)?;
    // 0x5da48: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x5da4c: ldr r0, [r11, #-0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-152)?)?;
    // 0x5da50: bl 0x35ee4 <entry+0xfffffffffffe06e4> @ imm = #-0x27b74
    c.emit(arm32::branch(C::Al,c.at as u32,0x35ee4,true)?)?;
    // 0x5da54: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5da58: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x5da5c: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5da60: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5da64: str r0, [r9, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,432)?)?;
    // 0x5da68: ldr r0, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,100)?)?;
    // 0x5da6c: str r0, [r9, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,400)?)?;
    // 0x5da70: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5da74: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5ae48
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5da78: ldr r2, [r9, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,432)?)?;
    // 0x5da7c: ldr r1, [pc, #0xf00]        @ 0x9184 <entry+0x9184>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3840)?)?;
    // 0x5da80: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0xa28
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
    // 0x5da84: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5da88: add r1, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,16)?)?;
    // 0x5da8c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5da90: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5da94: blo 0x5e78c <entry+0x8f8c>  @ imm = #0xcf0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e78c,false)?)?;
    // 0x5da98: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5da9c: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5aeb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5daa0: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5daa4: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // 0x5daa8: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5daac: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5dab0: str r0, [r9, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,444)?)?;
    // 0x5dab4: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5dab8: str r0, [r9, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,448)?)?;
    // 0x5dabc: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5dac0: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5ae94
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5dac4: ldr r2, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,8)?)?;
    // 0x5dac8: ldr r1, [pc, #0xe8c]        @ 0x915c <entry+0x915c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3724)?)?;
    // 0x5dacc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5dad0: str r2, [r9, #0xfe0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,4064)?)?;
    // 0x5dad4: ldr r3, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,12)?)?;
    // 0x5dad8: str r3, [r9, #0xfe4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,4068)?)?;
    // 0x5dadc: b 0x5dbe0 <entry+0x83e0>  @ imm = #0xfc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5dbe0,false)?)?;
 c.check(0x5dae0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DAE0:[u32;2]=[0x000efeeb,0xfdff9fff];
 for value in INLINE_DATA_5DAE0{c.emit(value)?;}
 // Executable A32 block 0x5dae8..0x5dcb4.
 c.check(0x5dae8)?;
    // 0x5dae8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5daec: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5daf0: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5daf4: blo 0x5e798 <entry+0x8f98>  @ imm = #0xc9c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e798,false)?)?;
    // 0x5daf8: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5dafc: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5af10
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5db00: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5db04: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x5db08: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5db0c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5db10: str r0, [r9, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,436)?)?;
    // 0x5db14: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5db18: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5aeec
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5db1c: ldr r2, [r9, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,436)?)?;
    // 0x5db20: ldr r1, [pc, #0xe64]        @ 0x918c <entry+0x918c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3684)?)?;
    // 0x5db24: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0x984
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
    // 0x5db28: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5db2c: add r1, r10, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,28)?)?;
    // 0x5db30: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5db34: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5db38: blo 0x5e7a4 <entry+0x8fa4>  @ imm = #0xc64
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e7a4,false)?)?;
    // 0x5db3c: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5db40: ldr r2, [r11, #-0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-140)?)?;
    // 0x5db44: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x5db48: and r1, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,255)?)?;
    // 0x5db4c: strh r1, [r2]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::R2,0)?)?;
    // 0x5db50: and r1, r3, r0, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R0,Shift::Lsr,8,false)?)?;
    // 0x5db54: ldr r2, [r11, #-0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-136)?)?;
    // 0x5db58: strh r1, [r2]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::R2,0)?)?;
    // 0x5db5c: and r1, r3, r0, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R0,Shift::Lsr,16,false)?)?;
    // 0x5db60: ldr r2, [r11, #-0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-156)?)?;
    // 0x5db64: strh r1, [r2]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::R2,0)?)?;
    // 0x5db68: strb r0, [r9, #0xf77]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3959)?)?;
    // 0x5db6c: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5db70: strb r0, [r9, #0xda2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3490)?)?;
    // 0x5db74: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5db78: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5af8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5db7c: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5db80: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x5db84: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5db88: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5db8c: str r0, [r9, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,436)?)?;
    // 0x5db90: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5db94: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5af68
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5db98: ldrb r3, [r9, #0xda2]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,3490)?)?;
    // 0x5db9c: ldr r2, [r9, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,436)?)?;
    // 0x5dba0: ldr r1, [pc, #0xde0]        @ 0x9188 <entry+0x9188>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3552)?)?;
    // 0x5dba4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5dba8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ef1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5dbac: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5dbb0: b 0x5dbe8 <entry+0x83e8>  @ imm = #0x30
    c.emit(arm32::branch(C::Al,c.at as u32,0x5dbe8,false)?)?;
    // 0x5dbb4: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5dbb8: add r1, r10, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,32)?)?;
    // 0x5dbbc: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5dbc0: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5dbc4: blo 0x5e7b0 <entry+0x8fb0>  @ imm = #0xbe4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e7b0,false)?)?;
    // 0x5dbc8: ldmib r10, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x5dbcc: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5dbd0: ldr r1, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,16)?)?;
    // 0x5dbd4: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5dbd8: ldr r1, [pc, #0xd74]        @ 0x9154 <entry+0x9154>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3444)?)?;
    // 0x5dbdc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5dbe0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2ef54
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5dbe4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5dbe8: ldr r1, [pc, #0xd54]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3412)?)?;
    // 0x5dbec: b 0x5e084 <entry+0x8884>  @ imm = #0x490
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e084,false)?)?;
    // 0x5dbf0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5dbf4: add r10, r10, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,20)?)?;
    // 0x5dbf8: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5dbfc: blo 0x5e7bc <entry+0x8fbc>  @ imm = #0xbb8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e7bc,false)?)?;
    // 0x5dc00: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5dc04: str r0, [r9, #0xdf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3572)?)?;
    // 0x5dc08: ldrb r0, [r9, #0xdf2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,3570)?)?;
    // 0x5dc0c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5dc10: beq 0x5dc3c <entry+0x843c>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5dc3c,false)?)?;
    // 0x5dc14: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5dc18: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x5b02c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x5dc1c: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // 0x5dc20: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x5dc24: str r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,296)?)?;
    // 0x5dc28: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x5dc2c: bl 0x58814 <entry+0x3014>  @ imm = #-0x5420
    c.emit(arm32::branch(C::Al,c.at as u32,0x58814,true)?)?;
    // 0x5dc30: str r0, [r9, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,436)?)?;
    // 0x5dc34: ldr r0, [r9, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,292)?)?;
    // 0x5dc38: bl 0x2c34 <entry+0xfffffffffffad434> @ imm = #-0x5b00c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2c34,true)?)?;
    // 0x5dc3c: ldr r3, [r9, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,436)?)?;
    // 0x5dc40: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5dc44: ldr r1, [pc, #0xd98]        @ 0x91e4 <entry+0x91e4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3480)?)?;
    // 0x5dc48: b 0x5c1fc <entry+0x69fc>  @ imm = #-0x1a54
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c1fc,false)?)?;
    // 0x5dc4c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5dc50: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5dc54: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5dc58: blo 0x5e7cc <entry+0x8fcc>  @ imm = #0xb6c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e7cc,false)?)?;
    // 0x5dc5c: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5dc60: ldr r1, [pc, #0xd78]        @ 0x91e0 <entry+0x91e0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3448)?)?;
    // 0x5dc64: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x5dc68: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x5dc6c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5dc70: strb r0, [r9, #0xdf8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3576)?)?;
    // 0x5dc74: b 0x5e4b0 <entry+0x8cb0>  @ imm = #0x834
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4b0,false)?)?;
    // 0x5dc78: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5dc7c: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5dc80: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5dc84: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5dc88: blo 0x5e7d8 <entry+0x8fd8>  @ imm = #0xb48
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e7d8,false)?)?;
    // 0x5dc8c: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // 0x5dc90: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5dc94: ldr r1, [pc, #0xd2c]        @ 0x91c8 <entry+0x91c8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3372)?)?;
    // 0x5dc98: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x5dc9c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5dca0: strb r0, [r9, #0xdd0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3536)?)?;
    // 0x5dca4: ldr r3, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,8)?)?;
    // 0x5dca8: add r0, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,255)?)?;
    // 0x5dcac: strb r0, [r9, #0xdd1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3537)?)?;
    // 0x5dcb0: b 0x5e074 <entry+0x8874>  @ imm = #0x3bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e074,false)?)?;
 c.check(0x5dcb4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DCB4:[u32;1]=[0xfddfafff];
 for value in INLINE_DATA_5DCB4{c.emit(value)?;}
 // Executable A32 block 0x5dcb8..0x5dd38.
 c.check(0x5dcb8)?;
    // 0x5dcb8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5dcbc: add r1, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,16)?)?;
    // 0x5dcc0: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5dcc4: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5dcc8: blo 0x5e7e4 <entry+0x8fe4>  @ imm = #0xb14
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e7e4,false)?)?;
    // 0x5dccc: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5dcd0: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x5dcd4: beq 0x5c568 <entry+0x6d68>  @ imm = #-0x1774
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c568,false)?)?;
    // 0x5dcd8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5dcdc: beq 0x5c574 <entry+0x6d74>  @ imm = #-0x1770
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c574,false)?)?;
    // 0x5dce0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5dce4: bne 0x5c580 <entry+0x6d80>  @ imm = #-0x176c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c580,false)?)?;
    // 0x5dce8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5dcec: strb r0, [r9, #0xdc4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3524)?)?;
    // 0x5dcf0: b 0x5c5b8 <entry+0x6db8>  @ imm = #-0x1740
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c5b8,false)?)?;
    // 0x5dcf4: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5dcf8: add r2, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(10)?,8)?)?;
    // 0x5dcfc: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x5dd00: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x5dd04: blo 0x5e7f0 <entry+0x8ff0>  @ imm = #0xae4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e7f0,false)?)?;
    // 0x5dd08: ldr r0, [r9, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1392)?)?;
    // 0x5dd0c: ldr r5, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,4)?)?;
    // 0x5dd10: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5dd14: bne 0x5c194 <entry+0x6994>  @ imm = #-0x1b88
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c194,false)?)?;
    // 0x5dd18: mvn r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,16777216)?)?;
    // 0x5dd1c: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // 0x5dd20: add r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5dd24: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x5dd28: bhi 0x5c910 <entry+0x7110>  @ imm = #-0x1420
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5c910,false)?)?;
    // 0x5dd2c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x5dd30: add r1, pc, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,8)?)?;
    // 0x5dd34: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x5dd38)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DD38:[u32;10]=[0x000eeea1,0xffdfefff,0x0005dd60,0x0005c1cc,0x0005c818,0x0005c7f0,0x0005c848,0x0005c94c,0x0005c954,0x0005c980];
 for value in INLINE_DATA_5DD38{c.emit(value)?;}
 // Executable A32 block 0x5dd60..0x5de18.
 c.check(0x5dd60)?;
    // 0x5dd60: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5dd64: str r0, [r9, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3472)?)?;
    // 0x5dd68: strb r0, [r9, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,3568)?)?;
    // 0x5dd6c: b 0x5c87c <entry+0x707c>  @ imm = #-0x14f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c87c,false)?)?;
    // 0x5dd70: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x5dd74: add r0, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,16)?)?;
    // 0x5dd78: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5dd7c: blo 0x5e7fc <entry+0x8ffc>  @ imm = #0xa78
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e7fc,false)?)?;
    // 0x5dd80: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5dd84: ldr r1, [pc, #0xc40]        @ 0x91cc <entry+0x91cc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3136)?)?;
    // 0x5dd88: strb r0, [r9, #0xfd3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,4051)?)?;
    // 0x5dd8c: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5dd90: lsr r0, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,8)?)?;
    // 0x5dd94: strb r0, [r9, #0xfd4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,4052)?)?;
    // 0x5dd98: ldrh r0, [r10, #6]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(10)?,6)?)?;
    // 0x5dd9c: strb r0, [r9, #0xfd5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,4053)?)?;
    // 0x5dda0: ldr r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,28)?)?;
    // 0x5dda4: strb r0, [r9, #0xfd0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,4048)?)?;
    // 0x5dda8: ldr r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,28)?)?;
    // 0x5ddac: lsr r0, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,8)?)?;
    // 0x5ddb0: strb r0, [r9, #0xfd1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,4049)?)?;
    // 0x5ddb4: ldrh r0, [r10, #30]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(10)?,30)?)?;
    // 0x5ddb8: strb r0, [r9, #0xfd2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,4050)?)?;
    // 0x5ddbc: ldr r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,28)?)?;
    // 0x5ddc0: ldr r3, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,4)?)?;
    // 0x5ddc4: ldr r2, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,8)?)?;
    // 0x5ddc8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x5ddcc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5ddd0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f144
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5ddd4: ldr r1, [pc, #0xb68]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2920)?)?;
    // 0x5ddd8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5dddc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f150
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5dde0: add r10, r10, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,68)?)?;
    // 0x5dde4: b 0x5e4c4 <entry+0x8cc4>  @ imm = #0x6d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4c4,false)?)?;
    // 0x5dde8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5ddec: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5ddf0: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5ddf4: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5ddf8: blo 0x5e808 <entry+0x9008>  @ imm = #0xa08
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e808,false)?)?;
    // 0x5ddfc: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5de00: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x5de04: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x5de08: bhi 0x5c720 <entry+0x6f20>  @ imm = #-0x16f0
    c.emit(arm32::branch(C::Hi,c.at as u32,0x5c720,false)?)?;
    // 0x5de0c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x5de10: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x5de14: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x5de18)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DE18:[u32;5]=[0x0005de2c,0x0005c718,0x0005c720,0x0005c744,0x0005c74c];
 for value in INLINE_DATA_5DE18{c.emit(value)?;}
 // Executable A32 block 0x5de2c..0x5de38.
 c.check(0x5de2c)?;
    // 0x5de2c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5de30: str r0, [r9, #0xdbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3516)?)?;
    // 0x5de34: b 0x5c758 <entry+0x6f58>  @ imm = #-0x16e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c758,false)?)?;
 c.check(0x5de38)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DE38:[u32;1]=[0x000f046f];
 for value in INLINE_DATA_5DE38{c.emit(value)?;}
 // Executable A32 block 0x5de3c..0x5de80.
 c.check(0x5de3c)?;
    // 0x5de3c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5de40: add r1, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,12)?)?;
    // 0x5de44: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x5de48: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5de4c: blo 0x5e818 <entry+0x9018>  @ imm = #0x9c4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e818,false)?)?;
    // 0x5de50: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5de54: str r0, [r9, #0xd98]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3480)?)?;
    // 0x5de58: ldr r1, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // 0x5de5c: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x5de60: beq 0x5c5d8 <entry+0x6dd8>  @ imm = #-0x1890
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c5d8,false)?)?;
    // 0x5de64: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x5de68: beq 0x5c5e8 <entry+0x6de8>  @ imm = #-0x1888
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5c5e8,false)?)?;
    // 0x5de6c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5de70: bne 0x5c5f4 <entry+0x6df4>  @ imm = #-0x1884
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5c5f4,false)?)?;
    // 0x5de74: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5de78: str r0, [r9, #0xd94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3476)?)?;
    // 0x5de7c: b 0x5c62c <entry+0x6e2c>  @ imm = #-0x1858
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c62c,false)?)?;
 c.check(0x5de80)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DE80:[u32;2]=[0x03007002,0x03008001];
 for value in INLINE_DATA_5DE80{c.emit(value)?;}
 // Executable A32 block 0x5de88..0x5dec8.
 c.check(0x5de88)?;
    // 0x5de88: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5de8c: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5de90: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5de94: blo 0x5e824 <entry+0x9024>  @ imm = #0x988
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e824,false)?)?;
    // 0x5de98: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x5de9c: str r0, [r9, #0xab0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,2736)?)?;
    // 0x5dea0: b 0x5e4c4 <entry+0x8cc4>  @ imm = #0x61c
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e4c4,false)?)?;
    // 0x5dea4: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5dea8: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5deac: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5deb0: blo 0x5e830 <entry+0x9030>  @ imm = #0x978
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e830,false)?)?;
    // 0x5deb4: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5deb8: ldr r0, [sp, #0xa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,164)?)?;
    // 0x5debc: ldr r1, [pc, #0xadc]        @ 0x91a0 <entry+0x91a0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2780)?)?;
    // 0x5dec0: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5dec4: b 0x5e478 <entry+0x8c78>  @ imm = #0x5ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
 c.check(0x5dec8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DEC8:[u32;1]=[0x000ef312];
 for value in INLINE_DATA_5DEC8{c.emit(value)?;}
 // Executable A32 block 0x5decc..0x5df38.
 c.check(0x5decc)?;
    // 0x5decc: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5ded0: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5ded4: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5ded8: blo 0x5e83c <entry+0x903c>  @ imm = #0x95c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e83c,false)?)?;
    // 0x5dedc: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5dee0: ldr r0, [sp, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,176)?)?;
    // 0x5dee4: ldr r1, [pc, #0xab0]        @ 0x919c <entry+0x919c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2736)?)?;
    // 0x5dee8: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5deec: b 0x5e478 <entry+0x8c78>  @ imm = #0x584
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
    // 0x5def0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5def4: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x5def8: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5defc: blo 0x5e848 <entry+0x9048>  @ imm = #0x944
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e848,false)?)?;
    // 0x5df00: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x5df04: ldr r0, [sp, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,168)?)?;
    // 0x5df08: ldr r1, [pc, #0xaa4]        @ 0x91b4 <entry+0x91b4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2724)?)?;
    // 0x5df0c: strb r2, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x5df10: b 0x5e478 <entry+0x8c78>  @ imm = #0x560
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e478,false)?)?;
    // 0x5df14: ldm r5, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5df18: ldr r0, [r11, #-0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-132)?)?;
    // 0x5df1c: add r10, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,12)?)?;
    // 0x5df20: stm r0, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x5df24: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5df28: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x5df2c: blo 0x5e854 <entry+0x9054>  @ imm = #0x920
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e854,false)?)?;
    // 0x5df30: ldr r1, [pc, #0xa60]        @ 0x9198 <entry+0x9198>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2656)?)?;
    // 0x5df34: b 0x5c660 <entry+0x6e60>  @ imm = #-0x18dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5c660,false)?)?;
 c.check(0x5df38)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DF38:[u32;2]=[0x88888889,0x000eee32];
 for value in INLINE_DATA_5DF38{c.emit(value)?;}
 // Executable A32 block 0x5df40..0x5dfa4.
 c.check(0x5df40)?;
    // 0x5df40: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // 0x5df44: add r1, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,16)?)?;
    // 0x5df48: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5df4c: add r6, r0, #27
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,27)?)?;
    // 0x5df50: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5df54: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5df58: blo 0x5e510 <entry+0x8d10>  @ imm = #0x5b0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e510,false)?)?;
    // 0x5df5c: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // 0x5df60: ldr r1, [pc, #0xa18]        @ 0x9180 <entry+0x9180>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2584)?)?;
    // 0x5df64: str r2, [r9, #0xe34]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,3636)?)?;
    // 0x5df68: ldr r7, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,8)?)?;
    // 0x5df6c: cmp r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4)?)?;
    // 0x5df70: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x5df74: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x5df78: movhi r0, #4
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5df7c: str r0, [r9, #0xe38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,3640)?)?;
    // 0x5df80: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5df84: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f2f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5df88: ldr r1, [pc, #0x9b4]        @ 0x9144 <entry+0x9144>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2484)?)?;
    // 0x5df8c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x5df90: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2f304
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5df94: cmp r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4)?)?;
    // 0x5df98: mov r10, r5
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(5)?))?;
    // 0x5df9c: bls 0x5e4c4 <entry+0x8cc4>  @ imm = #0x520
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5e4c4,false)?)?;
    // 0x5dfa0: b 0x5e510 <entry+0x8d10>  @ imm = #0x568
    c.emit(arm32::branch(C::Al,c.at as u32,0x5e510,false)?)?;
 c.check(0x5dfa4)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5DFA4:[u32;2]=[0x00202002,0x00202003];
 for value in INLINE_DATA_5DFA4{c.emit(value)?;}
 // Executable A32 block 0x5dfac..0x5e000.
 c.check(0x5dfac)?;
    // 0x5dfac: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x5dfb0: add r1, r10, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,32)?)?;
    // 0x5dfb4: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x5dfb8: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5dfbc: blo 0x5e864 <entry+0x9064>  @ imm = #0x8a0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5e864,false)?)?;
    // 0x5dfc0: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x5dfc4: ldr r1, [sp, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,180)?)?;
    // 0x5dfc8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5dfcc: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x5dfd0: ldr r1, [sp, #0xb8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,184)?)?;
    // 0x5dfd4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5dfd8: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x5dfdc: ldr r1, [sp, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,188)?)?;
    // 0x5dfe0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5dfe4: ldr r0, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,16)?)?;
    // 0x5dfe8: ldr r1, [sp, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,192)?)?;
    // 0x5dfec: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5dff0: ldr r0, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,20)?)?;
    // 0x5dff4: ldr r1, [sp, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,196)?)?;
    // 0x5dff8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x5dffc: ldr r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,24)?)?;
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
