// SPDX-License-Identifier: GPL-3.0-only
//! Descriptor throughput tier calculation and small allocation helpers.
//! ARM32 function-aligned interval, with the fixed thresholds and logging
//! addresses separately named. No raw firmware instructions are embedded.
use arm32::{branch, branch_exchange_immediate, bx, data_immediate,
            data_register, mov_register, mov_shifted_register, multiply_register, push_registers,
            transfer_multiple, transfer_word, transfer_byte, BlockMode, Condition as C,
            DataOp as Op, Register as R, Shift};

pub const START:usize=0x710c0;
pub const END:usize=0x71328;
pub const FILE_BASE:usize=0x4000;
// Fixed area threshold and allocator-failure log location are data words.
const MAX_QUALITY_AREA:u32=0x001f_a401;
const ALLOC_DIAGNOSTIC:u32=0x000f_2e4c;
// Identifiers/addresses loaded by the controller profile initializer.
const PROFILE_WORDS:[u32;5]=[
    0x0010_b098,0x0010_b070,0x0010_af08,0x0010_aee0,0x0010_aed5,
];

struct Encoder { at:usize, bytes:Vec<u8> }
impl Encoder {
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn word(&mut self,w:u32)->Result<(),String>{
        if self.at < START || self.at >= END || self.at%4!=0 {
            return Err(format!("invalid quality emitter location 0x{:x}",self.at));
        }
        self.bytes.extend(w.to_le_bytes());self.at+=4;Ok(())
    }
    fn at(&self,expected:usize)->Result<(),String>{
        if self.at == expected {Ok(())}
        else {Err(format!("location 0x{:x}, expected 0x{expected:x}",self.at))}
    }
    fn branch(&mut self,condition:C,target:usize)->Result<(),String>{
        self.word(branch(condition,self.at as u32,target as u32,false)?)
    }
    fn call(&mut self,target:usize)->Result<(),String>{
        self.word(branch(C::Al,self.at as u32,target as u32,true)?)
    }
    fn literal(&mut self,out:R,location:usize)->Result<(),String>{
        self.word(transfer_word(C::Al,true,out,R::PC,(location as i32)-(self.at as i32)-8)?)
    }
}
fn i(c:&mut Encoder,condition:C,op:Op,out:R,left:R,value:u32)->Result<(),String>{
    c.word(data_immediate(condition,op,out,left,value)?)
}
fn g(c:&mut Encoder,op:Op,out:R,left:R,right:R,shift:Shift,amount:u8)->Result<(),String>{
    c.word(data_register(C::Al,op,out,left,right,shift,amount,false)?)
}
fn mov(c:&mut Encoder,rd:R,rm:R)->Result<(),String>{c.word(mov_register(C::Al,rd,rm))}
fn cmp(c:&mut Encoder,rd:R,imm:u32)->Result<(),String>{i(c,C::Al,Op::Cmp,R::R0,rd,imm)}
fn set(c:&mut Encoder,condition:C,reg:R,n:u32)->Result<(),String>{i(c,condition,Op::Mov,reg,R::R0,n)}
fn frame()->u16{R::new(4).unwrap().bit()|R::new(7).unwrap().bit()|R::R11.bit()|R::LR.bit()}
fn leave(c:&mut Encoder)->Result<(),String>{
    c.word(transfer_multiple(C::Al,true,R::SP,(frame() & !R::LR.bit())|R::PC.bit(),BlockMode::IncrementAfter,true)?)
}

/// Calculate a resolution-dependent quality byte. The quality tier depends
/// on area plus the normalized sum of area and frame-rate scaled throughput.
/// Each size tier uses separate encodeable A32 immediate thresholds.
fn throughput_quality(c:&mut Encoder)->Result<(),String>{
    let r4=R::new(4)?;
    c.at(START)?;
    c.word(push_registers(C::Al,frame())?)?;
    i(c,C::Al,Op::Add,R::R11,R::SP,8)?;
    c.word(transfer_word(C::Al,true,R::R12,R::R0,0x54)?)?;
    c.word(transfer_word(C::Al,true,R::R2,R::R0,0x58)?)?;
    c.word(transfer_word(C::Al,true,R::R0,R::R0,0x60)?)?;
    set(c,C::Al,R::new(3)?,100)?;
    c.word(multiply_register(C::Al,R::R1,R::R0,R::new(3)?)?)?;
    c.word(multiply_register(C::Al,r4,R::R12,R::R2)?)?;
    g(c,Op::Add,R::R0,R::R1,r4,Shift::Lsr,1)?;
    mov(c,R::R1,r4)?;
    c.word(branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    c.literal(R::R1,0x71210)?;
    g(c,Op::Cmp,R::R0,r4,R::R1,Shift::Lsl,0)?;
    c.branch(C::Cc,0x71128)?;
    set(c,C::Al,R::R1,22)?;
    cmp(c,R::R0,31)?;
    set(c,C::Ls,R::R1,42)?;
    i(c,C::Ls,Op::Cmp,R::R0,R::R0,21)?;
    c.branch(C::Hi,0x71208)?;
    set(c,C::Al,R::R1,127)?;
    cmp(c,R::R0,1)?;
    set(c,C::Hi,R::R1,82)?;
    cmp(c,R::R0,8)?;
    set(c,C::Hi,R::R1,62)?;
    mov(c,R::R0,R::R1)?;
    leave(c)?;
    c.at(0x71128)?;

    // Each subsequent area threshold has three bitrate ranges. The jump to
    // the shared return retains the original branch and flag semantics.
    cmp(c,r4,921600)?;
    c.branch(C::Ls,0x7115c)?;
    set(c,C::Al,R::R1,14)?;
    cmp(c,R::R0,31)?;
    set(c,C::Ls,R::R1,22)?;
    i(c,C::Ls,Op::Cmp,R::R0,R::R0,21)?;
    c.branch(C::Hi,0x71208)?;
    set(c,C::Al,R::R1,127)?;
    cmp(c,R::R0,1)?;
    set(c,C::Hi,R::R1,76)?;
    cmp(c,R::R0,8)?;
    set(c,C::Hi,R::R1,54)?;
    c.branch(C::Al,0x71208)?;
    c.at(0x7115c)?;
    cmp(c,r4,307200)?;
    c.branch(C::Ls,0x71190)?;
    set(c,C::Al,R::R1,30)?;
    cmp(c,R::R0,31)?;
    set(c,C::Ls,R::R1,46)?;
    i(c,C::Ls,Op::Cmp,R::R0,R::R0,21)?;
    c.branch(C::Hi,0x71208)?;
    set(c,C::Al,R::R1,127)?;
    cmp(c,R::R0,1)?;
    set(c,C::Hi,R::R1,119)?;
    cmp(c,R::R0,8)?;
    set(c,C::Hi,R::R1,90)?;
    c.branch(C::Al,0x71208)?;
    c.at(0x71190)?;
    cmp(c,r4,101376)?;
    c.branch(C::Ls,0x711c4)?;
    set(c,C::Al,R::R1,28)?;
    cmp(c,R::R0,31)?;
    set(c,C::Ls,R::R1,46)?;
    i(c,C::Ls,Op::Cmp,R::R0,R::R0,21)?;
    c.branch(C::Hi,0x71208)?;
    set(c,C::Al,R::R1,127)?;
    cmp(c,R::R0,1)?;
    set(c,C::Hi,R::R1,116)?;
    cmp(c,R::R0,8)?;
    set(c,C::Hi,R::R1,86)?;
    c.branch(C::Al,0x71208)?;
    c.at(0x711c4)?;
    cmp(c,r4,25344)?;
    c.branch(C::Ls,0x711e8)?;
    set(c,C::Al,R::R1,36)?;
    cmp(c,R::R0,31)?;
    set(c,C::Ls,R::R1,56)?;
    i(c,C::Ls,Op::Cmp,R::R0,R::R0,21)?;
    c.branch(C::Hi,0x71208)?;
    set(c,C::Al,R::R1,100)?;
    c.branch(C::Al,0x71200)?;
    c.at(0x711e8)?;
    set(c,C::Al,R::R1,50)?;
    cmp(c,R::R0,31)?;
    set(c,C::Ls,R::R1,70)?;
    i(c,C::Ls,Op::Cmp,R::R0,R::R0,21)?;
    c.branch(C::Hi,0x71208)?;
    set(c,C::Al,R::R1,110)?;
    c.at(0x71200)?;
    cmp(c,R::R0,8)?;
    set(c,C::Ls,R::R1,127)?;
    c.at(0x71208)?;
    i(c,C::Al,Op::And,R::R0,R::R1,255)?;
    leave(c)?;
    c.at(0x71210)?;
    c.word(MAX_QUALITY_AREA)?;
    c.at(0x71214)
}

/// Allocate the 376-byte output record and return its pointer, with the
/// same zero-pointer logging condition as the device runtime.
fn create_output_record(c:&mut Encoder)->Result<(),String>{
    let r4=R::new(4)?;
    c.at(0x71214)?;
    c.word(push_registers(C::Al,frame())?)?;
    i(c,C::Al,Op::Add,R::R11,R::SP,8)?;
    set(c,C::Al,R::R0,376)?;
    c.call(0x2efc)?;
    mov(c,r4,R::R0)?;
    cmp(c,r4,0)?;
    c.branch(C::Ne,0x71244)?;
    c.literal(R::R2,0x7124c)?;
    set(c,C::Al,R::R1,41)?;
    set(c,C::Al,R::R0,0)?;
    i(c,C::Al,Op::Orr,R::R1,R::R1,8448)?;
    c.call(0x17b0)?;
    c.at(0x71244)?;
    mov(c,R::R0,r4)?;
    leave(c)?;
    c.at(0x7124c)?;
    c.word(ALLOC_DIAGNOSTIC)?;
    c.at(0x71250)
}

/// Null-guarded tail-call to the runtime resource-release helper.
fn release_if_allocated(c:&mut Encoder)->Result<(),String>{
    c.at(0x71250)?;
    cmp(c,R::R0,0)?;
    c.word(bx(C::Eq,R::LR))?;
    c.branch(C::Al,0x2f1c)?;
    c.at(0x7125c)
}

/// Configure the six named fields of a controller descriptor according to
/// whether the supplied profile's mode index exceeds fourteen. Preserve the
/// associated fixed callback/table pointers, and clear the 20-byte tail.
fn initialize_controller_profile(c:&mut Encoder)->Result<(),String>{
    let r3=R::new(3)?;let r4=R::new(4)?;let r5=R::new(5)?;
    let saved=r4.bit()|r5.bit()|R::R11.bit()|R::LR.bit();
    c.at(0x7125c)?;
    c.word(push_registers(C::Al,saved)?)?;
    i(c,C::Al,Op::Add,R::R11,R::SP,8)?;
    mov(c,r4,R::R1)?;
    cmp(c,R::R0,0)?;
    i(c,C::Ne,Op::Cmp,R::R0,r4,0)?;
    c.branch(C::Eq,0x71310)?;
    set(c,C::Al,r5,0)?;
    c.word(transfer_byte(C::Al,false,r5,r4,0x160)?)?;
    c.word(transfer_word(C::Al,true,R::R0,R::R0,0x20)?)?;
    c.word(mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,8)?)?;
    cmp(c,R::R0,14)?;
    c.branch(C::Hi,0x712ac)?;
    c.literal(R::R12,0x7131c)?;
    c.literal(R::LR,0x71320)?;
    set(c,C::Al,R::R0,75)?;
    set(c,C::Al,R::R2,26)?;
    set(c,C::Al,r3,63)?;
    set(c,C::Al,R::R1,55)?;
    i(c,C::Al,Op::Orr,R::R0,R::R0,18176)?;
    c.branch(C::Al,0x712c8)?;
    c.at(0x712ac)?;
    c.literal(R::R12,0x71314)?;
    c.literal(R::LR,0x71318)?;
    set(c,C::Al,R::R0,556)?;
    set(c,C::Al,R::R2,31)?;
    set(c,C::Al,r3,107)?;
    set(c,C::Al,R::R1,27)?;
    i(c,C::Al,Op::Orr,R::R0,R::R0,25600)?;
    c.at(0x712c8)?;
    for (reg,offset) in [(R::R1,0x140),(R::R0,0x144),(r3,0x148),
                         (R::R2,0x150)] {
        c.word(transfer_word(C::Al,false,reg,r4,offset)?)?;
        if offset==0x144 {set(c,C::Al,R::R0,1)?;}
    }
    set(c,C::Al,R::R1,0)?;
    set(c,C::Al,R::R2,320)?;
    c.word(transfer_word(C::Al,false,R::R0,r4,0x14c)?)?;
    c.literal(R::R0,0x71324)?;
    c.word(transfer_word(C::Al,false,R::R0,r4,0x15c)?)?;
    mov(c,R::R0,r4)?;
    c.word(transfer_word(C::Al,false,R::LR,r4,0x158)?)?;
    c.word(transfer_word(C::Al,false,R::R12,r4,0x154)?)?;
    c.call(0x31ec)?;
    for off in [0x164,0x168,0x16c,0x170] {
        c.word(transfer_word(C::Al,false,r5,r4,off)?)?;
    }
    c.at(0x71310)?;
    c.word(transfer_multiple(C::Al,true,R::SP,(saved & !R::LR.bit())|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.at(0x71314)?;
    for word in PROFILE_WORDS { c.word(word)?; }
    c.at(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Encoder::new();
    throughput_quality(&mut c)?;
    create_output_record(&mut c)?;
    release_if_allocated(&mut c)?;
    initialize_controller_profile(&mut c)?;
    Ok(c.bytes)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn area_qualities_and_allocator_layout(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        assert_eq!(&bytes[0x71210-START..0x71214-START],&MAX_QUALITY_AREA.to_le_bytes());
        assert_eq!(&bytes[0x7124c-START..0x71250-START],&ALLOC_DIAGNOSTIC.to_le_bytes());
        for (i,word) in PROFILE_WORDS.into_iter().enumerate(){
            let pos=0x71314-START+4*i;
            assert_eq!(&bytes[pos..pos+4],&word.to_le_bytes());
        }
    }
}
