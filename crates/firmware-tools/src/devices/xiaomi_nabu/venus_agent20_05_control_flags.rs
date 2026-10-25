// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-05: three small, standalone Venus descriptor-dirty flag helpers.
//! Located in the agent's exclusive ARM range but disjoint from the upper
//! descriptor routines. Each operation is emitted from ARM operands.
use arm32::{branch, bx, data_immediate, data_register, mov_register,
            mov_shifted_register, push_registers, transfer_multiple,
            transfer_word, transfer_word_postindexed,
            BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x6b860;
pub const END:usize=0x6b8f4;
pub const FILE_BASE:usize=0x4000;

struct Code{pc:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{pc:START,bytes:Vec::new()}}
    fn check(&self,want:usize)->Result<(),String>{
        if self.pc!=want{Err(format!("0x{:x} != expected 0x{want:x}",self.pc))}else{Ok(())}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.pc>=END || self.pc%4!=0 {return Err(format!("agent05 control flags out of range 0x{:x}",self.pc));}
        self.bytes.extend(word.to_le_bytes());self.pc+=4;Ok(())
    }
    fn b(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(branch(cond,self.pc as u32,target as u32,false)?)
    }
}
fn r(n:u8)->Result<R,String>{R::new(n)}
fn i(c:&mut Code,cond:C,op:Op,dst:R,src:R,val:u32)->Result<(),String>{
    c.emit(data_immediate(cond,op,dst,src,val)?)
}
fn g(c:&mut Code,cond:C,op:Op,dst:R,src:R,rhs:R)->Result<(),String>{
    c.emit(data_register(cond,op,dst,src,rhs,Shift::Lsl,0,false)?)
}
fn mem(c:&mut Code,cond:C,read:bool,dst:R,base:R,off:i32)->Result<(),String>{
    c.emit(transfer_word(cond,read,dst,base,off)?)
}

/// Copy exactly six controller words into a descriptor. Signal dirty-bit 0
/// if any stored word changes, preserving the original previous contents.
fn copy_six_and_mark_dirty(c:&mut Code)->Result<(),String>{
    let r3=r(3)?;let r2=R::R2;
    c.check(START)?;
    c.emit(push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    c.emit(mov_register(C::Al,R::R11,R::SP))?;
    i(c,C::Al,Op::Add,r2,R::R1,8)?;
    i(c,C::Al,Op::Mov,R::LR,R::R0,0)?;
    c.check(0x6b870)?;
    c.emit(transfer_word_postindexed(C::Al,true,R::R12,R::R0,4)?)?;
    mem(c,C::Al,true,r3,r2,0)?;
    i(c,C::Al,Op::Add,R::LR,R::LR,1)?;
    g(c,C::Al,Op::Cmp,R::R0,r3,R::R12)?;
    mem(c,C::Ne,false,R::R12,r2,0)?;
    i(c,C::Al,Op::Add,r2,r2,4)?;
    mem(c,C::Ne,true,r3,R::R1,0)?;
    i(c,C::Ne,Op::Orr,r3,r3,1)?;
    mem(c,C::Ne,false,r3,R::R1,0)?;
    i(c,C::Al,Op::Cmp,R::R0,R::LR,6)?;
    c.emit(transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),
                             BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x6b870)?;
    c.check(0x6b8a0)
}

/// Update the 16KB granular allocation count and mark dirty-bit 1 if the
/// stored count changed. Small allocations use the minimum count of four.
fn update_allocation_quanta(c:&mut Code)->Result<(),String>{
    c.check(0x6b8a0)?;
    c.emit(mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsr,14)?)?;
    i(c,C::Al,Op::Cmp,R::R0,R::R0,65536)?;
    mem(c,C::Al,true,R::R0,R::R1,0x24)?;
    i(c,C::Cc,Op::Mov,R::R2,R::R0,4)?;
    g(c,C::Al,Op::Cmp,R::R0,R::R0,R::R2)?;
    mem(c,C::Ne,false,R::R2,R::R1,0x24)?;
    mem(c,C::Ne,true,R::R0,R::R1,0)?;
    i(c,C::Ne,Op::Orr,R::R0,R::R0,2)?;
    mem(c,C::Ne,false,R::R0,R::R1,0)?;
    c.emit(bx(C::Al,R::LR))?;
    c.check(0x6b8c8)
}

/// Replace the pair of controller offsets and mark dirty-bit 2 only when
/// either value differs from the old pair.
fn update_controller_offsets(c:&mut Code)->Result<(),String>{
    let r3=r(3)?;
    c.check(0x6b8c8)?;
    mem(c,C::Al,true,r3,R::R2,0x2c)?;
    g(c,C::Al,Op::Cmp,R::R0,r3,R::R0)?;
    mem(c,C::Eq,true,r3,R::R2,0x30)?;
    g(c,C::Eq,Op::Cmp,R::R0,r3,R::R1)?;
    c.emit(bx(C::Eq,R::LR))?;
    mem(c,C::Al,false,R::R0,R::R2,0x2c)?;
    mem(c,C::Al,false,R::R1,R::R2,0x30)?;
    mem(c,C::Al,true,R::R0,R::R2,0)?;
    i(c,C::Al,Op::Orr,R::R0,R::R0,4)?;
    mem(c,C::Al,false,R::R0,R::R2,0)?;
    c.emit(bx(C::Al,R::LR))?;
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();
    copy_six_and_mark_dirty(&mut c)?;
    update_allocation_quanta(&mut c)?;
    update_controller_offsets(&mut c)?;
    Ok(c.bytes)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn three_helpers_have_exact_layout(){
        assert_eq!(fragment().unwrap().len(),END-START);
    }
}
