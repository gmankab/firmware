// SPDX-License-Identifier: GPL-3.0-only
//! Architecture-level reconstruction of Venus codec format/control register
//! writer. This independent function packs subtype, bit-depth, channel and
//! endian flags into an ARM32 command list. No reference opcode bytes are
//! stored. Two register-indexed signed-byte loads use a validated local A32
//! encoder not yet present in the shared minimal `arm32` crate.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};

pub const START:usize=0x412b8;
pub const INST_END:usize=0x416b0;
pub const END:usize=0x416c0;
pub const FILE_BASE:usize=0x4000;

/// Non-executable words addressed by PC-relative instructions in this code.
const FORMAT_DATA:[u32;4]=[0x3c0f_0002,0x007f_ffff,0x000e_e028,0x0003_fffd];

/// ARM A32 register-indexed LDRSB encoding: sign-extends the addressed byte
/// into Rt with pre-indexed positive offset `Rm` (no base writeback).
/// Verified against Clang/LLVM assembly of `ldrsb r1,[r0,r1]` and
/// `ldrsb r6,[r3,r6]`. This is not proprietary OEM instruction data.
pub(super) fn signed_byte_register(cond:C,dst:R,base:R,index:R)->Result<u32,String>{
    if dst==R::PC || base==R::PC || index==R::PC {
        return Err("LDRSB register mode cannot use PC".into());
    }
    Ok(((cond as u32)<<28) | 0x0190_00d0 |
       (base.number()<<16) | (dst.number()<<12) | index.number())
}

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at!=addr{return Err(format!("format register position 0x{:x} expected 0x{addr:x}",self.at));}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<0x40000 || self.at>=0x68000 || self.at%4!=0{
            return Err(format!("outside exclusive Venus middle region at 0x{:x}",self.at));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(cond,self.at as u32,target as u32,false)?)
    }
    fn call(&mut self,target:usize)->Result<(),String>{
        self.emit(arm32::branch(C::Al,self.at as u32,target as u32,true)?)
    }
    fn literal(&mut self,rd:R,target:usize)->Result<(),String>{
        let signed=i32::try_from(target as isize-self.at as isize-8)
           .map_err(|_| "A32 literal offset exceeds i32")?;
        self.emit(arm32::transfer_word(C::Al,true,rd,R::PC,signed)?)
    }
}

/// Inspect codec profile, select hardware layout and build flags/cmd packets.
fn write_format_commands(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x412b8: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x412bc: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x412c0: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x412c4: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x412c8: mov r3, #983040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,983040)?)?;
    // 0x412cc: ldr r9, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(5)?,44)?)?;
    // 0x412d0: ldr r1, [r9, #0x368]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,872)?)?;
    // 0x412d4: ldr r2, [r9, #0x36c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,876)?)?;
    // 0x412d8: ldr r0, [r9, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,400)?)?;
    // 0x412dc: and r1, r3, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,16,false)?)?;
    // 0x412e0: mov r3, #1006632960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1006632960)?)?;
    // 0x412e4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x412e8: and r2, r3, r2, lsl #26
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,26,false)?)?;
    // 0x412ec: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x412f0: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x412f4: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x412f8: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x412fc: ldr r2, [pc, #0x3ac]
    c.literal(R::R2,0x416b0)?;
    // 0x41300: and r7, r1, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41304: ldr r2, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2744)?)?;
    // 0x41308: ldrb r3, [r2, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,12)?)?;
    // 0x4130c: ldrb r4, [r2, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R2,204)?)?;
    // 0x41310: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x41314: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x41318: moveq r3, r1
    c.emit(arm32::mov_register(C::Eq,R::new(3)?,R::R1))?;
    // 0x4131c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x41320: moveq r3, r1
    c.emit(arm32::mov_register(C::Eq,R::new(3)?,R::R1))?;
    // 0x41324: ldr r1, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,644)?)?;
    // 0x41328: ldr r6, [pc, #0x384]
    c.literal(R::new(6)?,0x416b4)?;
    // 0x4132c: cmp r1, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x41330: movne r7, r3
    c.emit(arm32::mov_register(C::Ne,R::new(7)?,R::new(3)?))?;
    // 0x41334: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x41338: movne r7, r3
    c.emit(arm32::mov_register(C::Ne,R::new(7)?,R::new(3)?))?;
    // 0x4133c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x41340: bne 0x4135c <entry+0x145c>
    c.branch(C::Ne,0x4135c)?;
    // 0x41344: ldrb r2, [r2, #0xe8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,232)?)?;
    // 0x41348: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // 0x4134c: beq 0x4135c <entry+0x145c>
    c.branch(C::Eq,0x4135c)?;
    // 0x41350: ldrb r2, [r5, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,1173)?)?;
    // 0x41354: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x41358: beq 0x4137c <entry+0x147c>
    c.branch(C::Eq,0x4137c)?;
    // 0x4135c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x41360: ldrb r1, [r5, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,972)?)?;
    // 0x41364: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x41368: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4136c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x41370: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x41374: and r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x41378: orr r7, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4137c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x41380: beq 0x4139c <entry+0x149c>
    c.branch(C::Eq,0x4139c)?;
    // 0x41384: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x41388: beq 0x413a4 <entry+0x14a4>
    c.branch(C::Eq,0x413a4)?;
    // 0x4138c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x41390: bne 0x413c4 <entry+0x14c4>
    c.branch(C::Ne,0x413c4)?;
    // 0x41394: bic r7, r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,12)?)?;
    // 0x41398: b 0x413ec <entry+0x14ec>
    c.branch(C::Al,0x413ec)?;
    // 0x4139c: orr r7, r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,12)?)?;
    // 0x413a0: b 0x413ec <entry+0x14ec>
    c.branch(C::Al,0x413ec)?;
    // 0x413a4: ldrb r0, [r9, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1005)?)?;
    // 0x413a8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x413ac: beq 0x413cc <entry+0x14cc>
    c.branch(C::Eq,0x413cc)?;
    // 0x413b0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x413b4: bne 0x413d8 <entry+0x14d8>
    c.branch(C::Ne,0x413d8)?;
    // 0x413b8: bic r0, r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(7)?,12)?)?;
    // 0x413bc: orr r7, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,4)?)?;
    // 0x413c0: b 0x413ec <entry+0x14ec>
    c.branch(C::Al,0x413ec)?;
    // 0x413c4: mov r1, #212
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,212)?)?;
    // 0x413c8: b 0x413dc <entry+0x14dc>
    c.branch(C::Al,0x413dc)?;
    // 0x413cc: bic r0, r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(7)?,12)?)?;
    // 0x413d0: orr r7, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,8)?)?;
    // 0x413d4: b 0x413ec <entry+0x14ec>
    c.branch(C::Al,0x413ec)?;
    // 0x413d8: mov r1, #206
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,206)?)?;
    // 0x413dc: ldr r2, [pc, #0x2d4]
    c.literal(R::R2,0x416b8)?;
    // 0x413e0: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x413e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x413e8: bl 0x17b0 <entry+0xfffffffffffc18b0>
    c.call(0x17b0)?;
    // 0x413ec: ldr r0, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2744)?)?;
    // 0x413f0: mov r1, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,153)?)?;
    // 0x413f4: mov r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // 0x413f8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x413fc: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x41400: ldrsb r1, [r0, r1]
    c.emit(signed_byte_register(C::Al,R::R1,R::R0,R::R1)?)?;
    // 0x41404: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x41408: ldrb r1, [r0, #0x4b5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,1205)?)?;
    // 0x4140c: movgt r2, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x41410: and r1, r3, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,8,false)?)?;
    // 0x41414: bic r3, r7, #33024
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(7)?,33024)?)?;
    // 0x41418: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4141c: orr r1, r1, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // 0x41420: ldrb r2, [r0, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,204)?)?;
    // 0x41424: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x41428: bne 0x41444 <entry+0x1544>
    c.branch(C::Ne,0x41444)?;
    // 0x4142c: ldrb r2, [r0, #0xe8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,232)?)?;
    // 0x41430: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // 0x41434: beq 0x41444 <entry+0x1544>
    c.branch(C::Eq,0x41444)?;
    // 0x41438: ldrb r2, [r5, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,1173)?)?;
    // 0x4143c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x41440: beq 0x4145c <entry+0x155c>
    c.branch(C::Eq,0x4145c)?;
    // 0x41444: ldrb r2, [r5, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,972)?)?;
    // 0x41448: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4144c: beq 0x4145c <entry+0x155c>
    c.branch(C::Eq,0x4145c)?;
    // 0x41450: ldr r2, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,644)?)?;
    // 0x41454: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x41458: orreq r1, r1, #16777216
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R1,R::R1,16777216)?)?;
    // 0x4145c: ldrb r2, [r5, #0xb78]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2936)?)?;
    // 0x41460: ldrb r0, [r0, #0x528]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1320)?)?;
    // 0x41464: bic r3, r1, #58720256
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::R1,58720256)?)?;
    // 0x41468: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4146c: and r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,1)?)?;
    // 0x41470: orrne r1, r3, #8388608
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R1,R::new(3)?,8388608)?)?;
    // 0x41474: bic r1, r1, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,33554432)?)?;
    // 0x41478: orr r0, r1, r0, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,25,false)?)?;
    // 0x4147c: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x41480: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x41484: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x41488: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4148c: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x41490: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x41494: orr r2, r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x41498: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x4149c: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x414a0: add r0, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,8)?)?;
    // 0x414a4: mov r2, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1048576)?)?;
    // 0x414a8: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x414ac: ldr r1, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2744)?)?;
    // 0x414b0: ldr r1, [r1, #0x410]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1040)?)?;
    // 0x414b4: rsb r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R1,1)?)?;
    // 0x414b8: and r7, r2, r1, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R2,R::R1,Shift::Lsl,20,false)?)?;
    // 0x414bc: ldr r1, [r9, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,400)?)?;
    // 0x414c0: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x414c4: beq 0x414e4 <entry+0x15e4>
    c.branch(C::Eq,0x414e4)?;
    // 0x414c8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x414cc: bne 0x41520 <entry+0x1620>
    c.branch(C::Ne,0x41520)?;
    // 0x414d0: ldrb r1, [r9, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,1005)?)?;
    // 0x414d4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x414d8: beq 0x41520 <entry+0x1620>
    c.branch(C::Eq,0x41520)?;
    // 0x414dc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x414e0: bne 0x41508 <entry+0x1608>
    c.branch(C::Ne,0x41508)?;
    // 0x414e4: ldr r1, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,644)?)?;
    // 0x414e8: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x414ec: cmpne r1, #16384
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,16384)?)?;
    // 0x414f0: bne 0x41500 <entry+0x1600>
    c.branch(C::Ne,0x41500)?;
    // 0x414f4: orr r1, r7, #544
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(7)?,544)?)?;
    // 0x414f8: orr r7, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R1,8192)?)?;
    // 0x414fc: b 0x41520 <entry+0x1620>
    c.branch(C::Al,0x41520)?;
    // 0x41500: orr r7, r7, #544
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,544)?)?;
    // 0x41504: b 0x41520 <entry+0x1620>
    c.branch(C::Al,0x41520)?;
    // 0x41508: ldr r2, [pc, #0x1a8]
    c.literal(R::R2,0x416b8)?;
    // 0x4150c: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x41510: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x41514: orr r1, r1, #6400
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6400)?)?;
    // 0x41518: bl 0x17b0 <entry+0xfffffffffffc18b0>
    c.call(0x17b0)?;
    // 0x4151c: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x41520: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x41524: mov r2, #31457280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31457280)?)?;
    // 0x41528: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4152c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x41530: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x41534: mov r1, #6208
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6208)?)?;
    // 0x41538: stm r0, {r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4153c: add r1, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,8)?)?;
    // 0x41540: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x41544: ldr r3, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,2744)?)?;
    // 0x41548: ldr r7, [r3, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,924)?)?;
    // 0x4154c: and r2, r2, r7, lsl #21
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,21,false)?)?;
    // 0x41550: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // 0x41554: ldr r7, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,644)?)?;
    // 0x41558: movhi r2, #16777216
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R2,R::R0,16777216)?)?;
    // 0x4155c: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x41560: bge 0x415a4 <entry+0x16a4>
    c.branch(C::Ge,0x415a4)?;
    // 0x41564: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x41568: beq 0x41628 <entry+0x1728>
    c.branch(C::Eq,0x41628)?;
    // 0x4156c: cmp r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x41570: bne 0x41664 <entry+0x1764>
    c.branch(C::Ne,0x41664)?;
    // 0x41574: ldrb r7, [r3, #0x4b1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(3)?,1201)?)?;
    // 0x41578: ldrb r6, [r3, #0x4b2]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,1202)?)?;
    // 0x4157c: ldrb r3, [r3, #0x4b3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(3)?,1203)?)?;
    // 0x41580: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x41584: and r3, r5, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x41588: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x4158c: and r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x41590: and r6, r5, r6, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x41594: orr r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41598: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x4159c: orr r2, r2, r3, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,18,false)?)?;
    // 0x415a0: b 0x41680 <entry+0x1780>
    c.branch(C::Al,0x41680)?;
    // 0x415a4: ldr r6, [pc, #0x108]
    c.literal(R::new(6)?,0x416b4)?;
    // 0x415a8: cmp r7, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x415ac: beq 0x41620 <entry+0x1720>
    c.branch(C::Eq,0x41620)?;
    // 0x415b0: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x415b4: bne 0x41664 <entry+0x1764>
    c.branch(C::Ne,0x41664)?;
    // 0x415b8: ldr r5, [r9, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,400)?)?;
    // 0x415bc: ldr r7, [pc, #0xf8]
    c.literal(R::new(7)?,0x416bc)?;
    // 0x415c0: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x415c4: bne 0x415dc <entry+0x16dc>
    c.branch(C::Ne,0x415dc)?;
    // 0x415c8: ldrb r6, [r9, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(9)?,1005)?)?;
    // 0x415cc: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x415d0: ldreq r6, [r3, #0x3a0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(3)?,928)?)?;
    // 0x415d4: andeq r6, r6, r7
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x415d8: orreq r2, r6, r2
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x415dc: mov r6, #284
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,284)?)?;
    // 0x415e0: orr r6, r6, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1024)?)?;
    // 0x415e4: ldrsb r6, [r3, r6]
    c.emit(signed_byte_register(C::Al,R::new(6)?,R::new(3)?,R::new(6)?)?)?;
    // 0x415e8: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x415ec: bgt 0x41600 <entry+0x1700>
    c.branch(C::Gt,0x41600)?;
    // 0x415f0: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // 0x415f4: ldrbeq r6, [r3, #0xe0]
    c.emit(arm32::transfer_byte(C::Eq,true,R::new(6)?,R::new(3)?,224)?)?;
    // 0x415f8: cmpeq r6, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x415fc: bne 0x4166c <entry+0x176c>
    c.branch(C::Ne,0x4166c)?;
    // 0x41600: mov r6, #66846720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,66846720)?)?;
    // 0x41604: add r7, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // 0x41608: orr r6, r6, #-67108864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4227858432)?)?;
    // 0x4160c: and r2, r2, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x41610: ldr r6, [r3, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,928)?)?;
    // 0x41614: and r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x41618: orr r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4161c: b 0x4166c <entry+0x176c>
    c.branch(C::Al,0x4166c)?;
    // 0x41620: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x41624: strb r6, [r3, #0x4b1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(3)?,1201)?)?;
    // 0x41628: ldr r6, [r9, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,400)?)?;
    // 0x4162c: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x41630: beq 0x41644 <entry+0x1744>
    c.branch(C::Eq,0x41644)?;
    // 0x41634: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x41638: bne 0x41664 <entry+0x1764>
    c.branch(C::Ne,0x41664)?;
    // 0x4163c: ldrb r6, [r3, #0x4af]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,1199)?)?;
    // 0x41640: b 0x41648 <entry+0x1748>
    c.branch(C::Al,0x41648)?;
    // 0x41644: ldrb r6, [r3, #0x4b0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,1200)?)?;
    // 0x41648: ldrb r5, [r3, #0x4b1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(3)?,1201)?)?;
    // 0x4164c: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x41650: and r6, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x41654: mov r4, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,2)?)?;
    // 0x41658: and r5, r4, r5, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,1,false)?)?;
    // 0x4165c: orr r6, r5, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x41660: orr r2, r2, r6, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(6)?,Shift::Lsl,18,false)?)?;
    // 0x41664: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x41668: bne 0x41680 <entry+0x1780>
    c.branch(C::Ne,0x41680)?;
    // 0x4166c: ldr r3, [r3, #0x3a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,932)?)?;
    // 0x41670: mov r7, #234881024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,234881024)?)?;
    // 0x41674: bic r2, r2, #234881024
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,234881024)?)?;
    // 0x41678: and r3, r7, r3, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,25,false)?)?;
    // 0x4167c: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41680: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x41684: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x41688: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4168c: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x41690: mov r1, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,68)?)?;
    // 0x41694: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x41698: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x4169c: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x416a0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x416a4: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x416a8: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x416ac: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(INST_END)?;
    for constant in FORMAT_DATA {c.emit(constant)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();write_format_commands(&mut c)?;Ok(c.bytes)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]
    fn format_flags_have_correct_length_and_data_pool(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        assert_eq!(INST_END-START,254*4);
        for (i,value) in FORMAT_DATA.iter().enumerate(){
            let at=(INST_END-START)+4*i;
            assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),*value);
        }
    }
    #[test]
    fn registered_byte_load_syntax_is_checked(){
        let r3=R::new(3).unwrap();let r6=R::new(6).unwrap();
        assert_eq!(signed_byte_register(C::Al,R::R1,R::R0,R::R1).unwrap(),0xe190_10d1);
        assert_eq!(signed_byte_register(C::Al,r6,r3,r6).unwrap(),0xe193_60d6);
        assert!(signed_byte_register(C::Eq,R::PC,R::R0,R::R1).is_err());
    }
}
