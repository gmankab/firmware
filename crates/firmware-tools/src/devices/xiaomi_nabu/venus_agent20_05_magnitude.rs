// SPDX-License-Identifier: GPL-3.0-only
//! Convert a 64-bit accumulated codec metric into an integer magnitude tier.
//! Each of 10^9, 10^7, 10^4 and 10^2 is subtracted conditionally before
//! returning a compact multi-bit bucket index. Literal thresholds are data.
use arm32::{branch, data_immediate, data_immediate_with_flags, data_register,
    mov_register, push_registers, transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x71528;
pub const END:usize=0x715ec;
pub const FILE_BASE:usize=0x4000;
const HIGH_THRESHOLD:u32=0x0023_86f2;
const TEN_MILLION:u32=0x05f5_e100;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,want:usize)->Result<(),String>{
        if self.at==want{Ok(())}else{Err(format!("metric magnitude position 0x{:x} != 0x{want:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("invalid magnitude code at 0x{:x}",self.at));}
        self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
    fn b(&mut self,cond:C,dest:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,dest as u32,link)?)
    }
    fn lit(&mut self,out:R,loc:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,out,R::PC,loc as i32-self.at as i32-8)?)
    }
}
fn imm(c:&mut Code,cond:C,op:Op,dst:R,src:R,val:u32,flags:bool)->Result<(),String>{
    if flags {c.emit(data_immediate_with_flags(cond,op,dst,src,val,true)?)}
    else {c.emit(data_immediate(cond,op,dst,src,val)?)}
}
fn rg(c:&mut Code,cond:C,op:Op,out:R,base:R,rhs:R,flags:bool)->Result<(),String>{
    c.emit(data_register(cond,op,out,base,rhs,Shift::Lsl,0,flags)?)
}
fn mov(c:&mut Code,dst:R,src:R)->Result<(),String>{c.emit(mov_register(C::Al,dst,src))}
fn sub64_threshold(c:&mut Code,output_low:R,source_low:R,source_high:R,limit_low:R,limit_high:R)->Result<(),String>{
    rg(c,C::Al,Op::Sub,output_low,source_low,limit_low,true)?;
    rg(c,C::Al,Op::Sbc,R::R2,source_high,limit_high,true)
}
fn return_regs()->u16{
    R::new(4).unwrap().bit()|R::new(7).unwrap().bit()|R::R11.bit()|R::PC.bit()
}
fn bucketize(c:&mut Code)->Result<(),String>{
    let r3=R::new(3)?;let r4=R::new(4)?;
    c.check(START)?;
    c.emit(push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    imm(c,C::Al,Op::Add,R::R11,R::SP,8,false)?;
    rg(c,C::Al,Op::Orr,R::R2,R::R0,R::R1,true)?;
    imm(c,C::Al,Op::Mov,r3,R::R0,12648448,false)?;
    imm(c,C::Al,Op::Mov,r4,R::R0,0,false)?;
    c.lit(R::R2,0x715e4)?;
    imm(c,C::Al,Op::Orr,r3,r3,1862270976,false)?;
    imm(c,C::Eq,Op::Mvn,r4,R::R0,0,false)?;
    sub64_threshold(c,r3,R::R0,R::R1,r3,R::R2)?;
    c.b(C::Cc,0x7156c,false)?;
    c.lit(r3,0x715e4)?;
    imm(c,C::Al,Op::Mov,R::R2,R::R0,12648448,false)?;
    imm(c,C::Al,Op::Orr,R::R2,R::R2,1862270976,false)?;
    c.b(C::Al,0x8187c,true)?;
    imm(c,C::Al,Op::Add,r4,r4,16,false)?;
    c.b(C::Al,0x715b4,false)?;
    c.check(0x7156c)?;
    c.lit(R::R2,0x715e8)?;
    rg(c,C::Al,Op::Sub,R::R2,R::R0,R::R2,true)?;
    imm(c,C::Al,Op::Sbc,R::R2,R::R1,0,true)?;
    c.b(C::Cc,0x7158c,false)?;
    c.lit(R::R2,0x715e8)?;
    imm(c,C::Al,Op::Mov,r3,R::R0,0,false)?;
    c.b(C::Al,0x8187c,true)?;
    imm(c,C::Al,Op::Add,r4,r4,8,false)?;
    c.check(0x7158c)?;
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1808,false)?;
    imm(c,C::Al,Op::Orr,R::R2,R::R2,8192,false)?;
    rg(c,C::Al,Op::Sub,R::R2,R::R0,R::R2,true)?;
    imm(c,C::Al,Op::Sbc,R::R2,R::R1,0,true)?;
    c.b(C::Cc,0x715b4,false)?;
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1808,false)?;
    imm(c,C::Al,Op::Mov,r3,R::R0,0,false)?;
    imm(c,C::Al,Op::Orr,R::R2,R::R2,8192,false)?;
    c.b(C::Al,0x8187c,true)?;
    imm(c,C::Al,Op::Add,r4,r4,4,false)?;
    c.check(0x715b4)?;
    imm(c,C::Al,Op::Sub,R::R2,R::R0,100,true)?;
    imm(c,C::Al,Op::Sbc,R::R2,R::R1,0,true)?;
    c.b(C::Cc,0x715d0,false)?;
    imm(c,C::Al,Op::Mov,R::R2,R::R0,100,false)?;
    imm(c,C::Al,Op::Mov,r3,R::R0,0,false)?;
    c.b(C::Al,0x8187c,true)?;
    imm(c,C::Al,Op::Add,r4,r4,2,false)?;
    c.check(0x715d0)?;
    imm(c,C::Al,Op::Rsb,R::R0,R::R0,9,true)?;
    imm(c,C::Al,Op::Rsc,R::R0,R::R1,0,true)?;
    imm(c,C::Cc,Op::Add,r4,r4,1,false)?;
    mov(c,R::R0,r4)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,return_regs(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x715e4)?;
    for word in [HIGH_THRESHOLD,TEN_MILLION]{c.emit(word)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();bucketize(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn magnitude_thresholds_are_separate_data(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        assert_eq!(&b[b.len()-8..b.len()-4],&HIGH_THRESHOLD.to_le_bytes());
        assert_eq!(&b[b.len()-4..],&TEN_MILLION.to_le_bytes());
    }
}
