// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x56000,0x56800)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x56000;
pub const END:usize=0x56800;
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
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x56000..0x561b0.
 c.check(0x56000)?;
    // 0x56000: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x56004: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x56008: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x5600c: ldrb r12, [r4, #0x99c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(4)?,2460)?)?;
    // 0x56010: ldrb r8, [r4, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(4)?,2462)?)?;
    // 0x56014: ldrb r2, [r4, #0x9a2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2466)?)?;
    // 0x56018: lsr r3, r8, r12
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::new(8)?,Shift::Lsr,R::R12,false)?)?;
    // 0x5601c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x56020: and r5, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,255)?)?;
    // 0x56024: add r7, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R1,1)?)?;
    // 0x56028: add r0, r2, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x5602c: cmp r3, r6, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,R::R0,false)?)?;
    // 0x56030: bgt 0x5601c <entry+0x81c>   @ imm = #-0x1c
    c.emit(arm32::branch(C::Gt,c.at as u32,0x5601c,false)?)?;
    // 0x56034: add r0, r12, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x56038: strb r0, [r4, #0x99c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2460)?)?;
    // 0x5603c: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x56040: strb r7, [r4, #0x99d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,2461)?)?;
    // 0x56044: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x56048: bne 0x56060 <entry+0x860>   @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56060,false)?)?;
    // 0x5604c: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // 0x56050: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56054: streq r0, [r9, #0x428]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,1064)?)?;
    // 0x56058: streq r0, [r9, #0x42c]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,1068)?)?;
    // 0x5605c: streq r0, [r9, #0x430]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,1072)?)?;
    // 0x56060: ldr r0, [r9, #0x1ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,492)?)?;
    // 0x56064: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56068: beq 0x561cc <entry+0x9cc>   @ imm = #0x15c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x561cc,false)?)?;
    // 0x5606c: ldrb r0, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,988)?)?;
    // 0x56070: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x56074: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x56078: rsb r2, r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R2,R::new(8)?,0)?)?;
    // 0x5607c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56080: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x56084: movne r1, #10
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,10)?)?;
    // 0x56088: strb r1, [r4, #0x99f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2463)?)?;
    // 0x5608c: strb r0, [r4, #0x9a0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2464)?)?;
    // 0x56090: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x56094: strb r0, [r4, #0x9a1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2465)?)?;
    // 0x56098: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5609c: strb r0, [r4, #0x9a3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2467)?)?;
    // 0x560a0: ldr r1, [r9, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,92)?)?;
    // 0x560a4: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x560a8: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x560ac: strb r1, [r4, #0x9a4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2468)?)?;
    // 0x560b0: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x560b4: strb r5, [r4, #0x9a5]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2469)?)?;
    // 0x560b8: strb r5, [r4, #0x9a7]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2471)?)?;
    // 0x560bc: strb r1, [r4, #0x9a9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2473)?)?;
    // 0x560c0: strb r0, [r4, #0x9a6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2470)?)?;
    // 0x560c4: sub r1, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(8)?,1)?)?;
    // 0x560c8: ldr r9, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,716)?)?;
    // 0x560cc: add r0, r1, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x560d0: and r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x560d4: str r0, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2652)?)?;
    // 0x560d8: ldr r10, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(4)?,720)?)?;
    // 0x560dc: add r1, r10, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(10)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x560e0: and r7, r1, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x560e4: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x560e8: str r7, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2656)?)?;
    // 0x560ec: bl 0x81838 <entry+0x2c038> @ imm = #0x2b744
    c.emit(arm32::branch(C::Al,c.at as u32,0x81838,true)?)?;
    // 0x560f0: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x560f4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x560f8: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x560fc: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x56100: addne r6, r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x56104: str r6, [r4, #0xa64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,2660)?)?;
    // 0x56108: bl 0x81838 <entry+0x2c038> @ imm = #0x2b728
    c.emit(arm32::branch(C::Al,c.at as u32,0x81838,true)?)?;
    // 0x5610c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x56110: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x56114: add r1, r10, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,15)?)?;
    // 0x56118: addne r7, r7, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x5611c: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x56120: mul r0, r7, r6
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(7)?,R::new(6)?)?)?;
    // 0x56124: str r7, [r4, #0xa68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2664)?)?;
    // 0x56128: str r0, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2668)?)?;
    // 0x5612c: add r0, r9, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,15)?)?;
    // 0x56130: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,4)?)?;
    // 0x56134: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x56138: str r0, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1196)?)?;
    // 0x5613c: str r1, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1200)?)?;
    // 0x56140: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x56144: str r2, [r4, #0x4b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1204)?)?;
    // 0x56148: ldr r6, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,664)?)?;
    // 0x5614c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x56150: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2daf4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x56154: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x56158: beq 0x56188 <entry+0x988>   @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56188,false)?)?;
    // 0x5615c: mul r1, r0, r6
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(6)?)?)?;
    // 0x56160: add r2, r4, #2672
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,2672)?)?;
    // 0x56164: sub r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x56168: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5616c: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x56170: addlt r3, r3, #1
    c.emit(arm32::data_immediate(C::Lt,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x56174: str r3, [r2, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x56178: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x5617c: ldr r3, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,664)?)?;
    // 0x56180: cmp r5, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x56184: blo 0x56168 <entry+0x968>   @ imm = #-0x24
    c.emit(arm32::branch(C::Cc,c.at as u32,0x56168,false)?)?;
    // 0x56188: ldr r0, [r4, #0xd94]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3476)?)?;
    // 0x5618c: str r0, [r4, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1244)?)?;
    // 0x56190: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x56194: ldr r7, [r4, #0xd98]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,3480)?)?;
    // 0x56198: str r7, [r4, #0x4e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1248)?)?;
    // 0x5619c: ldr r8, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-32)?)?;
    // 0x561a0: bhi 0x56340 <entry+0xb40>   @ imm = #0x198
    c.emit(arm32::branch(C::Hi,c.at as u32,0x56340,false)?)?;
    // 0x561a4: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x561a8: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x561ac: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
 c.check(0x561b0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_561B0:[u32;4]=[0x000561c0,0x00056224,0x000562bc,0x00056310];
 for value in INLINE_DATA_561B0{c.emit(value)?;}
 // Executable A32 block 0x561c0..0x56304.
 c.check(0x561c0)?;
    // 0x561c0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x561c4: str r0, [r4, #0x4e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1248)?)?;
    // 0x561c8: b 0x56340 <entry+0xb40>   @ imm = #0x170
    c.emit(arm32::branch(C::Al,c.at as u32,0x56340,false)?)?;
    // 0x561cc: ldr r0, [r9, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,508)?)?;
    // 0x561d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x561d4: beq 0x56208 <entry+0xa08>   @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56208,false)?)?;
    // 0x561d8: ldr r0, [r9, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,540)?)?;
    // 0x561dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x561e0: bne 0x5606c <entry+0x86c>   @ imm = #-0x17c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5606c,false)?)?;
    // 0x561e4: ldr r0, [r9, #0x214]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,532)?)?;
    // 0x561e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x561ec: bne 0x5606c <entry+0x86c>   @ imm = #-0x188
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5606c,false)?)?;
    // 0x561f0: ldr r0, [r9, #0x20c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,524)?)?;
    // 0x561f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x561f8: bne 0x5606c <entry+0x86c>   @ imm = #-0x194
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5606c,false)?)?;
    // 0x561fc: ldr r0, [r9, #0x204]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,516)?)?;
    // 0x56200: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56204: bne 0x5606c <entry+0x86c>   @ imm = #-0x1a0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5606c,false)?)?;
    // 0x56208: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5620c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x56210: mov r2, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // 0x56214: str r0, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,20)?)?;
    // 0x56218: add r0, r9, #444
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,444)?)?;
    // 0x5621c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x53038
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x56220: b 0x5606c <entry+0x86c>   @ imm = #-0x1bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x5606c,false)?)?;
    // 0x56224: ldr r5, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,2668)?)?;
    // 0x56228: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5622c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x56230: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2da14
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x56234: mul r1, r0, r7
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(7)?)?)?;
    // 0x56238: cmp r1, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x5623c: addlo r0, r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Add,R::R0,R::R0,1)?)?;
    // 0x56240: str r0, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1572)?)?;
    // 0x56244: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x56248: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x5624c: bne 0x56340 <entry+0xb40>   @ imm = #0xec
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56340,false)?)?;
    // 0x56250: ldr r0, [r4, #0x3a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,936)?)?;
    // 0x56254: ldr r5, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,928)?)?;
    // 0x56258: sub r9, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::new(7)?,1)?)?;
    // 0x5625c: add r1, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,31)?)?;
    // 0x56260: add r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,31)?)?;
    // 0x56264: lsr r1, r1, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,5)?)?;
    // 0x56268: lsr r6, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::R0,Shift::Lsr,5)?)?;
    // 0x5626c: mla r0, r6, r1, r9
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(6)?,R::R1,R::new(9)?)?)?;
    // 0x56270: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x56274: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2d9d0
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x56278: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // 0x5627c: ldr r0, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2652)?)?;
    // 0x56280: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x56284: bl 0x81850 <entry+0x2c050> @ imm = #0x2b5c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x56288: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x5628c: add r0, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,31)?)?;
    // 0x56290: lsr r1, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsr,5)?)?;
    // 0x56294: mla r0, r6, r1, r9
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(6)?,R::R1,R::new(9)?)?)?;
    // 0x56298: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x5629c: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2d9a8
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x562a0: mla r1, r8, r5, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(8)?,R::new(5)?,R::R0)?)?;
    // 0x562a4: ldr r8, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-32)?)?;
    // 0x562a8: str r1, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1572)?)?;
    // 0x562ac: ldr r0, [r4, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,944)?)?;
    // 0x562b0: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x562b4: strlo r0, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Cc,false,R::R0,R::new(4)?,1572)?)?;
    // 0x562b8: b 0x56340 <entry+0xb40>   @ imm = #0x80
    c.emit(arm32::branch(C::Al,c.at as u32,0x56340,false)?)?;
    // 0x562bc: ldrb r0, [r4, #0x413]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1043)?)?;
    // 0x562c0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x562c4: bne 0x56340 <entry+0xb40>   @ imm = #0x74
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56340,false)?)?;
    // 0x562c8: ldr r1, [pc, #0xfb4]        @ 0x1a84 <entry+0x1a84>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4020)?)?;
    // 0x562cc: ldr r2, [pc, #0x9a0]        @ 0x1474 <entry+0x1474>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2464)?)?;
    // 0x562d0: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x562d4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x562d8: orr r3, r3, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3328)?)?;
    // 0x562dc: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x27650
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x562e0: ldr r1, [pc, #0x690]        @ 0x1178 <entry+0x1178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1680)?)?;
    // 0x562e4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x562e8: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x2765c
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x562ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x562f0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x562f4: str r0, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1572)?)?;
    // 0x562f8: strb r1, [r4, #0x413]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1043)?)?;
    // 0x562fc: strb r0, [r4, #0x579]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1401)?)?;
    // 0x56300: b 0x56340 <entry+0xb40>   @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x56340,false)?)?;
 c.check(0x56304)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56304:[u32;3]=[0xfffff558,0xfffff55a,0xfffff55c];
 for value in INLINE_DATA_56304{c.emit(value)?;}
 // Executable A32 block 0x56310..0x56778.
 c.check(0x56310)?;
    // 0x56310: ldr r1, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1196)?)?;
    // 0x56314: ldr r0, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1200)?)?;
    // 0x56318: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x5631c: mul r2, r0, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x56320: cmp r2, #1584
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1584)?)?;
    // 0x56324: movls r3, #2
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x56328: cmp r2, #396
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,396)?)?;
    // 0x5632c: movls r3, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x56330: mul r1, r3, r7
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(3)?,R::new(7)?)?)?;
    // 0x56334: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2d910
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x56338: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5633c: str r0, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1572)?)?;
    // 0x56340: ldrb r1, [r4, #0x3dd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,989)?)?;
    // 0x56344: ldr r0, [r4, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1244)?)?;
    // 0x56348: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5634c: cmpeq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x56350: bne 0x56390 <entry+0xb90>   @ imm = #0x38
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56390,false)?)?;
    // 0x56354: ldr r0, [r4, #0x4e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1248)?)?;
    // 0x56358: ldr r1, [r4, #0xa64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2660)?)?;
    // 0x5635c: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2d8e8
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x56360: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x56364: str r5, [r4, #0x4e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1248)?)?;
    // 0x56368: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5636c: ldr r6, [r4, #0xa68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2664)?)?;
    // 0x56370: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x56374: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2d8d0
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x56378: mul r1, r6, r5
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(6)?,R::new(5)?)?)?;
    // 0x5637c: str r0, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1572)?)?;
    // 0x56380: cmp r1, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x56384: bhs 0x563a0 <entry+0xba0>   @ imm = #0x14
    c.emit(arm32::branch(C::Cs,c.at as u32,0x563a0,false)?)?;
    // 0x56388: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5638c: b 0x5639c <entry+0xb9c>   @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5639c,false)?)?;
    // 0x56390: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x56394: bne 0x563a0 <entry+0xba0>   @ imm = #0x4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x563a0,false)?)?;
    // 0x56398: ldr r0, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2668)?)?;
    // 0x5639c: str r0, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1572)?)?;
    // 0x563a0: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x563a4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x563a8: mvn r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,255)?)?;
    // 0x563ac: str r0, [r4, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1224)?)?;
    // 0x563b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x563b4: str r1, [r4, #0x4cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1228)?)?;
    // 0x563b8: str r2, [r4, #0x4d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1232)?)?;
    // 0x563bc: strb r0, [r4, #0x3f8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1016)?)?;
    // 0x563c0: ldrb r0, [r4, #0x3fa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1018)?)?;
    // 0x563c4: ldrb r2, [r4, #0x3fc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1020)?)?;
    // 0x563c8: ldrb r1, [r4, #0x3fb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1019)?)?;
    // 0x563cc: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x563d0: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x563d4: ldr r1, [pc, #0xf28]        @ 0x1b04 <entry+0x1b04>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3880)?)?;
    // 0x563d8: strb r0, [r4, #0x57b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1403)?)?;
    // 0x563dc: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x563e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x563e4: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x27758
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x563e8: ldr r1, [pc, #0x588]        @ 0x1178 <entry+0x1178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1416)?)?;
    // 0x563ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x563f0: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x27764
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x563f4: ldr r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
    // 0x563f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x563fc: beq 0x56408 <entry+0xc08>   @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56408,false)?)?;
    // 0x56400: ldrb r0, [r4, #0xdfa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3578)?)?;
    // 0x56404: b 0x56418 <entry+0xc18>   @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x56418,false)?)?;
    // 0x56408: mov r0, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,221)?)?;
    // 0x5640c: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x56410: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x56414: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x56418: strb r0, [r4, #0x561]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1377)?)?;
    // 0x5641c: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x56420: ldrb r1, [r4, #0xe41]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3649)?)?;
    // 0x56424: strb r1, [r4, #0x399]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,921)?)?;
    // 0x56428: ldrb r1, [r4, #0xe40]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3648)?)?;
    // 0x5642c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x56430: strb r1, [r4, #0x398]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,920)?)?;
    // 0x56434: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x56438: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x5643c: strb r1, [r4, #0x99b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2459)?)?;
    // 0x56440: ldrb r1, [r4, #0x468]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1128)?)?;
    // 0x56444: sub r2, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,15)?)?;
    // 0x56448: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x5644c: blo 0x56474 <entry+0xc74>   @ imm = #0x20
    c.emit(arm32::branch(C::Cc,c.at as u32,0x56474,false)?)?;
    // 0x56450: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x56454: beq 0x56474 <entry+0xc74>   @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56474,false)?)?;
    // 0x56458: cmp r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,11)?)?;
    // 0x5645c: bne 0x5649c <entry+0xc9c>   @ imm = #0x38
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5649c,false)?)?;
    // 0x56460: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x56464: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x56468: blo 0x5649c <entry+0xc9c>   @ imm = #0x2c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5649c,false)?)?;
    // 0x5646c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56470: b 0x564a0 <entry+0xca0>   @ imm = #0x28
    c.emit(arm32::branch(C::Al,c.at as u32,0x564a0,false)?)?;
    // 0x56474: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x56478: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x5647c: bne 0x5649c <entry+0xc9c>   @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5649c,false)?)?;
    // 0x56480: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x56484: cmp r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // 0x56488: moveq r0, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x5648c: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // 0x56490: moveq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x56494: strb r0, [r4, #0x99b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2459)?)?;
    // 0x56498: b 0x564a4 <entry+0xca4>   @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x564a4,false)?)?;
    // 0x5649c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x564a0: str r0, [r4, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1236)?)?;
    // 0x564a4: ldr r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,908)?)?;
    // 0x564a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x564ac: beq 0x56504 <entry+0xd04>   @ imm = #0x50
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56504,false)?)?;
    // 0x564b0: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x564b4: ldr r6, [pc, #0xfc4]        @ 0x1c80 <entry+0x1c80>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,4036)?)?;
    // 0x564b8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x564bc: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x564c0: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x564c4: ldrne r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,2744)?)?;
    // 0x564c8: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x564cc: strne r1, [r0, #0x33c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,828)?)?;
    // 0x564d0: mov r1, #74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,74)?)?;
    // 0x564d4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x564d8: strb r5, [r4, #0x3e2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,994)?)?;
    // 0x564dc: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x564e0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x54d38
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x564e4: ldrb r0, [r4, #0x40e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1038)?)?;
    // 0x564e8: mov r1, #75
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,75)?)?;
    // 0x564ec: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x564f0: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x564f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x564f8: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x564fc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x56500: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x54d58
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x56504: mov r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,60)?)?;
    // 0x56508: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x5650c: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x56510: cmp r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,10)?)?;
    // 0x56514: beq 0x5652c <entry+0xd2c>   @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5652c,false)?)?;
    // 0x56518: cmp r0, #30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,30)?)?;
    // 0x5651c: beq 0x56534 <entry+0xd34>   @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56534,false)?)?;
    // 0x56520: cmp r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x56524: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56528: b 0x56538 <entry+0xd38>   @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x56538,false)?)?;
    // 0x5652c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x56530: b 0x56538 <entry+0xd38>   @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x56538,false)?)?;
    // 0x56534: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56538: mov r1, #225
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,225)?)?;
    // 0x5653c: str r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // 0x56540: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x56544: ldrb r1, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x56548: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5654c: addeq r0, r0, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::R0,3)?)?;
    // 0x56550: streq r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,24)?)?;
    // 0x56554: ldrb r0, [r4, #0x401]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1025)?)?;
    // 0x56558: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5655c: movne r0, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x56560: strbne r0, [r4, #0xf80]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,3968)?)?;
    // 0x56564: strbne r0, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,1156)?)?;
    // 0x56568: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5656c: ldrb r1, [r4, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1148)?)?;
    // 0x56570: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x56574: bne 0x56588 <entry+0xd88>   @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56588,false)?)?;
    // 0x56578: ldrb r1, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1156)?)?;
    // 0x5657c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56580: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x56584: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56588: strb r0, [r4, #0x9a7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2471)?)?;
    // 0x5658c: ldrb r0, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1173)?)?;
    // 0x56590: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56594: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56598: strbne r0, [r4, #0x9a7]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,2471)?)?;
    // 0x5659c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x565a0: str r0, [r4, #0x574]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1396)?)?;
    // 0x565a4: ldrb r0, [r4, #0xe8d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3725)?)?;
    // 0x565a8: strb r0, [r4, #0x3e5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,997)?)?;
    // 0x565ac: ldr r0, [r4, #0xdc8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3528)?)?;
    // 0x565b0: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // 0x565b4: str r0, [r4, #0x550]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1360)?)?;
    // 0x565b8: ldr r0, [r4, #0xdcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3532)?)?;
    // 0x565bc: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // 0x565c0: str r0, [r4, #0x554]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1364)?)?;
    // 0x565c4: ldr r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
    // 0x565c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x565cc: bne 0x565e0 <entry+0xde0>   @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x565e0,false)?)?;
    // 0x565d0: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x565d4: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x565d8: cmp r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,17)?)?;
    // 0x565dc: bne 0x56614 <entry+0xe14>   @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56614,false)?)?;
    // 0x565e0: ldr r0, [r4, #0xed0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3792)?)?;
    // 0x565e4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x565e8: beq 0x565f8 <entry+0xdf8>   @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x565f8,false)?)?;
    // 0x565ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x565f0: strb r0, [r4, #0xdc4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3524)?)?;
    // 0x565f4: b 0x56618 <entry+0xe18>   @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x56618,false)?)?;
    // 0x565f8: ldr r0, [r4, #0xec4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3780)?)?;
    // 0x565fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56600: beq 0x567cc <entry+0xfcc>   @ imm = #0x1c4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x567cc,false)?)?;
    // 0x56604: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x56608: strb r0, [r4, #0xdc4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3524)?)?;
    // 0x5660c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x56610: b 0x56618 <entry+0xe18>   @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x56618,false)?)?;
    // 0x56614: ldrb r0, [r4, #0xdc4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3524)?)?;
    // 0x56618: strb r0, [r4, #0x544]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1348)?)?;
    // 0x5661c: ldrb r0, [r4, #0x399]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,921)?)?;
    // 0x56620: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x56624: bne 0x566c0 <entry+0xec0>   @ imm = #0x94
    c.emit(arm32::branch(C::Ne,c.at as u32,0x566c0,false)?)?;
    // 0x56628: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x5662c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x56630: bne 0x56668 <entry+0xe68>   @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56668,false)?)?;
    // 0x56634: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x56638: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x5663c: str r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,908)?)?;
    // 0x56640: mov r0, #920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,920)?)?;
    // 0x56644: ldrsb r0, [r4, r0]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::R0,R::new(4)?,R::R0)?)?;
    // 0x56648: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x5664c: add r0, r1, r0, asr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Asr,1,false)?)?;
    // 0x56650: str r0, [r4, #0xe38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3640)?)?;
    // 0x56654: str r0, [r4, #0x390]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,912)?)?;
    // 0x56658: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x5665c: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x56660: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x56664: b 0x566c0 <entry+0xec0>   @ imm = #0x54
    c.emit(arm32::branch(C::Al,c.at as u32,0x566c0,false)?)?;
    // 0x56668: ldr r1, [pc, #0xfec]        @ 0x1e5c <entry+0x1e5c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4076)?)?;
    // 0x5666c: ldr r2, [pc, #0x600]        @ 0x1474 <entry+0x1474>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1536)?)?;
    // 0x56670: mov r3, #163
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,163)?)?;
    // 0x56674: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x56678: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5667c: orr r3, r3, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3328)?)?;
    // 0x56680: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x279f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x56684: ldr r1, [pc, #0x2ec]        @ 0x1178 <entry+0x1178>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,748)?)?;
    // 0x56688: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x5668c: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x27a00
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x56690: ldr r3, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,24)?)?;
    // 0x56694: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x56698: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x5669c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x566a0: add r3, r3, #43
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,43)?)?;
    // 0x566a4: str r3, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // 0x566a8: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x566ac: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x566b0: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x566b4: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x566b8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x566bc: bl 0x2a990 <entry+0xfffffffffffd5190> @ imm = #-0x2bd34
    c.emit(arm32::branch(C::Al,c.at as u32,0x2a990,true)?)?;
    // 0x566c0: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x566c4: bl 0x3580c <entry+0xfffffffffffe000c> @ imm = #-0x20ec0
    c.emit(arm32::branch(C::Al,c.at as u32,0x3580c,true)?)?;
    // 0x566c8: ldr r1, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,280)?)?;
    // 0x566cc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x566d0: bne 0x566e8 <entry+0xee8>   @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x566e8,false)?)?;
    // 0x566d4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x566d8: str r0, [r4, #0x5f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1520)?)?;
    // 0x566dc: str r0, [r4, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1524)?)?;
    // 0x566e0: b 0x566f4 <entry+0xef4>   @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x566f4,false)?)?;
    // 0x566e4: andeq r1, pc, r0, lsr r7
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::R1,R::PC,R::R0,Shift::Lsr,R::new(7)?,false)?)?;
    // 0x566e8: ldr r1, [r4, #0x5f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1520)?)?;
    // 0x566ec: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x566f0: strhi r0, [r4, #0x5f0]
    c.emit(arm32::transfer_word(C::Hi,false,R::R0,R::new(4)?,1520)?)?;
    // 0x566f4: ldr r0, [r4, #0x608]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1544)?)?;
    // 0x566f8: str r0, [r4, #0x60c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1548)?)?;
    // 0x566fc: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x56700: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x56704: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56708: bne 0x56d1c <entry+0x151c>  @ imm = #0x60c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56d1c,false)?)?;
    // 0x5670c: ldr r0, [r4, #0xd34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3380)?)?;
    // 0x56710: ldr r5, [r4, #0xd30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,3376)?)?;
    // 0x56714: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x56718: beq 0x567ac <entry+0xfac>   @ imm = #0x8c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x567ac,false)?)?;
    // 0x5671c: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x56720: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x56724: beq 0x56818 <entry+0x1018>  @ imm = #0xec
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56818,false)?)?;
    // 0x56728: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x5672c: beq 0x567bc <entry+0xfbc>   @ imm = #0x88
    c.emit(arm32::branch(C::Eq,c.at as u32,0x567bc,false)?)?;
    // 0x56730: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x56734: bne 0x5684c <entry+0x104c>  @ imm = #0x110
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5684c,false)?)?;
    // 0x56738: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5673c: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x56740: cmp r5, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,15)?)?;
    // 0x56744: str r1, [r4, #0x56c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1388)?)?;
    // 0x56748: strb r1, [r2, #0x55b]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R1,R::R2,1371)?)?;
    // 0x5674c: strb r1, [r2, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,3)?)?;
    // 0x56750: strb r1, [r2, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,2)?)?;
    // 0x56754: strb r1, [r2, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,1)?)?;
    // 0x56758: strb r1, [r2, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,4)?)?;
    // 0x5675c: bgt 0x56880 <entry+0x1080>  @ imm = #0x11c
    c.emit(arm32::branch(C::Gt,c.at as u32,0x56880,false)?)?;
    // 0x56760: sub r1, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(5)?,1)?)?;
    // 0x56764: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // 0x56768: bhi 0x569a4 <entry+0x11a4>  @ imm = #0x234
    c.emit(arm32::branch(C::Hi,c.at as u32,0x569a4,false)?)?;
    // 0x5676c: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x56770: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // 0x56774: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
 c.check(0x56778)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_56778:[u32;8]=[0x00056798,0x000569fc,0x000569a4,0x000569a4,0x000569a4,0x000569a4,0x000569a4,0x00056a08];
 for value in INLINE_DATA_56778{c.emit(value)?;}
 // Executable A32 block 0x56798..0x56800.
 c.check(0x56798)?;
    // 0x56798: mov r1, #66
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,66)?)?;
    // 0x5679c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x567a0: str r1, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,648)?)?;
    // 0x567a4: strb r2, [r4, #0x55b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1371)?)?;
    // 0x567a8: b 0x56a38 <entry+0x1238>  @ imm = #0x288
    c.emit(arm32::branch(C::Al,c.at as u32,0x56a38,false)?)?;
    // 0x567ac: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x567b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x567b4: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x567b8: bne 0x56860 <entry+0x1060>  @ imm = #0xa0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56860,false)?)?;
    // 0x567bc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x567c0: strb r1, [r4, #0x338]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,824)?)?;
    // 0x567c4: str r5, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,648)?)?;
    // 0x567c8: b 0x56844 <entry+0x1044>  @ imm = #0x74
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x567cc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x567d0: b 0x565f0 <entry+0xdf0>   @ imm = #-0x1e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x565f0,false)?)?;
    // 0x567d4: mov r2, #222
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,222)?)?;
    // 0x567d8: orr r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4864)?)?;
    // 0x567dc: ldrb r2, [r4, r2]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R2,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x567e0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x567e4: beq 0x56920 <entry+0x1120>  @ imm = #0x134
    c.emit(arm32::branch(C::Eq,c.at as u32,0x56920,false)?)?;
    // 0x567e8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x567ec: bne 0x56934 <entry+0x1134>  @ imm = #0x140
    c.emit(arm32::branch(C::Ne,c.at as u32,0x56934,false)?)?;
    // 0x567f0: ldr r2, [r4, #0xe44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,3652)?)?;
    // 0x567f4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x567f8: beq 0x569d4 <entry+0x11d4>  @ imm = #0x1d4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x569d4,false)?)?;
    // 0x567fc: ldrb r1, [r4, #0xde4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3556)?)?;
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
