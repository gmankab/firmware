// SPDX-License-Identifier: GPL-3.0-only
//! Copy a six-component descriptor into the current five-slot codec ring.
//! The source's 64-bit fields are initialized from control defaults when
//! unset; four diagnostic and modulo-five constants follow the ARM return.
use arm32::{branch, data_immediate, data_register, mov_register, mov_shifted_register,
    push_registers, transfer_byte, transfer_doubleword, transfer_multiple,
    transfer_word, transfer_word_preindexed_writeback, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x71828;
pub const END:usize=0x71978;
pub const FILE_BASE:usize=0x4000;
const DESCRIPTOR_LOG:u32=0x000f_2e4c;
const DETAILS_LOG:u32=0x000f_303c;
const TRACE_FOOTER:u32=0x000e_2788;
const MODULO_FIVE_RECIPROCAL:u32=0xcccc_cccd;

struct Code{at:usize, bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,target:usize)->Result<(),String>{if self.at==target{Ok(())}else{Err(format!("record-append 0x{:x} != 0x{target:x}",self.at))}}
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("invalid A32 at 0x{:x}",self.at));}
        self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,c:C,target:usize,link:bool)->Result<(),String>{self.emit(branch(c,self.at as u32,target as u32,link)?)}
    fn lit(&mut self,reg:R,target:usize)->Result<(),String>{self.emit(transfer_word(C::Al,true,reg,R::PC,target as i32-self.at as i32-8)?)}
}
fn r(n:u8)->R{R::new(n).unwrap()}
fn imm(c:&mut Code,cond:C,op:Op,dst:R,src:R,v:u32)->Result<(),String>{c.emit(data_immediate(cond,op,dst,src,v)?)}
fn rg(c:&mut Code,op:Op,dst:R,src:R,rhs:R,kind:Shift,amount:u8,flags:bool)->Result<(),String>{
    c.emit(data_register(C::Al,op,dst,src,rhs,kind,amount,flags)?)
}
fn set(c:&mut Code,dst:R,val:u32)->Result<(),String>{imm(c,C::Al,Op::Mov,dst,R::R0,val)}
fn cmp(c:&mut Code,src:R,val:u32)->Result<(),String>{imm(c,C::Al,Op::Cmp,R::R0,src,val)}
fn mov(c:&mut Code,dst:R,src:R)->Result<(),String>{c.emit(mov_register(C::Al,dst,src))}
fn load(c:&mut Code,dst:R,base:R,off:i32)->Result<(),String>{c.emit(transfer_word(C::Al,true,dst,base,off)?)}
fn store(c:&mut Code,src:R,base:R,off:i32)->Result<(),String>{c.emit(transfer_word(C::Al,false,src,base,off)?)}
fn umull(c:&mut Code,lo:R,hi:R,left:R,right:R)->Result<(),String>{
    if lo==hi||[lo,hi,left,right].iter().any(|r|r.number()>=13){return Err("invalid 64-bit multiply registers".into());}
    c.emit(0xe080_0090|(hi.number()<<16)|(lo.number()<<12)|(right.number()<<8)|left.number())
}
fn saved()->u16 {r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|R::R11.bit()|R::LR.bit()}
fn copy_descriptor(c:&mut Code)->Result<(),String>{
    let r3=r(3);let r4=r(4);let r5=r(5);let r6=r(6);let r7=r(7);let r8=r(8);
    c.check(START)?;
    c.emit(push_registers(C::Al,saved())?)?;
    imm(c,C::Al,Op::Add,R::R11,R::SP,24)?;
    imm(c,C::Al,Op::Sub,R::SP,R::SP,16)?;
    mov(c,r5,R::R1)?;
    mov(c,r4,R::R0)?;
    mov(c,r6,R::R2)?;
    c.lit(R::R2,0x71968)?;
    cmp(c,r5,0)?;
    mov(c,R::R0,r5)?;
    mov(c,R::R1,r4)?;
    imm(c,C::Ne,Op::Mov,R::R0,R::R0,1)?;
    cmp(c,r4,0)?;
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,1)?;
    rg(c,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?;
    set(c,R::R1,101)?;
    imm(c,C::Al,Op::Orr,R::R1,R::R1,8704)?;
    c.branch(C::Al,0x17b0,true)?;
    load(c,r3,r6,0)?;
    load(c,r7,r4,0x168)?;
    c.emit(transfer_multiple(C::Al,true,r5,R::R0.bit()|R::R1.bit()|R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    store(c,R::R1,R::SP,0)?;
    c.lit(R::R1,0x7196c)?;
    c.emit(transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    set(c,R::R0,4)?;
    mov(c,R::R2,r7)?;
    c.branch(C::Al,0x2ec94,true)?;
    c.lit(R::R1,0x71970)?;
    set(c,R::R0,4)?;
    c.branch(C::Al,0x2ec94,true)?;
    c.lit(R::R0,0x71974)?;
    umull(c,R::R1,R::R2,r7,R::R0)?;
    c.emit(mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsr,2)?)?;
    load(c,R::R1,r6,0)?;
    rg(c,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?;
    rg(c,Op::Sub,R::R2,r7,R::R0,Shift::Lsl,0,false)?;
    set(c,r7,0)?;
    rg(c,Op::Add,R::R0,r4,R::R2,Shift::Lsl,6,false)?;
    c.emit(transfer_byte(C::Al,false,R::R1,R::R0,0x39)?)?;
    load(c,r6,r5,8)?;
    c.emit(transfer_word_register(C::Al,false,r6,r4,R::R2,Shift::Lsl,6)?)?;
    store(c,r7,R::R0,4)?;
    cmp(c,r6,0)?;
    load(c,r3,r5,4)?;
    store(c,r7,R::R0,12)?;
    store(c,r3,R::R0,8)?;
    load(c,R::R2,r5,12)?;
    store(c,r7,R::R0,20)?;
    store(c,R::R2,R::R0,16)?;
    load(c,r8,r4,0x150)?;
    store(c,r8,R::R0,32)?;
    store(c,r7,R::R0,36)?;
    c.emit(transfer_word(C::Eq,true,r6,r4,0x140)?)?;
    c.emit(transfer_doubleword(C::Eq,false,r6,R::R0,0)?)?;
    mov(c,r7,R::R0)?;
    c.emit(transfer_word_preindexed_writeback(C::Al,true,r6,r7,24)?)?;
    load(c,r5,r7,4)?;
    rg(c,Op::Orr,r6,r6,r5,Shift::Lsl,0,true)?;
    c.branch(C::Ne,0x71920,false)?;
    load(c,r6,r4,0x14c)?;
    set(c,r5,0)?;
    store(c,r6,r7,0)?;
    store(c,r5,r7,4)?;
    c.check(0x71920)?;
    cmp(c,r3,0)?;
    c.branch(C::Ne,0x71938,false)?;
    load(c,r6,r4,0x144)?;
    imm(c,C::Al,Op::Add,r3,R::R0,8)?;
    set(c,r7,0)?;
    c.emit(transfer_doubleword(C::Al,false,r6,r3,0)?)?;
    c.check(0x71938)?;
    cmp(c,R::R2,0)?;
    c.branch(C::Ne,0x71950,false)?;
    load(c,r6,r4,0x148)?;
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    set(c,r7,0)?;
    c.emit(transfer_doubleword(C::Al,false,r6,R::R2,0)?)?;
    c.check(0x71950)?;
    c.emit(transfer_byte(C::Al,false,R::R1,R::R0,0x38)?)?;
    load(c,R::R0,r4,0x168)?;
    imm(c,C::Al,Op::Add,R::R0,R::R0,1)?;
    store(c,R::R0,r4,0x168)?;
    imm(c,C::Al,Op::Sub,R::SP,R::R11,24)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,(saved()&!R::LR.bit())|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x71968)?;
    for word in [DESCRIPTOR_LOG,DETAILS_LOG,TRACE_FOOTER,MODULO_FIVE_RECIPROCAL]{c.emit(word)?;}
    c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();copy_descriptor(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn descriptor_append_literals(){
        let b=fragment().unwrap(); assert_eq!(b.len(),END-START);
        for (i,word) in [DESCRIPTOR_LOG,DETAILS_LOG,TRACE_FOOTER,MODULO_FIVE_RECIPROCAL].into_iter().enumerate(){
            let off=b.len()-16+i*4;assert_eq!(&b[off..off+4],&word.to_le_bytes());
        }
    }
}
