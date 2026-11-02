// SPDX-License-Identifier: GPL-3.0-only
//! Convert one indexed Venus controller color record to six output halfwords.
//! Two table-layout paths share the final three channels. Source is typed
//! ARM32 encoding, with the table pointers modeled as non-executable data.
use arm32::{branch, data_immediate, data_register, load_halfword, mov_register,
    mov_shifted_register, push_registers, store_halfword, transfer_halfword_register,
    transfer_multiple, transfer_word, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x71e14;
pub const END:usize=0x71ed0;
pub const FILE_BASE:usize=0x4000;
const TABLE_SINGLE:u32=0x0010_5f60;
const TABLE_CLASSES:u32=0x000f_2e40;

struct A32 { at:usize, bytes:Vec<u8> }
impl A32 {
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START || self.at>=END || self.at%4!=0{return Err(format!("invalid ARM position 0x{:x}",self.at));}
        self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
    fn check(&self,want:usize)->Result<(),String>{
        if self.at!=want{Err(format!("0x{:x} != expected 0x{want:x}",self.at))}else{Ok(())}
    }
    fn b(&mut self,cond:C,target:usize)->Result<(),String>{self.emit(branch(cond,self.at as u32,target as u32,false)?)}
    fn lit(&mut self,dst:R,target:usize)->Result<(),String>{self.emit(transfer_word(C::Al,true,dst,R::PC,(target as isize-self.at as isize-8) as i32)?)}
}
fn r(n:u8)->R{R::new(n).expect("known register index")}
fn imm(c:&mut A32,op:Op,out:R,left:R,v:u32)->Result<(),String>{c.emit(data_immediate(C::Al,op,out,left,v)?)}
fn reg(c:&mut A32,op:Op,out:R,left:R,right:R,shift:Shift,bits:u8)->Result<(),String>{
    c.emit(data_register(C::Al,op,out,left,right,shift,bits,false)?)
}
fn half(c:&mut A32,out:R,base:R,off:i32)->Result<(),String>{c.emit(load_halfword(C::Al,out,base,off)?)}
fn halfreg(c:&mut A32,out:R,base:R,index:R)->Result<(),String>{c.emit(transfer_halfword_register(C::Al,true,out,base,index)?)}
fn halfreg_update(c:&mut A32,out:R,base:R,index:R)->Result<(),String>{
    if out==base || [out,base,index].contains(&R::PC){return Err("invalid A32 halfword base update".into());}
    c.emit(transfer_halfword_register(C::Al,true,out,base,index)?|(1<<21))
}
fn store(c:&mut A32,value:R,dst:R,off:i32)->Result<(),String>{c.emit(store_halfword(C::Al,value,dst,off)?)}
fn mov(c:&mut A32,out:R,from:R)->Result<(),String>{c.emit(mov_register(C::Al,out,from))}
fn shift(c:&mut A32,out:R,from:R,kind:Shift,bits:u8)->Result<(),String>{
    c.emit(mov_shifted_register(C::Al,out,from,kind,bits)?)
}

fn emit_record_unpack(c:&mut A32)->Result<(),String>{
    let r2=R::R2;let r3=r(3);
    c.check(START)?;
    c.emit(push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    mov(c,R::R11,R::SP)?;
    imm(c,Op::Sub,r2,r2,1)?;
    imm(c,Op::And,r3,r2,255)?;
    imm(c,Op::Cmp,R::R0,r3,3)?;
    c.b(C::Cs,0x71e70)?;
    c.lit(r3,0x71ecc)?;
    shift(c,r2,r2,Shift::Lsl,24)?;
    shift(c,r2,r2,Shift::Asr,24)?;
    c.emit(transfer_word_register(C::Al,true,R::LR,r3,r2,Shift::Lsl,2)?)?;
    shift(c,r2,R::R1,Shift::Lsl,1)?;
    mov(c,r3,R::LR)?;
    halfreg_update(c,r2,r3,r2)?;
    store(c,r2,R::R0,0)?;
    imm(c,Op::Mov,r2,R::R0,256)?;
    imm(c,Op::Add,R::R12,r3,1024)?;
    halfreg(c,r2,r3,r2)?;
    store(c,r2,R::R0,2)?;
    imm(c,Op::Mov,r2,R::R0,512)?;
    halfreg(c,r2,r3,r2)?;
    imm(c,Op::Add,r3,r3,768)?;
    store(c,r2,R::R0,4)?;
    c.b(C::Al,0x71ea4)?;
    c.check(0x71e70)?;
    c.lit(R::LR,0x71ec8)?;
    shift(c,r3,R::R1,Shift::Lsl,1)?;
    mov(c,r2,R::LR)?;
    halfreg_update(c,r3,r2,r3)?;
    store(c,r3,R::R0,0)?;
    imm(c,Op::Mov,r3,R::R0,256)?;
    imm(c,Op::Add,R::R12,r2,1024)?;
    halfreg(c,r3,r2,r3)?;
    store(c,r3,R::R0,2)?;
    imm(c,Op::Mov,r3,R::R0,512)?;
    halfreg(c,r3,r2,r3)?;
    store(c,r3,R::R0,4)?;
    imm(c,Op::Add,r3,r2,768)?;
    c.check(0x71ea4)?;
    half(c,r2,r3,0)?;
    reg(c,Op::Add,R::R1,R::LR,R::R1,Shift::Lsl,1)?;
    store(c,r2,R::R0,6)?;
    half(c,r2,R::R12,0)?;
    store(c,r2,R::R0,8)?;
    imm(c,Op::Mov,r2,R::R0,1280)?;
    halfreg(c,R::R1,R::R1,r2)?;
    store(c,R::R1,R::R0,10)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x71ec8)?;
    c.emit(TABLE_SINGLE)?;
    c.emit(TABLE_CLASSES)?;
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=A32::new();emit_record_unpack(&mut c)?;Ok(c.bytes)}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn six_component_table_unpack(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        assert_eq!(&b[b.len()-8..b.len()-4],&TABLE_SINGLE.to_le_bytes());
        assert_eq!(&b[b.len()-4..],&TABLE_CLASSES.to_le_bytes());
    }
}
