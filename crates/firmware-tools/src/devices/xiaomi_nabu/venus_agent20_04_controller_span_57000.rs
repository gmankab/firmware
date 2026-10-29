// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x57000,0x58000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x57000;
pub const END:usize=0x58000;
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
fn emit_umull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 UMULL operands".into())}
 Ok(((cond as u32)<<28)|0x0080_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x57000..0x57284.
 c.check(0x57000)?;
    // 0x57000: mul r7, r3, r2
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::new(3)?,R::R2)?)?;
    // 0x57004: lsr r2, r7, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(7)?,Shift::Lsr,2)?)?;
    // 0x57008: cmp r2, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x5700c: addhi r0, r1, #27136
    c.emit(arm32::data_immediate(C::Hi,Op::Add,R::R0,R::R1,27136)?)?;
    // 0x57010: addhi r0, r0, #1572864
    c.emit(arm32::data_immediate(C::Hi,Op::Add,R::R0,R::R0,1572864)?)?;
    // 0x57014: strhi r0, [r4, #0xaa4]
    c.emit(arm32::transfer_word(C::Hi,false,R::R0,R::new(4)?,2724)?)?;
    // 0x57018: str r0, [r4, #0xaa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2728)?)?;
    // 0x5701c: ldr r0, [r4, #0x30c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,780)?)?;
    // 0x57020: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x57024: blo 0x57050 <entry+0x1850>  @ imm = #0x24
    c.emit(arm32::branch(C::Cc,c.at as u32,0x57050,false)?)?;
    // 0x57028: sub r1, r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,21)?)?;
    // 0x5702c: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x57030: bhs 0x570a8 <entry+0x18a8>  @ imm = #0x70
    c.emit(arm32::branch(C::Cs,c.at as u32,0x570a8,false)?)?;
    // 0x57034: ldr r1, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1280)?)?;
    // 0x57038: ldr r2, [r4, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1284)?)?;
    // 0x5703c: ldr r3, [r4, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1272)?)?;
    // 0x57040: mul r0, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::R2)?)?;
    // 0x57044: ldr r2, [r4, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1268)?)?;
    // 0x57048: mul r1, r2, r3
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R2,R::new(3)?)?)?;
    // 0x5704c: b 0x570cc <entry+0x18cc>  @ imm = #0x78
    c.emit(arm32::branch(C::Al,c.at as u32,0x570cc,false)?)?;
    // 0x57050: ldr r0, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1280)?)?;
    // 0x57054: ldr r1, [r4, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1284)?)?;
    // 0x57058: mul r3, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R0)?)?;
    // 0x5705c: ldr r0, [r4, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1268)?)?;
    // 0x57060: ldr r1, [r4, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1272)?)?;
    // 0x57064: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x57068: ldrb r0, [r4, #0x3db]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,987)?)?;
    // 0x5706c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57070: beq 0x57108 <entry+0x1908>  @ imm = #0x90
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57108,false)?)?;
    // 0x57074: ldr r0, [r4, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,680)?)?;
    // 0x57078: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x5707c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57080: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // 0x57084: bne 0x57110 <entry+0x1910>  @ imm = #0x84
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57110,false)?)?;
    // 0x57088: ldr r7, [pc, #0xff4]        @ 0x2884 <entry+0x2884>
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::PC,4084)?)?;
    // 0x5708c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x57090: orr r1, r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3840)?)?;
    // 0x57094: add r0, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x57098: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x5709c: and r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x570a0: and r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x570a4: b 0x57110 <entry+0x1910>  @ imm = #0x64
    c.emit(arm32::branch(C::Al,c.at as u32,0x57110,false)?)?;
    // 0x570a8: cmp r0, #25
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,25)?)?;
    // 0x570ac: bne 0x57174 <entry+0x1974>  @ imm = #0xc0
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57174,false)?)?;
    // 0x570b0: ldr r0, [r4, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1268)?)?;
    // 0x570b4: ldr r2, [r4, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1272)?)?;
    // 0x570b8: mul r1, r2, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R2,R::R0)?)?;
    // 0x570bc: ldr r0, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1280)?)?;
    // 0x570c0: ldr r2, [r4, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1284)?)?;
    // 0x570c4: mul r3, r0, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    // 0x570c8: lsl r0, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsl,1)?)?;
    // 0x570cc: ldrb r2, [r4, #0x3ba]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,954)?)?;
    // 0x570d0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x570d4: beq 0x5715c <entry+0x195c>  @ imm = #0x80
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5715c,false)?)?;
    // 0x570d8: ldr r7, [r4, #0x514]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1300)?)?;
    // 0x570dc: ldr r6, [r4, #0x518]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1304)?)?;
    // 0x570e0: mov r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,255)?)?;
    // 0x570e4: ldr r2, [r4, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1292)?)?;
    // 0x570e8: ldr r3, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1296)?)?;
    // 0x570ec: orr r12, r12, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,3840)?)?;
    // 0x570f0: mla r5, r6, r7, r12
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(6)?,R::new(7)?,R::R12)?)?;
    // 0x570f4: ldr r7, [pc, #0xf88]        @ 0x2884 <entry+0x2884>
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::PC,3976)?)?;
    // 0x570f8: mla r6, r3, r2, r12
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(3)?,R::R2,R::R12)?)?;
    // 0x570fc: and r5, r5, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x57100: and r2, r6, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x57104: b 0x571bc <entry+0x19bc>  @ imm = #0xb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x571bc,false)?)?;
    // 0x57108: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x5710c: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // 0x57110: ldrb r7, [r4, #0x3ba]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,954)?)?;
    // 0x57114: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x57118: beq 0x5715c <entry+0x195c>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5715c,false)?)?;
    // 0x5711c: mov r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,255)?)?;
    // 0x57120: orr r7, r7, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,3840)?)?;
    // 0x57124: add r0, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x57128: ldr r3, [pc, #0xfdc]        @ 0x290c <entry+0x290c>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,4060)?)?;
    // 0x5712c: add r1, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x57130: ldr r12, [r4, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,1292)?)?;
    // 0x57134: ldr lr, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(4)?,1296)?)?;
    // 0x57138: ldr r5, [r4, #0x514]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,1300)?)?;
    // 0x5713c: ldr r2, [r4, #0x518]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1304)?)?;
    // 0x57140: mla r6, r2, r5, r7
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::R2,R::new(5)?,R::new(7)?)?)?;
    // 0x57144: mla r2, lr, r12, r7
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::LR,R::R12,R::new(7)?)?)?;
    // 0x57148: and r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x5714c: and r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x57150: and r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x57154: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x57158: b 0x571bc <entry+0x19bc>  @ imm = #0x5c
    c.emit(arm32::branch(C::Al,c.at as u32,0x571bc,false)?)?;
    // 0x5715c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x57160: b 0x571b8 <entry+0x19b8>  @ imm = #0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x571b8,false)?)?;
    // 0x57164: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x57168: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5716c: str r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,52)?)?;
    // 0x57170: b 0x56f14 <entry+0x1714>  @ imm = #-0x264
    c.emit(arm32::branch(C::Al,c.at as u32,0x56f14,false)?)?;
    // 0x57174: ldr r1, [pc, #0xf94]        @ 0x2910 <entry+0x2910>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3988)?)?;
    // 0x57178: ldr r2, [pc, #0xf94]        @ 0x2914 <entry+0x2914>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3988)?)?;
    // 0x5717c: mov r3, #231
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,231)?)?;
    // 0x57180: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x57184: orr r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x57188: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x284fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x5718c: ldr r1, [pc, #0xf70]        @ 0x2904 <entry+0x2904>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3952)?)?;
    // 0x57190: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x57194: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x28508
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x57198: ldr r2, [pc, #0xf68]        @ 0x2908 <entry+0x2908>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3944)?)?;
    // 0x5719c: mov r1, #744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,744)?)?;
    // 0x571a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x571a4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x571a8: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x571ac: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x55a04
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x571b0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x571b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x571b8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x571bc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x571c0: str r3, [r4, #0xad4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,2772)?)?;
    // 0x571c4: add r3, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x571c8: str r2, [r4, #0x520]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1312)?)?;
    // 0x571cc: str r3, [r4, #0x524]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1316)?)?;
    // 0x571d0: add r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x571d4: str r3, [r4, #0x528]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1320)?)?;
    // 0x571d8: str r2, [r4, #0xacc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2764)?)?;
    // 0x571dc: str r5, [r4, #0xad0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,2768)?)?;
    // 0x571e0: str r3, [r4, #0xad8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,2776)?)?;
    // 0x571e4: str r1, [r4, #0x52c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1324)?)?;
    // 0x571e8: str r0, [r4, #0x530]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1328)?)?;
    // 0x571ec: add r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x571f0: str r0, [r4, #0x628]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1576)?)?;
    // 0x571f4: ldrb r1, [r4, #0x413]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1043)?)?;
    // 0x571f8: ldr r0, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1528)?)?;
    // 0x571fc: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x57200: bne 0x57218 <entry+0x1a18>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57218,false)?)?;
    // 0x57204: ldr r2, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1572)?)?;
    // 0x57208: cmp r0, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,1,false)?)?;
    // 0x5720c: bhs 0x57228 <entry+0x1a28>  @ imm = #0x14
    c.emit(arm32::branch(C::Cs,c.at as u32,0x57228,false)?)?;
    // 0x57210: lsl r0, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsl,1)?)?;
    // 0x57214: b 0x57224 <entry+0x1a24>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x57224,false)?)?;
    // 0x57218: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x5721c: bhi 0x57228 <entry+0x1a28>  @ imm = #0x4
    c.emit(arm32::branch(C::Hi,c.at as u32,0x57228,false)?)?;
    // 0x57220: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x57224: str r0, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1528)?)?;
    // 0x57228: ldr r0, [r4, #0xab0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2736)?)?;
    // 0x5722c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57230: beq 0x57260 <entry+0x1a60>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57260,false)?)?;
    // 0x57234: ldr r9, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,2652)?)?;
    // 0x57238: ldr r5, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,2656)?)?;
    // 0x5723c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x57240: mul r0, r9, r5
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(9)?,R::new(5)?)?)?;
    // 0x57244: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x57248: str r0, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1592)?)?;
    // 0x5724c: bne 0x572b4 <entry+0x1ab4>  @ imm = #0x60
    c.emit(arm32::branch(C::Ne,c.at as u32,0x572b4,false)?)?;
    // 0x57250: ldr r1, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1572)?)?;
    // 0x57254: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2c9f0
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x57258: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // 0x5725c: b 0x572b0 <entry+0x1ab0>  @ imm = #0x4c
    c.emit(arm32::branch(C::Al,c.at as u32,0x572b0,false)?)?;
    // 0x57260: ldr r9, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,2652)?)?;
    // 0x57264: ldr r5, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,2656)?)?;
    // 0x57268: ldrb r2, [r4, #0x34a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,842)?)?;
    // 0x5726c: mul r0, r9, r5
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(9)?,R::new(5)?)?)?;
    // 0x57270: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x57274: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x57278: beq 0x57288 <entry+0x1a88>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57288,false)?)?;
    // 0x5727c: asr r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,1)?)?;
    // 0x57280: b 0x57298 <entry+0x1a98>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x57298,false)?)?;
 c.check(0x57284)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_57284:[u32;1]=[0x000ee57e];
 for value in INLINE_DATA_57284{c.emit(value)?;}
 // Executable A32 block 0x57288..0x57304.
 c.check(0x57288)?;
    // 0x57288: cmp r0, #786432
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,786432)?)?;
    // 0x5728c: str r0, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1592)?)?;
    // 0x57290: bls 0x5729c <entry+0x1a9c>  @ imm = #0x4
    c.emit(arm32::branch(C::Ls,c.at as u32,0x5729c,false)?)?;
    // 0x57294: lsr r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,2)?)?;
    // 0x57298: str r0, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1592)?)?;
    // 0x5729c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x572a0: bne 0x572b4 <entry+0x1ab4>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x572b4,false)?)?;
    // 0x572a4: ldr r1, [r4, #0x624]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1572)?)?;
    // 0x572a8: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2c99c
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x572ac: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x572b0: str r0, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1592)?)?;
    // 0x572b4: ldr r1, [r4, #0x4cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1228)?)?;
    // 0x572b8: ldr r8, [r4, #0x4d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,1232)?)?;
    // 0x572bc: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x572c0: and r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x572c4: str r0, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1592)?)?;
    // 0x572c8: ldrb r2, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,990)?)?;
    // 0x572cc: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x572d0: bne 0x57308 <entry+0x1b08>  @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57308,false)?)?;
    // 0x572d4: ldrb r2, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,991)?)?;
    // 0x572d8: ldrb r3, [r4, #0x3e0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,992)?)?;
    // 0x572dc: add r0, r0, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // 0x572e0: add r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x572e4: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x572e8: str r2, [r4, #0x5fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1532)?)?;
    // 0x572ec: ldr r2, [pc, #0xe24]        @ 0x2918 <entry+0x2918>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3620)?)?;
    // 0x572f0: umull r3, r7, r0, r2
    c.emit(emit_umull(C::Al,R::new(3)?,R::new(7)?,R::R0,R::R2)?)?;
    // 0x572f4: lsr r0, r7, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsr,3)?)?;
    // 0x572f8: str r0, [r4, #0x640]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1600)?)?;
    // 0x572fc: add r0, r1, r7, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(7)?,Shift::Lsr,3,false)?)?;
    // 0x57300: b 0x57338 <entry+0x1b38>  @ imm = #0x30
    c.emit(arm32::branch(C::Al,c.at as u32,0x57338,false)?)?;
 c.check(0x57304)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_57304:[u32;1]=[0x000ee5c3];
 for value in INLINE_DATA_57304{c.emit(value)?;}
 // Executable A32 block 0x57308..0x57480.
 c.check(0x57308)?;
    // 0x57308: ldr r0, [r4, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,940)?)?;
    // 0x5730c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57310: bne 0x57320 <entry+0x1b20>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57320,false)?)?;
    // 0x57314: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x57318: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x5731c: blo 0x57f04 <entry+0x2704>  @ imm = #0xbe0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x57f04,false)?)?;
    // 0x57320: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x57324: str r0, [r4, #0x5fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1532)?)?;
    // 0x57328: mul r0, r9, r5
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(9)?,R::new(5)?)?)?;
    // 0x5732c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x57330: str r0, [r4, #0x640]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1600)?)?;
    // 0x57334: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x57338: and r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x5733c: str r0, [r4, #0x640]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1600)?)?;
    // 0x57340: ldr r1, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2744)?)?;
    // 0x57344: ldrb r10, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(10)?,R::R1,339)?)?;
    // 0x57348: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x5734c: movne r0, #256
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x57350: strne r0, [r4, #0x640]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1600)?)?;
    // 0x57354: ldr r1, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,720)?)?;
    // 0x57358: ldr r6, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,716)?)?;
    // 0x5735c: ldr r5, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,664)?)?;
    // 0x57360: add r2, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,32)?)?;
    // 0x57364: add r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,32)?)?;
    // 0x57368: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // 0x5736c: ldr r1, [pc, #0xda8]        @ 0x291c <entry+0x291c>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3496)?)?;
    // 0x57370: and r1, r1, r3, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsr,4,false)?)?;
    // 0x57374: add r7, r1, #384
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R1,384)?)?;
    // 0x57378: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x5737c: str r7, [r4, #0x648]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1608)?)?;
    // 0x57380: blx 0x83c4c <entry+0x2e44c> @ imm = #0x2c8c4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x57384: and r0, r8, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x57388: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // 0x5738c: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // 0x57390: bic r1, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R0,255)?)?;
    // 0x57394: orr r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,255)?)?;
    // 0x57398: add r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x5739c: str r1, [r4, #0xa90]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2704)?)?;
    // 0x573a0: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x573a4: mul r1, r0, r5
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(5)?)?)?;
    // 0x573a8: str r1, [r4, #0x640]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1600)?)?;
    // 0x573ac: str r0, [r4, #0x644]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1604)?)?;
    // 0x573b0: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x573b4: ldrb r0, [r4, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,953)?)?;
    // 0x573b8: ldrb r2, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,988)?)?;
    // 0x573bc: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x573c0: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x573c4: bl 0x58894 <entry+0x3094>  @ imm = #0x14c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x58894,true)?)?;
    // 0x573c8: str r0, [r4, #0x51c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1308)?)?;
    // 0x573cc: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x573d0: bl 0x58900 <entry+0x3100>  @ imm = #0x1528
    c.emit(arm32::branch(C::Al,c.at as u32,0x58900,true)?)?;
    // 0x573d4: ldr r12, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,2744)?)?;
    // 0x573d8: ldrb r0, [r12, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R12,339)?)?;
    // 0x573dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x573e0: ldrne r0, [r4, #0xae8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,2792)?)?;
    // 0x573e4: strne r0, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1592)?)?;
    // 0x573e8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x573ec: ldr r10, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,0)?)?;
    // 0x573f0: cmp r10, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,8192)?)?;
    // 0x573f4: bge 0x57420 <entry+0x1c20>  @ imm = #0x24
    c.emit(arm32::branch(C::Ge,c.at as u32,0x57420,false)?)?;
    // 0x573f8: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // 0x573fc: beq 0x57444 <entry+0x1c44>  @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57444,false)?)?;
    // 0x57400: cmp r10, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,4096)?)?;
    // 0x57404: bne 0x57484 <entry+0x1c84>  @ imm = #0x78
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57484,false)?)?;
    // 0x57408: ldr r0, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1196)?)?;
    // 0x5740c: ldr r1, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1200)?)?;
    // 0x57410: mul r2, r0, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x57414: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x57418: lsl r1, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Lsl,4)?)?;
    // 0x5741c: b 0x57474 <entry+0x1c74>  @ imm = #0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x57474,false)?)?;
    // 0x57420: beq 0x57464 <entry+0x1c64>  @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57464,false)?)?;
    // 0x57424: cmp r10, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,16384)?)?;
    // 0x57428: bne 0x57484 <entry+0x1c84>  @ imm = #0x54
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57484,false)?)?;
    // 0x5742c: ldr r0, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1196)?)?;
    // 0x57430: ldr r1, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1200)?)?;
    // 0x57434: mul r2, r0, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x57438: lsl r1, r2, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Lsl,6)?)?;
    // 0x5743c: str r1, [r4, #0x69c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1692)?)?;
    // 0x57440: b 0x57488 <entry+0x1c88>  @ imm = #0x40
    c.emit(arm32::branch(C::Al,c.at as u32,0x57488,false)?)?;
    // 0x57444: ldr r0, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1196)?)?;
    // 0x57448: ldr r1, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1200)?)?;
    // 0x5744c: mul r2, r0, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x57450: mov r1, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1536)?)?;
    // 0x57454: add r0, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x57458: add r1, r1, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,4,false)?)?;
    // 0x5745c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x57460: b 0x57474 <entry+0x1c74>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x57474,false)?)?;
    // 0x57464: ldr r0, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2668)?)?;
    // 0x57468: mov r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,512)?)?;
    // 0x5746c: add r1, r1, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,6,false)?)?;
    // 0x57470: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x57474: str r1, [r4, #0x69c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1692)?)?;
    // 0x57478: str r0, [r4, #0xd04]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3332)?)?;
    // 0x5747c: b 0x57488 <entry+0x1c88>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x57488,false)?)?;
 c.check(0x57480)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_57480:[u32;1]=[0x000ee470];
 for value in INLINE_DATA_57480{c.emit(value)?;}
 // Executable A32 block 0x57484..0x5765c.
 c.check(0x57484)?;
    // 0x57484: ldr r1, [r4, #0x69c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1692)?)?;
    // 0x57488: add r0, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,255)?)?;
    // 0x5748c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x57490: str r0, [r4, #0x69c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1692)?)?;
    // 0x57494: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x57498: ldr r1, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,928)?)?;
    // 0x5749c: str r1, [r4, #0x73c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1852)?)?;
    // 0x574a0: ldr r5, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,2652)?)?;
    // 0x574a4: str r5, [r4, #0x714]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1812)?)?;
    // 0x574a8: str r0, [r4, #0x6a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1704)?)?;
    // 0x574ac: str r0, [r4, #0x718]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1816)?)?;
    // 0x574b0: str r0, [r4, #0x740]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1856)?)?;
    // 0x574b4: str r0, [r4, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1716)?)?;
    // 0x574b8: ldrb r0, [r12, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R12,204)?)?;
    // 0x574bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x574c0: bne 0x574d0 <entry+0x1cd0>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x574d0,false)?)?;
    // 0x574c4: ldr r0, [r4, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1244)?)?;
    // 0x574c8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x574cc: bne 0x57524 <entry+0x1d24>  @ imm = #0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57524,false)?)?;
    // 0x574d0: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // 0x574d4: beq 0x574e8 <entry+0x1ce8>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x574e8,false)?)?;
    // 0x574d8: cmp r10, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,8192)?)?;
    // 0x574dc: beq 0x574f4 <entry+0x1cf4>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x574f4,false)?)?;
    // 0x574e0: cmp r10, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,4096)?)?;
    // 0x574e4: bne 0x5751c <entry+0x1d1c>  @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5751c,false)?)?;
    // 0x574e8: bic r0, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R1,15)?)?;
    // 0x574ec: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x574f0: b 0x57500 <entry+0x1d00>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x57500,false)?)?;
    // 0x574f4: mvn r0, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,2147483663)?)?;
    // 0x574f8: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x574fc: and r0, r0, r1, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsr,1,false)?)?;
    // 0x57500: str r0, [r4, #0x6a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1704)?)?;
    // 0x57504: str r0, [r4, #0x740]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1856)?)?;
    // 0x57508: asr r0, r5, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(5)?,Shift::Asr,R::R2,false)?)?;
    // 0x5750c: lsl r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // 0x57510: str r0, [r4, #0x718]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1816)?)?;
    // 0x57514: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x57518: str r0, [r4, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1716)?)?;
    // 0x5751c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x57520: str r0, [r4, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1716)?)?;
    // 0x57524: cmp r10, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,8192)?)?;
    // 0x57528: bne 0x57548 <entry+0x1d48>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57548,false)?)?;
    // 0x5752c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x57530: str r0, [r4, #0x6b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1712)?)?;
    // 0x57534: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x57538: ldrb r2, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,988)?)?;
    // 0x5753c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x57540: movne r0, #96
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,96)?)?;
    // 0x57544: b 0x57554 <entry+0x1d54>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x57554,false)?)?;
    // 0x57548: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x5754c: str r0, [r4, #0x6b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1712)?)?;
    // 0x57550: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x57554: str r0, [r4, #0x6b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1720)?)?;
    // 0x57558: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5755c: str r0, [r4, #0x794]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1940)?)?;
    // 0x57560: ldrb r3, [r4, #0x3d9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,985)?)?;
    // 0x57564: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x57568: beq 0x57584 <entry+0x1d84>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57584,false)?)?;
    // 0x5756c: add r0, r5, #143
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,143)?)?;
    // 0x57570: bic r2, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R0,15)?)?;
    // 0x57574: str r2, [r4, #0x794]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1940)?)?;
    // 0x57578: ldr r0, [r4, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1696)?)?;
    // 0x5757c: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x57580: strhi r0, [r4, #0x794]
    c.emit(arm32::transfer_word(C::Hi,false,R::R0,R::new(4)?,1940)?)?;
    // 0x57584: ldrb lr, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(4)?,988)?)?;
    // 0x57588: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x5758c: beq 0x575fc <entry+0x1dfc>  @ imm = #0x68
    c.emit(arm32::branch(C::Eq,c.at as u32,0x575fc,false)?)?;
    // 0x57590: add r0, r5, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x57594: mov r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1024)?)?;
    // 0x57598: add r2, r2, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // 0x5759c: str r2, [r4, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1708)?)?;
    // 0x575a0: ldr r2, [r12, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,20)?)?;
    // 0x575a4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x575a8: beq 0x57660 <entry+0x1e60>  @ imm = #0xb0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57660,false)?)?;
    // 0x575ac: add r7, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x575b0: mov r6, #80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,80)?)?;
    // 0x575b4: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // 0x575b8: add r7, r6, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x575bc: add r0, r0, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,80)?)?;
    // 0x575c0: str r7, [r4, #0x76c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1900)?)?;
    // 0x575c4: str r7, [r4, #0x744]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1860)?)?;
    // 0x575c8: lsr r7, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R1,Shift::Lsr,1)?)?;
    // 0x575cc: add r7, r7, r7, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x575d0: add r7, r6, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x575d4: str r7, [r4, #0x770]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1904)?)?;
    // 0x575d8: str r7, [r4, #0x750]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1872)?)?;
    // 0x575dc: str r0, [r4, #0x71c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1820)?)?;
    // 0x575e0: add r0, r5, r5, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(5)?,Shift::Lsr,31,false)?)?;
    // 0x575e4: asr r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,1)?)?;
    // 0x575e8: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x575ec: add r7, r6, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::R0,Shift::Lsl,1,false)?)?;
    // 0x575f0: mov r0, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,240)?)?;
    // 0x575f4: mov r6, #400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,400)?)?;
    // 0x575f8: b 0x576a0 <entry+0x1ea0>  @ imm = #0xa0
    c.emit(arm32::branch(C::Al,c.at as u32,0x576a0,false)?)?;
    // 0x575fc: mov r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1024)?)?;
    // 0x57600: add r0, r0, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x57604: str r0, [r4, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1708)?)?;
    // 0x57608: ldr r2, [r12, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,20)?)?;
    // 0x5760c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x57610: beq 0x57670 <entry+0x1e70>  @ imm = #0x58
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57670,false)?)?;
    // 0x57614: mov r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,64)?)?;
    // 0x57618: lsl r0, r5, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsl,3)?)?;
    // 0x5761c: add r7, r7, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::R1,Shift::Lsl,3,false)?)?;
    // 0x57620: add r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
    // 0x57624: str r7, [r4, #0x76c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1900)?)?;
    // 0x57628: str r7, [r4, #0x744]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1860)?)?;
    // 0x5762c: mvn r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,7)?)?;
    // 0x57630: and r6, r7, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(7)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x57634: add r6, r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,64)?)?;
    // 0x57638: str r6, [r4, #0x770]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1904)?)?;
    // 0x5763c: str r6, [r4, #0x750]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1872)?)?;
    // 0x57640: str r0, [r4, #0x71c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1820)?)?;
    // 0x57644: add r0, r5, r5, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(5)?,Shift::Lsr,31,false)?)?;
    // 0x57648: mov r6, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,192)?)?;
    // 0x5764c: and r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(7)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x57650: add r7, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,64)?)?;
    // 0x57654: mov r0, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,192)?)?;
    // 0x57658: b 0x576a0 <entry+0x1ea0>  @ imm = #0x40
    c.emit(arm32::branch(C::Al,c.at as u32,0x576a0,false)?)?;
 c.check(0x5765c)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_5765C:[u32;1]=[0x000ee5e6];
 for value in INLINE_DATA_5765C{c.emit(value)?;}
 // Executable A32 block 0x57660..0x58000.
 c.check(0x57660)?;
    // 0x57660: ldr r7, [r12, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R12,56)?)?;
    // 0x57664: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x57668: bne 0x5767c <entry+0x1e7c>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5767c,false)?)?;
    // 0x5766c: b 0x575ac <entry+0x1dac>  @ imm = #-0xc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x575ac,false)?)?;
    // 0x57670: ldr r0, [r12, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,56)?)?;
    // 0x57674: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57678: beq 0x57614 <entry+0x1e14>  @ imm = #-0x6c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57614,false)?)?;
    // 0x5767c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x57680: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x57684: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x57688: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x5768c: str r2, [r4, #0x76c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1900)?)?;
    // 0x57690: str r2, [r4, #0x744]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1860)?)?;
    // 0x57694: str r2, [r4, #0x770]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1904)?)?;
    // 0x57698: str r2, [r4, #0x750]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1872)?)?;
    // 0x5769c: str r2, [r4, #0x71c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1820)?)?;
    // 0x576a0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x576a4: str r7, [r4, #0x728]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1832)?)?;
    // 0x576a8: str r6, [r4, #0x6c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1728)?)?;
    // 0x576ac: str r0, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1732)?)?;
    // 0x576b0: beq 0x5773c <entry+0x1f3c>  @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5773c,false)?)?;
    // 0x576b4: ldr r0, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2656)?)?;
    // 0x576b8: add r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,31)?)?;
    // 0x576bc: asr r3, r0, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R0,Shift::Asr,31)?)?;
    // 0x576c0: add r0, r0, r3, lsr #27
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsr,27,false)?)?;
    // 0x576c4: asr r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,5)?)?;
    // 0x576c8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x576cc: lsl r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // 0x576d0: str r0, [r4, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1752)?)?;
    // 0x576d4: ldr r7, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,664)?)?;
    // 0x576d8: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x576dc: blo 0x576f8 <entry+0x1ef8>  @ imm = #0x14
    c.emit(arm32::branch(C::Cc,c.at as u32,0x576f8,false)?)?;
    // 0x576e0: ldr r0, [r4, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1752)?)?;
    // 0x576e4: ldr r3, [pc, #0xa34]        @ 0x2920 <entry+0x2920>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,2612)?)?;
    // 0x576e8: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // 0x576ec: add r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,768)?)?;
    // 0x576f0: and r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x576f4: str r0, [r4, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1752)?)?;
    // 0x576f8: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // 0x576fc: str r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    // 0x57700: beq 0x57770 <entry+0x1f70>  @ imm = #0x68
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57770,false)?)?;
    // 0x57704: cmp r10, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,8192)?)?;
    // 0x57708: bne 0x577a8 <entry+0x1fa8>  @ imm = #0x98
    c.emit(arm32::branch(C::Ne,c.at as u32,0x577a8,false)?)?;
    // 0x5770c: mvn r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,63)?)?;
    // 0x57710: mov r3, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,64)?)?;
    // 0x57714: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x57718: and r9, r0, r5, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::R0,R::new(5)?,Shift::Lsl,1,false)?)?;
    // 0x5771c: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x57720: str r9, [r4, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1756)?)?;
    // 0x57724: str r9, [r4, #0x720]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1824)?)?;
    // 0x57728: str r3, [r4, #0x6bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1724)?)?;
    // 0x5772c: beq 0x577c0 <entry+0x1fc0>  @ imm = #0x8c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x577c0,false)?)?;
    // 0x57730: lsr r1, r1, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,5)?)?;
    // 0x57734: lsl r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x57738: b 0x577d4 <entry+0x1fd4>  @ imm = #0x94
    c.emit(arm32::branch(C::Al,c.at as u32,0x577d4,false)?)?;
    // 0x5773c: cmp r10, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,8192)?)?;
    // 0x57740: beq 0x57f10 <entry+0x2710>  @ imm = #0x7c8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57f10,false)?)?;
    // 0x57744: cmp r10, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,4096)?)?;
    // 0x57748: beq 0x57f2c <entry+0x272c>  @ imm = #0x7dc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57f2c,false)?)?;
    // 0x5774c: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // 0x57750: bne 0x576d4 <entry+0x1ed4>  @ imm = #-0x84
    c.emit(arm32::branch(C::Ne,c.at as u32,0x576d4,false)?)?;
    // 0x57754: ldr r0, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2656)?)?;
    // 0x57758: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // 0x5775c: asr r3, r0, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R0,Shift::Asr,31)?)?;
    // 0x57760: add r0, r0, r3, lsr #28
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsr,28,false)?)?;
    // 0x57764: asr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,4)?)?;
    // 0x57768: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x5776c: b 0x576cc <entry+0x1ecc>  @ imm = #-0xa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x576cc,false)?)?;
    // 0x57770: bic r0, r5, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(5)?,15)?)?;
    // 0x57774: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x57778: add r9, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R0,256)?)?;
    // 0x5777c: str r9, [r4, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1756)?)?;
    // 0x57780: str r9, [r4, #0x720]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1824)?)?;
    // 0x57784: ldr r0, [r12, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,56)?)?;
    // 0x57788: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5778c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x57790: biceq r0, r1, #15
    c.emit(arm32::data_immediate(C::Eq,Op::Bic,R::R0,R::R1,15)?)?;
    // 0x57794: moveq r2, #16
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x57798: str r0, [r4, #0x774]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1908)?)?;
    // 0x5779c: str r0, [r4, #0x748]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1864)?)?;
    // 0x577a0: str r2, [r4, #0x6bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1724)?)?;
    // 0x577a4: b 0x577e0 <entry+0x1fe0>  @ imm = #0x34
    c.emit(arm32::branch(C::Al,c.at as u32,0x577e0,false)?)?;
    // 0x577a8: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x577ac: str r9, [r4, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1756)?)?;
    // 0x577b0: str r9, [r4, #0x720]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1824)?)?;
    // 0x577b4: str r9, [r4, #0x748]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1864)?)?;
    // 0x577b8: str r9, [r4, #0x774]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1908)?)?;
    // 0x577bc: b 0x577e0 <entry+0x1fe0>  @ imm = #0x1c
    c.emit(arm32::branch(C::Al,c.at as u32,0x577e0,false)?)?;
    // 0x577c0: ldr r2, [r12, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,56)?)?;
    // 0x577c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x577c8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x577cc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x577d0: beq 0x5803c <entry+0x283c>  @ imm = #0x864
    c.emit(arm32::branch(C::Eq,c.at as u32,0x5803c,false)?)?;
    // 0x577d4: str r0, [r4, #0x774]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1908)?)?;
    // 0x577d8: str r0, [r4, #0x748]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1864)?)?;
    // 0x577dc: str r3, [r4, #0x6bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1724)?)?;
    // 0x577e0: ldr r6, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2656)?)?;
    // 0x577e4: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // 0x577e8: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x577ec: mov r5, lr
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::LR))?;
    // 0x577f0: movne r1, #32
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x577f4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x577f8: blx 0x82be4 <entry+0x2d3e4> @ imm = #0x2b3e4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82be4)?)?;
    // 0x577fc: mov r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,255)?)?;
    // 0x57800: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x57804: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x57808: orr r8, r8, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,256)?)?;
    // 0x5780c: movne r1, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x57810: add r0, r8, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsl,6,false)?)?;
    // 0x57814: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x57818: str r0, [r4, #0x764]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1892)?)?;
    // 0x5781c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x57820: blx 0x82be4 <entry+0x2d3e4> @ imm = #0x2b3bc
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82be4)?)?;
    // 0x57824: mov r12, r8
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(8)?))?;
    // 0x57828: mvn r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,63)?)?;
    // 0x5782c: mov r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // 0x57830: add r0, r12, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::R0,Shift::Lsl,6,false)?)?;
    // 0x57834: and r1, r1, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x57838: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x5783c: add r2, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,256)?)?;
    // 0x57840: str r0, [r4, #0x768]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1896)?)?;
    // 0x57844: add r0, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,255)?)?;
    // 0x57848: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x5784c: str r0, [r4, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1756)?)?;
    // 0x57850: str r2, [r4, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1760)?)?;
    // 0x57854: sub r2, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(7)?,1)?)?;
    // 0x57858: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x5785c: add r5, r0, r3, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x57860: add r1, r1, r3, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x57864: lsl r0, r3, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x57868: add r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,255)?)?;
    // 0x5786c: tst r5, r3, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(5)?,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x57870: beq 0x57884 <entry+0x2084>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57884,false)?)?;
    // 0x57874: add r2, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,7)?)?;
    // 0x57878: lsr r2, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R1,Shift::Lsr,R::R2,false)?)?;
    // 0x5787c: mul r1, r2, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R2,R::R0)?)?;
    // 0x57880: b 0x5788c <entry+0x208c>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5788c,false)?)?;
    // 0x57884: rsb r0, r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,0)?)?;
    // 0x57888: and r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x5788c: ldr r8, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,32)?)?;
    // 0x57890: str r1, [r4, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1760)?)?;
    // 0x57894: mvn r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,63)?)?;
    // 0x57898: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x5789c: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x578a0: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // 0x578a4: ldr r0, [r4, #0x3a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,936)?)?;
    // 0x578a8: and r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x578ac: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x578b0: str r0, [r4, #0x778]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1912)?)?;
    // 0x578b4: ldr r1, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2668)?)?;
    // 0x578b8: add r0, r12, r1, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::R1,Shift::Lsl,4,false)?)?;
    // 0x578bc: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x578c0: str r0, [r4, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1764)?)?;
    // 0x578c4: add r0, r12, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::R1,Shift::Lsl,2,false)?)?;
    // 0x578c8: add r1, r12, r1, asr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R12,R::R1,Shift::Asr,3,false)?)?;
    // 0x578cc: str r2, [r4, #0x6e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1768)?)?;
    // 0x578d0: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x578d4: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // 0x578d8: str r0, [r4, #0x68c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1676)?)?;
    // 0x578dc: str r1, [r4, #0x690]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1680)?)?;
    // 0x578e0: strb r3, [r4, #0x80a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,2058)?)?;
    // 0x578e4: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x578e8: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x578ec: strh r2, [r4, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R2,R::new(4)?,R::new(3)?,false)?)?;
    // 0x578f0: ldr r2, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1196)?)?;
    // 0x578f4: ldr r3, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1200)?)?;
    // 0x578f8: add r2, r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,7)?)?;
    // 0x578fc: lsr r2, r2, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsr,3)?)?;
    // 0x57900: mul r5, r3, r2
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::new(3)?,R::R2)?)?;
    // 0x57904: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x57908: lsl r2, r5, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(5)?,Shift::Lsl,5)?)?;
    // 0x5790c: str r2, [r4, #0x684]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1668)?)?;
    // 0x57910: bic r2, r8, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::new(8)?,15)?)?;
    // 0x57914: add r2, r2, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // 0x57918: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x5791c: str r2, [r4, #0x694]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1684)?)?;
    // 0x57920: mov r2, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,480)?)?;
    // 0x57924: beq 0x57944 <entry+0x2144>  @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57944,false)?)?;
    // 0x57928: cmp r10, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,4096)?)?;
    // 0x5792c: beq 0x57940 <entry+0x2140>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57940,false)?)?;
    // 0x57930: cmp r10, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,8192)?)?;
    // 0x57934: bne 0x5795c <entry+0x215c>  @ imm = #0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5795c,false)?)?;
    // 0x57938: mov r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x5793c: b 0x57944 <entry+0x2144>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x57944,false)?)?;
    // 0x57940: mov r2, #784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,784)?)?;
    // 0x57944: asr r3, r6, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::new(6)?,Shift::Asr,R::new(3)?,false)?)?;
    // 0x57948: mov r6, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,14)?)?;
    // 0x5794c: add r3, r6, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x57950: bic r3, r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,15)?)?;
    // 0x57954: add r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x57958: str r2, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1672)?)?;
    // 0x5795c: ldr r2, [r4, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,944)?)?;
    // 0x57960: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x57964: ldrhs r3, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Cs,true,R::new(3)?,R::new(4)?,1672)?)?;
    // 0x57968: mulhs r6, r3, r2
    c.emit(arm32::multiply_register(C::Cs,R::new(6)?,R::new(3)?,R::R2)?)?;
    // 0x5796c: strhs r6, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Cs,false,R::new(6)?,R::new(4)?,1672)?)?;
    // 0x57970: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x57974: blo 0x5798c <entry+0x218c>  @ imm = #0x10
    c.emit(arm32::branch(C::Cc,c.at as u32,0x5798c,false)?)?;
    // 0x57978: ldr r2, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1672)?)?;
    // 0x5797c: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,255)?)?;
    // 0x57980: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x57984: mul r3, r2, r7
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::new(7)?)?)?;
    // 0x57988: str r3, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1672)?)?;
    // 0x5798c: ldrb r2, [r4, #0x57b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1403)?)?;
    // 0x57990: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x57994: bne 0x57a10 <entry+0x2210>  @ imm = #0x74
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57a10,false)?)?;
    // 0x57998: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x5799c: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // 0x579a0: str r2, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1512)?)?;
    // 0x579a4: str r3, [r4, #0x5ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1516)?)?;
    // 0x579a8: ldrb r3, [r4, #0x3f9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1017)?)?;
    // 0x579ac: cmp r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // 0x579b0: movne r2, #128
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x579b4: movne r3, #64
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,64)?)?;
    // 0x579b8: strne r2, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::new(4)?,1512)?)?;
    // 0x579bc: strne r3, [r4, #0x568]
    c.emit(arm32::transfer_word(C::Ne,false,R::new(3)?,R::new(4)?,1384)?)?;
    // 0x579c0: ldrb r3, [r4, #0x3fa]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1018)?)?;
    // 0x579c4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x579c8: addne r2, r2, #24
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,24)?)?;
    // 0x579cc: strne r2, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::new(4)?,1512)?)?;
    // 0x579d0: ldrb r3, [r4, #0x3fc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1020)?)?;
    // 0x579d4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x579d8: addne r2, r2, #44
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,44)?)?;
    // 0x579dc: strne r2, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::new(4)?,1512)?)?;
    // 0x579e0: ldrb r3, [r4, #0x3fd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1021)?)?;
    // 0x579e4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x579e8: addne r2, r2, #68
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,68)?)?;
    // 0x579ec: strne r2, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::new(4)?,1512)?)?;
    // 0x579f0: ldrb r3, [r4, #0x3fb]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1019)?)?;
    // 0x579f4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x579f8: addne r2, r2, #32
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,32)?)?;
    // 0x579fc: strne r2, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::new(4)?,1512)?)?;
    // 0x57a00: ldr r2, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1592)?)?;
    // 0x57a04: str r2, [r4, #0x63c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1596)?)?;
    // 0x57a08: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x57a0c: b 0x57a20 <entry+0x2220>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x57a20,false)?)?;
    // 0x57a10: ldr r2, [r4, #0x638]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1592)?)?;
    // 0x57a14: str r2, [r4, #0x63c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1596)?)?;
    // 0x57a18: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x57a1c: str r2, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1512)?)?;
    // 0x57a20: str r2, [r4, #0x5ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1516)?)?;
    // 0x57a24: cmp r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    // 0x57a28: ldr r2, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1672)?)?;
    // 0x57a2c: ldr r3, [pc, #0x6ec]        @ 0x2920 <entry+0x2920>
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::PC,1772)?)?;
    // 0x57a30: add r2, r2, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // 0x57a34: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x57a38: mov r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // 0x57a3c: movls r0, r3
    c.emit(arm32::mov_register(C::Ls,R::R0,R::new(3)?))?;
    // 0x57a40: cmp r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,256)?)?;
    // 0x57a44: str r2, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1672)?)?;
    // 0x57a48: mov r2, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32768)?)?;
    // 0x57a4c: movhi r3, r1
    c.emit(arm32::mov_register(C::Hi,R::new(3)?,R::R1))?;
    // 0x57a50: str r0, [r4, #0x68c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1676)?)?;
    // 0x57a54: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // 0x57a58: mov r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2048)?)?;
    // 0x57a5c: str r3, [r4, #0x690]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1680)?)?;
    // 0x57a60: add r3, r4, #1616
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,1616)?)?;
    // 0x57a64: stm r3, {r0, r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // 0x57a68: ldrb r0, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3104)?)?;
    // 0x57a6c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57a70: beq 0x57aa0 <entry+0x22a0>  @ imm = #0x28
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57aa0,false)?)?;
    // 0x57a74: ldr r5, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,36)?)?;
    // 0x57a78: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x57a7c: bl 0x527bc <entry+0xffffffffffffcfbc> @ imm = #-0x52c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x527bc,true)?)?;
    // 0x57a80: str r0, [r4, #0xc24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3108)?)?;
    // 0x57a84: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x57a88: bl 0x52894 <entry+0xffffffffffffd094> @ imm = #-0x51fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x52894,true)?)?;
    // 0x57a8c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x57a90: str r0, [r4, #0xc28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3112)?)?;
    // 0x57a94: add r1, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x57a98: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // 0x57a9c: str r1, [r4, #0xc2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3116)?)?;
    // 0x57aa0: ldrb r0, [r4, #0x419]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1049)?)?;
    // 0x57aa4: ldrb r5, [r4, #0x3fe]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,1022)?)?;
    // 0x57aa8: ldrb r3, [r4, #0x400]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1024)?)?;
    // 0x57aac: ldr r7, [r4, #0x460]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1120)?)?;
    // 0x57ab0: ldrb r2, [r4, #0x401]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1025)?)?;
    // 0x57ab4: ldrb r12, [r4, #0x402]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(4)?,1026)?)?;
    // 0x57ab8: orr r6, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x57abc: orr r1, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x57ac0: orr r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x57ac4: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x57ac8: orr r1, r1, r12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // 0x57acc: strb r1, [r4, #0x57c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1404)?)?;
    // 0x57ad0: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // 0x57ad4: ldr r8, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-32)?)?;
    // 0x57ad8: beq 0x57b70 <entry+0x2370>  @ imm = #0x90
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57b70,false)?)?;
    // 0x57adc: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x57ae0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57ae4: mov lr, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,4)?)?;
    // 0x57ae8: str r1, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1504)?)?;
    // 0x57aec: mov r1, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,56)?)?;
    // 0x57af0: str lr, [r4, #0x5e4]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,1508)?)?;
    // 0x57af4: moveq r1, #24
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x57af8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x57afc: addne r1, r1, #52
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,52)?)?;
    // 0x57b00: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x57b04: addne r1, r1, #44
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,44)?)?;
    // 0x57b08: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x57b0c: addne r1, r1, #136
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,136)?)?;
    // 0x57b10: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x57b14: cmpeq r3, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x57b18: bne 0x57f44 <entry+0x2744>  @ imm = #0x424
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57f44,false)?)?;
    // 0x57b1c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x57b20: strne r1, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(4)?,1504)?)?;
    // 0x57b24: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x57b28: beq 0x57b4c <entry+0x234c>  @ imm = #0x1c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57b4c,false)?)?;
    // 0x57b2c: ldr r0, [r4, #0xa64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2660)?)?;
    // 0x57b30: ldr r2, [r4, #0xa68]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2664)?)?;
    // 0x57b34: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7)?)?;
    // 0x57b38: bic r0, r0, #-2147483641
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,2147483655)?)?;
    // 0x57b3c: mul r3, r2, r0
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R0)?)?;
    // 0x57b40: add r0, r1, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x57b44: add r1, r0, #292
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,292)?)?;
    // 0x57b48: str r1, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1504)?)?;
    // 0x57b4c: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x57b50: ldr r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // 0x57b54: addne r1, r1, #1056
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,1056)?)?;
    // 0x57b58: strne r1, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(4)?,1504)?)?;
    // 0x57b5c: ldrb r0, [r4, #0x40f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1039)?)?;
    // 0x57b60: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57b64: addne r0, r1, #96
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R1,96)?)?;
    // 0x57b68: strne r0, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1504)?)?;
    // 0x57b6c: b 0x57b80 <entry+0x2380>  @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x57b80,false)?)?;
    // 0x57b70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x57b74: str r0, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1504)?)?;
    // 0x57b78: str r0, [r4, #0x5e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1508)?)?;
    // 0x57b7c: ldr r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // 0x57b80: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x57b84: mov r1, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1020)?)?;
    // 0x57b88: mov r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5120)?)?;
    // 0x57b8c: str r0, [r4, #0x4e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1252)?)?;
    // 0x57b90: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x57b94: ldr r2, [r4, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x57b98: ldr r1, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x57b9c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x57ba0: str r1, [r4, #0x428]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1064)?)?;
    // 0x57ba4: str r2, [r4, #0x42c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1068)?)?;
    // 0x57ba8: beq 0x57bb8 <entry+0x23b8>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57bb8,false)?)?;
    // 0x57bac: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x57bb0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x57bb4: strne r0, [r4, #0x4e4]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1252)?)?;
    // 0x57bb8: mov r1, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1040)?)?;
    // 0x57bbc: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x57bc0: ldr r1, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x57bc4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x57bc8: bne 0x57bdc <entry+0x23dc>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57bdc,false)?)?;
    // 0x57bcc: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x57bd0: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x57bd4: orrne r0, r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,2)?)?;
    // 0x57bd8: strne r0, [r4, #0x4e4]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1252)?)?;
    // 0x57bdc: ldrb r1, [r3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(3)?,0)?)?;
    // 0x57be0: cmp r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,255)?)?;
    // 0x57be4: beq 0x57bf8 <entry+0x23f8>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57bf8,false)?)?;
    // 0x57be8: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x57bec: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x57bf0: orrne r0, r0, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,16)?)?;
    // 0x57bf4: strne r0, [r4, #0x4e4]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1252)?)?;
    // 0x57bf8: mov r1, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,230)?)?;
    // 0x57bfc: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x57c00: ldrb r1, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x57c04: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x57c08: bne 0x57c1c <entry+0x241c>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57c1c,false)?)?;
    // 0x57c0c: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x57c10: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x57c14: orrne r0, r0, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,8)?)?;
    // 0x57c18: strne r0, [r4, #0x4e4]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1252)?)?;
    // 0x57c1c: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x57c20: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x57c24: ldr r1, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x57c28: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x57c2c: bne 0x57c40 <entry+0x2440>  @ imm = #0xc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57c40,false)?)?;
    // 0x57c30: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x57c34: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x57c38: orrne r0, r0, #4
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,4)?)?;
    // 0x57c3c: strne r0, [r4, #0x4e4]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,1252)?)?;
    // 0x57c40: ldrb r1, [r4, #0x399]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,921)?)?;
    // 0x57c44: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x57c48: ldreq r1, [r8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(8)?,0)?)?;
    // 0x57c4c: cmpeq r1, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x57c50: bne 0x57c6c <entry+0x246c>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57c6c,false)?)?;
    // 0x57c54: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x57c58: orr r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // 0x57c5c: strb r1, [r4, #0x558]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1368)?)?;
    // 0x57c60: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x57c64: strb r1, [r4, #0x40b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1035)?)?;
    // 0x57c68: str r0, [r4, #0x4e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1252)?)?;
    // 0x57c6c: ldrb r0, [r4, #0x561]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1377)?)?;
    // 0x57c70: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x57c74: blo 0x57c88 <entry+0x2488>  @ imm = #0xc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x57c88,false)?)?;
    // 0x57c78: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x57c7c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x57c80: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x57c84: strbeq r0, [r4, #0x40b]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::new(4)?,1035)?)?;
    // 0x57c88: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x57c8c: str r0, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1656)?)?;
    // 0x57c90: ldrb r0, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1156)?)?;
    // 0x57c94: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57c98: beq 0x57cf0 <entry+0x24f0>  @ imm = #0x50
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57cf0,false)?)?;
    // 0x57c9c: ldr r0, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2652)?)?;
    // 0x57ca0: ldr r1, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2656)?)?;
    // 0x57ca4: add r7, r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,128)?)?;
    // 0x57ca8: asr r2, r1, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Asr,5)?)?;
    // 0x57cac: asr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Asr,4)?)?;
    // 0x57cb0: add r3, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,256)?)?;
    // 0x57cb4: asr r7, r7, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Asr,4)?)?;
    // 0x57cb8: asr r3, r3, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Asr,5)?)?;
    // 0x57cbc: mul r6, r7, r1
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::new(7)?,R::R1)?)?;
    // 0x57cc0: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x57cc4: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x57cc8: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x57ccc: muleq r6, r3, r2
    c.emit(arm32::multiply_register(C::Eq,R::new(6)?,R::new(3)?,R::R2)?)?;
    // 0x57cd0: add r1, r1, r6, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x57cd4: str r6, [r4, #0xd24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,3364)?)?;
    // 0x57cd8: bic r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,15)?)?;
    // 0x57cdc: cmp r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,255)?)?;
    // 0x57ce0: str r1, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1656)?)?;
    // 0x57ce4: movls r1, #256
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x57ce8: strls r1, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Ls,false,R::R1,R::new(4)?,1656)?)?;
    // 0x57cec: b 0x57cf4 <entry+0x24f4>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x57cf4,false)?)?;
    // 0x57cf0: ldr r0, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2652)?)?;
    // 0x57cf4: ldrb r1, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,988)?)?;
    // 0x57cf8: ldr r2, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,928)?)?;
    // 0x57cfc: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x57d00: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x57d04: movne r3, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x57d08: mul r1, r3, r2
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(3)?,R::R2)?)?;
    // 0x57d0c: mul r2, r3, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(3)?,R::R0)?)?;
    // 0x57d10: str r1, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1772)?)?;
    // 0x57d14: str r1, [r4, #0x74c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1868)?)?;
    // 0x57d18: str r2, [r4, #0x724]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1828)?)?;
    // 0x57d1c: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x57d20: ldrb r0, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x57d24: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57d28: beq 0x57d64 <entry+0x2564>  @ imm = #0x34
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57d64,false)?)?;
    // 0x57d2c: ldr r0, [r4, #0x714]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1812)?)?;
    // 0x57d30: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x57d34: str r0, [r4, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1696)?)?;
    // 0x57d38: str r2, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1772)?)?;
    // 0x57d3c: str r0, [r4, #0x6a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1700)?)?;
    // 0x57d40: add r0, r4, #812
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,812)?)?;
    // 0x57d44: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x57d48: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x57d4c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x57d50: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x57d54: str r2, [r0, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,-56)?)?;
    // 0x57d58: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x57d5c: bne 0x57d48 <entry+0x2548>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57d48,false)?)?;
    // 0x57d60: b 0x57d98 <entry+0x2598>  @ imm = #0x30
    c.emit(arm32::branch(C::Al,c.at as u32,0x57d98,false)?)?;
    // 0x57d64: ldr r0, [r4, #0x73c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1852)?)?;
    // 0x57d68: str r0, [r4, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1696)?)?;
    // 0x57d6c: str r1, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1772)?)?;
    // 0x57d70: str r0, [r4, #0x6a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1700)?)?;
    // 0x57d74: add r0, r4, #852
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,852)?)?;
    // 0x57d78: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x57d7c: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x57d80: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x57d84: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x57d88: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x57d8c: str r2, [r0, #-0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,-96)?)?;
    // 0x57d90: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x57d94: bne 0x57d80 <entry+0x2580>  @ imm = #-0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57d80,false)?)?;
    // 0x57d98: ldrb r0, [r4, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // 0x57d9c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57da0: beq 0x57e14 <entry+0x2614>  @ imm = #0x6c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57e14,false)?)?;
    // 0x57da4: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x57da8: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x57dac: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x57db0: bne 0x57e14 <entry+0x2614>  @ imm = #0x5c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57e14,false)?)?;
    // 0x57db4: mov r0, #988
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,988)?)?;
    // 0x57db8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x57dbc: mov r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,256)?)?;
    // 0x57dc0: ldrsb r0, [r4, r0]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::R0,R::new(4)?,R::R0)?)?;
    // 0x57dc4: add r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // 0x57dc8: ldr r1, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2656)?)?;
    // 0x57dcc: mul r2, r0, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x57dd0: add r1, r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,256)?)?;
    // 0x57dd4: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // 0x57dd8: str r1, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1776)?)?;
    // 0x57ddc: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x57de0: sub r3, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R0,1)?)?;
    // 0x57de4: add r6, r2, r7, lsl r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::new(6)?,R::R2,R::new(7)?,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x57de8: lsl r2, r7, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::new(7)?,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x57dec: add r1, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x57df0: tst r6, r7, lsl r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x57df4: beq 0x57e08 <entry+0x2608>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57e08,false)?)?;
    // 0x57df8: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7)?)?;
    // 0x57dfc: lsr r1, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::R0,false)?)?;
    // 0x57e00: mul r0, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::R2)?)?;
    // 0x57e04: b 0x57e10 <entry+0x2610>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x57e10,false)?)?;
    // 0x57e08: rsb r0, r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R2,0)?)?;
    // 0x57e0c: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x57e10: str r0, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1776)?)?;
    // 0x57e14: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x57e18: bl 0x38f70 <entry+0xfffffffffffe3770> @ imm = #-0x1eeb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x38f70,true)?)?;
    // 0x57e1c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x57e20: strb r0, [r4, #0xb7a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2938)?)?;
    // 0x57e24: ldr r5, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,0)?)?;
    // 0x57e28: cmp r5, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8192)?)?;
    // 0x57e2c: bne 0x57e74 <entry+0x2674>  @ imm = #0x40
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57e74,false)?)?;
    // 0x57e30: ldrb r0, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1173)?)?;
    // 0x57e34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57e38: bne 0x57e58 <entry+0x2658>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57e58,false)?)?;
    // 0x57e3c: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x57e40: ldrb r1, [r0, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,204)?)?;
    // 0x57e44: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x57e48: bne 0x57e58 <entry+0x2658>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57e58,false)?)?;
    // 0x57e4c: ldrb r0, [r0, #0xe8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,232)?)?;
    // 0x57e50: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x57e54: bne 0x57e74 <entry+0x2674>  @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57e74,false)?)?;
    // 0x57e58: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x57e5c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x57e60: blo 0x57e74 <entry+0x2674>  @ imm = #0xc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x57e74,false)?)?;
    // 0x57e64: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x57e68: ldr r0, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,56)?)?;
    // 0x57e6c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57e70: beq 0x57f4c <entry+0x274c>  @ imm = #0xd4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57f4c,false)?)?;
    // 0x57e74: add r0, r4, #892
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,892)?)?;
    // 0x57e78: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x57e7c: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x57e80: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // 0x57e84: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x54ca0
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x57e88: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // 0x57e8c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x57e90: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x57e94: sub r1, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,2)?)?;
    // 0x57e98: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x57e9c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x57ea0: ldrls r0, [pc, #0x284]        @ 0x292c <entry+0x292c>
    c.emit(arm32::transfer_word(C::Ls,true,R::R0,R::PC,644)?)?;
    // 0x57ea4: ldrls r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Ls,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x57ea8: str r0, [r4, #0x464]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1124)?)?;
    // 0x57eac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x57eb0: str r0, [r4, #0xc4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3148)?)?;
    // 0x57eb4: str r0, [r4, #0xd20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3360)?)?;
    // 0x57eb8: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x57ebc: ldrb r1, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,336)?)?;
    // 0x57ec0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x57ec4: beq 0x57edc <entry+0x26dc>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x57edc,false)?)?;
    // 0x57ec8: ldrb r2, [r0, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,325)?)?;
    // 0x57ecc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x57ed0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x57ed4: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x57ed8: b 0x57ee0 <entry+0x26e0>  @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x57ee0,false)?)?;
    // 0x57edc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x57ee0: strb r1, [r4, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,3272)?)?;
    // 0x57ee4: ldr r1, [r0, #0x350]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,848)?)?;
    // 0x57ee8: str r1, [r4, #0xd08]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3336)?)?;
    // 0x57eec: ldr r1, [r0, #0x354]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,852)?)?;
    // 0x57ef0: str r1, [r4, #0xd0c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3340)?)?;
    // 0x57ef4: ldr r0, [r0, #0x358]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,856)?)?;
    // 0x57ef8: str r0, [r4, #0xd10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3344)?)?;
    // 0x57efc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x57f00: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x57f04: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x57f08: str r0, [r4, #0x5fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1532)?)?;
    // 0x57f0c: b 0x5733c <entry+0x1b3c>  @ imm = #-0xbd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5733c,false)?)?;
    // 0x57f10: ldr r0, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2656)?)?;
    // 0x57f14: add r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,31)?)?;
    // 0x57f18: asr r3, r0, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R0,Shift::Asr,31)?)?;
    // 0x57f1c: add r0, r0, r3, lsr #27
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsr,27,false)?)?;
    // 0x57f20: mvn r3, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,63)?)?;
    // 0x57f24: and r0, r3, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,1,false)?)?;
    // 0x57f28: b 0x576d0 <entry+0x1ed0>  @ imm = #-0x860
    c.emit(arm32::branch(C::Al,c.at as u32,0x576d0,false)?)?;
    // 0x57f2c: ldr r0, [r4, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2656)?)?;
    // 0x57f30: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // 0x57f34: asr r3, r0, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R0,Shift::Asr,31)?)?;
    // 0x57f38: add r0, r0, r3, lsr #28
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsr,28,false)?)?;
    // 0x57f3c: asr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,4)?)?;
    // 0x57f40: b 0x576c8 <entry+0x1ec8>  @ imm = #-0x880
    c.emit(arm32::branch(C::Al,c.at as u32,0x576c8,false)?)?;
    // 0x57f44: str r1, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1504)?)?;
    // 0x57f48: b 0x57b24 <entry+0x2324>  @ imm = #-0x42c
    c.emit(arm32::branch(C::Al,c.at as u32,0x57b24,false)?)?;
    // 0x57f4c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x57f50: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x57f54: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x57f58: strb r0, [r4, #0xb7a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2938)?)?;
    // 0x57f5c: add r0, r4, #892
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,892)?)?;
    // 0x57f60: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // 0x57f64: bl 0x31ec <entry+0xfffffffffffad9ec> @ imm = #-0x54d80
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x57f68: cmp r5, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8192)?)?;
    // 0x57f6c: bne 0x57e88 <entry+0x2688>  @ imm = #-0xec
    c.emit(arm32::branch(C::Ne,c.at as u32,0x57e88,false)?)?;
    // 0x57f70: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x57f74: mov r12, #996
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,996)?)?;
    // 0x57f78: mov r5, #940
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,940)?)?;
    // 0x57f7c: mov lr, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,63)?)?;
    // 0x57f80: str r0, [r4, #0xb7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2940)?)?;
    // 0x57f84: orr r12, r12, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,1024)?)?;
    // 0x57f88: orr r5, r5, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,5120)?)?;
    // 0x57f8c: orr lr, lr, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,256)?)?;
    // 0x57f90: ldr r0, [r4, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2652)?)?;
    // 0x57f94: asr r1, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Asr,5)?)?;
    // 0x57f98: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x57f9c: add r2, r0, r1, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,4,false)?)?;
    // 0x57fa0: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x57fa4: str r2, [r4, #0xb84]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2948)?)?;
    // 0x57fa8: ldr r3, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,928)?)?;
    // 0x57fac: lsr r3, r3, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsr,5)?)?;
    // 0x57fb0: add r0, r0, r3, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,4,false)?)?;
    // 0x57fb4: bic r7, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::R0,255)?)?;
    // 0x57fb8: str r7, [r4, #0xb88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2952)?)?;
    // 0x57fbc: ldrb r0, [r4, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // 0x57fc0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57fc4: moveq r7, r2
    c.emit(arm32::mov_register(C::Eq,R::new(7)?,R::R2))?;
    // 0x57fc8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x57fcc: str r7, [r4, #0xb80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2944)?)?;
    // 0x57fd0: str r12, [r4, #0xb8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,2956)?)?;
    // 0x57fd4: ldr r7, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,664)?)?;
    // 0x57fd8: ldr r6, [pc, #0x144]        @ 0x2924 <entry+0x2924>
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::PC,324)?)?;
    // 0x57fdc: mla r2, r7, r5, r6
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::new(7)?,R::new(5)?,R::new(6)?)?)?;
    // 0x57fe0: ldr r7, [pc, #0x130]        @ 0x2918 <entry+0x2918>
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::PC,304)?)?;
    // 0x57fe4: umull r6, r5, r2, r7
    c.emit(emit_umull(C::Al,R::new(6)?,R::new(5)?,R::R2,R::new(7)?)?)?;
    // 0x57fe8: lsr r2, r5, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(5)?,Shift::Lsr,8)?)?;
    // 0x57fec: mla r6, r1, r12, lr
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::R1,R::R12,R::LR)?)?;
    // 0x57ff0: add r2, r2, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,2,false)?)?;
    // 0x57ff4: lsl r2, r2, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,6)?)?;
    // 0x57ff8: str r2, [r4, #0xb90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2960)?)?;
    // 0x57ffc: ldr r1, [pc, #0x124]        @ 0x2928 <entry+0x2928>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,292)?)?;
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
