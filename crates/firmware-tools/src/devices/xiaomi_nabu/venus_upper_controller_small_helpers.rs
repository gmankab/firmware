// SPDX-License-Identifier: GPL-3.0-only
//! Five small source-authored A32 Venus controller helpers: descriptor
//! release, sample accumulator initialization, buffer layout preparation,
//! runtime statistics transfer and allocation result finalization. Each
//! routine and literal pool has its original exact boundaries. Data pools
//! are explicitly named device addresses, never imported OEM opcodes.
use arm32::{branch, data_immediate, data_immediate_with_flags, data_register,
    mov_register, mov_shifted_register, push_registers, transfer_byte,
    transfer_word, transfer_word_postindexed, transfer_word_register,
    transfer_doubleword, transfer_multiple,
    BlockMode,Condition as C,DataOp as Op,Register as R,Shift};

pub const START:usize=0x7389c;
pub const END:usize=0x73e10;
pub const FILE_BASE:usize=0x4000;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,expect:usize)->Result<(),String>{
        if self.at==expect{Ok(())}else{Err(format!("controller helper current 0x{:x}, expected 0x{expect:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("controller helper output address 0x{:x}",self.at));}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,target as u32,link)?)
    }
    fn literal(&mut self,dst:R,target:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,dst,R::PC,target as i32-self.at as i32-8)?)
    }
}
fn r(n:u8)->R{R::new(n).expect("A32 register")}

fn release_controller_record(c:&mut Code)->Result<(),String>{
    c.check(0x7389c)?;
    // 0x7389c: push {r4, r5, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x738a0: add r11, sp, #8
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,8)?)?;
    // 0x738a4: ldr r2, [pc, #0x6c]         @ 0x104c <entry+0x104c>
    c.literal(r(2),0x73918)?;
    // 0x738a8: mov r4, r0
    c.emit(mov_register(C::Al,r(4),r(0)))?;
    // 0x738ac: mov r1, #840
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,840)?)?;
    // 0x738b0: cmp r4, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(4),0)?)?;
    // 0x738b4: movne r0, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(0),R::R0,1)?)?;
    // 0x738b8: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x72110
    c.branch(C::Al,0x17b0,true)?;
    // 0x738bc: ldr r5, [r4, #0x38]
    c.emit(transfer_word(C::Al,true,r(5),r(4),56)?)?;
    // 0x738c0: cmp r5, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),0)?)?;
    // 0x738c4: popeq {r4, r5, r11, pc}
    c.emit(transfer_multiple(C::Eq,true,R::SP,r(4).bit()|r(5).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x738c8: ldr r0, [r4, #0x118]
    c.emit(transfer_word(C::Al,true,r(0),r(4),280)?)?;
    // 0x738cc: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x738d0: bne 0x738ec <entry+0x1020>  @ imm = #0x14
    c.branch(C::Ne,0x738ec,false)?;
    // 0x738d4: ldr r0, [r5]
    c.emit(transfer_word(C::Al,true,r(0),r(5),0)?)?;
    // 0x738d8: bl 0x2f1c <entry+0xfffffffffff90650> @ imm = #-0x709c4
    c.branch(C::Al,0x2f1c,true)?;
    // 0x738dc: ldr r0, [r5, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(5),8)?)?;
    // 0x738e0: bl 0x35bec <entry+0xfffffffffffc3320> @ imm = #-0x3dcfc
    c.branch(C::Al,0x35bec,true)?;
    // 0x738e4: ldr r0, [r5, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(5),8)?)?;
    // 0x738e8: bl 0x2f1c <entry+0xfffffffffff90650> @ imm = #-0x709d4
    c.branch(C::Al,0x2f1c,true)?;
    // 0x738ec: mov r0, r5
    c.emit(mov_register(C::Al,r(0),r(5)))?;
    // 0x738f0: bl 0x2f1c <entry+0xfffffffffff90650> @ imm = #-0x709dc
    c.branch(C::Al,0x2f1c,true)?;
    // 0x738f4: ldr r1, [pc, #0x20]         @ 0x1050 <entry+0x1050>
    c.literal(r(1),0x7391c)?;
    // 0x738f8: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x738fc: str r0, [r4, #0x38]
    c.emit(transfer_word(C::Al,false,r(0),r(4),56)?)?;
    // 0x73900: mov r0, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,4)?)?;
    // 0x73904: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44c78
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73908: ldr r1, [pc, #0x10]         @ 0x1054 <entry+0x1054>
    c.literal(r(1),0x73920)?;
    // 0x7390c: mov r0, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,4)?)?;
    // 0x73910: pop {r4, r5, r11, lr}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(11).bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x73914: b 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44c88
    c.branch(C::Al,0x2ec94,false)?;
    c.check(0x73918)?;
    const DESCRIPTOR_DIAGNOSTIC:u32=0x000f32d4; // data at 0x73918
    const RELEASE_DIAGNOSTIC:u32=0x000f330c; // data at 0x7391c
    const COMMON_LOG_FOOTER:u32=0x000e2788; // data at 0x73920
    for data in [DESCRIPTOR_DIAGNOSTIC,RELEASE_DIAGNOSTIC,COMMON_LOG_FOOTER]{c.emit(data)?;}
    c.check(0x73924)
}

fn reset_sample_accumulators(c:&mut Code)->Result<(),String>{
    c.check(0x73924)?;
    // 0x73924: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x73928: add r11, sp, #24
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,24)?)?;
    // 0x7392c: sub sp, sp, #80
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,R::SP,80)?)?;
    // 0x73930: mov r4, r0
    c.emit(mov_register(C::Al,r(4),r(0)))?;
    // 0x73934: ldr r7, [r4, #0x38]
    c.emit(transfer_word(C::Al,true,r(7),r(4),56)?)?;
    // 0x73938: ldr r0, [r7, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(7),8)?)?;
    // 0x7393c: bl 0x35d68 <entry+0xfffffffffffc349c> @ imm = #-0x3dbdc
    c.branch(C::Al,0x35d68,true)?;
    // 0x73940: mov r6, r0
    c.emit(mov_register(C::Al,r(6),r(0)))?;
    // 0x73944: cmp r6, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(6),0)?)?;
    // 0x73948: beq 0x73a10 <entry+0x1144>  @ imm = #0xc0
    c.branch(C::Eq,0x73a10,false)?;
    // 0x7394c: mov r5, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(5),R::R0,0)?)?;
    // 0x73950: mov r9, #76
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,76)?)?;
    // 0x73954: add r8, sp, #4
    c.emit(data_immediate(C::Al,Op::Add,r(8),R::SP,4)?)?;
    // 0x73958: ldr r0, [pc, #0xb8]         @ 0x114c <entry+0x114c>
    c.literal(r(0),0x73a18)?;
    // 0x7395c: str r0, [sp, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,8)?)?;
    // 0x73960: str r5, [sp, #0x18]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,24)?)?;
    // 0x73964: ldr r0, [r6, #0x68]
    c.emit(transfer_word(C::Al,true,r(0),r(6),104)?)?;
    // 0x73968: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x7396c: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x73970: ldrne r0, [r7, #0x4]
    c.emit(transfer_word(C::Ne,true,r(0),r(7),4)?)?;
    // 0x73974: ldrne r0, [r0, #0x3bc]
    c.emit(transfer_word(C::Ne,true,r(0),r(0),956)?)?;
    // 0x73978: str r0, [sp, #0x30]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,48)?)?;
    // 0x7397c: str r5, [sp, #0x34]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,52)?)?;
    // 0x73980: str r5, [sp, #0x2c]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,44)?)?;
    // 0x73984: ldr r0, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(0),r(4),20)?)?;
    // 0x73988: str r0, [sp, #0xc]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,12)?)?;
    // 0x7398c: ldr r1, [r6, #0x6c]
    c.emit(transfer_word(C::Al,true,r(1),r(6),108)?)?;
    // 0x73990: str r1, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,68)?)?;
    // 0x73994: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x73998: ldr r0, [r6, #0x70]
    c.emit(transfer_word(C::Al,true,r(0),r(6),112)?)?;
    // 0x7399c: str r0, [sp, #0x48]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,72)?)?;
    // 0x739a0: str r5, [sp, #0x10]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,16)?)?;
    // 0x739a4: str r5, [sp, #0x14]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,20)?)?;
    // 0x739a8: str r9, [sp, #0x4]
    c.emit(transfer_word(C::Al,false,r(9),R::SP,4)?)?;
    // 0x739ac: str r5, [sp, #0x1c]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,28)?)?;
    // 0x739b0: str r5, [sp, #0x38]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,56)?)?;
    // 0x739b4: ldr r0, [r6, #0x30]
    c.emit(transfer_word(C::Al,true,r(0),r(6),48)?)?;
    // 0x739b8: str r0, [sp, #0x3c]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,60)?)?;
    // 0x739bc: str r5, [sp, #0x24]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,36)?)?;
    // 0x739c0: str r5, [sp, #0x20]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,32)?)?;
    // 0x739c4: str r5, [sp, #0x40]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,64)?)?;
    // 0x739c8: beq 0x739e0 <entry+0x1114>  @ imm = #0x10
    c.branch(C::Eq,0x739e0,false)?;
    // 0x739cc: ldr r0, [r4, #0x7c]
    c.emit(transfer_word(C::Al,true,r(0),r(4),124)?)?;
    // 0x739d0: bl 0x7ec70 <entry+0xc3a4>  @ imm = #0xb298
    c.branch(C::Al,0x7ec70,true)?;
    // 0x739d4: ldr r0, [r4, #0x1c]
    c.emit(transfer_word(C::Al,true,r(0),r(4),28)?)?;
    // 0x739d8: sub r0, r0, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(0),r(0),1)?)?;
    // 0x739dc: str r0, [r4, #0x1c]
    c.emit(transfer_word(C::Al,false,r(0),r(4),28)?)?;
    // 0x739e0: ldr r0, [r4, #0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(4),32)?)?;
    // 0x739e4: mov r1, r8
    c.emit(mov_register(C::Al,r(1),r(8)))?;
    // 0x739e8: bl 0x2a878 <entry+0xfffffffffffb7fac> @ imm = #-0x49178
    c.branch(C::Al,0x2a878,true)?;
    // 0x739ec: mov r0, r6
    c.emit(mov_register(C::Al,r(0),r(6)))?;
    // 0x739f0: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x739f4: mov r2, #280
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,280)?)?;
    // 0x739f8: bl 0x31ec <entry+0xfffffffffff90920> @ imm = #-0x70814
    c.branch(C::Al,0x31ec,true)?;
    // 0x739fc: ldr r0, [r7, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(7),8)?)?;
    // 0x73a00: bl 0x35d68 <entry+0xfffffffffffc349c> @ imm = #-0x3dca0
    c.branch(C::Al,0x35d68,true)?;
    // 0x73a04: mov r6, r0
    c.emit(mov_register(C::Al,r(6),r(0)))?;
    // 0x73a08: cmp r6, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(6),0)?)?;
    // 0x73a0c: bne 0x73958 <entry+0x108c>  @ imm = #-0xbc
    c.branch(C::Ne,0x73958,false)?;
    // 0x73a10: sub sp, r11, #24
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,r(11),24)?)?;
    // 0x73a14: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x73a18)?;
    const PLATFORM_STATUS_WORD:u32=0x00221008; // data at 0x73a18
    for data in [PLATFORM_STATUS_WORD]{c.emit(data)?;}
    c.check(0x73a1c)
}

fn prepare_codec_buffer_descriptor(c:&mut Code)->Result<(),String>{
    c.check(0x73a1c)?;
    // 0x73a1c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x73a20: add r11, sp, #28
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,28)?)?;
    // 0x73a24: sub sp, sp, #100
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,R::SP,100)?)?;
    // 0x73a28: mov r5, r3
    c.emit(mov_register(C::Al,r(5),r(3)))?;
    // 0x73a2c: mov r4, r2
    c.emit(mov_register(C::Al,r(4),r(2)))?;
    // 0x73a30: mov r8, r1
    c.emit(mov_register(C::Al,r(8),r(1)))?;
    // 0x73a34: ldr r9, [r11, #0x8]
    c.emit(transfer_word(C::Al,true,r(9),r(11),8)?)?;
    // 0x73a38: mov r10, r0
    c.emit(mov_register(C::Al,r(10),r(0)))?;
    // 0x73a3c: ldr r2, [r5, #0x4]
    c.emit(transfer_word(C::Al,true,r(2),r(5),4)?)?;
    // 0x73a40: ldr r1, [r4, #0x144]
    c.emit(transfer_word(C::Al,true,r(1),r(4),324)?)?;
    // 0x73a44: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x73a48: beq 0x73a98 <entry+0x11cc>  @ imm = #0x48
    c.branch(C::Eq,0x73a98,false)?;
    // 0x73a4c: mov r0, #6144
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,6144)?)?;
    // 0x73a50: ldr r7, [r5, #0x284]
    c.emit(transfer_word(C::Al,true,r(7),r(5),644)?)?;
    // 0x73a54: mov r6, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,0)?)?;
    // 0x73a58: cmp r2, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),1)?)?;
    // 0x73a5c: add r1, r1, #31
    c.emit(data_immediate(C::Al,Op::Add,r(1),r(1),31)?)?;
    // 0x73a60: ldr r3, [r5, r0]
    c.emit(transfer_word_register(C::Al,true,r(3),r(5),r(0),Shift::Lsl,0)?)?;
    // 0x73a64: ldr r0, [r5, #0x960]
    c.emit(transfer_word(C::Al,true,r(0),r(5),2400)?)?;
    // 0x73a68: str r6, [sp]
    c.emit(transfer_word(C::Al,false,r(6),R::SP,0)?)?;
    // 0x73a6c: str r3, [sp, #0x4]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,4)?)?;
    // 0x73a70: mov r3, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,4)?)?;
    // 0x73a74: str r2, [sp, #0x8]
    c.emit(transfer_word(C::Al,false,r(2),R::SP,8)?)?;
    // 0x73a78: bic r2, r1, #31
    c.emit(data_immediate(C::Al,Op::Bic,r(2),r(1),31)?)?;
    // 0x73a7c: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x73a80: str r7, [sp, #0xc]
    c.emit(transfer_word(C::Al,false,r(7),R::SP,12)?)?;
    // 0x73a84: str r6, [sp, #0x10]
    c.emit(transfer_word(C::Al,false,r(6),R::SP,16)?)?;
    // 0x73a88: str r6, [sp, #0x14]
    c.emit(transfer_word(C::Al,false,r(6),R::SP,20)?)?;
    // 0x73a8c: moveq r3, #2
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,2)?)?;
    // 0x73a90: bl 0x51d08 <entry+0xfffffffffffdf43c> @ imm = #-0x21d90
    c.branch(C::Al,0x51d08,true)?;
    // 0x73a94: b 0x73ab8 <entry+0x11ec>  @ imm = #0x1c
    c.branch(C::Al,0x73ab8,false)?;
    // 0x73a98: ldr r1, [pc, #0x104]        @ 0x12d8 <entry+0x12d8>
    c.literal(r(1),0x73ba4)?;
    // 0x73a9c: ldr r2, [pc, #0x104]        @ 0x12dc <entry+0x12dc>
    c.literal(r(2),0x73ba8)?;
    // 0x73aa0: mov r0, #8
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,8)?)?;
    // 0x73aa4: mov r3, #84
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,84)?)?;
    // 0x73aa8: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44e1c
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73aac: ldr r1, [pc, #0xf8]         @ 0x12e0 <entry+0x12e0>
    c.literal(r(1),0x73bac)?;
    // 0x73ab0: mov r0, #8
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,8)?)?;
    // 0x73ab4: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44e28
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73ab8: ldr r0, [pc, #0xf0]         @ 0x12e4 <entry+0x12e4>
    c.literal(r(0),0x73bb0)?;
    // 0x73abc: mov r7, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(7),R::R0,0)?)?;
    // 0x73ac0: cmp r10, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(10),0)?)?;
    // 0x73ac4: str r0, [sp, #0x1c]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,28)?)?;
    // 0x73ac8: str r7, [sp, #0x2c]
    c.emit(transfer_word(C::Al,false,r(7),R::SP,44)?)?;
    // 0x73acc: ldr r0, [r5, #0x63c]
    c.emit(transfer_word(C::Al,true,r(0),r(5),1596)?)?;
    // 0x73ad0: str r0, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,68)?)?;
    // 0x73ad4: ldr r0, [r4, #0x144]
    c.emit(transfer_word(C::Al,true,r(0),r(4),324)?)?;
    // 0x73ad8: str r0, [sp, #0x48]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,72)?)?;
    // 0x73adc: str r7, [sp, #0x40]
    c.emit(transfer_word(C::Al,false,r(7),R::SP,64)?)?;
    // 0x73ae0: ldr r0, [r5, #0x14]
    c.emit(transfer_word(C::Al,true,r(0),r(5),20)?)?;
    // 0x73ae4: str r0, [sp, #0x20]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,32)?)?;
    // 0x73ae8: str r10, [sp, #0x58]
    c.emit(transfer_word(C::Al,false,r(10),R::SP,88)?)?;
    // 0x73aec: str r8, [sp, #0x5c]
    c.emit(transfer_word(C::Al,false,r(8),R::SP,92)?)?;
    // 0x73af0: ldrd r0, r1, [r4, #56]
    c.emit(transfer_doubleword(C::Al,true,r(0),r(4),56)?)?;
    // 0x73af4: str r1, [sp, #0x24]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,36)?)?;
    // 0x73af8: str r0, [sp, #0x28]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,40)?)?;
    // 0x73afc: mov r0, #76
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,76)?)?;
    // 0x73b00: str r0, [sp, #0x18]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,24)?)?;
    // 0x73b04: mov r0, #128
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,128)?)?;
    // 0x73b08: str r0, [sp, #0x30]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,48)?)?;
    // 0x73b0c: ldr r0, [r4, #0x2c]
    c.emit(transfer_word(C::Al,true,r(0),r(4),44)?)?;
    // 0x73b10: str r0, [sp, #0x4c]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,76)?)?;
    // 0x73b14: str r9, [sp, #0x50]
    c.emit(transfer_word(C::Al,false,r(9),R::SP,80)?)?;
    // 0x73b18: ldr r0, [r4, #0x28]
    c.emit(transfer_word(C::Al,true,r(0),r(4),40)?)?;
    // 0x73b1c: str r0, [sp, #0x38]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,56)?)?;
    // 0x73b20: ldr r0, [r4, #0x24]
    c.emit(transfer_word(C::Al,true,r(0),r(4),36)?)?;
    // 0x73b24: str r0, [sp, #0x34]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,52)?)?;
    // 0x73b28: str r7, [sp, #0x54]
    c.emit(transfer_word(C::Al,false,r(7),R::SP,84)?)?;
    // 0x73b2c: beq 0x73b48 <entry+0x127c>  @ imm = #0x14
    c.branch(C::Eq,0x73b48,false)?;
    // 0x73b30: ldr r0, [r5, #0x7c]
    c.emit(transfer_word(C::Al,true,r(0),r(5),124)?)?;
    // 0x73b34: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x73b38: bl 0x7ec70 <entry+0xc3a4>  @ imm = #0xb130
    c.branch(C::Al,0x7ec70,true)?;
    // 0x73b3c: ldr r0, [r5, #0x1c]
    c.emit(transfer_word(C::Al,true,r(0),r(5),28)?)?;
    // 0x73b40: sub r0, r0, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(0),r(0),1)?)?;
    // 0x73b44: str r0, [r5, #0x1c]
    c.emit(transfer_word(C::Al,false,r(0),r(5),28)?)?;
    // 0x73b48: ldr r0, [r5, #0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(5),32)?)?;
    // 0x73b4c: add r1, sp, #24
    c.emit(data_immediate(C::Al,Op::Add,r(1),R::SP,24)?)?;
    // 0x73b50: mov r2, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,1)?)?;
    // 0x73b54: bl 0x2a18c <entry+0xfffffffffffb78c0> @ imm = #-0x499d0
    c.branch(C::Al,0x2a18c,true)?;
    // 0x73b58: ldr r5, [r5, #0x14]
    c.emit(transfer_word(C::Al,true,r(5),r(5),20)?)?;
    // 0x73b5c: bl 0x26754 <entry+0xfffffffffffb3e88> @ imm = #-0x4d410
    c.branch(C::Al,0x26754,true)?;
    // 0x73b60: ldr r2, [sp, #0x58]
    c.emit(transfer_word(C::Al,true,r(2),R::SP,88)?)?;
    // 0x73b64: ldr r1, [sp, #0x48]
    c.emit(transfer_word(C::Al,true,r(1),R::SP,72)?)?;
    // 0x73b68: mov r3, r0
    c.emit(mov_register(C::Al,r(3),r(0)))?;
    // 0x73b6c: ldr r0, [sp, #0x30]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,48)?)?;
    // 0x73b70: str r2, [sp]
    c.emit(transfer_word(C::Al,false,r(2),R::SP,0)?)?;
    // 0x73b74: str r1, [sp, #0x4]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,4)?)?;
    // 0x73b78: ldr r1, [pc, #0x34]         @ 0x12e8 <entry+0x12e8>
    c.literal(r(1),0x73bb4)?;
    // 0x73b7c: str r0, [sp, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,8)?)?;
    // 0x73b80: mov r0, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,32)?)?;
    // 0x73b84: mov r2, r5
    c.emit(mov_register(C::Al,r(2),r(5)))?;
    // 0x73b88: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44efc
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73b8c: ldr r1, [pc, #0x18]         @ 0x12e0 <entry+0x12e0>
    c.literal(r(1),0x73bac)?;
    // 0x73b90: mov r0, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,32)?)?;
    // 0x73b94: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44f08
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73b98: str r7, [r4, #0x140]
    c.emit(transfer_word(C::Al,false,r(7),r(4),320)?)?;
    // 0x73b9c: sub sp, r11, #28
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,r(11),28)?)?;
    // 0x73ba0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x73ba4)?;
    const DESCRIPTOR_LAYOUT_DIAGNOSTIC:u32=0x000f3562; // data at 0x73ba4
    const SAMPLE_INPUT_DIAGNOSTIC:u32=0x000f3591; // data at 0x73ba8
    const PLATFORM_LOG_FOOTER:u32=0x000e2788; // data at 0x73bac
    const PLATFORM_STATUS_WORD:u32=0x00221008; // data at 0x73bb0
    const CODEC_OUTPUT_DIAGNOSTIC:u32=0x000f35a3; // data at 0x73bb4
    for data in [DESCRIPTOR_LAYOUT_DIAGNOSTIC,SAMPLE_INPUT_DIAGNOSTIC,PLATFORM_LOG_FOOTER,PLATFORM_STATUS_WORD,CODEC_OUTPUT_DIAGNOSTIC]{c.emit(data)?;}
    c.check(0x73bb8)
}

fn populate_codec_runtime_statistics(c:&mut Code)->Result<(),String>{
    c.check(0x73bb8)?;
    // 0x73bb8: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x73bbc: add r11, sp, #24
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,24)?)?;
    // 0x73bc0: mov r7, r0
    c.emit(mov_register(C::Al,r(7),r(0)))?;
    // 0x73bc4: mov r4, r2
    c.emit(mov_register(C::Al,r(4),r(2)))?;
    // 0x73bc8: mov r5, r1
    c.emit(mov_register(C::Al,r(5),r(1)))?;
    // 0x73bcc: ldr r6, [r7, #0x30]
    c.emit(transfer_word(C::Al,true,r(6),r(7),48)?)?;
    // 0x73bd0: ldr r0, [r6, #0xc]
    c.emit(transfer_word(C::Al,true,r(0),r(6),12)?)?;
    // 0x73bd4: ldr r8, [r6, #0x18]
    c.emit(transfer_word(C::Al,true,r(8),r(6),24)?)?;
    // 0x73bd8: bl 0x293c <entry+0xfffffffffff90070> @ imm = #-0x712a4
    c.branch(C::Al,0x293c,true)?;
    // 0x73bdc: ldr r0, [r6, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(6),8)?)?;
    // 0x73be0: bl 0x293c <entry+0xfffffffffff90070> @ imm = #-0x712ac
    c.branch(C::Al,0x293c,true)?;
    // 0x73be4: ldr r1, [r7, #0xab8]
    c.emit(transfer_word(C::Al,true,r(1),r(7),2744)?)?;
    // 0x73be8: add r0, r7, #640
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(7),640)?)?;
    // 0x73bec: ldr r1, [r1, #0x1a4]
    c.emit(transfer_word(C::Al,true,r(1),r(1),420)?)?;
    // 0x73bf0: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x73bf4: bne 0x73c34 <entry+0x1368>  @ imm = #0x38
    c.branch(C::Ne,0x73c34,false)?;
    // 0x73bf8: mov r2, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,0)?)?;
    // 0x73bfc: ldr r1, [r7, #0x638]
    c.emit(transfer_word(C::Al,true,r(1),r(7),1592)?)?;
    // 0x73c00: str r4, [r7, #0xb4c]
    c.emit(transfer_word(C::Al,false,r(4),r(7),2892)?)?;
    // 0x73c04: str r2, [r7, #0xb6c]
    c.emit(transfer_word(C::Al,false,r(2),r(7),2924)?)?;
    // 0x73c08: str r2, [r7, #0xb60]
    c.emit(transfer_word(C::Al,false,r(2),r(7),2912)?)?;
    // 0x73c0c: str r2, [r7, #0xb5c]
    c.emit(transfer_word(C::Al,false,r(2),r(7),2908)?)?;
    // 0x73c10: ldr r2, [r5, #0xec]
    c.emit(transfer_word(C::Al,true,r(2),r(5),236)?)?;
    // 0x73c14: str r2, [r7, #0xb50]
    c.emit(transfer_word(C::Al,false,r(2),r(7),2896)?)?;
    // 0x73c18: ldr r2, [r5, #0xf0]
    c.emit(transfer_word(C::Al,true,r(2),r(5),240)?)?;
    // 0x73c1c: str r2, [r7, #0xb54]
    c.emit(transfer_word(C::Al,false,r(2),r(7),2900)?)?;
    // 0x73c20: str r1, [r7, #0xb58]
    c.emit(transfer_word(C::Al,false,r(1),r(7),2904)?)?;
    // 0x73c24: ldr r1, [r5, #0x110]
    c.emit(transfer_word(C::Al,true,r(1),r(5),272)?)?;
    // 0x73c28: str r1, [r7, #0xb70]
    c.emit(transfer_word(C::Al,false,r(1),r(7),2928)?)?;
    // 0x73c2c: ldr r1, [r8, #0x12c]
    c.emit(transfer_word(C::Al,true,r(1),r(8),300)?)?;
    // 0x73c30: str r1, [r7, #0xb74]
    c.emit(transfer_word(C::Al,false,r(1),r(7),2932)?)?;
    // 0x73c34: mov r1, r5
    c.emit(mov_register(C::Al,r(1),r(5)))?;
    // 0x73c38: mov r2, r6
    c.emit(mov_register(C::Al,r(2),r(6)))?;
    // 0x73c3c: mov r3, r4
    c.emit(mov_register(C::Al,r(3),r(4)))?;
    // 0x73c40: pop {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(11).bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x73c44: b 0x504e0 <entry+0xfffffffffffddc14> @ imm = #-0x2376c
    c.branch(C::Al,0x504e0,false)?;
    c.check(0x73c48)?;
    c.check(0x73c48)
}

fn finalize_codec_runtime_allocation(c:&mut Code)->Result<(),String>{
    c.check(0x73c48)?;
    // 0x73c48: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x73c4c: add r11, sp, #28
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,28)?)?;
    // 0x73c50: sub sp, sp, #12
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x73c54: mov r4, r2
    c.emit(mov_register(C::Al,r(4),r(2)))?;
    // 0x73c58: ldr r2, [pc, #0x1a8]        @ 0x153c <entry+0x153c>
    c.literal(r(2),0x73e08)?;
    // 0x73c5c: mov r6, r0
    c.emit(mov_register(C::Al,r(6),r(0)))?;
    // 0x73c60: mov r9, r1
    c.emit(mov_register(C::Al,r(9),r(1)))?;
    // 0x73c64: mov r1, #187
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,187)?)?;
    // 0x73c68: mov r8, r3
    c.emit(mov_register(C::Al,r(8),r(3)))?;
    // 0x73c6c: cmp r6, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(6),0)?)?;
    // 0x73c70: orr r1, r1, #1280
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),1280)?)?;
    // 0x73c74: movne r0, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73c78: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x724d0
    c.branch(C::Al,0x17b0,true)?;
    // 0x73c7c: mov r0, #124
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,124)?)?;
    // 0x73c80: bl 0x2efc <entry+0xfffffffffff90630> @ imm = #-0x70d8c
    c.branch(C::Al,0x2efc,true)?;
    // 0x73c84: mov r10, r0
    c.emit(mov_register(C::Al,r(10),r(0)))?;
    // 0x73c88: cmp r10, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(10),0)?)?;
    // 0x73c8c: beq 0x73de4 <entry+0x1518>  @ imm = #0x150
    c.branch(C::Eq,0x73de4,false)?;
    // 0x73c90: mov r0, r10
    c.emit(mov_register(C::Al,r(0),r(10)))?;
    // 0x73c94: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x73c98: mov r2, #124
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,124)?)?;
    // 0x73c9c: mov r5, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(5),R::R0,0)?)?;
    // 0x73ca0: bl 0x31ec <entry+0xfffffffffff90920> @ imm = #-0x70abc
    c.branch(C::Al,0x31ec,true)?;
    // 0x73ca4: str r6, [r10, #0x10]
    c.emit(transfer_word(C::Al,false,r(6),r(10),16)?)?;
    // 0x73ca8: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73cac: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x73cb0: ldrb r1, [r6, #0x15f]
    c.emit(transfer_byte(C::Al,true,r(1),r(6),351)?)?;
    // 0x73cb4: str r5, [sp]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,0)?)?;
    // 0x73cb8: str r0, [sp, #0x4]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,4)?)?;
    // 0x73cbc: add r0, r10, #28
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(10),28)?)?;
    // 0x73cc0: add r2, r1, #1
    c.emit(data_immediate(C::Al,Op::Add,r(2),r(1),1)?)?;
    // 0x73cc4: mov r1, #416
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,416)?)?;
    // 0x73cc8: bl 0x35b30 <entry+0xfffffffffffc3264> @ imm = #-0x3e1a0
    c.branch(C::Al,0x35b30,true)?;
    // 0x73ccc: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x73cd0: beq 0x73ce4 <entry+0x1418>  @ imm = #0xc
    c.branch(C::Eq,0x73ce4,false)?;
    // 0x73cd4: ldr r2, [pc, #0x12c]        @ 0x153c <entry+0x153c>
    c.literal(r(2),0x73e08)?;
    // 0x73cd8: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x73cdc: mov r1, #1488
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,1488)?)?;
    // 0x73ce0: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x72538
    c.branch(C::Al,0x17b0,true)?;
    // 0x73ce4: ldrb r0, [r6, #0x15e]
    c.emit(transfer_byte(C::Al,true,r(0),r(6),350)?)?;
    // 0x73ce8: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x73cec: bne 0x73d3c <entry+0x1470>  @ imm = #0x48
    c.branch(C::Ne,0x73d3c,false)?;
    // 0x73cf0: ldrb r1, [r6, #0x15f]
    c.emit(transfer_byte(C::Al,true,r(1),r(6),351)?)?;
    // 0x73cf4: ldrb r2, [r6, #0x160]
    c.emit(transfer_byte(C::Al,true,r(2),r(6),352)?)?;
    // 0x73cf8: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x73cfc: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73d00: str r3, [sp]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,0)?)?;
    // 0x73d04: str r0, [sp, #0x4]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,4)?)?;
    // 0x73d08: add r0, r10, #64
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(10),64)?)?;
    // 0x73d0c: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x73d10: add r1, r1, r2
    c.emit(data_register(C::Al,Op::Add,r(1),r(1),r(2),Shift::Lsl,0,false)?)?;
    // 0x73d14: add r2, r1, #1
    c.emit(data_immediate(C::Al,Op::Add,r(2),r(1),1)?)?;
    // 0x73d18: mov r1, #416
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,416)?)?;
    // 0x73d1c: bl 0x35b30 <entry+0xfffffffffffc3264> @ imm = #-0x3e1f4
    c.branch(C::Al,0x35b30,true)?;
    // 0x73d20: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x73d24: beq 0x73d3c <entry+0x1470>  @ imm = #0x10
    c.branch(C::Eq,0x73d3c,false)?;
    // 0x73d28: ldr r2, [pc, #0xd8]         @ 0x153c <entry+0x153c>
    c.literal(r(2),0x73e08)?;
    // 0x73d2c: mov r1, #476
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,476)?)?;
    // 0x73d30: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x73d34: orr r1, r1, #1024
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),1024)?)?;
    // 0x73d38: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x72590
    c.branch(C::Al,0x17b0,true)?;
    // 0x73d3c: cmp r4, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(4),1)?)?;
    // 0x73d40: bne 0x73d68 <entry+0x149c>  @ imm = #0x20
    c.branch(C::Ne,0x73d68,false)?;
    // 0x73d44: mov r0, #8
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,8)?)?;
    // 0x73d48: bl 0x2efc <entry+0xfffffffffff90630> @ imm = #-0x70e54
    c.branch(C::Al,0x2efc,true)?;
    // 0x73d4c: ldr r2, [pc, #0xb4]         @ 0x153c <entry+0x153c>
    c.literal(r(2),0x73e08)?;
    // 0x73d50: mov r1, #484
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,484)?)?;
    // 0x73d54: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73d58: str r0, [r10, #0x4]
    c.emit(transfer_word(C::Al,false,r(0),r(10),4)?)?;
    // 0x73d5c: movne r0, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73d60: orr r1, r1, #1024
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),1024)?)?;
    // 0x73d64: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x725bc
    c.branch(C::Al,0x17b0,true)?;
    // 0x73d68: ldrb r0, [r6, #0x193]
    c.emit(transfer_byte(C::Al,true,r(0),r(6),403)?)?;
    // 0x73d6c: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73d70: beq 0x73dd4 <entry+0x1508>  @ imm = #0x5c
    c.branch(C::Eq,0x73dd4,false)?;
    // 0x73d74: ldr r0, [r6, #0x3a4]
    c.emit(transfer_word(C::Al,true,r(0),r(6),932)?)?;
    // 0x73d78: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73d7c: moveq r0, #1
    c.emit(data_immediate(C::Eq,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73d80: streq r0, [r6, #0x3a4]
    c.emit(transfer_word(C::Eq,false,r(0),r(6),932)?)?;
    // 0x73d84: ldrb r1, [r6, #0x15f]
    c.emit(transfer_byte(C::Al,true,r(1),r(6),351)?)?;
    // 0x73d88: ldrb r2, [r6, #0x160]
    c.emit(transfer_byte(C::Al,true,r(2),r(6),352)?)?;
    // 0x73d8c: add r1, r1, r2
    c.emit(data_register(C::Al,Op::Add,r(1),r(1),r(2),Shift::Lsl,0,false)?)?;
    // 0x73d90: add r1, r1, #1
    c.emit(data_immediate(C::Al,Op::Add,r(1),r(1),1)?)?;
    // 0x73d94: ands r4, r1, #255
    c.emit(data_immediate_with_flags(C::Al,Op::And,r(4),r(1),255,true)?)?;
    // 0x73d98: beq 0x73dd4 <entry+0x1508>  @ imm = #0x34
    c.branch(C::Eq,0x73dd4,false)?;
    // 0x73d9c: lsl r0, r0, #2
    c.emit(mov_shifted_register(C::Al,r(0),r(0),Shift::Lsl,2)?)?;
    // 0x73da0: bl 0x2efc <entry+0xfffffffffff90630> @ imm = #-0x70eac
    c.branch(C::Al,0x2efc,true)?;
    // 0x73da4: cmp r4, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(4),1)?)?;
    // 0x73da8: str r0, [r10, #0x68]
    c.emit(transfer_word(C::Al,false,r(0),r(10),104)?)?;
    // 0x73dac: beq 0x73dd4 <entry+0x1508>  @ imm = #0x20
    c.branch(C::Eq,0x73dd4,false)?;
    // 0x73db0: add r5, r10, #108
    c.emit(data_immediate(C::Al,Op::Add,r(5),r(10),108)?)?;
    // 0x73db4: mov r7, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(7),R::R0,1)?)?;
    // 0x73db8: ldr r0, [r6, #0x3a4]
    c.emit(transfer_word(C::Al,true,r(0),r(6),932)?)?;
    // 0x73dbc: lsl r0, r0, #2
    c.emit(mov_shifted_register(C::Al,r(0),r(0),Shift::Lsl,2)?)?;
    // 0x73dc0: bl 0x2efc <entry+0xfffffffffff90630> @ imm = #-0x70ecc
    c.branch(C::Al,0x2efc,true)?;
    // 0x73dc4: add r7, r7, #1
    c.emit(data_immediate(C::Al,Op::Add,r(7),r(7),1)?)?;
    // 0x73dc8: str r0, [r5], #4
    c.emit(transfer_word_postindexed(C::Al,false,r(0),r(5),4)?)?;
    // 0x73dcc: cmp r7, r4
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(7),r(4),Shift::Lsl,0,false)?)?;
    // 0x73dd0: bne 0x73db8 <entry+0x14ec>  @ imm = #-0x20
    c.branch(C::Ne,0x73db8,false)?;
    // 0x73dd4: ldr r4, [pc, #0x30]         @ 0x1540 <entry+0x1540>
    c.literal(r(4),0x73e0c)?;
    // 0x73dd8: str r10, [r9]
    c.emit(transfer_word(C::Al,false,r(10),r(9),0)?)?;
    // 0x73ddc: str r8, [r10, #0x78]
    c.emit(transfer_word(C::Al,false,r(8),r(10),120)?)?;
    // 0x73de0: b 0x73dfc <entry+0x1530>  @ imm = #0x14
    c.branch(C::Al,0x73dfc,false)?;
    // 0x73de4: ldr r2, [pc, #0x1c]         @ 0x153c <entry+0x153c>
    c.literal(r(2),0x73e08)?;
    // 0x73de8: mov r1, #193
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,193)?)?;
    // 0x73dec: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x73df0: mov r4, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(4),R::R0,0)?)?;
    // 0x73df4: orr r1, r1, #1280
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),1280)?)?;
    // 0x73df8: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x72650
    c.branch(C::Al,0x17b0,true)?;
    // 0x73dfc: mov r0, r4
    c.emit(mov_register(C::Al,r(0),r(4)))?;
    // 0x73e00: sub sp, r11, #28
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,r(11),28)?)?;
    // 0x73e04: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x73e08)?;
    const CODEC_ALLOCATOR_DIAGNOSTIC:u32=0x000f35fa; // data at 0x73e08
    const CONTROLLER_NEXT_ROUTINE:u32=0x00073e10; // data at 0x73e0c
    for data in [CODEC_ALLOCATOR_DIAGNOSTIC,CONTROLLER_NEXT_ROUTINE]{c.emit(data)?;}
    c.check(0x73e10)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();
    release_controller_record(&mut c)?;
    reset_sample_accumulators(&mut c)?;
    prepare_codec_buffer_descriptor(&mut c)?;
    populate_codec_runtime_statistics(&mut c)?;
    finalize_codec_runtime_allocation(&mut c)?;
    Ok(c.bytes)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn helper_function_boundaries_are_contiguous(){
        assert_eq!(fragment().unwrap().len(),END-START);
    }
}
