// SPDX-License-Identifier: GPL-3.0-only
//! A32 codec controller bit-field serialization, including optional mask and
//! controller callback metadata. 0x460b8 is a literal address, *not* an A32
//! instruction. The overall calling function continues past this interval.
//! This is instruction-level reconstruction, not a hardware-tested equivalent.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::venus_controller_packets::{load_halfword_register};

pub const START:usize=0x46000;
pub const END:usize=0x46800;
const CALLBACK_POINTER:u32=0x000e_e028;

struct Code {at:usize, bytes:Vec<u8>}
impl Code {
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at!=addr{return Err(format!("codec capability write position 0x{:x} vs expected 0x{:x}",self.at,addr));}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0{return Err("outside Agent 20-04 area".into());}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,c:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,c:C,rd:R,target:usize)->Result<(),String>{
        let displacement=target as i64-self.at as i64-8;
        let offset=i32::try_from(displacement).map_err(|_|"invalid ARM32 literal offset")?;
        self.emit(arm32::transfer_word(c,true,rd,R::PC,offset)?)
    }
}

/// Continue controller command packets with callback-dependent flags.
fn write_extended_features(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x46000: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x46004: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x46008: mov r1, #150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,150)?)?;
    // 0x4600c: moveq r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x46010: orr r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x46014: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x46018: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x4601c: bl 0x17b0 <entry+0xfffffffffffbb7b0> @ imm = #-0x44874
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x46020: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x46024: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x46028: mov r6, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,31744)?)?;
    // 0x4602c: ldrb r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x46030: ldrb r2, [r0, #0xda]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,218)?)?;
    // 0x46034: ldrb r3, [r0, #0xdb]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,219)?)?;
    // 0x46038: ldrb r0, [r0, #0xdc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,220)?)?;
    // 0x4603c: and r0, r6, r0, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(6)?,R::R0,Shift::Lsl,10,false)?)?;
    // 0x46040: mov r6, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,992)?)?;
    // 0x46044: and r1, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,31)?)?;
    // 0x46048: and r2, r6, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,5,false)?)?;
    // 0x4604c: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x46050: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x46054: and r1, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(3)?,31)?)?;
    // 0x46058: orr r1, r0, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,15,false)?)?;
    // 0x4605c: ldr r0, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x46060: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46064: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x46068: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4606c: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x46070: mov r2, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x46074: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x46078: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x4607c: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x46080: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x46084: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x46088: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x4608c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x46090: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46094: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x46098: ldrb r1, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,339)?)?;
    // 0x4609c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x460a0: bne 0x46568 <entry+0x568>   @ imm = #0x4c0
    c.branch(C::Ne,0x46568)?;
    // 0x460a4: ldrb r1, [r10, #0x494]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1172)?)?;
    // 0x460a8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x460ac: beq 0x460bc <entry+0xbc>    @ imm = #0x8
    c.branch(C::Eq,0x460bc)?;
    // 0x460b0: ldr r0, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x460b4: b 0x46188 <entry+0x188>   @ imm = #0xcc
    c.branch(C::Al,0x46188)?;
    c.check(0x460b8)?;
    c.emit(CALLBACK_POINTER)?;
    c.check(0x460bc)?;
    // 0x460bc: ldr r1, [r10, #0x548]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1352)?)?;
    // 0x460c0: mov r2, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,120)?)?;
    // 0x460c4: ldr r6, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,644)?)?;
    // 0x460c8: mov r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,262144)?)?;
    // 0x460cc: and r1, r2, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // 0x460d0: ldr r2, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,400)?)?;
    // 0x460d4: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x460d8: mov r3, r2
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // 0x460dc: movne r3, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x460e0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x460e4: ldr r2, [r10, #0x54c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1356)?)?;
    // 0x460e8: moveq r3, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x460ec: cmp r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,8192)?)?;
    // 0x460f0: orr r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x460f4: mov r3, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,131072)?)?;
    // 0x460f8: moveq r6, #131072
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,131072)?)?;
    // 0x460fc: ldreq r3, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,48)?)?;
    // 0x46100: lsl r2, r2, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,6)?)?;
    // 0x46104: and r2, r2, #1920
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,1920)?)?;
    // 0x46108: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4610c: ldrb r2, [r10, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,972)?)?;
    // 0x46110: andeq r3, r6, r3, lsl #17
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,17,false)?)?;
    // 0x46114: ldr r6, [r0, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,172)?)?;
    // 0x46118: ldr r0, [r0, #0x244]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,580)?)?;
    // 0x4611c: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x46120: mov r3, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1048576)?)?;
    // 0x46124: and r2, r3, r2, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,20,false)?)?;
    // 0x46128: and r6, r5, r6, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,18,false)?)?;
    // 0x4612c: orr r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x46130: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x46134: mov r2, #8388608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8388608)?)?;
    // 0x46138: and r0, r2, r0, lsl #23
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,23,false)?)?;
    // 0x4613c: ldrb r2, [r10, #0xb7a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,2938)?)?;
    // 0x46140: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x46144: bic r1, r0, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R0,4278190080)?)?;
    // 0x46148: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4614c: orrne r0, r1, #-268435456
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R1,4026531840)?)?;
    // 0x46150: ldr r1, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,0)?)?;
    // 0x46154: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x46158: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4615c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x46160: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x46164: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x46168: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // 0x4616c: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x46170: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x46174: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x46178: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x4617c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46180: add r0, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,8)?)?;
    // 0x46184: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46188: mov r1, #604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,604)?)?;
    // 0x4618c: ldr r2, [r10, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2656)?)?;
    // 0x46190: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46194: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x46198: ldrh r1, [r10, r1]
    c.emit(load_halfword_register(C::Al,R::R1,R::new(10)?,R::R1)?)?;
    // 0x4619c: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x461a0: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x461a4: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x461a8: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x461ac: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x461b0: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // 0x461b4: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x461b8: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x461bc: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x461c0: add r1, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,1)?)?;
    // 0x461c4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x461c8: add r1, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,8)?)?;
    // 0x461cc: str r1, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x461d0: ldr r3, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,644)?)?;
    // 0x461d4: cmp r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8192)?)?;
    // 0x461d8: ldrb r3, [r10, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,988)?)?;
    // 0x461dc: movne r9, #4194304
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(9)?,R::R0,4194304)?)?;
    // 0x461e0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x461e4: movne r3, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x461e8: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x461ec: ldreq r7, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R1,0)?)?;
    // 0x461f0: lsl r6, r3, #19
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(3)?,Shift::Lsl,19)?)?;
    // 0x461f4: orr r3, r6, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x461f8: add r6, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,16)?)?;
    // 0x461fc: orr r3, r3, r9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x46200: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x46204: streq r7, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R1,0)?)?;
    // 0x46208: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x4620c: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x46210: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x46214: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x46218: str r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x4621c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x46220: str r6, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::LR,0)?)?;
    // 0x46224: ldrb r1, [r8, #0x3e9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1001)?)?;
    // 0x46228: ands r8, r1, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(8)?,R::R1,1,true)?)?;
    // 0x4622c: beq 0x4636c <entry+0x36c>   @ imm = #0x138
    c.branch(C::Eq,0x4636c)?;
    // 0x46230: ldr r4, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,2744)?)?;
    // 0x46234: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // 0x46238: mov lr, #253952
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,253952)?)?;
    // 0x4623c: add r2, r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3)?)?;
    // 0x46240: ldr r3, [r4, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,444)?)?;
    // 0x46244: ldr r5, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,452)?)?;
    // 0x46248: ldr r7, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,448)?)?;
    // 0x4624c: ldr r9, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,456)?)?;
    // 0x46250: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x46254: and r1, r1, r3, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,24,false)?)?;
    // 0x46258: and r3, lr, r7, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::LR,R::new(7)?,Shift::Lsl,13,false)?)?;
    // 0x4625c: ldr r7, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,472)?)?;
    // 0x46260: orr r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x46264: mov r3, #16252928
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16252928)?)?;
    // 0x46268: movne r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x4626c: and r3, r3, r9, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(9)?,Shift::Lsl,19,false)?)?;
    // 0x46270: orr r1, r1, r5, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,12,false)?)?;
    // 0x46274: mov r9, #3968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3968)?)?;
    // 0x46278: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4627c: ldr r3, [r4, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,460)?)?;
    // 0x46280: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x46284: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x46288: orr r1, r1, r3, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,18,false)?)?;
    // 0x4628c: ldr r3, [r4, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,464)?)?;
    // 0x46290: and r3, r9, r3, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(9)?,R::new(3)?,Shift::Lsl,7,false)?)?;
    // 0x46294: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x46298: ldr r3, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,468)?)?;
    // 0x4629c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x462a0: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x462a4: orr r1, r1, r3, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,6,false)?)?;
    // 0x462a8: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // 0x462ac: mov r10, #62
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,62)?)?;
    // 0x462b0: and r7, r10, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(10)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x462b4: orr r7, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x462b8: ldr r1, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,476)?)?;
    // 0x462bc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x462c0: orrne r7, r7, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(7)?,R::new(7)?,1)?)?;
    // 0x462c4: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x462c8: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x462cc: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x462d0: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x462d4: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x462d8: add r6, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,24)?)?;
    // 0x462dc: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x462e0: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x462e4: str r7, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,20)?)?;
    // 0x462e8: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x462ec: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x462f0: str r6, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,0)?)?;
    // 0x462f4: ldr r1, [r3, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,2744)?)?;
    // 0x462f8: ldr r4, [r1, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,480)?)?;
    // 0x462fc: ldr r7, [r1, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,484)?)?;
    // 0x46300: ldr r5, [r1, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,488)?)?;
    // 0x46304: ldr r3, [r1, #0x1ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,492)?)?;
    // 0x46308: and r4, r10, r4, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(10)?,R::new(4)?,Shift::Lsl,1,false)?)?;
    // 0x4630c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x46310: and r5, r9, r5, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(9)?,R::new(5)?,Shift::Lsl,7,false)?)?;
    // 0x46314: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // 0x46318: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x4631c: orr r4, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x46320: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x46324: mov r5, #16252928
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16252928)?)?;
    // 0x46328: orr r3, r4, r3, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Lsl,6,false)?)?;
    // 0x4632c: ldr r4, [r1, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,496)?)?;
    // 0x46330: and r7, lr, r4, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::LR,R::new(4)?,Shift::Lsl,13,false)?)?;
    // 0x46334: mov lr, r12
    c.emit(arm32::mov_register(C::Al,R::LR,R::R12))?;
    // 0x46338: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x4633c: ldr r7, [r1, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,500)?)?;
    // 0x46340: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x46344: movne r7, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x46348: orr r3, r3, r7, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,12,false)?)?;
    // 0x4634c: ldr r7, [r1, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,504)?)?;
    // 0x46350: ldr r1, [r1, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,508)?)?;
    // 0x46354: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x46358: and r7, r5, r7, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(5)?,R::new(7)?,Shift::Lsl,19,false)?)?;
    // 0x4635c: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x46360: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x46364: orr r3, r3, r1, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,18,false)?)?;
    // 0x46368: b 0x463a0 <entry+0x3a0>   @ imm = #0x30
    c.branch(C::Al,0x463a0)?;
    // 0x4636c: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x46370: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x46374: add r2, r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3)?)?;
    // 0x46378: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x4637c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46380: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x46384: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x46388: add r6, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,24)?)?;
    // 0x4638c: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x46390: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x46394: str r3, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,20)?)?;
    // 0x46398: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4639c: str r6, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::LR,0)?)?;
    // 0x463a0: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x463a4: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x463a8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x463ac: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x463b0: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x463b4: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x463b8: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x463bc: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x463c0: str r3, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,28)?)?;
    // 0x463c4: add r1, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,1)?)?;
    // 0x463c8: add r3, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,32)?)?;
    // 0x463cc: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x463d0: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::LR,0)?)?;
    // 0x463d4: ldr r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x463d8: beq 0x464fc <entry+0x4fc>   @ imm = #0x11c
    c.branch(C::Eq,0x464fc)?;
    // 0x463dc: ldr r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x463e0: mov r12, #62
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,62)?)?;
    // 0x463e4: mov lr, #3968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3968)?)?;
    // 0x463e8: mov r8, #253952
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,253952)?)?;
    // 0x463ec: mov r9, #16252928
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,16252928)?)?;
    // 0x463f0: ldr r6, [r1, #0x200]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,512)?)?;
    // 0x463f4: ldr r5, [r1, #0x204]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,516)?)?;
    // 0x463f8: ldr r4, [r1, #0x208]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,520)?)?;
    // 0x463fc: ldr r7, [r1, #0x20c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,524)?)?;
    // 0x46400: and r6, r12, r6, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R12,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x46404: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x46408: and r4, lr, r4, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::LR,R::new(4)?,Shift::Lsl,7,false)?)?;
    // 0x4640c: orrne r6, r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(6)?,R::new(6)?,1)?)?;
    // 0x46410: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x46414: orr r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x46418: ldr r6, [r1, #0x210]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,528)?)?;
    // 0x4641c: movne r7, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x46420: orr r4, r4, r7, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::new(7)?,Shift::Lsl,6,false)?)?;
    // 0x46424: ldr r7, [r1, #0x218]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,536)?)?;
    // 0x46428: and r6, r8, r6, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(8)?,R::new(6)?,Shift::Lsl,13,false)?)?;
    // 0x4642c: orr r4, r4, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x46430: ldr r6, [r1, #0x214]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,532)?)?;
    // 0x46434: ldr r1, [r1, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,540)?)?;
    // 0x46438: and r7, r9, r7, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(9)?,R::new(7)?,Shift::Lsl,19,false)?)?;
    // 0x4643c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x46440: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x46444: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x46448: orr r6, r4, r6, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,12,false)?)?;
    // 0x4644c: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x46450: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x46454: orr r6, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x46458: orr r1, r6, r1, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::R1,Shift::Lsl,18,false)?)?;
    // 0x4645c: ldreq r6, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(3)?,0)?)?;
    // 0x46460: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x46464: streq r6, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(3)?,0)?)?;
    // 0x46468: mov r3, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,28)?)?;
    // 0x4646c: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x46470: str r3, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,32)?)?;
    // 0x46474: str r1, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,36)?)?;
    // 0x46478: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4647c: add r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // 0x46480: ldr r5, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,104)?)?;
    // 0x46484: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x46488: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // 0x4648c: ldr r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x46490: add r10, r3, #544
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(3)?,544)?)?;
    // 0x46494: ldm r10, {r6, r7, r10}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::new(6)?.bit()|R::new(7)?.bit()|R::new(10)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x46498: ldr r4, [r3, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(3)?,556)?)?;
    // 0x4649c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x464a0: and r6, r12, r6, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R12,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x464a4: and r7, lr, r10, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::LR,R::new(10)?,Shift::Lsl,7,false)?)?;
    // 0x464a8: mov lr, r5
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(5)?))?;
    // 0x464ac: orrne r6, r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(6)?,R::new(6)?,1)?)?;
    // 0x464b0: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x464b4: orr r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x464b8: ldr r6, [r3, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,560)?)?;
    // 0x464bc: movne r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x464c0: orr r7, r7, r4, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(4)?,Shift::Lsl,6,false)?)?;
    // 0x464c4: and r6, r8, r6, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(8)?,R::new(6)?,Shift::Lsl,13,false)?)?;
    // 0x464c8: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x464cc: ldr r6, [r3, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,564)?)?;
    // 0x464d0: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x464d4: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x464d8: orr r7, r7, r6, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,12,false)?)?;
    // 0x464dc: ldr r6, [r3, #0x238]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,568)?)?;
    // 0x464e0: ldr r3, [r3, #0x23c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,572)?)?;
    // 0x464e4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x464e8: and r6, r9, r6, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(9)?,R::new(6)?,Shift::Lsl,19,false)?)?;
    // 0x464ec: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x464f0: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x464f4: orr r3, r7, r3, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,18,false)?)?;
    // 0x464f8: b 0x46530 <entry+0x530>   @ imm = #0x30
    c.branch(C::Al,0x46530)?;
    // 0x464fc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x46500: ldreq r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(3)?,0)?)?;
    // 0x46504: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46508: streq r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(3)?,0)?)?;
    // 0x4650c: mov r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // 0x46510: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x46514: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x46518: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x4651c: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x46520: add r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // 0x46524: str r3, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,36)?)?;
    // 0x46528: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4652c: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x46530: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x46534: ldr r8, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,80)?)?;
    // 0x46538: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x4653c: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x46540: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x46544: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x46548: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x4654c: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // 0x46550: str r2, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,40)?)?;
    // 0x46554: str r3, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,44)?)?;
    // 0x46558: add r0, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,48)?)?;
    // 0x4655c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x46560: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46564: ldr r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x46568: ldr r2, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,644)?)?;
    // 0x4656c: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x46570: beq 0x465a8 <entry+0x5a8>   @ imm = #0x30
    c.branch(C::Eq,0x465a8)?;
    // 0x46574: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x46578: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x4657c: orr r1, r1, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1048576)?)?;
    // 0x46580: beq 0x465c4 <entry+0x5c4>   @ imm = #0x3c
    c.branch(C::Eq,0x465c4)?;
    // 0x46584: mov r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1048576)?)?;
    // 0x46588: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x4658c: bne 0x465d0 <entry+0x5d0>   @ imm = #0x3c
    c.branch(C::Ne,0x465d0)?;
    // 0x46590: add r0, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,2)?)?;
    // 0x46594: ldrb r1, [r10, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,988)?)?;
    // 0x46598: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4659c: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x465a0: orr r0, r0, r1, lsl #23
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,23,false)?)?;
    // 0x465a4: b 0x465d0 <entry+0x5d0>   @ imm = #0x24
    c.branch(C::Al,0x465d0)?;
    // 0x465a8: ldr r1, [r10, #0x288]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,648)?)?;
    // 0x465ac: mov r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1048576)?)?;
    // 0x465b0: cmp r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,77)?)?;
    // 0x465b4: moveq r0, #9437184
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,9437184)?)?;
    // 0x465b8: cmp r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,100)?)?;
    // 0x465bc: moveq r0, #17825792
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,17825792)?)?;
    // 0x465c0: b 0x465d0 <entry+0x5d0>   @ imm = #0x8
    c.branch(C::Al,0x465d0)?;
    // 0x465c4: ldrb r0, [r10, #0x338]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,824)?)?;
    // 0x465c8: and r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,3)?)?;
    // 0x465cc: orr r0, r1, r0, lsl #23
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,23,false)?)?;
    // 0x465d0: ldr r1, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,400)?)?;
    // 0x465d4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x465d8: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x465dc: beq 0x465f0 <entry+0x5f0>   @ imm = #0xc
    c.branch(C::Eq,0x465f0)?;
    // 0x465e0: bic r0, r0, #240
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,240)?)?;
    // 0x465e4: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x465e8: orreq r0, r0, #192
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R0,R::R0,192)?)?;
    // 0x465ec: b 0x46608 <entry+0x608>   @ imm = #0x14
    c.branch(C::Al,0x46608)?;
    // 0x465f0: ldrb r1, [r8, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1005)?)?;
    // 0x465f4: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // 0x465f8: bic r0, r0, #240
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,240)?)?;
    // 0x465fc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x46600: moveq r2, #128
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x46604: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x46608: ldr r1, [pc, #0xf4c]        @ 0x155c <entry+0x155c>
    c.literal(C::Al,R::R1,0x4755c)?;
    // 0x4660c: ldrb r12, [r10, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(10)?,990)?)?;
    // 0x46610: ldr r3, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,664)?)?;
    // 0x46614: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x46618: ldr r7, [r10, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,940)?)?;
    // 0x4661c: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x46620: mov r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,255)?)?;
    // 0x46624: orr r8, r8, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,65280)?)?;
    // 0x46628: cmp r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,2)?)?;
    // 0x4662c: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x46630: and r0, r0, #192
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,192)?)?;
    // 0x46634: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x46638: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x4663c: orreq r1, r1, r2, lsl #30
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,30,false)?)?;
    // 0x46640: ldrb r2, [r10, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,988)?)?;
    // 0x46644: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x46648: str r2, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,96)?)?;
    // 0x4664c: movne r2, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x46650: orr r1, r1, r2, lsl #19
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,19,false)?)?;
    // 0x46654: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x46658: mov r2, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33554432)?)?;
    // 0x4665c: and r2, r2, r7, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,25,false)?)?;
    // 0x46660: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x46664: orr r2, r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,2147483648)?)?;
    // 0x46668: ldr r1, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,0)?)?;
    // 0x4666c: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x46670: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x46674: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x46678: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x4667c: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x46680: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x46684: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x46688: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x4668c: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,4)?)?;
    // 0x46690: ldr r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x46694: add r2, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,1)?)?;
    // 0x46698: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4669c: add r2, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,8)?)?;
    // 0x466a0: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x466a4: ldr r6, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2744)?)?;
    // 0x466a8: ldr r5, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,644)?)?;
    // 0x466ac: add r7, r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(6)?,24)?)?;
    // 0x466b0: addeq r7, r6, #32
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(7)?,R::new(6)?,32)?)?;
    // 0x466b4: cmp r5, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,16384)?)?;
    // 0x466b8: ldr r7, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,0)?)?;
    // 0x466bc: moveq r4, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,8)?)?;
    // 0x466c0: cmp r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,4096)?)?;
    // 0x466c4: moveq r4, #6
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,6)?)?;
    // 0x466c8: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x466cc: sub r7, r7, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x466d0: rsb r5, r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(5)?,R::new(7)?,0)?)?;
    // 0x466d4: and r5, r5, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x466d8: orr r7, r5, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(5)?,R::new(7)?,Shift::Lsl,16,false)?)?;
    // 0x466dc: add r5, r6, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,28)?)?;
    // 0x466e0: addeq r5, r6, #36
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(5)?,R::new(6)?,36)?)?;
    // 0x466e4: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x466e8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x466ec: ldreq r5, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R2,0)?)?;
    // 0x466f0: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x466f4: sub r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x466f8: streq r5, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R2,0)?)?;
    // 0x466fc: mov r2, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,52)?)?;
    // 0x46700: rsb r6, r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(6)?,R::R0,0)?)?;
    // 0x46704: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x46708: str r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x4670c: add r2, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,2)?)?;
    // 0x46710: str r7, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,12)?)?;
    // 0x46714: and r7, r6, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x46718: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4671c: add r2, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,16)?)?;
    // 0x46720: orr r0, r7, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x46724: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x46728: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x4672c: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x46730: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x46734: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x46738: mov r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // 0x4673c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x46740: str r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,16)?)?;
    // 0x46744: str r0, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,20)?)?;
    // 0x46748: add r0, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,3)?)?;
    // 0x4674c: mov r2, #2656
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2656)?)?;
    // 0x46750: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46754: add r0, r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,24)?)?;
    // 0x46758: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x4675c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46760: ldrh r2, [r10, r2]
    c.emit(load_halfword_register(C::Al,R::R2,R::new(10)?,R::R2)?)?;
    // 0x46764: ldr r7, [r10, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2652)?)?;
    // 0x46768: orr r2, r2, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,16,false)?)?;
    // 0x4676c: ldreq r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R0,0)?)?;
    // 0x46770: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x46774: streq r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R0,0)?)?;
    // 0x46778: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x4677c: add r7, r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(3)?,4)?)?;
    // 0x46780: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x46784: str r0, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,24)?)?;
    // 0x46788: add r0, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,32)?)?;
    // 0x4678c: str r2, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,28)?)?;
    // 0x46790: str r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-36)?)?;
    // 0x46794: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46798: ldr r2, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,644)?)?;
    // 0x4679c: ldr r4, [pc, #0xfa8]        @ 0x174c <entry+0x174c>
    c.literal(C::Al,R::new(4)?,0x4774c)?;
    // 0x467a0: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x467a4: beq 0x467e0 <entry+0x7e0>   @ imm = #0x34
    c.branch(C::Eq,0x467e0)?;
    // 0x467a8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x467ac: ldrb r2, [r10, #0x328]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,808)?)?;
    // 0x467b0: ldreq r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R0,0)?)?;
    // 0x467b4: lsl r2, r2, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,31)?)?;
    // 0x467b8: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x467bc: streq r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R0,0)?)?;
    // 0x467c0: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x467c4: add r7, r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(3)?,5)?)?;
    // 0x467c8: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x467cc: str r0, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,32)?)?;
    // 0x467d0: add r0, r1, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,40)?)?;
    // 0x467d4: str r2, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,36)?)?;
    // 0x467d8: str r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-36)?)?;
    // 0x467dc: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x467e0: ldrb r1, [r10, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,972)?)?;
    // 0x467e4: str r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,68)?)?;
    // 0x467e8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x467ec: str r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // 0x467f0: beq 0x46910 <entry+0x910>   @ imm = #0x118
    c.branch(C::Eq,0x46910)?;
    // 0x467f4: ldr r1, [r10, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,944)?)?;
    // 0x467f8: ldr r2, [r10, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,948)?)?;
    // 0x467fc: mov r3, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,254)?)?;
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();write_extended_features(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn feature_writes_preserve_inline_callback_pointer(){
        let v=fragment().unwrap();assert_eq!(v.len(),END-START);
        let off=0x460b8-START;
        assert_eq!(u32::from_le_bytes(v[off..off+4].try_into().unwrap()),CALLBACK_POINTER);
    }
}
