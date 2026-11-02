// SPDX-License-Identifier: GPL-3.0-only
//! Recreate a Venus controller allocation and callback registration helper.
//! Allocates a controller descriptor, fills its sample-ring pointer and
//! invokes a selected platform callback with independently encoded A32.
//! The following three words are literal device addresses, not instructions.
use arm32::{branch, data_immediate, data_register, mov_register,
    multiply_register, push_registers, transfer_byte, transfer_word,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op, Register as R,
    Shift};

pub const START:usize=0x72bc8;
pub const END:usize=0x72d14;
pub const FILE_BASE:usize=0x4000;
const DIAGNOSTIC_ADDRESS:u32=0x000f_32d4;
const CONTROLLER_RECORD_BASE:u32=0x0010_ceac;
const NO_DESCRIPTOR_RESULT:u32=0x0007_2d14;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self, expected:usize)->Result<(),String>{
        if self.at==expected{Ok(())}else{Err(format!("controller alloc 0x{:x}, expected 0x{expected:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("invalid output address 0x{:x}",self.at));}
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

fn allocate_controller(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x72bc8: push {r4, r5, r6, r7, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x72bcc: add r11, sp, #16
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,16)?)?;
    // 0x72bd0: sub sp, sp, #8
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x72bd4: mov r7, r2
    c.emit(mov_register(C::Al,r(7),r(2)))?;
    // 0x72bd8: ldr r2, [pc, #0x128]        @ 0x43c <entry+0x43c>
    c.literal(r(2),0x72d08)?;
    // 0x72bdc: mov r6, r0
    c.emit(mov_register(C::Al,r(6),r(0)))?;
    // 0x72be0: mov r4, r1
    c.emit(mov_register(C::Al,r(4),r(1)))?;
    // 0x72be4: mov r1, #15
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,15)?)?;
    // 0x72be8: cmp r6, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(6),0)?)?;
    // 0x72bec: orr r1, r1, #768
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),768)?)?;
    // 0x72bf0: movne r0, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(0),R::R0,1)?)?;
    // 0x72bf4: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x7144c
    c.branch(C::Al,0x17b0,true)?;
    // 0x72bf8: mov r0, #28
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,28)?)?;
    // 0x72bfc: bl 0x2efc <entry+0xfffffffffff90630> @ imm = #-0x6fd08
    c.branch(C::Al,0x2efc,true)?;
    // 0x72c00: mov r5, r0
    c.emit(mov_register(C::Al,r(5),r(0)))?;
    // 0x72c04: cmp r5, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),0)?)?;
    // 0x72c08: beq 0x72c50 <entry+0x384>   @ imm = #0x40
    c.branch(C::Eq,0x72c50,false)?;
    // 0x72c0c: mov r0, #36
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,36)?)?;
    // 0x72c10: str r6, [r5, #0x4]
    c.emit(transfer_word(C::Al,false,r(6),r(5),4)?)?;
    // 0x72c14: bl 0x2efc <entry+0xfffffffffff90630> @ imm = #-0x6fd20
    c.branch(C::Al,0x2efc,true)?;
    // 0x72c18: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72c1c: str r0, [r5, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),r(5),8)?)?;
    // 0x72c20: beq 0x72c5c <entry+0x390>   @ imm = #0x34
    c.branch(C::Eq,0x72c5c,false)?;
    // 0x72c24: str r7, [r5, #0x18]
    c.emit(transfer_word(C::Al,false,r(7),r(5),24)?)?;
    // 0x72c28: ldr r1, [r6, #0x378]
    c.emit(transfer_word(C::Al,true,r(1),r(6),888)?)?;
    // 0x72c2c: ldrb r2, [r6, #0x193]
    c.emit(transfer_byte(C::Al,true,r(2),r(6),403)?)?;
    // 0x72c30: add r1, r1, #1
    c.emit(data_immediate(C::Al,Op::Add,r(1),r(1),1)?)?;
    // 0x72c34: cmp r2, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),0)?)?;
    // 0x72c38: beq 0x72c80 <entry+0x3b4>   @ imm = #0x40
    c.branch(C::Eq,0x72c80,false)?;
    // 0x72c3c: ldr r3, [r6, #0x3a4]
    c.emit(transfer_word(C::Al,true,r(3),r(6),932)?)?;
    // 0x72c40: cmp r3, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x72c44: moveq r3, #1
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,1)?)?;
    // 0x72c48: mul r2, r3, r1
    c.emit(multiply_register(C::Al,r(2),r(3),r(1))?)?;
    // 0x72c4c: b 0x72c84 <entry+0x3b8>   @ imm = #0x30
    c.branch(C::Al,0x72c84,false)?;
    // 0x72c50: mov r4, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(4),R::R0,0)?)?;
    // 0x72c54: mov r1, #22
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,22)?)?;
    // 0x72c58: b 0x72c6c <entry+0x3a0>   @ imm = #0xc
    c.branch(C::Al,0x72c6c,false)?;
    // 0x72c5c: mov r0, r5
    c.emit(mov_register(C::Al,r(0),r(5)))?;
    // 0x72c60: bl 0x2f1c <entry+0xfffffffffff90650> @ imm = #-0x6fd4c
    c.branch(C::Al,0x2f1c,true)?;
    // 0x72c64: mov r4, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(4),R::R0,0)?)?;
    // 0x72c68: mov r1, #33
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,33)?)?;
    // 0x72c6c: ldr r2, [pc, #0x94]         @ 0x43c <entry+0x43c>
    c.literal(r(2),0x72d08)?;
    // 0x72c70: orr r1, r1, #768
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),768)?)?;
    // 0x72c74: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x72c78: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x714d0
    c.branch(C::Al,0x17b0,true)?;
    // 0x72c7c: b 0x72cfc <entry+0x430>   @ imm = #0x78
    c.branch(C::Al,0x72cfc,false)?;
    // 0x72c80: mov r2, r1
    c.emit(mov_register(C::Al,r(2),r(1)))?;
    // 0x72c84: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x72c88: mov r1, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,1)?)?;
    // 0x72c8c: str r3, [sp]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,0)?)?;
    // 0x72c90: str r1, [sp, #0x4]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,4)?)?;
    // 0x72c94: mov r1, #280
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,280)?)?;
    // 0x72c98: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x72c9c: bl 0x35b30 <entry+0xfffffffffffc3264> @ imm = #-0x3d174
    c.branch(C::Al,0x35b30,true)?;
    // 0x72ca0: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x72ca4: beq 0x72cbc <entry+0x3f0>   @ imm = #0x10
    c.branch(C::Eq,0x72cbc,false)?;
    // 0x72ca8: ldr r2, [pc, #0x58]         @ 0x43c <entry+0x43c>
    c.literal(r(2),0x72d08)?;
    // 0x72cac: mov r1, #50
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,50)?)?;
    // 0x72cb0: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x72cb4: orr r1, r1, #768
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),768)?)?;
    // 0x72cb8: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x71510
    c.branch(C::Al,0x17b0,true)?;
    // 0x72cbc: mov r0, #8
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,8)?)?;
    // 0x72cc0: bl 0x2efc <entry+0xfffffffffff90630> @ imm = #-0x6fdcc
    c.branch(C::Al,0x2efc,true)?;
    // 0x72cc4: ldr r2, [pc, #0x3c]         @ 0x43c <entry+0x43c>
    c.literal(r(2),0x72d08)?;
    // 0x72cc8: mov r1, #55
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,55)?)?;
    // 0x72ccc: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72cd0: str r0, [r5]
    c.emit(transfer_word(C::Al,false,r(0),r(5),0)?)?;
    // 0x72cd4: movne r0, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(0),R::R0,1)?)?;
    // 0x72cd8: orr r1, r1, #768
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),768)?)?;
    // 0x72cdc: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x71534
    c.branch(C::Al,0x17b0,true)?;
    // 0x72ce0: ldr r0, [r5, #0x18]
    c.emit(transfer_word(C::Al,true,r(0),r(5),24)?)?;
    // 0x72ce4: ldr r1, [pc, #0x20]         @ 0x440 <entry+0x440>
    c.literal(r(1),0x72d0c)?;
    // 0x72ce8: add r0, r1, r0, lsl #3
    c.emit(data_register(C::Al,Op::Add,r(0),r(1),r(0),Shift::Lsl,3,false)?)?;
    // 0x72cec: add r0, r0, #36
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(0),36)?)?;
    // 0x72cf0: str r0, [r5, #0x10]
    c.emit(transfer_word(C::Al,false,r(0),r(5),16)?)?;
    // 0x72cf4: str r5, [r4]
    c.emit(transfer_word(C::Al,false,r(5),r(4),0)?)?;
    // 0x72cf8: ldr r4, [pc, #0x10]         @ 0x444 <entry+0x444>
    c.literal(r(4),0x72d10)?;
    // 0x72cfc: mov r0, r4
    c.emit(mov_register(C::Al,r(0),r(4)))?;
    // 0x72d00: sub sp, r11, #16
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,r(11),16)?)?;
    // 0x72d04: pop {r4, r5, r6, r7, r11, pc}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;

    c.check(0x72d08)?;
    for word in [DIAGNOSTIC_ADDRESS,CONTROLLER_RECORD_BASE,NO_DESCRIPTOR_RESULT] {
        c.emit(word)?;
    }
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();allocate_controller(&mut c)?;Ok(c.bytes)}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_alloc_has_three_named_literals(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        for (i,word) in [DIAGNOSTIC_ADDRESS,CONTROLLER_RECORD_BASE,NO_DESCRIPTOR_RESULT].into_iter().enumerate(){
            let offset=b.len()-12+i*4;
            assert_eq!(&b[offset..offset+4],&word.to_le_bytes());
        }
    }
}
