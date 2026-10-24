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
pub const END: usize=0x13800;
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
    Ok(out.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn first_forward_fragment_is_exact_length(){
        assert_eq!(fragment().unwrap().len(),END-START);
    }
}
