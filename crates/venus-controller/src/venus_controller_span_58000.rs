// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x58000,0x59000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x58000;
pub const END:usize=0x59000;
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
fn emit_smull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 SMULL operands".into())}
 Ok(((cond as u32)<<28)|0x00c0_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x58000..0x58084.
 c.check(0x58000)?;
    // 0x58000: smull r5, r2, r6, r1
    c.emit(emit_smull(C::Al,R::new(5)?,R::R2,R::new(6)?,R::R1)?)?;
    // 0x58004: asr r1, r2, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Asr,7)?)?;
    // 0x58008: add r1, r1, r2, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsr,31,false)?)?;
    // 0x5800c: add r1, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x58010: lsl r2, r1, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,6)?)?;
    // 0x58014: str r2, [r4, #0xb98]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2968)?)?;
    // 0x58018: mla r2, r3, r12, lr
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::new(3)?,R::R12,R::LR)?)?;
    // 0x5801c: umull r3, r6, r2, r7
    c.emit(emit_umull(C::Al,R::new(3)?,R::new(6)?,R::R2,R::new(7)?)?)?;
    // 0x58020: lsr r2, r6, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(6)?,Shift::Lsr,8)?)?;
    // 0x58024: add r2, r2, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,2,false)?)?;
    // 0x58028: lsl r2, r2, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,6)?)?;
    // 0x5802c: str r2, [r4, #0xb9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2972)?)?;
    // 0x58030: lsleq r2, r1, #6
    c.emit(arm32::mov_shifted_register(C::Eq,R::R2,R::R1,Shift::Lsl,6)?)?;
    // 0x58034: str r2, [r4, #0xb94]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2964)?)?;
    // 0x58038: b 0x57e88 <entry+0x2688>  @ imm = #-0x1b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x57e88,false)?)?;
    // 0x5803c: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x58040: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x58044: b 0x57730 <entry+0x1f30>  @ imm = #-0x91c
    c.emit(arm32::branch(C::Al,c.at as u32,0x57730,false)?)?;
    // 0x58048: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5804c: beq 0x58088 <entry+0x2888>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58088,false)?)?;
    // 0x58050: ldr r1, [r4, #0xd90]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3472)?)?;
    // 0x58054: and r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,7)?)?;
    // 0x58058: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5805c: bne 0x58088 <entry+0x2888>  @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58088,false)?)?;
    // 0x58060: ldr r1, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1392)?)?;
    // 0x58064: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x58068: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x5806c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x58070: moveq r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x58074: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x58078: strb r7, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3104)?)?;
    // 0x5807c: str r1, [r4, #0xec0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3776)?)?;
    // 0x58080: b 0x58094 <entry+0x2894>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x58094,false)?)?;
 c.check(0x58084)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_58084:[u32;1]=[0xfffff000];
 for value in INLINE_DATA_58084{c.emit(value)?;}
 // Executable A32 block 0x58088..0x58104.
 c.check(0x58088)?;
    // 0x58088: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x5808c: strb r7, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3104)?)?;
    // 0x58090: str r7, [r4, #0xec0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,3776)?)?;
    // 0x58094: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x58098: strb r1, [r4, #0x494]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1172)?)?;
    // 0x5809c: b 0x54eb4 <entry+0xfffffffffffff6b4> @ imm = #-0x31f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x54eb4,false)?)?;
    // 0x580a0: mov r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,63)?)?;
    // 0x580a4: b 0x56844 <entry+0x1044>  @ imm = #-0x1868
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x580a8: mov r0, #90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,90)?)?;
    // 0x580ac: b 0x56844 <entry+0x1044>  @ imm = #-0x1870
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x580b0: mov r0, #150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,150)?)?;
    // 0x580b4: b 0x56844 <entry+0x1044>  @ imm = #-0x1878
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x580b8: mov r0, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,93)?)?;
    // 0x580bc: b 0x56844 <entry+0x1044>  @ imm = #-0x1880
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x580c0: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // 0x580c4: b 0x56844 <entry+0x1044>  @ imm = #-0x1888
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x580c8: mov r0, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,180)?)?;
    // 0x580cc: b 0x56844 <entry+0x1044>  @ imm = #-0x1890
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x580d0: mov r0, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,183)?)?;
    // 0x580d4: b 0x56844 <entry+0x1044>  @ imm = #-0x1898
    c.emit(arm32::branch(C::Al,c.at as u32,0x56844,false)?)?;
    // 0x580d8: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x580dc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x580e0: strb r7, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3104)?)?;
    // 0x580e4: ldrb r0, [r4, #0xdf9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3577)?)?;
    // 0x580e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x580ec: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x580f0: str r1, [r4, #0xec0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3776)?)?;
    // 0x580f4: strb r0, [r4, #0x494]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1172)?)?;
    // 0x580f8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x580fc: strb r0, [r4, #0x493]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1171)?)?;
    // 0x58100: b 0x54f50 <entry+0xfffffffffffff750> @ imm = #-0x31b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x54f50,false)?)?;
 c.check(0x58104)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_58104:[u32;11]=[0x000e2788,0x000ee470,0xfffff000,0x000f147f,0x000f14bf,0xcccccccd,0x0fffffc0,0xfffffe00,0xffffe993,0x66666667,0x000f1720];
 for value in INLINE_DATA_58104{c.emit(value)?;}
 // Executable A32 block 0x58130..0x587f8.
 c.check(0x58130)?;
    // 0x58130: ldr r2, [r0, #0x720]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,1824)?)?;
    // 0x58134: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x58138: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x5813c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x58140: ldrbeq r2, [r0, #0x13a]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R2,R::R0,314)?)?;
    // 0x58144: cmpeq r2, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x58148: beq 0x58154 <entry+0x2954>  @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58154,false)?)?;
    // 0x5814c: mov r3, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x58150: b 0x58158 <entry+0x2958>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x58158,false)?)?;
    // 0x58154: mov r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // 0x58158: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x5815c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x58160: mov lr, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,16384)?)?;
    // 0x58164: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x58168: str r3, [r0, #0x734]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1844)?)?;
    // 0x5816c: str lr, [r0, #0x738]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R0,1848)?)?;
    // 0x58170: str r2, [r0, #0x73c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1852)?)?;
    // 0x58174: ldrb r3, [r0, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,347)?)?;
    // 0x58178: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x5817c: beq 0x58198 <entry+0x2998>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58198,false)?)?;
    // 0x58180: ldr r2, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,40)?)?;
    // 0x58184: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x58188: movne r2, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x5818c: strne r2, [r0, #0x73c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::R0,1852)?)?;
    // 0x58190: movne r2, #256
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,256)?)?;
    // 0x58194: strne r2, [r0, #0x734]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::R0,1844)?)?;
    // 0x58198: mov r12, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,256)?)?;
    // 0x5819c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x581a0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x581a4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x581a8: str r12, [r0, #0x740]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,1856)?)?;
    // 0x581ac: moveq r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x581b0: ldrb r1, [r0, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,314)?)?;
    // 0x581b4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x581b8: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x581bc: ands r2, r4, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::R2,Shift::Lsl,0,true)?)?;
    // 0x581c0: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x581c4: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x581c8: movne r2, #256
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,256)?)?;
    // 0x581cc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x581d0: str r2, [r0, #0x744]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1860)?)?;
    // 0x581d4: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x581d8: str lr, [r0, #0x748]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R0,1864)?)?;
    // 0x581dc: movne r2, #32
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x581e0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x581e4: str r2, [r0, #0x74c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1868)?)?;
    // 0x581e8: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x581ec: ldrne r3, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::R0,40)?)?;
    // 0x581f0: cmpne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x581f4: beq 0x5820c <entry+0x2a0c>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5820c,false)?)?;
    // 0x581f8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x581fc: lsrne r1, r2, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::R1,R::R2,Shift::Lsr,1)?)?;
    // 0x58200: strne r1, [r0, #0x74c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,1868)?)?;
    // 0x58204: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x58208: str r1, [r0, #0x744]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1860)?)?;
    // 0x5820c: str r12, [r0, #0x750]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,1872)?)?;
    // 0x58210: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x58214: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x58218: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x5821c: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x58220: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x58224: ldr r0, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,140)?)?;
    // 0x58228: ldr r8, [pc, #0x5c8]        @ 0x2ff8 <entry+0x2ff8>
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::PC,1480)?)?;
    // 0x5822c: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x58230: blo 0x58268 <entry+0x2a68>  @ imm = #0x30
    c.emit(arm32::branch(C::Cc,c.at as u32,0x58268,false)?)?;
    // 0x58234: sub r1, r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,21)?)?;
    // 0x58238: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x5823c: bhs 0x582c8 <entry+0x2ac8>  @ imm = #0x84
    c.emit(arm32::branch(C::Cs,c.at as u32,0x582c8,false)?)?;
    // 0x58240: ldr r1, [r4, #0x734]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1844)?)?;
    // 0x58244: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x58248: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // 0x5824c: add r5, r2, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsl,2,false)?)?;
    // 0x58250: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58254: beq 0x584b4 <entry+0x2cb4>  @ imm = #0x258
    c.emit(arm32::branch(C::Eq,c.at as u32,0x584b4,false)?)?;
    // 0x58258: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5825c: bl 0x81850 <entry+0x2c050> @ imm = #0x295ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x58260: sub r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58264: b 0x584bc <entry+0x2cbc>  @ imm = #0x250
    c.emit(arm32::branch(C::Al,c.at as u32,0x584bc,false)?)?;
    // 0x58268: ldr r5, [pc, #0x58c]        @ 0x2ffc <entry+0x2ffc>
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::PC,1420)?)?;
    // 0x5826c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x58270: mov r1, #225
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,225)?)?;
    // 0x58274: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x58278: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x5827c: bic r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x58280: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x58284: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x56adc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x58288: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x5828c: mov r1, #226
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,226)?)?;
    // 0x58290: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x58294: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x58298: bic r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5829c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x56af4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x582a0: ldr r1, [r4, #0x734]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1844)?)?;
    // 0x582a4: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x582a8: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // 0x582ac: add r5, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x582b0: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x582b4: beq 0x582f8 <entry+0x2af8>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x582f8,false)?)?;
    // 0x582b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x582bc: bl 0x81850 <entry+0x2c050> @ imm = #0x2958c
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x582c0: sub r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x582c4: b 0x58300 <entry+0x2b00>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x58300,false)?)?;
    // 0x582c8: cmp r0, #25
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,25)?)?;
    // 0x582cc: bne 0x58564 <entry+0x2d64>  @ imm = #0x290
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58564,false)?)?;
    // 0x582d0: ldr r1, [r4, #0x734]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1844)?)?;
    // 0x582d4: ldr r7, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,32)?)?;
    // 0x582d8: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x582dc: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x582e0: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x582e4: beq 0x586ac <entry+0x2eac>  @ imm = #0x3c0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x586ac,false)?)?;
    // 0x582e8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x582ec: bl 0x81850 <entry+0x2c050> @ imm = #0x2955c
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x582f0: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x582f4: b 0x586b4 <entry+0x2eb4>  @ imm = #0x3b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x586b4,false)?)?;
    // 0x582f8: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x582fc: and r1, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x58300: str r1, [r4, #0x754]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1876)?)?;
    // 0x58304: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58308: ldr r2, [r4, #0x738]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1848)?)?;
    // 0x5830c: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x58310: ldr r2, [pc, #0x4e4]        @ 0x2ffc <entry+0x2ffc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1252)?)?;
    // 0x58314: mov r1, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,230)?)?;
    // 0x58318: movle r0, #1
    c.emit(arm32::data_immediate(C::Le,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5831c: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x58320: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x56b78
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x58324: ldr r1, [r4, #0x73c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1852)?)?;
    // 0x58328: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x5832c: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // 0x58330: add r5, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x58334: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58338: beq 0x5834c <entry+0x2b4c>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5834c,false)?)?;
    // 0x5833c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x58340: bl 0x81850 <entry+0x2c050> @ imm = #0x29508
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x58344: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58348: b 0x58354 <entry+0x2b54>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x58354,false)?)?;
    // 0x5834c: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x58350: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x58354: str r0, [r4, #0x758]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1880)?)?;
    // 0x58358: ldr r1, [r4, #0x744]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1860)?)?;
    // 0x5835c: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x58360: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // 0x58364: add r5, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x58368: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5836c: beq 0x58380 <entry+0x2b80>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58380,false)?)?;
    // 0x58370: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x58374: bl 0x81850 <entry+0x2c050> @ imm = #0x294d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x58378: sub r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5837c: b 0x58388 <entry+0x2b88>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x58388,false)?)?;
    // 0x58380: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x58384: and r1, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x58388: str r1, [r4, #0x75c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1884)?)?;
    // 0x5838c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58390: ldr r2, [r4, #0x748]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1864)?)?;
    // 0x58394: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x58398: ldr r2, [pc, #0x45c]        @ 0x2ffc <entry+0x2ffc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1116)?)?;
    // 0x5839c: mov r1, #237
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,237)?)?;
    // 0x583a0: movle r0, #1
    c.emit(arm32::data_immediate(C::Le,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x583a4: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x583a8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x56c00
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x583ac: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x583b0: ldr r1, [r4, #0x74c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1868)?)?;
    // 0x583b4: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // 0x583b8: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x583bc: add r5, r2, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsr,1,false)?)?;
    // 0x583c0: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x583c4: beq 0x583d8 <entry+0x2bd8>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x583d8,false)?)?;
    // 0x583c8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x583cc: bl 0x81850 <entry+0x2c050> @ imm = #0x2947c
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x583d0: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x583d4: b 0x583e0 <entry+0x2be0>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x583e0,false)?)?;
    // 0x583d8: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x583dc: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x583e0: str r0, [r4, #0x760]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1888)?)?;
    // 0x583e4: ldr r0, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,140)?)?;
    // 0x583e8: bic r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,1)?)?;
    // 0x583ec: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x583f0: bne 0x5843c <entry+0x2c3c>  @ imm = #0x44
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5843c,false)?)?;
    // 0x583f4: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x583f8: ldr r6, [pc, #0x3fc]        @ 0x2ffc <entry+0x2ffc>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,1020)?)?;
    // 0x583fc: mov r1, #1012
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1012)?)?;
    // 0x58400: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x58404: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x58408: tst r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,3)?)?;
    // 0x5840c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58410: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x58414: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x58418: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x56c70
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5841c: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x58420: mov r1, #245
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,245)?)?;
    // 0x58424: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x58428: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x5842c: tst r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,7)?)?;
    // 0x58430: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x58434: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x58438: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x56c90
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x5843c: ldrb r9, [r4, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(9)?,R::new(4)?,347)?)?;
    // 0x58440: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x58444: beq 0x585b8 <entry+0x2db8>  @ imm = #0x16c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x585b8,false)?)?;
    // 0x58448: ldr r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x5844c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x58450: bne 0x58460 <entry+0x2c60>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58460,false)?)?;
    // 0x58454: ldrb r0, [r4, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,314)?)?;
    // 0x58458: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5845c: beq 0x58680 <entry+0x2e80>  @ imm = #0x21c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58680,false)?)?;
    // 0x58460: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x58464: ldr r1, [pc, #0x394]        @ 0x3000 <entry+0x3000>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,916)?)?;
    // 0x58468: add r0, r0, #191
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,191)?)?;
    // 0x5846c: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x58470: lsr r0, r3, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,7)?)?;
    // 0x58474: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x58478: lsl r7, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R0,Shift::Lsl,6)?)?;
    // 0x5847c: str r7, [r4, #0x754]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1876)?)?;
    // 0x58480: lsl r0, r7, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x58484: ldr r1, [pc, #0x378]        @ 0x3004 <entry+0x3004>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,888)?)?;
    // 0x58488: smull r2, r3, r0, r1
    c.emit(emit_smull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x5848c: ldr r1, [r4, #0x744]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1860)?)?;
    // 0x58490: add r0, r3, r3, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::new(3)?,Shift::Lsr,31,false)?)?;
    // 0x58494: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // 0x58498: add r5, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5849c: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x584a0: beq 0x585a4 <entry+0x2da4>  @ imm = #0xfc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x585a4,false)?)?;
    // 0x584a4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x584a8: bl 0x81850 <entry+0x2c050> @ imm = #0x293a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x584ac: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x584b0: b 0x585ac <entry+0x2dac>  @ imm = #0xf4
    c.emit(arm32::branch(C::Al,c.at as u32,0x585ac,false)?)?;
    // 0x584b4: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x584b8: and r1, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x584bc: str r1, [r4, #0x754]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1876)?)?;
    // 0x584c0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x584c4: ldr r2, [r4, #0x738]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1848)?)?;
    // 0x584c8: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x584cc: ldr r2, [pc, #0x328]        @ 0x2ffc <entry+0x2ffc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,808)?)?;
    // 0x584d0: mov r1, #87
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,87)?)?;
    // 0x584d4: movle r0, #1
    c.emit(arm32::data_immediate(C::Le,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x584d8: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x584dc: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x56d34
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x584e0: ldr r1, [r4, #0x73c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1852)?)?;
    // 0x584e4: ldr r6, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,36)?)?;
    // 0x584e8: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x584ec: add r5, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x584f0: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x584f4: beq 0x58508 <entry+0x2d08>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58508,false)?)?;
    // 0x584f8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x584fc: bl 0x81850 <entry+0x2c050> @ imm = #0x2934c
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x58500: sub r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58504: b 0x58510 <entry+0x2d10>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x58510,false)?)?;
    // 0x58508: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x5850c: and r1, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x58510: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58514: str r1, [r4, #0x758]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1880)?)?;
    // 0x58518: str r0, [r4, #0x75c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1884)?)?;
    // 0x5851c: str r0, [r4, #0x760]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1888)?)?;
    // 0x58520: ldrb r1, [r4, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,314)?)?;
    // 0x58524: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x58528: beq 0x587b4 <entry+0x2fb4>  @ imm = #0x284
    c.emit(arm32::branch(C::Eq,c.at as u32,0x587b4,false)?)?;
    // 0x5852c: ldr r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    // 0x58530: mov r2, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63)?)?;
    // 0x58534: mov r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15)?)?;
    // 0x58538: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // 0x5853c: add r1, r2, r1, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsr,4,false)?)?;
    // 0x58540: add r2, r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3)?)?;
    // 0x58544: add r2, r3, r2, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsr,2,false)?)?;
    // 0x58548: and r1, r1, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x5854c: bic r2, r2, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,2147483663)?)?;
    // 0x58550: str r1, [r4, #0x764]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1892)?)?;
    // 0x58554: str r2, [r4, #0x768]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1896)?)?;
    // 0x58558: str r0, [r4, #0x76c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1900)?)?;
    // 0x5855c: str r0, [r4, #0x770]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1904)?)?;
    // 0x58560: b 0x587b4 <entry+0x2fb4>  @ imm = #0x24c
    c.emit(arm32::branch(C::Al,c.at as u32,0x587b4,false)?)?;
    // 0x58564: ldr r1, [pc, #0x29c]        @ 0x3008 <entry+0x3008>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,668)?)?;
    // 0x58568: ldr r2, [pc, #0x29c]        @ 0x300c <entry+0x300c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,668)?)?;
    // 0x5856c: mov r3, #109
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,109)?)?;
    // 0x58570: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x58574: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x58578: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x298ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5857c: ldr r1, [pc, #0x28c]        @ 0x3010 <entry+0x3010>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,652)?)?;
    // 0x58580: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x58584: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x298f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x58588: ldr r2, [pc, #0x26c]        @ 0x2ffc <entry+0x2ffc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,620)?)?;
    // 0x5858c: mov r1, #110
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,110)?)?;
    // 0x58590: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58594: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x58598: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x5859c: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x585a0: b 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x56df8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,false)?)?;
    // 0x585a4: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x585a8: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x585ac: str r0, [r4, #0x754]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1876)?)?;
    // 0x585b0: str r7, [r4, #0x75c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1884)?)?;
    // 0x585b4: str r0, [r4, #0x75c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1884)?)?;
    // 0x585b8: ldrb r0, [r4, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,314)?)?;
    // 0x585bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x585c0: beq 0x587b4 <entry+0x2fb4>  @ imm = #0x1ec
    c.emit(arm32::branch(C::Eq,c.at as u32,0x587b4,false)?)?;
    // 0x585c4: ldr r2, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,32)?)?;
    // 0x585c8: mov r12, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,63)?)?;
    // 0x585cc: add r3, r8, #-268435456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(8)?,4026531840)?)?;
    // 0x585d0: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x585d4: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x585d8: add r1, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,31)?)?;
    // 0x585dc: add r1, r12, r1, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R12,R::R1,Shift::Lsr,5,false)?)?;
    // 0x585e0: and r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x585e4: str r1, [r4, #0x764]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1892)?)?;
    // 0x585e8: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x585ec: ldr r5, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,36)?)?;
    // 0x585f0: add r7, r5, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,7)?)?;
    // 0x585f4: add r7, r1, r7, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R1,R::new(7)?,Shift::Lsr,3,false)?)?;
    // 0x585f8: bic r7, r7, #-1073741809
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,3221225487)?)?;
    // 0x585fc: str r7, [r4, #0x768]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1896)?)?;
    // 0x58600: add r7, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R2,1)?)?;
    // 0x58604: add r6, r1, r7, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::new(7)?,Shift::Lsr,1,false)?)?;
    // 0x58608: add r6, r12, r6, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R12,R::new(6)?,Shift::Lsr,4,false)?)?;
    // 0x5860c: and r6, r6, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x58610: str r6, [r4, #0x76c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1900)?)?;
    // 0x58614: add r6, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(5)?,1)?)?;
    // 0x58618: add r0, r0, r6, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsr,1,false)?)?;
    // 0x5861c: add r0, r1, r0, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,3,false)?)?;
    // 0x58620: bic r0, r0, #-1073741809
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,3221225487)?)?;
    // 0x58624: str r0, [r4, #0x770]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1904)?)?;
    // 0x58628: beq 0x587b4 <entry+0x2fb4>  @ imm = #0x184
    c.emit(arm32::branch(C::Eq,c.at as u32,0x587b4,false)?)?;
    // 0x5862c: lsr lr, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::LR,R::new(6)?,Shift::Lsr,1)?)?;
    // 0x58630: ldr r6, [pc, #0x1c8]        @ 0x3000 <entry+0x3000>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,456)?)?;
    // 0x58634: add r2, r2, #47
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,47)?)?;
    // 0x58638: lsr r9, r7, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(7)?,Shift::Lsr,1)?)?;
    // 0x5863c: umull r7, r0, r2, r6
    c.emit(emit_umull(C::Al,R::new(7)?,R::R0,R::R2,R::new(6)?)?)?;
    // 0x58640: add r2, r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,3)?)?;
    // 0x58644: add r0, r12, r0, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::R0,Shift::Lsr,5,false)?)?;
    // 0x58648: add r2, r1, r2, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R2,Shift::Lsr,2,false)?)?;
    // 0x5864c: and r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x58650: add r3, r9, #23
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(9)?,23)?)?;
    // 0x58654: bic r2, r2, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,2147483663)?)?;
    // 0x58658: umull r5, r7, r3, r6
    c.emit(emit_umull(C::Al,R::new(5)?,R::new(7)?,R::new(3)?,R::new(6)?)?)?;
    // 0x5865c: str r0, [r4, #0x764]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1892)?)?;
    // 0x58660: str r2, [r4, #0x768]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1896)?)?;
    // 0x58664: add r3, r12, r7, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R12,R::new(7)?,Shift::Lsr,4,false)?)?;
    // 0x58668: add r7, lr, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::LR,3)?)?;
    // 0x5866c: add r1, r1, r7, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(7)?,Shift::Lsr,2,false)?)?;
    // 0x58670: and r3, r3, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x58674: bic r1, r1, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,2147483663)?)?;
    // 0x58678: str r3, [r4, #0x76c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1900)?)?;
    // 0x5867c: b 0x587b0 <entry+0x2fb0>  @ imm = #0x12c
    c.emit(arm32::branch(C::Al,c.at as u32,0x587b0,false)?)?;
    // 0x58680: ldr r1, [r4, #0x734]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1844)?)?;
    // 0x58684: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x58688: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // 0x5868c: lsl r7, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R0,Shift::Lsl,1)?)?;
    // 0x58690: add r5, r2, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // 0x58694: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58698: beq 0x587bc <entry+0x2fbc>  @ imm = #0x11c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x587bc,false)?)?;
    // 0x5869c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x586a0: bl 0x81850 <entry+0x2c050> @ imm = #0x291a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x586a4: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x586a8: b 0x587c4 <entry+0x2fc4>  @ imm = #0x114
    c.emit(arm32::branch(C::Al,c.at as u32,0x587c4,false)?)?;
    // 0x586ac: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x586b0: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x586b4: str r0, [r4, #0x754]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1876)?)?;
    // 0x586b8: ldr r1, [r4, #0x73c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1852)?)?;
    // 0x586bc: ldr r9, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,36)?)?;
    // 0x586c0: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x586c4: add r5, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x586c8: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x586cc: beq 0x586e0 <entry+0x2ee0>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x586e0,false)?)?;
    // 0x586d0: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x586d4: bl 0x81850 <entry+0x2c050> @ imm = #0x29174
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x586d8: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x586dc: b 0x586e8 <entry+0x2ee8>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x586e8,false)?)?;
    // 0x586e0: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x586e4: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x586e8: str r0, [r4, #0x758]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1880)?)?;
    // 0x586ec: add r10, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(7)?,1)?)?;
    // 0x586f0: ldr r1, [r4, #0x744]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1860)?)?;
    // 0x586f4: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x586f8: add r5, r0, r10, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(10)?,Shift::Lsr,1,false)?)?;
    // 0x586fc: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58700: beq 0x58714 <entry+0x2f14>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58714,false)?)?;
    // 0x58704: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x58708: bl 0x81850 <entry+0x2c050> @ imm = #0x29140
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x5870c: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58710: b 0x5871c <entry+0x2f1c>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5871c,false)?)?;
    // 0x58714: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x58718: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5871c: str r0, [r4, #0x75c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1884)?)?;
    // 0x58720: add r6, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(9)?,1)?)?;
    // 0x58724: ldr r1, [r4, #0x74c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1868)?)?;
    // 0x58728: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x5872c: add r5, r0, r6, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(6)?,Shift::Lsr,1,false)?)?;
    // 0x58730: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58734: beq 0x58748 <entry+0x2f48>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58748,false)?)?;
    // 0x58738: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5873c: bl 0x81850 <entry+0x2c050> @ imm = #0x2910c
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x58740: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58744: b 0x58750 <entry+0x2f50>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x58750,false)?)?;
    // 0x58748: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x5874c: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x58750: str r0, [r4, #0x760]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1888)?)?;
    // 0x58754: ldrb r0, [r4, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,314)?)?;
    // 0x58758: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5875c: beq 0x587b4 <entry+0x2fb4>  @ imm = #0x50
    c.emit(arm32::branch(C::Eq,c.at as u32,0x587b4,false)?)?;
    // 0x58760: add r2, r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,31)?)?;
    // 0x58764: mov r3, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,63)?)?;
    // 0x58768: lsr r1, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(6)?,Shift::Lsr,1)?)?;
    // 0x5876c: lsr r0, r10, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(10)?,Shift::Lsr,1)?)?;
    // 0x58770: add r7, r8, #-268435456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(8)?,4026531840)?)?;
    // 0x58774: mov r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,15)?)?;
    // 0x58778: add r2, r3, r2, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsr,5,false)?)?;
    // 0x5877c: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // 0x58780: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,7)?)?;
    // 0x58784: and r2, r2, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x58788: add r7, r9, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(9)?,7)?)?;
    // 0x5878c: add r0, r3, r0, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsr,4,false)?)?;
    // 0x58790: add r1, r6, r1, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::R1,Shift::Lsr,3,false)?)?;
    // 0x58794: add r7, r6, r7, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsr,3,false)?)?;
    // 0x58798: and r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x5879c: bic r1, r1, #-1073741809
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,3221225487)?)?;
    // 0x587a0: str r2, [r4, #0x764]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1892)?)?;
    // 0x587a4: bic r7, r7, #-1073741809
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,3221225487)?)?;
    // 0x587a8: str r7, [r4, #0x768]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1896)?)?;
    // 0x587ac: str r0, [r4, #0x76c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1900)?)?;
    // 0x587b0: str r1, [r4, #0x770]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1904)?)?;
    // 0x587b4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x587b8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x587bc: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x587c0: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x587c4: str r0, [r4, #0x754]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1876)?)?;
    // 0x587c8: ldr r1, [r4, #0x744]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1860)?)?;
    // 0x587cc: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x587d0: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x587d4: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x587d8: beq 0x587ec <entry+0x2fec>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x587ec,false)?)?;
    // 0x587dc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x587e0: bl 0x81850 <entry+0x2c050> @ imm = #0x29068
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x587e4: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x587e8: b 0x585b4 <entry+0x2db4>  @ imm = #-0x23c
    c.emit(arm32::branch(C::Al,c.at as u32,0x585b4,false)?)?;
    // 0x587ec: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x587f0: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x587f4: b 0x585b4 <entry+0x2db4>  @ imm = #-0x248
    c.emit(arm32::branch(C::Al,c.at as u32,0x585b4,false)?)?;
 c.check(0x587f8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_587F8:[u32;7]=[0x1fffffc0,0x000ee470,0xaaaaaaab,0x55555556,0x000f147f,0x000f14a8,0x000e2788];
 for value in INLINE_DATA_587F8{c.emit(value)?;}
 // Executable A32 block 0x58814..0x58860.
 c.check(0x58814)?;
    // 0x58814: ldr r1, [r0, #0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,204)?)?;
    // 0x58818: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5881c: blt 0x5882c <entry+0x302c>  @ imm = #0x8
    c.emit(arm32::branch(C::Lt,c.at as u32,0x5882c,false)?)?;
    // 0x58820: cmp r1, #101
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,101)?)?;
    // 0x58824: movge r1, #100
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R1,R::R0,100)?)?;
    // 0x58828: b 0x58830 <entry+0x3030>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x58830,false)?)?;
    // 0x5882c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x58830: ldr r2, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,32)?)?;
    // 0x58834: ldr r0, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,36)?)?;
    // 0x58838: ldr r12, [pc, #0x24]        @ 0x3064 <entry+0x3064>
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::PC,36)?)?;
    // 0x5883c: mul r3, r0, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    // 0x58840: ldr r0, [pc, #0x18]         @ 0x3060 <entry+0x3060>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,24)?)?;
    // 0x58844: ldr r2, [pc, #0x1c]         @ 0x3068 <entry+0x3068>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,28)?)?;
    // 0x58848: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5884c: rsb r0, r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,100)?)?;
    // 0x58850: movlo r2, r12
    c.emit(arm32::mov_register(C::Cc,R::R2,R::R12))?;
    // 0x58854: lsl r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,24)?)?;
    // 0x58858: ldrb r0, [r2, r0, asr #24]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::R2,R::R0,Shift::Asr,24)?)?;
    // 0x5885c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
 c.check(0x58860)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_58860:[u32;3]=[0x001fe001,0x000f1590,0x000f15f5];
 for value in INLINE_DATA_58860{c.emit(value)?;}
 // Executable A32 block 0x5886c..0x588fc.
 c.check(0x5886c)?;
    // 0x5886c: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x58870: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58874: ldrb r2, [r1, #0x1e8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,488)?)?;
    // 0x58878: cmp r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,11)?)?;
    // 0x5887c: bxne lr
    c.emit(arm32::bx(C::Ne,R::LR))?;
    // 0x58880: ldrb r1, [r1, #0x2e1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,737)?)?;
    // 0x58884: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58888: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x5888c: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x58890: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x58894: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x58898: beq 0x588f0 <entry+0x30f0>  @ imm = #0x50
    c.emit(arm32::branch(C::Eq,c.at as u32,0x588f0,false)?)?;
    // 0x5889c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x588a0: beq 0x588cc <entry+0x30cc>  @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x588cc,false)?)?;
    // 0x588a4: ldr r2, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,0)?)?;
    // 0x588a8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x588ac: bne 0x588cc <entry+0x30cc>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x588cc,false)?)?;
    // 0x588b0: mov r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,127)?)?;
    // 0x588b4: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x588b8: add r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // 0x588bc: bic r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,127)?)?;
    // 0x588c0: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x588c4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x588c8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x588cc: add r0, r1, #191
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,191)?)?;
    // 0x588d0: ldr r1, [pc, #0x24]         @ 0x30fc <entry+0x30fc>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,36)?)?;
    // 0x588d4: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x588d8: lsr r0, r3, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,7)?)?;
    // 0x588dc: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x588e0: lsl r0, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,6)?)?;
    // 0x588e4: umull r2, r3, r0, r1
    c.emit(emit_umull(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x588e8: mvn r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,3)?)?;
    // 0x588ec: and r0, r0, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x588f0: add r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,127)?)?;
    // 0x588f4: bic r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,127)?)?;
    // 0x588f8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
 c.check(0x588fc)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_588FC:[u32;1]=[0xaaaaaaab];
 for value in INLINE_DATA_588FC{c.emit(value)?;}
 // Executable A32 block 0x58900..0x58c2c.
 c.check(0x58900)?;
    // 0x58900: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x58904: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x58908: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x5890c: ldr r8, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,80)?)?;
    // 0x58910: ldrb r5, [r0, #0x138]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R0,312)?)?;
    // 0x58914: ldr r6, [r0, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,668)?)?;
    // 0x58918: ldr r10, [pc, #0x30c]       @ 0x342c <entry+0x342c>
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::PC,780)?)?;
    // 0x5891c: add r1, r8, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,31)?)?;
    // 0x58920: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // 0x58924: bic r7, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::R1,31)?)?;
    // 0x58928: mul r3, r7, r6
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::new(7)?,R::new(6)?)?)?;
    // 0x5892c: beq 0x58968 <entry+0x3168>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58968,false)?)?;
    // 0x58930: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // 0x58934: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x58938: mov r4, r6
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(6)?))?;
    // 0x5893c: beq 0x58970 <entry+0x3170>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58970,false)?)?;
    // 0x58940: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x58944: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x58948: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x5894c: bne 0x58970 <entry+0x3170>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58970,false)?)?;
    // 0x58950: add r2, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,1)?)?;
    // 0x58954: mov r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,31)?)?;
    // 0x58958: mov r4, r6
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(6)?))?;
    // 0x5895c: add r2, r5, r2, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::R2,Shift::Lsr,1,false)?)?;
    // 0x58960: bic r1, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R2,31)?)?;
    // 0x58964: b 0x58970 <entry+0x3170>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x58970,false)?)?;
    // 0x58968: lsl r4, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(6)?,Shift::Lsl,1)?)?;
    // 0x5896c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x58970: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x58974: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x58978: orr lr, lr, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,3840)?)?;
    // 0x5897c: mla r5, r4, r1, lr
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(4)?,R::R1,R::LR)?)?;
    // 0x58980: ldr r4, [pc, #0x2a8]        @ 0x3430 <entry+0x3430>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,680)?)?;
    // 0x58984: add r3, r3, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x58988: and r5, r5, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x5898c: and r2, r3, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x58990: add r1, r5, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x58994: str r1, [r0, #0x868]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2152)?)?;
    // 0x58998: ldr r3, [r0, #0x82c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,2092)?)?;
    // 0x5899c: str r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // 0x589a0: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x589a4: str r7, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,20)?)?;
    // 0x589a8: mov r12, r3
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(3)?))?;
    // 0x589ac: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x589b0: beq 0x58a44 <entry+0x3244>  @ imm = #0x8c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58a44,false)?)?;
    // 0x589b4: ldr r4, [r0, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,2104)?)?;
    // 0x589b8: ldrb r4, [r4, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(4)?,339)?)?;
    // 0x589bc: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x589c0: beq 0x58a44 <entry+0x3244>  @ imm = #0x7c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58a44,false)?)?;
    // 0x589c4: ldrb r4, [r0, #0x139]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R0,313)?)?;
    // 0x589c8: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x589cc: bne 0x589dc <entry+0x31dc>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x589dc,false)?)?;
    // 0x589d0: ldrb r4, [r0, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R0,314)?)?;
    // 0x589d4: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x589d8: beq 0x58b08 <entry+0x3308>  @ imm = #0x128
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58b08,false)?)?;
    // 0x589dc: add r4, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,255)?)?;
    // 0x589e0: ldrb r6, [r0, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R0,348)?)?;
    // 0x589e4: bic r4, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(4)?,R::new(4)?,255)?)?;
    // 0x589e8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x589ec: bne 0x589fc <entry+0x31fc>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x589fc,false)?)?;
    // 0x589f0: ldrb r6, [r0, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R0,347)?)?;
    // 0x589f4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x589f8: beq 0x58c18 <entry+0x3418>  @ imm = #0x218
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58c18,false)?)?;
    // 0x589fc: mov r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,15)?)?;
    // 0x58a00: orr r4, r4, #47
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,47)?)?;
    // 0x58a04: add r6, r6, r7, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsr,2,false)?)?;
    // 0x58a08: bic r9, r6, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(9)?,R::new(6)?,2147483663)?)?;
    // 0x58a0c: ldr r6, [pc, #0x220]        @ 0x3434 <entry+0x3434>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,544)?)?;
    // 0x58a10: umull r7, r3, r4, r6
    c.emit(emit_umull(C::Al,R::new(7)?,R::new(3)?,R::new(4)?,R::new(6)?)?)?;
    // 0x58a14: lsr r4, r3, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(3)?,Shift::Lsr,5)?)?;
    // 0x58a18: add r3, r4, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,63)?)?;
    // 0x58a1c: mov r1, r12
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // 0x58a20: and r3, r3, r10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x58a24: mul r4, r9, r3
    c.emit(arm32::multiply_register(C::Al,R::new(4)?,R::new(9)?,R::new(3)?)?)?;
    // 0x58a28: orr r3, r4, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(4)?,256)?)?;
    // 0x58a2c: ldr r4, [pc, #0x1fc]        @ 0x3430 <entry+0x3430>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,508)?)?;
    // 0x58a30: ldr r12, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,16)?)?;
    // 0x58a34: add r3, r3, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x58a38: and r10, r3, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x58a3c: mov r9, r10
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(10)?))?;
    // 0x58a40: b 0x58b18 <entry+0x3318>  @ imm = #0xd0
    c.emit(arm32::branch(C::Al,c.at as u32,0x58b18,false)?)?;
    // 0x58a44: ldrb r3, [r0, #0x139]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,313)?)?;
    // 0x58a48: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x58a4c: beq 0x58b08 <entry+0x3308>  @ imm = #0xb4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58b08,false)?)?;
    // 0x58a50: add r1, r10, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,268435456)?)?;
    // 0x58a54: ldr lr, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,76)?)?;
    // 0x58a58: ldr r9, [pc, #0x1d0]        @ 0x3430 <entry+0x3430>
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::PC,464)?)?;
    // 0x58a5c: mov r4, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,63)?)?;
    // 0x58a60: add r2, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,1)?)?;
    // 0x58a64: mov r7, r10
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(10)?))?;
    // 0x58a68: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x58a6c: mov r1, r12
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // 0x58a70: ldr r12, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,16)?)?;
    // 0x58a74: ldrb r6, [r0, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R0,348)?)?;
    // 0x58a78: add r3, lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::LR,1)?)?;
    // 0x58a7c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x58a80: beq 0x58b84 <entry+0x3384>  @ imm = #0xfc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58b84,false)?)?;
    // 0x58a84: ldr r4, [pc, #0x1a8]        @ 0x3434 <entry+0x3434>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,424)?)?;
    // 0x58a88: lsr r6, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(3)?,Shift::Lsr,1)?)?;
    // 0x58a8c: lsr r9, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::R2,Shift::Lsr,1)?)?;
    // 0x58a90: mov r2, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63)?)?;
    // 0x58a94: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x58a98: add r6, r6, #23
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,23)?)?;
    // 0x58a9c: umull r3, r5, r6, r4
    c.emit(emit_umull(C::Al,R::new(3)?,R::new(5)?,R::new(6)?,R::new(4)?)?)?;
    // 0x58aa0: add r3, r2, r5, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::new(5)?,Shift::Lsr,4,false)?)?;
    // 0x58aa4: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // 0x58aa8: add r5, r9, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(9)?,3)?)?;
    // 0x58aac: and r3, r3, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x58ab0: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x58ab4: add r5, r2, r5, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::new(5)?,Shift::Lsr,2,false)?)?;
    // 0x58ab8: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x58abc: orr r2, r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3840)?)?;
    // 0x58ac0: bic r5, r5, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,2147483663)?)?;
    // 0x58ac4: mov r9, r2
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    // 0x58ac8: mla r6, r5, r3, r9
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(5)?,R::new(3)?,R::new(9)?)?)?;
    // 0x58acc: ldr r3, [pc, #0x15c]        @ 0x3430 <entry+0x3430>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,348)?)?;
    // 0x58ad0: add r5, lr, #47
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::LR,47)?)?;
    // 0x58ad4: and r10, r6, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x58ad8: umull r6, r2, r5, r4
    c.emit(emit_umull(C::Al,R::new(6)?,R::R2,R::new(5)?,R::new(4)?)?)?;
    // 0x58adc: add r4, r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,3)?)?;
    // 0x58ae0: mov r6, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,63)?)?;
    // 0x58ae4: add r2, r6, r2, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsr,5,false)?)?;
    // 0x58ae8: and r2, r2, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x58aec: mov r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,15)?)?;
    // 0x58af0: add r4, r7, r4, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(7)?,R::new(4)?,Shift::Lsr,2,false)?)?;
    // 0x58af4: bic r4, r4, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(4)?,R::new(4)?,2147483663)?)?;
    // 0x58af8: mla r5, r4, r2, r9
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(4)?,R::R2,R::new(9)?)?)?;
    // 0x58afc: and r9, r5, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x58b00: ldr r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // 0x58b04: b 0x58b18 <entry+0x3318>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x58b18,false)?)?;
    // 0x58b08: mov r1, r12
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // 0x58b0c: ldr r12, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,16)?)?;
    // 0x58b10: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x58b14: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x58b18: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x58b1c: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x58b20: add r2, r9, r9, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(9)?,R::new(9)?,Shift::Lsl,1,false)?)?;
    // 0x58b24: addeq r2, r9, r10
    c.emit(arm32::data_register(C::Eq,Op::Add,R::R2,R::new(9)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x58b28: add r3, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58b2c: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x58b30: str r1, [r0, #0x860]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2144)?)?;
    // 0x58b34: str r12, [r0, #0x864]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,2148)?)?;
    // 0x58b38: str r3, [r0, #0x868]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2152)?)?;
    // 0x58b3c: str r9, [r0, #0x86c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R0,2156)?)?;
    // 0x58b40: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x58b44: str r1, [r0, #0x870]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2160)?)?;
    // 0x58b48: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x58b4c: str r10, [r0, #0x874]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,2164)?)?;
    // 0x58b50: str r5, [r0, #0x878]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,2168)?)?;
    // 0x58b54: str r1, [r0, #0x710]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1808)?)?;
    // 0x58b58: ldr r1, [r0, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2104)?)?;
    // 0x58b5c: ldrb r2, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,339)?)?;
    // 0x58b60: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x58b64: beq 0x58b7c <entry+0x337c>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58b7c,false)?)?;
    // 0x58b68: ldrb r1, [r1, #0x154]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,340)?)?;
    // 0x58b6c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x58b70: addhs r1, r3, r3, lsl #2
    c.emit(arm32::data_register(C::Cs,Op::Add,R::R1,R::new(3)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x58b74: strhs r3, [r0, #0x710]
    c.emit(arm32::transfer_word(C::Cs,false,R::new(3)?,R::R0,1808)?)?;
    // 0x58b78: strhs r1, [r0, #0x868]
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R0,2152)?)?;
    // 0x58b7c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x58b80: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x58b84: add r6, lr, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::LR,31)?)?;
    // 0x58b88: str lr, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,0)?)?;
    // 0x58b8c: mov lr, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,15)?)?;
    // 0x58b90: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x58b94: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x58b98: add r6, r4, r6, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsr,5,false)?)?;
    // 0x58b9c: and r4, r6, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x58ba0: add r6, r8, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(8)?,7)?)?;
    // 0x58ba4: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // 0x58ba8: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x58bac: add r6, lr, r6, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::LR,R::new(6)?,Shift::Lsr,3,false)?)?;
    // 0x58bb0: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x58bb4: ldr r10, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,8)?)?;
    // 0x58bb8: bic r6, r6, #-1073741809
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,3221225487)?)?;
    // 0x58bbc: orr r4, r4, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,3840)?)?;
    // 0x58bc0: str r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,8)?)?;
    // 0x58bc4: mla r9, r6, r10, r4
    c.emit(arm32::multiply_accumulate(C::Al,R::new(9)?,R::new(6)?,R::new(10)?,R::new(4)?)?)?;
    // 0x58bc8: add r6, lr, r3, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::LR,R::new(3)?,Shift::Lsr,1,false)?)?;
    // 0x58bcc: mov r3, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,63)?)?;
    // 0x58bd0: add r6, r3, r6, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(3)?,R::new(6)?,Shift::Lsr,4,false)?)?;
    // 0x58bd4: ldr r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    // 0x58bd8: and r9, r9, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::new(9)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x58bdc: and r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x58be0: mov r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,7)?)?;
    // 0x58be4: add r6, r6, r2, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::R2,Shift::Lsr,1,false)?)?;
    // 0x58be8: add r6, lr, r6, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::LR,R::new(6)?,Shift::Lsr,3,false)?)?;
    // 0x58bec: bic r6, r6, #-1073741809
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,3221225487)?)?;
    // 0x58bf0: mla r10, r6, r3, r4
    c.emit(arm32::multiply_accumulate(C::Al,R::new(10)?,R::new(6)?,R::new(3)?,R::new(4)?)?)?;
    // 0x58bf4: and r10, r10, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::new(10)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x58bf8: beq 0x58b18 <entry+0x3318>  @ imm = #-0xe8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58b18,false)?)?;
    // 0x58bfc: ldrb r6, [r0, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R0,347)?)?;
    // 0x58c00: ldr r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,4)?)?;
    // 0x58c04: ldr lr, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,0)?)?;
    // 0x58c08: ldr r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,8)?)?;
    // 0x58c0c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x58c10: bne 0x58a84 <entry+0x3284>  @ imm = #-0x194
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58a84,false)?)?;
    // 0x58c14: b 0x58b18 <entry+0x3318>  @ imm = #-0x104
    c.emit(arm32::branch(C::Al,c.at as u32,0x58b18,false)?)?;
    // 0x58c18: mov r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,15)?)?;
    // 0x58c1c: lsr r4, r4, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(4)?,Shift::Lsr,5)?)?;
    // 0x58c20: add r6, r6, r7, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsr,3,false)?)?;
    // 0x58c24: bic r9, r6, #-1073741809
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(9)?,R::new(6)?,3221225487)?)?;
    // 0x58c28: b 0x58a18 <entry+0x3218>  @ imm = #-0x218
    c.emit(arm32::branch(C::Al,c.at as u32,0x58a18,false)?)?;
 c.check(0x58c2c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_58C2C:[u32;3]=[0x0fffffc0,0xfffff000,0xaaaaaaab];
 for value in INLINE_DATA_58C2C{c.emit(value)?;}
 // Executable A32 block 0x58c38..0x58f58.
 c.check(0x58c38)?;
    // 0x58c38: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x58c3c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x58c40: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x58c44: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x58c48: mov r0, #848
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,848)?)?;
    // 0x58c4c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x58c50: ldrsb r3, [r4, r0]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::new(3)?,R::new(4)?,R::R0)?)?;
    // 0x58c54: ldr r2, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,280)?)?;
    // 0x58c58: add r0, r4, #296
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,296)?)?;
    // 0x58c5c: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // 0x58c60: bl 0x36048 <entry+0xfffffffffffe0848> @ imm = #-0x22c20
    c.emit(arm32::branch(C::Al,c.at as u32,0x36048,true)?)?;
    // 0x58c64: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x58c68: add r0, r4, #240
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,240)?)?;
    // 0x58c6c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x58c70: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x58c74: str r6, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,24)?)?;
    // 0x58c78: str r6, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,28)?)?;
    // 0x58c7c: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x55a98
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x58c80: str r6, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,8)?)?;
    // 0x58c84: strb r6, [r4, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,12)?)?;
    // 0x58c88: strb r6, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,13)?)?;
    // 0x58c8c: strb r6, [r4, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,14)?)?;
    // 0x58c90: str r6, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,32)?)?;
    // 0x58c94: str r6, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,0)?)?;
    // 0x58c98: bl 0x2f918 <entry+0xfffffffffffda118> @ imm = #-0x29388
    c.emit(arm32::branch(C::Al,c.at as u32,0x2f918,true)?)?;
    // 0x58c9c: str r0, [r4, #0x4a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1192)?)?;
    // 0x58ca0: ldr r0, [pc, #0x2b0]        @ 0x3758 <entry+0x3758>
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::PC,688)?)?;
    // 0x58ca4: str r5, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,644)?)?;
    // 0x58ca8: str r5, [r4, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,3372)?)?;
    // 0x58cac: mov r1, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,230)?)?;
    // 0x58cb0: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x58cb4: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58cb8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x58cbc: str r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // 0x58cc0: mov r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5120)?)?;
    // 0x58cc4: str r6, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,4)?)?;
    // 0x58cc8: str r6, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x58ccc: mov r0, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1020)?)?;
    // 0x58cd0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x58cd4: str r6, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x58cd8: mov r0, #229
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,229)?)?;
    // 0x58cdc: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x58ce0: strb r6, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x58ce4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x58ce8: strb r0, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58cec: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x58cf0: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x58cf4: str r6, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58cf8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x58cfc: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x58d00: str r6, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d04: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x58d08: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x58d0c: str r6, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d10: mov r1, #238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,238)?)?;
    // 0x58d14: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58d18: strb r2, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R2,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d1c: mov r1, #972
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,972)?)?;
    // 0x58d20: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x58d24: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x58d28: strb r6, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d2c: mov r1, #210
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,210)?)?;
    // 0x58d30: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58d34: strb r6, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d38: mov r1, #211
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,211)?)?;
    // 0x58d3c: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58d40: strb r6, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d44: mov r1, #980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,980)?)?;
    // 0x58d48: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x58d4c: strb r6, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d50: mov r1, #206
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,206)?)?;
    // 0x58d54: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58d58: strb r6, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d5c: mov r1, #976
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,976)?)?;
    // 0x58d60: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x58d64: strb r6, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d68: mov r1, #237
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,237)?)?;
    // 0x58d6c: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58d70: strb r6, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d74: mov r1, #205
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,205)?)?;
    // 0x58d78: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58d7c: strb r2, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R2,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d80: mov r1, #207
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,207)?)?;
    // 0x58d84: mov r2, #2000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2000)?)?;
    // 0x58d88: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58d8c: strb r6, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(6)?,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d90: mov r1, #968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,968)?)?;
    // 0x58d94: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x58d98: str r2, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58d9c: mov r1, #225
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,225)?)?;
    // 0x58da0: str r6, [r4, #0xdbc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,3516)?)?;
    // 0x58da4: str r6, [r4, #0xdc0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,3520)?)?;
    // 0x58da8: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x58dac: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x58db0: strb r0, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58db4: mov r1, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    // 0x58db8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x58dbc: str r2, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58dc0: mov r1, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,56)?)?;
    // 0x58dc4: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x58dc8: str r0, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x58dcc: add r0, r4, #292
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,292)?)?;
    // 0x58dd0: strb r6, [r4, #0xe95]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,3733)?)?;
    // 0x58dd4: bl 0x2b8c <entry+0xfffffffffffad38c> @ imm = #-0x56250
    c.emit(arm32::branch(C::Al,c.at as u32,0x2b8c,true)?)?;
    // 0x58dd8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x58ddc: beq 0x58df4 <entry+0x35f4>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58df4,false)?)?;
    // 0x58de0: ldr r2, [pc, #0x174]        @ 0x375c <entry+0x375c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,372)?)?;
    // 0x58de4: mov r1, #54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,54)?)?;
    // 0x58de8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58dec: orr r1, r1, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4608)?)?;
    // 0x58df0: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x57648
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x58df4: add r0, r4, #492
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,492)?)?;
    // 0x58df8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x58dfc: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x58e00: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x58e04: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x58e08: str r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x58e0c: bl 0x35b30 <entry+0xfffffffffffe0330> @ imm = #-0x232e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x35b30,true)?)?;
    // 0x58e10: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x58e14: beq 0x58e2c <entry+0x362c>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58e2c,false)?)?;
    // 0x58e18: ldr r2, [pc, #0x13c]        @ 0x375c <entry+0x375c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,316)?)?;
    // 0x58e1c: mov r1, #61
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,61)?)?;
    // 0x58e20: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58e24: orr r1, r1, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4608)?)?;
    // 0x58e28: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x57680
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x58e2c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x58e30: add r0, r4, #456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,456)?)?;
    // 0x58e34: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x58e38: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x58e3c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x58e40: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x58e44: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x58e48: bl 0x35b30 <entry+0xfffffffffffe0330> @ imm = #-0x23320
    c.emit(arm32::branch(C::Al,c.at as u32,0x35b30,true)?)?;
    // 0x58e4c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x58e50: beq 0x58e68 <entry+0x3668>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58e68,false)?)?;
    // 0x58e54: ldr r2, [pc, #0x100]        @ 0x375c <entry+0x375c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,256)?)?;
    // 0x58e58: mov r1, #67
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,67)?)?;
    // 0x58e5c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58e60: orr r1, r1, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4608)?)?;
    // 0x58e64: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x576bc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x58e68: add r0, r4, #528
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,528)?)?;
    // 0x58e6c: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x58e70: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x58e74: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x58e78: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x58e7c: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x58e80: bl 0x35b30 <entry+0xfffffffffffe0330> @ imm = #-0x23358
    c.emit(arm32::branch(C::Al,c.at as u32,0x35b30,true)?)?;
    // 0x58e84: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x58e88: beq 0x58ea0 <entry+0x36a0>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58ea0,false)?)?;
    // 0x58e8c: ldr r2, [pc, #0xc8]         @ 0x375c <entry+0x375c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,200)?)?;
    // 0x58e90: mov r1, #73
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,73)?)?;
    // 0x58e94: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58e98: orr r1, r1, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4608)?)?;
    // 0x58e9c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x576f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x58ea0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x58ea4: add r0, r4, #564
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,564)?)?;
    // 0x58ea8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x58eac: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x58eb0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x58eb4: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x58eb8: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x58ebc: bl 0x35b30 <entry+0xfffffffffffe0330> @ imm = #-0x23394
    c.emit(arm32::branch(C::Al,c.at as u32,0x35b30,true)?)?;
    // 0x58ec0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x58ec4: beq 0x58edc <entry+0x36dc>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58edc,false)?)?;
    // 0x58ec8: ldr r2, [pc, #0x8c]         @ 0x375c <entry+0x375c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,140)?)?;
    // 0x58ecc: mov r1, #79
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,79)?)?;
    // 0x58ed0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58ed4: orr r1, r1, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4608)?)?;
    // 0x58ed8: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x57730
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x58edc: add r0, r4, #600
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,600)?)?;
    // 0x58ee0: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x58ee4: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x58ee8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x58eec: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x58ef0: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x58ef4: bl 0x35b30 <entry+0xfffffffffffe0330> @ imm = #-0x233cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x35b30,true)?)?;
    // 0x58ef8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x58efc: beq 0x58f14 <entry+0x3714>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58f14,false)?)?;
    // 0x58f00: ldr r2, [pc, #0x54]         @ 0x375c <entry+0x375c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,84)?)?;
    // 0x58f04: mov r1, #86
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,86)?)?;
    // 0x58f08: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x58f0c: orr r1, r1, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4608)?)?;
    // 0x58f10: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x57768
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x58f14: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x58f18: bl 0x35a30 <entry+0xfffffffffffe0230> @ imm = #-0x234f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x35a30,true)?)?;
    // 0x58f1c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x58f20: bl 0x35aac <entry+0xfffffffffffe02ac> @ imm = #-0x2347c
    c.emit(arm32::branch(C::Al,c.at as u32,0x35aac,true)?)?;
    // 0x58f24: mov r0, #1012
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1012)?)?;
    // 0x58f28: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x58f2c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x58f30: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x58f34: mov r0, #2032
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2032)?)?;
    // 0x58f38: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x58f3c: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x58f40: mov r0, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1016)?)?;
    // 0x58f44: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x58f48: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x58f4c: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x58f50: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x58f54: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x58f58)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_58F58:[u32;2]=[0x0010ceac,0x000ee470];
 for value in INLINE_DATA_58F58{c.emit(value)?;}
 // Executable A32 block 0x58f60..0x59000.
 c.check(0x58f60)?;
    // 0x58f60: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x58f64: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x58f68: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x58f6c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x58f70: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x58f74: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // 0x58f78: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x58f7c: mov r7, r2
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // 0x58f80: bl 0x39ac4 <entry+0xfffffffffffe42c4> @ imm = #-0x1f4c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x39ac4,true)?)?;
    // 0x58f84: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x58f88: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x58f8c: bl 0x2bf4 <entry+0xfffffffffffad3f4> @ imm = #-0x563a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x2bf4,true)?)?;
    // 0x58f90: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x58f94: ldr r8, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,8)?)?;
    // 0x58f98: tst r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,8)?)?;
    // 0x58f9c: beq 0x58fec <entry+0x37ec>  @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x58fec,false)?)?;
    // 0x58fa0: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x58fa4: bne 0x58fb4 <entry+0x37b4>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58fb4,false)?)?;
    // 0x58fa8: ldr r1, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,72)?)?;
    // 0x58fac: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x58fb0: blo 0x58fec <entry+0x37ec>  @ imm = #0x34
    c.emit(arm32::branch(C::Cc,c.at as u32,0x58fec,false)?)?;
    // 0x58fb4: add r0, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,4)?)?;
    // 0x58fb8: add r1, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,8)?)?;
    // 0x58fbc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x58fc0: ldr r3, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R1,4)?)?;
    // 0x58fc4: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x58fc8: cmp r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,6)?)?;
    // 0x58fcc: str r3, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x58fd0: bne 0x58fc0 <entry+0x37c0>  @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x58fc0,false)?)?;
    // 0x58fd4: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x58fd8: orr r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // 0x58fdc: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x58fe0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x58fe4: bic r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,8)?)?;
    // 0x58fe8: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x58fec: tst r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1024)?)?;
    // 0x58ff0: beq 0x59028 <entry+0x3828>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x59028,false)?)?;
    // 0x58ff4: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x58ff8: bne 0x59008 <entry+0x3808>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x59008,false)?)?;
    // 0x58ffc: ldr r1, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,76)?)?;
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
