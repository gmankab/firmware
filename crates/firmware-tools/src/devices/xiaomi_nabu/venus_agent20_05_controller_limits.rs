// SPDX-License-Identifier: GPL-3.0-only
//! Leading 548 bytes of Venus codec controller boundary selection. This
//! function is not complete: branches intentionally target later portions
//! still to be reconstructed, and the source does not fabricate a return.
//! It chooses mode-specific sampling limits and signed halfword accumulators.
use arm32::{branch, data_immediate, data_register,
    mov_register, push_registers, transfer_byte,
    transfer_word, transfer_word_register,
    Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x71ed0;
pub const END:usize=0x720f4;
pub const FILE_BASE:usize=0x4000;
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at==addr{Ok(())}else{Err(format!("controller selection 0x{:x} != 0x{addr:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("invalid partial A32 address 0x{:x}",self.at));}
        self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,target as u32,link)?)
    }
}
fn r(n:u8)->R{R::new(n).unwrap()}
fn smlabb(c:&mut Code,rd:R,rm:R,rs:R,addend:R)->Result<(),String>{
    // ARMv6 signed lower-halfword multiply accumulate:
    // SMLABB Rd, Rm, Rs, Rn; register fields validated before encoding.
    if [rd,rm,rs,addend].iter().any(|x|x.number()>=13){return Err("invalid SMLABB register operands".into());}
    c.emit(0xe100_0080|(rd.number()<<16)|(addend.number()<<12)|(rs.number()<<8)|rm.number())
}
fn select_limits_prefix(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x71ed0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x71ed4: add r11, sp, #28
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,28)?)?;
    // 0x71ed8: sub sp, sp, #44
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,R::SP,44)?)?;
    // 0x71edc: mov r4, r0
    c.emit(mov_register(C::Al,r(4),r(0)))?;
    // 0x71ee0: mov r7, r2
    c.emit(mov_register(C::Al,r(7),r(2)))?;
    // 0x71ee4: mov r6, r1
    c.emit(mov_register(C::Al,r(6),r(1)))?;
    // 0x71ee8: ldr r0, [r4, #0xc]
    c.emit(transfer_word(C::Al,true,r(0),r(4),12)?)?;
    // 0x71eec: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x71ef0: beq 0x71f20 <entry+0x934>   @ imm = #0x28
    c.branch(C::Eq,0x71f20,false)?;
    // 0x71ef4: cmp r0, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),2)?)?;
    // 0x71ef8: ldrbne r1, [r4, #0x7d]
    c.emit(transfer_byte(C::Ne,true,r(1),r(4),125)?)?;
    // 0x71efc: cmpne r1, #1
    c.emit(data_immediate(C::Ne,Op::Cmp,R::R0,r(1),1)?)?;
    // 0x71f00: bne 0x71f3c <entry+0x950>   @ imm = #0x34
    c.branch(C::Ne,0x71f3c,false)?;
    // 0x71f04: cmp r0, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),2)?)?;
    // 0x71f08: ldreq r0, [r4, #0x2d8]
    c.emit(transfer_word(C::Eq,true,r(0),r(4),728)?)?;
    // 0x71f0c: cmpeq r0, #0
    c.emit(data_immediate(C::Eq,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x71f10: beq 0x71fa4 <entry+0x9b8>   @ imm = #0x8c
    c.branch(C::Eq,0x71fa4,false)?;
    // 0x71f14: ldrb r9, [r6, #0x7]
    c.emit(transfer_byte(C::Al,true,r(9),r(6),7)?)?;
    // 0x71f18: ldrb r3, [r6, #0x9]
    c.emit(transfer_byte(C::Al,true,r(3),r(6),9)?)?;
    // 0x71f1c: b 0x71fe8 <entry+0x9fc>   @ imm = #0xc4
    c.branch(C::Al,0x71fe8,false)?;
    // 0x71f20: ldr r0, [r4, #0x2d8]
    c.emit(transfer_word(C::Al,true,r(0),r(4),728)?)?;
    // 0x71f24: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x71f28: beq 0x71f6c <entry+0x980>   @ imm = #0x3c
    c.branch(C::Eq,0x71f6c,false)?;
    // 0x71f2c: ldr r0, [r4, #0xa80]
    c.emit(transfer_word(C::Al,true,r(0),r(4),2688)?)?;
    // 0x71f30: ldrb r9, [r0, #0x7]
    c.emit(transfer_byte(C::Al,true,r(9),r(0),7)?)?;
    // 0x71f34: ldrb r3, [r0, #0x9]
    c.emit(transfer_byte(C::Al,true,r(3),r(0),9)?)?;
    // 0x71f38: b 0x71fe8 <entry+0x9fc>   @ imm = #0xa8
    c.branch(C::Al,0x71fe8,false)?;
    // 0x71f3c: ldrb r0, [r4, #0x2d2]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),722)?)?;
    // 0x71f40: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x71f44: beq 0x71f80 <entry+0x994>   @ imm = #0x34
    c.branch(C::Eq,0x71f80,false)?;
    // 0x71f48: mov r1, #2864
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,2864)?)?;
    // 0x71f4c: mov r2, #948
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,948)?)?;
    // 0x71f50: orr r1, r1, #4096
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),4096)?)?;
    // 0x71f54: orr r2, r2, #1024
    c.emit(data_immediate(C::Al,Op::Orr,r(2),r(2),1024)?)?;
    // 0x71f58: ldr r1, [r4, r1]
    c.emit(transfer_word_register(C::Al,true,r(1),r(4),r(1),Shift::Lsl,0)?)?;
    // 0x71f5c: smlabb r0, r0, r2, r1
    smlabb(c,r(0),r(0),r(2),r(1))?;
    // 0x71f60: ldrb r9, [r0, #0xef]
    c.emit(transfer_byte(C::Al,true,r(9),r(0),239)?)?;
    // 0x71f64: ldrb r3, [r0, #0xf1]
    c.emit(transfer_byte(C::Al,true,r(3),r(0),241)?)?;
    // 0x71f68: b 0x71fe8 <entry+0x9fc>   @ imm = #0x78
    c.branch(C::Al,0x71fe8,false)?;
    // 0x71f6c: ldr r0, [r4, #0x64]
    c.emit(transfer_word(C::Al,true,r(0),r(4),100)?)?;
    // 0x71f70: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x71f74: beq 0x71f2c <entry+0x940>   @ imm = #-0x50
    c.branch(C::Eq,0x71f2c,false)?;
    // 0x71f78: ldrb r0, [r4, #0xd16]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),3350)?)?;
    // 0x71f7c: b 0x71fb4 <entry+0x9c8>   @ imm = #0x30
    c.branch(C::Al,0x71fb4,false)?;
    // 0x71f80: mov r0, #2864
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,2864)?)?;
    // 0x71f84: ldr r1, [r4, #0x2dc]
    c.emit(transfer_word(C::Al,true,r(1),r(4),732)?)?;
    // 0x71f88: orr r0, r0, #4096
    c.emit(data_immediate(C::Al,Op::Orr,r(0),r(0),4096)?)?;
    // 0x71f8c: ldr r0, [r4, r0]
    c.emit(transfer_word_register(C::Al,true,r(0),r(4),r(0),Shift::Lsl,0)?)?;
    // 0x71f90: cmp r1, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),1)?)?;
    // 0x71f94: bne 0x71fe0 <entry+0x9f4>   @ imm = #0x44
    c.branch(C::Ne,0x71fe0,false)?;
    // 0x71f98: ldrb r9, [r0, #0x8a3]
    c.emit(transfer_byte(C::Al,true,r(9),r(0),2211)?)?;
    // 0x71f9c: ldrb r3, [r0, #0x8a5]
    c.emit(transfer_byte(C::Al,true,r(3),r(0),2213)?)?;
    // 0x71fa0: b 0x71fe8 <entry+0x9fc>   @ imm = #0x40
    c.branch(C::Al,0x71fe8,false)?;
    // 0x71fa4: mov r0, #2864
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,2864)?)?;
    // 0x71fa8: orr r0, r0, #4096
    c.emit(data_immediate(C::Al,Op::Orr,r(0),r(0),4096)?)?;
    // 0x71fac: ldr r0, [r4, r0]
    c.emit(transfer_word_register(C::Al,true,r(0),r(4),r(0),Shift::Lsl,0)?)?;
    // 0x71fb0: ldrb r0, [r0, #0x89e]
    c.emit(transfer_byte(C::Al,true,r(0),r(0),2206)?)?;
    // 0x71fb4: ldrb r1, [r4, #0x2c]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),44)?)?;
    // 0x71fb8: ldrb r2, [r4, #0x2d]
    c.emit(transfer_byte(C::Al,true,r(2),r(4),45)?)?;
    // 0x71fbc: ldrb r3, [r6, #0xc]
    c.emit(transfer_byte(C::Al,true,r(3),r(6),12)?)?;
    // 0x71fc0: ldrb r9, [r6, #0xb]
    c.emit(transfer_byte(C::Al,true,r(9),r(6),11)?)?;
    // 0x71fc4: add r1, r1, r0
    c.emit(data_register(C::Al,Op::Add,r(1),r(1),r(0),Shift::Lsl,0,false)?)?;
    // 0x71fc8: sub r0, r0, r2
    c.emit(data_register(C::Al,Op::Sub,r(0),r(0),r(2),Shift::Lsl,0,false)?)?;
    // 0x71fcc: cmp r1, r3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(1),r(3),Shift::Lsl,0,false)?)?;
    // 0x71fd0: movlo r3, r1
    c.emit(mov_register(C::Cc,r(3),r(1)))?;
    // 0x71fd4: cmp r0, r9
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(9),Shift::Lsl,0,false)?)?;
    // 0x71fd8: movgt r9, r0
    c.emit(mov_register(C::Gt,r(9),r(0)))?;
    // 0x71fdc: b 0x71fe8 <entry+0x9fc>   @ imm = #0x4
    c.branch(C::Al,0x71fe8,false)?;
    // 0x71fe0: ldrb r9, [r0, #0x613]
    c.emit(transfer_byte(C::Al,true,r(9),r(0),1555)?)?;
    // 0x71fe4: ldrb r3, [r0, #0x615]
    c.emit(transfer_byte(C::Al,true,r(3),r(0),1557)?)?;
    // 0x71fe8: ldrb r0, [r4, #0x778]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1912)?)?;
    // 0x71fec: ldrb r8, [r4, #0x2d0]
    c.emit(transfer_byte(C::Al,true,r(8),r(4),720)?)?;
    // 0x71ff0: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x71ff4: beq 0x72120 <entry+0xb34>   @ imm = #0x124
    c.branch(C::Eq,0x72120,false)?;
    // 0x71ff8: ldrb r0, [r4, #0x2c]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),44)?)?;
    // 0x71ffc: mov r10, r3
    c.emit(mov_register(C::Al,r(10),r(3)))?;
    // 0x72000: add r0, r0, r8
    c.emit(data_register(C::Al,Op::Add,r(0),r(0),r(8),Shift::Lsl,0,false)?)?;
    // 0x72004: ldrb r1, [r4, #0x30]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),48)?)?;
    // 0x72008: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x7200c: beq 0x722b8 <entry+0xccc>   @ imm = #0x2a4
    c.branch(C::Eq,0x722b8,false)?;
    // 0x72010: ldr r1, [r4, #0xc]
    c.emit(transfer_word(C::Al,true,r(1),r(4),12)?)?;
    // 0x72014: cmp r1, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),1)?)?;
    // 0x72018: ldreq r1, [r4, #0x2d8]
    c.emit(transfer_word(C::Eq,true,r(1),r(4),728)?)?;
    // 0x7201c: cmpeq r1, #1
    c.emit(data_immediate(C::Eq,Op::Cmp,R::R0,r(1),1)?)?;
    // 0x72020: beq 0x72310 <entry+0xd24>   @ imm = #0x2e8
    c.branch(C::Eq,0x72310,false)?;
    // 0x72024: ldrb r1, [r4, #0x89]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),137)?)?;
    // 0x72028: cmp r1, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),1)?)?;
    // 0x7202c: bne 0x72088 <entry+0xa9c>   @ imm = #0x54
    c.branch(C::Ne,0x72088,false)?;
    // 0x72030: ldr r1, [r4]
    c.emit(transfer_word(C::Al,true,r(1),r(4),0)?)?;
    // 0x72034: cmp r1, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),4096)?)?;
    // 0x72038: beq 0x72088 <entry+0xa9c>   @ imm = #0x48
    c.branch(C::Eq,0x72088,false)?;
    // 0x7203c: ldr r1, [r4, #0x2d8]
    c.emit(transfer_word(C::Al,true,r(1),r(4),728)?)?;
    // 0x72040: ldr r2, [r4, #0x94]
    c.emit(transfer_word(C::Al,true,r(2),r(4),148)?)?;
    // 0x72044: ldr r3, [r4, #0x2e4]
    c.emit(transfer_word(C::Al,true,r(3),r(4),740)?)?;
    // 0x72048: add r1, r4, r1, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(1),r(4),r(1),Shift::Lsl,2,false)?)?;
    // 0x7204c: ldr r1, [r1, #0x5f4]
    c.emit(transfer_word(C::Al,true,r(1),r(1),1524)?)?;
    // 0x72050: sub r1, r1, r2
    c.emit(data_register(C::Al,Op::Sub,r(1),r(1),r(2),Shift::Lsl,0,false)?)?;
    // 0x72054: ldr r2, [r7, #0x4]
    c.emit(transfer_word(C::Al,true,r(2),r(7),4)?)?;
    // 0x72058: cmp r3, r2
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(2),Shift::Lsl,0,false)?)?;
    // 0x7205c: bhi 0x72070 <entry+0xa84>   @ imm = #0xc
    c.branch(C::Hi,0x72070,false)?;
    // 0x72060: ldrb r3, [r4, #0x630]
    c.emit(transfer_byte(C::Al,true,r(3),r(4),1584)?)?;
    // 0x72064: and r2, r0, #255
    c.emit(data_immediate(C::Al,Op::And,r(2),r(0),255)?)?;
    // 0x72068: cmp r3, r2
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(2),Shift::Lsl,0,false)?)?;
    // 0x7206c: movhi r0, r3
    c.emit(mov_register(C::Hi,r(0),r(3)))?;
    // 0x72070: cmp r1, #16
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),16)?)?;
    // 0x72074: blt 0x72088 <entry+0xa9c>   @ imm = #0xc
    c.branch(C::Lt,0x72088,false)?;
    // 0x72078: ldrb r1, [r4, #0x630]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),1584)?)?;
    // 0x7207c: and r2, r0, #255
    c.emit(data_immediate(C::Al,Op::And,r(2),r(0),255)?)?;
    // 0x72080: cmp r2, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(1),Shift::Lsl,0,false)?)?;
    // 0x72084: addls r0, r0, #1
    c.emit(data_immediate(C::Ls,Op::Add,r(0),r(0),1)?)?;
    // 0x72088: ldr r2, [r4, #0x2d8]
    c.emit(transfer_word(C::Al,true,r(2),r(4),728)?)?;
    // 0x7208c: cmp r2, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),1)?)?;
    // 0x72090: bne 0x720a4 <entry+0xab8>   @ imm = #0xc
    c.branch(C::Ne,0x720a4,false)?;
    // 0x72094: ldr r1, [r4, #0x294]
    c.emit(transfer_word(C::Al,true,r(1),r(4),660)?)?;
    // 0x72098: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x7209c: ldrbne r1, [r6, #0x2]
    c.emit(transfer_byte(C::Ne,true,r(1),r(6),2)?)?;
    // 0x720a0: addne r10, r1, #2
    c.emit(data_immediate(C::Ne,Op::Add,r(10),r(1),2)?)?;
    // 0x720a4: and r1, r10, #255
    c.emit(data_immediate(C::Al,Op::And,r(1),r(10),255)?)?;
    // 0x720a8: and r3, r0, #255
    c.emit(data_immediate(C::Al,Op::And,r(3),r(0),255)?)?;
    // 0x720ac: cmp r3, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(1),Shift::Lsl,0,false)?)?;
    // 0x720b0: and r1, r9, #255
    c.emit(data_immediate(C::Al,Op::And,r(1),r(9),255)?)?;
    // 0x720b4: movhi r0, r10
    c.emit(mov_register(C::Hi,r(0),r(10)))?;
    // 0x720b8: cmp r3, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(1),Shift::Lsl,0,false)?)?;
    // 0x720bc: ldrb r1, [r4, #0x25]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),37)?)?;
    // 0x720c0: movlo r0, r9
    c.emit(mov_register(C::Cc,r(0),r(9)))?;
    // 0x720c4: cmp r1, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),2)?)?;
    // 0x720c8: bhi 0x72364 <entry+0xd78>   @ imm = #0x294
    c.branch(C::Hi,0x72364,false)?;
    // 0x720cc: ldr r3, [r4, #0x2dc]
    c.emit(transfer_word(C::Al,true,r(3),r(4),732)?)?;
    // 0x720d0: cmp r2, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),1)?)?;
    // 0x720d4: mov r2, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,0)?)?;
    // 0x720d8: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x720dc: movne r2, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(2),R::R0,1)?)?;
    // 0x720e0: cmp r3, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x720e4: cmpeq r2, #0
    c.emit(data_immediate(C::Eq,Op::Cmp,R::R0,r(2),0)?)?;
    // 0x720e8: bne 0x72338 <entry+0xd4c>   @ imm = #0x248
    c.branch(C::Ne,0x72338,false)?;
    // 0x720ec: ldrb r7, [r4, #0x2d1]
    c.emit(transfer_byte(C::Al,true,r(7),r(4),721)?)?;
    // 0x720f0: and r3, r0, #255
    c.emit(data_immediate(C::Al,Op::And,r(3),r(0),255)?)?;

    c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();select_limits_prefix(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_limits_prefix_is_explicitly_partial(){
        assert_eq!(fragment().unwrap().len(),END-START);
        assert_eq!(END,0x720f4);
    }
}
