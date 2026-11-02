// SPDX-License-Identifier: GPL-3.0-only
//! Finalize one five-slot codec occupancy update with the controller's
//! scaled 64-bit sample and persist its paired result atomically in STRD.
//! The reference has 36 A32 instructions followed by two named literals.
use arm32::{branch, data_immediate, data_immediate_with_flags, data_register,
    mov_register, mov_shifted_register, multiply_accumulate, multiply_register,
    push_registers, transfer_doubleword, transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x71790;
pub const END:usize=0x71828;
pub const FILE_BASE:usize=0x4000;
const DIVIDE_BY_FIVE_RECIPROCAL:u32=0xcccc_cccd;
const METRIC_OUTPUT_FACTOR:u32=0x0002_4d77;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,target:usize)->Result<(),String>{
        if self.at==target {Ok(())}else{Err(format!("occupancy commit addr 0x{:x}, expected 0x{target:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0 {return Err(format!("out-of-range A32 0x{:x}",self.at));}
        self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,where_to:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,where_to as u32,link)?)
    }
    fn literal(&mut self,dst:R,where_at:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,dst,R::PC,where_at as i32-self.at as i32-8)?)
    }
}
fn r(n:u8)->R{R::new(n).unwrap()}
fn imm(c:&mut Code,cond:C,op:Op,dst:R,src:R,val:u32,flags:bool)->Result<(),String>{
    if flags {c.emit(data_immediate_with_flags(cond,op,dst,src,val,true)?)}
    else {c.emit(data_immediate(cond,op,dst,src,val)?)}
}
fn rg(c:&mut Code,op:Op,dst:R,left:R,right:R,shift:Shift,bits:u8,flags:bool)->Result<(),String>{
    c.emit(data_register(C::Al,op,dst,left,right,shift,bits,flags)?)
}
fn set(c:&mut Code,dst:R,val:u32)->Result<(),String>{imm(c,C::Al,Op::Mov,dst,R::R0,val,false)}
fn cmp(c:&mut Code,dst:R,val:u32)->Result<(),String>{imm(c,C::Al,Op::Cmp,R::R0,dst,val,false)}
fn load(c:&mut Code,dst:R,base:R,off:i32)->Result<(),String>{c.emit(transfer_word(C::Al,true,dst,base,off)?)}
fn store(c:&mut Code,src:R,base:R,off:i32)->Result<(),String>{c.emit(transfer_word(C::Al,false,src,base,off)?)}
fn shift(c:&mut Code,dst:R,src:R,kind:Shift,bits:u8)->Result<(),String>{c.emit(mov_shifted_register(C::Al,dst,src,kind,bits)?)}
fn umull(c:&mut Code,lo:R,hi:R,left:R,right:R)->Result<(),String>{
    if lo==hi||[lo,hi,left,right].iter().any(|r|r.number()>=13){return Err("invalid unsigned multiply operands".into());}
    c.emit(0xe080_0090 | (hi.number()<<16) | (lo.number()<<12) | (right.number()<<8) | left.number())
}
fn frame()->u16 {r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|R::R11.bit()|R::LR.bit()}
fn rebuild(c:&mut Code)->Result<(),String>{
    let r3=r(3);let r4=r(4);let r5=r(5);let r6=r(6);let r8=r(8);let r9=r(9);
    c.check(START)?;
    c.emit(push_registers(C::Al,frame())?)?;
    imm(c,C::Al,Op::Add,R::R11,R::SP,24,false)?;
    c.emit(mov_register(C::Al,r4,R::R0))?;
    cmp(c,r4,0)?;
    c.emit(transfer_multiple(C::Eq,true,R::SP,(frame()&!R::LR.bit())|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    cmp(c,R::R1,0)?;
    imm(c,C::Ne,Op::Cmp,R::R0,R::R2,0,false)?;
    c.branch(C::Eq,0x7181c,false)?;
    load(c,r5,r4,0x170)?;
    c.literal(R::R0,0x71820)?;
    load(c,R::R2,R::R2,0x24)?;
    umull(c,R::R1,r3,r5,R::R0)?;
    shift(c,R::R0,r3,Shift::Lsr,2)?;
    rg(c,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?;
    rg(c,Op::Sub,R::R0,r5,R::R0,Shift::Lsl,0,false)?;
    rg(c,Op::Add,r6,r4,R::R0,Shift::Lsl,6,false)?;
    c.emit(transfer_doubleword(C::Al,true,r8,r6,48)?)?;
    umull(c,R::R0,r3,r8,R::R2)?;
    c.emit(multiply_accumulate(C::Al,R::R1,r9,R::R2,r3)?)?;
    rg(c,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,true)?;
    c.branch(C::Eq,0x71808,false)?;
    imm(c,C::Al,Op::Add,R::R0,R::R0,1,true)?;
    c.emit(data_immediate(C::Al,Op::Add,R::R1,R::R1,0)? | (1<<21))?;
    c.branch(C::Al,0x71528,true)?;
    c.literal(R::R1,0x71824)?;
    c.emit(multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    set(c,R::R0,1)?;
    rg(c,Op::Add,R::R0,R::R0,R::R2,Shift::Asr,16,false)?;
    shift(c,R::R1,R::R0,Shift::Asr,31)?;
    c.branch(C::Al,0x71810,false)?;
    c.check(0x71808)?;
    load(c,R::R0,r4,0x150)?;
    set(c,R::R1,0)?;
    c.check(0x71810)?;
    c.emit(transfer_doubleword(C::Al,false,R::R0,r6,32)?)?;
    imm(c,C::Al,Op::Add,R::R0,r5,1,false)?;
    store(c,R::R0,r4,0x170)?;
    c.check(0x7181c)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,(frame()&!R::LR.bit())|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x71820)?;
    for word in [DIVIDE_BY_FIVE_RECIPROCAL,METRIC_OUTPUT_FACTOR]{c.emit(word)?;}
    c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();rebuild(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn occupancy_commit_literals(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        assert_eq!(&b[b.len()-8..b.len()-4],&DIVIDE_BY_FIVE_RECIPROCAL.to_le_bytes());
        assert_eq!(&b[b.len()-4..],&METRIC_OUTPUT_FACTOR.to_le_bytes());
    }
}
