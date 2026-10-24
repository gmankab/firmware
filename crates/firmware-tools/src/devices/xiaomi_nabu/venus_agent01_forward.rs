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
pub const END: usize=0x14800;
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
    Ok(out.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn first_forward_fragment_is_exact_length(){
        assert_eq!(fragment().unwrap().len(),END-START);
    }
}
