// SPDX-License-Identifier: GPL-3.0-only
//! Two adjacent Venus descriptor-init routines: normalize per-stream
//! flags, generate format-specific dimensions and backing-buffer counts.
//! Eleven literal words are separately identified non-executable data.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

pub(super) fn emit_next(c:&mut Code)->Result<(),String>{
    c.check(0x22040)?;
    // 0x22040
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x22044
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x22048
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x2204c
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x22050
    c.bl(0x21ed8)?;
    // 0x22054
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x22058
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x2205c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x22060
    c.b(C::Al,0x21f8c)?;
    // 0x22064
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x22068
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x2206c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x22070
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x22074
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x22078
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2207c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,60)?)?;
    // 0x22080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x22084
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,12)?)?;
    // 0x22088
    c.bl(0x31ec)?;
    // 0x2208c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,2)?)?;
    // 0x22090
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x22094
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x22098
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(6)?,1)?)?;
    // 0x2209c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x220a0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,0)?)?;
    // 0x220a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,49152)?)?;
    // 0x220a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,32768)?)?;
    // 0x220ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32768)?)?;
    // 0x220b0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x220b4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x220b8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x220bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x220c0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x220c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // 0x220c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x220cc
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x220d0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x220d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x220d8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x220dc
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x220e0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x220e4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x220e8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x220ec
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x220f0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,8)?)?;
    // 0x220f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x220f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x220fc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16384)?)?;
    // 0x22100
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x22104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x22108
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2210c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x22110
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x22114
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // 0x22118
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x2211c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // 0x22120
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x22124
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x22128
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,32768)?)?;
    // 0x2212c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,56)?)?;
    // 0x22130
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,16)?)?;
    // 0x22134
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x22138
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x2213c
    c.b(C::Cc,0x22148)?;
    // 0x22140
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x22144
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x22148
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,64)?)?;
    // 0x2214c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,24)?)?;
    // 0x22150
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x22154
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,48)?)?;
    // 0x22158
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x2215c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // 0x22160
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x22164)?;
    // 0x22164
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x22168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2216c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x22170
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x22174
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // 0x22178
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x2217c
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x22180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x22184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x22188
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x2218c
    c.bl(0x31ec)?;
    // 0x22190
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,0)?)?;
    // 0x22194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x22198
    c.literal(R::R0,0x22630)?;
    // 0x2219c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16384)?)?;
    // 0x221a0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x221a4
    c.b(C::Gt,0x2222c)?;
    // 0x221a8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,2)?)?;
    // 0x221ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x221b0
    c.b(C::Cc,0x22388)?;
    // 0x221b4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,4)?)?;
    // 0x221b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x221bc
    c.b(C::Cs,0x223cc)?;
    // 0x221c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,127)?)?;
    // 0x221c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,31)?)?;
    // 0x221c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31)?)?;
    // 0x221cc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,127)?)?;
    // 0x221d0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,31)?)?;
    // 0x221d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // 0x221d8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,44)?)?;
    // 0x221dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // 0x221e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // 0x221e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x221e8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsr,1,false)?)?;
    // 0x221ec
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,31)?)?;
    // 0x221f0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,60)?)?;
    // 0x221f4
    c.literal(R::R1,0x22644)?;
    // 0x221f8
    c.bl(0x2ec94)?;
    // 0x221fc
    c.literal(R::R1,0x22648)?;
    // 0x22200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x22204
    c.bl(0x2ec94)?;
    // 0x22208
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x2220c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,44)?)?;
    // 0x22210
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,56)?)?;
    // 0x22214
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,60)?)?;
    // 0x22218
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2221c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x22220
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,4)?)?;
    // 0x22224
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,8)?)?;
    // 0x22228
    c.b(C::Al,0x22604)?;
    // 0x2222c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x22230
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32768)?)?;
    // 0x22234
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x22238
    c.b(C::Le,0x222f8)?;
    // 0x2223c
    c.literal(R::R1,0x22634)?;
    // 0x22240
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x22244
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x22248
    c.b(C::Cs,0x22420)?;
    // 0x2224c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,127)?)?;
    // 0x22250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,31)?)?;
    // 0x22254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63)?)?;
    // 0x22258
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::R1,127)?)?;
    // 0x2225c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,31)?)?;
    // 0x22260
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(8)?,R::R1,31)?)?;
    // 0x22264
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // 0x22268
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,7)?)?;
    // 0x2226c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,40)?)?;
    // 0x22270
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::LR,R::R1,Shift::Lsr,1,false)?)?;
    // 0x22274
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::LR,R::new(5)?,Shift::Lsr,3,false)?)?;
    // 0x22278
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x2227c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,44)?)?;
    // 0x22280
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,56)?)?;
    // 0x22284
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R12,R::R1,31)?)?;
    // 0x22288
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,31)?)?;
    // 0x2228c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,15)?)?;
    // 0x22290
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,3221225503)?)?;
    // 0x22294
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,3840)?)?;
    // 0x22298
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsr,5,false)?)?;
    // 0x2229c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(6)?,Shift::Lsr,4,false)?)?;
    // 0x222a0
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,60)?)?;
    // 0x222a4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x222a8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,268435456)?)?;
    // 0x222ac
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x222b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x222b4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,48)?)?;
    // 0x222b8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,52)?)?;
    // 0x222bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,64)?)?;
    // 0x222c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,68)?)?;
    // 0x222c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x222c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x222cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x222d0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::new(6)?,R::new(5)?,R::LR)?)?;
    // 0x222d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // 0x222d8
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(5)?,R::R1,R::LR)?)?;
    // 0x222dc
    c.literal(R::R1,0x22640)?;
    // 0x222e0
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x222e4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x222e8
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(8)?,R::new(7)?)?)?;
    // 0x222ec
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R12,R::new(7)?)?)?;
    // 0x222f0
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(8)?,R::new(7)?,R::R2)?)?;
    // 0x222f4
    c.b(C::Al,0x22618)?;
    // 0x222f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x222fc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16384)?)?;
    // 0x22300
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x22304
    c.b(C::Eq,0x22524)?;
    // 0x22308
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2230c
    c.b(C::Ne,0x22574)?;
    // 0x22310
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,127)?)?;
    // 0x22314
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63)?)?;
    // 0x22318
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::R1,127)?)?;
    // 0x2231c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,31)?)?;
    // 0x22320
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::R1,31)?)?;
    // 0x22324
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,31)?)?;
    // 0x22328
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,40)?)?;
    // 0x2232c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsr,5,false)?)?;
    // 0x22330
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31)?)?;
    // 0x22334
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,44)?)?;
    // 0x22338
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2233c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,7)?)?;
    // 0x22340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x22344
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsr,3,false)?)?;
    // 0x22348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2234c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,48)?)?;
    // 0x22350
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,3221225503)?)?;
    // 0x22354
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,52)?)?;
    // 0x22358
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,4)?)?;
    // 0x2235c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,8)?)?;
    // 0x22360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x22364
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,28)?)?;
    // 0x22368
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3840)?)?;
    // 0x2236c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::R0,R::R1,R::R2)?)?;
    // 0x22370
    c.literal(R::R0,0x22640)?;
    // 0x22374
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(7)?,R::new(3)?)?)?;
    // 0x22378
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2237c
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(7)?,R::new(3)?,R::R2)?)?;
    // 0x22380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x22384
    c.b(C::Al,0x22618)?;
    // 0x22388
    c.literal(R::R0,0x2264c)?;
    // 0x2238c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // 0x22390
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x22394
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x22398
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x2239c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,255)?)?;
    // 0x223a0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,256)?)?;
    // 0x223a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x223a8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // 0x223ac
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x223b0
    c.b(C::Eq,0x225a0)?;
    // 0x223b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x223b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x223bc
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x223c0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x223c4
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::R1)?)?;
    // 0x223c8
    c.b(C::Al,0x225a8)?;
    // 0x223cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // 0x223d0
    c.b(C::Ne,0x22574)?;
    // 0x223d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,127)?)?;
    // 0x223d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,31)?)?;
    // 0x223dc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,127)?)?;
    // 0x223e0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,31)?)?;
    // 0x223e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // 0x223e8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,44)?)?;
    // 0x223ec
    c.literal(R::R1,0x22654)?;
    // 0x223f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x223f4
    c.bl(0x2ec94)?;
    // 0x223f8
    c.literal(R::R1,0x22648)?;
    // 0x223fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x22400
    c.bl(0x2ec94)?;
    // 0x22404
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x22408
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,44)?)?;
    // 0x2240c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x22410
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x22414
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,4)?)?;
    // 0x22418
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,8)?)?;
    // 0x2241c
    c.b(C::Al,0x22608)?;
    // 0x22420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x22424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,49152)?)?;
    // 0x22428
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2242c
    c.b(C::Ne,0x22574)?;
    // 0x22430
    c.literal(R::LR,0x22638)?;
    // 0x22434
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,191)?)?;
    // 0x22438
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(5)?,1)?)?;
    // 0x2243c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,15)?)?;
    // 0x22440
    c.emit(super::encode_long_multiply(C::Al,false,R::new(3)?,R::new(7)?,R::R1,R::LR)?)?;
    // 0x22444
    c.literal(R::new(3)?,0x2263c)?;
    // 0x22448
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(7)?,Shift::Lsr,7)?)?;
    // 0x2244c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x22450
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,8)?)?;
    // 0x22454
    c.emit(super::encode_long_multiply(C::Al,true,R::new(7)?,R::R2,R::R1,R::new(3)?)?)?;
    // 0x22458
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,47)?)?;
    // 0x2245c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R2,Shift::Lsr,31,false)?)?;
    // 0x22460
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,255)?)?;
    // 0x22464
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::R1,255)?)?;
    // 0x22468
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,15)?)?;
    // 0x2246c
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R12,R::R1,15)?)?;
    // 0x22470
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(8)?,R::new(10)?,Shift::Lsr,1,false)?)?;
    // 0x22474
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,40)?)?;
    // 0x22478
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(9)?,R::R1,15)?)?;
    // 0x2247c
    c.emit(super::encode_long_multiply(C::Al,false,R::R1,R::R2,R::new(3)?,R::LR)?)?;
    // 0x22480
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(5)?,3)?)?;
    // 0x22484
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,1)?)?;
    // 0x22488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,23)?)?;
    // 0x2248c
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,44)?)?;
    // 0x22490
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,56)?)?;
    // 0x22494
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // 0x22498
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(5)?,Shift::Lsr,1,false)?)?;
    // 0x2249c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(8)?,R::new(3)?,Shift::Lsr,2,false)?)?;
    // 0x224a0
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,60)?)?;
    // 0x224a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R2,Shift::Lsr,5,false)?)?;
    // 0x224a8
    c.emit(super::encode_long_multiply(C::Al,false,R::new(6)?,R::R1,R::new(5)?,R::LR)?)?;
    // 0x224ac
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,2147483663)?)?;
    // 0x224b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x224b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,63)?)?;
    // 0x224b8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x224bc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,268435456)?)?;
    // 0x224c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,3840)?)?;
    // 0x224c4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::R1,Shift::Lsr,4,false)?)?;
    // 0x224c8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,48)?)?;
    // 0x224cc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,52)?)?;
    // 0x224d0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x224d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x224d8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(10)?,Shift::Lsr,1,false)?)?;
    // 0x224dc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,64)?)?;
    // 0x224e0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsr,2,false)?)?;
    // 0x224e4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::R0,2147483663)?)?;
    // 0x224e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x224ec
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,68)?)?;
    // 0x224f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x224f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x224f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x224fc
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::R1,R::new(6)?,R::LR)?)?;
    // 0x22500
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R12,R::new(7)?)?)?;
    // 0x22504
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // 0x22508
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(3)?,R::R2,R::LR)?)?;
    // 0x2250c
    c.literal(R::new(3)?,0x22640)?;
    // 0x22510
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x22514
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x22518
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::new(9)?,R::new(7)?)?)?;
    // 0x2251c
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R12,R::new(7)?,R::R2)?)?;
    // 0x22520
    c.b(C::Al,0x22618)?;
    // 0x22524
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,31)?)?;
    // 0x22528
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x2252c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15)?)?;
    // 0x22530
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R1,31)?)?;
    // 0x22534
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // 0x22538
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x2253c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x22540
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsr,1,false)?)?;
    // 0x22544
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x22548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x2254c
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,15)?)?;
    // 0x22550
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // 0x22554
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,44)?)?;
    // 0x22558
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // 0x2255c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,60)?)?;
    // 0x22560
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,4)?)?;
    // 0x22564
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,8)?)?;
    // 0x22568
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,28)?)?;
    // 0x2256c
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R1)?)?;
    // 0x22570
    c.b(C::Al,0x2260c)?;
    // 0x22574
    c.literal(R::R2,0x22658)?;
    // 0x22578
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,243)?)?;
    // 0x2257c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x22580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x22584
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x22588
    c.bl(0x17b0)?;
    // 0x2258c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x22590
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x22594
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x22598
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2259c
    c.b(C::Al,0x22618)?;
    // 0x225a0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x225a4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x225a8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,31)?)?;
    // 0x225ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // 0x225b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31)?)?;
    // 0x225b4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,31)?)?;
    // 0x225b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,44)?)?;
    // 0x225bc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // 0x225c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // 0x225c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x225c8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsr,1,false)?)?;
    // 0x225cc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,31)?)?;
    // 0x225d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,60)?)?;
    // 0x225d4
    c.literal(R::R1,0x22650)?;
    // 0x225d8
    c.bl(0x2ec94)?;
    // 0x225dc
    c.literal(R::R1,0x22648)?;
    // 0x225e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x225e4
    c.bl(0x2ec94)?;
    // 0x225e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x225ec
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,44)?)?;
    // 0x225f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,56)?)?;
    // 0x225f4
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,60)?)?;
    // 0x225f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x225fc
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,4)?)?;
    // 0x22600
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,8)?)?;
    // 0x22604
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::new(7)?,R::R1)?)?;
    // 0x22608
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,28)?)?;
    // 0x2260c
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R2,R::R0)?)?;
    // 0x22610
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x22614
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x22618
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,12)?)?;
    // 0x2261c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,16)?)?;
    // 0x22620
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,20)?)?;
    // 0x22624
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,24)?)?;
    // 0x22628
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2262c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x22630)?;
    const STRIDE_AND_RESOURCE_LITERALS:[u32;11]=[
        0x0fff_ffc0,0xffff_7ffe,0xaaaa_aaab,0x5555_5556,
        0xffff_f000,0x000e_6c3e,0x000e_2788,0x0010_df58,
        0x000e_6c19,0x000e_6bee,0x000e_6a43,
    ];
    for word in STRIDE_AND_RESOURCE_LITERALS {c.emit(word)?;}
    c.check(0x2265c)
}
