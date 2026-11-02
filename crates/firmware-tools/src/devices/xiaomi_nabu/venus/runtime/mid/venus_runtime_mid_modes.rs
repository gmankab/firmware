// SPDX-License-Identifier: GPL-3.0-only
//! Multi-codec Venus buffer profile validator and controller event handler.
//! The A32 instruction stream is generated from architecture operand APIs;
//! the 19-entry jump table and 46 constant/diagnostic words are separated
//! as typed data, not arbitrary copied instruction words. This routine
//! performs codec input-size checks and logs invalid profile combinations.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

const VALIDATION_MODE_TARGETS:[u32;19]=[
    0x00024158, 0x00024a30, 0x00023d90, 0x00024a30, 0x00024a30,
    0x000244c8, 0x00024a30, 0x000245ac, 0x00024a30, 0x000247ac,
    0x00024a30, 0x00024a30, 0x00023d90, 0x00024a30, 0x00023d90,
    0x00024a30, 0x00024a30, 0x00024a30, 0x00024920,
];

// Values from the original controller's explicit literal pool; not
// executable instructions (the first value is a signed negative offset).
const VALIDATION_LOG_WORDS:[u32;46]=[
    0xfbfeeffb, 0x000e7bcf, 0x000e72f4, 0x000e2788, 0x000e7c11,
    0x000e70b5, 0x000e7c57, 0x000e7c79, 0x000e7cc1, 0x000e7cea,
    0x000e7d13, 0x000e7af5, 0x000e7b3a, 0x000e7b80, 0x000e791b,
    0x000e795e, 0x000e79a5, 0x000e79e2, 0x000e7a20, 0x000e7a5c,
    0x000e7aa8, 0x000e75eb, 0x000e7636, 0x04011007, 0x000e7691,
    0x000e7732, 0x000e76e1, 0x000e777a, 0x00800201, 0x000e7886,
    0x000e78d5, 0x000e77e4, 0x000e781b, 0x000e784f, 0x000e74e7,
    0x000e7528, 0x000e7571, 0x000e75b7, 0x000e72c7, 0x000e730c,
    0x000e7355, 0x000e741e, 0x000e73bf, 0x000e748a, 0x000e74b7,
    0x000e6a43,
];

fn dispatch_profile(c:&mut Code)->Result<(),String>{
    c.check(0x23cf0)?;
    // 0x23cf0
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x23cf4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x23cf8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x23cfc
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(9)?,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x23d00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2960)?)?;
    // 0x23d04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x23d08
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x23d0c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23d10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x23d14
    c.b(C::Eq,0x23d24)?;
    // 0x23d18
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x23d1c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x23d20
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x23d24
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,4)?)?;
    // 0x23d28
    c.literal(R::R0,0x24c1c)?;
    // 0x23d2c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23d30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,18)?)?;
    // 0x23d34
    c.b(C::Hi,0x24a30)?;
    // 0x23d38
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x23d3c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x23d40
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x23d44)?;
    for target in VALIDATION_MODE_TARGETS {c.emit(target)?;}
    c.check(0x23d90)
}

/// Validate codec frame-index bounds, buffer dimensions and register source.
fn validate_frame_dimensions(c:&mut Code)->Result<(),String>{
    c.check(0x23d90)?;
    // 0x23d90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,624)?)?;
    // 0x23d94
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x23d98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x23d9c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x23da0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23da4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x23da8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x23dac
    c.b(C::Cc,0x23df4)?;
    // 0x23db0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,32768)?)?;
    // 0x23db4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23db8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x23dbc
    c.b(C::Cc,0x23df4)?;
    // 0x23dc0
    c.literal(R::R1,0x24c70)?;
    // 0x23dc4
    c.literal(R::R2,0x24c24)?;
    // 0x23dc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,95)?)?;
    // 0x23dcc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x23dd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23dd4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x23dd8
    c.bl(0x2ec94)?;
    // 0x23ddc
    c.literal(R::R1,0x24c28)?;
    // 0x23de0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23de4
    c.bl(0x2ec94)?;
    // 0x23de8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x23dec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x23df0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x23df4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x23df8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x23dfc
    c.b(C::Hi,0x23e58)?;
    // 0x23e00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,644)?)?;
    // 0x23e04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,648)?)?;
    // 0x23e08
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x23e0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x23e10
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x23e14
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23e18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1280)?)?;
    // 0x23e1c
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,720)?)?;
    // 0x23e20
    c.b(C::Ls,0x23e58)?;
    // 0x23e24
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x23e28
    c.literal(R::R1,0x24c74)?;
    // 0x23e2c
    c.literal(R::R2,0x24c24)?;
    // 0x23e30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,102)?)?;
    // 0x23e34
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x23e38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23e3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x23e40
    c.bl(0x2ec94)?;
    // 0x23e44
    c.literal(R::R1,0x24c28)?;
    // 0x23e48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23e4c
    c.bl(0x2ec94)?;
    // 0x23e50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x23e54
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x23e58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,644)?)?;
    // 0x23e5c
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // 0x23e60
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x23e64
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23e68
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,4)?)?;
    // 0x23e6c
    c.literal(R::new(8)?,0x24c78)?;
    // 0x23e70
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x23e74
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x23e78
    c.b(C::Ls,0x23eac)?;
    // 0x23e7c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x23e80
    c.literal(R::R1,0x24c7c)?;
    // 0x23e84
    c.literal(R::R2,0x24c24)?;
    // 0x23e88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,108)?)?;
    // 0x23e8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23e90
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x23e94
    c.bl(0x2ec94)?;
    // 0x23e98
    c.literal(R::R1,0x24c28)?;
    // 0x23e9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23ea0
    c.bl(0x2ec94)?;
    // 0x23ea4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x23ea8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x23eac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(8)?,12)?)?;
    // 0x23eb0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x23eb4
    c.b(C::Ne,0x23eec)?;
    // 0x23eb8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,648)?)?;
    // 0x23ebc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    // 0x23ec0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x23ec4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23ec8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,64)?)?;
    // 0x23ecc
    c.emit(arm32::data_immediate(C::Cs,Op::Cmp,R::R0,R::R0,127)?)?;
    // 0x23ed0
    c.b(C::Hi,0x23f44)?;
    // 0x23ed4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x23ed8
    c.literal(R::R1,0x24c84)?;
    // 0x23edc
    c.literal(R::R2,0x24c24)?;
    // 0x23ee0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x23ee4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,117)?)?;
    // 0x23ee8
    c.b(C::Al,0x23f24)?;
    // 0x23eec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x23ef0
    c.b(C::Ne,0x23f44)?;
    // 0x23ef4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,648)?)?;
    // 0x23ef8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    // 0x23efc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x23f00
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23f04
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,64)?)?;
    // 0x23f08
    c.emit(arm32::data_immediate(C::Cs,Op::Cmp,R::R0,R::R0,63)?)?;
    // 0x23f0c
    c.b(C::Hi,0x23f44)?;
    // 0x23f10
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x23f14
    c.literal(R::R1,0x24c80)?;
    // 0x23f18
    c.literal(R::R2,0x24c24)?;
    // 0x23f1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,125)?)?;
    // 0x23f20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x23f24
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x23f28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23f2c
    c.bl(0x2ec94)?;
    // 0x23f30
    c.literal(R::R1,0x24c28)?;
    // 0x23f34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23f38
    c.bl(0x2ec94)?;
    // 0x23f3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x23f40
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x23f44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,672)?)?;
    // 0x23f48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,668)?)?;
    // 0x23f4c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x23f50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x23f54
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23f58
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x23f5c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23f60
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x23f64
    c.b(C::Cc,0x23f9c)?;
    // 0x23f68
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x23f6c
    c.literal(R::R1,0x24c88)?;
    // 0x23f70
    c.literal(R::R2,0x24c24)?;
    // 0x23f74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,133)?)?;
    // 0x23f78
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x23f7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23f80
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x23f84
    c.bl(0x2ec94)?;
    // 0x23f88
    c.literal(R::R1,0x24c28)?;
    // 0x23f8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23f90
    c.bl(0x2ec94)?;
    // 0x23f94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x23f98
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x23f9c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x23fa0
    c.b(C::Ne,0x240c0)?;
    // 0x23fa4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,660)?)?;
    // 0x23fa8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x23fac
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23fb0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x23fb4
    c.b(C::Ne,0x23fec)?;
    // 0x23fb8
    c.literal(R::R1,0x24c98)?;
    // 0x23fbc
    c.literal(R::R2,0x24c24)?;
    // 0x23fc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,213)?)?;
    // 0x23fc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x23fc8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x23fcc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x23fd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23fd4
    c.bl(0x2ec94)?;
    // 0x23fd8
    c.literal(R::R1,0x24c28)?;
    // 0x23fdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23fe0
    c.bl(0x2ec94)?;
    // 0x23fe4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x23fe8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x23fec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,228)?)?;
    // 0x23ff0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x23ff4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x23ff8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x23ffc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x24000
    c.b(C::Ne,0x24038)?;
    // 0x24004
    c.literal(R::R1,0x24c9c)?;
    // 0x24008
    c.literal(R::R2,0x24c24)?;
    // 0x2400c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x24010
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2272)?)?;
    // 0x24014
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24018
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2401c
    c.bl(0x2ec94)?;
    // 0x24020
    c.literal(R::R1,0x24c28)?;
    // 0x24024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24028
    c.bl(0x2ec94)?;
    // 0x2402c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x24030
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24034
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24038
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2403c
    c.b(C::Ne,0x23d18)?;
    // 0x24040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,232)?)?;
    // 0x24044
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x24048
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x2404c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,2)?)?;
    // 0x24050
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,28)?)?;
    // 0x24054
    c.b(C::Hi,0x240b0)?;
    // 0x24058
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,1,true)?)?;
    // 0x2405c
    c.b(C::Ne,0x240b0)?;
    // 0x24060
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1264)?)?;
    // 0x24064
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24068
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x2406c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x24070
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x24074
    c.b(C::Hi,0x240b0)?;
    // 0x24078
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,236)?)?;
    // 0x2407c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x24080
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24084
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,2)?)?;
    // 0x24088
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,28)?)?;
    // 0x2408c
    c.b(C::Hi,0x240b0)?;
    // 0x24090
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,1,true)?)?;
    // 0x24094
    c.b(C::Ne,0x240b0)?;
    // 0x24098
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,244)?)?;
    // 0x2409c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x240a0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x240a4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x240a8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x240ac
    c.b(C::Cc,0x23d18)?;
    // 0x240b0
    c.literal(R::R1,0x24ca0)?;
    // 0x240b4
    c.literal(R::R2,0x24c24)?;
    // 0x240b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,232)?)?;
    // 0x240bc
    c.b(C::Al,0x248fc)?;
    // 0x240c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,10)?)?;
    // 0x240c4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x240c8
    c.b(C::Ne,0x23d18)?;
    // 0x240cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,220)?)?;
    // 0x240d0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x240d4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x240d8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x240dc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,13)?)?;
    // 0x240e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,23)?)?;
    // 0x240e4
    c.b(C::Hi,0x240f8)?;
    // 0x240e8
    c.literal(R::new(3)?,0x24c8c)?;
    // 0x240ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x240f0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::R2,Shift::Lsl,R::R1,false)?)?;
    // 0x240f4
    c.b(C::Ne,0x24128)?;
    // 0x240f8
    c.literal(R::R1,0x24c90)?;
    // 0x240fc
    c.literal(R::R2,0x24c24)?;
    // 0x24100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,244)?)?;
    // 0x24104
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2410c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x24110
    c.bl(0x2ec94)?;
    // 0x24114
    c.literal(R::R1,0x24c28)?;
    // 0x24118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2411c
    c.bl(0x2ec94)?;
    // 0x24120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,216)?)?;
    // 0x2412c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x24130
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24134
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x24138
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x2413c
    c.b(C::Cc,0x23d18)?;
    // 0x24140
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x24144
    c.literal(R::R1,0x24c94)?;
    // 0x24148
    c.literal(R::R2,0x24c24)?;
    // 0x2414c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,249)?)?;
    // 0x24150
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24154
    c.b(C::Al,0x248fc)?;
    c.check(0x24158)
}

/// Apply controller-specific accepted-size and invalid-state rules.
fn validate_codec_sizes(c:&mut Code)->Result<(),String>{
    c.check(0x24158)?;
    // 0x24158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2415c
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(9)?))?;
    // 0x24160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x24164
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24168
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,0)?)?;
    // 0x2416c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x24170
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x24174
    c.b(C::Cc,0x241b8)?;
    // 0x24178
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,32768)?)?;
    // 0x2417c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24180
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x24184
    c.b(C::Cc,0x241b8)?;
    // 0x24188
    c.literal(R::R1,0x24c20)?;
    // 0x2418c
    c.literal(R::R2,0x24c24)?;
    // 0x24190
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,123)?)?;
    // 0x24194
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2419c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x241a0
    c.bl(0x2ec94)?;
    // 0x241a4
    c.literal(R::R1,0x24c28)?;
    // 0x241a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x241ac
    c.bl(0x2ec94)?;
    // 0x241b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x241b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x241b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x241bc
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // 0x241c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x241c4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x241c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,4)?)?;
    // 0x241cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x241d0
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x241d4
    c.b(C::Ls,0x24204)?;
    // 0x241d8
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x241dc
    c.literal(R::R1,0x24c2c)?;
    // 0x241e0
    c.literal(R::R2,0x24c24)?;
    // 0x241e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x241e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2432)?)?;
    // 0x241ec
    c.bl(0x2ec94)?;
    // 0x241f0
    c.literal(R::R1,0x24c28)?;
    // 0x241f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x241f8
    c.bl(0x2ec94)?;
    // 0x241fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24200
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24204
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,48)?)?;
    // 0x24208
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x2420c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24210
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24214
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,-4)?)?;
    // 0x24218
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x2421c
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x24220
    c.b(C::Ls,0x24260)?;
    // 0x24224
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x24228
    c.literal(R::R1,0x24c30)?;
    // 0x2422c
    c.literal(R::R2,0x24c24)?;
    // 0x24230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,388)?)?;
    // 0x24234
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24238
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x2423c
    c.bl(0x2ec94)?;
    // 0x24240
    c.literal(R::R1,0x24c28)?;
    // 0x24244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24248
    c.bl(0x2ec94)?;
    // 0x2424c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // 0x24250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24254
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24258
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x2425c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24260
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x24264
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsl,3)?)?;
    // 0x24268
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2426c
    c.b(C::Cc,0x2429c)?;
    // 0x24270
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24274
    c.b(C::Hi,0x2429c)?;
    // 0x24278
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x2427c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x24280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24284
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24288
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,3)?)?;
    // 0x2428c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x24290
    c.b(C::Cc,0x2429c)?;
    // 0x24294
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x24298
    c.b(C::Ls,0x242c8)?;
    // 0x2429c
    c.literal(R::R1,0x24c34)?;
    // 0x242a0
    c.literal(R::R2,0x24c24)?;
    // 0x242a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,137)?)?;
    // 0x242a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x242ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x242b0
    c.bl(0x2ec94)?;
    // 0x242b4
    c.literal(R::R1,0x24c28)?;
    // 0x242b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x242bc
    c.bl(0x2ec94)?;
    // 0x242c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3)?)?;
    // 0x242c4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x242c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // 0x242cc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x242d0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x242d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x242d8
    c.b(C::Eq,0x243f0)?;
    // 0x242dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // 0x242e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x242e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,44)?)?;
    // 0x242e8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x242ec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32768)?)?;
    // 0x242f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x242f4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x242f8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x242fc
    c.b(C::Eq,0x24334)?;
    // 0x24300
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x24304
    c.literal(R::R1,0x24c38)?;
    // 0x24308
    c.literal(R::R2,0x24c24)?;
    // 0x2430c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,145)?)?;
    // 0x24310
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24314
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24318
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2431c
    c.bl(0x2ec94)?;
    // 0x24320
    c.literal(R::R1,0x24c28)?;
    // 0x24324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24328
    c.bl(0x2ec94)?;
    // 0x2432c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24330
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24334
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(9)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x24338
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4160)?)?;
    // 0x2433c
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // 0x24340
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24344
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,-4)?)?;
    // 0x24348
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x2434c
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x24350
    c.b(C::Ls,0x24390)?;
    // 0x24354
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x24358
    c.literal(R::R1,0x24c30)?;
    // 0x2435c
    c.literal(R::R2,0x24c24)?;
    // 0x24360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,150)?)?;
    // 0x24364
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24368
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x2436c
    c.bl(0x2ec94)?;
    // 0x24370
    c.literal(R::R1,0x24c28)?;
    // 0x24374
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24378
    c.bl(0x2ec94)?;
    // 0x2437c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,60)?)?;
    // 0x24380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24388
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x2438c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24390
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    // 0x24394
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsl,3)?)?;
    // 0x24398
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2439c
    c.b(C::Cc,0x243c4)?;
    // 0x243a0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x243a4
    c.b(C::Hi,0x243c4)?;
    // 0x243a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x243ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x243b0
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,3)?)?;
    // 0x243b4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x243b8
    c.b(C::Cc,0x243c4)?;
    // 0x243bc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x243c0
    c.b(C::Ls,0x243f0)?;
    // 0x243c4
    c.literal(R::R1,0x24c3c)?;
    // 0x243c8
    c.literal(R::R2,0x24c24)?;
    // 0x243cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,412)?)?;
    // 0x243d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x243d4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x243d8
    c.bl(0x2ec94)?;
    // 0x243dc
    c.literal(R::R1,0x24c28)?;
    // 0x243e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x243e4
    c.bl(0x2ec94)?;
    // 0x243e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3)?)?;
    // 0x243ec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x243f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // 0x243f4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x243f8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x243fc
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24400
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x24404
    c.b(C::Ne,0x24440)?;
    // 0x24408
    c.literal(R::R1,0x24c40)?;
    // 0x2440c
    c.literal(R::R2,0x24c24)?;
    // 0x24410
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,420)?)?;
    // 0x24414
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x24418
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2441c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x24420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24424
    c.bl(0x2ec94)?;
    // 0x24428
    c.literal(R::R1,0x24c28)?;
    // 0x2442c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24430
    c.bl(0x2ec94)?;
    // 0x24434
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x24438
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x2443c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24440
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x24444
    c.b(C::Ne,0x23d18)?;
    // 0x24448
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,84)?)?;
    // 0x2444c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24450
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24454
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,2)?)?;
    // 0x24458
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,28)?)?;
    // 0x2445c
    c.b(C::Hi,0x244b8)?;
    // 0x24460
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,1,true)?)?;
    // 0x24464
    c.b(C::Ne,0x244b8)?;
    // 0x24468
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,92)?)?;
    // 0x2446c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24470
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24474
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x24478
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x2447c
    c.b(C::Hi,0x244b8)?;
    // 0x24480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // 0x24484
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24488
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x2448c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,2)?)?;
    // 0x24490
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,28)?)?;
    // 0x24494
    c.b(C::Hi,0x244b8)?;
    // 0x24498
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,1,true)?)?;
    // 0x2449c
    c.b(C::Ne,0x244b8)?;
    // 0x244a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,96)?)?;
    // 0x244a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x244a8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x244ac
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x244b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x244b4
    c.b(C::Cc,0x23d18)?;
    // 0x244b8
    c.literal(R::R1,0x24c44)?;
    // 0x244bc
    c.literal(R::R2,0x24c24)?;
    // 0x244c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,428)?)?;
    // 0x244c4
    c.b(C::Al,0x248fc)?;
    c.check(0x244c8)
}

/// Decode requested codec profile and clamp/diagnose buffer geometry.
fn validate_codec_profile(c:&mut Code)->Result<(),String>{
    c.check(0x244c8)?;
    // 0x244c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,236)?)?;
    // 0x244cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x244d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x244d4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x244d8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x244dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x244e0
    c.b(C::Cc,0x24524)?;
    // 0x244e4
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,32768)?)?;
    // 0x244e8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x244ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x244f0
    c.b(C::Cc,0x24524)?;
    // 0x244f4
    c.literal(R::R1,0x24c48)?;
    // 0x244f8
    c.literal(R::R2,0x24c24)?;
    // 0x244fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,356)?)?;
    // 0x24500
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24508
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x2450c
    c.bl(0x2ec94)?;
    // 0x24510
    c.literal(R::R1,0x24c28)?;
    // 0x24514
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24518
    c.bl(0x2ec94)?;
    // 0x2451c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24520
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24524
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,272)?)?;
    // 0x24528
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,276)?)?;
    // 0x2452c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x24530
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24534
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x24538
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x2453c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x24540
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x24544
    c.b(C::Ls,0x2457c)?;
    // 0x24548
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x2454c
    c.literal(R::R1,0x24c4c)?;
    // 0x24550
    c.literal(R::R2,0x24c24)?;
    // 0x24554
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,105)?)?;
    // 0x24558
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2455c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24560
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24564
    c.bl(0x2ec94)?;
    // 0x24568
    c.literal(R::R1,0x24c28)?;
    // 0x2456c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24570
    c.bl(0x2ec94)?;
    // 0x24574
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24578
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x2457c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,300)?)?;
    // 0x24580
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24584
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24588
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2458c
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x24590
    c.b(C::Eq,0x23d18)?;
    // 0x24594
    c.literal(R::R1,0x24c50)?;
    // 0x24598
    c.literal(R::R2,0x24c24)?;
    // 0x2459c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,110)?)?;
    // 0x245a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x245a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x245a8
    c.b(C::Al,0x24900)?;
    // 0x245ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,132)?)?;
    // 0x245b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x245b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x245b8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x245bc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x245c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x245c4
    c.b(C::Cc,0x24608)?;
    // 0x245c8
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,32768)?)?;
    // 0x245cc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x245d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x245d4
    c.b(C::Cc,0x24608)?;
    // 0x245d8
    c.literal(R::R1,0x24c54)?;
    // 0x245dc
    c.literal(R::R2,0x24c24)?;
    // 0x245e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,308)?)?;
    // 0x245e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x245e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x245ec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x245f0
    c.bl(0x2ec94)?;
    // 0x245f4
    c.literal(R::R1,0x24c28)?;
    // 0x245f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x245fc
    c.bl(0x2ec94)?;
    // 0x24600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24604
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,168)?)?;
    // 0x2460c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,172)?)?;
    // 0x24610
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x24614
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24618
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x2461c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24620
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x24624
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x24628
    c.b(C::Ls,0x24660)?;
    // 0x2462c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x24630
    c.literal(R::R1,0x24c58)?;
    // 0x24634
    c.literal(R::R2,0x24c24)?;
    // 0x24638
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,57)?)?;
    // 0x2463c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x24640
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24644
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24648
    c.bl(0x2ec94)?;
    // 0x2464c
    c.literal(R::R1,0x24c28)?;
    // 0x24650
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24654
    c.bl(0x2ec94)?;
    // 0x24658
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x2465c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24660
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,180)?)?;
    // 0x24664
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24668
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x2466c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,101)?)?;
    // 0x24670
    c.b(C::Cc,0x246a4)?;
    // 0x24674
    c.literal(R::R1,0x24c5c)?;
    // 0x24678
    c.literal(R::R2,0x24c24)?;
    // 0x2467c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,62)?)?;
    // 0x24680
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24688
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x2468c
    c.bl(0x2ec94)?;
    // 0x24690
    c.literal(R::R1,0x24c28)?;
    // 0x24694
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24698
    c.bl(0x2ec94)?;
    // 0x2469c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x246a0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x246a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,184)?)?;
    // 0x246a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x246ac
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x246b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,101)?)?;
    // 0x246b4
    c.b(C::Cc,0x246e8)?;
    // 0x246b8
    c.literal(R::R1,0x24c60)?;
    // 0x246bc
    c.literal(R::R2,0x24c24)?;
    // 0x246c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,67)?)?;
    // 0x246c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x246c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x246cc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x246d0
    c.bl(0x2ec94)?;
    // 0x246d4
    c.literal(R::R1,0x24c28)?;
    // 0x246d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x246dc
    c.bl(0x2ec94)?;
    // 0x246e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x246e4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x246e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,188)?)?;
    // 0x246ec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x246f0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x246f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x246f8
    c.b(C::Cc,0x2472c)?;
    // 0x246fc
    c.literal(R::R1,0x24c64)?;
    // 0x24700
    c.literal(R::R2,0x24c24)?;
    // 0x24704
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,328)?)?;
    // 0x24708
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2470c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24710
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x24714
    c.bl(0x2ec94)?;
    // 0x24718
    c.literal(R::R1,0x24c28)?;
    // 0x2471c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24720
    c.bl(0x2ec94)?;
    // 0x24724
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24728
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x2472c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4288)?)?;
    // 0x24730
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x24734
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24738
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,4)?)?;
    // 0x2473c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,61)?)?;
    // 0x24740
    c.b(C::Cc,0x2477c)?;
    // 0x24744
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R0,3,true)?)?;
    // 0x24748
    c.b(C::Eq,0x2477c)?;
    // 0x2474c
    c.literal(R::R1,0x24c68)?;
    // 0x24750
    c.literal(R::R2,0x24c24)?;
    // 0x24754
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24758
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2475c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2384)?)?;
    // 0x24760
    c.bl(0x2ec94)?;
    // 0x24764
    c.literal(R::R1,0x24c28)?;
    // 0x24768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2476c
    c.bl(0x2ec94)?;
    // 0x24770
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x24774
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24778
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x2477c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,196)?)?;
    // 0x24780
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,254)?)?;
    // 0x24784
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x24788
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x2478c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x24790
    c.b(C::Ls,0x23d18)?;
    // 0x24794
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x24798
    c.literal(R::R1,0x24c6c)?;
    // 0x2479c
    c.literal(R::R2,0x24c24)?;
    // 0x247a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,85)?)?;
    // 0x247a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x247a8
    c.b(C::Al,0x24900)?;
    // 0x247ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,496)?)?;
    // 0x247b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x247b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x247b8
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x247bc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x247c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x247c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x247c8
    c.b(C::Cc,0x247e0)?;
    // 0x247cc
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,32768)?)?;
    // 0x247d0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x247d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x247d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x247dc
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x247e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,476)?)?;
    // 0x247e4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // 0x247e8
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,32768)?)?;
    // 0x247ec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x247f0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(6)?,R::R2,Shift::Lsl,0)?)?;
    // 0x247f4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x247f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x247fc
    c.b(C::Cc,0x24808)?;
    // 0x24800
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x24804
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x24808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2480c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x24810
    c.b(C::Ne,0x24844)?;
    // 0x24814
    c.literal(R::R1,0x24ca4)?;
    // 0x24818
    c.literal(R::R2,0x24c24)?;
    // 0x2481c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,18)?)?;
    // 0x24820
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24828
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x2482c
    c.bl(0x2ec94)?;
    // 0x24830
    c.literal(R::R1,0x24c28)?;
    // 0x24834
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24838
    c.bl(0x2ec94)?;
    // 0x2483c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24844
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x24848
    c.b(C::Ne,0x24880)?;
    // 0x2484c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x24850
    c.literal(R::R1,0x24ca8)?;
    // 0x24854
    c.literal(R::R2,0x24c24)?;
    // 0x24858
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,23)?)?;
    // 0x2485c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x24860
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24868
    c.bl(0x2ec94)?;
    // 0x2486c
    c.literal(R::R1,0x24c28)?;
    // 0x24870
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24874
    c.bl(0x2ec94)?;
    // 0x24878
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x2487c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24880
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,532)?)?;
    // 0x24884
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,536)?)?;
    // 0x24888
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x2488c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24890
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x24894
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24898
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x2489c
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x248a0
    c.b(C::Ls,0x248d8)?;
    // 0x248a4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x248a8
    c.literal(R::R1,0x24cac)?;
    // 0x248ac
    c.literal(R::R2,0x24c24)?;
    // 0x248b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,29)?)?;
    // 0x248b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x248b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x248bc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x248c0
    c.bl(0x2ec94)?;
    // 0x248c4
    c.literal(R::R1,0x24c28)?;
    // 0x248c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x248cc
    c.bl(0x2ec94)?;
    // 0x248d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x248d4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x248d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,540)?)?;
    // 0x248dc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x248e0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x248e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x248e8
    c.b(C::Cc,0x23d18)?;
    // 0x248ec
    c.literal(R::R1,0x24cb0)?;
    // 0x248f0
    c.literal(R::R2,0x24c24)?;
    // 0x248f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x248f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,35)?)?;
    // 0x248fc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x24900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24904
    c.bl(0x2ec94)?;
    // 0x24908
    c.literal(R::R1,0x24c28)?;
    // 0x2490c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24910
    c.bl(0x2ec94)?;
    // 0x24914
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24918
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x2491c
    c.b(C::Al,0x23d18)?;
    c.check(0x24920)
}

/// Check global constraints and finish with independently classified pool.
fn finish_profile_validation(c:&mut Code)->Result<(),String>{
    c.check(0x24920)?;
    // 0x24920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,276)?)?;
    // 0x24924
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,32768)?)?;
    // 0x24928
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x2492c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24930
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24934
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x24938
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x2493c
    c.emit(arm32::data_immediate(C::Cs,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x24940
    c.b(C::Hi,0x24a8c)?;
    // 0x24944
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,728)?)?;
    // 0x24948
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x2494c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x24950
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // 0x24954
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(6)?,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24958
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,296)?)?;
    // 0x2495c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,300)?)?;
    // 0x24960
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,32768)?)?;
    // 0x24964
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x24968
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x2496c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x24970
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x24974
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24978
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x2497c
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x24980
    c.b(C::Ls,0x249b8)?;
    // 0x24984
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x24988
    c.literal(R::R1,0x24cb8)?;
    // 0x2498c
    c.literal(R::R2,0x24c24)?;
    // 0x24990
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,169)?)?;
    // 0x24994
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x24998
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2499c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // 0x249a0
    c.bl(0x2ec94)?;
    // 0x249a4
    c.literal(R::R1,0x24c28)?;
    // 0x249a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x249ac
    c.bl(0x2ec94)?;
    // 0x249b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x249b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x249b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x249bc
    c.b(C::Cc,0x249f4)?;
    // 0x249c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x249c4
    c.literal(R::R1,0x24cbc)?;
    // 0x249c8
    c.literal(R::R2,0x24c24)?;
    // 0x249cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,174)?)?;
    // 0x249d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // 0x249d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x249d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x249dc
    c.bl(0x2ec94)?;
    // 0x249e0
    c.literal(R::R1,0x24c28)?;
    // 0x249e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x249e8
    c.bl(0x2ec94)?;
    // 0x249ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x249f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x249f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1328)?)?;
    // 0x249f8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // 0x249fc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24a00
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24a04
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // 0x24a08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // 0x24a0c
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x24a10
    c.b(C::Cc,0x24a4c)?;
    // 0x24a14
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x24a18
    c.literal(R::R1,0x24cc4)?;
    // 0x24a1c
    c.literal(R::R2,0x24c24)?;
    // 0x24a20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,948)?)?;
    // 0x24a24
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x24a28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x24a2c
    c.b(C::Al,0x24aec)?;
    // 0x24a30
    c.literal(R::R2,0x24cd0)?;
    // 0x24a34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,182)?)?;
    // 0x24a38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x24a3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x24a40
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    // 0x24a44
    c.bl(0x17b0)?;
    // 0x24a48
    c.b(C::Al,0x23d18)?;
    // 0x24a4c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x24a50
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x24a54
    c.b(C::Ne,0x24a6c)?;
    // 0x24a58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,732)?)?;
    // 0x24a5c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x24a60
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(9)?,R::R2,Shift::Lsl,0)?)?;
    // 0x24a64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x24a68
    c.b(C::Eq,0x24ad0)?;
    // 0x24a6c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x24a70
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x24a74
    c.b(C::Ne,0x24b08)?;
    // 0x24a78
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x24a7c
    c.literal(R::R1,0x24cc0)?;
    // 0x24a80
    c.literal(R::R2,0x24c24)?;
    // 0x24a84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,190)?)?;
    // 0x24a88
    c.b(C::Al,0x24ae4)?;
    // 0x24a8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,728)?)?;
    // 0x24a90
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x24a94
    c.literal(R::R2,0x24c24)?;
    // 0x24a98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,932)?)?;
    // 0x24a9c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x24aa0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x24aa4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(6)?,R::new(5)?,R::R1,Shift::Lsl,0)?)?;
    // 0x24aa8
    c.literal(R::R1,0x24cb4)?;
    // 0x24aac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24ab0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24ab4
    c.bl(0x2ec94)?;
    // 0x24ab8
    c.literal(R::R1,0x24c28)?;
    // 0x24abc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24ac0
    c.bl(0x2ec94)?;
    // 0x24ac4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24ac8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24acc
    c.b(C::Al,0x24958)?;
    // 0x24ad0
    c.literal(R::R1,0x24cc0)?;
    // 0x24ad4
    c.literal(R::R2,0x24c24)?;
    // 0x24ad8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x24adc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,185)?)?;
    // 0x24ae0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24ae4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x24ae8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // 0x24aec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24af0
    c.bl(0x2ec94)?;
    // 0x24af4
    c.literal(R::R1,0x24c28)?;
    // 0x24af8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24afc
    c.bl(0x2ec94)?;
    // 0x24b00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24b04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24b08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2800)?)?;
    // 0x24b0c
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x24b10
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x24b14
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x24b18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x24b1c
    c.b(C::Ne,0x24b54)?;
    // 0x24b20
    c.literal(R::R1,0x24cc8)?;
    // 0x24b24
    c.literal(R::R2,0x24c24)?;
    // 0x24b28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,964)?)?;
    // 0x24b2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x24b30
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x24b34
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x24b38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24b3c
    c.bl(0x2ec94)?;
    // 0x24b40
    c.literal(R::R1,0x24c28)?;
    // 0x24b44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24b48
    c.bl(0x2ec94)?;
    // 0x24b4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x24b50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x24b54
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x24b58
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x24b5c
    c.b(C::Eq,0x23d18)?;
    // 0x24b60
    c.literal(R::new(7)?,0x24c28)?;
    // 0x24b64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,776)?)?;
    // 0x24b68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,5)?)?;
    // 0x24b6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x24b70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R1,6144)?)?;
    // 0x24b74
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,8192)?)?;
    // 0x24b78
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,-44)?)?;
    // 0x24b7c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x24b80
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,0)?)?;
    // 0x24b84
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x24b88
    c.b(C::Ne,0x24c08)?;
    // 0x24b8c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,-20)?)?;
    // 0x24b90
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,2)?)?;
    // 0x24b94
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,28)?)?;
    // 0x24b98
    c.b(C::Hi,0x24bdc)?;
    // 0x24b9c
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,1,true)?)?;
    // 0x24ba0
    c.b(C::Ne,0x24bdc)?;
    // 0x24ba4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,20)?)?;
    // 0x24ba8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x24bac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // 0x24bb0
    c.b(C::Hi,0x24bdc)?;
    // 0x24bb4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x24bb8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,2)?)?;
    // 0x24bbc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,28)?)?;
    // 0x24bc0
    c.b(C::Hi,0x24bdc)?;
    // 0x24bc4
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,1,true)?)?;
    // 0x24bc8
    c.b(C::Ne,0x24bdc)?;
    // 0x24bcc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,40)?)?;
    // 0x24bd0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x24bd4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // 0x24bd8
    c.b(C::Cc,0x24c08)?;
    // 0x24bdc
    c.literal(R::R1,0x24ccc)?;
    // 0x24be0
    c.literal(R::R2,0x24c24)?;
    // 0x24be4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,980)?)?;
    // 0x24be8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24bec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x24bf0
    c.bl(0x2ec94)?;
    // 0x24bf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24bf8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x24bfc
    c.bl(0x2ec94)?;
    // 0x24c00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x24c04
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(10)?))?;
    // 0x24c08
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x24c0c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    // 0x24c10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x24c14
    c.b(C::Cc,0x24b78)?;
    // 0x24c18
    c.b(C::Al,0x23d18)?;
    c.check(0x24c1c)?;
    for word in VALIDATION_LOG_WORDS {c.emit(word)?;}
    c.check(0x24cd4)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    dispatch_profile(c)?;
    validate_frame_dimensions(c)?;
    validate_codec_sizes(c)?;
    validate_codec_profile(c)?;
    finish_profile_validation(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_profile_dispatch_uses_19_targets(){
        assert_eq!(VALIDATION_MODE_TARGETS.len(),19);
        assert_eq!(VALIDATION_LOG_WORDS.len(),46);
        let mut c=Code::new(0x23cf0);
        emit(&mut c).unwrap();
        assert_eq!(c.at(),0x24cd4);
        assert_eq!(c.bytes.len(),0xfe4);
        for (i,addr) in VALIDATION_MODE_TARGETS.iter().enumerate(){
            let off=0x23d44-0x23cf0+4*i;
            assert_eq!(u32::from_le_bytes(c.bytes[off..off+4].try_into().unwrap()),*addr);
        }
    }
}
