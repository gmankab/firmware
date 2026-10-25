// SPDX-License-Identifier: GPL-3.0-only
//! Venus parameter validation and interrupt/control logging routines:
//! per-plane bounds-check logging, write-register helpers, vector lookup,
//! buffer pool callbacks, and transparent diagnostic data classifications.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

pub(super) fn emit_next(c:&mut Code)->Result<(),String>{
    c.check(0x2326c)?;
    // 0x2326c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x23270
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x23274
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x23278
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x2327c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // 0x23280
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x23284
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R2,Shift::Lsl,12,false)?)?;
    // 0x23288
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,12)?)?;
    // 0x2328c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,16)?)?;
    // 0x23290
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::R0,R::R1,Shift::Lsl,4,false)?)?;
    // 0x23294
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,20)?)?;
    // 0x23298
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,0)?)?;
    // 0x2329c
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(6)?,Shift::Lsl,16)?)?;
    // 0x232a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x232a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x232a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,24)?)?;
    // 0x232ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x232b0
    c.b(C::Eq,0x23330)?;
    // 0x232b4
    c.literal(R::R2,0x233f0)?;
    // 0x232b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,256)?)?;
    // 0x232bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,242)?)?;
    // 0x232c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x232c4
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x232c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x232cc
    c.bl(0x17b0)?;
    // 0x232d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x232d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    // 0x232d8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x232dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,548)?)?;
    // 0x232e0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,37888)?)?;
    // 0x232e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x232e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x232ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    // 0x232f0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x232f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x232f8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x232fc
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23300
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x23304
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x23308
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    // 0x2330c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x23310
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1568)?)?;
    // 0x23314
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,36864)?)?;
    // 0x23318
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2331c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x23320
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    // 0x23324
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x23328
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x2332c
    c.b(C::Al,0x233ac)?;
    // 0x23330
    c.literal(R::R2,0x233f0)?;
    // 0x23334
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,256)?)?;
    // 0x23338
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,231)?)?;
    // 0x2333c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23340
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23344
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x23348
    c.bl(0x17b0)?;
    // 0x2334c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x23350
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    // 0x23354
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x23358
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,556)?)?;
    // 0x2335c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,37888)?)?;
    // 0x23360
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x23364
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x23368
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    // 0x2336c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x23370
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x23374
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x23378
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2337c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x23380
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x23384
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    // 0x23388
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x2338c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,552)?)?;
    // 0x23390
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,37888)?)?;
    // 0x23394
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x23398
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2339c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    // 0x233a0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x233a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,256)?)?;
    // 0x233a8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x233ac
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x233b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x233b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x233b8
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x233bc
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x233c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x233c4
    c.literal(R::R2,0x233f0)?;
    // 0x233c8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x233cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x233d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,256)?)?;
    // 0x233d4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x233d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,508)?)?;
    // 0x233dc
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x233e0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x233e4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x233e8
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x233ec
    c.b(C::Al,0x17b0)?;
    c.check(0x233f0)?;
    // 1 non-instruction control/diagnostic words from literal pool
    for value in [0x000e6a43u32] {c.emit(value)?;}
    c.check(0x233f4)?;
    c.check(0x233f4)?;
    // 0x233f4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x233f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x233fc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x23400
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x23404
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,12,false)?)?;
    // 0x23408
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,8)?)?;
    // 0x2340c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,0)?)?;
    // 0x23410
    c.literal(R::new(7)?,0x234dc)?;
    // 0x23414
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(10)?,R::R0,R::R2,Shift::Lsl,4,false)?)?;
    // 0x23418
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,16)?)?;
    // 0x2341c
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,12)?)?;
    // 0x23420
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(5)?,Shift::Lsl,16)?)?;
    // 0x23424
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x23428
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2342c
    c.b(C::Eq,0x23468)?;
    // 0x23430
    c.literal(R::R2,0x234e0)?;
    // 0x23434
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,256)?)?;
    // 0x23438
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,536)?)?;
    // 0x2343c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23440
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23444
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x23448
    c.bl(0x17b0)?;
    // 0x2344c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x23450
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x23454
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23458
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x2345c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x23460
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x23464
    c.b(C::Al,0x234a0)?;
    // 0x23468
    c.literal(R::R2,0x234e0)?;
    // 0x2346c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,256)?)?;
    // 0x23470
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x23474
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23478
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2347c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x23480
    c.bl(0x17b0)?;
    // 0x23484
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x23488
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x2348c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23490
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,12)?)?;
    // 0x23494
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,256)?)?;
    // 0x23498
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x2349c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x234a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x234a4
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x234a8
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x234ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x234b0
    c.literal(R::R2,0x234e0)?;
    // 0x234b4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x234b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x234bc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,256)?)?;
    // 0x234c0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x234c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,34)?)?;
    // 0x234c8
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x234cc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x234d0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x234d4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x234d8
    c.b(C::Al,0x17b0)?;
    c.check(0x234dc)?;
    // 2 non-instruction control/diagnostic words from literal pool
    for value in [0xe0009620u32,0x000e6a43u32] {c.emit(value)?;}
    c.check(0x234e4)?;
    c.check(0x234e4)?;
    // 0x234e4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x234e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x234ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R11,12)?)?;
    // 0x234f0
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(9)?,R::new(4)?.bit() | R::new(8)?.bit() | R::new(9)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x234f4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,24)?)?;
    // 0x234f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x234fc
    c.b(C::Eq,0x23568)?;
    // 0x23500
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,0)?)?;
    // 0x23504
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,8)?)?;
    // 0x23508
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,0)?)?;
    // 0x2350c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,12,false)?)?;
    // 0x23510
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,4)?)?;
    // 0x23514
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(5)?,Shift::Lsl,4,false)?)?;
    // 0x23518
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x2351c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,548)?)?;
    // 0x23520
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,37888)?)?;
    // 0x23524
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,0)?)?;
    // 0x23528
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // 0x2352c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,4)?)?;
    // 0x23530
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x23534
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x23538
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x2353c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,4)?)?;
    // 0x23540
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x23544
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1568)?)?;
    // 0x23548
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,36864)?)?;
    // 0x2354c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x23550
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x23554
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,4)?)?;
    // 0x23558
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x2355c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R12,R::new(8)?,Shift::Lsl,16,false)?)?;
    // 0x23560
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x23564
    c.b(C::Al,0x2357c)?;
    // 0x23568
    c.literal(R::R2,0x235b0)?;
    // 0x2356c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,49)?)?;
    // 0x23570
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23574
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x23578
    c.bl(0x17b0)?;
    // 0x2357c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x23580
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x23584
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x23588
    c.literal(R::R2,0x235b0)?;
    // 0x2358c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x23590
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23594
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x23598
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x2359c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,55)?)?;
    // 0x235a0
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x235a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x235a8
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x235ac
    c.b(C::Al,0x17b0)?;
    c.check(0x235b0)?;
    // 1 non-instruction control/diagnostic words from literal pool
    for value in [0x000e6a43u32] {c.emit(value)?;}
    c.check(0x235b4)?;
    c.check(0x235b4)?;
    // 0x235b4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x235b8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x235bc
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x235c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,16)?)?;
    // 0x235c4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,12)?)?;
    // 0x235c8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,8)?)?;
    // 0x235cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x235d0
    c.b(C::Eq,0x235f4)?;
    // 0x235d4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,12,false)?)?;
    // 0x235d8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x235dc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,4,false)?)?;
    // 0x235e0
    c.literal(R::R2,0x2363c)?;
    // 0x235e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,4)?)?;
    // 0x235e8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x235ec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x235f0
    c.b(C::Al,0x23608)?;
    // 0x235f4
    c.literal(R::R2,0x23640)?;
    // 0x235f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,69)?)?;
    // 0x235fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23600
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x23604
    c.bl(0x17b0)?;
    // 0x23608
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x2360c
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x23610
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x23614
    c.literal(R::R2,0x23640)?;
    // 0x23618
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x2361c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23620
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x23624
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x23628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,75)?)?;
    // 0x2362c
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23630
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x23634
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x23638
    c.b(C::Al,0x17b0)?;
    c.check(0x2363c)?;
    // 2 non-instruction control/diagnostic words from literal pool
    for value in [0xe0009620u32,0x000e6a43u32] {c.emit(value)?;}
    c.check(0x23644)?;
    c.check(0x23644)?;
    // 0x23644
    c.literal(R::R1,0x23660)?;
    // 0x23648
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2364c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x23650
    c.literal_cond(C::Ls,R::R1,0x23664)?;
    // 0x23654
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x23658
    c.emit(arm32::transfer_word_register(C::Ls,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x2365c
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x23660)?;
    // 2 non-instruction control/diagnostic words from literal pool
    for value in [0xfbfeeffdu32,0x000e8220u32] {c.emit(value)?;}
    c.check(0x23668)?;
    c.check(0x23668)?;
    // 0x23668
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x2366c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x23670
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // 0x23674
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
    // 0x23678
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2367c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::new(4)?,Shift::Lsl,2)?)?;
    // 0x23680
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x23684
    c.b(C::Eq,0x2369c)?;
    // 0x23688
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x2368c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,16)?)?;
    // 0x23690
    c.b(C::Cc,0x2367c)?;
    // 0x23694
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(4)?,R::R0,0)?)?;
    // 0x23698
    c.b(C::Al,0x236b8)?;
    // 0x2369c
    c.literal(R::R1,0x236c0)?;
    // 0x236a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x236a4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x236a8
    c.bl(0x2ec94)?;
    // 0x236ac
    c.literal(R::R1,0x236c4)?;
    // 0x236b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x236b4
    c.bl(0x2ec94)?;
    // 0x236b8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x236bc
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x236c0)?;
    // 2 non-instruction control/diagnostic words from literal pool
    for value in [0x000e6f55u32,0x000e2788u32] {c.emit(value)?;}
    c.check(0x236c8)?;
    c.check(0x236c8)?;
    // 0x236c8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x236cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x236d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,916)?)?;
    // 0x236d4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x236d8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x236dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x236e0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,6144)?)?;
    // 0x236e4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x236e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,64)?)?;
    // 0x236ec
    c.literal(R::R1,0x23760)?;
    // 0x236f0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x236f4
    c.b(C::Ne,0x2370c)?;
    // 0x236f8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x236fc
    c.bl(0x2efc)?;
    // 0x23700
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x23704
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x23708
    c.b(C::Ne,0x23720)?;
    // 0x2370c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x23710
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,16)?)?;
    // 0x23714
    c.b(C::Cc,0x236e4)?;
    // 0x23718
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2371c
    c.b(C::Al,0x23758)?;
    // 0x23720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,916)?)?;
    // 0x23724
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x23728
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x2372c
    c.bl(0x31ec)?;
    // 0x23730
    c.literal(R::R1,0x23764)?;
    // 0x23734
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23738
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x2373c
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x23740
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,64)?)?;
    // 0x23744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x23748
    c.bl(0x2ec94)?;
    // 0x2374c
    c.literal(R::R1,0x23768)?;
    // 0x23750
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x23754
    c.bl(0x2ec94)?;
    // 0x23758
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x2375c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x23760)?;
    // 3 non-instruction control/diagnostic words from literal pool
    for value in [0xdeadbeefu32,0x000e6f7cu32,0x000e2788u32] {c.emit(value)?;}
    c.check(0x2376c)?;
    c.check(0x2376c)?;
    // 0x2376c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x23770
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x23774
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x23778
    c.literal(R::R1,0x237c4)?;
    // 0x2377c
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x23780
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x23784
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // 0x23788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2378c
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x23790
    c.bl(0x2ec94)?;
    // 0x23794
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x23798
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2379c
    c.literal(R::new(6)?,0x237c8)?;
    // 0x237a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x237a4
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::R2,R::new(5)?,1)?)?;
    // 0x237a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x237ac
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x237b0
    c.bl(0x2ec94)?;
    // 0x237b4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x237b8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x237bc
    c.b(C::Ne,0x237a4)?;
    // 0x237c0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x237c4)?;
    // 2 non-instruction control/diagnostic words from literal pool
    for value in [0x000e6fa7u32,0x000e6fc2u32] {c.emit(value)?;}
    c.check(0x237cc)?;
    c.check(0x237cc)?;
    // 0x237cc
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    // 0x237d0
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x237d4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,40)?)?;
    // 0x237d8
    c.literal(R::R12,0x2383c)?;
    // 0x237dc
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,4)?)?;
    // 0x237e0
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,8)?)?;
    // 0x237e4
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,8)?)?;
    // 0x237e8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x237ec
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,12)?)?;
    // 0x237f0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x237f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x237f8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x237fc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,16)?)?;
    // 0x23800
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x23804
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,20)?)?;
    // 0x23808
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // 0x2380c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,24)?)?;
    // 0x23810
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,32)?)?;
    // 0x23814
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,28)?)?;
    // 0x23818
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,36)?)?;
    // 0x2381c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x23820
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x23824
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,4)?)?;
    // 0x23828
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,128)?)?;
    // 0x2382c
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // 0x23830
    c.bl(0x2a198)?;
    // 0x23834
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // 0x23838
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2383c)?;
    // 1 non-instruction control/diagnostic words from literal pool
    for value in [0x040210ffu32] {c.emit(value)?;}
    c.check(0x23840)?;
    Ok(())
}
