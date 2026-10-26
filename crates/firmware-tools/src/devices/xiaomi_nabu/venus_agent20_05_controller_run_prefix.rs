// SPDX-License-Identifier: GPL-3.0-only
//! First section of the Venus controller session-run state machine.
//! Identifies the profile, dispatches the configured codec runtime and
//! writes the initial packet/ring state. Branch successors beyond 0x73000
//! are intentionally unresolved: this is NOT a standalone callable function.
use arm32::{branch, data_immediate, data_register, mov_register,
    mov_shifted_register, push_registers, transfer_byte,
    transfer_word, transfer_multiple, transfer_doubleword,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};
pub const START:usize=0x72d14;
pub const END:usize=0x73000;
pub const FILE_BASE:usize=0x4000;
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,expected:usize)->Result<(),String>{
        if self.at==expected{Ok(())}else{Err(format!("Venus controller runner 0x{:x} expected 0x{expected:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("invalid controller runner output at 0x{:x}",self.at));}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,target as u32,link)?)
    }
    fn literal(&mut self,dst:R,target:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,dst,R::PC,target as i32-self.at as i32-8)?)
    }
}
fn r(x:u8)->R{R::new(x).expect("A32 register")}
fn controller_run_prefix(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x72d14: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x72d18: add r11, sp, #28
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,28)?)?;
    // 0x72d1c: sub sp, sp, #252
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,R::SP,252)?)?;
    // 0x72d20: mov r4, r0
    c.emit(mov_register(C::Al,r(4),r(0)))?;
    // 0x72d24: ldr r1, [r4, #0x284]
    c.emit(transfer_word(C::Al,true,r(1),r(4),644)?)?;
    // 0x72d28: ldr r5, [r4, #0x38]
    c.emit(transfer_word(C::Al,true,r(5),r(4),56)?)?;
    // 0x72d2c: cmp r1, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),8192)?)?;
    // 0x72d30: bge 0x72d4c <entry+0x480>   @ imm = #0x14
    c.branch(C::Ge,0x72d4c,false)?;
    // 0x72d34: cmp r1, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),2)?)?;
    // 0x72d38: beq 0x72d60 <entry+0x494>   @ imm = #0x20
    c.branch(C::Eq,0x72d60,false)?;
    // 0x72d3c: cmp r1, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),4096)?)?;
    // 0x72d40: bne 0x72d70 <entry+0x4a4>   @ imm = #0x28
    c.branch(C::Ne,0x72d70,false)?;
    // 0x72d44: mov r0, #13
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,13)?)?;
    // 0x72d48: b 0x72d7c <entry+0x4b0>   @ imm = #0x2c
    c.branch(C::Al,0x72d7c,false)?;
    // 0x72d4c: beq 0x72d68 <entry+0x49c>   @ imm = #0x14
    c.branch(C::Eq,0x72d68,false)?;
    // 0x72d50: cmp r1, #16384
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),16384)?)?;
    // 0x72d54: bne 0x72d70 <entry+0x4a4>   @ imm = #0x14
    c.branch(C::Ne,0x72d70,false)?;
    // 0x72d58: mov r0, #14
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,14)?)?;
    // 0x72d5c: b 0x72d7c <entry+0x4b0>   @ imm = #0x18
    c.branch(C::Al,0x72d7c,false)?;
    // 0x72d60: mov r0, #11
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,11)?)?;
    // 0x72d64: b 0x72d7c <entry+0x4b0>   @ imm = #0x10
    c.branch(C::Al,0x72d7c,false)?;
    // 0x72d68: mov r0, #12
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,12)?)?;
    // 0x72d6c: b 0x72d7c <entry+0x4b0>   @ imm = #0x8
    c.branch(C::Al,0x72d7c,false)?;
    // 0x72d70: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x72d74: cmp r1, #32768
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),32768)?)?;
    // 0x72d78: moveq r0, #12
    c.emit(data_immediate(C::Eq,Op::Mov,r(0),R::R0,12)?)?;
    // 0x72d7c: ldrd r2, r3, [r4, #16]
    c.emit(transfer_doubleword(C::Al,true,r(2),r(4),16)?)?;
    // 0x72d80: mov r1, r3
    c.emit(mov_register(C::Al,r(1),r(3)))?;
    // 0x72d84: bl 0x3084 <entry+0xfffffffffff907b8> @ imm = #-0x6fd08
    c.branch(C::Al,0x3084,true)?;
    // 0x72d88: ldr r0, [r5]
    c.emit(transfer_word(C::Al,true,r(0),r(5),0)?)?;
    // 0x72d8c: bl 0x293c <entry+0xfffffffffff90070> @ imm = #-0x70458
    c.branch(C::Al,0x293c,true)?;
    // 0x72d90: ldr r0, [r4]
    c.emit(transfer_word(C::Al,true,r(0),r(4),0)?)?;
    // 0x72d94: sub r1, r0, #6
    c.emit(data_immediate(C::Al,Op::Sub,r(1),r(0),6)?)?;
    // 0x72d98: cmp r1, #5
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),5)?)?;
    // 0x72d9c: bhs 0x72dc8 <entry+0x4fc>   @ imm = #0x24
    c.branch(C::Cs,0x72dc8,false)?;
    // 0x72da0: ldrb r0, [r4, #0xe]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),14)?)?;
    // 0x72da4: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72da8: bne 0x7384c <entry+0xf80>   @ imm = #0xa9c
    c.branch(C::Ne,0x7384c,false)?;
    // 0x72dac: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x72db0: strb r0, [r4, #0xe]
    c.emit(transfer_byte(C::Al,false,r(0),r(4),14)?)?;
    // 0x72db4: ldr r0, [r4, #0x2c]
    c.emit(transfer_word(C::Al,true,r(0),r(4),44)?)?;
    // 0x72db8: ldr r0, [r0, #0x14]
    c.emit(transfer_word(C::Al,true,r(0),r(0),20)?)?;
    // 0x72dbc: sub sp, r11, #28
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,r(11),28)?)?;
    // 0x72dc0: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x72dc4: b 0x292c <entry+0xfffffffffff90060> @ imm = #-0x704a0
    c.branch(C::Al,0x292c,false)?;
    // 0x72dc8: cmp r0, #5
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),5)?)?;
    // 0x72dcc: bne 0x7384c <entry+0xf80>   @ imm = #0xa78
    c.branch(C::Ne,0x7384c,false)?;
    // 0x72dd0: ldr r8, [r4, #0x38]
    c.emit(transfer_word(C::Al,true,r(8),r(4),56)?)?;
    // 0x72dd4: ldr r0, [r8, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(8),8)?)?;
    // 0x72dd8: bl 0x35d44 <entry+0xfffffffffffc3478> @ imm = #-0x3d09c
    c.branch(C::Al,0x35d44,true)?;
    // 0x72ddc: mov r7, r0
    c.emit(mov_register(C::Al,r(7),r(0)))?;
    // 0x72de0: cmp r7, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),0)?)?;
    // 0x72de4: beq 0x73834 <entry+0xf68>   @ imm = #0xa48
    c.branch(C::Eq,0x73834,false)?;
    // 0x72de8: ldr r10, [pc, #0xa74]       @ 0xf98 <entry+0xf98>
    c.literal(r(10),0x73864)?;
    // 0x72dec: add r0, sp, #44
    c.emit(data_immediate(C::Al,Op::Add,r(0),R::SP,44)?)?;
    // 0x72df0: add r0, r0, #12
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(0),12)?)?;
    // 0x72df4: str r0, [sp, #0x28]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,40)?)?;
    // 0x72df8: add r0, r4, #640
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(4),640)?)?;
    // 0x72dfc: str r0, [sp, #0x18]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,24)?)?;
    // 0x72e00: ldr r0, [r4, #0x284]
    c.emit(transfer_word(C::Al,true,r(0),r(4),644)?)?;
    // 0x72e04: cmp r0, #16384
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),16384)?)?;
    // 0x72e08: cmpne r0, #4096
    c.emit(data_immediate(C::Ne,Op::Cmp,R::R0,r(0),4096)?)?;
    // 0x72e0c: bne 0x72f88 <entry+0x6bc>   @ imm = #0x174
    c.branch(C::Ne,0x72f88,false)?;
    // 0x72e10: cmp r0, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),4096)?)?;
    // 0x72e14: bne 0x72f7c <entry+0x6b0>   @ imm = #0x160
    c.branch(C::Ne,0x72f7c,false)?;
    // 0x72e18: ldr r1, [r7, #0x58]
    c.emit(transfer_word(C::Al,true,r(1),r(7),88)?)?;
    // 0x72e1c: mov r0, #4096
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,4096)?)?;
    // 0x72e20: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72e24: beq 0x72f7c <entry+0x6b0>   @ imm = #0x150
    c.branch(C::Eq,0x72f7c,false)?;
    // 0x72e28: ldr r2, [r7, #0x64]
    c.emit(transfer_word(C::Al,true,r(2),r(7),100)?)?;
    // 0x72e2c: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x72e30: cmp r1, r2, lsr #3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(1),r(2),Shift::Lsr,3,false)?)?;
    // 0x72e34: beq 0x72f7c <entry+0x6b0>   @ imm = #0x140
    c.branch(C::Eq,0x72f7c,false)?;
    // 0x72e38: ldr r1, [r7, #0x5c]
    c.emit(transfer_word(C::Al,true,r(1),r(7),92)?)?;
    // 0x72e3c: ldr r0, [r4, #0x698]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1688)?)?;
    // 0x72e40: mov r10, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(10),R::R0,0)?)?;
    // 0x72e44: str r1, [sp, #0x20]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,32)?)?;
    // 0x72e48: lsr r1, r2, #3
    c.emit(mov_shifted_register(C::Al,r(1),r(2),Shift::Lsr,3)?)?;
    // 0x72e4c: ldr r3, [r7, #0x68]
    c.emit(transfer_word(C::Al,true,r(3),r(7),104)?)?;
    // 0x72e50: ldr lr, [r7, #0x60]
    c.emit(transfer_word(C::Al,true,R::LR,r(7),96)?)?;
    // 0x72e54: str r3, [sp, #0x24]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,36)?)?;
    // 0x72e58: str r1, [sp, #0x1c]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,28)?)?;
    // 0x72e5c: add r0, r3, r0
    c.emit(data_register(C::Al,Op::Add,r(0),r(3),r(0),Shift::Lsl,0,false)?)?;
    // 0x72e60: ldr r6, [r4, #0x38]
    c.emit(transfer_word(C::Al,true,r(6),r(4),56)?)?;
    // 0x72e64: ldrb r9, [r7]
    c.emit(transfer_byte(C::Al,true,r(9),r(7),0)?)?;
    // 0x72e68: ldr r12, [r4, #0x4]
    c.emit(transfer_word(C::Al,true,r(12),r(4),4)?)?;
    // 0x72e6c: add r0, r0, #127
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(0),127)?)?;
    // 0x72e70: bic r0, r0, #127
    c.emit(data_immediate(C::Al,Op::Bic,r(0),r(0),127)?)?;
    // 0x72e74: ldr r5, [r6, #0x18]
    c.emit(transfer_word(C::Al,true,r(5),r(6),24)?)?;
    // 0x72e78: ldr r1, [r4, #0x4]
    c.emit(transfer_word(C::Al,true,r(1),r(4),4)?)?;
    // 0x72e7c: ldr r2, [r6, #0x10]
    c.emit(transfer_word(C::Al,true,r(2),r(6),16)?)?;
    // 0x72e80: str r10, [sp]
    c.emit(transfer_word(C::Al,false,r(10),R::SP,0)?)?;
    // 0x72e84: cmp r12, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(12),1)?)?;
    // 0x72e88: str r5, [sp, #0x4]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,4)?)?;
    // 0x72e8c: str r1, [sp, #0x8]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,8)?)?;
    // 0x72e90: mov r1, #4096
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,4096)?)?;
    // 0x72e94: str r1, [sp, #0xc]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,12)?)?;
    // 0x72e98: mov r1, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,1)?)?;
    // 0x72e9c: str r1, [sp, #0x10]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,16)?)?;
    // 0x72ea0: str r2, [sp, #0x14]
    c.emit(transfer_word(C::Al,false,r(2),R::SP,20)?)?;
    // 0x72ea4: ldr r1, [sp, #0x20]
    c.emit(transfer_word(C::Al,true,r(1),R::SP,32)?)?;
    // 0x72ea8: add r2, lr, r1
    c.emit(data_register(C::Al,Op::Add,r(2),R::LR,r(1),Shift::Lsl,0,false)?)?;
    // 0x72eac: mov r5, r2
    c.emit(mov_register(C::Al,r(5),r(2)))?;
    // 0x72eb0: add r1, r3, r2, lsr #3
    c.emit(data_register(C::Al,Op::Add,r(1),r(3),r(2),Shift::Lsr,3,false)?)?;
    // 0x72eb4: mov r3, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,4)?)?;
    // 0x72eb8: str r5, [sp, #0x20]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,32)?)?;
    // 0x72ebc: moveq r3, #5
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,5)?)?;
    // 0x72ec0: ldr r10, [sp, #0x1c]
    c.emit(transfer_word(C::Al,true,r(10),R::SP,28)?)?;
    // 0x72ec4: mov r2, r10
    c.emit(mov_register(C::Al,r(2),r(10)))?;
    // 0x72ec8: bl 0x51d08 <entry+0xfffffffffffdf43c> @ imm = #-0x211c8
    c.branch(C::Al,0x51d08,true)?;
    // 0x72ecc: cmp r9, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(9),0)?)?;
    // 0x72ed0: mvn r0, #14
    c.emit(data_immediate(C::Al,Op::Mvn,r(0),R::R0,14)?)?;
    // 0x72ed4: mvneq r0, #21
    c.emit(data_immediate(C::Eq,Op::Mvn,r(0),R::R0,21)?)?;
    // 0x72ed8: cmp r9, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(9),0)?)?;
    // 0x72edc: add r1, r0, r5, lsr #3
    c.emit(data_register(C::Al,Op::Add,r(1),r(0),r(5),Shift::Lsr,3,false)?)?;
    // 0x72ee0: ldr r0, [pc, #0x96c]        @ 0xf88 <entry+0xf88>
    c.literal(r(0),0x73854)?;
    // 0x72ee4: mov r5, #31
    c.emit(data_immediate(C::Al,Op::Mov,r(5),R::R0,31)?)?;
    // 0x72ee8: orr r5, r5, #-16777216
    c.emit(data_immediate(C::Al,Op::Orr,r(5),r(5),0xff000000)?)?;
    // 0x72eec: and r2, r0, r1, lsl #5
    c.emit(data_register(C::Al,Op::And,r(2),r(0),r(1),Shift::Lsl,5,false)?)?;
    // 0x72ef0: ldr r0, [r7, #0x74]
    c.emit(transfer_word(C::Al,true,r(0),r(7),116)?)?;
    // 0x72ef4: ldr r3, [r0, #0xc]
    c.emit(transfer_word(C::Al,true,r(3),r(0),12)?)?;
    // 0x72ef8: and r3, r3, r5
    c.emit(data_register(C::Al,Op::And,r(3),r(3),r(5),Shift::Lsl,0,false)?)?;
    // 0x72efc: orr r2, r3, r2
    c.emit(data_register(C::Al,Op::Orr,r(2),r(3),r(2),Shift::Lsl,0,false)?)?;
    // 0x72f00: str r2, [r0, #0xc]
    c.emit(transfer_word(C::Al,false,r(2),r(0),12)?)?;
    // 0x72f04: mov r2, #3
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,3)?)?;
    // 0x72f08: moveq r2, #10
    c.emit(data_immediate(C::Eq,Op::Mov,r(2),R::R0,10)?)?;
    // 0x72f0c: add r2, r2, r10
    c.emit(data_register(C::Al,Op::Add,r(2),r(2),r(10),Shift::Lsl,0,false)?)?;
    // 0x72f10: add r1, r2, r1
    c.emit(data_register(C::Al,Op::Add,r(1),r(2),r(1),Shift::Lsl,0,false)?)?;
    // 0x72f14: str r1, [r0]
    c.emit(transfer_word(C::Al,false,r(1),r(0),0)?)?;
    // 0x72f18: mov r1, #16
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,16)?)?;
    // 0x72f1c: ldr r9, [r4, #0x4]
    c.emit(transfer_word(C::Al,true,r(9),r(4),4)?)?;
    // 0x72f20: bl 0x205c <entry+0xfffffffffff8f790> @ imm = #-0x70ecc
    c.branch(C::Al,0x205c,true)?;
    // 0x72f24: ldr r0, [r7, #0x74]
    c.emit(transfer_word(C::Al,true,r(0),r(7),116)?)?;
    // 0x72f28: ldr r1, [r6, #0x18]
    c.emit(transfer_word(C::Al,true,r(1),r(6),24)?)?;
    // 0x72f2c: ldr r2, [r4, #0x4]
    c.emit(transfer_word(C::Al,true,r(2),r(4),4)?)?;
    // 0x72f30: ldr r3, [r6, #0x10]
    c.emit(transfer_word(C::Al,true,r(3),r(6),16)?)?;
    // 0x72f34: mov r6, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,0)?)?;
    // 0x72f38: cmp r9, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(9),1)?)?;
    // 0x72f3c: str r6, [sp]
    c.emit(transfer_word(C::Al,false,r(6),R::SP,0)?)?;
    // 0x72f40: stmib sp, {r1, r2}
    c.emit(transfer_multiple(C::Al,false,R::SP,r(1).bit()|r(2).bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x72f44: mov r1, #4096
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,4096)?)?;
    // 0x72f48: mov r2, #16
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,16)?)?;
    // 0x72f4c: str r1, [sp, #0xc]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,12)?)?;
    // 0x72f50: str r6, [sp, #0x10]
    c.emit(transfer_word(C::Al,false,r(6),R::SP,16)?)?;
    // 0x72f54: str r3, [sp, #0x14]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,20)?)?;
    // 0x72f58: mov r3, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,4)?)?;
    // 0x72f5c: ldr r1, [sp, #0x24]
    c.emit(transfer_word(C::Al,true,r(1),R::SP,36)?)?;
    // 0x72f60: moveq r3, #5
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,5)?)?;
    // 0x72f64: bl 0x51d08 <entry+0xfffffffffffdf43c> @ imm = #-0x21264
    c.branch(C::Al,0x51d08,true)?;
    // 0x72f68: ldr r0, [sp, #0x20]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,32)?)?;
    // 0x72f6c: add r1, r10, r0, lsr #3
    c.emit(data_register(C::Al,Op::Add,r(1),r(10),r(0),Shift::Lsr,3,false)?)?;
    // 0x72f70: ldr r10, [pc, #0x8ec]       @ 0xf98 <entry+0xf98>
    c.literal(r(10),0x73864)?;
    // 0x72f74: ldr r0, [r4, #0x284]
    c.emit(transfer_word(C::Al,true,r(0),r(4),644)?)?;
    // 0x72f78: b 0x72f7c <entry+0x6b0>   @ imm = #-0x4
    c.branch(C::Al,0x72f7c,false)?;
    // 0x72f7c: mov r2, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,0)?)?;
    // 0x72f80: str r2, [r7, #0x54]
    c.emit(transfer_word(C::Al,false,r(2),r(7),84)?)?;
    // 0x72f84: str r1, [r7, #0x58]
    c.emit(transfer_word(C::Al,false,r(1),r(7),88)?)?;
    // 0x72f88: cmp r0, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),8192)?)?;
    // 0x72f8c: bne 0x73008 <entry+0x73c>   @ imm = #0x74
    c.branch(C::Ne,0x73008,false)?;
    // 0x72f90: ldrb r0, [r7, #0x7]
    c.emit(transfer_byte(C::Al,true,r(0),r(7),7)?)?;
    // 0x72f94: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72f98: beq 0x73008 <entry+0x73c>   @ imm = #0x68
    c.branch(C::Eq,0x73008,false)?;
    // 0x72f9c: ldrb r0, [r4, #0x491]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1169)?)?;
    // 0x72fa0: cmp r0, #4
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),4)?)?;
    // 0x72fa4: cmpne r0, #1
    c.emit(data_immediate(C::Ne,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x72fa8: bne 0x73008 <entry+0x73c>   @ imm = #0x58
    c.branch(C::Ne,0x73008,false)?;
    // 0x72fac: add r6, sp, #120
    c.emit(data_immediate(C::Al,Op::Add,r(6),R::SP,120)?)?;
    // 0x72fb0: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x72fb4: mov r2, #128
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,128)?)?;
    // 0x72fb8: mov r0, r6
    c.emit(mov_register(C::Al,r(0),r(6)))?;
    // 0x72fbc: bl 0x31ec <entry+0xfffffffffff90920> @ imm = #-0x6fdd8
    c.branch(C::Al,0x31ec,true)?;
    // 0x72fc0: ldr r0, [sp, #0x18]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,24)?)?;
    // 0x72fc4: mov r1, r6
    c.emit(mov_register(C::Al,r(1),r(6)))?;
    // 0x72fc8: mov r2, r4
    c.emit(mov_register(C::Al,r(2),r(4)))?;
    // 0x72fcc: bl 0x7d328 <entry+0xaa5c>  @ imm = #0xa354
    c.branch(C::Al,0x7d328,true)?;
    // 0x72fd0: ldr r1, [r7, #0x68]
    c.emit(transfer_word(C::Al,true,r(1),r(7),104)?)?;
    // 0x72fd4: ldr r2, [r7, #0x58]
    c.emit(transfer_word(C::Al,true,r(2),r(7),88)?)?;
    // 0x72fd8: mov r5, r0
    c.emit(mov_register(C::Al,r(5),r(0)))?;
    // 0x72fdc: add r0, r1, r5
    c.emit(data_register(C::Al,Op::Add,r(0),r(1),r(5),Shift::Lsl,0,false)?)?;
    // 0x72fe0: bl 0x84030 <entry+0x11764> @ imm = #0x11048
    c.branch(C::Al,0x84030,true)?;
    // 0x72fe4: ldr r0, [r7, #0x68]
    c.emit(transfer_word(C::Al,true,r(0),r(7),104)?)?;
    // 0x72fe8: mov r1, r6
    c.emit(mov_register(C::Al,r(1),r(6)))?;
    // 0x72fec: mov r2, r5
    c.emit(mov_register(C::Al,r(2),r(5)))?;
    // 0x72ff0: bl 0x471c <entry+0xfffffffffff91e50> @ imm = #-0x6e8dc
    c.branch(C::Al,0x471c,true)?;
    // 0x72ff4: ldr r0, [r7, #0x58]
    c.emit(transfer_word(C::Al,true,r(0),r(7),88)?)?;
    // 0x72ff8: add r1, r0, r5
    c.emit(data_register(C::Al,Op::Add,r(1),r(0),r(5),Shift::Lsl,0,false)?)?;
    // 0x72ffc: str r1, [r7, #0x58]
    c.emit(transfer_word(C::Al,false,r(1),r(7),88)?)?;

    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();controller_run_prefix(&mut c)?;Ok(c.bytes)}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn partial_controller_runner_keeps_strict_address_span(){
        assert_eq!(fragment().unwrap().len(),END-START);
    }
}
