// SPDX-License-Identifier: GPL-3.0-only
//! Small Venus runtime helpers for codec command mode selection, per-plane
//! status iteration and diagnostic-driven input metadata synchronization.
//! Separated runtime address tables are *not* ARM instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const CODEC_TYPE_TARGETS:[u32;20]=[
    0x0002501c, 0x00025028, 0x00025034, 0x00025010, 0x00025034,
    0x0002506c, 0x00025078, 0x00025034, 0x00025084, 0x00025034,
    0x00025090, 0x00025034, 0x0002509c, 0x00025010, 0x00025034,
    0x00025010, 0x00025034, 0x00025034, 0x00025034, 0x000250a8,
];
const CODEC_HELPER_LOGS:[u32;14]=[
    0xfbfeeffd, 0x000e4940, 0x000e2788, 0x000e490b, 0x000e48d0,
    0x000e4895, 0x000e485a, 0x000e481f, 0x000e47df, 0x000e47a8,
    0x000e4774, 0x000e516f, 0x000e7f0b, 0x000e6a43,
];
const CODEC_RECORD_LOGS:[u32;8]=[
    0x000e7f25, 0x000e2788, 0x000e7f3c, 0x000e6a43, 0x000e7f7b,
    0x000e7fa9, 0x000e7fc0, 0x000e7f4b,
];
const CODEC_APPLY_LOGS:[u32;6]=[
    0x000e550b, 0x000e2788, 0x000e7fd3, 0x000e7fff, 0x000e802c,
    0x000e8056,
];

/// Dispatch command flags via 20 indexed codec modes. The handler table
/// supports repeated destinations and reports malformed mode IDs.
fn initialize_codec_type(c:&mut Code)->Result<(),String>{
    c.check(0x24f90)?;
    // 0x24f90
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x24f94
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x24f98
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x24f9c
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x24fa0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x24fa4
    c.literal(R::R1,0x250e0)?;
    // 0x24fa8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24fac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,19)?)?;
    // 0x24fb0
    c.b(C::Hi,0x25034)?;
    // 0x24fb4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x24fb8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x24fbc
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x24fc0)?;
    for target in CODEC_TYPE_TARGETS {c.emit(target)?;}
    c.check(0x25010)?;
    // 0x25010
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x25014
    c.literal(R::R1,0x25100)?;
    // 0x25018
    c.b(C::Al,0x250b0)?;
    // 0x2501c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x25020
    c.literal(R::R1,0x25108)?;
    // 0x25024
    c.b(C::Al,0x250b0)?;
    // 0x25028
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x2502c
    c.literal(R::R1,0x250fc)?;
    // 0x25030
    c.b(C::Al,0x250b0)?;
    // 0x25034
    c.literal(R::R1,0x2510c)?;
    // 0x25038
    c.literal(R::R2,0x25110)?;
    // 0x2503c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x25040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2720)?)?;
    // 0x25044
    c.bl(0x2ec94)?;
    // 0x25048
    c.literal(R::R1,0x250e8)?;
    // 0x2504c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x25050
    c.bl(0x2ec94)?;
    // 0x25054
    c.literal(R::R2,0x25114)?;
    // 0x25058
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,161)?)?;
    // 0x2505c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x25060
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2560)?)?;
    // 0x25064
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x25068
    c.b(C::Al,0x17b0)?;
    // 0x2506c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x25070
    c.literal(R::R1,0x250ec)?;
    // 0x25074
    c.b(C::Al,0x250b0)?;
    // 0x25078
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x2507c
    c.literal(R::R1,0x250f0)?;
    // 0x25080
    c.b(C::Al,0x250b0)?;
    // 0x25084
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x25088
    c.literal(R::R1,0x250f4)?;
    // 0x2508c
    c.b(C::Al,0x250b0)?;
    // 0x25090
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x25094
    c.literal(R::R1,0x250f8)?;
    // 0x25098
    c.b(C::Al,0x250b0)?;
    // 0x2509c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x250a0
    c.literal(R::R1,0x25104)?;
    // 0x250a4
    c.b(C::Al,0x250b0)?;
    // 0x250a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2668)?)?;
    // 0x250ac
    c.literal(R::R1,0x250e4)?;
    // 0x250b0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2664)?)?;
    // 0x250b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x250b8
    c.bl(0x2ec94)?;
    // 0x250bc
    c.literal(R::R1,0x250e8)?;
    // 0x250c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x250c4
    c.bl(0x2ec94)?;
    // 0x250c8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,4)?)?;
    // 0x250cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,128)?)?;
    // 0x250d0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,2656)?)?;
    // 0x250d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x250d8
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x250dc
    c.b(C::Al,0x2a198)?;
    c.check(0x250e0)?;
    for word in CODEC_HELPER_LOGS {c.emit(word)?;}
    c.check(0x25118)
}

/// Iterate codec output records, validate descriptor ownership, and request
/// synchronization callbacks for each indexed platform session buffer.
fn synchronize_codec_records(c:&mut Code)->Result<(),String>{
    c.check(0x25118)?;
    // 0x25118
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x2511c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x25120
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x25124
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x25128
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2512c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,4)?)?;
    // 0x25130
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x25134
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x25138
    c.bl(0x80b90)?;
    // 0x2513c
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x25140
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x25144
    c.b(C::Eq,0x25238)?;
    // 0x25148
    c.literal(R::R1,0x25288)?;
    // 0x2514c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25150
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x25154
    c.bl(0x2ec94)?;
    // 0x25158
    c.literal(R::new(10)?,0x2528c)?;
    // 0x2515c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25160
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x25164
    c.bl(0x2ec94)?;
    // 0x25168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x2516c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::SP,8)?)?;
    // 0x25170
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x25174
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x25178
    c.bl(0x80994)?;
    // 0x2517c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x25180
    c.b(C::Eq,0x251b0)?;
    // 0x25184
    c.literal(R::R1,0x25290)?;
    // 0x25188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2518c
    c.bl(0x2ec94)?;
    // 0x25190
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x25194
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x25198
    c.bl(0x2ec94)?;
    // 0x2519c
    c.literal(R::R2,0x25294)?;
    // 0x251a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,182)?)?;
    // 0x251a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x251a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2560)?)?;
    // 0x251ac
    c.bl(0x17b0)?;
    // 0x251b0
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,8)?)?;
    // 0x251b4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,12)?)?;
    // 0x251b8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x251bc
    c.b(C::Ne,0x25200)?;
    // 0x251c0
    c.literal(R::R1,0x252a4)?;
    // 0x251c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x251c8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x251cc
    c.bl(0x2ec94)?;
    // 0x251d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x251d4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x251d8
    c.bl(0x2ec94)?;
    // 0x251dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x251e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x251e4
    c.b(C::Eq,0x2522c)?;
    // 0x251e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x251ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x251f0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x251f4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // 0x251f8
    c.bl(0x24f90)?;
    // 0x251fc
    c.b(C::Al,0x2522c)?;
    // 0x25200
    c.literal(R::R1,0x25298)?;
    // 0x25204
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x25208
    c.bl(0x2ec94)?;
    // 0x2520c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x25210
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x25214
    c.bl(0x2ec94)?;
    // 0x25218
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x2521c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x25220
    c.bl(0x80928)?;
    // 0x25224
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x25228
    c.b(C::Ne,0x25240)?;
    // 0x2522c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x25230
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x25234
    c.b(C::Cc,0x25170)?;
    // 0x25238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2523c
    c.b(C::Al,0x2527c)?;
    // 0x25240
    c.literal(R::R1,0x2529c)?;
    // 0x25244
    c.literal(R::R2,0x252a0)?;
    // 0x25248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,203)?)?;
    // 0x2524c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x25250
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2560)?)?;
    // 0x25254
    c.bl(0x2ec94)?;
    // 0x25258
    c.literal(R::R1,0x2528c)?;
    // 0x2525c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x25260
    c.bl(0x2ec94)?;
    // 0x25264
    c.literal(R::R2,0x25294)?;
    // 0x25268
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,716)?)?;
    // 0x2526c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x25270
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x25274
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x25278
    c.bl(0x17b0)?;
    // 0x2527c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x25280
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x25284
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x25288)?;
    for word in CODEC_RECORD_LOGS {c.emit(word)?;}
    c.check(0x252a8)
}

/// Apply four distinct codec output-plane settings, preserving the original
/// callback ordering and structured debug message addresses.
fn apply_codec_parameters(c:&mut Code)->Result<(),String>{
    c.check(0x252a8)?;
    // 0x252a8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x252ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x252b0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x252b4
    c.literal(R::R1,0x253e4)?;
    // 0x252b8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x252bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x252c0
    c.bl(0x2ec94)?;
    // 0x252c4
    c.literal(R::new(6)?,0x253e8)?;
    // 0x252c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x252cc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x252d0
    c.bl(0x2ec94)?;
    // 0x252d4
    c.bl(0x25954)?;
    // 0x252d8
    c.bl(0x25984)?;
    // 0x252dc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x252e0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x252e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x252e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x252ec
    c.bl(0xd8b0)?;
    // 0x252f0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x252f4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x252f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x252fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x25300
    c.bl(0x1b284)?;
    // 0x25304
    c.literal(R::R1,0x253ec)?;
    // 0x25308
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2530c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25310
    c.bl(0x2ec94)?;
    // 0x25314
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25318
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2531c
    c.bl(0x2ec94)?;
    // 0x25320
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,152)?)?;
    // 0x25324
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2532c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,12)?)?;
    // 0x25330
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x25334
    c.bl(0x25118)?;
    // 0x25338
    c.literal(R::R1,0x253f0)?;
    // 0x2533c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25340
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25344
    c.bl(0x2ec94)?;
    // 0x25348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2534c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x25350
    c.bl(0x2ec94)?;
    // 0x25354
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,152)?)?;
    // 0x25358
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x2535c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x25360
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // 0x25364
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x25368
    c.bl(0x25118)?;
    // 0x2536c
    c.literal(R::R1,0x253f4)?;
    // 0x25370
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25374
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25378
    c.bl(0x2ec94)?;
    // 0x2537c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25380
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x25384
    c.bl(0x2ec94)?;
    // 0x25388
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,140)?)?;
    // 0x2538c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25390
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x25394
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,12)?)?;
    // 0x25398
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2539c
    c.bl(0x25118)?;
    // 0x253a0
    c.literal(R::R1,0x253f8)?;
    // 0x253a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x253a8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x253ac
    c.bl(0x2ec94)?;
    // 0x253b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x253b4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x253b8
    c.bl(0x2ec94)?;
    // 0x253bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,140)?)?;
    // 0x253c0
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x253c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x253c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // 0x253cc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x253d0
    c.bl(0x25118)?;
    // 0x253d4
    c.bl(0x259a4)?;
    // 0x253d8
    c.bl(0x25974)?;
    // 0x253dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x253e0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x253e4)?;
    for word in CODEC_APPLY_LOGS {c.emit(word)?;}
    c.check(0x253fc)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    initialize_codec_type(c)?;
    synchronize_codec_records(c)?;
    apply_codec_parameters(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn mode_type_dispatch_is_bounded(){
        assert_eq!(CODEC_TYPE_TARGETS.len(),20);
        let mut c=Code::new(0x24f90);
        emit(&mut c).unwrap();
        assert_eq!(c.at(),0x253fc);
        assert_eq!(c.bytes.len(),0x46c);
        assert_eq!(CODEC_HELPER_LOGS.len()+CODEC_RECORD_LOGS.len()+CODEC_APPLY_LOGS.len(),28);
    }
}
