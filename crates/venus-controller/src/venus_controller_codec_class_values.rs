// SPDX-License-Identifier: GPL-3.0-only
//! Continue the Venus A32 controller packet writer from 0x49000 to 0x4a000.
//! Five inline, PC-referenced codec-mode constants [0x49e54,0x49e68) are
//! not executable: the original code branches over them and LDR(pc) reads
//! the final literal. Every instruction uses typed ARM operand constructors.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};

pub const START:usize=0x49000;
pub const END:usize=0x4a000;
const COMPONENT_CLASS_TABLE:[u32;5]=[
    0x0100_0302,0x0001_1111,0x0202_0202,0x0303_0302,0x000e_e028,
];
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,a:usize)->Result<(),String>{
        if self.at!=a{return Err(format!("controller A32 at {:#x}, expected {a:#x}",self.at));} Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0 {return Err("out of Agent 04 owned area".into());}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,condition:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(condition,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,c:C,rd:R,target:usize)->Result<(),String>{
        let offset=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"literal displacement overflow")?;
        self.emit(arm32::transfer_word(c,true,rd,R::PC,offset)?)
    }
}

/// Iterate codec buffer class and width descriptors, dispatch optional mode
/// capability word packets and update bounded stream-buffer pointers.
fn build_controller_class_fields(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x49000: add r0, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // 0x49004: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x49008: blo 0x48e98 <entry+0xfffffffffffffe98> @ imm = #-0x178
    c.branch(C::Cc,0x48e98)?;
    // 0x4900c: add r4, r7, r7, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(7)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x49010: ldr r7, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,92)?)?;
    // 0x49014: mov r8, #700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,700)?)?;
    // 0x49018: mov r10, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,19)?)?;
    // 0x4901c: add r6, sp, #204
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::SP,204)?)?;
    // 0x49020: orr r8, r8, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,14336)?)?;
    // 0x49024: b 0x490b0 <entry+0xb0>    @ imm = #0x84
    c.branch(C::Al,0x490b0)?;
    // 0x49028: add r0, sp, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,108)?)?;
    // 0x4902c: ldr r0, [r0, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(10)?,Shift::Lsl,2)?)?;
    // 0x49030: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x49034: beq 0x49144 <entry+0x144>   @ imm = #0x108
    c.branch(C::Eq,0x49144)?;
    // 0x49038: ldr r1, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,88)?)?;
    // 0x4903c: ldr r2, [r1, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,664)?)?;
    // 0x49040: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x49044: blo 0x49144 <entry+0x144>   @ imm = #0xf8
    c.branch(C::Cc,0x49144)?;
    // 0x49048: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // 0x4904c: and r1, r7, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x49050: mul r3, r0, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    // 0x49054: add r0, r3, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x49058: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4905c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x49060: blx 0x83c4c <entry+0x3ac4c> @ imm = #0x3abe4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x49064: ldr r2, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,100)?)?;
    // 0x49068: ldr r5, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,104)?)?;
    // 0x4906c: mul r1, r2, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R2,R::R0)?)?;
    // 0x49070: orr r2, r1, r9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x49074: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x49078: add r9, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x4907c: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x49080: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x49084: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x49088: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x4908c: str r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // 0x49090: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,4)?)?;
    // 0x49094: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x49098: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4909c: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x490a0: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x490a4: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x490a8: mov r1, #61
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,61)?)?;
    // 0x490ac: b 0x49120 <entry+0x120>   @ imm = #0x6c
    c.branch(C::Al,0x49120)?;
    // 0x490b0: ldr r0, [r6, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::new(10)?,Shift::Lsl,2)?)?;
    // 0x490b4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x490b8: beq 0x49028 <entry+0x28>    @ imm = #-0x98
    c.branch(C::Eq,0x49028)?;
    // 0x490bc: ldr r7, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,92)?)?;
    // 0x490c0: cmp r10, #19
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,19)?)?;
    // 0x490c4: lslhi r7, r4, #5
    c.emit(arm32::mov_shifted_register(C::Hi,R::new(7)?,R::new(4)?,Shift::Lsl,5)?)?;
    // 0x490c8: and r1, r7, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x490cc: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x490d0: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x490d4: blx 0x83c4c <entry+0x3ac4c> @ imm = #0x3ab70
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x490d8: ldr r2, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,100)?)?;
    // 0x490dc: ldr r5, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,104)?)?;
    // 0x490e0: mul r1, r2, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R2,R::R0)?)?;
    // 0x490e4: orr r2, r1, r9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x490e8: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x490ec: add r9, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x490f0: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x490f4: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x490f8: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x490fc: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x49100: str r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // 0x49104: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,4)?)?;
    // 0x49108: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x4910c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x49110: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x49114: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x49118: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x4911c: mov r1, #53
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,53)?)?;
    // 0x49120: ldr r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,96)?)?;
    // 0x49124: cmp r9, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x49128: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4912c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x49130: ldr r2, [pc, #-0x710]       @ 0xfffffffffffffa28 <entry+0xfffffffffffffa28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x49134: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x49138: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x47990
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4913c: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x49140: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x49144: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // 0x49148: add r8, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    // 0x4914c: cmp r10, #24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,24)?)?;
    // 0x49150: bne 0x490b0 <entry+0xb0>    @ imm = #-0xa8
    c.branch(C::Ne,0x490b0)?;
    // 0x49154: ldr r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x49158: ldrb r0, [r10, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,990)?)?;
    // 0x4915c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x49160: bne 0x49238 <entry+0x238>   @ imm = #0xd0
    c.branch(C::Ne,0x49238)?;
    // 0x49164: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x49168: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x4916c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x49170: bne 0x49238 <entry+0x238>   @ imm = #0xc0
    c.branch(C::Ne,0x49238)?;
    // 0x49174: ldr r0, [r10, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,940)?)?;
    // 0x49178: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4917c: bne 0x49238 <entry+0x238>   @ imm = #0xb4
    c.branch(C::Ne,0x49238)?;
    // 0x49180: ldrb r0, [sp, #0x20c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::SP,524)?)?;
    // 0x49184: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x49188: beq 0x49238 <entry+0x238>   @ imm = #0xa8
    c.branch(C::Eq,0x49238)?;
    // 0x4918c: ldr r3, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,104)?)?;
    // 0x49190: orr r1, r9, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,131072)?)?;
    // 0x49194: add r4, r9, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,2)?)?;
    // 0x49198: ldr r0, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,0)?)?;
    // 0x4919c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x491a0: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x491a4: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x491a8: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x491ac: mov r2, #2736
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2736)?)?;
    // 0x491b0: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x491b4: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x491b8: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x491bc: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x491c0: ldr r2, [pc, #-0x7a0]       @ 0xfffffffffffffa28 <entry+0xfffffffffffffa28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x491c4: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x491c8: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x491cc: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x491d0: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // 0x491d4: mov r1, #73
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,73)?)?;
    // 0x491d8: ldr r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,96)?)?;
    // 0x491dc: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x491e0: sub r5, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x491e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x491e8: cmp r5, #39
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,39)?)?;
    // 0x491ec: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x491f0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x47a48
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x491f4: ldr r3, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,104)?)?;
    // 0x491f8: orr r1, r4, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(4)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x491fc: ldr r0, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,0)?)?;
    // 0x49200: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49204: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x49208: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4920c: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x49210: mov r2, #684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,684)?)?;
    // 0x49214: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x49218: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x4921c: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x49220: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x49224: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x49228: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x4922c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49230: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // 0x49234: ldr r9, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,96)?)?;
    // 0x49238: ldr r2, [pc, #-0x818]       @ 0xfffffffffffffa28 <entry+0xfffffffffffffa28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x4923c: mov r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,77)?)?;
    // 0x49240: cmp r9, #1872
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,1872)?)?;
    // 0x49244: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49248: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4924c: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x49250: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x47aa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x49254: ldr r0, [r10, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1696)?)?;
    // 0x49258: ldr r1, [r10, #0x7a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1952)?)?;
    // 0x4925c: ldr r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x49260: sub r9, r11, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,36)?)?;
    // 0x49264: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x49268: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4926c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49270: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x49274: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x49278: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4927c: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x49280: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x49284: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49288: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x4928c: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb268
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x49290: ldr r0, [r10, #0x6a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1700)?)?;
    // 0x49294: ldr r1, [r10, #0x7a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1956)?)?;
    // 0x49298: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x4929c: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x492a0: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x492a4: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x492a8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x492ac: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x492b0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x492b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x492b8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x492bc: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb298
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x492c0: ldr r0, [r10, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1772)?)?;
    // 0x492c4: ldr r1, [r10, #0x8d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2264)?)?;
    // 0x492c8: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x492cc: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x492d0: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x492d4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x492d8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x492dc: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x492e0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x492e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x492e8: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x492ec: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb2c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x492f0: ldrb r0, [r10, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,972)?)?;
    // 0x492f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x492f8: beq 0x493ac <entry+0x3ac>   @ imm = #0xac
    c.branch(C::Eq,0x493ac)?;
    // 0x492fc: ldr r0, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,644)?)?;
    // 0x49300: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x49304: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x49308: bne 0x493ac <entry+0x3ac>   @ imm = #0x9c
    c.branch(C::Ne,0x493ac)?;
    // 0x4930c: ldr r1, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,664)?)?;
    // 0x49310: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x49314: blo 0x49374 <entry+0x374>   @ imm = #0x58
    c.branch(C::Cc,0x49374)?;
    // 0x49318: ldr r0, [r10, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1776)?)?;
    // 0x4931c: blx 0x83c4c <entry+0x3ac4c> @ imm = #0x3a928
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x49320: ldr r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x49324: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x49328: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4932c: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x49330: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x49334: ldr r1, [r10, #0x8dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2268)?)?;
    // 0x49338: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x4933c: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x49340: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x49344: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49348: mla r2, r0, r4, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::new(4)?,R::R1)?)?;
    // 0x4934c: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x49350: stmib sp, {r2, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(4)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x49354: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x49358: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb334
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4935c: ldr r1, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,664)?)?;
    // 0x49360: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x49364: and r0, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(7)?,255)?)?;
    // 0x49368: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4936c: bhi 0x49334 <entry+0x334>   @ imm = #-0x40
    c.branch(C::Hi,0x49334)?;
    // 0x49370: b 0x493ac <entry+0x3ac>   @ imm = #0x34
    c.branch(C::Al,0x493ac)?;
    // 0x49374: ldr r0, [r10, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1776)?)?;
    // 0x49378: ldr r1, [r10, #0x8dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2268)?)?;
    // 0x4937c: ldr r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x49380: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x49384: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49388: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x4938c: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x49390: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x49394: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x49398: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4939c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x493a0: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x493a4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x493a8: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb384
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x493ac: ldr r0, [r10, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1708)?)?;
    // 0x493b0: ldr r1, [r10, #0x7a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1960)?)?;
    // 0x493b4: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x493b8: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x493bc: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x493c0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x493c4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x493c8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x493cc: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x493d0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x493d4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x493d8: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x493dc: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb3b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x493e0: ldr r0, [r10, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1708)?)?;
    // 0x493e4: ldr r1, [r10, #0x7ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1964)?)?;
    // 0x493e8: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x493ec: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x493f0: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x493f4: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x493f8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x493fc: lsr r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,1)?)?;
    // 0x49400: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x49404: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x49408: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4940c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49410: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb3ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x49414: ldr r0, [r10, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1708)?)?;
    // 0x49418: ldr r1, [r10, #0x7b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1968)?)?;
    // 0x4941c: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x49420: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x49424: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x49428: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x4942c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49430: lsr r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,1)?)?;
    // 0x49434: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x49438: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x4943c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x49440: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49444: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb420
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x49448: ldr r0, [r10, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1756)?)?;
    // 0x4944c: ldr r1, [r10, #0x7b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1972)?)?;
    // 0x49450: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x49454: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x49458: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4945c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x49460: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49464: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x49468: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x4946c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49470: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x49474: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb450
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x49478: ldr r0, [r10, #0x954]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2388)?)?;
    // 0x4947c: ldr r1, [r10, #0xb80]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2944)?)?;
    // 0x49480: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x49484: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x49488: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x4948c: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x49490: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49494: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x49498: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4949c: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x494a0: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb47c
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x494a4: ldr r1, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,664)?)?;
    // 0x494a8: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x494ac: blo 0x49508 <entry+0x508>   @ imm = #0x54
    c.branch(C::Cc,0x49508)?;
    // 0x494b0: ldr r0, [r10, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1760)?)?;
    // 0x494b4: blx 0x83c4c <entry+0x3ac4c> @ imm = #0x3a790
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x494b8: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x494bc: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x494c0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x494c4: ldr r0, [r10, #0x7b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1976)?)?;
    // 0x494c8: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x494cc: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x494d0: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x494d4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x494d8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x494dc: mla r1, r4, r6, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(4)?,R::new(6)?,R::R0)?)?;
    // 0x494e0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x494e4: stmib sp, {r1, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit()|R::new(6)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x494e8: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x494ec: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb4c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x494f0: ldr r1, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,664)?)?;
    // 0x494f4: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x494f8: and r4, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(7)?,255)?)?;
    // 0x494fc: cmp r1, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x49500: bhi 0x494c4 <entry+0x4c4>   @ imm = #-0x44
    c.branch(C::Hi,0x494c4)?;
    // 0x49504: b 0x49540 <entry+0x540>   @ imm = #0x34
    c.branch(C::Al,0x49540)?;
    // 0x49508: ldr r0, [r10, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1760)?)?;
    // 0x4950c: ldr r1, [r10, #0x7b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1976)?)?;
    // 0x49510: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x49514: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x49518: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x4951c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49520: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x49524: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x49528: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x4952c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x49530: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49534: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x49538: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb514
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4953c: ldr r1, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,664)?)?;
    // 0x49540: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x49544: blo 0x495a4 <entry+0x5a4>   @ imm = #0x58
    c.branch(C::Cc,0x495a4)?;
    // 0x49548: ldr r0, [r10, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1752)?)?;
    // 0x4954c: blx 0x83c4c <entry+0x3ac4c> @ imm = #0x3a6f8
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x49550: ldr r7, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,104)?)?;
    // 0x49554: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x49558: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4955c: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x49560: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x49564: ldr r1, [r10, #0x7c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1988)?)?;
    // 0x49568: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x4956c: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x49570: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x49574: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49578: mla r2, r0, r4, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::new(4)?,R::R1)?)?;
    // 0x4957c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x49580: stmib sp, {r2, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(4)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x49584: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x49588: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb564
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x4958c: ldr r1, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,664)?)?;
    // 0x49590: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x49594: and r0, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(6)?,255)?)?;
    // 0x49598: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4959c: bhi 0x49564 <entry+0x564>   @ imm = #-0x40
    c.branch(C::Hi,0x49564)?;
    // 0x495a0: b 0x495dc <entry+0x5dc>   @ imm = #0x34
    c.branch(C::Al,0x495dc)?;
    // 0x495a4: ldr r0, [r10, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1752)?)?;
    // 0x495a8: ldr r1, [r10, #0x7c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1988)?)?;
    // 0x495ac: ldr r7, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,104)?)?;
    // 0x495b0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x495b4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x495b8: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x495bc: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x495c0: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x495c4: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x495c8: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x495cc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x495d0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x495d4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x495d8: bl 0x3e02c <entry+0xffffffffffff502c> @ imm = #-0xb5b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x495dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x495e0: mov r12, r7
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(7)?))?;
    // 0x495e4: mov r4, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,124)?)?;
    // 0x495e8: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x495ec: str r0, [sp, #0x334]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,820)?)?;
    // 0x495f0: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x495f4: ldr r6, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,0)?)?;
    // 0x495f8: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x495fc: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x49600: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x49604: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x49608: mov r1, #2832
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2832)?)?;
    // 0x4960c: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x49610: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // 0x49614: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x49618: ldr r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-36)?)?;
    // 0x4961c: add r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,1)?)?;
    // 0x49620: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x49624: add r0, r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,8)?)?;
    // 0x49628: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x4962c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49630: ldrb r5, [r10, #0x3be]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,958)?)?;
    // 0x49634: ldrb r1, [r10, #0x3bb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,955)?)?;
    // 0x49638: ldrb r3, [r10, #0x3bd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,957)?)?;
    // 0x4963c: ldrb r2, [r10, #0x3bc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,956)?)?;
    // 0x49640: and r5, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x49644: mov r4, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,2)?)?;
    // 0x49648: and r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,1)?)?;
    // 0x4964c: and r3, r4, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x49650: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x49654: orr r1, r1, r2, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,7,false)?)?;
    // 0x49658: ldrb r2, [r10, #0x3c2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,962)?)?;
    // 0x4965c: orr r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x49660: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x49664: and r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,1)?)?;
    // 0x49668: orr r1, r1, r2, lsl #11
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,11,false)?)?;
    // 0x4966c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x49670: str r1, [sp, #0x338]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,824)?)?;
    // 0x49674: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x49678: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4967c: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x49680: mov r0, #788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,788)?)?;
    // 0x49684: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x49688: str r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // 0x4968c: str r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,12)?)?;
    // 0x49690: add r1, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,2)?)?;
    // 0x49694: add r0, r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,16)?)?;
    // 0x49698: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4969c: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x496a0: ldr r5, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2744)?)?;
    // 0x496a4: ldrb r4, [r5, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,1308)?)?;
    // 0x496a8: ldr r3, [pc, #0x7a4]        @ 0xe54 <entry+0xe54>
    c.literal(C::Al,R::new(3)?,0x49e54)?;
    // 0x496ac: ldr r2, [pc, #0x7a4]        @ 0xe58 <entry+0xe58>
    c.literal(C::Al,R::R2,0x49e58)?;
    // 0x496b0: cmp r4, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,13)?)?;
    // 0x496b4: bne 0x4976c <entry+0x76c>   @ imm = #0xb0
    c.branch(C::Ne,0x4976c)?;
    // 0x496b8: mov r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,100)?)?;
    // 0x496bc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x496c0: str r1, [sp, #0x344]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,836)?)?;
    // 0x496c4: mov r1, #1200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1200)?)?;
    // 0x496c8: str r1, [sp, #0x340]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,832)?)?;
    // 0x496cc: str r1, [sp, #0x33c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,828)?)?;
    // 0x496d0: ldreq r5, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R0,0)?)?;
    // 0x496d4: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x496d8: streq r5, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R0,0)?)?;
    // 0x496dc: mov r0, #2848
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2848)?)?;
    // 0x496e0: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x496e4: str r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // 0x496e8: str r1, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,20)?)?;
    // 0x496ec: add r0, r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,3)?)?;
    // 0x496f0: add r1, r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,24)?)?;
    // 0x496f4: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x496f8: str r1, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x496fc: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x49700: ldreq r5, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R1,0)?)?;
    // 0x49704: ldr r0, [sp, #0x340]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,832)?)?;
    // 0x49708: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4970c: streq r5, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R1,0)?)?;
    // 0x49710: mov r1, #804
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,804)?)?;
    // 0x49714: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x49718: str r1, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,24)?)?;
    // 0x4971c: str r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,28)?)?;
    // 0x49720: add r0, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,4)?)?;
    // 0x49724: add r1, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,32)?)?;
    // 0x49728: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x4972c: str r1, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x49730: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x49734: ldreq r5, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R1,0)?)?;
    // 0x49738: ldr r0, [sp, #0x344]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,836)?)?;
    // 0x4973c: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x49740: streq r5, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R1,0)?)?;
    // 0x49744: mov r1, #808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,808)?)?;
    // 0x49748: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x4974c: str r1, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,32)?)?;
    // 0x49750: str r0, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,36)?)?;
    // 0x49754: add r1, r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,5)?)?;
    // 0x49758: add r0, r6, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,40)?)?;
    // 0x4975c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49760: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x49764: ldr r5, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2744)?)?;
    // 0x49768: ldrb r4, [r5, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,1308)?)?;
    // 0x4976c: sub r7, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::new(4)?,8)?)?;
    // 0x49770: and r7, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,255)?)?;
    // 0x49774: cmp r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4)?)?;
    // 0x49778: bhi 0x497c0 <entry+0x7c0>   @ imm = #0x40
    c.branch(C::Hi,0x497c0)?;
    // 0x4977c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x49780: cmp r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,12)?)?;
    // 0x49784: str r7, [sp, #0x354]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,852)?)?;
    // 0x49788: str r2, [sp, #0x348]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,840)?)?;
    // 0x4978c: str r3, [sp, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,844)?)?;
    // 0x49790: mov r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // 0x49794: str r3, [sp, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,848)?)?;
    // 0x49798: add r3, sp, #300
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,300)?)?;
    // 0x4979c: beq 0x4987c <entry+0x87c>   @ imm = #0xd8
    c.branch(C::Eq,0x4987c)?;
    // 0x497a0: lsl r7, r4, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(4)?,Shift::Lsl,24)?)?;
    // 0x497a4: asr r7, r7, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Asr,24)?)?;
    // 0x497a8: cmp r7, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,11)?)?;
    // 0x497ac: beq 0x49884 <entry+0x884>   @ imm = #0xd0
    c.branch(C::Eq,0x49884)?;
    // 0x497b0: cmp r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,10)?)?;
    // 0x497b4: bne 0x49890 <entry+0x890>   @ imm = #0xd4
    c.branch(C::Ne,0x49890)?;
    // 0x497b8: add r3, r3, #544
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,544)?)?;
    // 0x497bc: b 0x49888 <entry+0x888>   @ imm = #0xc4
    c.branch(C::Al,0x49888)?;
    // 0x497c0: cmp r4, #30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,30)?)?;
    // 0x497c4: bne 0x4984c <entry+0x84c>   @ imm = #0x80
    c.branch(C::Ne,0x4984c)?;
    // 0x497c8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x497cc: sub r2, r2, #17
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,17)?)?;
    // 0x497d0: str r7, [sp, #0x354]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,852)?)?;
    // 0x497d4: str r7, [sp, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,848)?)?;
    // 0x497d8: str r7, [sp, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,844)?)?;
    // 0x497dc: str r7, [sp, #0x348]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,840)?)?;
    // 0x497e0: ldr r7, [r5, #0x520]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,1312)?)?;
    // 0x497e4: and r6, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(7)?,1)?)?;
    // 0x497e8: tst r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,2)?)?;
    // 0x497ec: rsb r6, r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(6)?,R::new(6)?,0)?)?;
    // 0x497f0: and r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x497f4: orrne r2, r2, #17
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::R2,17)?)?;
    // 0x497f8: tst r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,3)?)?;
    // 0x497fc: strne r2, [sp, #0x348]
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::SP,840)?)?;
    // 0x49800: str r3, [sp, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,844)?)?;
    // 0x49804: mov r3, #260
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,260)?)?;
    // 0x49808: tst r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,4)?)?;
    // 0x4980c: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x49810: str r3, [sp, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,848)?)?;
    // 0x49814: beq 0x4982c <entry+0x82c>   @ imm = #0x10
    c.branch(C::Eq,0x4982c)?;
    // 0x49818: ldr r3, [pc, #0x63c]        @ 0xe5c <entry+0xe5c>
    c.literal(C::Al,R::new(3)?,0x49e5c)?;
    // 0x4981c: str r3, [sp, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,844)?)?;
    // 0x49820: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x49824: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x49828: str r3, [sp, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,848)?)?;
    // 0x4982c: tst r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,8)?)?;
    // 0x49830: beq 0x49890 <entry+0x890>   @ imm = #0x58
    c.branch(C::Eq,0x49890)?;
    // 0x49834: ldr r3, [pc, #0x624]        @ 0xe60 <entry+0xe60>
    c.literal(C::Al,R::new(3)?,0x49e60)?;
    // 0x49838: str r3, [sp, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,844)?)?;
    // 0x4983c: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x49840: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // 0x49844: str r3, [sp, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,848)?)?;
    // 0x49848: b 0x49890 <entry+0x890>   @ imm = #0x40
    c.branch(C::Al,0x49890)?;
    // 0x4984c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49850: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x49854: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x49858: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4985c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x49860: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x49864: mov r2, #2864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2864)?)?;
    // 0x49868: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x4986c: stm r0, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x49870: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x49874: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49878: b 0x49958 <entry+0x958>   @ imm = #0xd8
    c.branch(C::Al,0x49958)?;
    // 0x4987c: add r3, r3, #552
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,552)?)?;
    // 0x49880: b 0x49888 <entry+0x888>   @ imm = #0x0
    c.branch(C::Al,0x49888)?;
    // 0x49884: add r3, r3, #548
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,548)?)?;
    // 0x49888: ldr r7, [r5, #0x520]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,1312)?)?;
    // 0x4988c: str r7, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49890: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49894: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x49898: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4989c: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x498a0: mov r3, #2864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2864)?)?;
    // 0x498a4: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // 0x498a8: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x498ac: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x498b0: add r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1)?)?;
    // 0x498b4: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x498b8: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x498bc: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // 0x498c0: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x498c4: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x498c8: ldr r2, [sp, #0x34c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,844)?)?;
    // 0x498cc: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x498d0: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x498d4: mov r3, #820
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,820)?)?;
    // 0x498d8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x498dc: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x498e0: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x498e4: add r2, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,2)?)?;
    // 0x498e8: add r3, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,16)?)?;
    // 0x498ec: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x498f0: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // 0x498f4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x498f8: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x498fc: ldr r2, [sp, #0x350]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,848)?)?;
    // 0x49900: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x49904: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49908: mov r3, #824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,824)?)?;
    // 0x4990c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x49910: str r3, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,16)?)?;
    // 0x49914: str r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // 0x49918: add r2, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,3)?)?;
    // 0x4991c: add r3, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,24)?)?;
    // 0x49920: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // 0x49924: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x49928: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // 0x4992c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x49930: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49934: ldr r2, [sp, #0x354]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,852)?)?;
    // 0x49938: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4993c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49940: mov r3, #828
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,828)?)?;
    // 0x49944: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x49948: str r3, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,24)?)?;
    // 0x4994c: str r2, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,28)?)?;
    // 0x49950: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49954: add r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x49958: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x4995c: mov r4, #220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,220)?)?;
    // 0x49960: ldrb r0, [r10, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,990)?)?;
    // 0x49964: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x49968: beq 0x49d48 <entry+0xd48>   @ imm = #0x3d8
    c.branch(C::Eq,0x49d48)?;
    // 0x4996c: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x49970: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x49974: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x49978: bne 0x49d48 <entry+0xd48>   @ imm = #0x3c8
    c.branch(C::Ne,0x49d48)?;
    // 0x4997c: ldrb r0, [r10, #0x3d9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,985)?)?;
    // 0x49980: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x49984: bne 0x49d48 <entry+0xd48>   @ imm = #0x3bc
    c.branch(C::Ne,0x49d48)?;
    // 0x49988: ldr r8, [pc, #-0xf68]       @ 0xfffffffffffffa28 <entry+0xfffffffffffffa28>
    c.literal(C::Al,R::new(8)?,0x48a28)?;
    // 0x4998c: ldr r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x49990: mov r9, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,34)?)?;
    // 0x49994: mov r4, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,56)?)?;
    // 0x49998: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4999c: orr r9, r9, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,10496)?)?;
    // 0x499a0: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x499a4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x499a8: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x499ac: beq 0x499a0 <entry+0x9a0>   @ imm = #-0x14
    c.branch(C::Eq,0x499a0)?;
    // 0x499b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x499b4: mov r3, #908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,908)?)?;
    // 0x499b8: and r7, r4, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x499bc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x499c0: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x499c4: orr r2, r0, r5, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::new(5)?,Shift::Lsl,4,false)?)?;
    // 0x499c8: ldr r0, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,0)?)?;
    // 0x499cc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x499d0: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x499d4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x499d8: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x499dc: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x499e0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x499e4: orr r1, r7, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(7)?,262144)?)?;
    // 0x499e8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x499ec: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x499f0: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x499f4: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x499f8: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // 0x499fc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x49a00: ldreq r6, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(3)?,0)?)?;
    // 0x49a04: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x49a08: streq r6, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(3)?,0)?)?;
    // 0x49a0c: mov r3, #904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,904)?)?;
    // 0x49a10: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x49a14: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x49a18: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x49a1c: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x49a20: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x49a24: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x49a28: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49a2c: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x49a30: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x49a34: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x49a38: and r0, r0, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49a3c: cmp r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,768)?)?;
    // 0x49a40: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49a44: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x49a48: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x482a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x49a4c: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x49a50: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x49a54: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
    // 0x49a58: add r4, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,4)?)?;
    // 0x49a5c: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x49a60: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x49a64: bne 0x499a0 <entry+0x9a0>   @ imm = #-0xcc
    c.branch(C::Ne,0x499a0)?;
    // 0x49a68: ldr r0, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,644)?)?;
    // 0x49a6c: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // 0x49a70: mov r6, #45
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,45)?)?;
    // 0x49a74: mov r7, #908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,908)?)?;
    // 0x49a78: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x49a7c: orr r6, r6, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,10496)?)?;
    // 0x49a80: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // 0x49a84: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x49a88: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x49a8c: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x49a90: ldrbne r10, [r0, #0x3b0]
    c.emit(arm32::transfer_byte(C::Ne,true,R::new(10)?,R::R0,944)?)?;
    // 0x49a94: ldr r0, [r0, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,668)?)?;
    // 0x49a98: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x49a9c: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x49aa0: movhi r1, #15
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x49aa4: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x49aa8: moveq r10, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x49aac: str r1, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,96)?)?;
    // 0x49ab0: b 0x49b6c <entry+0xb6c>   @ imm = #0xb4
    c.branch(C::Al,0x49b6c)?;
    // 0x49ab4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x49ab8: and r5, r4, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49abc: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x49ac0: orr r2, r1, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R0,Shift::Lsl,4,false)?)?;
    // 0x49ac4: ldr r0, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,0)?)?;
    // 0x49ac8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49acc: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x49ad0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x49ad4: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x49ad8: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // 0x49adc: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x49ae0: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x49ae4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x49ae8: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x49aec: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x49af0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x49af4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x49af8: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // 0x49afc: bne 0x49b14 <entry+0xb14>   @ imm = #0x10
    c.branch(C::Ne,0x49b14)?;
    // 0x49b00: ldr r7, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49b04: and r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x49b08: str r7, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49b0c: mov r7, #908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,908)?)?;
    // 0x49b10: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // 0x49b14: mov r3, #904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,904)?)?;
    // 0x49b18: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x49b1c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x49b20: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x49b24: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x49b28: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x49b2c: ldr r2, [pc, #0x330]        @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x49b30: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49b34: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x49b38: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x49b3c: mov r1, #50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,50)?)?;
    // 0x49b40: and r0, r0, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49b44: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x49b48: cmp r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,768)?)?;
    // 0x49b4c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49b50: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x49b54: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x483ac
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x49b58: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x49b5c: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x49b60: add r4, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(5)?,4)?)?;
    // 0x49b64: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x49b68: b 0x49c5c <entry+0xc5c>   @ imm = #0xec
    c.branch(C::Al,0x49c5c)?;
    // 0x49b6c: and r1, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R2,255)?)?;
    // 0x49b70: str r2, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,100)?)?;
    // 0x49b74: cmp r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,13)?)?;
    // 0x49b78: bhi 0x49ab4 <entry+0xab4>   @ imm = #-0xcc
    c.branch(C::Hi,0x49ab4)?;
    // 0x49b7c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x49b80: mov r3, #10880
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10880)?)?;
    // 0x49b84: tst r3, r2, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::R2,Shift::Lsl,R::R1,false)?)?;
    // 0x49b88: beq 0x49ab4 <entry+0xab4>   @ imm = #-0xdc
    c.branch(C::Eq,0x49ab4)?;
    // 0x49b8c: lsl r5, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R0,Shift::Lsl,4)?)?;
    // 0x49b90: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49b94: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x49b98: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x49b9c: orr r0, r5, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,8,false)?)?;
    // 0x49ba0: and r8, r4, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49ba4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x49ba8: orr r2, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x49bac: ldr r0, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,0)?)?;
    // 0x49bb0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49bb4: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x49bb8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x49bbc: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x49bc0: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // 0x49bc4: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x49bc8: orr r1, r8, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,262144)?)?;
    // 0x49bcc: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x49bd0: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x49bd4: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x49bd8: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x49bdc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x49be0: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // 0x49be4: bne 0x49bfc <entry+0xbfc>   @ imm = #0x10
    c.branch(C::Ne,0x49bfc)?;
    // 0x49be8: ldr r7, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49bec: and r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x49bf0: str r7, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49bf4: mov r7, #908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,908)?)?;
    // 0x49bf8: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // 0x49bfc: mov r3, #904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,904)?)?;
    // 0x49c00: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x49c04: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x49c08: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x49c0c: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x49c10: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x49c14: ldr r2, [pc, #0x248]        @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x49c18: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49c1c: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x49c20: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x49c24: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x49c28: and r0, r0, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49c2c: cmp r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,768)?)?;
    // 0x49c30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49c34: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x49c38: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48490
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x49c3c: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x49c40: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x49c44: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x49c48: add r4, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,4)?)?;
    // 0x49c4c: and r0, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(9)?,255)?)?;
    // 0x49c50: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x49c54: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x49c58: bhi 0x49b98 <entry+0xb98>   @ imm = #-0xc8
    c.branch(C::Hi,0x49b98)?;
    // 0x49c5c: ldr r2, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,100)?)?;
    // 0x49c60: ldr r1, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,96)?)?;
    // 0x49c64: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x49c68: and r0, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R2,255)?)?;
    // 0x49c6c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x49c70: bhi 0x49b6c <entry+0xb6c>   @ imm = #-0x10c
    c.branch(C::Hi,0x49b6c)?;
    // 0x49c74: ldr r10, [pc, #0x1e8]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::new(10)?,0x49e64)?;
    // 0x49c78: mov r9, #55
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,55)?)?;
    // 0x49c7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49c80: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x49c84: orr r9, r9, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,10496)?)?;
    // 0x49c88: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x49c8c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x49c90: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x49c94: beq 0x49c88 <entry+0xc88>   @ imm = #-0x14
    c.branch(C::Eq,0x49c88)?;
    // 0x49c98: ldr r0, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,0)?)?;
    // 0x49c9c: mov r3, #908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,908)?)?;
    // 0x49ca0: orr r2, r8, r5, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(8)?,R::new(5)?,Shift::Lsl,4,false)?)?;
    // 0x49ca4: and r7, r4, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49ca8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x49cac: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49cb0: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x49cb4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x49cb8: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x49cbc: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x49cc0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x49cc4: orr r1, r7, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(7)?,262144)?)?;
    // 0x49cc8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x49ccc: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x49cd0: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x49cd4: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x49cd8: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // 0x49cdc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x49ce0: ldreq r6, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(3)?,0)?)?;
    // 0x49ce4: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x49ce8: streq r6, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(3)?,0)?)?;
    // 0x49cec: mov r3, #904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,904)?)?;
    // 0x49cf0: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x49cf4: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x49cf8: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x49cfc: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x49d00: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x49d04: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x49d08: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49d0c: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x49d10: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x49d14: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x49d18: and r0, r0, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49d1c: cmp r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,768)?)?;
    // 0x49d20: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49d24: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x49d28: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48580
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x49d2c: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x49d30: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x49d34: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
    // 0x49d38: add r4, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,4)?)?;
    // 0x49d3c: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x49d40: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // 0x49d44: bne 0x49c88 <entry+0xc88>   @ imm = #-0xc4
    c.branch(C::Ne,0x49c88)?;
    // 0x49d48: ldr r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,84)?)?;
    // 0x49d4c: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x49d50: bne 0x49e68 <entry+0xe68>   @ imm = #0x110
    c.branch(C::Ne,0x49e68)?;
    // 0x49d54: ldr r2, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,0)?)?;
    // 0x49d58: ldr r1, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,88)?)?;
    // 0x49d5c: mov r6, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,900)?)?;
    // 0x49d60: mov r7, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,18)?)?;
    // 0x49d64: and r0, r4, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49d68: orr r6, r6, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,14336)?)?;
    // 0x49d6c: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x49d70: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x49d74: ldr r1, [r1, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1752)?)?;
    // 0x49d78: ldreq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R2,0)?)?;
    // 0x49d7c: add r1, r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,63)?)?;
    // 0x49d80: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x49d84: lsr r1, r1, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,6)?)?;
    // 0x49d88: streq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R2,0)?)?;
    // 0x49d8c: stm r2, {r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x49d90: orr r3, r0, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::R1,Shift::Lsl,16,false)?)?;
    // 0x49d94: add r9, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x49d98: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49d9c: ldr r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-36)?)?;
    // 0x49da0: and r1, r9, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49da4: add r5, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(7)?,1)?)?;
    // 0x49da8: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x49dac: add r5, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,8)?)?;
    // 0x49db0: str r5, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R12,0)?)?;
    // 0x49db4: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x49db8: ldreq r4, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(5)?,0)?)?;
    // 0x49dbc: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x49dc0: streq r4, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(5)?,0)?)?;
    // 0x49dc4: mov r5, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,15232)?)?;
    // 0x49dc8: str r5, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,8)?)?;
    // 0x49dcc: add r5, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(7)?,2)?)?;
    // 0x49dd0: str r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,12)?)?;
    // 0x49dd4: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x49dd8: add r5, r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,16)?)?;
    // 0x49ddc: str r5, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R12,0)?)?;
    // 0x49de0: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x49de4: ldreq r4, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(5)?,0)?)?;
    // 0x49de8: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x49dec: streq r4, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(5)?,0)?)?;
    // 0x49df0: str r6, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,16)?)?;
    // 0x49df4: mov r5, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,18)?)?;
    // 0x49df8: add r6, r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,3)?)?;
    // 0x49dfc: str r5, [r2, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,20)?)?;
    // 0x49e00: str r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // 0x49e04: add r6, r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,24)?)?;
    // 0x49e08: str r6, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,0)?)?;
    // 0x49e0c: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x49e10: ldreq r5, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::new(6)?,0)?)?;
    // 0x49e14: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x49e18: streq r5, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::new(6)?,0)?)?;
    // 0x49e1c: mov r6, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,15232)?)?;
    // 0x49e20: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x49e24: mov r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // 0x49e28: str r6, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,24)?)?;
    // 0x49e2c: str r3, [r2, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,28)?)?;
    // 0x49e30: add r3, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(7)?,4)?)?;
    // 0x49e34: add r2, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,32)?)?;
    // 0x49e38: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x49e3c: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x49e40: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x49e44: str r2, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,0)?)?;
    // 0x49e48: ldr r2, [pc, #0x14]         @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x49e4c: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x486a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x49e50: b 0x49f98 <entry+0xf98>   @ imm = #0x140
    c.branch(C::Al,0x49f98)?;
    c.check(0x49e54)?;
    for word in COMPONENT_CLASS_TABLE {c.emit(word)?;}
    c.check(0x49e68)?;
    // 0x49e68: ldr r0, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,0)?)?;
    // 0x49e6c: mov r8, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,900)?)?;
    // 0x49e70: and r1, r4, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49e74: orr r8, r8, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,14336)?)?;
    // 0x49e78: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x49e7c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49e80: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x49e84: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x49e88: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x49e8c: mov r2, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,18)?)?;
    // 0x49e90: str r8, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,0)?)?;
    // 0x49e94: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x49e98: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x49e9c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x49ea0: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x49ea4: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x49ea8: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x49eac: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // 0x49eb0: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x49eb4: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49eb8: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x49ebc: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49ec0: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x49ec4: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x49ec8: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x49ecc: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x49ed0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x49ed4: ldr r2, [pc, #-0x78]        @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x49ed8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49edc: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x49ee0: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x49ee4: mov r1, #66
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,66)?)?;
    // 0x49ee8: and r5, r0, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // 0x49eec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49ef0: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x49ef4: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x49ef8: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x49efc: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48754
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x49f00: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x49f04: mov r2, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,18)?)?;
    // 0x49f08: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x49f0c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49f10: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x49f14: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x49f18: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x49f1c: str r8, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,0)?)?;
    // 0x49f20: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x49f24: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x49f28: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x49f2c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x49f30: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x49f34: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x49f38: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x49f3c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x49f40: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49f44: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x49f48: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x49f4c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x49f50: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x49f54: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x49f58: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x49f5c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x49f60: ldr r2, [pc, #-0x104]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x49f64: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x49f68: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x49f6c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x49f70: add r0, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,8)?)?;
    // 0x49f74: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x49f78: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x49f7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x49f80: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x49f84: mov r1, #67
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,67)?)?;
    // 0x49f88: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x49f8c: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x49f90: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x487e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x49f94: add r9, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(5)?,4)?)?;
    // 0x49f98: ldr r0, [sp, #0x228]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,552)?)?;
    // 0x49f9c: and r1, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,3)?)?;
    // 0x49fa0: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x49fa4: beq 0x4a0e8 <entry+0x10e8>  @ imm = #0x13c
    c.branch(C::Eq,0x4a0e8)?;
    // 0x49fa8: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x49fac: mov r5, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,900)?)?;
    // 0x49fb0: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x49fb4: orr r5, r5, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,14336)?)?;
    // 0x49fb8: orr r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x49fbc: and r1, r9, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x49fc0: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x49fc4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x49fc8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x49fcc: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x49fd0: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x49fd4: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x49fd8: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x49fdc: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x49fe0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x49fe4: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x49fe8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x49fec: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x49ff0: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x49ff4: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x49ff8: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x49ffc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();build_controller_class_fields(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn inline_codec_class_table_is_explicit_data(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        for (i,value) in COMPONENT_CLASS_TABLE.iter().enumerate(){
            let offset=0x49e54-START+i*4;
            assert_eq!(u32::from_le_bytes(b[offset..offset+4].try_into().unwrap()),*value);
        }
    }
}
