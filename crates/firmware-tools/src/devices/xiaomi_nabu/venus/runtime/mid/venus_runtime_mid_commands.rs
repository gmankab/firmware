// SPDX-License-Identifier: GPL-3.0-only
//! Codec resource handles and controller command emission. The routine
//! configures input/output buffer format descriptors, reports validation
//! failures and hands work to the platform callback at runtime 0x2a158.
//! Non-instruction immediate data values follow its epilogue.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

pub(super) fn emit_next(c:&mut Code)->Result<(),String>{
    c.check(0x22a44)?;
    // 0x22a44
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x22a48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x22a4c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // 0x22a50
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x22a54
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x22a58
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // 0x22a5c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,540)?)?;
    // 0x22a60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,552)?)?;
    // 0x22a64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22a68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,564)?)?;
    // 0x22a6c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::R1,7168)?)?;
    // 0x22a70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,3600)?)?;
    // 0x22a74
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,7168)?)?;
    // 0x22a78
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R1,4096)?)?;
    // 0x22a7c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x22a80
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(10)?))?;
    // 0x22a84
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22a88
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::new(3)?,7168)?)?;
    // 0x22a8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x22a90
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x22a94
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x22a98
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x22a9c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // 0x22aa0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,917504)?)?;
    // 0x22aa4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22aa8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,3758096384)?)?;
    // 0x22aac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x22ab0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x22ab4
    c.b(C::Eq,0x22b08)?;
    // 0x22ab8
    c.literal(R::R1,0x22f70)?;
    // 0x22abc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22ac0
    c.bl(0x2ec94)?;
    // 0x22ac4
    c.literal(R::R1,0x22f6c)?;
    // 0x22ac8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22acc
    c.bl(0x2ec94)?;
    // 0x22ad0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,285212672)?)?;
    // 0x22ad4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,280)?)?;
    // 0x22ad8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x22adc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x22ae0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x22ae4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x22ae8
    c.b(C::Ne,0x22adc)?;
    // 0x22aec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x22af0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,280)?)?;
    // 0x22af4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x22af8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x22afc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x22b00
    c.b(C::Ne,0x22af4)?;
    // 0x22b04
    c.b(C::Al,0x22b54)?;
    // 0x22b08
    c.literal(R::R1,0x22f68)?;
    // 0x22b0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22b10
    c.bl(0x2ec94)?;
    // 0x22b14
    c.literal(R::R1,0x22f6c)?;
    // 0x22b18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22b1c
    c.bl(0x2ec94)?;
    // 0x22b20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,134217728)?)?;
    // 0x22b24
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,280)?)?;
    // 0x22b28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x22b2c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x22b30
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x22b34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x22b38
    c.b(C::Ne,0x22b2c)?;
    // 0x22b3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x22b40
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,280)?)?;
    // 0x22b44
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x22b48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x22b4c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x22b50
    c.b(C::Ne,0x22b44)?;
    // 0x22b54
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x22b58
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x22b5c
    c.bl(0x227e0)?;
    // 0x22b60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,684)?)?;
    // 0x22b64
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x22b68
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22b6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x22b70
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x22b74
    c.literal(R::R1,0x22f74)?;
    // 0x22b78
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(5)?,R::R1,Shift::Lsl,0)?)?;
    // 0x22b7c
    c.literal(R::new(7)?,0x22f78)?;
    // 0x22b80
    c.b(C::Eq,0x22d18)?;
    // 0x22b84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x22b88
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x22b8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x22b90
    c.b(C::Ne,0x22ba4)?;
    // 0x22b94
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R2,Shift::Lsl,2)?)?;
    // 0x22b98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,908)?)?;
    // 0x22b9c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x22ba0
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x22ba4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,728)?)?;
    // 0x22ba8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x22bac
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x22bb0
    c.literal(R::R1,0x22f7c)?;
    // 0x22bb4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,4,false)?)?;
    // 0x22bb8
    c.literal(R::R1,0x22f98)?;
    // 0x22bbc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x22bc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22bc4
    c.bl(0x2ec94)?;
    // 0x22bc8
    c.literal(R::R1,0x22f6c)?;
    // 0x22bcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22bd0
    c.bl(0x2ec94)?;
    // 0x22bd4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,116)?)?;
    // 0x22bd8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,120)?)?;
    // 0x22bdc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x22be0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x22be4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x22be8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,124)?)?;
    // 0x22bec
    c.bl(0x26754)?;
    // 0x22bf0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,648)?)?;
    // 0x22bf4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(8)?,164)?)?;
    // 0x22bf8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x22bfc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x22c00
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x22c04
    c.bl(0x293c)?;
    // 0x22c08
    c.bl(0x3070)?;
    // 0x22c0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,712)?)?;
    // 0x22c10
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x22c14
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x22c18
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x22c1c
    c.literal(R::new(3)?,0x22f9c)?;
    // 0x22c20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,6)?)?;
    // 0x22c24
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x22c28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x22c2c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // 0x22c30
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R2,4096)?)?;
    // 0x22c34
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x22c38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,708)?)?;
    // 0x22c3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x22c40
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x22c44
    c.bl(0x25a24)?;
    // 0x22c48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22c4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // 0x22c50
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,112)?)?;
    // 0x22c54
    c.bl(0x3074)?;
    // 0x22c58
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x22c5c
    c.bl(0x294c)?;
    // 0x22c60
    c.literal(R::R0,0x22fa0)?;
    // 0x22c64
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x22c68
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x22c6c
    c.b(C::Eq,0x22ebc)?;
    // 0x22c70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,696)?)?;
    // 0x22c74
    c.literal(R::new(3)?,0x22f7c)?;
    // 0x22c78
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x22c7c
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(9)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x22c80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x22c84
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,428)?)?;
    // 0x22c88
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x22c8c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,0)?)?;
    // 0x22c90
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x22c94
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,4,false)?)?;
    // 0x22c98
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,728)?)?;
    // 0x22c9c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22ca0
    c.b(C::Ne,0x22cb4)?;
    // 0x22ca4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R1,Shift::Lsl,2)?)?;
    // 0x22ca8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,908)?)?;
    // 0x22cac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,6144)?)?;
    // 0x22cb0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x22cb4
    c.literal(R::new(8)?,0x22f8c)?;
    // 0x22cb8
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x22cbc
    c.literal(R::R1,0x22fa4)?;
    // 0x22cc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,179)?)?;
    // 0x22cc4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x22cc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x22ccc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x22cd0
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x22cd4
    c.bl(0x2ec94)?;
    // 0x22cd8
    c.literal(R::new(6)?,0x22f6c)?;
    // 0x22cdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x22ce0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x22ce4
    c.bl(0x2ec94)?;
    // 0x22ce8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,136)?)?;
    // 0x22cec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,140)?)?;
    // 0x22cf0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,0)?)?;
    // 0x22cf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,184)?)?;
    // 0x22cf8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x22cfc
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R2,R::R0,Shift::Lsl,2)?)?;
    // 0x22d00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3616)?)?;
    // 0x22d04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x22d08
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x22d0c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,2)?)?;
    // 0x22d10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x22d14
    c.b(C::Al,0x22e94)?;
    // 0x22d18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x22d1c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x22d20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x22d24
    c.b(C::Ne,0x22d38)?;
    // 0x22d28
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R2,Shift::Lsl,2)?)?;
    // 0x22d2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,908)?)?;
    // 0x22d30
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x22d34
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x22d38
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x22d3c
    c.literal(R::R1,0x22f7c)?;
    // 0x22d40
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,4,false)?)?;
    // 0x22d44
    c.literal(R::R1,0x22f80)?;
    // 0x22d48
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x22d4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22d50
    c.bl(0x2ec94)?;
    // 0x22d54
    c.literal(R::R1,0x22f6c)?;
    // 0x22d58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22d5c
    c.bl(0x2ec94)?;
    // 0x22d60
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,84)?)?;
    // 0x22d64
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,88)?)?;
    // 0x22d68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x22d6c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x22d70
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x22d74
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,92)?)?;
    // 0x22d78
    c.bl(0x26754)?;
    // 0x22d7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7808)?)?;
    // 0x22d80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(8)?,156)?)?;
    // 0x22d84
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x22d88
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x22d8c
    c.bl(0x293c)?;
    // 0x22d90
    c.bl(0x3070)?;
    // 0x22d94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,712)?)?;
    // 0x22d98
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x22d9c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x22da0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x22da4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,5)?)?;
    // 0x22da8
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,53)?)?;
    // 0x22dac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // 0x22db0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x22db4
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R2,256)?)?;
    // 0x22db8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x22dbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7872)?)?;
    // 0x22dc0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x22dc4
    c.bl(0x259d4)?;
    // 0x22dc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22dcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x22dd0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,80)?)?;
    // 0x22dd4
    c.bl(0x3074)?;
    // 0x22dd8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x22ddc
    c.bl(0x294c)?;
    // 0x22de0
    c.literal(R::R0,0x22f84)?;
    // 0x22de4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x22de8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x22dec
    c.b(C::Eq,0x22f04)?;
    // 0x22df0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,696)?)?;
    // 0x22df4
    c.literal(R::new(3)?,0x22f7c)?;
    // 0x22df8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x22dfc
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x22e00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x22e04
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,432)?)?;
    // 0x22e08
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x22e0c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,0)?)?;
    // 0x22e10
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x22e14
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,4,false)?)?;
    // 0x22e18
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,728)?)?;
    // 0x22e1c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22e20
    c.b(C::Ne,0x22e34)?;
    // 0x22e24
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R1,Shift::Lsl,2)?)?;
    // 0x22e28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,908)?)?;
    // 0x22e2c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,6144)?)?;
    // 0x22e30
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x22e34
    c.literal(R::new(5)?,0x22f8c)?;
    // 0x22e38
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x22e3c
    c.literal(R::R1,0x22f88)?;
    // 0x22e40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,34)?)?;
    // 0x22e44
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x22e48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x22e4c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x22e50
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x22e54
    c.bl(0x2ec94)?;
    // 0x22e58
    c.literal(R::new(6)?,0x22f6c)?;
    // 0x22e5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x22e60
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x22e64
    c.bl(0x2ec94)?;
    // 0x22e68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,104)?)?;
    // 0x22e6c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,108)?)?;
    // 0x22e70
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,0)?)?;
    // 0x22e74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,39)?)?;
    // 0x22e78
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x22e7c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R2,R::R0,Shift::Lsl,2)?)?;
    // 0x22e80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,532)?)?;
    // 0x22e84
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x22e88
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x22e8c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,2)?)?;
    // 0x22e90
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x22e94
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x22e98
    c.literal(R::R1,0x22f90)?;
    // 0x22e9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x22ea0
    c.bl(0x2ec94)?;
    // 0x22ea4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x22ea8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x22eac
    c.bl(0x2ec94)?;
    // 0x22eb0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x22eb4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x22eb8
    c.b(C::Al,0x2a158)?;
    // 0x22ebc
    c.literal(R::new(9)?,0x22f7c)?;
    // 0x22ec0
    c.bl(0x26754)?;
    // 0x22ec4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,652)?)?;
    // 0x22ec8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x22ecc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x22ed0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x22ed4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x22ed8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,12)?)?;
    // 0x22edc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x22ee0
    c.b(C::Ne,0x22ef4)?;
    // 0x22ee4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R2,Shift::Lsl,2)?)?;
    // 0x22ee8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,908)?)?;
    // 0x22eec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x22ef0
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x22ef4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x22ef8
    c.literal(R::R1,0x22fa8)?;
    // 0x22efc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,4,false)?)?;
    // 0x22f00
    c.b(C::Al,0x22f48)?;
    // 0x22f04
    c.literal(R::new(10)?,0x22f7c)?;
    // 0x22f08
    c.bl(0x26754)?;
    // 0x22f0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,644)?)?;
    // 0x22f10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x22f14
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x22f18
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x22f1c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x22f20
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,12)?)?;
    // 0x22f24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x22f28
    c.b(C::Ne,0x22f3c)?;
    // 0x22f2c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::R2,Shift::Lsl,2)?)?;
    // 0x22f30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,908)?)?;
    // 0x22f34
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x22f38
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x22f3c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x22f40
    c.literal(R::R1,0x22f94)?;
    // 0x22f44
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R0,Shift::Lsl,4,false)?)?;
    // 0x22f48
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x22f4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22f50
    c.bl(0x2ec94)?;
    // 0x22f54
    c.literal(R::R1,0x22f6c)?;
    // 0x22f58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22f5c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x22f60
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x22f64
    c.b(C::Al,0x2ec94)?;
    c.check(0x22f68)?;
    const RESOURCE_AND_DIAGNOSTIC_LITERALS:[u32;17]=[
        0x000e_6cdd,0x000e_2788,0x000e_6cae,0xffff_e160,
        0xe000_8060,0x0010_3f10,0x000e_6e1e,0x0010_409c,
        0x000e_6e90,0x000e_6dd4,0x000e_6de7,0x000e_6e57,
        0x000e_6d0c,0xffff_fcf9,0x0010_40a0,0x000e_6d83,
        0x000e_6d4b,
    ];
    for constant in RESOURCE_AND_DIAGNOSTIC_LITERALS {c.emit(constant)?;}
    c.check(0x22fac)
}
