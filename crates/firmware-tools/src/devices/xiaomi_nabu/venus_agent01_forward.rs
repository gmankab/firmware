// SPDX-License-Identifier: GPL-3.0-only
//! Agent 01 forward-only Venus ARM32 reconstruction, starting at runtime
//! address 0x12e5c. Instruction encoding uses validated public ARM A32
//! constructors; unimplemented later code is never copied from the OEM image.
//!
//! This is independently emitted partial executable code, not a firmware
//! container and not an OEM-signed image. The original function continues
//! beyond this fragment and its return is not yet reconstructed.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
use super::venus_runtime_services::Code;

pub const START: usize=0x12e5c;
pub const END: usize=0x13020;
pub const FILE_OFFSET:usize=START+0x4000;

fn imm(c:&mut Code<'_>,cond:C,op:Op,rd:R,rn:R,value:u32)->Result<(),String>{
    c.emit(arm32::data_immediate(cond,op,rd,rn,value)?)
}
fn reg(c:&mut Code<'_>,cond:C,op:Op,rd:R,rn:R,rm:R)->Result<(),String>{
    c.emit(arm32::data_register(cond,op,rd,rn,rm,Shift::Lsl,0,false)?)
}

/// Save the caller's registers, select the codec metadata layout, validate
/// stream mode and establish the initial input/output descriptor counts.
fn classify_active_buffers(c:&mut Code<'_>)->Result<(),String>{
    c.check(START)?;
    // 0x12e5c
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x12e60
    imm(c,C::Al,Op::Add,R::R11,R::SP,28)?;
    // 0x12e64
    imm(c,C::Al,Op::Sub,R::SP,R::SP,60)?;
    // 0x12e68
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R1))?;
    // 0x12e6c
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 0x12e70
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,148)?;
    // 0x12e74
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,3072)?;
    // 0x12e78
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x12e7c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,7)?;
    // 0x12e80
    imm(c,C::Ne,Op::Cmp,R::R0,R::R0,2)?;
    // 0x12e84
    c.branch(C::Ne, 0x12eb8)?;
    // 0x12e88
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // 0x12e8c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,496)?;
    // 0x12e90
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x12e94
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R1,Shift::Lsl,2)?)?;
    // 0x12e98
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x12e9c
    imm(c,C::Al,Op::Sub,R::R1,R::R1,1)?;
    // 0x12ea0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,1)?;
    // 0x12ea4
    c.branch(C::Hi, 0x12eb8)?;
    // 0x12ea8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,44)?)?;
    // 0x12eac
    imm(c,C::Al,Op::Add,R::R1,R::new(5)?,220)?;
    // 0x12eb0
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x12eb4
    imm(c,C::Eq,Op::Add,R::new(3)?,R::R1,3072)?;
    // 0x12eb8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x12ebc
    imm(c,C::Al,Op::Mov,R::R12,R::R0,0)?;
    // 0x12ec0
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,728)?)?;
    // 0x12ec4
    imm(c,C::Eq,Op::Cmp,R::R0,R::R1,2)?;
    // 0x12ec8
    c.branch(C::Ne, 0x12edc)?;
    // 0x12ecc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,44)?)?;
    // 0x12ed0
    imm(c,C::Al,Op::Mov,R::R12,R::R0,0)?;
    // 0x12ed4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x12ed8
    imm(c,C::Eq,Op::Mov,R::R12,R::R0,1)?;
    // 0x12edc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2648)?)?;
    // 0x12ee0
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(5)?,68)?)?;
    // 0x12ee4
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R1,60)?)?;
    // 0x12ee8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R1,820)?)?;
    // 0x12eec
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x12ef0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // 0x12ef4
    imm(c,C::Al,Op::Cmp,R::R0,R::new(10)?,0)?;
    // 0x12ef8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0x12efc
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,1)?;
    // 0x12f00
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x12f04
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,44)?)?;
    // 0x12f08
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,44)?)?;
    // 0x12f0c
    imm(c,C::Eq,Op::Cmp,R::R0,R::R1,0)?;
    // 0x12f10
    c.branch(C::Eq, 0x12f50)?;
    // 0x12f14
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,3372)?)?;
    // 0x12f18
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,3368)?)?;
    // 0x12f1c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x12f20
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x12f24
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x12f28
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,1)?;
    // 0x12f2c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x12f30
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,2)?;
    // 0x12f34
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // 0x12f38
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(3)?, R::R1.bit() | R::new(8)?.bit(), arm32::BlockMode::IncrementBefore, false)?)?;
    // 0x12f3c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(8)?,0)?;
    // 0x12f40
    imm(c,C::Ne,Op::Mov,R::new(8)?,R::R0,1)?;
    // 0x12f44
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x12f48
    imm(c,C::Ne,Op::Mov,R::new(8)?,R::R0,2)?;
    // 0x12f4c
    c.branch(C::Al, 0x13020)?;
    // 0x12f50
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2304)?)?;
    // 0x12f54
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,4)?)?;
    // 0x12f58
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x12f5c
    c.branch(C::Eq, 0x12f6c)?;
    // 0x12f60
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(3)?,8)?)?;
    // 0x12f64
    imm(c,C::Al,Op::Mov,R::R0,R::R0,2)?;
    // 0x12f68
    c.branch(C::Al, 0x13004)?;
    // 0x12f6c
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,0)?)?;
    // 0x12f70
    c.literal(R::new(7)?,0x13afc)?;
    // 0x12f74
    imm(c,C::Al,Op::Mov,R::R1,R::R0,63)?;
    // 0x12f78
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2184)?)?;
    // 0x12f7c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,1792)?;
    // 0x12f80
    reg(c,C::Al,Op::Cmp,R::R0,R::R2,R::R0)?;
    // 0x12f84
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x12f88
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 0x12f8c
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,1)?;
    // 0x12f90
    c.call(0x17b0)?;
    // 0x12f94
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x12f98
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x12f9c
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 0x12fa0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2188)?)?;
    // 0x12fa4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,8)?)?;
    // 0x12fa8
    reg(c,C::Al,Op::Cmp,R::R0,R::R1,R::R0)?;
    // 0x12fac
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x12fb0
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1856)?;
    // 0x12fb4
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,1)?;
    // 0x12fb8
    c.call(0x17b0)?;
    // 0x12fbc
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x12fc0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-36)?)?;
    // 0x12fc4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x12fc8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,4)?)?;
    // 0x12fcc
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x12fd0
    c.branch(C::Eq, 0x12fe0)?;
    // 0x12fd4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2184)?)?;
    // 0x12fd8
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x12fdc
    imm(c,C::Ne,Op::Mov,R::R2,R::R0,1)?;
    // 0x12fe0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,8)?)?;
    // 0x12fe4
    imm(c,C::Al,Op::Mov,R::new(8)?,R::R0,0)?;
    // 0x12fe8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x12fec
    c.branch(C::Eq, 0x12ffc)?;
    // 0x12ff0
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(5)?,2188)?)?;
    // 0x12ff4
    imm(c,C::Al,Op::Cmp,R::R0,R::new(8)?,0)?;
    // 0x12ff8
    imm(c,C::Ne,Op::Mov,R::new(8)?,R::R0,1)?;
    // 0x12ffc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x13000
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,0)?)?;
    // 0x13004
    imm(c,C::Al,Op::Cmp,R::R0,R::new(8)?,0)?;
    // 0x13008
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,8)?)?;
    // 0x1300c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x13010
    imm(c,C::Ne,Op::Mov,R::new(8)?,R::R0,1)?;
    // 0x13014
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x13018
    imm(c,C::Ne,Op::Mov,R::new(8)?,R::R0,2)?;
    // 0x1301c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,40)?)?;
    c.check(END)
}

/// Partial independently encoded region; does not read private firmware.
pub fn fragment()->Result<Vec<u8>,String>{
    let skip=START-0x1000; // Scratch prefix is discarded; no padding is emitted.
    let mut out=vec![0;skip];
    let mut c=Code{bytes:&mut out,at:START};
    classify_active_buffers(&mut c)?;
    Ok(out.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn first_forward_fragment_is_exact_length(){
        assert_eq!(fragment().unwrap().len(),END-START);
    }
}
