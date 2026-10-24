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
pub const END: usize=0x174e4;
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
    c.check(0x13020)
}

/// Assemble alternate codec plane dimensions, record stride rounding and
/// zero-copy fallback metadata for the selected input format.
fn normalize_codec_layout(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x13020)?;
    // 0x13020
    imm(c,C::Al,Op::And,R::R1,R::new(4)?,2)?;
    // 0x13024
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x13028
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x1302c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,40)?)?;
    // 0x13030
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x13034
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,368)?)?;
    // 0x13038
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,488)?)?;
    // 0x1303c
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::new(4)?, Shift::Lsr, 1)?)?;
    // 0x13040
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::R1, Shift::Lsr, 1)?)?;
    // 0x13044
    c.branch(C::Ne, 0x1309c)?;
    // 0x13048
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x1304c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x13050
    c.branch(C::Eq, 0x131a0)?;
    // 0x13054
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13058
    imm(c,C::Al,Op::Mov,R::R1,R::R0,536)?;
    // 0x1305c
    imm(c,C::Al,Op::Mov,R::new(8)?,R::R0,0)?;
    // 0x13060
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x13064
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13068
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x1306c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,532)?;
    // 0x13070
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x13074
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x13078
    imm(c,C::Al,Op::Mov,R::R0,R::R0,2)?;
    // 0x1307c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x13080
    c.emit(arm32::mov_register(C::Al, R::R0, R::R2))?;
    // 0x13084
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x13088
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x1308c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x13090
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x13094
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,28)?)?;
    // 0x13098
    c.branch(C::Al, 0x133f4)?;
    // 0x1309c
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R1))?;
    // 0x130a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,48)?)?;
    // 0x130a4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,1)?;
    // 0x130a8
    c.branch(C::Ne, 0x13108)?;
    // 0x130ac
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x130b0
    imm(c,C::Al,Op::Mvn,R::new(3)?,R::R0,2147483655)?;
    // 0x130b4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x130b8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,68)?)?;
    // 0x130bc
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
    // 0x130c0
    imm(c,C::Al,Op::Add,R::R0,R::R0,612)?;
    // 0x130c4
    imm(c,C::Al,Op::Add,R::R0,R::R0,4096)?;
    // 0x130c8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x130cc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,4,false)?)?;
    // 0x130d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R2,304)?)?;
    // 0x130d4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,344)?)?;
    // 0x130d8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x130dc
    imm(c,C::Al,Op::Add,R::R2,R::R2,15)?;
    // 0x130e0
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,28)?)?;
    // 0x130e4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,1,false)?)?;
    // 0x130e8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x130ec
    imm(c,C::Al,Op::Add,R::R2,R::new(4)?,15)?;
    // 0x130f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,388)?)?;
    // 0x130f4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,1,false)?)?;
    // 0x130f8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x130fc
    c.emit(arm32::mov_register(C::Al, R::R1, R::R2))?;
    // 0x13100
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x13104
    c.branch(C::Al, 0x133f4)?;
    // 0x13108
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,88)?)?;
    // 0x1310c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,92)?)?;
    // 0x13110
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,4)?;
    // 0x13114
    c.branch(C::Eq, 0x131ec)?;
    // 0x13118
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,7)?;
    // 0x1311c
    c.branch(C::Eq, 0x13204)?;
    // 0x13120
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,11)?;
    // 0x13124
    c.branch(C::Ne, 0x1324c)?;
    // 0x13128
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x1312c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,68)?)?;
    // 0x13130
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,6272)?;
    // 0x13134
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13138
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R2,Shift::Lsl,2,false)?)?;
    // 0x1313c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x13140
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x13144
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::R2,Shift::Lsl,3,false)?)?;
    // 0x13148
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,24)?)?;
    // 0x1314c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,8)?;
    // 0x13150
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?;
    // 0x13154
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x13158
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,444)?;
    // 0x1315c
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?;
    // 0x13160
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x13164
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-32)?)?;
    // 0x13168
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,76)?)?;
    // 0x1316c
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,28)?)?;
    // 0x13170
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x13174
    c.branch(C::Eq, 0x13364)?;
    // 0x13178
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1456)?;
    // 0x1317c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x13180
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x13184
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x13188
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x1318c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,312)?;
    // 0x13190
    imm(c,C::Al,Op::Orr,R::R0,R::R0,5120)?;
    // 0x13194
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x13198
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x1319c
    c.branch(C::Al, 0x133b0)?;
    // 0x131a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x131a4
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,0)?)?;
    // 0x131a8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,36)?)?;
    // 0x131ac
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,3)?;
    // 0x131b0
    imm(c,C::Ne,Op::Cmp,R::R0,R::R0,2)?;
    // 0x131b4
    c.branch(C::Eq, 0x13260)?;
    // 0x131b8
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x131bc
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x131c0
    c.branch(C::Ne, 0x13288)?;
    // 0x131c4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2452)?)?;
    // 0x131c8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2648)?)?;
    // 0x131cc
    imm(c,C::Al,Op::Add,R::R1,R::R1,1)?;
    // 0x131d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // 0x131d4
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,60)?)?;
    // 0x131d8
    imm(c,C::Al,Op::Cmp,R::R0,R::new(10)?,0)?;
    // 0x131dc
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0x131e0
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,1)?;
    // 0x131e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,44)?)?;
    // 0x131e8
    c.branch(C::Al, 0x132a4)?;
    // 0x131ec
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x131f0
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x131f4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,24)?)?;
    // 0x131f8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x131fc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x13200
    c.branch(C::Al, 0x133b4)?;
    // 0x13204
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x13208
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x1320c
    c.branch(C::Eq, 0x133bc)?;
    // 0x13210
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13214
    imm(c,C::Al,Op::Mov,R::R1,R::R0,276)?;
    // 0x13218
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x1321c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13220
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x13224
    imm(c,C::Al,Op::Mov,R::R1,R::R0,272)?;
    // 0x13228
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x1322c
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x13230
    c.emit(arm32::mov_register(C::Al, R::R0, R::R2))?;
    // 0x13234
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x13238
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x1323c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x13240
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x13244
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,28)?)?;
    // 0x13248
    c.branch(C::Al, 0x133b4)?;
    // 0x1324c
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x13250
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,24)?)?;
    // 0x13254
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x13258
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x1325c
    c.branch(C::Al, 0x133ec)?;
    // 0x13260
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2648)?)?;
    // 0x13264
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2452)?)?;
    // 0x13268
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x1326c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,60)?)?;
    // 0x13270
    imm(c,C::Al,Op::Add,R::new(8)?,R::R1,1)?;
    // 0x13274
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,40)?)?;
    // 0x13278
    imm(c,C::Al,Op::Cmp,R::R0,R::new(10)?,0)?;
    // 0x1327c
    imm(c,C::Ne,Op::Mov,R::new(10)?,R::R0,1)?;
    // 0x13280
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,44)?)?;
    // 0x13284
    c.branch(C::Al, 0x132a4)?;
    // 0x13288
    c.literal(R::R2,0x13afc)?;
    // 0x1328c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,904)?;
    // 0x13290
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x13294
    imm(c,C::Al,Op::Orr,R::R1,R::R1,1024)?;
    // 0x13298
    c.call(0x17b0)?;
    // 0x1329c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-36)?)?;
    // 0x132a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2648)?)?;
    // 0x132a4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,824)?)?;
    // 0x132a8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(9)?,Shift::Lsl,2,false)?)?;
    // 0x132ac
    imm(c,C::Al,Op::Mov,R::R2,R::R0,7)?;
    // 0x132b0
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,3)?;
    // 0x132b4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,1720)?)?;
    // 0x132b8
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,1708)?)?;
    // 0x132bc
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x132c0
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,15)?;
    // 0x132c4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x132c8
    reg(c,C::Al,Op::Add,R::new(4)?,R::R2,R::new(6)?)?;
    // 0x132cc
    imm(c,C::Eq,Op::Mov,R::new(3)?,R::R0,4)?;
    // 0x132d0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::new(4)?,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x132d4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x132d8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x132dc
    reg(c,C::Al,Op::Add,R::R1,R::R2,R::R12)?;
    // 0x132e0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x132e4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x132e8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-40)?)?;
    // 0x132ec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,728)?)?;
    // 0x132f0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x132f4
    c.emit(arm32::mov_register(C::Eq, R::new(7)?, R::new(6)?))?;
    // 0x132f8
    reg(c,C::Al,Op::Add,R::R1,R::R2,R::new(7)?)?;
    // 0x132fc
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x13300
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(4)?,R::R0,R::R1,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x13304
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // 0x13308
    c.emit(arm32::mov_register(C::Eq, R::R1, R::R12))?;
    // 0x1330c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,1)?;
    // 0x13310
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,12)?)?;
    // 0x13314
    reg(c,C::Al,Op::Add,R::R1,R::R2,R::R1)?;
    // 0x13318
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x1331c
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(7)?,R::R0,R::R1,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x13320
    c.branch(C::Ne, 0x1333c)?;
    // 0x13324
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,88)?)?;
    // 0x13328
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x1332c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,92)?)?;
    // 0x13330
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x13334
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,96)?)?;
    // 0x13338
    c.branch(C::Al, 0x13350)?;
    // 0x1333c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1704)?)?;
    // 0x13340
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x13344
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1716)?)?;
    // 0x13348
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x1334c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1728)?)?;
    // 0x13350
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,0)?)?;
    // 0x13354
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x13358
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // 0x1335c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x13360
    c.branch(C::Al, 0x133f4)?;
    // 0x13364
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,72)?)?;
    // 0x13368
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,60)?)?;
    // 0x1336c
    imm(c,C::Al,Op::Add,R::R0,R::R0,264)?;
    // 0x13370
    imm(c,C::Al,Op::Add,R::R0,R::R0,5120)?;
    // 0x13374
    imm(c,C::Al,Op::Add,R::new(3)?,R::R2,1)?;
    // 0x13378
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::R1,R::new(3)?)?)?;
    // 0x1337c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R2,Shift::Lsl,2,false)?)?;
    // 0x13380
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13384
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x13388
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1492)?)?;
    // 0x1338c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,48)?)?;
    // 0x13390
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    // 0x13394
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,1)?;
    // 0x13398
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x1339c
    c.emit(arm32::mov_register(C::Eq, R::R2, R::R0))?;
    // 0x133a0
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,0)?;
    // 0x133a4
    c.emit(arm32::mov_register(C::Eq, R::R1, R::new(7)?))?;
    // 0x133a8
    imm(c,C::Eq,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x133ac
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x133b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x133b4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x133b8
    c.branch(C::Al, 0x133f4)?;
    // 0x133bc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x133c0
    c.literal(R::R2,0x13afc)?;
    // 0x133c4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x133c8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,2)?;
    // 0x133cc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x133d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x133d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,24)?)?;
    // 0x133d8
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R12))?;
    // 0x133dc
    imm(c,C::Al,Op::Orr,R::R1,R::R1,2048)?;
    // 0x133e0
    c.call(0x17b0)?;
    // 0x133e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-36)?)?;
    // 0x133e8
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(6)?))?;
    // 0x133ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x133f0
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x133f4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x133f8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,36)?)?;
    // 0x133fc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-40)?)?;
    c.check(0x13400)
}


/// Dispatch 11 codec types into the appropriate size/format calculations.
/// The pointer table is A32 runtime data, not executable opcodes. Preserve
/// all repeated target addresses explicitly.
fn classify_codec_mode_and_dimensions(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x13400)?;
    // 0x13400
    imm(c,C::Al,Op::Add,R::new(4)?,R::R1,127)?;
    // 0x13404
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(9)?,R::R12,R::R0,Shift::Lsr,1,false)?)?;
    // 0x13408
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x1340c
    imm(c,C::Al,Op::Sub,R::R0,R::R0,1)?;
    // 0x13410
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,10)?;
    // 0x13414
    c.branch(C::Hi, 0x134e8)?;
    // 0x13418
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x1341c
    imm(c,C::Al,Op::Add,R::R1,R::PC,0)?;
    // 0x13420
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x13424
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(3)?,R::R1,R::R0,Shift::Asr,R::new(4)?,false)?)?;
    // 0x13428
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(3)?,R::R1,R::R0,Shift::Ror,R::new(4)?,false)?)?;
    // 0x1342c
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::R1,R::new(8)?,Shift::Ror,9,false)?)?;
    // 0x13430
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::R1,R::new(8)?,Shift::Ror,9,false)?)?;
    // 0x13434
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::R1,R::new(8)?,Shift::Ror,9,false)?)?;
    // 0x13438
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::R1,R::new(8)?,Shift::Ror,9,false)?)?;
    // 0x1343c
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::R1,R::new(8)?,Shift::Lsl,9,false)?)?;
    // 0x13440
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(3)?,R::R1,R::R0,Shift::Asr,R::new(4)?,false)?)?;
    // 0x13444
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::R1,R::new(8)?,Shift::Ror,9,false)?)?;
    // 0x13448
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(3)?,R::R1,R::R0,Shift::Asr,R::new(4)?,false)?)?;
    // 0x1344c
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::R1,R::new(8)?,Shift::Asr,9,false)?)?;
    // 0x13450
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,72)?)?;
    // 0x13454
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x13458
    c.branch(C::Eq, 0x134e8)?;
    // 0x1345c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x13460
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x13464
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,1028)?)?;
    // 0x13468
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,1032)?)?;
    // 0x1346c
    c.branch(C::Al, 0x134f0)?;
    // 0x13470
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13474
    imm(c,C::Al,Op::Cmp,R::R0,R::new(8)?,2)?;
    // 0x13478
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,1)?;
    // 0x1347c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(8)?,0)?;
    // 0x13480
    imm(c,C::Ne,Op::Mov,R::new(8)?,R::R0,1)?;
    // 0x13484
    c.branch(C::Al, 0x134f0)?;
    // 0x13488
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x1348c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13490
    imm(c,C::Al,Op::Mov,R::new(8)?,R::R0,0)?;
    // 0x13494
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x13498
    c.branch(C::Ne, 0x134f0)?;
    // 0x1349c
    c.literal(R::R2,0x13afc)?;
    // 0x134a0
    imm(c,C::Al,Op::Mov,R::R1,R::R0,69)?;
    // 0x134a4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x134a8
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R12))?;
    // 0x134ac
    imm(c,C::Al,Op::Orr,R::R1,R::R1,2048)?;
    // 0x134b0
    c.call(0x17b0)?;
    // 0x134b4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-36)?)?;
    // 0x134b8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // 0x134bc
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::R2.bit() | R::new(8)?.bit(), arm32::BlockMode::IncrementBefore, false)?)?;
    // 0x134c0
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(7)?))?;
    // 0x134c4
    c.branch(C::Al, 0x134f0)?;
    // 0x134c8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,72)?)?;
    // 0x134cc
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x134d0
    c.branch(C::Eq, 0x134e8)?;
    // 0x134d4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x134d8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x134dc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,3368)?)?;
    // 0x134e0
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,3372)?)?;
    // 0x134e4
    c.branch(C::Al, 0x134f0)?;
    // 0x134e8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,4)?)?;
    // 0x134ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(3)?,8)?)?;
    // 0x134f0
    imm(c,C::Al,Op::Bic,R::R0,R::new(4)?,127)?;
    // 0x134f4
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(5)?))?;
    // 0x134f8
    imm(c,C::Al,Op::Cmp,R::R0,R::new(10)?,0)?;
    // 0x134fc
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(5)?))?;
    // 0x13500
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,65536)?;
    // 0x13504
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x13508
    imm(c,C::Al,Op::Mov,R::R0,R::R0,552)?;
    // 0x1350c
    imm(c,C::Ne,Op::Mov,R::new(10)?,R::R0,1)?;
    // 0x13510
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x13514
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x13518
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1056)?;
    // 0x1351c
    imm(c,C::Al,Op::Orr,R::R0,R::R0,36864)?;
    // 0x13520
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x13524
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x13528
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R2, Shift::Lsl, 12)?)?;
    // 0x1352c
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(6)?))?;
    // 0x13530
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,28)?)?;
    // 0x13534
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(10)?,Shift::Lsl,1,false)?)?;
    // 0x13538
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(8)?,Shift::Lsl,4,false)?)?;
    // 0x1353c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x13540
    imm(c,C::Al,Op::Mov,R::R2,R::R0,556)?;
    // 0x13544
    imm(c,C::Al,Op::Orr,R::R2,R::R2,7168)?;
    // 0x13548
    imm(c,C::Eq,Op::Orr,R::R0,R::R0,1)?;
    // 0x1354c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x13550
    c.literal(R::new(3)?,0x13b00)?;
    // 0x13554
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(10)?,R::new(4)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x13558
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,832)?)?;
    // 0x1355c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,816)?)?;
    // 0x13560
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x13564
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::new(7)?)?;
    // 0x13568
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,1048576)?;
    // 0x1356c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,19,false)?)?;
    // 0x13570
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,0)?;
    // 0x13574
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::new(7)?)?;
    // 0x13578
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,256)?;
    // 0x1357c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x13580
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::new(3)?)?;
    // 0x13584
    c.literal(R::new(3)?,0x13b04)?;
    // 0x13588
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(4)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x1358c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,5)?;
    // 0x13590
    imm(c,C::Eq,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x13594
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,1)?;
    // 0x13598
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,24,false)?)?;
    // 0x1359c
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,1136)?;
    // 0x135a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x135a4
    imm(c,C::Al,Op::Add,R::R0,R::R1,8)?;
    // 0x135a8
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,36864)?;
    // 0x135ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x135b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,4)?)?;
    // 0x135b4
    imm(c,C::Al,Op::Add,R::R1,R::R2,8)?;
    // 0x135b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x135bc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // 0x135c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,4)?)?;
    // 0x135c4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x135c8
    c.branch(C::Ne, 0x13600)?;
    // 0x135cc
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,44)?)?;
    // 0x135d0
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x135d4
    c.branch(C::Ne, 0x13634)?;
    // 0x135d8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,40)?)?;
    // 0x135dc
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::LR))?;
    // 0x135e0
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-36)?)?;
    // 0x135e4
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,0)?;
    // 0x135e8
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x135ec
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x135f0
    imm(c,C::Eq,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x135f4
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x135f8
    imm(c,C::Ne,Op::Mov,R::new(7)?,R::R0,1)?;
    // 0x135fc
    c.branch(C::Al, 0x13770)?;
    // 0x13600
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,44)?)?;
    // 0x13604
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x13608
    c.branch(C::Ne, 0x13668)?;
    // 0x1360c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,40)?)?;
    // 0x13610
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::LR))?;
    // 0x13614
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-36)?)?;
    // 0x13618
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,0)?;
    // 0x1361c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x13620
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x13624
    imm(c,C::Eq,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x13628
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x1362c
    imm(c,C::Ne,Op::Mov,R::new(7)?,R::R0,1)?;
    // 0x13630
    c.branch(C::Al, 0x13858)?;
    // 0x13634
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,7)?;
    // 0x13638
    c.branch(C::Eq, 0x136c8)?;
    // 0x1363c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,11)?;
    // 0x13640
    c.branch(C::Ne, 0x13724)?;
    // 0x13644
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x13648
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,72)?)?;
    // 0x1364c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x13650
    c.branch(C::Eq, 0x13760)?;
    // 0x13654
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x13658
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,3,false)?)?;
    // 0x1365c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,3368)?)?;
    // 0x13660
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,3372)?)?;
    // 0x13664
    c.branch(C::Al, 0x13768)?;
    // 0x13668
    imm(c,C::Al,Op::Sub,R::R2,R::R2,1)?;
    // 0x1366c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,10)?;
    // 0x13670
    c.branch(C::Hi, 0x137dc)?;
    // 0x13674
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 2)?)?;
    // 0x13678
    imm(c,C::Al,Op::Add,R::new(3)?,R::PC,0)?;
    // 0x1367c
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0x13680)?;
    const CODEC_MODE_BRANCH_TARGETS:[u32;11]=[0x136ac,0x137dc,0x137dc,0x137ec,0x1373c,0x137dc,0x13804,0x136ac,0x137dc,0x136ac,0x1373c];
    for target in CODEC_MODE_BRANCH_TARGETS { c.emit(target)?; }
    c.check(0x136ac)?;
    // 0x136ac
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x136b0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,72)?)?;
    // 0x136b4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x136b8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,3,false)?)?;
    // 0x136bc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,1100)?)?;
    // 0x136c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,1104)?)?;
    // 0x136c4
    c.branch(C::Al, 0x13754)?;
    // 0x136c8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x136cc
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::LR))?;
    // 0x136d0
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x136d4
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,1)?;
    // 0x136d8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,44)?)?;
    // 0x136dc
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(3)?))?;
    // 0x136e0
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x136e4
    c.branch(C::Ne, 0x13770)?;
    // 0x136e8
    c.literal(R::R2,0x13afc)?;
    // 0x136ec
    imm(c,C::Al,Op::Mov,R::R1,R::R0,129)?;
    // 0x136f0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x136f4
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(9)?))?;
    // 0x136f8
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R12))?;
    // 0x136fc
    imm(c,C::Al,Op::Orr,R::R1,R::R1,2048)?;
    // 0x13700
    c.call(0x17b0)?;
    // 0x13704
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-36)?)?;
    // 0x13708
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x1370c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x13710
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // 0x13714
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(7)?))?;
    // 0x13718
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(8)?))?;
    // 0x1371c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::LR,2648)?)?;
    // 0x13720
    c.branch(C::Al, 0x13770)?;
    // 0x13724
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x13728
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::LR))?;
    // 0x1372c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,3368)?)?;
    // 0x13730
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,3372)?)?;
    // 0x13734
    c.emit(arm32::mov_register(C::Al, R::LR, R::R2))?;
    // 0x13738
    c.branch(C::Al, 0x13770)?;
    // 0x1373c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x13740
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,68)?)?;
    // 0x13744
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x13748
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,3,false)?)?;
    // 0x1374c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,3368)?)?;
    // 0x13750
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,3372)?)?;
    // 0x13754
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::LR))?;
    // 0x13758
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(3)?))?;
    // 0x1375c
    c.branch(C::Al, 0x13858)?;
    // 0x13760
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // 0x13764
    c.emit(arm32::transfer_multiple(C::Al, true, R::R2, R::new(5)?.bit() | R::new(7)?.bit(), arm32::BlockMode::IncrementBefore, false)?)?;
    // 0x13768
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::LR))?;
    // 0x1376c
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(3)?))?;
    // 0x13770
    imm(c,C::Al,Op::Mov,R::R2,R::R0,36)?;
    // 0x13774
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::R12, Shift::Lsl, 16)?)?;
    // 0x13778
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,0)?;
    // 0x1377c
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x13780
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x13784
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x13788
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,44)?)?;
    // 0x1378c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x13790
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(3)?,R::R2,R::R12,Shift::Lsl,16,false)?)?;
    // 0x13794
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,12,false)?)?;
    // 0x13798
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,816)?)?;
    // 0x1379c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,4,false)?)?;
    // 0x137a0
    imm(c,C::Al,Op::Add,R::new(7)?,R::R0,12)?;
    // 0x137a4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x137a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x137ac
    imm(c,C::Al,Op::Mov,R::R2,R::R0,68)?;
    // 0x137b0
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x137b4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x137b8
    imm(c,C::Al,Op::Add,R::R2,R::R1,8)?;
    // 0x137bc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,0)?)?;
    // 0x137c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // 0x137c4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-40)?)?;
    // 0x137c8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x137cc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x137d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // 0x137d4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,116)?;
    // 0x137d8
    c.branch(C::Al, 0x138c0)?;
    // 0x137dc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x137e0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,3368)?)?;
    // 0x137e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,3372)?)?;
    // 0x137e8
    c.branch(C::Al, 0x137f8)?;
    // 0x137ec
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x137f0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,1028)?)?;
    // 0x137f4
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,1032)?)?;
    // 0x137f8
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::LR))?;
    // 0x137fc
    c.emit(arm32::mov_register(C::Al, R::LR, R::R2))?;
    c.check(0x13800)
}


/// Finish the first codec input-mode selection, pack controller command
/// registers, then return through the original saved-register epilogue.
/// The six words after the return are addresses/negative table offsets.
fn finish_initial_codec_session(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x13800)?;
    // 0x13800
    c.branch(C::Al, 0x13858)?;
    // 0x13804
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x13808
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::LR))?;
    // 0x1380c
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x13810
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,1)?;
    // 0x13814
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,44)?)?;
    // 0x13818
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(3)?))?;
    // 0x1381c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x13820
    c.branch(C::Ne, 0x13858)?;
    // 0x13824
    c.literal(R::R2,0x13afc)?;
    // 0x13828
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x1382c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,2240)?;
    // 0x13830
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(9)?))?;
    // 0x13834
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R12))?;
    // 0x13838
    c.call(0x17b0)?;
    // 0x1383c
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-36)?)?;
    // 0x13840
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x13844
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x13848
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // 0x1384c
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(7)?))?;
    // 0x13850
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(8)?))?;
    // 0x13854
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::LR,2648)?)?;
    // 0x13858
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,40)?;
    // 0x1385c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,816)?)?;
    // 0x13860
    imm(c,C::Al,Op::Mov,R::new(8)?,R::R0,1)?;
    // 0x13864
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,0)?;
    // 0x13868
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x1386c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x13870
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 0x13874
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::new(5)?, Shift::Lsl, 12)?)?;
    // 0x13878
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,44)?)?;
    // 0x1387c
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(7)?,R::new(8)?,R::new(5)?,Shift::Lsl,12,false)?)?;
    // 0x13880
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x13884
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,4,false)?)?;
    // 0x13888
    imm(c,C::Al,Op::Add,R::new(7)?,R::R0,12)?;
    // 0x1388c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,8,false)?)?;
    // 0x13890
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x13894
    imm(c,C::Al,Op::Mov,R::R2,R::R0,72)?;
    // 0x13898
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x1389c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x138a0
    imm(c,C::Al,Op::Add,R::R2,R::R1,8)?;
    // 0x138a4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,0)?)?;
    // 0x138a8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // 0x138ac
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-40)?)?;
    // 0x138b0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x138b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x138b8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,120)?;
    // 0x138bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // 0x138c0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x138c4
    imm(c,C::Al,Op::Add,R::new(8)?,R::R2,4)?;
    // 0x138c8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x138cc
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,0)?)?;
    // 0x138d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x138d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x138d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,820)?)?;
    // 0x138dc
    imm(c,C::Al,Op::And,R::R0,R::R0,2)?;
    // 0x138e0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R12,R::R0,Shift::Lsr,1,false)?)?;
    // 0x138e4
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x138e8
    c.branch(C::Eq, 0x13a58)?;
    // 0x138ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,36)?)?;
    // 0x138f0
    imm(c,C::Al,Op::Add,R::R0,R::new(7)?,127)?;
    // 0x138f4
    imm(c,C::Al,Op::Bic,R::R12,R::R0,127)?;
    // 0x138f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,16)?)?;
    // 0x138fc
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,6)?;
    // 0x13900
    c.branch(C::Gt, 0x13930)?;
    // 0x13904
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x13908
    c.branch(C::Eq, 0x13958)?;
    // 0x1390c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x13910
    c.branch(C::Ne, 0x139e0)?;
    // 0x13914
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,40)?)?;
    // 0x13918
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x1391c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,2)?;
    // 0x13920
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,1)?;
    // 0x13924
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,0)?;
    // 0x13928
    imm(c,C::Ne,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x1392c
    c.branch(C::Al, 0x139e8)?;
    // 0x13930
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,7)?;
    // 0x13934
    c.branch(C::Eq, 0x13994)?;
    // 0x13938
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,11)?;
    // 0x1393c
    c.branch(C::Ne, 0x139e0)?;
    // 0x13940
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,68)?)?;
    // 0x13944
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x13948
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::LR,R::R0,Shift::Lsl,3,false)?)?;
    // 0x1394c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,3368)?)?;
    // 0x13950
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,3372)?)?;
    // 0x13954
    c.branch(C::Al, 0x139e8)?;
    // 0x13958
    c.literal(R::R1,0x13b08)?;
    // 0x1395c
    c.literal(R::R2,0x13b0c)?;
    // 0x13960
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,246)?;
    // 0x13964
    imm(c,C::Al,Op::Mov,R::R0,R::R0,8)?;
    // 0x13968
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(9)?))?;
    // 0x1396c
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R12))?;
    // 0x13970
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?;
    // 0x13974
    c.call(0x2ec94)?;
    // 0x13978
    c.literal(R::R1,0x13b10)?;
    // 0x1397c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,8)?;
    // 0x13980
    c.call(0x2ec94)?;
    // 0x13984
    c.literal(R::R2,0x13afc)?;
    // 0x13988
    imm(c,C::Al,Op::Mov,R::R1,R::R0,247)?;
    // 0x1398c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x13990
    c.branch(C::Al, 0x139bc)?;
    // 0x13994
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,44)?)?;
    // 0x13998
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x1399c
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x139a0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x139a4
    c.branch(C::Ne, 0x139e8)?;
    // 0x139a8
    c.literal(R::R2,0x13afc)?;
    // 0x139ac
    imm(c,C::Al,Op::Mov,R::R1,R::R0,241)?;
    // 0x139b0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x139b4
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(9)?))?;
    // 0x139b8
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R12))?;
    // 0x139bc
    imm(c,C::Al,Op::Orr,R::R1,R::R1,2048)?;
    // 0x139c0
    c.call(0x17b0)?;
    // 0x139c4
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(5)?))?;
    // 0x139c8
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(7)?))?;
    // 0x139cc
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-36)?)?;
    // 0x139d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,36)?)?;
    // 0x139d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x139d8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    // 0x139dc
    c.branch(C::Al, 0x139e8)?;
    // 0x139e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,3440)?)?;
    // 0x139e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,3444)?)?;
    // 0x139e8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x139ec
    imm(c,C::Al,Op::Mov,R::R2,R::R0,40)?;
    // 0x139f0
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::R0, Shift::Lsl, 12)?)?;
    // 0x139f4
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,0)?;
    // 0x139f8
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x139fc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x13a00
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x13a04
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::LR,2648)?)?;
    // 0x13a08
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(3)?,R::R2,R::R0,Shift::Lsl,12,false)?)?;
    // 0x13a0c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::new(5)?,Shift::Lsl,4,false)?)?;
    // 0x13a10
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,816)?)?;
    // 0x13a14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,8,false)?)?;
    // 0x13a18
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    // 0x13a1c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x13a20
    imm(c,C::Al,Op::Mov,R::R0,R::R0,72)?;
    // 0x13a24
    imm(c,C::Al,Op::Add,R::new(8)?,R::R2,12)?;
    // 0x13a28
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x13a2c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x13a30
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x13a34
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x13a38
    imm(c,C::Al,Op::Add,R::new(7)?,R::R1,12)?;
    // 0x13a3c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,4)?)?;
    // 0x13a40
    imm(c,C::Al,Op::Mov,R::R0,R::R0,120)?;
    // 0x13a44
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // 0x13a48
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x13a4c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,8)?)?;
    // 0x13a50
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,0)?)?;
    // 0x13a54
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R2,8)?)?;
    // 0x13a58
    imm(c,C::Al,Op::Mov,R::R0,R::R0,5184)?;
    // 0x13a5c
    imm(c,C::Al,Op::Orr,R::R0,R::R0,32768)?;
    // 0x13a60
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x13a64
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x13a68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x13a6c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x13a70
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x13a74
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x13a78
    imm(c,C::Al,Op::Mov,R::R0,R::R0,262144)?;
    // 0x13a7c
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x13a80
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,868)?)?;
    // 0x13a84
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1136)?;
    // 0x13a88
    imm(c,C::Al,Op::Add,R::new(5)?,R::new(8)?,16)?;
    // 0x13a8c
    imm(c,C::Al,Op::Orr,R::R0,R::R0,36864)?;
    // 0x13a90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x13a94
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x13a98
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,4)?)?;
    // 0x13a9c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,5504)?;
    // 0x13aa0
    imm(c,C::Al,Op::Orr,R::R0,R::R0,32768)?;
    // 0x13aa4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x13aa8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x13aac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,8)?)?;
    // 0x13ab0
    imm(c,C::Al,Op::Add,R::R0,R::new(7)?,16)?;
    // 0x13ab4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x13ab8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,388)?;
    // 0x13abc
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x13ac0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,12)?)?;
    // 0x13ac4
    imm(c,C::Al,Op::Add,R::new(7)?,R::new(10)?,68)?;
    // 0x13ac8
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(7)?, R::R0.bit() | R::R1.bit() | R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x13acc
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,88)?)?;
    // 0x13ad0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // 0x13ad4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x13ad8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,7,false)?)?;
    // 0x13adc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x13ae0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,10,false)?)?;
    // 0x13ae4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,11,false)?)?;
    // 0x13ae8
    imm(c,C::Al,Op::Orr,R::R0,R::R0,512)?;
    // 0x13aec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,12)?)?;
    // 0x13af0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x13af4
    imm(c,C::Al,Op::Sub,R::SP,R::R11,28)?;
    // 0x13af8
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    c.check(0x13afc)?;
    const SESSION_REFERENCES:[u32;6]=[
        0x000e_3d16,0xffff_ec2c,0xffff_e1e4,
        0x000e_420f,0x000e_421f,0x000e_2788,
    ];
    for word in SESSION_REFERENCES {c.emit(word)?;}
    c.check(0x13b14)
}


/// Initialize the next codec session frame. This path validates input mode,
/// selects format-specific descriptors, and packs the controller's command
/// parameters. Its final return is still beyond the current coverage.
fn setup_next_codec_session(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x13b14)?;
    // 0x13b14
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x13b18
    imm(c,C::Al,Op::Add,R::R11,R::SP,28)?;
    // 0x13b1c
    imm(c,C::Al,Op::Sub,R::SP,R::SP,124)?;
    // 0x13b20
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R1))?;
    // 0x13b24
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::R0))?;
    // 0x13b28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x13b2c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,1500)?)?;
    // 0x13b30
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x13b34
    c.branch(C::Ne, 0x13b74)?;
    // 0x13b38
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,728)?)?;
    // 0x13b3c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13b40
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x13b44
    c.branch(C::Ne, 0x13b58)?;
    // 0x13b48
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,44)?)?;
    // 0x13b4c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13b50
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x13b54
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,1)?;
    // 0x13b58
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-56)?)?;
    // 0x13b5c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x13b60
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(9)?,R::R1,20)?)?;
    // 0x13b64
    imm(c,C::Al,Op::Add,R::new(7)?,R::R1,3200)?;
    // 0x13b68
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,68)?)?;
    // 0x13b6c
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x13b70
    c.branch(C::Al, 0x13b98)?;
    // 0x13b74
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x13b78
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,7)?;
    // 0x13b7c
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(9)?,R::R1,20)?)?;
    // 0x13b80
    imm(c,C::Al,Op::Add,R::new(6)?,R::R1,3200)?;
    // 0x13b84
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,68)?)?;
    // 0x13b88
    c.branch(C::Ne, 0x13c2c)?;
    // 0x13b8c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13b90
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(6)?))?;
    // 0x13b94
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-56)?)?;
    // 0x13b98
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // 0x13b9c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,496)?;
    // 0x13ba0
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x13ba4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(8)?,R::R1,Shift::Lsl,2)?)?;
    // 0x13ba8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x13bac
    imm(c,C::Al,Op::Sub,R::R1,R::R1,1)?;
    // 0x13bb0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,1)?;
    // 0x13bb4
    c.branch(C::Hi, 0x13bcc)?;
    // 0x13bb8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,44)?)?;
    // 0x13bbc
    imm(c,C::Al,Op::Add,R::R1,R::new(5)?,220)?;
    // 0x13bc0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x13bc4
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x13bc8
    imm(c,C::Eq,Op::Add,R::new(6)?,R::R1,3072)?;
    // 0x13bcc
    imm(c,C::Al,Op::Add,R::R1,R::new(5)?,600)?;
    // 0x13bd0
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x13bd4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13bd8
    imm(c,C::Al,Op::Add,R::R1,R::R1,2048)?;
    // 0x13bdc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-72)?)?;
    // 0x13be0
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,44)?)?;
    // 0x13be4
    imm(c,C::Eq,Op::Cmp,R::R0,R::R0,0)?;
    // 0x13be8
    c.branch(C::Ne, 0x13c48)?;
    // 0x13bec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x13bf0
    c.literal(R::R2,0x14bb4)?;
    // 0x13bf4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,340)?;
    // 0x13bf8
    imm(c,C::Al,Op::Orr,R::R1,R::R1,2048)?;
    // 0x13bfc
    c.call(0x17b0)?;
    // 0x13c00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13c04
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,68)?)?;
    // 0x13c08
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13c0c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x13c10
    imm(c,C::Al,Op::Mov,R::R1,R::R0,324)?;
    // 0x13c14
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x13c18
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x13c1c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1)?;
    // 0x13c20
    imm(c,C::Al,Op::Add,R::R0,R::R0,15)?;
    // 0x13c24
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsr,4,false)?)?;
    // 0x13c28
    c.branch(C::Al, 0x13c48)?;
    // 0x13c2c
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,600)?;
    // 0x13c30
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(6)?))?;
    // 0x13c34
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13c38
    imm(c,C::Al,Op::Add,R::R0,R::R0,2048)?;
    // 0x13c3c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-72)?)?;
    // 0x13c40
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x13c44
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // 0x13c48
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-44)?)?;
    // 0x13c4c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,24)?)?;
    // 0x13c50
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,48)?)?;
    // 0x13c54
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x13c58
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-52)?)?;
    // 0x13c5c
    c.branch(C::Ne, 0x13c84)?;
    // 0x13c60
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13c64
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,68)?)?;
    // 0x13c68
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13c6c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x13c70
    imm(c,C::Al,Op::Mov,R::R1,R::R0,676)?;
    // 0x13c74
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x13c78
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x13c7c
    imm(c,C::Al,Op::Add,R::R0,R::R0,15)?;
    // 0x13c80
    c.emit(arm32::mov_shifted_register(C::Al, R::new(9)?, R::R0, Shift::Lsr, 4)?)?;
    // 0x13c84
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(5)?,16)?)?;
    // 0x13c88
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,28)?)?;
    // 0x13c8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,72)?)?;
    // 0x13c90
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x13c94
    imm(c,C::Al,Op::Cmp,R::R0,R::R12,2)?;
    // 0x13c98
    c.branch(C::Ne, 0x13cf0)?;
    // 0x13c9c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13ca0
    imm(c,C::Al,Op::Mov,R::R1,R::R0,496)?;
    // 0x13ca4
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x13ca8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13cac
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x13cb0
    imm(c,C::Al,Op::Sub,R::R0,R::R0,1)?;
    // 0x13cb4
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x13cb8
    c.branch(C::Hi, 0x13cc8)?;
    // 0x13cbc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x13cc0
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x13cc4
    c.branch(C::Ne, 0x13d18)?;
    // 0x13cc8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,728)?)?;
    // 0x13ccc
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x13cd0
    c.branch(C::Cc, 0x13d18)?;
    // 0x13cd4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,4)?;
    // 0x13cd8
    imm(c,C::Al,Op::Mov,R::LR,R::R0,48)?;
    // 0x13cdc
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,2)?;
    // 0x13ce0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,52)?)?;
    // 0x13ce4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,4)?;
    // 0x13ce8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,48)?)?;
    // 0x13cec
    c.branch(C::Al, 0x13d30)?;
    // 0x13cf0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x13cf4
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x13cf8
    c.branch(C::Ne, 0x13d18)?;
    // 0x13cfc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,4)?;
    // 0x13d00
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,2)?;
    // 0x13d04
    imm(c,C::Al,Op::Mov,R::LR,R::R0,4)?;
    // 0x13d08
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x13d0c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,4)?;
    // 0x13d10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x13d14
    c.branch(C::Al, 0x13d30)?;
    // 0x13d18
    imm(c,C::Al,Op::Mov,R::R0,R::R0,14)?;
    // 0x13d1c
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,5)?;
    // 0x13d20
    imm(c,C::Al,Op::Mov,R::LR,R::R0,10)?;
    // 0x13d24
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x13d28
    imm(c,C::Al,Op::Mov,R::R0,R::R0,10)?;
    // 0x13d2c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x13d30
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,92)?)?;
    // 0x13d34
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(5)?,488)?)?;
    // 0x13d38
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,88)?)?;
    // 0x13d3c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,368)?)?;
    // 0x13d40
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x13d44
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(3)?))?;
    // 0x13d48
    reg(c,C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(9)?)?;
    // 0x13d4c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x13d50
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,76)?)?;
    // 0x13d54
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,40)?)?;
    // 0x13d58
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,44)?)?;
    // 0x13d5c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13d60
    imm(c,C::Hi,Op::Mov,R::R2,R::R0,1)?;
    // 0x13d64
    reg(c,C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?)?;
    // 0x13d68
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R2, Shift::Lsl, 1)?)?;
    // 0x13d6c
    c.emit(arm32::data_register(C::Hi,Op::Orr,R::R0,R::new(3)?,R::R2,Shift::Lsl,1,false)?)?;
    // 0x13d70
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // 0x13d74
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-48)?)?;
    // 0x13d78
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-72)?)?;
    // 0x13d7c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,8)?)?;
    // 0x13d80
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,28)?)?;
    // 0x13d84
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x13d88
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,0)?;
    // 0x13d8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,60)?)?;
    // 0x13d90
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-60)?)?;
    // 0x13d94
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,1)?;
    // 0x13d98
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,2)?;
    // 0x13d9c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,32)?;
    // 0x13da0
    imm(c,C::Eq,Op::Mov,R::R1,R::R0,1)?;
    // 0x13da4
    imm(c,C::Eq,Op::Mov,R::new(7)?,R::R0,48)?;
    // 0x13da8
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,0)?;
    // 0x13dac
    reg(c,C::Al,Op::Orr,R::R1,R::R1,R::R2)?;
    // 0x13db0
    imm(c,C::Eq,Op::Mov,R::new(7)?,R::R0,128)?;
    // 0x13db4
    imm(c,C::Al,Op::Cmp,R::R0,R::R12,2)?;
    // 0x13db8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,56)?)?;
    // 0x13dbc
    c.emit(arm32::signed_multiply_bottom_halves(C::Al,R::R1,R::LR,R::new(6)?)?)?;
    // 0x13dc0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x13dc4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x13dc8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,820)?)?;
    // 0x13dcc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-68)?)?;
    // 0x13dd0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // 0x13dd4
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-40)?)?;
    // 0x13dd8
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R11,-64)?)?;
    // 0x13ddc
    imm(c,C::Al,Op::And,R::new(10)?,R::R0,2)?;
    // 0x13de0
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::new(6)?, Shift::Lsl, 3)?)?;
    // 0x13de4
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,64)?)?;
    // 0x13de8
    c.branch(C::Ne, 0x13e5c)?;
    // 0x13dec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x13df0
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,496)?;
    // 0x13df4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13df8
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?;
    // 0x13dfc
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13e00
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x13e04
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x13e08
    imm(c,C::Al,Op::Sub,R::R0,R::R0,1)?;
    // 0x13e0c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x13e10
    c.branch(C::Hi, 0x13f0c)?;
    // 0x13e14
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x13e18
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x13e1c
    c.branch(C::Eq, 0x13f0c)?;
    // 0x13e20
    imm(c,C::Al,Op::Mov,R::R0,R::R0,532)?;
    // 0x13e24
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x13e28
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x13e2c
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R1, Shift::Lsl, 4)?)?;
    // 0x13e30
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x13e34
    imm(c,C::Al,Op::Add,R::new(5)?,R::R0,3)?;
    // 0x13e38
    imm(c,C::Al,Op::Mov,R::R0,R::R0,536)?;
    // 0x13e3c
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x13e40
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x13e44
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x13e48
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x13e4c
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x13e50
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-64)?)?;
    // 0x13e54
    imm(c,C::Al,Op::Add,R::R0,R::R0,3)?;
    // 0x13e58
    c.branch(C::Al, 0x13f30)?;
    // 0x13e5c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-52)?)?;
    // 0x13e60
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1)?;
    // 0x13e64
    c.branch(C::Ne, 0x13f78)?;
    // 0x13e68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13e6c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,56)?)?;
    // 0x13e70
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 0x13e74
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,68)?)?;
    // 0x13e78
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13e7c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x13e80
    c.branch(C::Eq, 0x13e9c)?;
    // 0x13e84
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,60)?)?;
    // 0x13e88
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x13e8c
    c.call(0x81850)?;
    // 0x13e90
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x13e94
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,3)?;
    // 0x13e98
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::R11,-48)?)?;
    // 0x13e9c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,676)?;
    // 0x13ea0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x13ea4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x13ea8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x13eac
    imm(c,C::Al,Op::Mov,R::R0,R::R0,916)?;
    // 0x13eb0
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x13eb4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x13eb8
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 4)?)?;
    // 0x13ebc
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x13ec0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // 0x13ec4
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,21)?;
    // 0x13ec8
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x13ecc
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x13ed0
    imm(c,C::Ne,Op::Mov,R::new(4)?,R::R0,3)?;
    // 0x13ed4
    reg(c,C::Al,Op::Add,R::new(5)?,R::R0,R::new(4)?)?;
    // 0x13ed8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,716)?;
    // 0x13edc
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x13ee0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x13ee4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,956)?;
    // 0x13ee8
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x13eec
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x13ef0
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(3)?)?)?;
    // 0x13ef4
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x13ef8
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-64)?)?;
    // 0x13efc
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x13f00
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x13f04
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x13f08
    c.branch(C::Al, 0x14070)?;
    // 0x13f0c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x13f10
    imm(c,C::Al,Op::Mov,R::R1,R::R0,3)?;
    // 0x13f14
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,88)?;
    // 0x13f18
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-48)?)?;
    // 0x13f1c
    imm(c,C::Al,Op::Bic,R::R0,R::R0,1)?;
    // 0x13f20
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x13f24
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,56)?;
    // 0x13f28
    imm(c,C::Eq,Op::Mov,R::new(5)?,R::R0,56)?;
    // 0x13f2c
    imm(c,C::Ne,Op::Mov,R::R0,R::R0,88)?;
    // 0x13f30
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // 0x13f34
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // 0x13f38
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x13f3c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,12)?)?;
    // 0x13f40
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(3)?,R::R1,Shift::Lsl,2)?)?;
    // 0x13f44
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x13f48
    imm(c,C::Al,Op::Sub,R::R1,R::R1,1)?;
    // 0x13f4c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,1)?;
    // 0x13f50
    c.branch(C::Hi, 0x13f60)?;
    // 0x13f54
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,44)?)?;
    // 0x13f58
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x13f5c
    c.branch(C::Ne, 0x14204)?;
    // 0x13f60
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,68)?)?;
    // 0x13f64
    imm(c,C::Al,Op::Bic,R::R1,R::R0,1)?;
    // 0x13f68
    imm(c,C::Al,Op::Mov,R::R0,R::R0,84)?;
    // 0x13f6c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x13f70
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,42)?;
    // 0x13f74
    c.branch(C::Al, 0x141f8)?;
    // 0x13f78
    imm(c,C::Al,Op::Cmp,R::R0,R::R12,7)?;
    // 0x13f7c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x13f80
    c.branch(C::Eq, 0x13fd0)?;
    // 0x13f84
    imm(c,C::Al,Op::Cmp,R::R0,R::R12,11)?;
    // 0x13f88
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x13f8c
    c.branch(C::Ne, 0x1402c)?;
    // 0x13f90
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x13f94
    imm(c,C::Al,Op::Mov,R::R2,R::R0,8)?;
    // 0x13f98
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13f9c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,68)?)?;
    // 0x13fa0
    imm(c,C::Al,Op::Orr,R::R2,R::R2,6144)?;
    // 0x13fa4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x13fa8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x13fac
    imm(c,C::Al,Op::Mov,R::R1,R::R0,312)?;
    // 0x13fb0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,5120)?;
    // 0x13fb4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x13fb8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x13fbc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // 0x13fc0
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x13fc4
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x13fc8
    imm(c,C::Al,Op::Add,R::new(5)?,R::R0,3)?;
    // 0x13fcc
    c.branch(C::Al, 0x140e0)?;
    // 0x13fd0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x13fd4
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x13fd8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x13fdc
    c.branch(C::Eq, 0x1402c)?;
    // 0x13fe0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x13fe4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // 0x13fe8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x13fec
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x13ff0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,272)?;
    // 0x13ff4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x13ff8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x13ffc
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R1, Shift::Lsl, 4)?)?;
    c.check(0x14000)
}


/// Compute buffer and frame sizes for the selected controller format.
/// The twelve-word switch table is encoded as named target addresses,
/// rather than being misinterpreted as executable A32 instructions.
fn compute_codec_stream_dimensions(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x14000)?;
    // 0x14000
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14004
    imm(c,C::Al,Op::Add,R::new(5)?,R::R0,3)?;
    // 0x14008
    imm(c,C::Al,Op::Mov,R::R0,R::R0,276)?;
    // 0x1400c
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x14010
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x14014
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x14018
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x1401c
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14020
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-64)?)?;
    // 0x14024
    imm(c,C::Al,Op::Add,R::R0,R::R0,3)?;
    // 0x14028
    c.branch(C::Al, 0x14204)?;
    // 0x1402c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,76)?)?;
    // 0x14030
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R1))?;
    // 0x14034
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // 0x14038
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 4)?)?;
    // 0x1403c
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14040
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // 0x14044
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // 0x14048
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x1404c
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x14050
    imm(c,C::Al,Op::Mov,R::R1,R::R0,21)?;
    // 0x14054
    imm(c,C::Ne,Op::Mov,R::new(4)?,R::R0,3)?;
    // 0x14058
    reg(c,C::Al,Op::Add,R::new(5)?,R::R0,R::new(4)?)?;
    // 0x1405c
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::R1)?)?;
    // 0x14060
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x14064
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14068
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-64)?)?;
    // 0x1406c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x14070
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x14074
    reg(c,C::Al,Op::Add,R::R0,R::R0,R::new(4)?)?;
    // 0x14078
    imm(c,C::Al,Op::Cmp,R::R0,R::R12,11)?;
    // 0x1407c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x14080
    c.branch(C::Hi, 0x14204)?;
    // 0x14084
    imm(c,C::Al,Op::Add,R::R2,R::PC,8)?;
    // 0x14088
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-40)?)?;
    // 0x1408c
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R12, Shift::Lsl, 2)?)?;
    // 0x14090
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
    c.check(0x14094)?;
    const FORMAT_SIZE_DISPATCH:[u32;12]=[0x1411c,0x1417c,0x14204,0x1412c,0x141d8,0x141e0,0x14204,0x14204,0x14138,0x140c4,0x140c4,0x140e0];
    for target in FORMAT_SIZE_DISPATCH {c.emit(target)?;}
    c.check(0x140c4)?;
    // 0x140c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x140c8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x140cc
    c.branch(C::Eq, 0x14144)?;
    // 0x140d0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // 0x140d4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,21)?;
    // 0x140d8
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::R1)?)?;
    // 0x140dc
    c.branch(C::Al, 0x141c4)?;
    // 0x140e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x140e4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,6272)?;
    // 0x140e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,12)?)?;
    // 0x140ec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,68)?)?;
    // 0x140f0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x140f4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x140f8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1456)?;
    // 0x140fc
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x14100
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x14104
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x14108
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // 0x1410c
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x14110
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14114
    imm(c,C::Al,Op::Add,R::R0,R::R0,3)?;
    // 0x14118
    c.branch(C::Al, 0x14210)?;
    // 0x1411c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x14120
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,3,false)?)?;
    // 0x14124
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x14128
    c.branch(C::Al, 0x141ec)?;
    // 0x1412c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x14130
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 4)?)?;
    // 0x14134
    c.branch(C::Al, 0x141ec)?;
    // 0x14138
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x1413c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x14140
    c.branch(C::Ne, 0x141bc)?;
    // 0x14144
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x14148
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,12)?)?;
    // 0x1414c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,68)?)?;
    // 0x14150
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x14154
    imm(c,C::Al,Op::Mov,R::R2,R::R0,956)?;
    // 0x14158
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x1415c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,716)?;
    // 0x14160
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,21)?;
    // 0x14164
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x14168
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x1416c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x14170
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x14174
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(3)?)?)?;
    // 0x14178
    c.branch(C::Al, 0x141c8)?;
    // 0x1417c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x14180
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x14184
    c.branch(C::Ne, 0x141bc)?;
    // 0x14188
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x1418c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,956)?;
    // 0x14190
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x14194
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,12)?)?;
    // 0x14198
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,68)?)?;
    // 0x1419c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x141a0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x141a4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,716)?;
    // 0x141a8
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x141ac
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x141b0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x141b4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // 0x141b8
    c.branch(C::Al, 0x141c8)?;
    // 0x141bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x141c0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // 0x141c4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x141c8
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x141cc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // 0x141d0
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-64)?)?;
    // 0x141d4
    c.branch(C::Al, 0x141fc)?;
    // 0x141d8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,26)?;
    // 0x141dc
    c.branch(C::Al, 0x141e4)?;
    // 0x141e0
    imm(c,C::Al,Op::Mov,R::R1,R::R0,21)?;
    // 0x141e4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // 0x141e8
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::R1)?)?;
    // 0x141ec
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x141f0
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x141f4
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-64)?)?;
    // 0x141f8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // 0x141fc
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x14200
    imm(c,C::Ne,Op::Add,R::R0,R::R0,3)?;
    // 0x14204
    imm(c,C::Al,Op::Cmp,R::R0,R::new(10)?,0)?;
    // 0x14208
    imm(c,C::Ne,Op::Cmp,R::R0,R::R12,11)?;
    // 0x1420c
    c.branch(C::Ne, 0x14218)?;
    // 0x14210
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,76)?)?;
    // 0x14214
    c.branch(C::Al, 0x14250)?;
    // 0x14218
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(9)?))?;
    // 0x1421c
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,76)?)?;
    // 0x14220
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // 0x14224
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(9)?,Shift::Lsl,4,false)?)?;
    // 0x14228
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x1422c
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14230
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x14234
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x14238
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1423c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // 0x14240
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x14244
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14248
    imm(c,C::Al,Op::Add,R::R0,R::R0,3)?;
    // 0x1424c
    imm(c,C::Al,Op::Add,R::new(5)?,R::new(5)?,3)?;
    // 0x14250
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,56)?)?;
    // 0x14254
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-48)?)?;
    // 0x14258
    imm(c,C::Al,Op::Mov,R::R1,R::R0,3)?;
    // 0x1425c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x14260
    imm(c,C::Al,Op::Mov,R::new(10)?,R::R0,3)?;
    // 0x14264
    imm(c,C::Al,Op::Sub,R::new(4)?,R::new(6)?,1)?;
    // 0x14268
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,0)?;
    // 0x1426c
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,2)?;
    // 0x14270
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,0)?;
    // 0x14274
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,7)?;
    // 0x14278
    imm(c,C::Ne,Op::Mov,R::R2,R::R0,2)?;
    // 0x1427c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,0)?;
    // 0x14280
    imm(c,C::Ne,Op::Mov,R::new(3)?,R::R0,3)?;
    // 0x14284
    reg(c,C::Al,Op::Add,R::R0,R::new(3)?,R::R0)?;
    // 0x14288
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsr,R::R1,false)?)?;
    // 0x1428c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x14290
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,3)?;
    // 0x14294
    c.emit(arm32::mov_register(C::Hi, R::new(10)?, R::R0))?;
    // 0x14298
    reg(c,C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?)?;
    // 0x1429c
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x142a0
    imm(c,C::Al,Op::Add,R::new(7)?,R::R0,1)?;
    // 0x142a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-60)?)?;
    // 0x142a8
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,5)?;
    // 0x142ac
    imm(c,C::Cs,Op::Mov,R::new(7)?,R::R0,5)?;
    // 0x142b0
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,3)?;
    // 0x142b4
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(7)?))?;
    // 0x142b8
    imm(c,C::Cc,Op::Mov,R::new(5)?,R::R0,3)?;
    // 0x142bc
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x142c0
    c.emit(arm32::mov_register(C::Eq, R::new(5)?, R::new(7)?))?;
    // 0x142c4
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(5)?,R::new(6)?)?)?;
    // 0x142c8
    reg(c,C::Al,Op::Cmp,R::R0,R::new(9)?,R::R0)?;
    // 0x142cc
    c.branch(C::Cs, 0x142e0)?;
    // 0x142d0
    reg(c,C::Al,Op::Add,R::R0,R::new(4)?,R::new(9)?)?;
    // 0x142d4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x142d8
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x142dc
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x142e0
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(7)?,R::new(6)?)?)?;
    // 0x142e4
    reg(c,C::Al,Op::Cmp,R::R0,R::new(9)?,R::R0)?;
    // 0x142e8
    c.branch(C::Cs, 0x142fc)?;
    // 0x142ec
    reg(c,C::Al,Op::Add,R::R0,R::new(4)?,R::new(9)?)?;
    // 0x142f0
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x142f4
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x142f8
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R0))?;
    // 0x142fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x14300
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-64)?)?;
    // 0x14304
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x14308
    c.branch(C::Ne, 0x1434c)?;
    // 0x1430c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x14310
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-40)?)?;
    // 0x14314
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,12)?)?;
    // 0x14318
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,68)?)?;
    // 0x1431c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R2,R::R0,Shift::Lsl,2)?)?;
    // 0x14320
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x14324
    imm(c,C::Al,Op::Mov,R::R1,R::R0,916)?;
    // 0x14328
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x1432c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x14330
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(5)?,R::new(6)?)?)?;
    // 0x14334
    reg(c,C::Al,Op::Cmp,R::R0,R::R1,R::R0)?;
    // 0x14338
    c.branch(C::Ls, 0x1434c)?;
    // 0x1433c
    reg(c,C::Al,Op::Add,R::R0,R::new(4)?,R::R0)?;
    // 0x14340
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x14344
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14348
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x1434c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // 0x14350
    imm(c,C::Al,Op::Mov,R::R1,R::R0,2)?;
    // 0x14354
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,2)?;
    // 0x14358
    c.emit(arm32::mov_register(C::Ls, R::new(7)?, R::R1))?;
    // 0x1435c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,2)?;
    // 0x14360
    c.emit(arm32::mov_register(C::Hi, R::R1, R::new(5)?))?;
    // 0x14364
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // 0x14368
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,964)?)?;
    // 0x1436c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x14370
    imm(c,C::Al,Op::Bic,R::R0,R::new(8)?,1)?;
    // 0x14374
    c.emit(arm32::transfer_word(C::Eq,true,R::new(10)?,R::new(6)?,960)?)?;
    // 0x14378
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,956)?)?;
    // 0x1437c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,4)?;
    // 0x14380
    c.emit(arm32::mov_register(C::Eq, R::R1, R::R0))?;
    // 0x14384
    imm(c,C::Al,Op::Cmp,R::R0,R::new(8)?,10)?;
    // 0x14388
    c.branch(C::Hi, 0x143bc)?;
    // 0x1438c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,2)?;
    // 0x14390
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14394
    imm(c,C::Al,Op::Orr,R::R2,R::R2,1280)?;
    // 0x14398
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R0,Shift::Lsl,R::new(8)?,false)?)?;
    // 0x1439c
    c.branch(C::Eq, 0x143bc)?;
    // 0x143a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x143a4
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-60)?)?;
    // 0x143a8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-44)?)?;
    // 0x143ac
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,60)?)?;
    // 0x143b0
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x143b4
    imm(c,C::Cs,Op::Mov,R::R1,R::R0,4)?;
    // 0x143b8
    c.branch(C::Al, 0x143c8)?;
    // 0x143bc
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-60)?)?;
    // 0x143c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-44)?)?;
    // 0x143c4
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,60)?)?;
    // 0x143c8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,4)?)?;
    // 0x143cc
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,0)?;
    // 0x143d0
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    // 0x143d4
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,2)?;
    // 0x143d8
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x143dc
    c.emit(arm32::mov_register(C::Ne, R::R0, R::R1))?;
    // 0x143e0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(10)?)?)?;
    // 0x143e4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,129)?;
    // 0x143e8
    c.branch(C::Cc, 0x143f8)?;
    // 0x143ec
    imm(c,C::Al,Op::Mov,R::R0,R::R0,128)?;
    // 0x143f0
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0x143f4
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x143f8
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(10)?,R::new(7)?)?)?;
    // 0x143fc
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::new(10)?)?)?;
    c.check(0x14400)
}


/// Switch on frame-specific packet classes to compute the buffer stride,
/// minimum descriptor count and alignment for the selected format.
fn route_frame_formats(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x14400)?;
    // 0x14400
    imm(c,C::Al,Op::Cmp,R::R0,R::new(8)?,2)?;
    // 0x14404
    c.branch(C::Ne, 0x14424)?;
    // 0x14408
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x1440c
    imm(c,C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?;
    // 0x14410
    c.emit(arm32::mov_shifted_register(C::Al, R::new(6)?, R::new(3)?, Shift::Lsr, 1)?)?;
    // 0x14414
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x14418
    imm(c,C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?;
    // 0x1441c
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(3)?, Shift::Lsr, 1)?)?;
    // 0x14420
    c.branch(C::Al, 0x1442c)?;
    // 0x14424
    c.emit(arm32::mov_shifted_register(C::Al, R::new(6)?, R::R1, Shift::Lsl, 1)?)?;
    // 0x14428
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::R2, Shift::Lsl, 1)?)?;
    // 0x1442c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-56)?)?;
    // 0x14430
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,64)?)?;
    // 0x14434
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,0)?;
    // 0x14438
    c.emit(arm32::mov_shifted_register(C::Eq, R::new(5)?, R::R2, Shift::Lsl, 2)?)?;
    // 0x1443c
    c.emit(arm32::mov_shifted_register(C::Eq, R::new(6)?, R::R1, Shift::Lsl, 2)?)?;
    // 0x14440
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,129)?;
    // 0x14444
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-52)?)?;
    // 0x14448
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-48)?)?;
    // 0x1444c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(3)?,R::new(4)?,Shift::Lsr,1,false)?)?;
    // 0x14450
    c.branch(C::Cc, 0x14460)?;
    // 0x14454
    imm(c,C::Al,Op::Mov,R::R0,R::R0,128)?;
    // 0x14458
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0x1445c
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x14460
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-32)?)?;
    // 0x14464
    imm(c,C::Al,Op::Mov,R::R1,R::R0,552)?;
    // 0x14468
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,244)?;
    // 0x1446c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,7168)?;
    // 0x14470
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14474
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(9)?))?;
    // 0x14478
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(8)?,R::R1,Shift::Lsl,0)?)?;
    // 0x1447c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x14480
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,816)?)?;
    // 0x14484
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // 0x14488
    imm(c,C::Al,Op::Add,R::R12,R::R2,12)?;
    // 0x1448c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x14490
    imm(c,C::Al,Op::Mov,R::R1,R::R0,556)?;
    // 0x14494
    imm(c,C::Al,Op::Orr,R::R1,R::R1,7168)?;
    // 0x14498
    c.emit(arm32::mov_register(C::Eq, R::new(7)?, R::R0))?;
    // 0x1449c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x144a0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::new(10)?,Shift::Lsl,8,false)?)?;
    // 0x144a4
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(4)?))?;
    // 0x144a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // 0x144ac
    imm(c,C::Al,Op::Mov,R::R0,R::R0,236)?;
    // 0x144b0
    imm(c,C::Al,Op::Add,R::LR,R::new(3)?,12)?;
    // 0x144b4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x144b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,4)?)?;
    // 0x144bc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,2048)?;
    // 0x144c0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-56)?)?;
    // 0x144c4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x144c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-68)?)?;
    // 0x144cc
    imm(c,C::Ne,Op::Mov,R::R0,R::R0,12288)?;
    // 0x144d0
    imm(c,C::Al,Op::Cmp,R::R0,R::new(4)?,0)?;
    // 0x144d4
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,1024)?;
    // 0x144d8
    imm(c,C::Al,Op::Sub,R::new(7)?,R::R1,1)?;
    // 0x144dc
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::new(7)?)?;
    // 0x144e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,4)?)?;
    // 0x144e4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1264)?;
    // 0x144e8
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(8)?,0)?)?;
    // 0x144ec
    imm(c,C::Al,Op::Orr,R::R0,R::R0,36864)?;
    // 0x144f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,8)?)?;
    // 0x144f4
    imm(c,C::Al,Op::Orr,R::R2,R::new(7)?,1024)?;
    // 0x144f8
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(9)?,0)?)?;
    // 0x144fc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,8)?)?;
    // 0x14500
    c.literal(R::R2,0x15358)?;
    // 0x14504
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // 0x14508
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(9)?,R::R2,Shift::Lsl,0)?)?;
    // 0x1450c
    imm(c,C::Al,Op::Add,R::R2,R::R2,3)?;
    // 0x14510
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsr, 2)?)?;
    // 0x14514
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::R2,R::R0)?)?;
    // 0x14518
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x1451c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,512)?;
    // 0x14520
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // 0x14524
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::new(3)?,R::new(5)?)?)?;
    // 0x14528
    c.literal(R::new(5)?,0x1535c)?;
    // 0x1452c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(7)?,Shift::Lsl,5,false)?)?;
    // 0x14530
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(9)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x14534
    imm(c,C::Al,Op::Sub,R::new(5)?,R::new(5)?,1)?;
    // 0x14538
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,10)?;
    // 0x1453c
    c.branch(C::Hi, 0x1478c)?;
    // 0x14540
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x14544
    imm(c,C::Al,Op::Add,R::new(4)?,R::PC,4)?;
    // 0x14548
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::new(7)?, Shift::Lsl, 1)?)?;
    // 0x1454c
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::new(5)?,R::new(4)?,Shift::Lsl,0)?)?;
    c.check(0x14550)?;
    const FRAME_CLASS_DISPATCH:[u32;11]=[0x1457c,0x145c4,0x14624,0x14778,0x14630,0x1478c,0x1478c,0x14650,0x14674,0x146d8,0x14740];
    for target in FRAME_CLASS_DISPATCH {c.emit(target)?;}
    c.check(0x1457c)?;
    // 0x1457c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x14580
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // 0x14584
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,660)?;
    // 0x14588
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?;
    // 0x1458c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,12)?)?;
    // 0x14590
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x14594
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x14598
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x1459c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,1)?;
    // 0x145a0
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,1)?;
    // 0x145a4
    imm(c,C::Eq,Op::Mov,R::new(7)?,R::R0,2)?;
    // 0x145a8
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::new(7)?)?)?;
    // 0x145ac
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,32)?;
    // 0x145b0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::R1,Shift::Lsl,5,false)?)?;
    // 0x145b4
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 7)?)?;
    // 0x145b8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,780)?)?;
    // 0x145bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,784)?)?;
    // 0x145c0
    c.branch(C::Al, 0x1478c)?;
    // 0x145c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x145c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // 0x145cc
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,496)?;
    // 0x145d0
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?;
    // 0x145d4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,12)?)?;
    // 0x145d8
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x145dc
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x145e0
    imm(c,C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?;
    // 0x145e4
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,1)?;
    // 0x145e8
    c.branch(C::Hi, 0x145f8)?;
    // 0x145ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,44)?)?;
    // 0x145f0
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,0)?;
    // 0x145f4
    c.branch(C::Ne, 0x1478c)?;
    // 0x145f8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x145fc
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,80)?;
    // 0x14600
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x14604
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,4,false)?)?;
    // 0x14608
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,4)?;
    // 0x1460c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::R1,Shift::Lsl,4,false)?)?;
    // 0x14610
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,780)?)?;
    // 0x14614
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x14618
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::new(7)?, Shift::Lsl, 3)?)?;
    // 0x1461c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,784)?)?;
    // 0x14620
    c.branch(C::Al, 0x1478c)?;
    // 0x14624
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x14628
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,796)?)?;
    // 0x1462c
    c.branch(C::Al, 0x1478c)?;
    // 0x14630
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x14634
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,4)?;
    // 0x14638
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,4,false)?)?;
    // 0x1463c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x14640
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x14644
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 1)?)?;
    // 0x14648
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,780)?)?;
    // 0x1464c
    c.branch(C::Al, 0x1478c)?;
    // 0x14650
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x14654
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,32)?;
    // 0x14658
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,5,false)?)?;
    // 0x1465c
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::R0, Shift::Lsl, 8)?)?;
    // 0x14660
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x14664
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,780)?)?;
    // 0x14668
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,784)?)?;
    // 0x1466c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,796)?)?;
    // 0x14670
    c.branch(C::Al, 0x1478c)?;
    // 0x14674
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x14678
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // 0x1467c
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,660)?;
    // 0x14680
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,32)?;
    // 0x14684
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?;
    // 0x14688
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Lsl,7,false)?)?;
    // 0x1468c
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,1024)?;
    // 0x14690
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,12)?)?;
    // 0x14694
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x14698
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x1469c
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x146a0
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x146a4
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,1)?;
    // 0x146a8
    imm(c,C::Eq,Op::Mov,R::new(5)?,R::R0,2)?;
    // 0x146ac
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::R1,R::new(5)?)?)?;
    // 0x146b0
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,4)?;
    // 0x146b4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(5)?,R::R1,Shift::Lsl,4,false)?)?;
    // 0x146b8
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::new(7)?, Shift::Lsl, 7)?)?;
    // 0x146bc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::new(5)?,Shift::Lsl,1,false)?)?;
    // 0x146c0
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(5)?, Shift::Lsl, 1)?)?;
    // 0x146c4
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,780)?)?;
    // 0x146c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,784)?)?;
    // 0x146cc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,788)?)?;
    // 0x146d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,792)?)?;
    // 0x146d4
    c.branch(C::Al, 0x1478c)?;
    // 0x146d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x146dc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // 0x146e0
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,660)?;
    // 0x146e4
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,32)?;
    // 0x146e8
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?;
    // 0x146ec
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Lsl,7,false)?)?;
    // 0x146f0
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,1024)?;
    // 0x146f4
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,12)?)?;
    // 0x146f8
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x146fc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x14700
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x14704
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x14708
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,1)?;
    // 0x1470c
    imm(c,C::Eq,Op::Mov,R::new(5)?,R::R0,2)?;
    // 0x14710
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::R1,R::new(5)?)?)?;
    // 0x14714
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,4)?;
    // 0x14718
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(5)?,R::R1,Shift::Lsl,4,false)?)?;
    // 0x1471c
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::new(7)?, Shift::Lsl, 7)?)?;
    // 0x14720
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::new(5)?,Shift::Lsl,1,false)?)?;
    // 0x14724
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(5)?, Shift::Lsl, 1)?)?;
    // 0x14728
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,780)?)?;
    // 0x1472c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,784)?)?;
    // 0x14730
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,788)?)?;
    // 0x14734
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,792)?)?;
    // 0x14738
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,796)?)?;
    // 0x1473c
    c.branch(C::Al, 0x1478c)?;
    // 0x14740
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x14744
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // 0x14748
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,12)?)?;
    // 0x1474c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,72)?)?;
    // 0x14750
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x14754
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x14758
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,732)?;
    // 0x1475c
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,6144)?;
    // 0x14760
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x14764
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,0)?;
    // 0x14768
    c.branch(C::Eq, 0x1478c)?;
    // 0x1476c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,76)?)?;
    // 0x14770
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,0)?;
    // 0x14774
    c.branch(C::Ne, 0x1478c)?;
    // 0x14778
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x1477c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1024)?;
    // 0x14780
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,792)?)?;
    // 0x14784
    imm(c,C::Al,Op::Mov,R::R2,R::R0,512)?;
    // 0x14788
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(7)?,Shift::Lsl,5,false)?)?;
    // 0x1478c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x14790
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,15360)?;
    // 0x14794
    c.branch(C::Hi, 0x147a4)?;
    // 0x14798
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,48)?)?;
    // 0x1479c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1)?;
    // 0x147a0
    c.branch(C::Ne, 0x147bc)?;
    // 0x147a4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,52)?)?;
    // 0x147a8
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1)?;
    // 0x147ac
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,0)?;
    // 0x147b0
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,796)?)?;
    // 0x147b4
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,1)?;
    // 0x147b8
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,988)?)?;
    // 0x147bc
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,1312)?;
    // 0x147c0
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,780)?;
    // 0x147c4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,872)?;
    // 0x147c8
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x147cc
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,36864)?;
    // 0x147d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,0)?)?;
    // 0x147d4
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,0)?;
    // 0x147d8
    c.branch(C::Eq, 0x14814)?;
    // 0x147dc
    imm(c,C::Al,Op::Add,R::new(5)?,R::new(5)?,127)?;
    // 0x147e0
    imm(c,C::Al,Op::Add,R::R1,R::R12,8)?;
    // 0x147e4
    imm(c,C::Al,Op::Add,R::R0,R::LR,8)?;
    // 0x147e8
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,0)?)?;
    // 0x147ec
    c.emit(arm32::mov_shifted_register(C::Al, R::new(6)?, R::new(5)?, Shift::Lsr, 7)?)?;
    // 0x147f0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R2,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x147f4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(5)?,Shift::Lsr,7,false)?)?;
    // 0x147f8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::LR,0)?)?;
    // 0x147fc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    c.check(0x14800)
}


/// Construct codec-specific command descriptors and dispatch packet variants.
/// The twelve-way jump table preserves repeated target entries explicitly.
fn build_format_packets(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x14800)?;
    // 0x14800
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,4)?)?;
    // 0x14804
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x14808
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::LR,4)?)?;
    // 0x1480c
    c.emit(arm32::mov_register(C::Al, R::LR, R::R0))?;
    // 0x14810
    c.emit(arm32::mov_register(C::Al, R::R12, R::R1))?;
    // 0x14814
    imm(c,C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?;
    // 0x14818
    imm(c,C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?;
    // 0x1481c
    imm(c,C::Al,Op::Add,R::new(3)?,R::new(3)?,4)?;
    // 0x14820
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,5)?;
    // 0x14824
    c.branch(C::Ne, 0x147d0)?;
    // 0x14828
    imm(c,C::Al,Op::Cmp,R::R0,R::new(10)?,1)?;
    // 0x1482c
    c.branch(C::Ne, 0x14878)?;
    // 0x14830
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-36)?)?;
    // 0x14834
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1296)?;
    // 0x14838
    imm(c,C::Al,Op::Add,R::new(7)?,R::R12,4)?;
    // 0x1483c
    imm(c,C::Al,Op::Add,R::new(10)?,R::LR,4)?;
    // 0x14840
    imm(c,C::Al,Op::Orr,R::R1,R::R1,36864)?;
    // 0x14844
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,816)?)?;
    // 0x14848
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,0)?)?;
    // 0x1484c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x14850
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(9)?,0)?)?;
    // 0x14854
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // 0x14858
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x1485c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x14860
    c.emit(arm32::mov_register(C::Eq, R::R1, R::R0))?;
    // 0x14864
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // 0x14868
    reg(c,C::Al,Op::Add,R::R2,R::R2,R::R1)?;
    // 0x1486c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x14870
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-32)?)?;
    // 0x14874
    c.branch(C::Al, 0x14888)?;
    // 0x14878
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R12))?;
    // 0x1487c
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-32)?)?;
    // 0x14880
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-36)?)?;
    // 0x14884
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::LR))?;
    // 0x14888
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,2316)?)?;
    // 0x1488c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-48)?)?;
    // 0x14890
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x14894
    c.branch(C::Ne, 0x148a4)?;
    // 0x14898
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,2312)?)?;
    // 0x1489c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x148a0
    c.branch(C::Eq, 0x14a3c)?;
    // 0x148a4
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,276)?;
    // 0x148a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,816)?)?;
    // 0x148ac
    imm(c,C::Al,Op::Add,R::R0,R::new(7)?,4)?;
    // 0x148b0
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x148b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x148b8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,0)?)?;
    // 0x148bc
    imm(c,C::Al,Op::Add,R::new(3)?,R::new(10)?,4)?;
    // 0x148c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x148c4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x148c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // 0x148cc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-68)?)?;
    // 0x148d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,52)?)?;
    // 0x148d4
    c.emit(arm32::mov_register(C::Eq, R::new(6)?, R::R1))?;
    // 0x148d8
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(6)?,R::new(5)?)?)?;
    // 0x148dc
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::new(6)?,R::new(5)?,R::R2)?)?;
    // 0x148e0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // 0x148e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x148e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-36)?)?;
    // 0x148ec
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,816)?)?;
    // 0x148f0
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,280)?;
    // 0x148f4
    imm(c,C::Al,Op::Add,R::R1,R::R0,4)?;
    // 0x148f8
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R12))?;
    // 0x148fc
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?;
    // 0x14900
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x14904
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x14908
    imm(c,C::Al,Op::Add,R::R0,R::new(3)?,4)?;
    // 0x1490c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x14910
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x14914
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-52)?)?;
    // 0x14918
    c.emit(arm32::mov_register(C::Eq, R::new(4)?, R::R2))?;
    // 0x1491c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x14920
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x14924
    reg(c,C::Al,Op::Add,R::R2,R::new(4)?,R::new(7)?)?;
    // 0x14928
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-40)?)?;
    // 0x1492c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1376)?;
    // 0x14930
    c.branch(C::Ls, 0x1497c)?;
    // 0x14934
    imm(c,C::Al,Op::Sub,R::R0,R::R2,1376)?;
    // 0x14938
    c.literal(R::R1,0x157ec)?;
    // 0x1493c
    c.literal(R::R2,0x157f0)?;
    // 0x14940
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,137)?;
    // 0x14944
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x14948
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?;
    // 0x1494c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,8)?;
    // 0x14950
    c.call(0x2ec94)?;
    // 0x14954
    c.literal(R::R1,0x157f4)?;
    // 0x14958
    imm(c,C::Al,Op::Mov,R::R0,R::R0,8)?;
    // 0x1495c
    c.call(0x2ec94)?;
    // 0x14960
    imm(c,C::Al,Op::Mov,R::R0,R::R0,700)?;
    // 0x14964
    imm(c,C::Al,Op::Mov,R::R1,R::R0,8)?;
    // 0x14968
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x1496c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,8192)?;
    // 0x14970
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x14974
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x14978
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x1497c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,988)?)?;
    // 0x14980
    imm(c,C::Al,Op::Add,R::new(3)?,R::R1,4)?;
    // 0x14984
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,0)?)?;
    // 0x14988
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,38144)?;
    // 0x1498c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x14990
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,4)?;
    // 0x14994
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x14998
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,2)?;
    // 0x1499c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1)?;
    // 0x149a0
    imm(c,C::Eq,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x149a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x149a8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,16)?)?;
    // 0x149ac
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,11)?;
    // 0x149b0
    c.branch(C::Hi, 0x150f4)?;
    // 0x149b4
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 2)?)?;
    // 0x149b8
    imm(c,C::Al,Op::Add,R::new(3)?,R::PC,0)?;
    // 0x149bc
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0x149c0)?;
    const CODEC_PACKET_DISPATCH:[u32;12]=[0x149f0,0x14a4c,0x14bb8,0x14c0c,0x14f5c,0x1500c,0x150f4,0x1510c,0x15144,0x150f4,0x15360,0x156fc];
    for target in CODEC_PACKET_DISPATCH {c.emit(target)?;}
    c.check(0x149f0)?;
    // 0x149f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,108)?)?;
    // 0x149f4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // 0x149f8
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x149fc
    c.call(0xfce4)?;
    // 0x14a00
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,248)?)?;
    // 0x14a04
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,292)?;
    // 0x14a08
    imm(c,C::Al,Op::Add,R::R2,R::R0,3072)?;
    // 0x14a0c
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x14a10
    c.call(0xffbc)?;
    // 0x14a14
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-72)?)?;
    // 0x14a18
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(5)?))?;
    // 0x14a1c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x14a20
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,820)?)?;
    // 0x14a24
    imm(c,C::Al,Op::Tst,R::R0,R::R0,2)?;
    // 0x14a28
    c.branch(C::Eq, 0x15808)?;
    // 0x14a2c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,252)?)?;
    // 0x14a30
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x14a34
    c.call(0x104d8)?;
    // 0x14a38
    c.branch(C::Al, 0x15808)?;
    // 0x14a3c
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 0x14a40
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(10)?))?;
    // 0x14a44
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x14a48
    c.branch(C::Al, 0x148ec)?;
    // 0x14a4c
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x14a50
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14a54
    c.call(0x11b58)?;
    // 0x14a58
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x14a5c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14a60
    c.call(0x11e10)?;
    // 0x14a64
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x14a68
    imm(c,C::Al,Op::Mov,R::R1,R::R0,524)?;
    // 0x14a6c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,520)?;
    // 0x14a70
    imm(c,C::Al,Op::Mov,R::R2,R::R0,100)?;
    // 0x14a74
    imm(c,C::Al,Op::Add,R::new(7)?,R::new(5)?,608)?;
    // 0x14a78
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(5)?))?;
    // 0x14a7c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x14a80
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14a84
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x14a88
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,248)?)?;
    // 0x14a8c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14a90
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x14a94
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14a98
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14a9c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x14aa0
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x14aa4
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,4)?)?;
    // 0x14aa8
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14aac
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x14ab0
    imm(c,C::Al,Op::Add,R::new(3)?,R::R1,12)?;
    // 0x14ab4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x14ab8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x14abc
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x14ac0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14ac4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14ac8
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14acc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14ad0
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x14ad4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x14ad8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    // 0x14adc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14ae0
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x14ae4
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x14ae8
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x14aec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x14af0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x14af4
    c.literal(R::new(5)?,0x15adc)?;
    // 0x14af8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    // 0x14afc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14b00
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14b04
    c.call(0x2ec94)?;
    // 0x14b08
    c.literal(R::new(6)?,0x15ad8)?;
    // 0x14b0c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14b10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x14b14
    c.call(0x2ec94)?;
    // 0x14b18
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14b1c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,524)?;
    // 0x14b20
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,520)?;
    // 0x14b24
    imm(c,C::Al,Op::Mov,R::R2,R::R0,84)?;
    // 0x14b28
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x14b2c
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14b30
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x14b34
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14b38
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14b3c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x14b40
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x14b44
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,4)?)?;
    // 0x14b48
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14b4c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x14b50
    imm(c,C::Al,Op::Add,R::new(3)?,R::R1,12)?;
    // 0x14b54
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x14b58
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x14b5c
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x14b60
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14b64
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14b68
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14b6c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14b70
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x14b74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x14b78
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    // 0x14b7c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14b80
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x14b84
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x14b88
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x14b8c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x14b90
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14b94
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x14b98
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    // 0x14b9c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14ba0
    c.call(0x2ec94)?;
    // 0x14ba4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14ba8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x14bac
    c.call(0x2ec94)?;
    // 0x14bb0
    c.branch(C::Al, 0x15808)?;
    // 0x14bb4
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(3)?,R::LR,R::new(6)?,Shift::Lsl,R::SP,false)?)?;
    // 0x14bb8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x14bbc
    imm(c,C::Al,Op::Mov,R::R1,R::R0,496)?;
    // 0x14bc0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x14bc4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x14bc8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x14bcc
    imm(c,C::Al,Op::Sub,R::R0,R::R0,1)?;
    // 0x14bd0
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x14bd4
    c.branch(C::Hi, 0x157f8)?;
    // 0x14bd8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x14bdc
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x14be0
    c.branch(C::Eq, 0x157f8)?;
    // 0x14be4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,108)?)?;
    // 0x14be8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // 0x14bec
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x14bf0
    c.call(0xfce4)?;
    // 0x14bf4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,112)?)?;
    // 0x14bf8
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,220)?;
    // 0x14bfc
    imm(c,C::Al,Op::Add,R::R2,R::R0,3072)?;
    c.check(0x14c00)
}


/// Build packed codec stream flags, update controller buffer words and
/// conditionally initialize output packet metadata.
fn build_encoded_stream_flags(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x14c00)?;
    // 0x14c00
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x14c04
    c.call(0xffbc)?;
    // 0x14c08
    c.branch(C::Al, 0x15804)?;
    // 0x14c0c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,524)?;
    // 0x14c10
    imm(c,C::Al,Op::Mov,R::R2,R::R0,84)?;
    // 0x14c14
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,520)?;
    // 0x14c18
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(5)?))?;
    // 0x14c1c
    c.literal(R::new(4)?,0x15adc)?;
    // 0x14c20
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,37888)?;
    // 0x14c24
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x14c28
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14c2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,4)?)?;
    // 0x14c30
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x14c34
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,8)?)?;
    // 0x14c38
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,1472)?)?;
    // 0x14c3c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x14c40
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x14c44
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14c48
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,12)?)?;
    // 0x14c4c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14c50
    imm(c,C::Al,Op::Add,R::R2,R::R1,20)?;
    // 0x14c54
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14c58
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14c5c
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14c60
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,16)?)?;
    // 0x14c64
    imm(c,C::Al,Op::Add,R::R2,R::R0,20)?;
    // 0x14c68
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1476)?)?;
    // 0x14c6c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14c70
    imm(c,C::Al,Op::Add,R::R1,R::R1,255)?;
    // 0x14c74
    imm(c,C::Al,Op::Bic,R::R1,R::R1,255)?;
    // 0x14c78
    imm(c,C::Al,Op::Orr,R::R1,R::R1,1)?;
    // 0x14c7c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x14c80
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14c84
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x14c88
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1476)?)?;
    // 0x14c8c
    c.call(0x2ec94)?;
    // 0x14c90
    c.literal(R::new(5)?,0x15ad8)?;
    // 0x14c94
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14c98
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14c9c
    c.call(0x2ec94)?;
    // 0x14ca0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14ca4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,100)?;
    // 0x14ca8
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,520)?;
    // 0x14cac
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x14cb0
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?;
    // 0x14cb4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // 0x14cb8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14cbc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14cc0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x14cc4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1472)?)?;
    // 0x14cc8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x14ccc
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x14cd0
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14cd4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x14cd8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x14cdc
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x14ce0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14ce4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14ce8
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14cec
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14cf0
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x14cf4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1476)?)?;
    // 0x14cf8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14cfc
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x14d00
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x14d04
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x14d08
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x14d0c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14d10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x14d14
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1476)?)?;
    // 0x14d18
    c.call(0x2ec94)?;
    // 0x14d1c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14d20
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14d24
    c.call(0x2ec94)?;
    // 0x14d28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14d2c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,116)?;
    // 0x14d30
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x14d34
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // 0x14d38
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14d3c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14d40
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x14d44
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,732)?)?;
    // 0x14d48
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x14d4c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x14d50
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14d54
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x14d58
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x14d5c
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x14d60
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14d64
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14d68
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14d6c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14d70
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x14d74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,736)?)?;
    // 0x14d78
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14d7c
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x14d80
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x14d84
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x14d88
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x14d8c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14d90
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x14d94
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,736)?)?;
    // 0x14d98
    c.call(0x2ec94)?;
    // 0x14d9c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14da0
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14da4
    c.call(0x2ec94)?;
    // 0x14da8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14dac
    imm(c,C::Al,Op::Mov,R::R2,R::R0,132)?;
    // 0x14db0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // 0x14db4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14db8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14dbc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x14dc0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1480)?)?;
    // 0x14dc4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x14dc8
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x14dcc
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14dd0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x14dd4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x14dd8
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x14ddc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14de0
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14de4
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14de8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14dec
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x14df0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1484)?)?;
    // 0x14df4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14df8
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x14dfc
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x14e00
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x14e04
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x14e08
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14e0c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x14e10
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1484)?)?;
    // 0x14e14
    c.call(0x2ec94)?;
    // 0x14e18
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14e1c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14e20
    c.call(0x2ec94)?;
    // 0x14e24
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14e28
    imm(c,C::Al,Op::Mov,R::R2,R::R0,116)?;
    // 0x14e2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // 0x14e30
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14e34
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14e38
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x14e3c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1480)?)?;
    // 0x14e40
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x14e44
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x14e48
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14e4c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x14e50
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x14e54
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x14e58
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(3)?))?;
    // 0x14e5c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14e60
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14e64
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14e68
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14e6c
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x14e70
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1484)?)?;
    // 0x14e74
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14e78
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x14e7c
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x14e80
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x14e84
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x14e88
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14e8c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x14e90
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1484)?)?;
    // 0x14e94
    c.call(0x2ec94)?;
    // 0x14e98
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14e9c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14ea0
    c.call(0x2ec94)?;
    // 0x14ea4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,988)?)?;
    // 0x14ea8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x14eac
    c.branch(C::Ne, 0x15808)?;
    // 0x14eb0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14eb4
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,524)?;
    // 0x14eb8
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,520)?;
    // 0x14ebc
    imm(c,C::Al,Op::Mov,R::R2,R::R0,68)?;
    // 0x14ec0
    c.literal(R::new(4)?,0x15adc)?;
    // 0x14ec4
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?;
    // 0x14ec8
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,37888)?;
    // 0x14ecc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x14ed0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14ed4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14ed8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,4)?)?;
    // 0x14edc
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,992)?)?;
    // 0x14ee0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x14ee4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    // 0x14ee8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x14eec
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x14ef0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14ef4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14ef8
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14efc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14f00
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x14f04
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,984)?)?;
    // 0x14f08
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14f0c
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x14f10
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x14f14
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x14f18
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x14f1c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14f20
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x14f24
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,984)?)?;
    // 0x14f28
    c.call(0x2ec94)?;
    // 0x14f2c
    c.literal(R::new(7)?,0x15ad8)?;
    // 0x14f30
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14f34
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x14f38
    c.call(0x2ec94)?;
    // 0x14f3c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14f40
    imm(c,C::Al,Op::Mov,R::R2,R::R0,68)?;
    // 0x14f44
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x14f48
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x14f4c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14f50
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14f54
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,4)?)?;
    // 0x14f58
    c.branch(C::Al, 0x15694)?;
    // 0x14f5c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x14f60
    c.call(0xfa0c)?;
    // 0x14f64
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-72)?)?;
    // 0x14f68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x14f6c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,820)?)?;
    // 0x14f70
    imm(c,C::Al,Op::Tst,R::R0,R::R0,3)?;
    // 0x14f74
    c.emit(arm32::mov_register(C::Ne, R::R1, R::new(5)?))?;
    // 0x14f78 (conditional link)
    c.emit(arm32::branch(C::Ne,c.at as u32,0x109cc,true)?)?;
    // 0x14f7c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x14f80
    imm(c,C::Al,Op::Mov,R::R1,R::R0,524)?;
    // 0x14f84
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,520)?;
    // 0x14f88
    imm(c,C::Al,Op::Mov,R::R2,R::R0,148)?;
    // 0x14f8c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x14f90
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14f94
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x14f98
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x14f9c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x14fa0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x14fa4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,248)?)?;
    // 0x14fa8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x14fac
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x14fb0
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x14fb4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x14fb8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x14fbc
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x14fc0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x14fc4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x14fc8
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x14fcc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x14fd0
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x14fd4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,608)?)?;
    // 0x14fd8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x14fdc
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x14fe0
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x14fe4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x14fe8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x14fec
    c.literal(R::R1,0x15adc)?;
    // 0x14ff0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x14ff4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,608)?)?;
    // 0x14ff8
    c.call(0x2ec94)?;
    // 0x14ffc
    c.literal(R::R1,0x15ad8)?;
    c.check(0x15000)
}


/// Assemble another codec command record from the active input descriptor
/// and send it to the shared buffer writer. The two literal words supply
/// controller-table negative offsets, not executable opcodes.
fn configure_secondary_codec_packing(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x15000)?;
    // 0x15000
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15004
    c.call(0x2ec94)?;
    // 0x15008
    c.branch(C::Al, 0x15804)?;
    // 0x1500c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x15010
    c.call(0xfa0c)?;
    // 0x15014
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,728)?)?;
    // 0x15018
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x1501c
    c.branch(C::Ne, 0x150ac)?;
    // 0x15020
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x15024
    imm(c,C::Al,Op::Mov,R::R1,R::R0,524)?;
    // 0x15028
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,520)?;
    // 0x1502c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,148)?;
    // 0x15030
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x15034
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x15038
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x1503c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x15040
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x15044
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x15048
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,252)?)?;
    // 0x1504c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x15050
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x15054
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x15058
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x1505c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x15060
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x15064
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x15068
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x1506c
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x15070
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x15074
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x15078
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,612)?)?;
    // 0x1507c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x15080
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x15084
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x15088
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x1508c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x15090
    c.literal(R::R1,0x15adc)?;
    // 0x15094
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15098
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,612)?)?;
    // 0x1509c
    c.call(0x2ec94)?;
    // 0x150a0
    c.literal(R::R1,0x15ad8)?;
    // 0x150a4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x150a8
    c.call(0x2ec94)?;
    // 0x150ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-72)?)?;
    // 0x150b0
    c.literal(R::R2,0x15ad4)?;
    // 0x150b4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x150b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,820)?)?;
    // 0x150bc
    imm(c,C::Al,Op::And,R::R1,R::R0,2)?;
    // 0x150c0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x150c4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x150c8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,4)?;
    // 0x150cc
    imm(c,C::Cc,Op::Mov,R::R0,R::R0,1)?;
    // 0x150d0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3072)?;
    // 0x150d4
    c.call(0x17b0)?;
    // 0x150d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,608)?)?;
    // 0x150dc
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(5)?))?;
    // 0x150e0
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x150e4
    c.branch(C::Eq, 0x15808)?;
    // 0x150e8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,248)?)?;
    // 0x150ec
    imm(c,C::Al,Op::Add,R::R0,R::new(10)?,292)?;
    // 0x150f0
    c.branch(C::Al, 0x15134)?;
    // 0x150f4
    c.literal(R::R2,0x15ad4)?;
    // 0x150f8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,95)?;
    // 0x150fc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x15100
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3072)?;
    // 0x15104
    c.call(0x17b0)?;
    // 0x15108
    c.branch(C::Al, 0x15804)?;
    // 0x1510c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x15110
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(5)?))?;
    // 0x15114
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x15118
    c.branch(C::Eq, 0x15808)?;
    // 0x1511c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2560)?)?;
    // 0x15120
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // 0x15124
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x15128
    c.call(0xfce4)?;
    // 0x1512c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2564)?)?;
    // 0x15130
    imm(c,C::Al,Op::Add,R::R0,R::new(10)?,220)?;
    // 0x15134
    imm(c,C::Al,Op::Add,R::R2,R::R0,3072)?;
    // 0x15138
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x1513c
    c.call(0xffbc)?;
    // 0x15140
    c.branch(C::Al, 0x15808)?;
    // 0x15144
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x15148
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1514c
    c.call(0x11b58)?;
    // 0x15150
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x15154
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,524)?;
    // 0x15158
    imm(c,C::Al,Op::Mov,R::R2,R::R0,116)?;
    // 0x1515c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,520)?;
    // 0x15160
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,652)?;
    // 0x15164
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?;
    // 0x15168
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x1516c
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x15170
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,37888)?;
    // 0x15174
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(3)?))?;
    // 0x15178
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x1517c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x15180
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x15184
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x15188
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,732)?)?;
    // 0x1518c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x15190
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    // 0x15194
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x15198
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x1519c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x151a0
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x151a4
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x151a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x151ac
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x151b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,736)?)?;
    // 0x151b4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x151b8
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x151bc
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x151c0
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x151c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x151c8
    c.literal(R::R1,0x15adc)?;
    // 0x151cc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x151d0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,736)?)?;
    // 0x151d4
    c.call(0x2ec94)?;
    // 0x151d8
    c.literal(R::R1,0x15ad8)?;
    // 0x151dc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x151e0
    c.call(0x2ec94)?;
    // 0x151e4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x151e8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x151ec
    c.call(0x11e10)?;
    // 0x151f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x151f4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x151f8
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,8)?;
    // 0x151fc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x15200
    imm(c,C::Al,Op::Add,R::R2,R::R1,4)?;
    // 0x15204
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x15208
    imm(c,C::Al,Op::Mov,R::R2,R::R0,132)?;
    // 0x1520c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x15210
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,0)?)?;
    // 0x15214
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,4)?)?;
    // 0x15218
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(5)?))?;
    // 0x1521c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,68)?)?;
    // 0x15220
    imm(c,C::Al,Op::Add,R::new(5)?,R::new(10)?,248)?;
    // 0x15224
    imm(c,C::Al,Op::Add,R::new(4)?,R::new(10)?,608)?;
    // 0x15228
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x1522c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x15230
    imm(c,C::Al,Op::Add,R::new(3)?,R::R1,12)?;
    // 0x15234
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    // 0x15238
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x1523c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x15240
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x15244
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x15248
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x1524c
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x15250
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x15254
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x15258
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x1525c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x15260
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x15264
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x15268
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x1526c
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x15270
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x15274
    c.literal(R::R1,0x15adc)?;
    // 0x15278
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x1527c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x15280
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15284
    c.call(0x2ec94)?;
    // 0x15288
    c.literal(R::R1,0x15ad8)?;
    // 0x1528c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15290
    c.call(0x2ec94)?;
    // 0x15294
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x15298
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x1529c
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,520)?;
    // 0x152a0
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,8)?;
    // 0x152a4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x152a8
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?;
    // 0x152ac
    imm(c,C::Al,Op::Add,R::R2,R::R1,4)?;
    // 0x152b0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x152b4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,116)?;
    // 0x152b8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x152bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,0)?)?;
    // 0x152c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x152c4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,68)?)?;
    // 0x152c8
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x152cc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x152d0
    imm(c,C::Al,Op::Add,R::new(3)?,R::R1,12)?;
    // 0x152d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    // 0x152d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x152dc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x152e0
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x152e4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x152e8
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x152ec
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x152f0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x152f4
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x152f8
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x152fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x15300
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x15304
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x15308
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x1530c
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x15310
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x15314
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x15318
    c.literal(R::R1,0x15adc)?;
    // 0x1531c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x15320
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x15324
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15328
    c.call(0x2ec94)?;
    // 0x1532c
    c.literal(R::R1,0x15ad8)?;
    // 0x15330
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15334
    c.call(0x2ec94)?;
    // 0x15338
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,988)?)?;
    // 0x1533c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x15340
    c.branch(C::Ne, 0x15808)?;
    // 0x15344
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x15348
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,524)?;
    // 0x1534c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,68)?;
    // 0x15350
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,37888)?;
    // 0x15354
    c.branch(C::Al, 0x15604)?;
    c.check(0x15358)?;
    const BUFFER_FIELD_OFFSETS:[u32;2]=[0xffff_e7a8,0xffff_e1e4];
    for offset in BUFFER_FIELD_OFFSETS {c.emit(offset)?;}
    c.check(0x15360)?;
    // 0x15360
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x15364
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x15368
    c.call(0x11b58)?;
    // 0x1536c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x15370
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,524)?;
    // 0x15374
    imm(c,C::Al,Op::Mov,R::R2,R::R0,116)?;
    // 0x15378
    imm(c,C::Al,Op::Mov,R::new(10)?,R::R0,520)?;
    // 0x1537c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,652)?;
    // 0x15380
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?;
    // 0x15384
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x15388
    imm(c,C::Al,Op::Orr,R::new(10)?,R::new(10)?,37888)?;
    // 0x1538c
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,37888)?;
    // 0x15390
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x15394
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x15398
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x1539c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,4)?)?;
    // 0x153a0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,732)?)?;
    // 0x153a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x153a8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    // 0x153ac
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x153b0
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x153b4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x153b8
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x153bc
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x153c0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x153c4
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x153c8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,736)?)?;
    // 0x153cc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x153d0
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x153d4
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x153d8
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x153dc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x153e0
    c.literal(R::R1,0x15adc)?;
    // 0x153e4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x153e8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,736)?)?;
    // 0x153ec
    c.call(0x2ec94)?;
    // 0x153f0
    c.literal(R::R1,0x15ad8)?;
    // 0x153f4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x153f8
    c.call(0x2ec94)?;
    // 0x153fc
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.check(0x15400)
}


/// Configure payload and buffer pointers for the remaining codec dispatch
/// variants, including conditional input record submission and alignment.
fn submit_codec_payload_variants(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x15400)?;
    // 0x15400
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x15404
    c.call(0x11e10)?;
    // 0x15408
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x1540c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x15410
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,8)?;
    // 0x15414
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x15418
    imm(c,C::Al,Op::Add,R::R2,R::R1,4)?;
    // 0x1541c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x15420
    imm(c,C::Al,Op::Mov,R::R2,R::R0,132)?;
    // 0x15424
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x15428
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,0)?)?;
    // 0x1542c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,4)?)?;
    // 0x15430
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // 0x15434
    imm(c,C::Al,Op::Add,R::new(5)?,R::new(3)?,248)?;
    // 0x15438
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // 0x1543c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,68)?)?;
    // 0x15440
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x15444
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x15448
    imm(c,C::Al,Op::Add,R::new(3)?,R::R1,12)?;
    // 0x1544c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    // 0x15450
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x15454
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x15458
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x1545c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x15460
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x15464
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x15468
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x1546c
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x15470
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x15474
    imm(c,C::Al,Op::Add,R::new(4)?,R::R0,608)?;
    // 0x15478
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x1547c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,68)?)?;
    // 0x15480
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x15484
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x15488
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x1548c
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x15490
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x15494
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x15498
    c.literal(R::R1,0x15adc)?;
    // 0x1549c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x154a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,68)?)?;
    // 0x154a4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x154a8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x154ac
    c.call(0x2ec94)?;
    // 0x154b0
    c.literal(R::R1,0x15ad8)?;
    // 0x154b4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x154b8
    c.call(0x2ec94)?;
    // 0x154bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x154c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x154c4
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,8)?;
    // 0x154c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x154cc
    imm(c,C::Al,Op::Add,R::R2,R::R1,4)?;
    // 0x154d0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x154d4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,116)?;
    // 0x154d8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x154dc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,0)?)?;
    // 0x154e0
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,4)?)?;
    // 0x154e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // 0x154e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-32)?)?;
    // 0x154ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,68)?)?;
    // 0x154f0
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x154f4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x154f8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    // 0x154fc
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,648)?;
    // 0x15500
    imm(c,C::Al,Op::Add,R::new(3)?,R::R1,12)?;
    // 0x15504
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x15508
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x1550c
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x15510
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,37888)?;
    // 0x15514
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x15518
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,12)?)?;
    // 0x1551c
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x15520
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x15524
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x15528
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x1552c
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x15530
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x15534
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x15538
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x1553c
    c.literal(R::R1,0x15adc)?;
    // 0x15540
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x15544
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x15548
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x1554c
    c.call(0x2ec94)?;
    // 0x15550
    c.literal(R::R1,0x15ad8)?;
    // 0x15554
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15558
    c.call(0x2ec94)?;
    // 0x1555c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,68)?)?;
    // 0x15560
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x15564
    c.branch(C::Ne, 0x155e4)?;
    // 0x15568
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x1556c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,520)?;
    // 0x15570
    imm(c,C::Al,Op::Mov,R::R2,R::R0,148)?;
    // 0x15574
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x15578
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x1557c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x15580
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x15584
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x15588
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,740)?)?;
    // 0x1558c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x15590
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x15594
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x15598
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x1559c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x155a0
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x155a4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x155a8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,12)?)?;
    // 0x155ac
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x155b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,760)?)?;
    // 0x155b4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x155b8
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x155bc
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x155c0
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x155c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x155c8
    c.literal(R::R1,0x15adc)?;
    // 0x155cc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x155d0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,760)?)?;
    // 0x155d4
    c.call(0x2ec94)?;
    // 0x155d8
    c.literal(R::R1,0x15ad8)?;
    // 0x155dc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x155e0
    c.call(0x2ec94)?;
    // 0x155e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,988)?)?;
    // 0x155e8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x155ec
    c.branch(C::Ne, 0x15808)?;
    // 0x155f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x155f4
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(6)?))?;
    // 0x155f8
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,520)?;
    // 0x155fc
    imm(c,C::Al,Op::Mov,R::R2,R::R0,68)?;
    // 0x15600
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?;
    // 0x15604
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x15608
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x1560c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x15610
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x15614
    c.literal(R::new(4)?,0x15adc)?;
    // 0x15618
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,992)?)?;
    // 0x1561c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x15620
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x15624
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x15628
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x1562c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x15630
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x15634
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x15638
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,12)?)?;
    // 0x1563c
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x15640
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,984)?)?;
    // 0x15644
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x15648
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x1564c
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x15650
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x15654
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x15658
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x1565c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x15660
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,984)?)?;
    // 0x15664
    c.call(0x2ec94)?;
    // 0x15668
    c.literal(R::new(7)?,0x15ad8)?;
    // 0x1566c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15670
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x15674
    c.call(0x2ec94)?;
    // 0x15678
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x1567c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,68)?;
    // 0x15680
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x15684
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x15688
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x1568c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x15690
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x15694
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,992)?)?;
    // 0x15698
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x1569c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,652)?;
    // 0x156a0
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x156a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x156a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x156ac
    imm(c,C::Al,Op::Add,R::R2,R::R0,16)?;
    // 0x156b0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x156b4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,648)?;
    // 0x156b8
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x156bc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x156c0
    imm(c,C::Al,Op::Add,R::R2,R::R1,16)?;
    // 0x156c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,984)?)?;
    // 0x156c8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x156cc
    imm(c,C::Al,Op::Add,R::R0,R::R0,255)?;
    // 0x156d0
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x156d4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x156d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x156dc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x156e0
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x156e4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,984)?)?;
    // 0x156e8
    c.call(0x2ec94)?;
    // 0x156ec
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x156f0
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x156f4
    c.call(0x2ec94)?;
    // 0x156f8
    c.branch(C::Al, 0x15808)?;
    // 0x156fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x15700
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(4)?))?;
    // 0x15704
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x15708
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
    // 0x1570c
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x15710
    c.call(0x118b8)?;
    // 0x15714
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x15718
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1571c
    c.call(0x120f4)?;
    // 0x15720
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,72)?)?;
    // 0x15724
    imm(c,C::Al,Op::Mov,R::R2,R::R0,732)?;
    // 0x15728
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(5)?))?;
    // 0x1572c
    imm(c,C::Al,Op::Orr,R::R2,R::R2,6144)?;
    // 0x15730
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x15734
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x15738
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x1573c
    c.branch(C::Eq, 0x15808)?;
    // 0x15740
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,144)?)?;
    // 0x15744
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,12)?)?;
    // 0x15748
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,524)?;
    // 0x1574c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,148)?;
    // 0x15750
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x15754
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,32)?)?;
    // 0x15758
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R1,Shift::Lsl,2)?)?;
    // 0x1575c
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,520)?;
    // 0x15760
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?;
    // 0x15764
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,2,false)?)?;
    // 0x15768
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,136)?)?;
    // 0x1576c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x15770
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x15774
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // 0x15778
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,12)?)?;
    // 0x1577c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x15780
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x15784
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,0)?)?;
    // 0x15788
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x1578c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x15790
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,4)?)?;
    // 0x15794
    imm(c,C::Al,Op::Mov,R::R1,R::R0,652)?;
    // 0x15798
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x1579c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x157a0
    imm(c,C::Al,Op::Add,R::R1,R::R0,16)?;
    // 0x157a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,8)?)?;
    // 0x157a8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x157ac
    imm(c,C::Al,Op::Mov,R::R1,R::R0,648)?;
    // 0x157b0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x157b4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x157b8
    imm(c,C::Al,Op::Add,R::R0,R::new(3)?,16)?;
    // 0x157bc
    c.literal(R::R1,0x15adc)?;
    // 0x157c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x157c4
    imm(c,C::Al,Op::Add,R::R0,R::R2,255)?;
    // 0x157c8
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x157cc
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x157d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,12)?)?;
    // 0x157d4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x157d8
    c.call(0x2ec94)?;
    // 0x157dc
    c.literal(R::R1,0x15ad8)?;
    // 0x157e0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x157e4
    c.call(0x2ec94)?;
    // 0x157e8
    c.branch(C::Al, 0x15808)?;
    // 0x157ec
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(4)?,R::LR,R::new(9)?,Shift::Lsr,R::R2,false)?)?;
    // 0x157f0
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(4)?,R::LR,R::R11,Shift::Ror,4,false)?)?;
    // 0x157f4
    c.emit(arm32::data_register(C::Eq,Op::And,R::R2,R::LR,R::new(8)?,Shift::Lsl,15,false)?)?;
    // 0x157f8
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x157fc
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    c.check(0x15800)
}


/// Finish session descriptor publication, including optional callback and
/// command-handoff branches. Values after the return are address constants,
/// not decoded instructions; a small branch-owned tail precedes this pool.
fn finalize_next_codec_session(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x15800)?;
    // 0x15800
    c.call(0xee38)?;
    // 0x15804
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(5)?))?;
    // 0x15808
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x1580c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,556)?;
    // 0x15810
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,68)?;
    // 0x15814
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,264)?;
    // 0x15818
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,84)?;
    // 0x1581c
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,4)?;
    // 0x15820
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x15824
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,262144)?;
    // 0x15828
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x1582c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x15830
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x15834
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,552)?;
    // 0x15838
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,37888)?;
    // 0x1583c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x15840
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,4)?)?;
    // 0x15844
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x15848
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,8)?)?;
    // 0x1584c
    imm(c,C::Al,Op::Add,R::new(6)?,R::new(7)?,4)?;
    // 0x15850
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,12)?)?;
    // 0x15854
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,12)?)?;
    // 0x15858
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,100)?;
    // 0x1585c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,16)?)?;
    // 0x15860
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,16)?)?;
    // 0x15864
    imm(c,C::Al,Op::Add,R::new(6)?,R::new(7)?,8)?;
    // 0x15868
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,20)?)?;
    // 0x1586c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,20)?)?;
    // 0x15870
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,116)?;
    // 0x15874
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,24)?)?;
    // 0x15878
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,24)?)?;
    // 0x1587c
    imm(c,C::Al,Op::Add,R::new(6)?,R::new(7)?,12)?;
    // 0x15880
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,28)?)?;
    // 0x15884
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,28)?)?;
    // 0x15888
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,132)?;
    // 0x1588c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,32)?)?;
    // 0x15890
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,32)?)?;
    // 0x15894
    imm(c,C::Al,Op::Add,R::new(6)?,R::new(7)?,16)?;
    // 0x15898
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,36)?)?;
    // 0x1589c
    imm(c,C::Al,Op::Add,R::new(7)?,R::new(7)?,20)?;
    // 0x158a0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,36)?)?;
    // 0x158a4
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,148)?;
    // 0x158a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,40)?)?;
    // 0x158ac
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,40)?)?;
    // 0x158b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,44)?)?;
    // 0x158b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,44)?)?;
    // 0x158b8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,48)?)?;
    // 0x158bc
    imm(c,C::Al,Op::Mov,R::R2,R::R0,19)?;
    // 0x158c0
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x158c4
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x158c8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,48)?)?;
    // 0x158cc
    imm(c,C::Al,Op::Add,R::R2,R::R1,56)?;
    // 0x158d0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x158d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,52)?)?;
    // 0x158d8
    imm(c,C::Al,Op::Add,R::R1,R::R0,56)?;
    // 0x158dc
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x158e0
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,4)?;
    // 0x158e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x158e8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,288)?;
    // 0x158ec
    imm(c,C::Al,Op::Orr,R::R1,R::R1,1048576)?;
    // 0x158f0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,52)?)?;
    // 0x158f4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,7744)?;
    // 0x158f8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,48)?;
    // 0x158fc
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(10)?,R::R0,Shift::Lsl,0)?)?;
    // 0x15900
    imm(c,C::Al,Op::Mov,R::R1,R::R0,8)?;
    // 0x15904
    c.literal(R::R0,0x15ae0)?;
    // 0x15908
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(10)?,R::R0,Shift::Lsl,0)?)?;
    // 0x1590c
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::new(5)?.bit() | R::new(10)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x15910
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x15914
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x15918
    imm(c,C::Eq,Op::Mov,R::R1,R::R0,32)?;
    // 0x1591c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x15920
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x15924
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
    // 0x15928
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x1592c
    c.call(0x2326c)?;
    // 0x15930
    c.literal(R::R0,0x15ae4)?;
    // 0x15934
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(10)?,R::R0,Shift::Lsl,0)?)?;
    // 0x15938
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,3)?;
    // 0x1593c
    c.branch(C::Ne, 0x15990)?;
    // 0x15940
    imm(c,C::Al,Op::Mov,R::R0,R::R0,5)?;
    // 0x15944
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x15948
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x1594c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,4)?;
    // 0x15950
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::new(10)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x15954
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x15958
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x1595c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
    // 0x15960
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x15964
    c.call(0x2326c)?;
    // 0x15968
    imm(c,C::Al,Op::Mov,R::R0,R::R0,6)?;
    // 0x1596c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x15970
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x15974
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,4)?;
    // 0x15978
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::new(10)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x1597c
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x15980
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x15984
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
    // 0x15988
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x1598c
    c.call(0x2326c)?;
    // 0x15990
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // 0x15994
    imm(c,C::Al,Op::Mov,R::R2,R::R0,7)?;
    // 0x15998
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,4)?;
    // 0x1599c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,28)?)?;
    // 0x159a0
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R2.bit() | R::new(10)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x159a4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x159a8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x159ac
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x159b0
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,12)?;
    // 0x159b4
    imm(c,C::Eq,Op::Mov,R::R1,R::R0,32)?;
    // 0x159b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x159bc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x159c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x159c4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x159c8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x159cc
    c.call(0x2326c)?;
    // 0x159d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x159d4
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x159d8
    c.branch(C::Ne, 0x159e8)?;
    // 0x159dc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,44)?)?;
    // 0x159e0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,1)?;
    // 0x159e4
    c.branch(C::Ne, 0x159f8)?;
    // 0x159e8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // 0x159ec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,4)?)?;
    // 0x159f0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x159f4
    c.branch(C::Eq, 0x15a70)?;
    // 0x159f8
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x159fc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x15a00
    imm(c,C::Al,Op::Mov,R::R1,R::R0,96)?;
    // 0x15a04
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R2.bit() | R::new(10)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x15a08
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x15a0c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x15a10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x15a14
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x15a18
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x15a1c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x15a20
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,2)?;
    // 0x15a24
    c.call(0x2326c)?;
    // 0x15a28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,28)?)?;
    // 0x15a2c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x15a30
    c.branch(C::Ne, 0x15a64)?;
    // 0x15a34
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x15a38
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x15a3c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,32)?;
    // 0x15a40
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,2)?;
    // 0x15a44
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R2.bit() | R::new(10)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x15a48
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x15a4c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x15a50
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x15a54
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x15a58
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x15a5c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x15a60
    c.call(0x2326c)?;
    // 0x15a64
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x15a68
    imm(c,C::Al,Op::Sub,R::SP,R::R11,28)?;
    // 0x15a6c
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    // 0x15a70
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,8)?)?;
    // 0x15a74
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x15a78
    c.branch(C::Ne, 0x159f8)?;
    // 0x15a7c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,12)?;
    // 0x15a80
    c.branch(C::Hi, 0x159f8)?;
    // 0x15a84
    imm(c,C::Al,Op::Mov,R::R1,R::R0,3)?;
    // 0x15a88
    imm(c,C::Al,Op::Mov,R::R0,R::R0,128)?;
    // 0x15a8c
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,0)?;
    // 0x15a90
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x15a94
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,2)?;
    // 0x15a98
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,1)?;
    // 0x15a9c
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R1.bit() | R::new(10)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x15aa0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x15aa4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x15aa8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x15aac
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // 0x15ab0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x15ab4
    c.call(0x2326c)?;
    // 0x15ab8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,4)?;
    // 0x15abc
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::new(6)?.bit() | R::new(10)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x15ac0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x15ac4
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // 0x15ac8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x15acc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // 0x15ad0
    c.branch(C::Al, 0x15a14)?;
    c.check(0x15ad4)?;
    const CODEC_DIAGNOSTICS:[u32;5]=[
        0x000e_3d16, 0x000e_2788, 0x000e_3dfc,
        0xffff_e1f4, 0xffff_e1d0,
    ];
    for addr in CODEC_DIAGNOSTICS {c.emit(addr)?;}
    c.check(0x15ae8)
}


/// Compact an unsigned descriptor length to its highest occupied word,
/// then write codec metadata/flags through two conditional buffer stores.
fn write_stream_small_helpers(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x15ae8)?;
    // 0x15ae8
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x15aec
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsr, 16)?)?;
    // 0x15af0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsr,16,false)?)?;
    // 0x15af4
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,1)?;
    // 0x15af8
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::R1, Shift::Lsl, 4)?)?;
    // 0x15afc
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x15b00
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x15b04
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,255)?;
    // 0x15b08
    imm(c,C::Hi,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x15b0c
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 3)?)?;
    // 0x15b10
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x15b14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,4,false)?)?;
    // 0x15b18
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x15b1c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,15)?;
    // 0x15b20
    imm(c,C::Hi,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x15b24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x15b28
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 2)?)?;
    // 0x15b2c
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x15b30
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,3)?;
    // 0x15b34
    imm(c,C::Hi,Op::Mov,R::R2,R::R0,1)?;
    // 0x15b38
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,1,false)?)?;
    // 0x15b3c
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 1)?)?;
    // 0x15b40
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::R2,false)?)?;
    // 0x15b44
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // 0x15b48
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x15b4c
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x15b50
    imm(c,C::Al,Op::Add,R::R11,R::SP,8)?;
    // 0x15b54
    imm(c,C::Al,Op::Mov,R::R0,R::R0,552)?;
    // 0x15b58
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x15b5c
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,152)?;
    // 0x15b60
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2648)?)?;
    // 0x15b64
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x15b68
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,35840)?;
    // 0x15b6c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x15b70
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,960)?)?;
    // 0x15b74
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R2,956)?)?;
    // 0x15b78
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R2,964)?)?;
    // 0x15b7c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x15b80
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,556)?;
    // 0x15b84
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,7168)?;
    // 0x15b88
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x15b8c
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 8)?)?;
    // 0x15b90
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::R12,Shift::Lsl,16,false)?)?;
    // 0x15b94
    reg(c,C::Al,Op::Add,R::new(3)?,R::new(3)?,R::LR)?;
    // 0x15b98
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,0)?)?;
    // 0x15b9c
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,8)?;
    // 0x15ba0
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R2,956)?)?;
    // 0x15ba4
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R2,960)?)?;
    // 0x15ba8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,964)?)?;
    // 0x15bac
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x15bb0
    imm(c,C::Al,Op::Mov,R::R1,R::R0,28)?;
    // 0x15bb4
    imm(c,C::Al,Op::Orr,R::R1,R::R1,35840)?;
    // 0x15bb8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x15bbc
    reg(c,C::Al,Op::Add,R::R0,R::LR,R::R12)?;
    // 0x15bc0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // 0x15bc4
    c.call(0x15ae8)?;
    // 0x15bc8
    imm(c,C::Al,Op::Add,R::R1,R::new(5)?,8)?;
    // 0x15bcc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x15bd0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,4)?)?;
    // 0x15bd4
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    // 0x15bd8
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x15bdc
    imm(c,C::Al,Op::Add,R::R11,R::SP,16)?;
    // 0x15be0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x15be4
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,1)?;
    // 0x15be8
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x15bec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // 0x15bf0
    c.literal(R::R0,0x15df0)?;
    // 0x15bf4
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,1280)?;
    // 0x15bf8
    reg(c,C::Al,Op::Add,R::R2,R::R1,R::R0)?;
    // 0x15bfc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,255)?;
    // 0x15c00
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1792)?;
    // 0x15c04
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,11)?;
    // 0x15c08
    reg(c,C::Al,Op::And,R::R0,R::R2,R::R0)?;
    // 0x15c0c
    imm(c,C::Cc,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x15c10
    imm(c,C::Al,Op::Add,R::R2,R::R2,2)?;
    // 0x15c14
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::And,R::R12,R::new(3)?,R::new(6)?,Shift::Lsr,R::R0,false)?)?;
    // 0x15c18
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,12)?;
    // 0x15c1c
    c.branch(C::Hi, 0x15d14)?;
    // 0x15c20
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,255)?;
    // 0x15c24
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,1)?;
    // 0x15c28
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,7936)?;
    // 0x15c2c
    reg(c,C::Al,Op::And,R::R2,R::R2,R::new(3)?)?;
    // 0x15c30
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,5)?;
    // 0x15c34
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?;
    // 0x15c38
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::And,R::R2,R::new(6)?,R::new(3)?,Shift::Lsr,R::R2,false)?)?;
    // 0x15c3c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x15c40
    c.branch(C::Eq, 0x15d14)?;
    // 0x15c44
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x15c48
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x15c4c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2648)?)?;
    // 0x15c50
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1516)?)?;
    // 0x15c54
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(4)?,1488)?)?;
    // 0x15c58
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,9)?;
    // 0x15c5c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,852)?)?;
    // 0x15c60
    imm(c,C::Eq,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x15c64
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1)?;
    // 0x15c68
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1512)?)?;
    // 0x15c6c
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(3)?, Shift::Lsl, 17)?)?;
    // 0x15c70
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1520)?)?;
    // 0x15c74
    imm(c,C::Eq,Op::Mov,R::new(5)?,R::R0,131072)?;
    // 0x15c78
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 8)?)?;
    // 0x15c7c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,4,false)?)?;
    // 0x15c80
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,848)?)?;
    // 0x15c84
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,860)?)?;
    // 0x15c88
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x15c8c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::LR,Shift::Lsl,1,false)?)?;
    // 0x15c90
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x15c94
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x15c98
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,5,false)?)?;
    // 0x15c9c
    reg(c,C::Al,Op::Orr,R::R2,R::R0,R::new(5)?)?;
    // 0x15ca0
    imm(c,C::Al,Op::Orr,R::R0,R::R1,2)?;
    // 0x15ca4
    c.literal(R::R1,0x15df4)?;
    // 0x15ca8
    reg(c,C::Al,Op::Cmp,R::R0,R::R0,R::R1)?;
    // 0x15cac
    c.branch(C::Ne, 0x15cd4)?;
    // 0x15cb0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,36)?;
    // 0x15cb4
    imm(c,C::Al,Op::Cmp,R::R0,R::new(6)?,1)?;
    // 0x15cb8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1508)?)?;
    // 0x15cbc
    c.branch(C::Ne, 0x15cd4)?;
    // 0x15cc0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,13)?;
    // 0x15cc4
    imm(c,C::Al,Op::Cmp,R::R0,R::LR,1)?;
    // 0x15cc8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1508)?)?;
    // 0x15ccc
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,22)?;
    // 0x15cd0
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,1508)?)?;
    // 0x15cd4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,552)?;
    // 0x15cd8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x15cdc
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x15ce0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x15ce4
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,4)?;
    // 0x15ce8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x15cec
    imm(c,C::Al,Op::Mov,R::R1,R::R0,52)?;
    // 0x15cf0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,35840)?;
    // 0x15cf4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x15cf8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,556)?;
    // 0x15cfc
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x15d00
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x15d04
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x15d08
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,4)?;
    // 0x15d0c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x15d10
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x15d14
    imm(c,C::Al,Op::Cmp,R::R0,R::R12,1)?;
    // 0x15d18
    c.emit(arm32::transfer_multiple(C::Ne, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    // 0x15d1c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1496)?)?;
    // 0x15d20
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1504)?)?;
    // 0x15d24
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1500)?)?;
    // 0x15d28
    imm(c,C::Al,Op::Mvn,R::new(7)?,R::R0,31)?;
    // 0x15d2c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1492)?)?;
    // 0x15d30
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,5,false)?)?;
    // 0x15d34
    imm(c,C::Al,Op::Sub,R::new(7)?,R::new(3)?,1)?;
    // 0x15d38
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 16)?)?;
    // 0x15d3c
    imm(c,C::Al,Op::Sub,R::R2,R::R2,1)?;
    // 0x15d40
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,14)?;
    // 0x15d44
    imm(c,C::Hi,Op::Mov,R::new(3)?,R::R0,262144)?;
    // 0x15d48
    reg(c,C::Al,Op::Orr,R::R2,R::new(3)?,R::R2)?;
    // 0x15d4c
    reg(c,C::Al,Op::Orr,R::R1,R::R2,R::R1)?;
    // 0x15d50
    c.literal(R::R2,0x15df8)?;
    // 0x15d54
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,10,false)?)?;
    // 0x15d58
    reg(c,C::Al,Op::Orr,R::R0,R::R1,R::R0)?;
    // 0x15d5c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1524)?)?;
    // 0x15d60
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R0,R::R1,Shift::Lsl,22,false)?)?;
    // 0x15d64
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1508)?)?;
    // 0x15d68
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,13)?;
    // 0x15d6c
    c.branch(C::Eq, 0x15da0)?;
    // 0x15d70
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,22)?;
    // 0x15d74
    c.branch(C::Eq, 0x15da8)?;
    // 0x15d78
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,0)?;
    // 0x15d7c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,36)?;
    // 0x15d80
    c.branch(C::Eq, 0x15dac)?;
    // 0x15d84
    c.literal(R::R2,0x15dfc)?;
    // 0x15d88
    imm(c,C::Al,Op::Mov,R::R1,R::R0,95)?;
    // 0x15d8c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x15d90
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,0)?;
    // 0x15d94
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3328)?;
    // 0x15d98
    c.call(0x17b0)?;
    // 0x15d9c
    c.branch(C::Al, 0x15dac)?;
    // 0x15da0
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,1048576)?;
    // 0x15da4
    c.branch(C::Al, 0x15dac)?;
    // 0x15da8
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,2097152)?;
    // 0x15dac
    imm(c,C::Al,Op::Mov,R::R0,R::R0,552)?;
    // 0x15db0
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x15db4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x15db8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x15dbc
    imm(c,C::Al,Op::Add,R::R2,R::R0,4)?;
    // 0x15dc0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x15dc4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,36)?;
    // 0x15dc8
    imm(c,C::Al,Op::Orr,R::R1,R::R1,35840)?;
    // 0x15dcc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x15dd0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,556)?;
    // 0x15dd4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x15dd8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x15ddc
    imm(c,C::Al,Op::Add,R::R1,R::R0,4)?;
    // 0x15de0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x15de4
    reg(c,C::Al,Op::Orr,R::R1,R::new(5)?,R::new(6)?)?;
    // 0x15de8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x15dec
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    c.check(0x15df0)?;
    const CODEC_POINTER_AND_OFFSETS:[u32;4]=[
        0xfbfe_eff8,0x0401_1012,0xffff_fc00,0x000e_3d16,
    ];
    for v in CODEC_POINTER_AND_OFFSETS {c.emit(v)?;}
    c.check(0x15e00)?;
    // 0x15e00
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x15e04
    imm(c,C::Al,Op::Add,R::R11,R::SP,24)?;
    // 0x15e08
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,8)?)?;
    // 0x15e0c
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x15e10
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(3)?))?;
    // 0x15e14
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R2))?;
    // 0x15e18
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R1))?;
    // 0x15e1c
    c.literal(R::R2,0x15ea4)?;
    // 0x15e20
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x15e24
    imm(c,C::Al,Op::Cmp,R::R0,R::new(8)?,0)?;
    // 0x15e28
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x15e2c
    imm(c,C::Ne,Op::Mov,R::R0,R::R0,1)?;
    // 0x15e30
    imm(c,C::Al,Op::Cmp,R::R0,R::new(6)?,0)?;
    // 0x15e34
    imm(c,C::Ne,Op::Mov,R::R1,R::R0,1)?;
    // 0x15e38
    reg(c,C::Al,Op::And,R::R0,R::R1,R::R0)?;
    // 0x15e3c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,105)?;
    // 0x15e40
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3328)?;
    // 0x15e44
    c.call(0x17b0)?;
    // 0x15e48
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R0,R::new(6)?,R::new(6)?,Shift::Lsl,3,false)?)?;
    // 0x15e4c
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R2,R::new(8)?,R::new(8)?,Shift::Lsl,3,false)?)?;
    // 0x15e50
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(6)?, Shift::Lsl, 2)?)?;
    // 0x15e54
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x15e58
    reg(c,C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?)?;
    // 0x15e5c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x15e60
    imm(c,C::Cc,Op::Mov,R::R0,R::R0,1)?;
    // 0x15e64
    reg(c,C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?)?;
    // 0x15e68
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x15e6c
    imm(c,C::Cc,Op::Mov,R::R2,R::R0,1)?;
    // 0x15e70
    reg(c,C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?)?;
    // 0x15e74
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x15e78
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x15e7c
    imm(c,C::Cc,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x15e80
    reg(c,C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?)?;
    // 0x15e84
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x15e88
    imm(c,C::Cc,Op::Mov,R::new(7)?,R::R0,1)?;
    // 0x15e8c
    c.emit(arm32::transfer_multiple(C::Al, false, R::new(4)?, R::new(3)?.bit() | R::new(7)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x15e90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x15e94
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,12)?)?;
    // 0x15e98
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,16)?)?;
    // 0x15e9c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,20)?)?;
    // 0x15ea0
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    c.check(0x15ea4)?;
    c.emit(0x000e_3d16)?; // Signed diagnostic pointer data.
    c.check(0x15ea8)
}

/// Compute valid codec input-plane widths and heights, then dispatch format
/// cases through a bounded ten-entry branch table and pack descriptor state.
fn update_stream_mode_metadata(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x15ea8)?;
    // 0x15ea8
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x15eac
    imm(c,C::Al,Op::Add,R::R11,R::SP,24)?;
    // 0x15eb0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,8)?)?;
    // 0x15eb4
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,16)?)?;
    // 0x15eb8
    c.literal(R::new(3)?,0x163cc)?;
    // 0x15ebc
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,0)?;
    // 0x15ec0
    reg(c,C::Al,Op::Add,R::R2,R::R2,R::new(3)?)?;
    // 0x15ec4
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,255)?;
    // 0x15ec8
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,7936)?;
    // 0x15ecc
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,13)?;
    // 0x15ed0
    reg(c,C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?)?;
    // 0x15ed4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x15ed8
    imm(c,C::Cc,Op::Mov,R::new(6)?,R::R0,1)?;
    // 0x15edc
    imm(c,C::Al,Op::Cmp,R::R0,R::new(4)?,6)?;
    // 0x15ee0
    imm(c,C::Al,Op::Orr,R::R2,R::R2,5120)?;
    // 0x15ee4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x15ee8
    imm(c,C::Al,Op::Sub,R::new(3)?,R::new(4)?,2)?;
    // 0x15eec
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,48)?)?;
    // 0x15ef0
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 0x15ef4
    c.emit(arm32::mov_register(C::Hi, R::R2, R::new(3)?))?;
    // 0x15ef8
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,9)?;
    // 0x15efc
    c.branch(C::Hi, 0x15ff8)?;
    // 0x15f00
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 2)?)?;
    // 0x15f04
    imm(c,C::Al,Op::Add,R::new(6)?,R::PC,0)?;
    // 0x15f08
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::new(3)?,R::new(6)?,Shift::Lsl,0)?)?;
    c.check(0x15f0c)?;
    const FORMAT_PLANES:[u32;10]=[
        0x15f34,0x15ff8,0x15ff8,0x15ff8,0x15ff8,
        0x15f74,0x15ff8,0x15ff8,0x15fa8,0x15fbc,
    ];
    for target in FORMAT_PLANES {c.emit(target)?;}
    c.check(0x15f34)?;
    // 0x15f34
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,728)?)?;
    // 0x15f38
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,1)?;
    // 0x15f3c
    c.branch(C::Eq, 0x15fec)?;
    // 0x15f40
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,12)?)?;
    // 0x15f44
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,496)?;
    // 0x15f48
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?;
    // 0x15f4c
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x15f50
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x15f54
    imm(c,C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?;
    // 0x15f58
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,1)?;
    // 0x15f5c
    c.branch(C::Hi, 0x16234)?;
    // 0x15f60
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,44)?)?;
    // 0x15f64
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R2))?;
    // 0x15f68
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,1)?;
    // 0x15f6c
    imm(c,C::Eq,Op::Mov,R::new(4)?,R::R0,0)?;
    // 0x15f70
    c.branch(C::Al, 0x16000)?;
    // 0x15f74
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,44)?)?;
    // 0x15f78
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x15f7c
    c.branch(C::Eq, 0x15ff4)?;
    // 0x15f80
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,12)?)?;
    // 0x15f84
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,236)?;
    // 0x15f88
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,0)?;
    // 0x15f8c
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?;
    // 0x15f90
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x15f94
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x15f98
    imm(c,C::Al,Op::Sub,R::R2,R::R2,1)?;
    // 0x15f9c
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x15fa0
    imm(c,C::Cs,Op::Mov,R::new(4)?,R::R0,7)?;
    // 0x15fa4
    c.branch(C::Al, 0x16000)?;
    // 0x15fa8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,68)?)?;
    // 0x15fac
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R2))?;
    // 0x15fb0
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,0)?;
    // 0x15fb4
    imm(c,C::Ne,Op::Mov,R::new(4)?,R::R0,6)?;
    // 0x15fb8
    c.branch(C::Al, 0x16000)?;
    // 0x15fbc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,12)?)?;
    // 0x15fc0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,72)?)?;
    // 0x15fc4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x15fc8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x15fcc
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,732)?;
    // 0x15fd0
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?;
    // 0x15fd4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x15fd8
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x15fdc
    c.branch(C::Eq, 0x15fec)?;
    // 0x15fe0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,76)?)?;
    // 0x15fe4
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x15fe8
    c.branch(C::Eq, 0x16284)?;
    // 0x15fec
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,0)?;
    // 0x15ff0
    c.branch(C::Al, 0x16000)?;
    // 0x15ff4
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,7)?;
    // 0x15ff8
    imm(c,C::Al,Op::Cmp,R::R0,R::new(4)?,6)?;
    // 0x15ffc
    imm(c,C::Hi,Op::Sub,R::new(4)?,R::new(4)?,2)?;
    // 0x16000
    imm(c,C::Al,Op::Mov,R::R2,R::R0,552)?;
    // 0x16004
    c.emit(arm32::mov_register(C::Al, R::LR, R::R1))?;
    // 0x16008
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,108)?;
    // 0x1600c
    c.emit(arm32::mov_register(C::Al, R::R12, R::R1))?;
    // 0x16010
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,255)?;
    // 0x16014
    imm(c,C::Al,Op::Orr,R::R2,R::R2,7168)?;
    // 0x16018
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,32768)?;
    // 0x1601c
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,65280)?;
    // 0x16020
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::LR,R::R2,Shift::Lsl,0)?)?;
    // 0x16024
    imm(c,C::Al,Op::Mov,R::R2,R::R0,556)?;
    // 0x16028
    imm(c,C::Al,Op::Orr,R::R2,R::R2,7168)?;
    // 0x1602c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // 0x16030
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(8)?,R::R12,R::R2,Shift::Lsl,0)?)?;
    // 0x16034
    imm(c,C::Al,Op::Add,R::R2,R::new(3)?,12)?;
    // 0x16038
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,0)?)?;
    // 0x1603c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,4)?)?;
    // 0x16040
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,0)?;
    // 0x16044
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,4)?)?;
    // 0x16048
    c.literal(R::new(5)?,0x163d0)?;
    // 0x1604c
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R12,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x16050
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,816)?)?;
    // 0x16054
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,808)?)?;
    // 0x16058
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x1605c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,3088)?;
    // 0x16060
    imm(c,C::Al,Op::Orr,R::R2,R::R2,32768)?;
    // 0x16064
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,8)?)?;
    // 0x16068
    imm(c,C::Al,Op::Add,R::R2,R::new(8)?,12)?;
    // 0x1606c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,0)?)?;
    // 0x16070
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(4)?,R::new(7)?,Shift::Lsl,9,false)?)?;
    // 0x16074
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(6)?,Shift::Lsl,5,false)?)?;
    // 0x16078
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,8)?)?;
    // 0x1607c
    c.literal(R::R2,0x163d4)?;
    // 0x16080
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::R12,R::R2,Shift::Lsl,0)?)?;
    // 0x16084
    c.literal(R::R2,0x163d8)?;
    // 0x16088
    reg(c,C::Al,Op::Add,R::new(6)?,R::new(4)?,R::R2)?;
    // 0x1608c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(6)?,12)?;
    // 0x16090
    c.branch(C::Hi, 0x16160)?;
    // 0x16094
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,35)?;
    // 0x16098
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x1609c
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?;
    // 0x160a0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(7)?,R::R2,Shift::Lsl,R::new(6)?,false)?)?;
    // 0x160a4
    c.branch(C::Eq, 0x16160)?;
    // 0x160a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,88)?)?;
    // 0x160ac
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,92)?)?;
    // 0x160b0
    imm(c,C::Al,Op::Add,R::new(4)?,R::new(3)?,16)?;
    // 0x160b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x160b8
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,168)?;
    // 0x160bc
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,35840)?;
    // 0x160c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(3)?,12)?)?;
    // 0x160c4
    imm(c,C::Al,Op::Add,R::new(4)?,R::new(8)?,16)?;
    // 0x160c8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,16,false)?)?;
    // 0x160cc
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,0)?)?;
    // 0x160d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,12)?)?;
    // 0x160d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,8)?)?;
    // 0x160d8
    c.literal(R::R2,0x163dc)?;
    // 0x160dc
    reg(c,C::Al,Op::Cmp,R::R0,R::R0,R::R2)?;
    // 0x160e0
    c.branch(C::Eq, 0x16120)?;
    // 0x160e4
    c.literal(R::R2,0x163e0)?;
    // 0x160e8
    reg(c,C::Al,Op::Cmp,R::R0,R::R0,R::R2)?;
    // 0x160ec
    c.branch(C::Ne, 0x1612c)?;
    // 0x160f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,124)?)?;
    // 0x160f4
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x160f8
    c.branch(C::Eq, 0x1612c)?;
    // 0x160fc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,820)?)?;
    // 0x16100
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,368)?)?;
    // 0x16104
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,488)?)?;
    // 0x16108
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,1)?;
    // 0x1610c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::R2,Shift::Lsr,3,false)?)?;
    // 0x16110
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsr,R::R0,false)?)?;
    // 0x16114
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::R2,Shift::Lsr,2,false)?)?;
    // 0x16118
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(4)?,R::R0,R::new(5)?,Shift::Lsr,R::R1,false)?)?;
    // 0x1611c
    c.branch(C::Al, 0x16134)?;
    // 0x16120
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,820)?)?;
    // 0x16124
    imm(c,C::Al,Op::And,R::R0,R::R0,3)?;
    // 0x16128
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x1612c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,368)?)?;
    // 0x16130
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,488)?)?;
    // 0x16134
    imm(c,C::Al,Op::Add,R::R1,R::new(3)?,20)?;
    // 0x16138
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x1613c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x16140
    imm(c,C::Al,Op::Mov,R::R1,R::R0,172)?;
    // 0x16144
    imm(c,C::Al,Op::Orr,R::R1,R::R1,35840)?;
    // 0x16148
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,16)?)?;
    // 0x1614c
    imm(c,C::Al,Op::Add,R::R1,R::new(8)?,20)?;
    // 0x16150
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x16154
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,16)?)?;
    // 0x16158
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x1615c
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    // 0x16160
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,48)?)?;
    // 0x16164
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1)?;
    // 0x16168
    c.branch(C::Ne, 0x161d4)?;
    // 0x1616c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,12)?)?;
    // 0x16170
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,68)?)?;
    // 0x16174
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,168)?;
    // 0x16178
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,35840)?;
    // 0x1617c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x16180
    imm(c,C::Al,Op::Add,R::R0,R::R0,612)?;
    // 0x16184
    imm(c,C::Al,Op::Add,R::R0,R::R0,4096)?;
    // 0x16188
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::new(4)?,Shift::Lsl,2,false)?)?;
    // 0x1618c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R2,304)?)?;
    // 0x16190
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,344)?)?;
    // 0x16194
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,12)?)?;
    // 0x16198
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(4)?,R::R2,Shift::Lsl,16,false)?)?;
    // 0x1619c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,12)?)?;
    // 0x161a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,68)?)?;
    // 0x161a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x161a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,64)?)?;
    // 0x161ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,104)?)?;
    // 0x161b0
    imm(c,C::Al,Op::Add,R::R2,R::new(3)?,20)?;
    // 0x161b4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x161b8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x161bc
    imm(c,C::Al,Op::Mov,R::R2,R::R0,172)?;
    // 0x161c0
    imm(c,C::Al,Op::Orr,R::R2,R::R2,35840)?;
    // 0x161c4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,16)?)?;
    // 0x161c8
    imm(c,C::Al,Op::Add,R::R2,R::new(8)?,20)?;
    // 0x161cc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,0)?)?;
    // 0x161d0
    c.branch(C::Al, 0x16154)?;
    // 0x161d4
    c.literal(R::R2,0x163e4)?;
    // 0x161d8
    reg(c,C::Al,Op::Cmp,R::R0,R::new(4)?,R::R2)?;
    // 0x161dc
    c.branch(C::Ne, 0x1623c)?;
    // 0x161e0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,12)?)?;
    // 0x161e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,72)?)?;
    // 0x161e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,76)?)?;
    // 0x161ec
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x161f0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x161f4
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,732)?;
    // 0x161f8
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,6144)?;
    // 0x161fc
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x16200
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x16204
    c.branch(C::Eq, 0x1628c)?;
    // 0x16208
    imm(c,C::Al,Op::Cmp,R::R0,R::new(4)?,0)?;
    // 0x1620c
    c.branch(C::Ne, 0x1628c)?;
    // 0x16210
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,68)?)?;
    // 0x16214
    imm(c,C::Al,Op::Mov,R::R1,R::R0,312)?;
    // 0x16218
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,1456)?;
    // 0x1621c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,5120)?;
    // 0x16220
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?;
    // 0x16224
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x16228
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x1622c
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x16230
    c.branch(C::Al, 0x162c0)?;
    // 0x16234
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R2))?;
    // 0x16238
    c.branch(C::Al, 0x16000)?;
    // 0x1623c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,16)?)?;
    // 0x16240
    imm(c,C::Al,Op::Cmp,R::R0,R::new(4)?,7)?;
    // 0x16244
    c.branch(C::Eq, 0x16310)?;
    // 0x16248
    imm(c,C::Al,Op::Cmp,R::R0,R::new(4)?,2)?;
    // 0x1624c
    c.branch(C::Ne, 0x16158)?;
    // 0x16250
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,12)?)?;
    // 0x16254
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x16258
    imm(c,C::Al,Op::Mov,R::R2,R::R0,496)?;
    // 0x1625c
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x16260
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x16264
    imm(c,C::Al,Op::Sub,R::R2,R::R2,1)?;
    // 0x16268
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1)?;
    // 0x1626c
    c.branch(C::Hi, 0x16360)?;
    // 0x16270
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,44)?)?;
    // 0x16274
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,1)?;
    // 0x16278
    c.branch(C::Ne, 0x16360)?;
    // 0x1627c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,532)?;
    // 0x16280
    c.branch(C::Al, 0x16334)?;
    // 0x16284
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,4)?;
    // 0x16288
    c.branch(C::Al, 0x16000)?;
    // 0x1628c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,68)?)?;
    // 0x16290
    imm(c,C::Al,Op::Mov,R::R1,R::R0,312)?;
    // 0x16294
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,1456)?;
    // 0x16298
    imm(c,C::Al,Op::Cmp,R::R0,R::new(4)?,0)?;
    // 0x1629c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x162a0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,5120)?;
    // 0x162a4
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?;
    // 0x162a8
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,1)?;
    // 0x162ac
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R0,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x162b0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x162b4
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::new(4)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x162b8
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::R2,false)?)?;
    // 0x162bc
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(4)?,R::R0,R::new(4)?,Shift::Lsr,R::R2,false)?)?;
    // 0x162c0
    imm(c,C::Al,Op::Mov,R::R2,R::R0,8)?;
    // 0x162c4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x162c8
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,6272)?;
    // 0x162cc
    imm(c,C::Al,Op::Orr,R::R2,R::R2,6144)?;
    // 0x162d0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x162d4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x162d8
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,168)?;
    // 0x162dc
    imm(c,C::Al,Op::Orr,R::new(5)?,R::new(5)?,35840)?;
    // 0x162e0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,12)?)?;
    // 0x162e4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,16,false)?)?;
    // 0x162e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,12)?)?;
    // 0x162ec
    imm(c,C::Al,Op::Add,R::R0,R::new(3)?,20)?;
    // 0x162f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x162f4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,172)?;
    // 0x162f8
    imm(c,C::Al,Op::Orr,R::R0,R::R0,35840)?;
    // 0x162fc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,16)?)?;
    // 0x16300
    imm(c,C::Al,Op::Add,R::R0,R::new(8)?,20)?;
    // 0x16304
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x16308
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x1630c
    c.branch(C::Al, 0x16154)?;
    // 0x16310
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,12)?)?;
    // 0x16314
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x16318
    imm(c,C::Al,Op::Mov,R::R1,R::R0,236)?;
    // 0x1631c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x16320
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x16324
    imm(c,C::Al,Op::Sub,R::R1,R::R1,1)?;
    // 0x16328
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,1)?;
    // 0x1632c
    c.branch(C::Hi, 0x16158)?;
    // 0x16330
    imm(c,C::Al,Op::Mov,R::R1,R::R0,272)?;
    // 0x16334
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x16338
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,168)?;
    // 0x1633c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x16340
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,35840)?;
    // 0x16344
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // 0x16348
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(3)?,12)?)?;
    // 0x1634c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x16350
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,12)?)?;
    // 0x16354
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x16358
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x1635c
    c.branch(C::Al, 0x161b0)?;
    // 0x16360
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,68)?)?;
    // 0x16364
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,1)?;
    // 0x16368
    c.branch(C::Ne, 0x16378)?;
    // 0x1636c
    imm(c,C::Al,Op::Add,R::R0,R::R1,92)?;
    // 0x16370
    imm(c,C::Al,Op::Add,R::new(4)?,R::R1,88)?;
    // 0x16374
    c.branch(C::Al, 0x1638c)?;
    // 0x16378
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x1637c
    imm(c,C::Al,Op::Add,R::R0,R::R2,692)?;
    // 0x16380
    imm(c,C::Al,Op::Add,R::R2,R::R2,680)?;
    // 0x16384
    imm(c,C::Al,Op::Add,R::R0,R::R0,1024)?;
    // 0x16388
    imm(c,C::Al,Op::Add,R::new(4)?,R::R2,1024)?;
    // 0x1638c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    // 0x16390
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x16394
    imm(c,C::Al,Op::Add,R::new(4)?,R::new(3)?,16)?;
    // 0x16398
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x1639c
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,168)?;
    // 0x163a0
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,35840)?;
    // 0x163a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(3)?,12)?)?;
    // 0x163a8
    imm(c,C::Al,Op::Add,R::new(4)?,R::new(8)?,16)?;
    // 0x163ac
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,16,false)?)?;
    // 0x163b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,0)?)?;
    // 0x163b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,12)?)?;
    // 0x163b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,68)?)?;
    // 0x163bc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x163c0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1708)?)?;
    // 0x163c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1720)?)?;
    // 0x163c8
    c.branch(C::Al, 0x161b0)?;
    c.check(0x163cc)?;
    const DEVICE_REFERENCES:[u32;7]=[
        0xfbfe_effa,0xffff_ec2c,0xffff_e1dc,0xfbfe_effd,
        0x0401_1004,0x0401_1003,0x0401_1016,
    ];
    for value in DEVICE_REFERENCES {c.emit(value)?;}
    c.check(0x163e8)
}


/// Populate codec input/output hardware command metadata for one session.
/// Select among four format layouts; both switch tables and six diagnostic
/// values are distinct runtime data, not A32 instruction encodings.
fn configure_controller_channels(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x163e8)?;
    // 0x163e8
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x163ec
    imm(c,C::Al,Op::Add,R::R11,R::SP,28)?;
    // 0x163f0
    imm(c,C::Al,Op::Sub,R::SP,R::SP,20)?;
    // 0x163f4
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x163f8
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::R0))?;
    // 0x163fc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,684)?;
    // 0x16400
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x16404
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(4)?))?;
    // 0x16408
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // 0x1640c
    c.literal(R::R1,0x16974)?;
    // 0x16410
    reg(c,C::Al,Op::Add,R::new(6)?,R::new(7)?,R::R1)?;
    // 0x16414
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x16418
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(6)?))?;
    // 0x1641c
    c.branch(C::Ne, 0x16454)?;
    // 0x16420
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x16424
    imm(c,C::Al,Op::Mov,R::R1,R::R0,496)?;
    // 0x16428
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x1642c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x16430
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x16434
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x16438
    imm(c,C::Al,Op::Sub,R::R0,R::R0,1)?;
    // 0x1643c
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x16440
    imm(c,C::Cc,Op::Mov,R::R1,R::R0,1)?;
    // 0x16444
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // 0x16448
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,3,false)?)?;
    // 0x1644c
    imm(c,C::Al,Op::Add,R::R0,R::R0,148)?;
    // 0x16450
    imm(c,C::Al,Op::Add,R::new(10)?,R::R0,3072)?;
    // 0x16454
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x16458
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,4)?)?;
    // 0x1645c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x16460
    imm(c,C::Al,Op::Mov,R::R12,R::R0,0)?;
    // 0x16464
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,0)?;
    // 0x16468
    imm(c,C::Ne,Op::Cmp,R::R0,R::R0,7)?;
    // 0x1646c
    c.branch(C::Eq, 0x164e4)?;
    // 0x16470
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x16474
    c.branch(C::Ne, 0x16510)?;
    // 0x16478
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // 0x1647c
    imm(c,C::Al,Op::Sub,R::R0,R::R0,4)?;
    // 0x16480
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,3)?;
    // 0x16484
    c.branch(C::Hi, 0x16538)?;
    // 0x16488
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x1648c
    imm(c,C::Al,Op::Add,R::R1,R::PC,0)?;
    // 0x16490
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x16494
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(6)?,R::R1,R::new(4)?,Shift::Lsr,9,false)?)?;
    // 0x16498
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(6)?,R::R1,R::R0,Shift::Lsl,11,false)?)?;
    // 0x1649c
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(6)?,R::R1,R::new(4)?,Shift::Asr,11,false)?)?;
    // 0x164a0
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(6)?,R::R1,R::new(8)?,Shift::Lsl,R::new(6)?,false)?)?;
    // 0x164a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2648)?)?;
    // 0x164a8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,60)?)?;
    // 0x164ac
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,0)?;
    // 0x164b0
    imm(c,C::Ne,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x164b4
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,2452)?)?;
    // 0x164b8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x164bc
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R12))?;
    // 0x164c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x164c4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,348)?;
    // 0x164c8
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(4)?,1744)?)?;
    // 0x164cc
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x164d0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x164d4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x164d8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x164dc
    imm(c,C::Al,Op::Mov,R::R1,R::R0,336)?;
    // 0x164e0
    c.branch(C::Al, 0x16604)?;
    // 0x164e4
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,12)?)?;
    // 0x164e8
    imm(c,C::Al,Op::Mov,R::LR,R::R0,0)?;
    // 0x164ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,88)?)?;
    // 0x164f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x164f4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // 0x164f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x164fc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16500
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x16504
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16508
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x1650c
    c.branch(C::Al, 0x166b4)?;
    // 0x16510
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16514
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,12)?)?;
    // 0x16518
    imm(c,C::Al,Op::Mov,R::LR,R::R0,0)?;
    // 0x1651c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x16520
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16524
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x16528
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x1652c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x16530
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16534
    c.branch(C::Al, 0x166b0)?;
    // 0x16538
    c.literal(R::R2,0x16978)?;
    // 0x1653c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,126)?;
    // 0x16540
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16544
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,0)?;
    // 0x16548
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3584)?;
    // 0x1654c
    c.call(0x17b0)?;
    // 0x16550
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // 0x16554
    imm(c,C::Al,Op::Sub,R::R0,R::R0,4)?;
    // 0x16558
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,3)?;
    // 0x1655c
    c.branch(C::Hi, 0x168b8)?;
    // 0x16560
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 0x16564
    imm(c,C::Al,Op::Add,R::R2,R::PC,4)?;
    // 0x16568
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x1656c
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
    c.check(0x16570)?;
    const PRIMARY_CHANNELS:[u32;4]=[0x164b4,0x16590,0x165d4,0x16634];
    for addr in PRIMARY_CHANNELS {c.emit(addr)?;}
    c.check(0x16580)?;
    // 0x16580
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2648)?)?;
    // 0x16584
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,60)?)?;
    // 0x16588
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,0)?;
    // 0x1658c
    imm(c,C::Ne,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x16590
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,2452)?)?;
    // 0x16594
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x16598
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(7)?))?;
    // 0x1659c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x165a0
    imm(c,C::Al,Op::Mov,R::R1,R::R0,344)?;
    // 0x165a4
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(4)?,1740)?)?;
    // 0x165a8
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x165ac
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(7)?))?;
    // 0x165b0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x165b4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x165b8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x165bc
    imm(c,C::Al,Op::Mov,R::R1,R::R0,332)?;
    // 0x165c0
    c.branch(C::Al, 0x16604)?;
    // 0x165c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2648)?)?;
    // 0x165c8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,60)?)?;
    // 0x165cc
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,0)?;
    // 0x165d0
    imm(c,C::Ne,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x165d4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2452)?)?;
    // 0x165d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x165dc
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(6)?))?;
    // 0x165e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x165e4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,340)?;
    // 0x165e8
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(4)?,1736)?)?;
    // 0x165ec
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x165f0
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(6)?))?;
    // 0x165f4
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x165f8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x165fc
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x16600
    imm(c,C::Al,Op::Mov,R::R1,R::R0,328)?;
    // 0x16604
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x16608
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x1660c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x16610
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x16614
    c.branch(C::Al, 0x16698)?;
    // 0x16618
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2648)?)?;
    // 0x1661c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x16620
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,60)?)?;
    // 0x16624
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,2)?;
    // 0x16628
    imm(c,C::Eq,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x1662c
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,0)?;
    // 0x16630
    imm(c,C::Ne,Op::Mov,R::new(5)?,R::R0,1)?;
    // 0x16634
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    // 0x16638
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x1663c
    c.branch(C::Ne, 0x16674)?;
    // 0x16640
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x16644
    imm(c,C::Al,Op::Mov,R::R2,R::R0,496)?;
    // 0x16648
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x1664c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(8)?,R::R1,Shift::Lsl,2)?)?;
    // 0x16650
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x16654
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x16658
    imm(c,C::Al,Op::Sub,R::R1,R::R1,1)?;
    // 0x1665c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x16660
    imm(c,C::Cc,Op::Mov,R::R2,R::R0,1)?;
    // 0x16664
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R2,Shift::Lsl,3,false)?)?;
    // 0x16668
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::R1,Shift::Lsl,3,false)?)?;
    // 0x1666c
    imm(c,C::Al,Op::Add,R::R1,R::R1,148)?;
    // 0x16670
    imm(c,C::Al,Op::Add,R::new(6)?,R::R1,3072)?;
    // 0x16674
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // 0x16678
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(6)?))?;
    // 0x1667c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x16680
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,8)?)?;
    // 0x16684
    imm(c,C::Al,Op::Add,R::LR,R::new(4)?,88)?;
    // 0x16688
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,16)?)?;
    // 0x1668c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x16690
    c.emit(arm32::transfer_multiple(C::Al, true, R::LR, R::R0.bit() | R::R2.bit() | R::LR.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x16694
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x16698
    imm(c,C::Al,Op::Add,R::R1,R::R2,7)?;
    // 0x1669c
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,12)?)?;
    // 0x166a0
    imm(c,C::Al,Op::Bic,R::R1,R::R1,7)?;
    // 0x166a4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x166a8
    imm(c,C::Al,Op::Add,R::R1,R::R0,7)?;
    // 0x166ac
    imm(c,C::Al,Op::Bic,R::R0,R::R1,7)?;
    // 0x166b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x166b4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,564)?;
    // 0x166b8
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(4)?))?;
    // 0x166bc
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(4)?))?;
    // 0x166c0
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,0)?;
    // 0x166c4
    imm(c,C::Al,Op::Mov,R::new(9)?,R::R0,0)?;
    // 0x166c8
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x166cc
    imm(c,C::Al,Op::Orr,R::R1,R::R1,7168)?;
    // 0x166d0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(5)?,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x166d4
    imm(c,C::Al,Op::Add,R::R1,R::new(5)?,4)?;
    // 0x166d8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // 0x166dc
    imm(c,C::Al,Op::Mov,R::R1,R::R0,37888)?;
    // 0x166e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x166e4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,568)?;
    // 0x166e8
    imm(c,C::Al,Op::Orr,R::R1,R::R1,7168)?;
    // 0x166ec
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(8)?,R::R1,Shift::Lsl,0)?)?;
    // 0x166f0
    imm(c,C::Al,Op::Add,R::R1,R::R2,4)?;
    // 0x166f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x166f8
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x166fc
    imm(c,C::Eq,Op::Mov,R::R1,R::R0,1)?;
    // 0x16700
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x16704
    c.literal(R::R1,0x16988)?;
    // 0x16708
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(8)?,R::R1,Shift::Lsl,0)?)?;
    // 0x1670c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x16710
    c.branch(C::Ne, 0x16734)?;
    // 0x16714
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,728)?)?;
    // 0x16718
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x1671c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x16720
    c.branch(C::Ne, 0x16734)?;
    // 0x16724
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,44)?)?;
    // 0x16728
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x1672c
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x16730
    imm(c,C::Eq,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x16734
    imm(c,C::Al,Op::Add,R::R1,R::new(5)?,8)?;
    // 0x16738
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::R12, Shift::Lsl, 12)?)?;
    // 0x1673c
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(10)?))?;
    // 0x16740
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,1)?;
    // 0x16744
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // 0x16748
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1040)?;
    // 0x1674c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,36864)?;
    // 0x16750
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,4)?)?;
    // 0x16754
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,8)?)?;
    // 0x16758
    c.emit(arm32::mov_shifted_register(C::Al, R::new(10)?, R::R1, Shift::Lsl, 4)?)?;
    // 0x1675c
    c.branch(C::Ne, 0x16774)?;
    // 0x16760
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,68)?)?;
    // 0x16764
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16768
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,7)?;
    // 0x1676c
    imm(c,C::Ne,Op::Mov,R::R0,R::R0,1)?;
    // 0x16770
    c.emit(arm32::mov_shifted_register(C::Al, R::new(9)?, R::R0, Shift::Lsl, 16)?)?;
    // 0x16774
    imm(c,C::Al,Op::Add,R::R0,R::R2,8)?;
    // 0x16778
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,1)?;
    // 0x1677c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x16780
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x16784
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x16788
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::new(7)?)?;
    // 0x1678c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,8)?)?;
    // 0x16790
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x16794
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::new(7)?, Shift::Lsl, 2)?)?;
    // 0x16798
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::new(10)?)?;
    // 0x1679c
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::new(9)?)?;
    // 0x167a0
    imm(c,C::Al,Op::Mov,R::new(9)?,R::R0,0)?;
    // 0x167a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,4)?)?;
    // 0x167a8
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,12)?;
    // 0x167ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x167b0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,20)?;
    // 0x167b4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x167b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,8)?)?;
    // 0x167bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x167c0
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsl, 12)?)?;
    // 0x167c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,8)?)?;
    // 0x167c8
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 4)?)?;
    // 0x167cc
    c.branch(C::Ne, 0x167e4)?;
    // 0x167d0
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,68)?)?;
    // 0x167d4
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x167d8
    imm(c,C::Al,Op::Cmp,R::R0,R::R12,7)?;
    // 0x167dc
    imm(c,C::Ne,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x167e0
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 16)?)?;
    // 0x167e4
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(4)?))?;
    // 0x167e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,16)?)?;
    // 0x167ec
    reg(c,C::Al,Op::Orr,R::R1,R::new(4)?,R::R1)?;
    // 0x167f0
    imm(c,C::Al,Op::Add,R::new(4)?,R::R2,36)?;
    // 0x167f4
    reg(c,C::Al,Op::Orr,R::R1,R::R1,R::new(7)?)?;
    // 0x167f8
    reg(c,C::Al,Op::Orr,R::R0,R::R1,R::R0)?;
    // 0x167fc
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::new(3)?)?;
    // 0x16800
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,8)?)?;
    // 0x16804
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1072)?;
    // 0x16808
    imm(c,C::Al,Op::Orr,R::R0,R::R0,36864)?;
    // 0x1680c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // 0x16810
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x16814
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,0)?)?;
    // 0x16818
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x1681c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,52)?;
    // 0x16820
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x16824
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,12)?)?;
    // 0x16828
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,16)?)?;
    // 0x1682c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,16)?)?;
    // 0x16830
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1120)?;
    // 0x16834
    imm(c,C::Al,Op::Orr,R::R0,R::R0,36864)?;
    // 0x16838
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,20)?)?;
    // 0x1683c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,100)?;
    // 0x16840
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R2,20)?)?;
    // 0x16844
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x16848
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,24)?)?;
    // 0x1684c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,5504)?;
    // 0x16850
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R2,24)?)?;
    // 0x16854
    imm(c,C::Al,Op::Orr,R::R0,R::R0,32768)?;
    // 0x16858
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,28)?)?;
    // 0x1685c
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,36)?;
    // 0x16860
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R2,28)?)?;
    // 0x16864
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x16868
    imm(c,C::Al,Op::Mov,R::R0,R::R0,388)?;
    // 0x1686c
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x16870
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,32)?)?;
    // 0x16874
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,2648)?)?;
    // 0x16878
    imm(c,C::Al,Op::Add,R::new(7)?,R::R0,68)?;
    // 0x1687c
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(7)?, R::R1.bit() | R::new(3)?.bit() | R::new(7)?.bit(), arm32::BlockMode::IncrementAfter, false)?)?;
    // 0x16880
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,80)?)?;
    // 0x16884
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,84)?)?;
    // 0x16888
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,88)?)?;
    // 0x1688c
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,0)?)?;
    // 0x16890
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x16894
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,7,false)?)?;
    // 0x16898
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x1689c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,10,false)?)?;
    // 0x168a0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,11,false)?)?;
    // 0x168a4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,512)?;
    // 0x168a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,32)?)?;
    // 0x168ac
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x168b0
    imm(c,C::Al,Op::Sub,R::SP,R::R11,28)?;
    // 0x168b4
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    // 0x168b8
    c.literal(R::R2,0x16978)?;
    // 0x168bc
    imm(c,C::Al,Op::Mov,R::R1,R::R0,167)?;
    // 0x168c0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x168c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x168c8
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3584)?;
    // 0x168cc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x168d0
    c.call(0x17b0)?;
    // 0x168d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // 0x168d8
    imm(c,C::Al,Op::Sub,R::R0,R::R0,4)?;
    // 0x168dc
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,3)?;
    // 0x168e0
    c.branch(C::Hi, 0x16914)?;
    // 0x168e4
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 0x168e8
    imm(c,C::Al,Op::Add,R::R2,R::PC,20)?;
    // 0x168ec
    imm(c,C::Al,Op::Mov,R::R12,R::R0,0)?;
    // 0x168f0
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x168f4
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,0)?;
    // 0x168f8
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x168fc
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,0)?;
    // 0x16900
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
    c.check(0x16904)?;
    const SECONDARY_CHANNELS:[u32;4]=[0x164c0,0x1659c,0x165e0,0x16680];
    for addr in SECONDARY_CHANNELS {c.emit(addr)?;}
    c.check(0x16914)?;
    // 0x16914
    c.literal(R::R1,0x1697c)?;
    // 0x16918
    c.literal(R::R2,0x16980)?;
    // 0x1691c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,744)?;
    // 0x16920
    imm(c,C::Al,Op::Mov,R::R0,R::R0,8)?;
    // 0x16924
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,3072)?;
    // 0x16928
    c.call(0x2ec94)?;
    // 0x1692c
    c.literal(R::R1,0x16984)?;
    // 0x16930
    imm(c,C::Al,Op::Mov,R::R0,R::R0,8)?;
    // 0x16934
    c.call(0x2ec94)?;
    // 0x16938
    c.literal(R::R2,0x16978)?;
    // 0x1693c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,233)?;
    // 0x16940
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16944
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x16948
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3584)?;
    // 0x1694c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16950
    c.call(0x17b0)?;
    // 0x16954
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16958
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x1695c
    imm(c,C::Al,Op::Mov,R::R12,R::R0,0)?;
    // 0x16960
    imm(c,C::Al,Op::Mov,R::LR,R::R0,0)?;
    // 0x16964
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x16968
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x1696c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16970
    c.branch(C::Al, 0x16698)?;
    c.check(0x16974)?;
    const CONTROLLER_DIAGNOSTICS:[u32;6]=[
        0xffff_ede8,0x000e_3d16,0x000e_428c,
        0x000e_42ab,0x000e_2788,0xffff_e1d8,
    ];
    for addr in CONTROLLER_DIAGNOSTICS {c.emit(addr)?;}
    c.check(0x1698c)
}


/// Reconstruct codec frame-plane descriptor sizes, command packet scheduling,
/// and the controller's four-mode runtime selector. The function returns
/// at 0x174d4 and has three trailing literal constants.
fn build_codec_frame_planes(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x1698c)?;
    // 0x1698c
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x16990
    imm(c,C::Al,Op::Add,R::R11,R::SP,28)?;
    // 0x16994
    imm(c,C::Al,Op::Sub,R::SP,R::SP,44)?;
    // 0x16998
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R1))?;
    // 0x1699c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1)?;
    // 0x169a0
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R0))?;
    // 0x169a4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,2352)?)?;
    // 0x169a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2356)?)?;
    // 0x169ac
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,2388)?)?;
    // 0x169b0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,16)?)?;
    // 0x169b4
    imm(c,C::Al,Op::Add,R::new(3)?,R::R0,7)?;
    // 0x169b8
    imm(c,C::Al,Op::Add,R::R2,R::R2,15)?;
    // 0x169bc
    c.emit(arm32::mov_shifted_register(C::Al, R::new(10)?, R::new(3)?, Shift::Asr, 3)?)?;
    // 0x169c0
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::R2, Shift::Asr, 4)?)?;
    // 0x169c4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x169c8
    c.branch(C::Eq, 0x16a18)?;
    // 0x169cc
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,16)?;
    // 0x169d0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,6)?;
    // 0x169d4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x169d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x169dc
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,12)?)?;
    // 0x169e0
    c.branch(C::Eq, 0x16a68)?;
    // 0x169e4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,7)?;
    // 0x169e8
    c.branch(C::Ne, 0x16aa0)?;
    // 0x169ec
    c.literal(R::R2,0x174d8)?;
    // 0x169f0
    imm(c,C::Al,Op::Mov,R::R1,R::R0,109)?;
    // 0x169f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x169f8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x169fc
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3840)?;
    // 0x16a00
    c.call(0x17b0)?;
    // 0x16a04
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,88)?)?;
    // 0x16a08
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,92)?)?;
    // 0x16a0c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16a10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x16a14
    c.branch(C::Al, 0x16a90)?;
    // 0x16a18
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,68)?)?;
    // 0x16a1c
    imm(c,C::Al,Op::Sub,R::R1,R::R1,4)?;
    // 0x16a20
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,3)?;
    // 0x16a24
    c.branch(C::Hi, 0x16ad0)?;
    // 0x16a28
    imm(c,C::Al,Op::Add,R::R0,R::R0,15)?;
    // 0x16a2c
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 2)?)?;
    // 0x16a30
    imm(c,C::Al,Op::Add,R::R2,R::PC,4)?;
    // 0x16a34
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Asr, 4)?)?;
    // 0x16a38
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x16a3c
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(6)?,R::R1,R::R12,Shift::Asr,20,false)?)?;
    // 0x16a40
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(6)?,R::R1,R::new(8)?,Shift::Lsl,R::R11,false)?)?;
    // 0x16a44
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(6)?,R::R1,R::new(4)?,Shift::Ror,R::R11,false)?)?;
    // 0x16a48
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(6)?,R::R1,R::new(4)?,Shift::Ror,23,false)?)?;
    // 0x16a4c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // 0x16a50
    imm(c,C::Al,Op::Mov,R::R1,R::R0,348)?;
    // 0x16a54
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x16a58
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,2)?)?;
    // 0x16a5c
    c.emit(arm32::transfer_word_register(C::Al,true,R::LR,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x16a60
    imm(c,C::Al,Op::Mov,R::R1,R::R0,336)?;
    // 0x16a64
    c.branch(C::Al, 0x16b30)?;
    // 0x16a68
    c.literal(R::R2,0x174d8)?;
    // 0x16a6c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,242)?;
    // 0x16a70
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x16a74
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16a78
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3840)?;
    // 0x16a7c
    c.call(0x17b0)?;
    // 0x16a80
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,88)?)?;
    // 0x16a84
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,92)?)?;
    // 0x16a88
    imm(c,C::Al,Op::Mov,R::R0,R::R0,2048)?;
    // 0x16a8c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x16a90
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16a94
    imm(c,C::Al,Op::Mov,R::new(10)?,R::R0,0)?;
    // 0x16a98
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(7)?))?;
    // 0x16a9c
    c.branch(C::Al, 0x16ac0)?;
    // 0x16aa0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x16aa4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16aa8
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x16aac
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,0)?;
    // 0x16ab0
    imm(c,C::Al,Op::Mov,R::new(10)?,R::R0,0)?;
    // 0x16ab4
    imm(c,C::Al,Op::Mov,R::new(8)?,R::R0,0)?;
    // 0x16ab8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x16abc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16ac0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x16ac4
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16ac8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x16acc
    c.branch(C::Al, 0x16ce8)?;
    // 0x16ad0
    c.literal(R::R2,0x174d8)?;
    // 0x16ad4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,221)?;
    // 0x16ad8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16adc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x16ae0
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,12)?)?;
    // 0x16ae4
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x16ae8
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3840)?;
    // 0x16aec
    c.call(0x17b0)?;
    // 0x16af0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16af4
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,16)?;
    // 0x16af8
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x16afc
    imm(c,C::Al,Op::Mov,R::LR,R::R0,0)?;
    // 0x16b00
    imm(c,C::Al,Op::Mov,R::new(10)?,R::R0,0)?;
    // 0x16b04
    imm(c,C::Al,Op::Mov,R::new(8)?,R::R0,0)?;
    // 0x16b08
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x16b0c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16b10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x16b14
    c.branch(C::Al, 0x16c9c)?;
    // 0x16b18
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // 0x16b1c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,344)?;
    // 0x16b20
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x16b24
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,2)?)?;
    // 0x16b28
    c.emit(arm32::transfer_word_register(C::Al,true,R::LR,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x16b2c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,332)?;
    // 0x16b30
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x16b34
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,16)?;
    // 0x16b38
    imm(c,C::Al,Op::Mov,R::new(10)?,R::R0,0)?;
    // 0x16b3c
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(8)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x16b40
    imm(c,C::Al,Op::Mov,R::R1,R::R0,4)?;
    // 0x16b44
    imm(c,C::Al,Op::Add,R::R0,R::new(8)?,31)?;
    // 0x16b48
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(8)?))?;
    // 0x16b4c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R1,R::R0,Shift::Lsr,5,false)?)?;
    // 0x16b50
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 5)?)?;
    // 0x16b54
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x16b58
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16b5c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x16b60
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x16b64
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x16b68
    imm(c,C::Al,Op::Mov,R::R0,R::R0,2)?;
    // 0x16b6c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x16b70
    c.branch(C::Al, 0x16c9c)?;
    // 0x16b74
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x16b78
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,728)?)?;
    // 0x16b7c
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(3)?))?;
    // 0x16b80
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,328)?;
    // 0x16b84
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x16b88
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?;
    // 0x16b8c
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x16b90
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,12)?)?;
    // 0x16b94
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x16b98
    imm(c,C::Eq,Op::Mov,R::R1,R::R0,1)?;
    // 0x16b9c
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(9)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x16ba0
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x16ba4
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,0)?;
    // 0x16ba8
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,96)?;
    // 0x16bac
    imm(c,C::Hi,Op::Mov,R::new(7)?,R::R0,1)?;
    // 0x16bb0
    reg(c,C::Al,Op::And,R::new(10)?,R::R1,R::new(7)?)?;
    // 0x16bb4
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,340)?;
    // 0x16bb8
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?;
    // 0x16bbc
    c.emit(arm32::transfer_word_register(C::Al,true,R::LR,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x16bc0
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(5)?))?;
    // 0x16bc4
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,97)?;
    // 0x16bc8
    c.branch(C::Cc, 0x16c48)?;
    // 0x16bcc
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x16bd0
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 0x16bd4
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(6)?,2648)?)?;
    // 0x16bd8
    c.emit(arm32::mov_register(C::Eq, R::new(10)?, R::R1))?;
    // 0x16bdc
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,40)?)?;
    // 0x16be0
    c.branch(C::Al, 0x16c4c)?;
    // 0x16be4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,2648)?)?;
    // 0x16be8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,88)?)?;
    // 0x16bec
    imm(c,C::Al,Op::Mov,R::new(10)?,R::R0,0)?;
    // 0x16bf0
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(6)?,92)?)?;
    // 0x16bf4
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,32)?;
    // 0x16bf8
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R1,40)?)?;
    // 0x16bfc
    c.emit(arm32::mov_register(C::Al, R::R1, R::R2))?;
    // 0x16c00
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x16c04
    reg(c,C::Al,Op::Cmp,R::R0,R::R2,R::new(8)?)?;
    // 0x16c08
    c.emit(arm32::data_register(C::Hi,Op::Add,R::R1,R::new(8)?,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x16c0c
    imm(c,C::Hi,Op::Mov,R::new(10)?,R::R0,1)?;
    // 0x16c10
    c.emit(arm32::mov_register(C::Ls, R::new(8)?, R::R2))?;
    // 0x16c14
    imm(c,C::Al,Op::Add,R::R1,R::R1,31)?;
    // 0x16c18
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::R1, Shift::Lsr, 5)?)?;
    // 0x16c1c
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(7)?,R::R0)?)?;
    // 0x16c20
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x16c24
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x16c28
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,R::new(10)?,false)?)?;
    // 0x16c2c
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::R2))?;
    // 0x16c30
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x16c34
    imm(c,C::Al,Op::Add,R::new(7)?,R::R1,2)?;
    // 0x16c38
    imm(c,C::Al,Op::Add,R::R1,R::R1,5)?;
    // 0x16c3c
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 2)?)?;
    // 0x16c40
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x16c44
    c.branch(C::Al, 0x16c9c)?;
    // 0x16c48
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 0x16c4c
    imm(c,C::Al,Op::Add,R::R0,R::R0,3)?;
    // 0x16c50
    imm(c,C::Al,Op::Mvn,R::R1,R::R0,2147483649)?;
    // 0x16c54
    imm(c,C::Al,Op::Cmp,R::R0,R::new(10)?,0)?;
    // 0x16c58
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(7)?))?;
    // 0x16c5c
    imm(c,C::Al,Op::Mov,R::new(5)?,R::R0,16)?;
    // 0x16c60
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // 0x16c64
    imm(c,C::Al,Op::Add,R::R0,R::R12,3)?;
    // 0x16c68
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1)?;
    // 0x16c6c
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsr,2,false)?)?;
    // 0x16c70
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x16c74
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsr,2,false)?)?;
    // 0x16c78
    imm(c,C::Al,Op::Mov,R::R0,R::R0,31)?;
    // 0x16c7c
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,5,false)?)?;
    // 0x16c80
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x16c84
    reg(c,C::Al,Op::Add,R::R0,R::R0,R::new(7)?)?;
    // 0x16c88
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::R0, Shift::Lsr, 5)?)?;
    // 0x16c8c
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(7)?)?)?;
    // 0x16c90
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x16c94
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,R::new(10)?,false)?)?;
    // 0x16c98
    imm(c,C::Al,Op::Add,R::new(7)?,R::R0,2)?;
    // 0x16c9c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,728)?)?;
    // 0x16ca0
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x16ca4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 0x16ca8
    c.branch(C::Ne, 0x16cc0)?;
    // 0x16cac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,68)?)?;
    // 0x16cb0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x16cb4
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,7)?;
    // 0x16cb8
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 0x16cbc
    c.emit(arm32::mov_shifted_register(C::Ne, R::R0, R::R1, Shift::Lsl, 2)?)?;
    // 0x16cc0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x16cc4
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1)?;
    // 0x16cc8
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::LR))?;
    // 0x16ccc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // 0x16cd0
    c.literal(R::R1,0x174dc)?;
    // 0x16cd4
    reg(c,C::Al,Op::Cmp,R::R0,R::R0,R::R1)?;
    // 0x16cd8
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,12)?;
    // 0x16cdc
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(7)?)?)?;
    // 0x16ce0
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(3)?))?;
    // 0x16ce4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x16ce8
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x16cec
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x16cf0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2360)?)?;
    // 0x16cf4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,68)?)?;
    // 0x16cf8
    reg(c,C::Al,Op::Add,R::R0,R::new(6)?,R::R0)?;
    // 0x16cfc
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,972)?)?;
    // 0x16d00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2648)?)?;
    // 0x16d04
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,40)?)?;
    // 0x16d08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,88)?)?;
    // 0x16d0c
    reg(c,C::Al,Op::Cmp,R::R0,R::R1,R::R0)?;
    // 0x16d10
    c.branch(C::Cs, 0x16d40)?;
    // 0x16d14
    reg(c,C::Al,Op::Add,R::R0,R::new(7)?,R::R1)?;
    // 0x16d18
    imm(c,C::Al,Op::Sub,R::R0,R::R0,1)?;
    // 0x16d1c
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x16d20
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x16d24
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2364)?)?;
    // 0x16d28
    c.branch(C::Cc, 0x16d40)?;
    // 0x16d2c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,68)?)?;
    // 0x16d30
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,6)?;
    // 0x16d34
    reg(c,C::Cs,Op::Add,R::R0,R::new(6)?,R::R0)?;
    // 0x16d38
    imm(c,C::Cs,Op::Mov,R::R1,R::R0,1)?;
    // 0x16d3c
    c.emit(arm32::transfer_byte(C::Cs,false,R::R1,R::R0,972)?)?;
    // 0x16d40
    imm(c,C::Al,Op::Cmp,R::R0,R::new(5)?,16)?;
    // 0x16d44
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,28)?)?;
    // 0x16d48
    c.branch(C::Ne, 0x16d6c)?;
    // 0x16d4c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,68)?)?;
    // 0x16d50
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,16)?;
    // 0x16d54
    imm(c,C::Al,Op::Bic,R::R0,R::R0,1)?;
    // 0x16d58
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,4)?;
    // 0x16d5c
    imm(c,C::Al,Op::Add,R::R0,R::new(4)?,15)?;
    // 0x16d60
    imm(c,C::Eq,Op::Mov,R::new(7)?,R::R0,48)?;
    // 0x16d64
    imm(c,C::Al,Op::Bic,R::new(4)?,R::R0,15)?;
    // 0x16d68
    c.branch(C::Al, 0x16da0)?;
    // 0x16d6c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,2648)?)?;
    // 0x16d70
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::new(5)?, Shift::Lsr, 3)?)?;
    // 0x16d74
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,712)?)?;
    // 0x16d78
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x16d7c
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    // 0x16d80
    imm(c,C::Eq,Op::Mov,R::R1,R::R0,1)?;
    // 0x16d84
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x16d88
    reg(c,C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?)?;
    // 0x16d8c
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x16d90
    imm(c,C::Al,Op::Sub,R::R0,R::R0,1)?;
    // 0x16d94
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::R2, Shift::Lsl, 2)?)?;
    // 0x16d98
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x16d9c
    c.emit(arm32::multiply_register(C::Al,R::new(4)?,R::new(7)?,R::R0)?)?;
    // 0x16da0
    reg(c,C::Al,Op::Add,R::R0,R::new(8)?,R::new(5)?)?;
    // 0x16da4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x16da8
    imm(c,C::Al,Op::Sub,R::R0,R::R0,1)?;
    // 0x16dac
    c.emit(arm32::branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    // 0x16db0
    imm(c,C::Al,Op::Mov,R::R2,R::R0,684)?;
    // 0x16db4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,148)?;
    // 0x16db8
    imm(c,C::Al,Op::Orr,R::R2,R::R2,7168)?;
    // 0x16dbc
    imm(c,C::Al,Op::Orr,R::R1,R::R1,3072)?;
    // 0x16dc0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(6)?,R::R2,Shift::Lsl,0)?)?;
    // 0x16dc4
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x16dc8
    c.branch(C::Eq, 0x16dd4)?;
    // 0x16dcc
    reg(c,C::Al,Op::Add,R::R1,R::new(6)?,R::R1)?;
    // 0x16dd0
    c.branch(C::Al, 0x16e04)?;
    // 0x16dd4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,12)?)?;
    // 0x16dd8
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,496)?;
    // 0x16ddc
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?;
    // 0x16de0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(9)?,R::R2,Shift::Lsl,2)?)?;
    // 0x16de4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x16de8
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x16dec
    imm(c,C::Al,Op::Sub,R::R2,R::R2,1)?;
    // 0x16df0
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x16df4
    imm(c,C::Cc,Op::Mov,R::new(3)?,R::R0,1)?;
    // 0x16df8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x16dfc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,3,false)?)?;
    // 0x16e00
    reg(c,C::Al,Op::Add,R::R1,R::R2,R::R1)?;
    // 0x16e04
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,16)?)?;
    // 0x16e08
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,36)?)?;
    // 0x16e0c
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,4)?)?;
    // 0x16e10
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    // 0x16e14
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,2)?;
    // 0x16e18
    c.branch(C::Eq, 0x16e38)?;
    // 0x16e1c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x16e20
    imm(c,C::Al,Op::Cmp,R::R0,R::new(3)?,7)?;
    // 0x16e24
    c.branch(C::Ne, 0x16e84)?;
    // 0x16e28
    imm(c,C::Al,Op::Mov,R::R0,R::R0,6144)?;
    // 0x16e2c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x16e30
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16e34
    c.branch(C::Al, 0x16e90)?;
    // 0x16e38
    imm(c,C::Al,Op::Add,R::R0,R::R0,1)?;
    // 0x16e3c
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::new(7)?)?)?;
    // 0x16e40
    imm(c,C::Al,Op::Mov,R::R0,R::R0,64)?;
    // 0x16e44
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,828)?)?;
    // 0x16e48
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,840)?)?;
    // 0x16e4c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,8)?)?;
    // 0x16e50
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x16e54
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,728)?)?;
    // 0x16e58
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,256)?;
    // 0x16e5c
    c.emit(arm32::signed_multiply_bottom_halves(C::Al,R::new(7)?,R::R0,R::new(5)?)?)?;
    // 0x16e60
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,2)?;
    // 0x16e64
    c.branch(C::Ne, 0x16e78)?;
    // 0x16e68
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,68)?)?;
    // 0x16e6c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x16e70
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,7)?;
    // 0x16e74
    c.emit(arm32::mov_shifted_register(C::Ne, R::new(7)?, R::new(3)?, Shift::Lsl, 2)?)?;
    // 0x16e78
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 0x16e7c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,24)?)?;
    // 0x16e80
    c.branch(C::Al, 0x16e90)?;
    // 0x16e84
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,0)?;
    // 0x16e88
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16e8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,24)?)?;
    // 0x16e90
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,564)?;
    // 0x16e94
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(6)?))?;
    // 0x16e98
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,232)?;
    // 0x16e9c
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(8)?))?;
    // 0x16ea0
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(6)?))?;
    // 0x16ea4
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,7168)?;
    // 0x16ea8
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,37888)?;
    // 0x16eac
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x16eb0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x16eb4
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,5)?;
    // 0x16eb8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,8)?)?;
    // 0x16ebc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,36)?)?;
    // 0x16ec0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x16ec4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,3)?;
    // 0x16ec8
    imm(c,C::Eq,Op::Mov,R::R1,R::R0,2)?;
    // 0x16ecc
    imm(c,C::Eq,Op::Mov,R::new(7)?,R::R0,7)?;
    // 0x16ed0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R2,Shift::Lsr,R::R1,false)?)?;
    // 0x16ed4
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 8)?)?;
    // 0x16ed8
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsr,R::new(7)?,false)?)?;
    // 0x16edc
    imm(c,C::Al,Op::Mov,R::R1,R::R0,568)?;
    // 0x16ee0
    imm(c,C::Al,Op::Orr,R::R1,R::R1,7168)?;
    // 0x16ee4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(6)?,R::new(8)?,R::R1,Shift::Lsl,0)?)?;
    // 0x16ee8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x16eec
    imm(c,C::Al,Op::Mov,R::R0,R::R0,216)?;
    // 0x16ef0
    imm(c,C::Al,Op::Add,R::new(9)?,R::new(6)?,20)?;
    // 0x16ef4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x16ef8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,4)?)?;
    // 0x16efc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x16f00
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // 0x16f04
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,8,false)?)?;
    // 0x16f08
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x16f0c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,220)?;
    // 0x16f10
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x16f14
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,8)?)?;
    // 0x16f18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::new(10)?,Shift::Lsl,31,false)?)?;
    // 0x16f1c
    imm(c,C::Al,Op::Add,R::new(10)?,R::new(3)?,20)?;
    // 0x16f20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // 0x16f24
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1248)?;
    // 0x16f28
    imm(c,C::Al,Op::Orr,R::R0,R::R0,36864)?;
    // 0x16f2c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,12)?)?;
    // 0x16f30
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x16f34
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x16f38
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R12,R::R0,Shift::Lsl,20,false)?)?;
    // 0x16f3c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,12,false)?)?;
    // 0x16f40
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x16f44
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // 0x16f48
    imm(c,C::Al,Op::Mov,R::R0,R::R0,228)?;
    // 0x16f4c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,0)?)?;
    // 0x16f50
    imm(c,C::Al,Op::Orr,R::R0,R::R0,37888)?;
    // 0x16f54
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,16)?)?;
    // 0x16f58
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,0)?)?;
    // 0x16f5c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x16f60
    c.call(0x81850)?;
    // 0x16f64
    imm(c,C::Al,Op::Add,R::R0,R::R1,31)?;
    // 0x16f68
    imm(c,C::Al,Op::Mvn,R::R2,R::R0,0)?;
    // 0x16f6c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,1360)?;
    // 0x16f70
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(4)?))?;
    // 0x16f74
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R0,Shift::Lsr,5,false)?)?;
    // 0x16f78
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 5)?)?;
    // 0x16f7c
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,36864)?;
    // 0x16f80
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsr,5,false)?)?;
    // 0x16f84
    imm(c,C::Al,Op::Mov,R::R2,R::R0,0)?;
    // 0x16f88
    c.emit(arm32::mov_shifted_register(C::Cs, R::R0, R::new(5)?, Shift::Lsr, 5)?)?;
    // 0x16f8c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,16,false)?)?;
    // 0x16f90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // 0x16f94
    c.literal(R::R0,0x174e0)?;
    // 0x16f98
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,24)?)?;
    // 0x16f9c
    reg(c,C::Al,Op::Add,R::R1,R::new(8)?,R::R0)?;
    // 0x16fa0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x16fa4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // 0x16fa8
    imm(c,C::Al,Op::Cmp,R::R0,R::new(6)?,0)?;
    // 0x16fac
    c.branch(C::Eq, 0x16fe8)?;
    // 0x16fb0
    imm(c,C::Al,Op::Add,R::new(6)?,R::new(6)?,127)?;
    // 0x16fb4
    imm(c,C::Al,Op::Add,R::new(7)?,R::new(10)?,8)?;
    // 0x16fb8
    imm(c,C::Al,Op::Add,R::new(4)?,R::new(9)?,8)?;
    // 0x16fbc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,0)?)?;
    // 0x16fc0
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(6)?, Shift::Lsr, 7)?)?;
    // 0x16fc4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x16fc8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsr,7,false)?)?;
    // 0x16fcc
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(9)?,0)?)?;
    // 0x16fd0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R12,0)?)?;
    // 0x16fd4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,4)?)?;
    // 0x16fd8
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,0)?)?;
    // 0x16fdc
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(9)?,4)?)?;
    // 0x16fe0
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(4)?))?;
    // 0x16fe4
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(7)?))?;
    // 0x16fe8
    imm(c,C::Al,Op::Add,R::R2,R::R2,1)?;
    // 0x16fec
    imm(c,C::Al,Op::Add,R::new(3)?,R::new(3)?,4)?;
    // 0x16ff0
    imm(c,C::Al,Op::Add,R::R1,R::R1,4)?;
    // 0x16ff4
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,4)?;
    // 0x16ff8
    c.branch(C::Ne, 0x16fa4)?;
    // 0x16ffc
    imm(c,C::Al,Op::Mov,R::R1,R::R0,5440)?;
    // 0x17000
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R12))?;
    // 0x17004
    imm(c,C::Al,Op::Orr,R::R1,R::R1,32768)?;
    // 0x17008
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x1700c
    imm(c,C::Al,Op::Add,R::R1,R::LR,127)?;
    // 0x17010
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R1, Shift::Lsr, 7)?)?;
    // 0x17014
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R2,Shift::Lsl,16,false)?)?;
    // 0x17018
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsr,7,false)?)?;
    // 0x1701c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x17020
    imm(c,C::Al,Op::Mov,R::R2,R::R0,324)?;
    // 0x17024
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x17028
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,4)?)?;
    // 0x1702c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,8)?;
    // 0x17030
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,32)?)?;
    // 0x17034
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // 0x17038
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,16)?;
    // 0x1703c
    imm(c,C::Eq,Op::Mov,R::R2,R::R0,4)?;
    // 0x17040
    imm(c,C::Al,Op::Cmp,R::R0,R::new(7)?,16)?;
    // 0x17044
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::new(6)?)?)?;
    // 0x17048
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x1704c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,4)?)?;
    // 0x17050
    imm(c,C::Al,Op::Add,R::R1,R::new(10)?,12)?;
    // 0x17054
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x17058
    imm(c,C::Al,Op::Mov,R::R1,R::R0,328)?;
    // 0x1705c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x17060
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,8)?)?;
    // 0x17064
    imm(c,C::Al,Op::Add,R::R1,R::new(9)?,12)?;
    // 0x17068
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x1706c
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R2,R::new(6)?,R::R0)?)?;
    // 0x17070
    imm(c,C::Al,Op::Mov,R::R0,R::R0,2)?;
    // 0x17074
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // 0x17078
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,1)?;
    // 0x1707c
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::new(3)?)?)?;
    // 0x17080
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x17084
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,8)?)?;
    // 0x17088
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::new(3)?,R::R1)?)?;
    // 0x1708c
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x17090
    imm(c,C::Al,Op::Mov,R::R1,R::R0,4288)?;
    // 0x17094
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,872)?;
    // 0x17098
    c.literal(R::R2,0x174d8)?;
    // 0x1709c
    imm(c,C::Ls,Op::Mov,R::R0,R::R0,1)?;
    // 0x170a0
    c.call(0x17b0)?;
    // 0x170a4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,36)?)?;
    // 0x170a8
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(4)?))?;
    // 0x170ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x170b0
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,2)?;
    // 0x170b4
    c.branch(C::Ne, 0x17394)?;
    // 0x170b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x170bc
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,1660)?)?;
    // 0x170c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(5)?,1676)?)?;
    // 0x170c4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x170c8
    c.call(0x126f4)?;
    // 0x170cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x170d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,2396)?)?;
    // 0x170d4
    imm(c,C::Al,Op::Sub,R::R0,R::R0,4)?;
    // 0x170d8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,3)?;
    // 0x170dc
    c.branch(C::Hi, 0x17110)?;
    // 0x170e0
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x170e4
    imm(c,C::Al,Op::Add,R::R1,R::PC,0)?;
    // 0x170e8
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x170ec)?;
    const FRAME_LAYOUT_BRANCHES:[u32;4]=[0x170fc,0x17130,0x170fc,0x1714c];
    for target in FRAME_LAYOUT_BRANCHES {c.emit(target)?;}
    c.check(0x170fc)?;
    // 0x170fc
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,2412)?)?;
    // 0x17100
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(5)?,2416)?)?;
    // 0x17104
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2424)?)?;
    // 0x17108
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,372)?;
    // 0x1710c
    c.branch(C::Al, 0x17140)?;
    // 0x17110
    c.literal(R::R2,0x174d8)?;
    // 0x17114
    imm(c,C::Al,Op::Mov,R::R1,R::R0,248)?;
    // 0x17118
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    // 0x1711c
    imm(c,C::Al,Op::Mov,R::new(9)?,R::R0,0)?;
    // 0x17120
    imm(c,C::Al,Op::Orr,R::R1,R::R1,4096)?;
    // 0x17124
    c.call(0x17b0)?;
    // 0x17128
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(4)?))?;
    // 0x1712c
    c.branch(C::Al, 0x172d8)?;
    // 0x17130
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2416)?)?;
    // 0x17134
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,2420)?)?;
    // 0x17138
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(5)?,2424)?)?;
    // 0x1713c
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,364)?;
    // 0x17140
    imm(c,C::Al,Op::Add,R::R2,R::R0,2048)?;
    // 0x17144
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(7)?))?;
    // 0x17148
    c.branch(C::Al, 0x1715c)?;
    // 0x1714c
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(5)?,2400)?)?;
    // 0x17150
    imm(c,C::Al,Op::Add,R::R0,R::new(5)?,364)?;
    // 0x17154
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    // 0x17158
    imm(c,C::Al,Op::Add,R::R2,R::R0,2048)?;
    // 0x1715c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x17160
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(4)?))?;
    // 0x17164
    imm(c,C::Al,Op::Cmp,R::R0,R::R2,0)?;
    // 0x17168
    c.branch(C::Eq, 0x171d4)?;
    // 0x1716c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::LR,0)?)?;
    // 0x17170
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,516)?;
    // 0x17174
    imm(c,C::Al,Op::Mov,R::R0,R::R0,81)?;
    // 0x17178
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,38400)?;
    // 0x1717c
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,37888)?;
    // 0x17180
    imm(c,C::Al,Op::Orr,R::R0,R::R0,4096)?;
    // 0x17184
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(3)?,0)?)?;
    // 0x17188
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,0)?)?;
    // 0x1718c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x17190
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,4)?)?;
    // 0x17194
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,4)?)?;
    // 0x17198
    imm(c,C::Al,Op::Mov,R::R2,R::R0,644)?;
    // 0x1719c
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x171a0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,8)?)?;
    // 0x171a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x171a8
    imm(c,C::Al,Op::Add,R::R0,R::new(3)?,16)?;
    // 0x171ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x171b0
    imm(c,C::Al,Op::Mov,R::R0,R::R0,5760)?;
    // 0x171b4
    imm(c,C::Al,Op::Orr,R::R0,R::R0,32768)?;
    // 0x171b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,12)?)?;
    // 0x171bc
    imm(c,C::Al,Op::Add,R::R0,R::new(4)?,16)?;
    // 0x171c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x171c4
    imm(c,C::Al,Op::Add,R::R0,R::new(7)?,255)?;
    // 0x171c8
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x171cc
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x171d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x171d4
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,0)?;
    // 0x171d8
    c.branch(C::Eq, 0x17244)?;
    // 0x171dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x171e0
    imm(c,C::Al,Op::Mov,R::R2,R::R0,516)?;
    // 0x171e4
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,65)?;
    // 0x171e8
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,38400)?;
    // 0x171ec
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x171f0
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?;
    // 0x171f4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x171f8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // 0x171fc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // 0x17200
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x17204
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,4)?)?;
    // 0x17208
    imm(c,C::Al,Op::Mov,R::R1,R::R0,644)?;
    // 0x1720c
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x17210
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x17214
    imm(c,C::Al,Op::Add,R::R1,R::R0,16)?;
    // 0x17218
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,8)?)?;
    // 0x1721c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x17220
    imm(c,C::Al,Op::Mov,R::R1,R::R0,5760)?;
    // 0x17224
    imm(c,C::Al,Op::Orr,R::R1,R::R1,32768)?;
    // 0x17228
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x1722c
    imm(c,C::Al,Op::Add,R::R0,R::R2,16)?;
    // 0x17230
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x17234
    imm(c,C::Al,Op::Add,R::R0,R::new(7)?,255)?;
    // 0x17238
    imm(c,C::Al,Op::Bic,R::R0,R::R0,255)?;
    // 0x1723c
    imm(c,C::Al,Op::Orr,R::R0,R::R0,1)?;
    // 0x17240
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,12)?)?;
    // 0x17244
    imm(c,C::Al,Op::Cmp,R::R0,R::new(10)?,0)?;
    // 0x17248
    c.branch(C::Eq, 0x172d4)?;
    // 0x1724c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x17250
    imm(c,C::Al,Op::Mov,R::R1,R::R0,516)?;
    // 0x17254
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,644)?;
    // 0x17258
    imm(c,C::Al,Op::Add,R::new(7)?,R::new(7)?,255)?;
    // 0x1725c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,97)?;
    // 0x17260
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,38400)?;
    // 0x17264
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x17268
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,37888)?;
    // 0x1726c
    imm(c,C::Al,Op::Bic,R::new(7)?,R::new(7)?,255)?;
    // 0x17270
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,1)?;
    // 0x17274
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x17278
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // 0x1727c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // 0x17280
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x17284
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R2,4)?)?;
    // 0x17288
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,8)?)?;
    // 0x1728c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,8)?)?;
    // 0x17290
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,5760)?;
    // 0x17294
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,32768)?;
    // 0x17298
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x1729c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,12)?)?;
    // 0x172a0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x172a4
    imm(c,C::Al,Op::Mov,R::R1,R::R0,113)?;
    // 0x172a8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,16)?)?;
    // 0x172ac
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,20)?)?;
    // 0x172b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R2,20)?)?;
    // 0x172b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,24)?)?;
    // 0x172b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,24)?)?;
    // 0x172bc
    imm(c,C::Al,Op::Add,R::R1,R::R0,32)?;
    // 0x172c0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x172c4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,28)?)?;
    // 0x172c8
    imm(c,C::Al,Op::Add,R::R0,R::R2,32)?;
    // 0x172cc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x172d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,28)?)?;
    // 0x172d4
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R12))?;
    // 0x172d8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2300)?)?;
    // 0x172dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x172e0
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,2)?;
    // 0x172e4
    c.branch(C::Cc, 0x172fc)?;
    // 0x172e8
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,7)?;
    // 0x172ec
    c.branch(C::Ne, 0x172fc)?;
    // 0x172f0
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,0)?;
    // 0x172f4
    c.branch(C::Ne, 0x1730c)?;
    // 0x172f8
    c.branch(C::Al, 0x17394)?;
    // 0x172fc
    imm(c,C::Al,Op::Cmp,R::R0,R::new(9)?,0)?;
    // 0x17300
    c.branch(C::Eq, 0x17394)?;
    // 0x17304
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,7)?;
    // 0x17308
    c.branch(C::Eq, 0x17394)?;
    // 0x1730c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x17310
    imm(c,C::Al,Op::Mov,R::R1,R::R0,516)?;
    // 0x17314
    imm(c,C::Al,Op::Mov,R::new(4)?,R::R0,644)?;
    // 0x17318
    imm(c,C::Al,Op::Add,R::new(7)?,R::new(7)?,255)?;
    // 0x1731c
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,65)?;
    // 0x17320
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,38400)?;
    // 0x17324
    imm(c,C::Al,Op::Orr,R::R1,R::R1,37888)?;
    // 0x17328
    imm(c,C::Al,Op::Orr,R::new(4)?,R::new(4)?,37888)?;
    // 0x1732c
    imm(c,C::Al,Op::Bic,R::new(7)?,R::new(7)?,255)?;
    // 0x17330
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,1)?;
    // 0x17334
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x17338
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // 0x1733c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // 0x17340
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x17344
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R2,4)?)?;
    // 0x17348
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,8)?)?;
    // 0x1734c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,8)?)?;
    // 0x17350
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,5760)?;
    // 0x17354
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,32768)?;
    // 0x17358
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x1735c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,12)?)?;
    // 0x17360
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x17364
    imm(c,C::Al,Op::Mov,R::R1,R::R0,81)?;
    // 0x17368
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,16)?)?;
    // 0x1736c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,20)?)?;
    // 0x17370
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R2,20)?)?;
    // 0x17374
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,24)?)?;
    // 0x17378
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,24)?)?;
    // 0x1737c
    imm(c,C::Al,Op::Add,R::R1,R::R0,32)?;
    // 0x17380
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x17384
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,28)?)?;
    // 0x17388
    imm(c,C::Al,Op::Add,R::R0,R::R2,32)?;
    // 0x1738c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x17390
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,28)?)?;
    // 0x17394
    imm(c,C::Al,Op::Mov,R::R0,R::R0,580)?;
    // 0x17398
    imm(c,C::Al,Op::Mov,R::R1,R::R0,8)?;
    // 0x1739c
    imm(c,C::Al,Op::Mov,R::R2,R::R0,548)?;
    // 0x173a0
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,65)?;
    // 0x173a4
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,8)?;
    // 0x173a8
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,81)?;
    // 0x173ac
    imm(c,C::Al,Op::Orr,R::R0,R::R0,7168)?;
    // 0x173b0
    imm(c,C::Al,Op::Orr,R::R2,R::R2,37888)?;
    // 0x173b4
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,262144)?;
    // 0x173b8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x173bc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,0)?)?;
    // 0x173c0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x173c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x173c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x173cc
    imm(c,C::Al,Op::Mov,R::new(3)?,R::R0,1568)?;
    // 0x173d0
    imm(c,C::Al,Op::Orr,R::new(3)?,R::new(3)?,36864)?;
    // 0x173d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x173d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,4)?)?;
    // 0x173dc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x173e0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,8)?)?;
    // 0x173e4
    imm(c,C::Al,Op::Add,R::new(6)?,R::new(7)?,4)?;
    // 0x173e8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,12)?)?;
    // 0x173ec
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,12)?)?;
    // 0x173f0
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,97)?;
    // 0x173f4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,16)?)?;
    // 0x173f8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,16)?)?;
    // 0x173fc
    imm(c,C::Al,Op::Add,R::new(6)?,R::new(7)?,8)?;
    // 0x17400
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,20)?)?;
    // 0x17404
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,20)?)?;
    // 0x17408
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,113)?;
    // 0x1740c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,24)?)?;
    // 0x17410
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,24)?)?;
    // 0x17414
    imm(c,C::Al,Op::Add,R::new(6)?,R::new(7)?,12)?;
    // 0x17418
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,28)?)?;
    // 0x1741c
    imm(c,C::Al,Op::Add,R::new(7)?,R::new(7)?,16)?;
    // 0x17420
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,28)?)?;
    // 0x17424
    imm(c,C::Al,Op::Mov,R::new(6)?,R::R0,49)?;
    // 0x17428
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,32)?)?;
    // 0x1742c
    imm(c,C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?;
    // 0x17430
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,32)?)?;
    // 0x17434
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,36)?)?;
    // 0x17438
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,36)?)?;
    // 0x1743c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,65)?;
    // 0x17440
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,40)?)?;
    // 0x17444
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?;
    // 0x17448
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,40)?)?;
    // 0x1744c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,28)?;
    // 0x17450
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,44)?)?;
    // 0x17454
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,524288)?;
    // 0x17458
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,44)?)?;
    // 0x1745c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,81)?;
    // 0x17460
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,48)?)?;
    // 0x17464
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?;
    // 0x17468
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,48)?)?;
    // 0x1746c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,36)?;
    // 0x17470
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,52)?)?;
    // 0x17474
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,393216)?;
    // 0x17478
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,52)?)?;
    // 0x1747c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,2)?;
    // 0x17480
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,56)?)?;
    // 0x17484
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?;
    // 0x17488
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,56)?)?;
    // 0x1748c
    imm(c,C::Al,Op::Mov,R::new(7)?,R::R0,42)?;
    // 0x17490
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,60)?)?;
    // 0x17494
    imm(c,C::Al,Op::Orr,R::new(7)?,R::new(7)?,8388608)?;
    // 0x17498
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,60)?)?;
    // 0x1749c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,64)?)?;
    // 0x174a0
    imm(c,C::Al,Op::Mov,R::R2,R::R0,3)?;
    // 0x174a4
    imm(c,C::Al,Op::Orr,R::R2,R::R2,4096)?;
    // 0x174a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,64)?)?;
    // 0x174ac
    imm(c,C::Al,Op::Add,R::R2,R::R1,72)?;
    // 0x174b0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x174b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,68)?)?;
    // 0x174b8
    imm(c,C::Al,Op::Add,R::R1,R::R0,72)?;
    // 0x174bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x174c0
    imm(c,C::Al,Op::Mov,R::R1,R::R0,170)?;
    // 0x174c4
    imm(c,C::Al,Op::Orr,R::R1,R::R1,2097152)?;
    // 0x174c8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,68)?)?;
    // 0x174cc
    imm(c,C::Al,Op::Mov,R::R0,R::R0,1)?;
    // 0x174d0
    imm(c,C::Al,Op::Sub,R::SP,R::R11,28)?;
    // 0x174d4
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), arm32::BlockMode::IncrementAfter, true)?)?;
    c.check(0x174d8)?;
    const FRAME_DIAGNOSTICS:[u32;3]=[0x000e_3d16,0x0401_1006,0xffff_e504];
    for value in FRAME_DIAGNOSTICS {c.emit(value)?;}
    c.check(END)
}

/// Partial independently encoded region; does not read private firmware.
pub fn fragment()->Result<Vec<u8>,String>{
    let skip=START-0x1000; // Scratch prefix is discarded; no padding is emitted.
    let mut out=vec![0;skip];
    let mut c=Code{bytes:&mut out,at:START};
    classify_active_buffers(&mut c)?;
    normalize_codec_layout(&mut c)?;
    classify_codec_mode_and_dimensions(&mut c)?;
    finish_initial_codec_session(&mut c)?;
    setup_next_codec_session(&mut c)?;
    compute_codec_stream_dimensions(&mut c)?;
    route_frame_formats(&mut c)?;
    build_format_packets(&mut c)?;
    build_encoded_stream_flags(&mut c)?;
    configure_secondary_codec_packing(&mut c)?;
    submit_codec_payload_variants(&mut c)?;
    finalize_next_codec_session(&mut c)?;
    write_stream_small_helpers(&mut c)?;
    update_stream_mode_metadata(&mut c)?;
    configure_controller_channels(&mut c)?;
    build_codec_frame_planes(&mut c)?;
    Ok(out.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn first_forward_fragment_is_exact_length(){
        assert_eq!(fragment().unwrap().len(),END-START);
    }
}
