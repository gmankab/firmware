// SPDX-License-Identifier: GPL-3.0-only
//! Codec record occupancy estimator and five-slot 64-bit rolling metrics.
//! Two record layouts (0x2000 and ordinary) use different byte-strides.
//! Every A32 instruction is built from named operands and public ISA fields;
//! three trailing literal words are treated separately as data.
use arm32::{branch, data_immediate, data_immediate_with_flags, data_register,
            mov_register, mov_shifted_register, multiply_register,
            multiply_accumulate, push_registers, transfer_byte,
            transfer_byte_postindexed, transfer_doubleword, transfer_multiple,
            transfer_word, BlockMode, Condition as C, DataOp as Op,
            Register as R, Shift};

pub const START:usize=0x71328;
pub const END:usize=0x71528;
pub const FILE_BASE:usize=0x4000;
const DIVIDE_BY_FIVE_RECIPROCAL:u32=0xcccc_cccd;
const ALLOCATION_DIAGNOSTIC:u32=0x000f_2e4c;
const METRIC_OUTPUT_FACTOR:u32=0x0002_4d77;

struct Code{pc:usize, bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{pc:START,bytes:Vec::new()}}
    fn check(&self,target:usize)->Result<(),String>{
        if self.pc==target{Ok(())}else{Err(format!("record metrics 0x{:x} != 0x{target:x}",self.pc))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.pc < START || self.pc >= END || self.pc%4 != 0 {return Err(format!("metrics output outside range 0x{:x}",self.pc));}
        self.bytes.extend(word.to_le_bytes());self.pc+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.pc as u32,target as u32,link)?)
    }
    fn lit(&mut self,out:R,location:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,out,R::PC,location as i32-self.pc as i32-8)?)
    }
}
fn r(n:u8)->R{R::new(n).expect("valid register index")}
fn i(c:&mut Code,cond:C,op:Op,out:R,base:R,value:u32)->Result<(),String>{
    c.emit(data_immediate(cond,op,out,base,value)?)
}
fn gi(c:&mut Code,op:Op,out:R,base:R,rhs:R,shift:Shift,n:u8,set_flags:bool)->Result<(),String>{
    c.emit(data_register(C::Al,op,out,base,rhs,shift,n,set_flags)?)
}
fn load(c:&mut Code,dst:R,base:R,off:i32)->Result<(),String>{
    c.emit(transfer_word(C::Al,true,dst,base,off)?)
}
fn store(c:&mut Code,src:R,base:R,off:i32)->Result<(),String>{
    c.emit(transfer_word(C::Al,false,src,base,off)?)
}
fn set(c:&mut Code,dst:R,value:u32)->Result<(),String>{i(c,C::Al,Op::Mov,dst,R::R0,value)}
fn cmp(c:&mut Code,base:R,value:u32)->Result<(),String>{i(c,C::Al,Op::Cmp,R::R0,base,value)}
fn regcmp(c:&mut Code,a:R,b:R)->Result<(),String>{gi(c,Op::Cmp,R::R0,a,b,Shift::Lsl,0,false)}
fn mov(c:&mut Code,dst:R,src:R)->Result<(),String>{c.emit(mov_register(C::Al,dst,src))}
fn shift(c:&mut Code,dst:R,src:R,kind:Shift,amount:u8)->Result<(),String>{
    c.emit(mov_shifted_register(C::Al,dst,src,kind,amount)?)
}
fn umull(c:&mut Code,low:R,high:R,left:R,right:R)->Result<(),String>{
    if low==high || [low,high,left,right].iter().any(|r|r.number()>=13){return Err("bad UMULL registers".into());}
    c.emit(0xe080_0090 | (high.number()<<16)|(low.number()<<12)|(right.number()<<8)|left.number())
}
fn adc(c:&mut Code,dst:R,left:R,value:u32)->Result<(),String>{
    // Same encoded modified-immediate fields as ADD, with architectural
    // ADC opcode=5 instead of ADD=4. All operand fields are checked first.
    c.emit(data_immediate(C::Al,Op::Add,dst,left,value)? | (1 << 21))
}
fn inc_flags(c:&mut Code,dst:R,base:R)->Result<(),String>{
    c.emit(data_immediate_with_flags(C::Al,Op::Add,dst,base,1,true)?)
}
fn frame()->u16{(4..=11).map(|n|1_u16<<n).sum::<u16>() | R::LR.bit()}

fn calculate_occupancy(c:&mut Code)->Result<(),String>{
    let r3=r(3);let r4=r(4);let r5=r(5);let r6=r(6);
    let r7=r(7);let r8=r(8);let r9=r(9);let r10=r(10);
    c.check(START)?;
    c.emit(push_registers(C::Al,frame())?)?;
    i(c,C::Al,Op::Add,R::R11,R::SP,28)?;
    i(c,C::Al,Op::Sub,R::SP,R::SP,12)?;
    mov(c,r9,R::R0)?;
    mov(c,r5,R::R1)?;
    cmp(c,r9,0)?;
    c.branch(C::Eq,0x71514,false)?;
    cmp(c,r5,0)?;
    i(c,C::Ne,Op::Cmp,R::R0,R::R2,0)?;
    c.branch(C::Eq,0x71514,false)?;
    load(c,R::R0,r9,0x16c)?;
    c.lit(R::R1,0x7151c)?;
    load(c,r8,R::R2,0x28)?;
    umull(c,r3,r7,R::R0,R::R1)?;
    shift(c,R::R1,r7,Shift::Lsr,2)?;
    load(c,r7,R::R2,0x24)?;
    gi(c,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?;
    gi(c,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?;
    set(c,R::R1,203)?;
    gi(c,Op::Add,R::R0,r9,R::R0,Shift::Lsl,6,false)?;
    i(c,C::Al,Op::Orr,R::R1,R::R1,8448)?;
    c.emit(transfer_byte(C::Al,true,r6,R::R0,0x38)?)?;
    load(c,R::R0,R::R2,0x38)?;
    store(c,R::R0,R::SP,8)?;
    load(c,r10,r5,0x7ec)?;
    load(c,r4,R::R2,0x50)?;
    c.lit(R::R2,0x71520)?;
    cmp(c,r10,0)?;
    mov(c,R::R0,r10)?;
    i(c,C::Ne,Op::Mov,R::R0,R::R0,1)?;
    c.branch(C::Al,0x17b0,true)?;
    load(c,R::R0,r5,4)?;
    gi(c,Op::Add,R::R12,r8,r7,Shift::Lsl,0,false)?;
    cmp(c,R::R0,8192)?;
    c.branch(C::Ne,0x71424,false)?;
    set(c,R::LR,0)?;
    cmp(c,r10,0)?;
    c.branch(C::Eq,0x71468,false)?;
    i(c,C::Al,Op::Add,r8,r4,32)?;
    set(c,r3,0)?;
    set(c,R::R0,0)?;
    set(c,r5,0)?;
    store(c,r6,R::SP,4)?;
    mov(c,R::R2,r8)?;
    set(c,R::R1,0)?;
    c.check(0x713dc)?;
    c.emit(transfer_byte_postindexed(C::Al,true,r7,R::R2,8)?)?;
    inc_flags(c,r4,R::R0)?;
    i(c,C::Al,Op::Add,R::R1,R::R1,1)?;
    adc(c,r6,r5,0)?;
    c.emit(data_immediate(C::Al,Op::And,r7,r7,3)? | (1<<20))?;
    c.emit(mov_register(C::Eq,R::R0,r4))?;
    c.emit(mov_register(C::Eq,r5,r6))?;
    cmp(c,R::R1,16)?;
    c.branch(C::Ne,0x713dc,false)?;
    i(c,C::Al,Op::Add,r3,r3,1)?;
    i(c,C::Al,Op::Add,r8,r8,144)?;
    regcmp(c,r3,r10)?;
    c.branch(C::Ne,0x713d4,false)?;
    load(c,r6,R::SP,4)?;
    shift(c,R::R0,R::R0,Shift::Lsr,4)?;
    shift(c,r3,r5,Shift::Lsr,4)?;
    gi(c,Op::Orr,R::R2,R::R0,r5,Shift::Lsl,28,false)?;
    c.branch(C::Al,0x71470,false)?;
    c.check(0x71424)?;
    cmp(c,r10,0)?;
    c.branch(C::Eq,0x7147c,false)?;
    i(c,C::Al,Op::Add,R::R0,r4,32)?;
    set(c,R::LR,0)?;
    set(c,r7,0)?;
    set(c,R::R2,0)?;
    set(c,r3,0)?;
    c.check(0x71440)?;
    c.emit(transfer_byte_postindexed(C::Al,true,r5,R::R0,64)?)?;
    inc_flags(c,R::R1,R::R2)?;
    i(c,C::Al,Op::Add,r7,r7,1)?;
    adc(c,r4,r3,0)?;
    c.emit(data_immediate(C::Al,Op::And,r5,r5,3)? | (1<<20))?;
    c.emit(mov_register(C::Eq,R::R2,R::R1))?;
    c.emit(mov_register(C::Eq,r3,r4))?;
    regcmp(c,r7,r10)?;
    c.branch(C::Ne,0x71440,false)?;
    c.branch(C::Al,0x71488,false)?;
    c.check(0x71468)?;
    set(c,R::R2,0)?;
    set(c,r3,0)?;
    c.check(0x71470)?;
    load(c,R::R0,R::SP,8)?;
    shift(c,R::R0,R::R0,Shift::Lsr,4)?;
    c.branch(C::Al,0x7148c,false)?;
    c.check(0x7147c)?;
    set(c,R::LR,0)?;
    set(c,R::R2,0)?;
    set(c,r3,0)?;
    c.check(0x71488)?;
    load(c,R::R0,R::SP,8)?;
    c.check(0x7148c)?;
    inc_flags(c,R::R2,R::R2)?;
    adc(c,R::R1,r3,0)?;
    umull(c,r3,r8,r6,R::R12)?;
    inc_flags(c,r7,R::R0)?;
    umull(c,r6,r5,r3,r7)?;
    umull(c,R::R0,r4,r6,R::R2)?;
    c.emit(multiply_accumulate(C::Al,R::R12,r6,R::R1,r4)?)?;
    adc(c,R::R1,R::LR,0)?;
    c.emit(multiply_accumulate(C::Al,r6,r3,R::R1,r5)?)?;
    c.emit(multiply_accumulate(C::Al,r3,r8,r7,r6)?)?;
    c.emit(multiply_accumulate(C::Al,R::R1,r3,R::R2,R::R12)?)?;
    gi(c,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,true)?;
    c.branch(C::Eq,0x714e4,false)?;
    inc_flags(c,R::R0,R::R0)?;
    adc(c,R::R1,R::R1,0)?;
    c.branch(C::Al,0x71528,true)?;
    c.lit(R::R1,0x71524)?;
    c.emit(multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    set(c,R::R0,1)?;
    gi(c,Op::Add,R::R0,R::R0,R::R2,Shift::Asr,16,false)?;
    shift(c,R::R1,R::R0,Shift::Asr,31)?;
    c.branch(C::Al,0x714ec,false)?;
    c.check(0x714e4)?;
    load(c,R::R0,r9,0x150)?;
    set(c,R::R1,0)?;
    c.check(0x714ec)?;
    load(c,R::R2,r9,0x16c)?;
    c.lit(r3,0x7151c)?;
    umull(c,r7,r6,R::R2,r3)?;
    shift(c,r3,r6,Shift::Lsr,2)?;
    gi(c,Op::Add,r3,r3,r3,Shift::Lsl,2,false)?;
    gi(c,Op::Sub,r3,R::R2,r3,Shift::Lsl,0,false)?;
    gi(c,Op::Add,r3,r9,r3,Shift::Lsl,6,false)?;
    c.emit(transfer_doubleword(C::Al,false,R::R0,r3,32)?)?;
    i(c,C::Al,Op::Add,R::R0,R::R2,1)?;
    store(c,R::R0,r9,0x16c)?;
    c.check(0x71514)?;
    i(c,C::Al,Op::Sub,R::SP,R::R11,28)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,
        (frame() & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter,true)?)?;
    c.check(0x7151c)?;
    for literal in [DIVIDE_BY_FIVE_RECIPROCAL,ALLOCATION_DIAGNOSTIC,METRIC_OUTPUT_FACTOR] {
        c.emit(literal)?;
    }
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();calculate_occupancy(&mut c)?;Ok(c.bytes)
}

#[cfg(test)] mod tests{
    use super::*;
    #[test] fn rolling_metrics_three_literals(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        for (i,w) in [DIVIDE_BY_FIVE_RECIPROCAL,ALLOCATION_DIAGNOSTIC,METRIC_OUTPUT_FACTOR]
            .into_iter().enumerate() {
            let off=0x7151c-START+4*i;
            assert_eq!(&bytes[off..off+4],&w.to_le_bytes());
        }
    }
}
