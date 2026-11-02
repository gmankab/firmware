// SPDX-License-Identifier: GPL-3.0-only
//! Extend the A32 controller codec packet serializer with width/height and
//! stride conversions. An inline negative-word mask at 0x47e48 is literal
//! data skipped by branch, not executable (LLVM displays it as unknown).
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};

pub const START:usize=0x47800;
pub const END:usize=0x48000;
const STREAM_PLANE_OFFSET:u32=0xffff_fefc;

/// Public A32 UMULL, resulting in the unsigned 64-bit pair hi:lo. Checked
/// operands, not a copied vendor machine-word array.
fn unsigned_long_multiply(c:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
    if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("invalid UMULL operands".into());}
    Ok(((c as u32)<<28)|0x0080_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
/// Public ARM SMULBB signed low-halfword multiply.
fn signed_multiply_bottom_halves(c:C,rd:R,rm:R,rs:R)->Result<u32,String>{
    if [rd,rm,rs].iter().any(|x|x.number()>12){return Err("invalid SMULBB operands".into());}
    Ok(((c as u32)<<28)|0x0160_0080|(rd.number()<<16)|(rs.number()<<8)|rm.number())
}
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at!=addr{return Err(format!("codec stride code at 0x{:x} expected 0x{addr:x}",self.at));}Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0{return Err("outside Agent 04 firmware reservation".into());}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,c:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,c:C,rd:R,target:usize)->Result<(),String>{
        let d=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"literal overflow")?;
        self.emit(arm32::transfer_word(c,true,rd,R::PC,d)?)
    }
}

/// Compute plane sizes and descriptors through fixed-point multiply and
/// controller format checks. This is an interval of a longer function.
fn encode_additional_stream_lengths(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x47800: bne 0x47814 <entry+0x1814>  @ imm = #0xc
    c.branch(C::Ne,0x47814)?;
    // 0x47804: add r1, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,1)?)?;
    // 0x47808: b 0x47810 <entry+0x1810>  @ imm = #0x0
    c.branch(C::Al,0x47810)?;
    // 0x4780c: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x47810: str r1, [sp, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,452)?)?;
    // 0x47814: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x47818: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4781c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x47820: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x47824: mov r0, #14592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14592)?)?;
    // 0x47828: str r0, [r9, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,80)?)?;
    // 0x4782c: str r1, [r9, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,84)?)?;
    // 0x47830: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x47834: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x47838: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4783c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x47840: add r0, r9, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,88)?)?;
    // 0x47844: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x47848: str r1, [sp, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,452)?)?;
    // 0x4784c: ldr r2, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2744)?)?;
    // 0x47850: ldrb r2, [r2, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,1308)?)?;
    // 0x47854: cmp r2, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,14)?)?;
    // 0x47858: bne 0x478fc <entry+0x18fc>  @ imm = #0x9c
    c.branch(C::Ne,0x478fc)?;
    // 0x4785c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x47860: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x47864: str r2, [sp, #0x4f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1264)?)?;
    // 0x47868: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4786c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x47870: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x47874: mov r1, #3408
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3408)?)?;
    // 0x47878: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x4787c: str r1, [r9, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,88)?)?;
    // 0x47880: str r2, [r9, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,92)?)?;
    // 0x47884: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x47888: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4788c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x47890: add r0, r9, #96
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,96)?)?;
    // 0x47894: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x47898: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4789c: ldr r2, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2744)?)?;
    // 0x478a0: ldr r2, [r2, #0x520]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,1312)?)?;
    // 0x478a4: bic r2, r2, #-268435456
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,4026531840)?)?;
    // 0x478a8: orr r2, r2, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,268435456)?)?;
    // 0x478ac: str r2, [sp, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,456)?)?;
    // 0x478b0: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x478b4: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x478b8: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x478bc: mov r0, #264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,264)?)?;
    // 0x478c0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x478c4: str r0, [r9, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,96)?)?;
    // 0x478c8: str r2, [r9, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,100)?)?;
    // 0x478cc: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x478d0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x478d4: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x478d8: add r0, r9, #104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,104)?)?;
    // 0x478dc: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x478e0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x478e4: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x478e8: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x478ec: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x478f0: str r1, [r9, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,104)?)?;
    // 0x478f4: add r1, r9, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,108)?)?;
    // 0x478f8: b 0x47920 <entry+0x1920>  @ imm = #0x20
    c.branch(C::Al,0x47920)?;
    // 0x478fc: str r1, [sp, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,456)?)?;
    // 0x47900: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x47904: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x47908: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4790c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x47910: mov r1, #264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,264)?)?;
    // 0x47914: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x47918: str r1, [r9, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,88)?)?;
    // 0x4791c: add r1, r9, #92
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,92)?)?;
    // 0x47920: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x47924: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // 0x47928: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x4792c: add r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1)?)?;
    // 0x47930: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x47934: add r2, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,8)?)?;
    // 0x47938: str r2, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,0)?)?;
    // 0x4793c: ldrb r3, [r6, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,1005)?)?;
    // 0x47940: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x47944: bhi 0x47964 <entry+0x1964>  @ imm = #0x18
    c.branch(C::Hi,0x47964)?;
    // 0x47948: ldr r3, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,400)?)?;
    // 0x4794c: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x47950: beq 0x4799c <entry+0x199c>  @ imm = #0x44
    c.branch(C::Eq,0x4799c)?;
    // 0x47954: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x47958: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x4795c: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x47960: bne 0x47a58 <entry+0x1a58>  @ imm = #0xf0
    c.branch(C::Ne,0x47a58)?;
    // 0x47964: ldr r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x47968: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x4796c: ldr r7, [r3, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,28)?)?;
    // 0x47970: add r7, r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,15)?)?;
    // 0x47974: lsr r6, r7, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(7)?,Shift::Lsr,4)?)?;
    // 0x47978: ldr r7, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,40)?)?;
    // 0x4797c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x47980: beq 0x4798c <entry+0x198c>  @ imm = #0x4
    c.branch(C::Eq,0x4798c)?;
    // 0x47984: mov r9, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,6)?)?;
    // 0x47988: b 0x47a58 <entry+0x1a58>  @ imm = #0xc8
    c.branch(C::Al,0x47a58)?;
    // 0x4798c: ldr r3, [r3, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,24)?)?;
    // 0x47990: add r3, r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,15)?)?;
    // 0x47994: asr r9, r3, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(3)?,Shift::Asr,4)?)?;
    // 0x47998: b 0x47a58 <entry+0x1a58>  @ imm = #0xb8
    c.branch(C::Al,0x47a58)?;
    // 0x4799c: ldr r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x479a0: ldr r3, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,644)?)?;
    // 0x479a4: ldr r6, [r7, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,32)?)?;
    // 0x479a8: ldr r7, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,36)?)?;
    // 0x479ac: cmp r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8192)?)?;
    // 0x479b0: add r6, r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,15)?)?;
    // 0x479b4: add r7, r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,15)?)?;
    // 0x479b8: asr r9, r6, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(6)?,Shift::Asr,4)?)?;
    // 0x479bc: lsr r6, r7, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(7)?,Shift::Lsr,4)?)?;
    // 0x479c0: bne 0x47a08 <entry+0x1a08>  @ imm = #0x40
    c.branch(C::Ne,0x47a08)?;
    // 0x479c4: ldr r7, [r10, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2656)?)?;
    // 0x479c8: cmp r7, #192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,192)?)?;
    // 0x479cc: blt 0x479dc <entry+0x19dc>  @ imm = #0x8
    c.branch(C::Lt,0x479dc)?;
    // 0x479d0: ldr r7, [r10, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,928)?)?;
    // 0x479d4: cmp r7, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,960)?)?;
    // 0x479d8: movls r6, #4
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x479dc: ldr r5, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,40)?)?;
    // 0x479e0: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x479e4: beq 0x47a1c <entry+0x1a1c>  @ imm = #0x30
    c.branch(C::Eq,0x47a1c)?;
    // 0x479e8: cmp r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8192)?)?;
    // 0x479ec: bne 0x47a1c <entry+0x1a1c>  @ imm = #0x28
    c.branch(C::Ne,0x47a1c)?;
    // 0x479f0: and r3, r9, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(9)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x479f4: b 0x47a48 <entry+0x1a48>  @ imm = #0x4c
    c.branch(C::Al,0x47a48)?;
    // 0x479f8: ldr r0, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,300)?)?;
    // 0x479fc: mov r5, r12
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // 0x47a00: bic r4, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(4)?,R::R0,16)?)?;
    // 0x47a04: b 0x47244 <entry+0x1244>  @ imm = #-0x7c8
    c.branch(C::Al,0x47244)?;
    // 0x47a08: ldr r5, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,40)?)?;
    // 0x47a0c: and r3, r6, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x47a10: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x47a14: cmp r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // 0x47a18: movlo r6, #2
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x47a1c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x47a20: beq 0x47a54 <entry+0x1a54>  @ imm = #0x2c
    c.branch(C::Eq,0x47a54)?;
    // 0x47a24: ldr r7, [r10, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,940)?)?;
    // 0x47a28: and r3, r9, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(9)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x47a2c: cmp r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // 0x47a30: bne 0x47a48 <entry+0x1a48>  @ imm = #0x10
    c.branch(C::Ne,0x47a48)?;
    // 0x47a34: mov r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,12)?)?;
    // 0x47a38: cmp r3, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,9)?)?;
    // 0x47a3c: movlo r9, #8
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(9)?,R::R0,8)?)?;
    // 0x47a40: b 0x47a4c <entry+0x1a4c>  @ imm = #0x4
    c.branch(C::Al,0x47a4c)?;
    // 0x47a44: andeq lr, lr, r0, asr #8
    c.emit(arm32::data_register(C::Eq,Op::And,R::LR,R::LR,R::R0,Shift::Asr,8,false)?)?;
    // 0x47a48: mov r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,12)?)?;
    // 0x47a4c: cmp r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,7)?)?;
    // 0x47a50: movlo r9, #6
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(9)?,R::R0,6)?)?;
    // 0x47a54: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x47a58: mov r3, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,65536)?)?;
    // 0x47a5c: str r6, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,100)?)?;
    // 0x47a60: and r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,255)?)?;
    // 0x47a64: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x47a68: str r5, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,96)?)?;
    // 0x47a6c: add r1, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // 0x47a70: bic r7, r3, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Bic,R::new(7)?,R::new(3)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x47a74: mov r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,65280)?)?;
    // 0x47a78: and r3, r3, r9, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(9)?,Shift::Lsl,8,false)?)?;
    // 0x47a7c: orr r3, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x47a80: orrne r3, r3, r7
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x47a84: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x47a88: str r3, [sp, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,416)?)?;
    // 0x47a8c: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x47a90: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x47a94: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x47a98: mov r2, #212
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,212)?)?;
    // 0x47a9c: add r7, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,16)?)?;
    // 0x47aa0: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x47aa4: str r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    // 0x47aa8: str r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x47aac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x47ab0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x47ab4: str r7, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R12,0)?)?;
    // 0x47ab8: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x47abc: str r0, [sp, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,424)?)?;
    // 0x47ac0: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,64)?)?;
    // 0x47ac4: ldr r3, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,76)?)?;
    // 0x47ac8: ldr r5, [r10, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,928)?)?;
    // 0x47acc: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // 0x47ad0: add r2, r3, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x47ad4: movne r1, #5
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x47ad8: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // 0x47adc: lsr r1, r2, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R2,Shift::Lsr,R::R1,false)?)?;
    // 0x47ae0: mul r2, r1, r3
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::new(3)?)?)?;
    // 0x47ae4: ldr r1, [r10, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2652)?)?;
    // 0x47ae8: str r8, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,80)?)?;
    // 0x47aec: str r4, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,92)?)?;
    // 0x47af0: mov r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,255)?)?;
    // 0x47af4: orr r8, r8, #16128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,16128)?)?;
    // 0x47af8: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x47afc: movne r0, #-2147483648
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,2147483648)?)?;
    // 0x47b00: strne r0, [sp, #0x1a8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::SP,424)?)?;
    // 0x47b04: ldr r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x47b08: ldrb r2, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,339)?)?;
    // 0x47b0c: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x47b10: and r2, r5, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x47b14: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x47b18: streq r0, [sp, #0x1a8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::SP,424)?)?;
    // 0x47b1c: ldr r3, [pc, #0xefc]        @ 0x2a20 <entry+0x2a20>
    c.literal(C::Al,R::new(3)?,0x48a20)?;
    // 0x47b20: and r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x47b24: orr r4, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x47b28: ldrb r0, [r1, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,336)?)?;
    // 0x47b2c: add r1, r10, #696
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,696)?)?;
    // 0x47b30: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x47b34: addeq r1, r10, #692
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::new(10)?,692)?)?;
    // 0x47b38: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x47b3c: ldr r1, [r10, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,716)?)?;
    // 0x47b40: mla r2, r0, r5, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::new(5)?,R::R1)?)?;
    // 0x47b44: sub r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,1)?)?;
    // 0x47b48: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c0fc
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47b4c: ldr r1, [pc, #0xed0]        @ 0x2a24 <entry+0x2a24>
    c.literal(C::Al,R::R1,0x48a24)?;
    // 0x47b50: ldr r2, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,68)?)?;
    // 0x47b54: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x47b58: add r2, r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,16384)?)?;
    // 0x47b5c: and r1, r4, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47b60: and r0, r2, r0, lsl #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,14,false)?)?;
    // 0x47b64: orr r6, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47b68: str r6, [sp, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,424)?)?;
    // 0x47b6c: bge 0x47b94 <entry+0x1b94>  @ imm = #0x20
    c.branch(C::Ge,0x47b94)?;
    // 0x47b70: ldr r2, [pc, #0xeb0]        @ 0x2a28 <entry+0x2a28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x47b74: mov r1, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,30)?)?;
    // 0x47b78: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x47b7c: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x47b80: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x47b84: orr r1, r1, #9472
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9472)?)?;
    // 0x47b88: bl 0x17b0 <entry+0xfffffffffffbb7b0> @ imm = #-0x463e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x47b8c: ldr r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,104)?)?;
    // 0x47b90: ldr r7, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,0)?)?;
    // 0x47b94: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x47b98: str r9, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,84)?)?;
    // 0x47b9c: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x47ba0: ldr r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x47ba4: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x47ba8: and r2, r9, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x47bac: ldreq r0, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(7)?,0)?)?;
    // 0x47bb0: mov r3, r2
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // 0x47bb4: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x47bb8: streq r0, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(7)?,0)?)?;
    // 0x47bbc: mov r0, #220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,220)?)?;
    // 0x47bc0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x47bc4: stm r7, {r0, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(7)?,R::R0.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x47bc8: str r7, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,72)?)?;
    // 0x47bcc: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x47bd0: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // 0x47bd4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x47bd8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x47bdc: add r0, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,8)?)?;
    // 0x47be0: add r7, sp, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::SP,256)?)?;
    // 0x47be4: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x47be8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x47bec: str r3, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,76)?)?;
    // 0x47bf0: ldr r9, [sp, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,424)?)?;
    // 0x47bf4: ldr r3, [pc, #0xe30]        @ 0x2a2c <entry+0x2a2c>
    c.literal(C::Al,R::new(3)?,0x48a2c)?;
    // 0x47bf8: and r1, r9, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x47bfc: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x47c00: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x47c04: addlt r0, r0, r2, lsl #5
    c.emit(arm32::data_register(C::Lt,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,5,false)?)?;
    // 0x47c08: mov r2, #3968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3968)?)?;
    // 0x47c0c: add r2, r2, r0, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R0,Shift::Lsl,7,false)?)?;
    // 0x47c10: add r0, r0, #95
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,95)?)?;
    // 0x47c14: umull r6, r5, r0, r3
    c.emit(unsigned_long_multiply(C::Al,R::new(6)?,R::new(5)?,R::R0,R::new(3)?)?)?;
    // 0x47c18: lsr r0, r5, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsr,6)?)?;
    // 0x47c1c: ldrb r5, [r10, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,988)?)?;
    // 0x47c20: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x47c24: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x47c28: lslne r2, r0, #12
    c.emit(arm32::mov_shifted_register(C::Ne,R::R2,R::R0,Shift::Lsl,12)?)?;
    // 0x47c2c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x47c30: and r8, r2, #1044480
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(8)?,R::R2,1044480)?)?;
    // 0x47c34: str r8, [sp, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,428)?)?;
    // 0x47c38: str r0, [sp, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,432)?)?;
    // 0x47c3c: ldrh r0, [r7, #80]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(7)?,80)?)?;
    // 0x47c40: bl 0x81850 <entry+0x3b850> @ imm = #0x39c08
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x47c44: mov r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,32)?)?;
    // 0x47c48: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x47c4c: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x47c50: movne r6, #48
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,48)?)?;
    // 0x47c54: add r0, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x47c58: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x47c5c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x47c60: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3bfe4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47c64: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x47c68: str r5, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,56)?)?;
    // 0x47c6c: beq 0x47c88 <entry+0x1c88>  @ imm = #0x14
    c.branch(C::Eq,0x47c88)?;
    // 0x47c70: mov r1, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,254)?)?;
    // 0x47c74: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x47c78: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x47c7c: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47c80: smulbb r0, r0, r6
    c.emit(signed_multiply_bottom_halves(C::Al,R::R0,R::R0,R::new(6)?)?)?;
    // 0x47c84: lsr r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,5)?)?;
    // 0x47c88: ldr r2, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,100)?)?;
    // 0x47c8c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x47c90: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x47c94: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x47c98: and r2, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47c9c: beq 0x47cb4 <entry+0x1cb4>  @ imm = #0x10
    c.branch(C::Eq,0x47cb4)?;
    // 0x47ca0: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x47ca4: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x47ca8: and r1, r1, r9, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(9)?,Shift::Lsr,5,false)?)?;
    // 0x47cac: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47cb0: bls 0x47cc0 <entry+0x1cc0>  @ imm = #0x8
    c.branch(C::Ls,0x47cc0)?;
    // 0x47cb4: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x47cb8: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x47cbc: and r0, r0, r9, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsr,5,false)?)?;
    // 0x47cc0: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x47cc4: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x47cc8: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x47ccc: orr r0, r0, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x47cd0: lsl r4, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R2,Shift::Lsl,1)?)?;
    // 0x47cd4: str r0, [sp, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,432)?)?;
    // 0x47cd8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x47cdc: ldrb r3, [r10, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,2462)?)?;
    // 0x47ce0: ldr r1, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,300)?)?;
    // 0x47ce4: cmp r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // 0x47ce8: bic r1, r1, #49152
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,49152)?)?;
    // 0x47cec: str r3, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,100)?)?;
    // 0x47cf0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x47cf4: cmp r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // 0x47cf8: orr r0, r1, r0, lsl #14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,14,false)?)?;
    // 0x47cfc: movne r7, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x47d00: str r0, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,300)?)?;
    // 0x47d04: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x47d08: ldr r5, [r0, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,668)?)?;
    // 0x47d0c: ldr r10, [r0, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,664)?)?;
    // 0x47d10: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x47d14: str r2, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,52)?)?;
    // 0x47d18: mul r1, r5, r7
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(5)?,R::new(7)?)?)?;
    // 0x47d1c: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3bf28
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47d20: ldr r2, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,76)?)?;
    // 0x47d24: str r8, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,60)?)?;
    // 0x47d28: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x47d2c: lsr r8, r8, #12
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::new(8)?,Shift::Lsr,12)?)?;
    // 0x47d30: cmp r10, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x47d34: mul r1, r8, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(8)?,R::R0)?)?;
    // 0x47d38: lsr r0, r9, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(9)?,Shift::Lsr,31)?)?;
    // 0x47d3c: ldr r9, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,84)?)?;
    // 0x47d40: add r0, r1, r2, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R0,R::R1,R::R2,Shift::Lsl,R::R0,false)?)?;
    // 0x47d44: sub r1, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(5)?,1)?)?;
    // 0x47d48: mul r2, r1, r7
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::new(7)?)?)?;
    // 0x47d4c: add r1, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x47d50: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x47d54: add r0, r0, r1, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsr,1,false)?)?;
    // 0x47d58: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x47d5c: add r7, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,6)?)?;
    // 0x47d60: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x47d64: blo 0x47d88 <entry+0x1d88>  @ imm = #0x1c
    c.branch(C::Cc,0x47d88)?;
    // 0x47d68: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x47d6c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x47d70: bne 0x47dd8 <entry+0x1dd8>  @ imm = #0x60
    c.branch(C::Ne,0x47dd8)?;
    // 0x47d74: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x47d78: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x47d7c: beq 0x47dd8 <entry+0x1dd8>  @ imm = #0x54
    c.branch(C::Eq,0x47dd8)?;
    // 0x47d80: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // 0x47d84: blo 0x47dd8 <entry+0x1dd8>  @ imm = #0x4c
    c.branch(C::Cc,0x47dd8)?;
    // 0x47d88: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x47d8c: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x47d90: movne r6, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x47d94: mul r0, r6, r5
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::new(5)?)?)?;
    // 0x47d98: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47d9c: add r0, r1, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x47da0: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3bea4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47da4: mla r1, r8, r0, r9
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(8)?,R::R0,R::new(9)?)?)?;
    // 0x47da8: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x47dac: add r8, sp, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,56)?)?;
    // 0x47db0: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x47db4: ldr r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x47db8: ldr r12, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,96)?)?;
    // 0x47dbc: ldr r4, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,72)?)?;
    // 0x47dc0: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x47dc4: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x47dc8: ldm r8, {r1, r5, r6, r8}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(8)?,R::R1.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x47dcc: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x47dd0: movlo r7, r0
    c.emit(arm32::mov_register(C::Cc,R::new(7)?,R::R0))?;
    // 0x47dd4: b 0x47df0 <entry+0x1df0>  @ imm = #0x14
    c.branch(C::Al,0x47df0)?;
    // 0x47dd8: add r8, sp, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,56)?)?;
    // 0x47ddc: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x47de0: ldr r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x47de4: ldr r12, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,96)?)?;
    // 0x47de8: ldr r4, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,72)?)?;
    // 0x47dec: ldm r8, {r1, r5, r6, r8}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(8)?,R::R1.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x47df0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x47df4: beq 0x47e4c <entry+0x1e4c>  @ imm = #0x50
    c.branch(C::Eq,0x47e4c)?;
    // 0x47df8: ldr r1, [pc, #0xc2c]        @ 0x2a2c <entry+0x2a2c>
    c.literal(C::Al,R::R1,0x48a2c)?;
    // 0x47dfc: mov r0, #95
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,95)?)?;
    // 0x47e00: add r0, r0, r7, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,5,false)?)?;
    // 0x47e04: umull r2, r3, r0, r1
    c.emit(unsigned_long_multiply(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x47e08: lsr r0, r3, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,6)?)?;
    // 0x47e0c: add r3, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x47e10: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x47e14: orr r0, r0, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3840)?)?;
    // 0x47e18: mov r2, r3
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // 0x47e1c: and r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x47e20: str r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,64)?)?;
    // 0x47e24: add r2, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,3)?)?;
    // 0x47e28: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x47e2c: add r2, r3, r2, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsr,2,false)?)?;
    // 0x47e30: orr r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x47e34: umull r3, r7, r2, r1
    c.emit(unsigned_long_multiply(C::Al,R::new(3)?,R::new(7)?,R::R2,R::R1)?)?;
    // 0x47e38: str r0, [sp, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,428)?)?;
    // 0x47e3c: lsr r1, r7, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(7)?,Shift::Lsr,1)?)?;
    // 0x47e40: add r7, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x47e44: b 0x47e70 <entry+0x1e70>  @ imm = #0x24
    c.branch(C::Al,0x47e70)?;
    c.check(0x47e48)?;
    c.emit(STREAM_PLANE_OFFSET)?;
    c.check(0x47e4c)?;
    // 0x47e4c: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x47e50: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x47e54: orr r0, r0, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3840)?)?;
    // 0x47e58: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // 0x47e5c: add r1, r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,3)?)?;
    // 0x47e60: and r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x47e64: lsr r7, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R1,Shift::Lsr,2)?)?;
    // 0x47e68: orr r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x47e6c: str r0, [sp, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,428)?)?;
    // 0x47e70: ldr r2, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,100)?)?;
    // 0x47e74: mov r1, #267386880
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,267386880)?)?;
    // 0x47e78: orr r1, r1, #805306368
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,805306368)?)?;
    // 0x47e7c: and r1, r1, r7, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,20,false)?)?;
    // 0x47e80: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47e84: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x47e88: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // 0x47e8c: str r0, [sp, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,428)?)?;
    // 0x47e90: movne r1, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x47e94: str r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // 0x47e98: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x47e9c: movne r1, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x47ea0: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x47ea4: str r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,68)?)?;
    // 0x47ea8: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x47eac: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x47eb0: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x47eb4: mov r1, #2272
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2272)?)?;
    // 0x47eb8: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x47ebc: str r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,8)?)?;
    // 0x47ec0: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x47ec4: add r0, r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,2)?)?;
    // 0x47ec8: add r1, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,16)?)?;
    // 0x47ecc: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x47ed0: str r1, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x47ed4: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x47ed8: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x47edc: ldr r0, [sp, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,432)?)?;
    // 0x47ee0: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x47ee4: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x47ee8: mov r1, #228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,228)?)?;
    // 0x47eec: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x47ef0: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x47ef4: str r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,16)?)?;
    // 0x47ef8: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // 0x47efc: add r0, r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,3)?)?;
    // 0x47f00: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x47f04: add r0, r4, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,24)?)?;
    // 0x47f08: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x47f0c: beq 0x47f74 <entry+0x1f74>  @ imm = #0x60
    c.branch(C::Eq,0x47f74)?;
    // 0x47f10: ldr r1, [r10, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,668)?)?;
    // 0x47f14: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x47f18: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x47f1c: bhi 0x47f74 <entry+0x1f74>  @ imm = #0x50
    c.branch(C::Hi,0x47f74)?;
    // 0x47f20: ldr r3, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,100)?)?;
    // 0x47f24: ldr r2, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,52)?)?;
    // 0x47f28: mov r0, #224
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,224)?)?;
    // 0x47f2c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x47f30: and r0, r0, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,5,false)?)?;
    // 0x47f34: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x47f38: cmp r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // 0x47f3c: movne r2, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x47f40: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // 0x47f44: mov r1, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,240)?)?;
    // 0x47f48: and r1, r1, r3, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,4,false)?)?;
    // 0x47f4c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47f50: blo 0x47f60 <entry+0x1f60>  @ imm = #0x8
    c.branch(C::Cc,0x47f60)?;
    // 0x47f54: bl 0x81850 <entry+0x3b850> @ imm = #0x398f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x47f58: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x47f5c: moveq r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x47f60: ldr r2, [pc, #0xac0]        @ 0x2a28 <entry+0x2a28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x47f64: mov r1, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,98)?)?;
    // 0x47f68: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x47f6c: orr r1, r1, #9472
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9472)?)?;
    // 0x47f70: bl 0x17b0 <entry+0xfffffffffffbb7b0> @ imm = #-0x467c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x47f74: add r8, sp, #204
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,204)?)?;
    // 0x47f78: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x47f7c: mov r2, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,96)?)?;
    // 0x47f80: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x47f84: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x47f88: bl 0x31ec <entry+0xfffffffffffbd1ec> @ imm = #-0x44da4
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x47f8c: add r0, sp, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,108)?)?;
    // 0x47f90: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x47f94: mov r2, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,96)?)?;
    // 0x47f98: bl 0x31ec <entry+0xfffffffffffbd1ec> @ imm = #-0x44db4
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x47f9c: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x47fa0: ldr r0, [r0, #0x518]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1304)?)?;
    // 0x47fa4: str r4, [sp, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,548)?)?;
    // 0x47fa8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x47fac: beq 0x47fdc <entry+0x1fdc>  @ imm = #0x28
    c.branch(C::Eq,0x47fdc)?;
    // 0x47fb0: and r4, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::R0,3)?)?;
    // 0x47fb4: cmp r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,3)?)?;
    // 0x47fb8: str r4, [sp, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,548)?)?;
    // 0x47fbc: bne 0x47fd4 <entry+0x1fd4>  @ imm = #0x10
    c.branch(C::Ne,0x47fd4)?;
    // 0x47fc0: ldr r0, [r10, #0x694]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1684)?)?;
    // 0x47fc4: mov r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3)?)?;
    // 0x47fc8: cmp r0, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,384)?)?;
    // 0x47fcc: movhi r4, #0
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x47fd0: strhi r4, [sp, #0x224]
    c.emit(arm32::transfer_word(C::Hi,false,R::new(4)?,R::SP,548)?)?;
    // 0x47fd4: cmp r4, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,2)?)?;
    // 0x47fd8: moveq r4, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x47fdc: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x47fe0: str r4, [sp, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,548)?)?;
    // 0x47fe4: ldr r0, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x47fe8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x47fec: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x47ff0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x47ff4: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x47ff8: mov r1, #2464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2464)?)?;
    // 0x47ffc: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();encode_additional_stream_lengths(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn plane_offset_is_literal_data(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        let off=0x47e48-START;
        assert_eq!(u32::from_le_bytes(bytes[off..off+4].try_into().unwrap()),STREAM_PLANE_OFFSET);
        assert_eq!(unsigned_long_multiply(C::Al,R::new(6).unwrap(),R::new(5).unwrap(),R::R0,R::new(3).unwrap()).unwrap(),0xe0856390);
        assert_eq!(signed_multiply_bottom_halves(C::Al,R::R0,R::R0,R::new(6).unwrap()).unwrap(),0xe1600680);
    }
}
