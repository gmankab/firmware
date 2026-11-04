// SPDX-License-Identifier: GPL-3.0-only
//! Typed A32 source for the initial part of an upper Venus controller routine.
//! This interval is a verified function prefix, **not** complete executable
//! firmware. Execution continues after END; external literals are unowned.
use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_register, mov_register,
    mov_shifted_register, multiply_register, push_registers, store_halfword,
    transfer_doubleword, transfer_multiple, BlockMode,
    signed_multiply_accumulate_bottom_halves, transfer_byte, transfer_byte_register,
    transfer_word, transfer_word_register, transfer_word_register_preindexed_writeback,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START:usize=0x69e44;
pub const END:usize=0x6a1d4;
pub const FILE_MAPPING_DELTA:usize=0x4000;

struct Emitter { pc:usize, bytes:Vec<u8> }
impl Emitter {
    fn new()->Self { Self { pc:START, bytes:Vec::with_capacity(END-START) } }
    fn emit(&mut self,code:u32)->Result<(),String> {
        if self.pc>=END || self.pc&3!=0 {
            return Err(format!("Venus dispatch out of 0x{START:x}..0x{END:x}: 0x{:x}",self.pc));
        }
        self.bytes.extend_from_slice(&code.to_le_bytes());
        self.pc+=4;
        Ok(())
    }
    fn check(&self,wanted:usize)->Result<(),String> {
        if self.pc!=wanted { return Err(format!("Venus cursor 0x{:x}, expected 0x{wanted:x}",self.pc)); }
        Ok(())
    }
    fn imm(&mut self,condition:C,op:Op,rd:R,rn:R,value:u32)->Result<(),String> {
        self.emit(data_immediate(condition,op,rd,rn,value)?)
    }
    fn mov(&mut self,condition:C,rd:R,rm:R)->Result<(),String> {
        self.emit(mov_register(condition,rd,rm))
    }
    fn ldr(&mut self,rd:R,rn:R,offset:i32)->Result<(),String> {
        self.emit(transfer_word(C::Al,true,rd,rn,offset)?)
    }
    fn str(&mut self,rd:R,rn:R,offset:i32)->Result<(),String> {
        self.emit(transfer_word(C::Al,false,rd,rn,offset)?)
    }
    fn ldrb(&mut self,rd:R,rn:R,offset:i32)->Result<(),String> {
        self.emit(transfer_byte(C::Al,true,rd,rn,offset)?)
    }
    fn branch(&mut self,condition:C,target:usize,link:bool)->Result<(),String> {
        self.emit(branch(condition,self.pc as u32,target as u32,link)?)
    }
}

pub fn fragment()->Result<Vec<u8>,String> {
    let mut c=Emitter::new();
    let r4=R::new(4)?; let r5=R::new(5)?; let r6=R::new(6)?;
    let r7=R::new(7)?; let r8=R::new(8)?; let r9=R::new(9)?;
    let r10=R::new(10)?;
    let preserved=(4..=11).try_fold(R::LR.bit(),|mask,n|{
        Ok::<u16,String>(mask|R::new(n)?.bit())
    })?;
    c.emit(push_registers(C::Al,preserved)?)?;
    c.imm(C::Al,Op::Add,R::R11,R::SP,28)?;
    c.imm(C::Al,Op::Sub,R::SP,R::SP,172)?;
    c.imm(C::Al,Op::Sub,R::SP,R::SP,1024)?;
    c.mov(C::Al,r6,R::R0)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,2864)?;
    c.mov(C::Al,r8,R::R1)?;
    c.ldr(R::R2,R::PC,0xd8c)?;
    c.imm(C::Al,Op::Orr,R::R0,R::R0,4096)?;
    c.mov(C::Al,r5,r6)?;
    c.mov(C::Al,r4,r6)?;
    c.emit(transfer_word_register_preindexed_writeback(
        C::Al,true,R::R0,r5,R::R0,Shift::Lsl,0)?)?;
    c.str(R::R0,R::SP,0xa4)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,728)?;
    c.imm(C::Al,Op::Orr,R::R0,R::R0,4096)?;
    c.emit(transfer_word_register_preindexed_writeback(
        C::Al,true,R::R1,r4,R::R0,Shift::Lsl,0)?)?;
    c.check(0x69e84)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,0)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R1,10)?;
    c.ldrb(R::R1,r6,0x1c)?;
    c.imm(C::Cc,Op::Mov,R::R0,R::R0,1)?;
    c.str(R::R1,R::SP,0x30)?;
    c.ldrb(R::R1,r6,0x1d)?;
    c.str(R::R1,R::SP,0x2c)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,10)?;
    c.ldrb(r10,r6,0x2b)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,2048)?;
    c.check(0x69eac)?;
    c.branch(C::Al,0x17b0,true)?;
    c.ldr(R::R0,r4,0)?;
    c.mov(C::Al,r7,r6)?;
    c.str(r4,R::SP,0x34)?;
    c.emit(data_register(C::Al,Op::Add,r9,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    c.ldr(R::R0,r7,0)?;
    c.emit(data_register(C::Al,Op::Add,R::R1,r4,r9,Shift::Lsl,6,false)?)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R0,8192)?;
    c.branch(C::Eq,0x69ee4,false)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R0,4096)?;
    c.branch(C::Eq,0x69ef8,false)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R0,2)?;
    c.imm(C::Eq,Op::Mov,r10,R::R0,51)?;
    c.branch(C::Al,0x69efc,false)?;
    c.check(0x69ee4)?;
    c.ldrb(R::R2,r7,0x7e)?;
    c.imm(C::Al,Op::Mov,r10,R::R0,51)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R2,0)?;
    c.imm(C::Ne,Op::Mov,r10,R::R0,63)?;
    c.branch(C::Al,0x69efc,false)?;
    c.imm(C::Al,Op::Mov,r10,R::R0,127)?;
    c.check(0x69efc)?;
    c.str(r10,R::SP,0x6c)?;
    c.imm(C::Al,Op::Add,r4,R::R1,16)?;
    c.ldr(R::R2,r7,12)?;
    c.ldrb(R::R1,r8,0)?;
    c.str(r7,R::SP,0x70)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R2,0)?;
    c.branch(C::Eq,0x69f64,false)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R1,2)?;
    c.branch(C::Eq,0x69f94,false)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R1,1)?;
    c.branch(C::Eq,0x69fdc,false)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R1,0)?;
    c.branch(C::Ne,0x6a014,false)?;
    c.check(0x69f30)?;
    // Mode-zero controller record addresses.
    c.imm(C::Al,Op::Mov,R::R1,R::R0,0)?;
    c.ldrb(R::R0,r8,8)?;
    c.str(R::R1,r4,0)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,948)?;
    c.ldr(R::new(3)?,R::SP,0xa4)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,1024)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al,
        R::R1,R::R0,R::R1,R::new(3)?)?)?;
    c.imm(C::Al,Op::Add,R::R1,R::R1,524)?;
    c.imm(C::Al,Op::Add,R::R2,R::R1,1024)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,232)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al,
        R::R0,R::R0,R::R1,R::new(3)?)?)?;
    c.imm(C::Al,Op::Add,r6,R::R0,13376)?;
    c.branch(C::Al,0x6a078,false)?;
    c.check(0x69f64)?;
    // Alternate mode switch using the input selector.
    c.imm(C::Al,Op::Cmp,R::R0,R::R1,2)?;
    c.branch(C::Eq,0x6a028,false)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R1,1)?;
    c.branch(C::Eq,0x6a040,false)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R1,0)?;
    c.branch(C::Ne,0x6a058,false)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,0)?;
    c.imm(C::Al,Op::Add,r6,r7,784)?;
    c.str(R::R0,r4,0)?;
    c.imm(C::Al,Op::Add,R::R0,r7,644)?;
    c.imm(C::Al,Op::Add,R::R2,R::R0,2048)?;
    c.branch(C::Al,0x6a078,false)?;
    c.check(0x69f94)?;
    c.imm(C::Al,Op::Mov,r6,R::R0,0)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R0,8192)?;
    c.imm(C::Al,Op::Mov,R::R2,R::R0,0)?;
    c.branch(C::Ne,0x6a078,false)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,2)?;
    c.ldrb(R::R0,r8,8)?;
    c.str(R::R1,r4,0)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,232)?;
    c.ldr(R::R2,R::SP,0xa4)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al,
        R::R1,R::R0,R::R1,R::R2)?)?;
    c.imm(C::Al,Op::Add,R::R1,R::R1,3872)?;
    c.imm(C::Al,Op::Add,r6,R::R1,12288)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,948)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,1024)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al,
        R::R0,R::R0,R::R1,R::R2)?)?;
    c.imm(C::Al,Op::Add,R::R0,R::R0,812)?;
    c.imm(C::Al,Op::Add,R::R2,R::R0,2048)?;
    c.branch(C::Al,0x6a078,false)?;
    c.check(0x69fdc)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,1)?;
    c.ldrb(R::R0,r8,8)?;
    c.str(R::R1,r4,0)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,232)?;
    c.ldr(R::R2,R::SP,0xa4)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al,
        R::R1,R::R0,R::R1,R::R2)?)?;
    c.imm(C::Al,Op::Add,R::R1,R::R1,2480)?;
    c.imm(C::Al,Op::Add,r6,R::R1,12288)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,948)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,1024)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al,
        R::R0,R::R0,R::R1,R::R2)?)?;
    c.imm(C::Al,Op::Add,R::R0,R::R0,156)?;
    c.imm(C::Al,Op::Add,R::R2,R::R0,2048)?;
    c.branch(C::Al,0x6a078,false)?;
    c.check(0x6a014)?;
    c.ldr(R::R2,R::PC,0xbd8)?;
    c.imm(C::Al,Op::Mov,r6,R::R0,0)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,0)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,2112)?;
    c.branch(C::Al,0x6a06c,false)?;
    c.check(0x6a028)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,2)?;
    c.imm(C::Al,Op::Add,r6,r7,1248)?;
    c.str(R::R0,r4,0)?;
    c.imm(C::Al,Op::Add,R::R0,r7,932)?;
    c.imm(C::Al,Op::Add,R::R2,R::R0,3072)?;
    c.branch(C::Al,0x6a078,false)?;
    c.check(0x6a040)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,1)?;
    c.imm(C::Al,Op::Add,r6,r7,1016)?;
    c.str(R::R0,r4,0)?;
    c.imm(C::Al,Op::Add,R::R0,r7,276)?;
    c.imm(C::Al,Op::Add,R::R2,R::R0,3072)?;
    c.branch(C::Al,0x6a078,false)?;
    c.check(0x6a058)?;
    c.ldr(R::R2,R::PC,0xb94)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,97)?;
    c.imm(C::Al,Op::Mov,r6,R::R0,0)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,0)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,2048)?;
    c.branch(C::Al,0x17b0,true)?;
    c.ldr(r7,R::SP,0x70)?;
    c.imm(C::Al,Op::Mov,R::R2,R::R0,0)?;
    c.check(0x6a078)?;
    // Populate controller-input record fields selected by the preceding modes.
    c.ldr(R::R0,r8,12)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,752)?;
    c.emit(data_register(C::Al,Op::Add,r9,r7,r9,Shift::Lsl,6,false)?)?;
    c.imm(C::Al,Op::Cmp,R::R0,r6,0)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,4096)?;
    c.emit(transfer_word_register(C::Al,false,R::R0,r9,R::R1,Shift::Lsl,0)?)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,237)?;
    c.ldrb(R::R0,r8,1)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,4608)?;
    c.emit(transfer_byte_register(C::Al,false,R::R0,r9,R::R1,Shift::Lsl,0)?)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,748)?;
    c.ldrb(R::R0,r8,8)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,4096)?;
    c.emit(transfer_byte_register(C::Al,false,R::R0,r9,R::R1,Shift::Lsl,0)?)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,165)?;
    c.ldrb(R::R0,r8,0xf0)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,4864)?;
    c.emit(transfer_byte_register(C::Al,false,R::R0,r9,R::R1,Shift::Lsl,0)?)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,872)?;
    c.ldr(R::R0,r8,0xf4)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,4096)?;
    c.emit(transfer_word_register(C::Al,false,R::R0,r9,R::R1,Shift::Lsl,0)?)?;
    c.branch(C::Eq,0x6a1bc,false)?;
    c.check(0x6a0d4)?;
    c.str(r4,R::SP,0x24)?;
    c.str(R::R2,R::SP,0x48)?;
    c.str(r5,R::SP,0x20)?;
    c.str(R::R0,r6,0x24)?;
    c.ldr(R::R0,r7,0x78)?;
    c.emit(store_halfword(C::Al,R::R0,r6,40)?)?;
    c.ldr(R::R0,r8,0xdc)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R0,0)?;
    c.mov(C::Al,R::R1,R::R0)?;
    c.emit(transfer_doubleword(C::Eq,true,R::R2,r7,56)?)?;
    c.emit(multiply_register(C::Eq,R::R1,R::R2,R::new(3)?)?)?;
    c.ldr(R::R2,r8,0xd8)?;
    c.emit(mov_shifted_register(C::Eq,R::R1,R::R1,Shift::Lsl,2)?)?;
    c.str(R::R2,r7,0x660)?;
    c.str(R::R0,r7,0x664)?;
    c.ldr(R::R0,r8,0xe0)?;
    c.str(R::R0,r7,0x668)?;
    c.ldr(R::R0,r8,0xe4)?;
    c.str(R::R0,r7,0x66c)?;
    c.emit(data_register(C::Al,Op::Add,R::R0,R::R2,R::R1,
        Shift::Lsr,1,false)?)?;
    c.emit(branch_exchange_immediate(c.pc as u32,0x83c4c)?)?;
    c.imm(C::Al,Op::And,R::R0,R::R0,255)?;
    c.emit(branch_exchange_immediate(c.pc as u32,0x82f74)?)?;
    c.check(0x6a130)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,924)?;
    c.imm(C::Al,Op::Add,r10,r8,88)?;
    c.ldr(R::R12,R::SP,0x70)?;
    c.mov(C::Al,r4,r6)?;
    c.imm(C::Al,Op::Add,R::LR,r8,132)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,4096)?;
    c.emit(transfer_word_register(C::Al,false,R::R0,r9,R::R1,Shift::Lsl,0)?)?;
    c.str(R::R0,r6,0x60)?;
    c.mov(C::Al,R::R1,r10)?;
    c.imm(C::Al,Op::Add,R::R0,r4,108)?;
    let words=R::R2.bit()|R::new(3)?.bit()|r5.bit()|r6.bit()|r7.bit();
    c.emit(transfer_multiple(C::Al,true,R::R1,words,
        BlockMode::IncrementAfter,true)?)?;
    c.emit(transfer_multiple(C::Al,false,R::R0,words,
        BlockMode::IncrementAfter,true)?)?;
    c.emit(transfer_multiple(C::Al,true,R::R1,words,
        BlockMode::IncrementAfter,false)?)?;
    c.mov(C::Al,R::R1,R::LR)?;
    c.emit(transfer_multiple(C::Al,false,R::R0,words,
        BlockMode::IncrementAfter,false)?)?;
    c.imm(C::Al,Op::Add,R::R0,r4,152)?;
    c.emit(transfer_multiple(C::Al,true,R::R1,words,
        BlockMode::IncrementAfter,false)?)?;
    c.emit(transfer_multiple(C::Al,false,R::R0,words,
        BlockMode::IncrementAfter,false)?)?;
    c.check(0x6a178)?;
    for (source,destination) in [(0x80,0x94),(0x98,0xac),(0x9c,0xb0)] {
        c.ldr(R::R0,r8,source)?;
        c.str(R::R0,r4,destination)?;
    }
    c.ldr(R::R0,r8,0xa4)?;
    c.str(r4,R::SP,0x3c)?;
    c.str(R::R0,r4,0xb4)?;
    c.ldrb(R::R0,R::R12,0x30)?;
    c.imm(C::Al,Op::Cmp,R::R0,R::R0,0)?;
    c.branch(C::Eq,0x6a1d4,false)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,0)?;
    for offset in [0x1c,0x18,0x14] {
        c.str(R::R0,R::SP,offset)?;
    }
    c.branch(C::Al,0x6a300,false)?;
    c.check(0x6a1bc)?;
    c.ldr(R::R2,R::PC,0xa30)?;
    c.imm(C::Al,Op::Mov,R::R1,R::R0,118)?;
    c.imm(C::Al,Op::Mov,R::R0,R::R0,0)?;
    c.imm(C::Al,Op::Orr,R::R1,R::R1,2048)?;
    c.branch(C::Al,0x17b0,true)?;
    c.branch(C::Al,0x6abec,false)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn covers_bounded_function_prefix() {
        assert_eq!(fragment().unwrap().len(),END-START);
        assert_eq!(END-START,912);
    }
}
