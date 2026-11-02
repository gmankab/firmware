// SPDX-License-Identifier: GPL-3.0-only
//! Alternate 64-bit occupancy accumulator for Venus codec controller records.
//! The 8192-byte record layout uses 16-byte samples and nine contiguous
//! 144-byte blocks; the alternate layout scans one byte each 64-byte record.
//! Signed flags and carry bits remain explicit in typed ARM32 operations.
use arm32::{branch, data_immediate, data_immediate_with_flags, data_register,
    mov_register, mov_shifted_register, multiply_accumulate, push_registers,
    transfer_byte, transfer_byte_postindexed, transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START: usize = 0x715ec;
pub const END: usize = 0x71790;
pub const FILE_BASE: usize = 0x4000;
const DIVIDE_BY_FIVE_RECIPROCAL: u32 = 0xcccc_cccd;
const ALLOCATION_DIAGNOSTIC: u32 = 0x000f_2e4c;

struct Code{at:usize,bytes:Vec<u8>}
impl Code {
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,target:usize)->Result<(),String>{
        if self.at==target {Ok(())} else {Err(format!("alt occupancy at 0x{:x}, expected 0x{target:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at < START || self.at>=END || self.at%4!=0 {return Err(format!("out-of-range A32 at 0x{:x}",self.at));}
        self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
    fn jump(&mut self,cond:C,dst:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,dst as u32,link)?)
    }
    fn literal(&mut self,dst:R,target:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,dst,R::PC,target as i32-self.at as i32-8)?)
    }
}
fn r(n:u8)->R{R::new(n).expect("register number is in 0..15")}
fn imm(c:&mut Code, cond:C, op:Op, dst:R, src:R, val:u32, flags:bool)->Result<(),String>{
    if flags {c.emit(data_immediate_with_flags(cond,op,dst,src,val,true)?)}
    else {c.emit(data_immediate(cond,op,dst,src,val)?)}
}
fn reg(c:&mut Code,cond:C,op:Op,dst:R,lhs:R,rhs:R,shift:Shift,count:u8,flags:bool)->Result<(),String>{
    c.emit(data_register(cond,op,dst,lhs,rhs,shift,count,flags)?)
}
fn load(c:&mut Code,dst:R,src:R,off:i32)->Result<(),String>{c.emit(transfer_word(C::Al,true,dst,src,off)?)}
fn store(c:&mut Code,src:R,dst:R,off:i32)->Result<(),String>{c.emit(transfer_word(C::Al,false,src,dst,off)?)}
fn cmp(c:&mut Code,src:R,what:u32)->Result<(),String>{imm(c,C::Al,Op::Cmp,R::R0,src,what,false)}
fn set(c:&mut Code,dst:R,value:u32)->Result<(),String>{imm(c,C::Al,Op::Mov,dst,R::R0,value,false)}
fn mov(c:&mut Code,dst:R,src:R)->Result<(),String>{c.emit(mov_register(C::Al,dst,src))}
fn shift(c:&mut Code,dst:R,src:R,shift:Shift,n:u8)->Result<(),String>{c.emit(mov_shifted_register(C::Al,dst,src,shift,n)?)}
fn umull(c:&mut Code,low:R,high:R,left:R,right:R)->Result<(),String>{
    if low==high || [low,high,left,right].iter().any(|r|r.number()>=13){return Err("UMULL invalid register operands".into());}
    c.emit(0xe080_0090 | (high.number()<<16)|(low.number()<<12)|(right.number()<<8)|left.number())
}
fn adc(c:&mut Code,dst:R,lhs:R,value:u32)->Result<(),String>{
    c.emit(data_immediate(C::Al,Op::Add,dst,lhs,value)? | (1<<21))
}
fn frame()->u16 {(4..=11).map(|n|1u16<<n).sum::<u16>()|R::LR.bit()}
fn alt_occupancy(c:&mut Code)->Result<(),String>{
    let r3=r(3);let r4=r(4);let r5=r(5);let r6=r(6);
    let r7=r(7);let r8=r(8);let r9=r(9);let r10=r(10);
    c.check(START)?;
    c.emit(push_registers(C::Al,frame())?)?;
    imm(c,C::Al,Op::Add,R::R11,R::SP,28,false)?;
    imm(c,C::Al,Op::Sub,R::SP,R::SP,4,false)?;
    mov(c,r10,R::R0)?;
    mov(c,r5,R::R1)?;
    cmp(c,r10,0)?;
    c.jump(C::Eq,0x71780,false)?;
    cmp(c,r5,0)?;
    imm(c,C::Ne,Op::Cmp,R::R0,R::R2,0,false)?;
    c.jump(C::Eq,0x71780,false)?;
    load(c,R::R0,r10,0x16c)?;
    c.literal(R::R1,0x71788)?;
    load(c,r8,r5,0x7ec)?;
    load(c,r9,R::R2,0x38)?;
    umull(c,r3,r7,R::R0,R::R1)?;
    cmp(c,r8,0)?;
    shift(c,R::R1,r7,Shift::Lsr,2)?;
    load(c,r7,R::R2,0x50)?;
    c.literal(R::R2,0x7178c)?;
    reg(c,C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?;
    reg(c,C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?;
    set(c,R::R1,22)?;
    reg(c,C::Al,Op::Add,R::R0,r10,R::R0,Shift::Lsl,6,false)?;
    imm(c,C::Al,Op::Orr,R::R1,R::R1,8704,false)?;
    c.emit(transfer_byte(C::Al,true,r4,R::R0,0x38)?)?;
    mov(c,R::R0,r8)?;
    imm(c,C::Ne,Op::Mov,R::R0,R::R0,1,false)?;
    c.jump(C::Al,0x17b0,true)?;
    load(c,R::R0,r5,4)?;
    cmp(c,R::R0,8192)?;
    c.jump(C::Ne,0x716d4,false)?;
    cmp(c,r8,0)?;
    c.jump(C::Eq,0x71714,false)?;
    mov(c,R::LR,r4)?;
    imm(c,C::Al,Op::Add,R::R12,r7,32,false)?;
    set(c,R::R1,0)?;
    set(c,r3,0)?;
    set(c,R::R2,0)?;
    set(c,r5,0)?;
    mov(c,R::R0,R::R12)?;
    c.check(0x7168c)?;
    c.emit(transfer_byte_postindexed(C::Al,true,r7,R::R0,8)?)?;
    imm(c,C::Al,Op::Add,r4,r3,1,true)?;
    imm(c,C::Al,Op::Add,r5,r5,1,false)?;
    adc(c,r6,R::R2,0)?;
    imm(c,C::Al,Op::And,r7,r7,3,true)?;
    c.emit(mov_register(C::Eq,r3,r4))?;
    c.emit(mov_register(C::Eq,R::R2,r6))?;
    cmp(c,r5,16)?;
    c.jump(C::Ne,0x7168c,false)?;
    imm(c,C::Al,Op::Add,R::R1,R::R1,1,false)?;
    imm(c,C::Al,Op::Add,R::R12,R::R12,144,false)?;
    reg(c,C::Al,Op::Cmp,R::R0,R::R1,r8,Shift::Lsl,0,false)?;
    c.jump(C::Ne,0x71684,false)?;
    shift(c,R::R0,r3,Shift::Lsr,4)?;
    shift(c,R::R1,R::R2,Shift::Lsr,4)?;
    mov(c,r4,R::LR)?;
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,28,false)?;
    c.jump(C::Al,0x7171c,false)?;
    c.check(0x716d4)?;
    cmp(c,r8,0)?;
    c.jump(C::Eq,0x71724,false)?;
    imm(c,C::Al,Op::Add,R::R2,r7,32,false)?;
    set(c,r3,0)?;
    set(c,R::R0,0)?;
    set(c,R::R1,0)?;
    c.check(0x716ec)?;
    c.emit(transfer_byte_postindexed(C::Al,true,r5,R::R2,64)?)?;
    imm(c,C::Al,Op::Add,r7,R::R0,1,true)?;
    imm(c,C::Al,Op::Add,r3,r3,1,false)?;
    adc(c,r6,R::R1,0)?;
    imm(c,C::Al,Op::And,r5,r5,3,true)?;
    c.emit(mov_register(C::Eq,R::R0,r7))?;
    c.emit(mov_register(C::Eq,R::R1,r6))?;
    reg(c,C::Al,Op::Cmp,R::R0,r3,r8,Shift::Lsl,0,false)?;
    c.jump(C::Ne,0x716ec,false)?;
    c.jump(C::Al,0x7172c,false)?;
    c.check(0x71714)?;
    set(c,R::R0,0)?;
    set(c,R::R1,0)?;
    c.check(0x7171c)?;
    shift(c,r9,r9,Shift::Lsr,4)?;
    c.jump(C::Al,0x7172c,false)?;
    c.check(0x71724)?;
    set(c,R::R0,0)?;
    set(c,R::R1,0)?;
    c.check(0x7172c)?;
    imm(c,C::Al,Op::Add,R::R0,R::R0,1,true)?;
    adc(c,R::R1,R::R1,0)?;
    imm(c,C::Al,Op::Add,R::R2,r9,1,true)?;
    umull(c,r3,r7,R::R2,r4)?;
    umull(c,R::R2,r6,r3,R::R0)?;
    c.emit(multiply_accumulate(C::Al,r5,r3,R::R1,r6)?)?;
    set(c,R::R1,0)?;
    adc(c,R::R1,R::R1,0)?;
    c.emit(multiply_accumulate(C::Al,r3,R::R1,r4,r7)?)?;
    c.emit(multiply_accumulate(C::Al,R::R1,r3,R::R0,r5)?)?;
    load(c,R::R0,r10,0x16c)?;
    c.literal(r3,0x71788)?;
    umull(c,r7,r6,R::R0,r3)?;
    shift(c,r3,r6,Shift::Lsr,2)?;
    reg(c,C::Al,Op::Add,r3,r3,r3,Shift::Lsl,2,false)?;
    reg(c,C::Al,Op::Sub,r3,R::R0,r3,Shift::Lsl,0,false)?;
    imm(c,C::Al,Op::Add,R::R0,R::R0,1,false)?;
    reg(c,C::Al,Op::Add,r3,r10,r3,Shift::Lsl,6,false)?;
    store(c,R::R2,r3,0x30)?;
    store(c,R::R1,r3,0x34)?;
    store(c,R::R0,r10,0x16c)?;
    c.check(0x71780)?;
    imm(c,C::Al,Op::Sub,R::SP,R::R11,28,false)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,(frame()&!R::LR.bit())|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x71788)?;
    for word in [DIVIDE_BY_FIVE_RECIPROCAL,ALLOCATION_DIAGNOSTIC]{c.emit(word)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();alt_occupancy(&mut c)?;Ok(c.bytes)}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn alternate_occupancy_has_two_named_data_words(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        assert_eq!(&bytes[bytes.len()-8..bytes.len()-4],&DIVIDE_BY_FIVE_RECIPROCAL.to_le_bytes());
        assert_eq!(&bytes[bytes.len()-4..],&ALLOCATION_DIAGNOSTIC.to_le_bytes());
    }
}
