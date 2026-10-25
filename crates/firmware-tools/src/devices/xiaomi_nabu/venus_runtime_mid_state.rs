// SPDX-License-Identifier: GPL-3.0-only
//! Register command state transitions and controller input setup.
//! Handles MMIO status reads/writes, bounded descriptor loop and error
//! reporting, with eight non-instruction diagnostic/control constants.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

pub(super) fn emit_next(c:&mut Code)->Result<(),String>{
    c.check(0x22fac)?;
    // 0x22fac
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x22fb0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x22fb4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x22fb8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x22fbc
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x22fc0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // 0x22fc4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,3600)?)?;
    // 0x22fc8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,540)?)?;
    // 0x22fcc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x22fd0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,532)?)?;
    // 0x22fd4
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,132)?)?;
    // 0x22fd8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4096)?)?;
    // 0x22fdc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,7168)?)?;
    // 0x22fe0
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R0,7168)?)?;
    // 0x22fe4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,564)?)?;
    // 0x22fe8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x22fec
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R1,0)?)?;
    // 0x22ff0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,552)?)?;
    // 0x22ff4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,7168)?)?;
    // 0x22ff8
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R0,7168)?)?;
    // 0x22ffc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23000
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // 0x23004
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,3616)?)?;
    // 0x23008
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x2300c
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23010
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x23014
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R1,4096)?)?;
    // 0x23018
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,62)?)?;
    // 0x2301c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R2,0)?)?;
    // 0x23020
    c.literal(R::R2,0x2324c)?;
    // 0x23024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x23028
    c.bl(0x17b0)?;
    // 0x2302c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x23030
    c.b(C::Eq,0x2306c)?;
    // 0x23034
    c.literal(R::R1,0x23250)?;
    // 0x23038
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2303c
    c.bl(0x2ec94)?;
    // 0x23040
    c.literal(R::R1,0x23254)?;
    // 0x23044
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23048
    c.bl(0x2ec94)?;
    // 0x2304c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x23050
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x23054
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x23058
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x2305c
    c.bl(0x22a44)?;
    // 0x23060
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23064
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x23068
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2306c
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x23070
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,2)?)?;
    // 0x23074
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x23078
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x2307c
    c.b(C::Eq,0x23060)?;
    // 0x23080
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,584)?)?;
    // 0x23084
    c.literal(R::new(5)?,0x23254)?;
    // 0x23088
    c.literal(R::new(6)?,0x23268)?;
    // 0x2308c
    c.literal(R::new(7)?,0x23264)?;
    // 0x23090
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x23094
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7168)?)?;
    // 0x23098
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2309c
    c.literal(R::new(4)?,0x23258)?;
    // 0x230a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x230a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,35840)?)?;
    // 0x230a8
    c.b(C::Ge,0x23148)?;
    // 0x230ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,33792)?)?;
    // 0x230b0
    c.b(C::Eq,0x231e4)?;
    // 0x230b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1120)?)?;
    // 0x230b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32768)?)?;
    // 0x230bc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x230c0
    c.b(C::Ne,0x23218)?;
    // 0x230c4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    // 0x230c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x230cc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x230d0
    c.bl(0x2ec94)?;
    // 0x230d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x230d8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x230dc
    c.bl(0x2ec94)?;
    // 0x230e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x230e4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x230e8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x230ec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x230f0
    c.b(C::Ne,0x230c4)?;
    // 0x230f4
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x230f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,100)?)?;
    // 0x230fc
    c.literal(R::R2,0x2324c)?;
    // 0x23100
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,33792)?)?;
    // 0x23104
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,102)?)?;
    // 0x2310c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23110
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23114
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x23118
    c.bl(0x17b0)?;
    // 0x2311c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x23120
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // 0x23124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x23128
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x2312c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,224)?)?;
    // 0x23130
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x23134
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32768)?)?;
    // 0x23138
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32)?)?;
    // 0x2313c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23140
    c.b(C::Ne,0x23238)?;
    // 0x23144
    c.b(C::Al,0x23060)?;
    // 0x23148
    c.b(C::Eq,0x231ec)?;
    // 0x2314c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,36096)?)?;
    // 0x23150
    c.b(C::Ne,0x23218)?;
    // 0x23154
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2208)?)?;
    // 0x23158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2315c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x23160
    c.bl(0x2ec94)?;
    // 0x23164
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23168
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2316c
    c.bl(0x2ec94)?;
    // 0x23170
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x23174
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2208)?)?;
    // 0x23178
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2317c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23180
    c.b(C::Ne,0x23154)?;
    // 0x23184
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x23188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,260)?)?;
    // 0x2318c
    c.literal(R::R2,0x2324c)?;
    // 0x23190
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,35840)?)?;
    // 0x23194
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,119)?)?;
    // 0x2319c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x231a0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x231a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x231a8
    c.bl(0x17b0)?;
    // 0x231ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x231b0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // 0x231b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x231b8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x231bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,192)?)?;
    // 0x231c0
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x231c4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32768)?)?;
    // 0x231c8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x231cc
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,224)?)?;
    // 0x231d0
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R1,R::R1,32768)?)?;
    // 0x231d4
    c.emit(arm32::data_register(C::Ne,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x231d8
    c.b(C::Eq,0x23060)?;
    // 0x231dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x231e0
    c.b(C::Al,0x23238)?;
    // 0x231e4
    c.literal(R::R1,0x2325c)?;
    // 0x231e8
    c.b(C::Al,0x231f0)?;
    // 0x231ec
    c.literal(R::R1,0x23260)?;
    // 0x231f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x231f4
    c.bl(0x2ec94)?;
    // 0x231f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x231fc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x23200
    c.bl(0x2ec94)?;
    // 0x23204
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x23208
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x2320c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x23210
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x23214
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x23218
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // 0x2321c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x23220
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x23224
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x23228
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2322c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x23230
    c.b(C::Eq,0x23060)?;
    // 0x23234
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,4)?)?;
    // 0x23238
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x2323c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x23240
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23244
    c.b(C::Cc,0x230a0)?;
    // 0x23248
    c.b(C::Al,0x23060)?;
    c.check(0x2324c)?;
    const REGISTER_STATUS_LITERALS:[u32;8]=[
        0x000e_6a43,0x000e_6ee3,0x000e_2788,0xe000_8460,
        0x000e_6f3c,0x000e_6f21,0x000e_6f06,0x000e_6eed,
    ];
    for word in REGISTER_STATUS_LITERALS {c.emit(word)?;}
    c.check(0x2326c)
}
