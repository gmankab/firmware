

/// Finish the frame-descriptor state machine, preserving the inline error
/// returns and optional recoveries. A separate non-instruction data word,
/// two short diagnostic pools and the thirteen-word final diagnostic pool
/// are modeled independently of ARM opcodes. The next real function starts
/// at 0xb9904 (a PUSH frame prologue).
fn finish_frame_descriptor_dispatch(c:&mut Code)->Result<(),String>{
    c.check(0xb9208)?;
    // ARM VMA 0xb9208
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM VMA 0xb920c
    c.b(C::Al,0xb921c,false)?;
    // ARM VMA 0xb9210
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM VMA 0xb9214
    c.b(C::Al,0xb921c,false)?;
    // ARM VMA 0xb9218
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM VMA 0xb921c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM VMA 0xb9220
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,304)?)?;
    // ARM VMA 0xb9224
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,24)?)?;
    // ARM VMA 0xb9228
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb922c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,28)?)?;
    // ARM VMA 0xb9230
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb9234
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,0)?)?;
    // ARM VMA 0xb9238
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R2,8)?)?;
    // ARM VMA 0xb923c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9240
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2048)?)?;
    // ARM VMA 0xb9244
    c.b(C::Eq,0xb92b0,false)?;
    // ARM VMA 0xb9248
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,44)?)?;
    // ARM VMA 0xb924c
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb9250
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,24)?)?;
    // ARM VMA 0xb9254
    c.emit(arm32::transfer_word(C::Cs,false,R::new(10)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb9258
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,8)?)?;
    // ARM VMA 0xb925c
    c.emit(arm32::transfer_word(C::Cs,false,R::new(5)?,R::new(4)?,12)?)?;
    // ARM VMA 0xb9260
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,16)?)?;
    // ARM VMA 0xb9264
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,44)?)?;
    // ARM VMA 0xb9268
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,0)?)?;
    // ARM VMA 0xb926c
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R1,R::new(5)?,1)?)?;
    // ARM VMA 0xb9270
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,20)?)?;
    // ARM VMA 0xb9274
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,32)?)?;
    // ARM VMA 0xb9278
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,20)?)?;
    // ARM VMA 0xb927c
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,24)?)?;
    // ARM VMA 0xb9280
    c.emit(arm32::transfer_word(C::Cs,false,R::new(10)?,R::new(4)?,28)?)?;
    // ARM VMA 0xb9284
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::new(9)?,668)?)?;
    // ARM VMA 0xb9288
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,40)?)?;
    // ARM VMA 0xb928c
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::new(9)?,664)?)?;
    // ARM VMA 0xb9290
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,36)?)?;
    // ARM VMA 0xb9294
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::SP,72)?)?;
    // ARM VMA 0xb9298
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R0,44)?)?;
    // ARM VMA 0xb929c
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb92a0
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb92a4
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::new(4)?,R::new(4)?,44)?)?;
    // ARM VMA 0xb92a8
    c.emit(arm32::data_register(C::Cs,Op::Add,R::new(6)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb92ac
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::new(8)?,1116)?)?;
    // ARM VMA 0xb92b0
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2097152)?)?;
    // ARM VMA 0xb92b4
    c.b(C::Eq,0xb9454,false)?;
    // ARM VMA 0xb92b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,336)?)?;
    // ARM VMA 0xb92bc
    c.b(C::Cc,0xb9454,false)?;
    // ARM VMA 0xb92c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,336)?)?;
    // ARM VMA 0xb92c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb92c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,316)?)?;
    // ARM VMA 0xb92cc
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(4)?,R::R2.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb92d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // ARM VMA 0xb92d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // ARM VMA 0xb92d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,12)?)?;
    // ARM VMA 0xb92dc
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // ARM VMA 0xb92e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,16)?)?;
    // ARM VMA 0xb92e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // ARM VMA 0xb92e8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,117440512)?)?;
    // ARM VMA 0xb92ec
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(5)?,20)?)?;
    // ARM VMA 0xb92f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb92f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // ARM VMA 0xb92f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,312)?)?;
    // ARM VMA 0xb92fc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(5)?,52)?)?;
    // ARM VMA 0xb9300
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::R0.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // ARM VMA 0xb9304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9308
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,20)?)?;
    // ARM VMA 0xb930c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,16)?)?;
    // ARM VMA 0xb9310
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,40)?)?;
    // ARM VMA 0xb9314
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,24)?)?;
    // ARM VMA 0xb9318
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,44)?)?;
    // ARM VMA 0xb931c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,28)?)?;
    // ARM VMA 0xb9320
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,56)?)?;
    // ARM VMA 0xb9324
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // ARM VMA 0xb9328
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,32)?)?;
    // ARM VMA 0xb932c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // ARM VMA 0xb9330
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // ARM VMA 0xb9334
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,36)?)?;
    // ARM VMA 0xb9338
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,248)?)?;
    // ARM VMA 0xb933c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1024)?)?;
    // ARM VMA 0xb9340
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM VMA 0xb9344
    c.b(C::Al,0x471c,true)?;
    // ARM VMA 0xb9348
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,28)?)?;
    // ARM VMA 0xb934c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,184)?)?;
    // ARM VMA 0xb9350
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,728)?)?;
    // ARM VMA 0xb9354
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,32)?)?;
    // ARM VMA 0xb9358
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,48)?)?;
    // ARM VMA 0xb935c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // ARM VMA 0xb9360
    c.b(C::Ne,0xb9374,false)?;
    // ARM VMA 0xb9364
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb9368
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // ARM VMA 0xb936c
    c.b(C::Al,0x31ec,true)?;
    // ARM VMA 0xb9370
    c.b(C::Al,0xb9384,false)?;
    // ARM VMA 0xb9374
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,380)?)?;
    // ARM VMA 0xb9378
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // ARM VMA 0xb937c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // ARM VMA 0xb9380
    c.b(C::Al,0x471c,true)?;
    // ARM VMA 0xb9384
    c.literal(R::new(6)?,0xb98ec)?;
    // ARM VMA 0xb9388
    c.literal(R::new(7)?,0xb98d8)?;
    // ARM VMA 0xb938c
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,48)?)?;
    // ARM VMA 0xb9390
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM VMA 0xb9394
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(4)?,Shift::Lsl,5,false)?)?;
    // ARM VMA 0xb9398
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // ARM VMA 0xb939c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,2)?)?;
    // ARM VMA 0xb93a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb93a4
    c.b(C::Eq,0xb93f8,false)?;
    // ARM VMA 0xb93a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM VMA 0xb93ac
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(10)?,Shift::Lsl,2,false)?)?;
    // ARM VMA 0xb93b0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,76)?)?;
    // ARM VMA 0xb93b4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,108)?)?;
    // ARM VMA 0xb93b8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,140)?)?;
    // ARM VMA 0xb93bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,172)?)?;
    // ARM VMA 0xb93c0
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb93c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM VMA 0xb93c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb93cc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb93d0
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // ARM VMA 0xb93d4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // ARM VMA 0xb93d8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb93dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb93e0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb93e4
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb93e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // ARM VMA 0xb93ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // ARM VMA 0xb93f0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb93f4
    c.b(C::Cc,0xb93ac,false)?;
    // ARM VMA 0xb93f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // ARM VMA 0xb93fc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,132)?)?;
    // ARM VMA 0xb9400
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,2)?)?;
    // ARM VMA 0xb9404
    c.b(C::Ne,0xb9394,false)?;
    // ARM VMA 0xb9408
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,48)?)?;
    // ARM VMA 0xb940c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9410
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,64)?)?;
    // ARM VMA 0xb9414
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,68)?)?;
    // ARM VMA 0xb9418
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,0)?)?;
    // ARM VMA 0xb941c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb9420
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,16)?)?;
    // ARM VMA 0xb9424
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,32)?)?;
    // ARM VMA 0xb9428
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,44)?)?;
    // ARM VMA 0xb942c
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,40)?)?;
    // ARM VMA 0xb9430
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,36)?)?;
    // ARM VMA 0xb9434
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,56)?)?;
    // ARM VMA 0xb9438
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb943c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // ARM VMA 0xb9440
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb9444
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9448
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,0)?)?;
    // ARM VMA 0xb944c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9450
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1116)?)?;
    // ARM VMA 0xb9454
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,33554432)?)?;
    // ARM VMA 0xb9458
    c.b(C::Eq,0xb94d4,false)?;
    // ARM VMA 0xb945c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,48)?)?;
    // ARM VMA 0xb9460
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,3)?)?;
    // ARM VMA 0xb9464
    c.emit(arm32::transfer_word(C::Cs,false,R::new(10)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb9468
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,8)?)?;
    // ARM VMA 0xb946c
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,13)?)?;
    // ARM VMA 0xb9470
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,12)?)?;
    // ARM VMA 0xb9474
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,28)?)?;
    // ARM VMA 0xb9478
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,16)?)?;
    // ARM VMA 0xb947c
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,48)?)?;
    // ARM VMA 0xb9480
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,0)?)?;
    // ARM VMA 0xb9484
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,676)?)?;
    // ARM VMA 0xb9488
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,20)?)?;
    // ARM VMA 0xb948c
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,680)?)?;
    // ARM VMA 0xb9490
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,24)?)?;
    // ARM VMA 0xb9494
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,684)?)?;
    // ARM VMA 0xb9498
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,28)?)?;
    // ARM VMA 0xb949c
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,688)?)?;
    // ARM VMA 0xb94a0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,32)?)?;
    // ARM VMA 0xb94a4
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,692)?)?;
    // ARM VMA 0xb94a8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,36)?)?;
    // ARM VMA 0xb94ac
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,696)?)?;
    // ARM VMA 0xb94b0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,40)?)?;
    // ARM VMA 0xb94b4
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,700)?)?;
    // ARM VMA 0xb94b8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,44)?)?;
    // ARM VMA 0xb94bc
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb94c0
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R1,48)?)?;
    // ARM VMA 0xb94c4
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb94c8
    c.emit(arm32::transfer_word(C::Cs,true,R::R2,R::new(4)?,0)?)?;
    // ARM VMA 0xb94cc
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::new(4)?,R::new(4)?,48)?)?;
    // ARM VMA 0xb94d0
    c.emit(arm32::data_register(C::Cs,Op::Add,R::new(6)?,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb94d4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1024)?)?;
    // ARM VMA 0xb94d8
    c.b(C::Eq,0xb9524,false)?;
    // ARM VMA 0xb94dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,24)?)?;
    // ARM VMA 0xb94e0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,3)?)?;
    // ARM VMA 0xb94e4
    c.emit(arm32::transfer_word(C::Cs,false,R::new(10)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb94e8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,8)?)?;
    // ARM VMA 0xb94ec
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::new(5)?,1)?)?;
    // ARM VMA 0xb94f0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,12)?)?;
    // ARM VMA 0xb94f4
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM VMA 0xb94f8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,16)?)?;
    // ARM VMA 0xb94fc
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,24)?)?;
    // ARM VMA 0xb9500
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,0)?)?;
    // ARM VMA 0xb9504
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,720)?)?;
    // ARM VMA 0xb9508
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,20)?)?;
    // ARM VMA 0xb950c
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb9510
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R1,24)?)?;
    // ARM VMA 0xb9514
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb9518
    c.emit(arm32::transfer_word(C::Cs,true,R::R2,R::new(4)?,0)?)?;
    // ARM VMA 0xb951c
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::new(4)?,R::new(4)?,24)?)?;
    // ARM VMA 0xb9520
    c.emit(arm32::data_register(C::Cs,Op::Add,R::new(6)?,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9524
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,32768)?)?;
    // ARM VMA 0xb9528
    c.b(C::Eq,0xb98b8,false)?;
    // ARM VMA 0xb952c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,40)?)?;
    // ARM VMA 0xb9530
    c.b(C::Cc,0xb98b8,false)?;
    // ARM VMA 0xb9534
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb9538
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb953c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // ARM VMA 0xb9540
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // ARM VMA 0xb9544
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // ARM VMA 0xb9548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // ARM VMA 0xb954c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // ARM VMA 0xb9550
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // ARM VMA 0xb9554
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb9558
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1008)?)?;
    // ARM VMA 0xb955c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,20)?)?;
    // ARM VMA 0xb9560
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,1012)?)?;
    // ARM VMA 0xb9564
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,24)?)?;
    // ARM VMA 0xb9568
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1016)?)?;
    // ARM VMA 0xb956c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // ARM VMA 0xb9570
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1020)?)?;
    // ARM VMA 0xb9574
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,32)?)?;
    // ARM VMA 0xb9578
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,1024)?)?;
    // ARM VMA 0xb957c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,36)?)?;
    // ARM VMA 0xb9580
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb9584
    c.literal(R::R1,0xb98f0)?;
    // ARM VMA 0xb9588
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb958c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9590
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb9594
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9598
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb959c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb95a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb95a4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // ARM VMA 0xb95a8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb95ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,20)?)?;
    // ARM VMA 0xb95b0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb95b4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb95b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb95bc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb95c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1116)?)?;
    // ARM VMA 0xb95c4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,65536)?)?;
    // ARM VMA 0xb95c8
    c.b(C::Eq,0xb961c,false)?;
    // ARM VMA 0xb95cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,28)?)?;
    // ARM VMA 0xb95d0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,3)?)?;
    // ARM VMA 0xb95d4
    c.emit(arm32::transfer_word(C::Cs,false,R::new(10)?,R::R2,4)?)?;
    // ARM VMA 0xb95d8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,8)?)?;
    // ARM VMA 0xb95dc
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,16)?)?;
    // ARM VMA 0xb95e0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,12)?)?;
    // ARM VMA 0xb95e4
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,8)?)?;
    // ARM VMA 0xb95e8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,16)?)?;
    // ARM VMA 0xb95ec
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,28)?)?;
    // ARM VMA 0xb95f0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,0)?)?;
    // ARM VMA 0xb95f4
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,996)?)?;
    // ARM VMA 0xb95f8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,20)?)?;
    // ARM VMA 0xb95fc
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,1000)?)?;
    // ARM VMA 0xb9600
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,24)?)?;
    // ARM VMA 0xb9604
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb9608
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R1,28)?)?;
    // ARM VMA 0xb960c
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb9610
    c.emit(arm32::transfer_word(C::Cs,true,R::new(3)?,R::R2,0)?)?;
    // ARM VMA 0xb9614
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R2,R::R2,28)?)?;
    // ARM VMA 0xb9618
    c.emit(arm32::data_register(C::Cs,Op::Add,R::new(6)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb961c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,8388608)?)?;
    // ARM VMA 0xb9620
    c.b(C::Eq,0xb9698,false)?;
    // ARM VMA 0xb9624
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,76)?)?;
    // ARM VMA 0xb9628
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(7)?,R::R0,76)?)?;
    // ARM VMA 0xb962c
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb9630
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,25)?)?;
    // ARM VMA 0xb9634
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(3)?,R::R0,56)?)?;
    // ARM VMA 0xb9638
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R12,R::R2,8)?)?;
    // ARM VMA 0xb963c
    c.emit(arm32::mov_register(C::Cs,R::new(4)?,R::new(6)?))?;
    // ARM VMA 0xb9640
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R2,R::new(7)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb9644
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R12,R::R0.bit() | R::R1.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb9648
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R0,R::new(9)?,1216)?)?;
    // ARM VMA 0xb964c
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R1,R::R2,20)?)?;
    // ARM VMA 0xb9650
    c.emit(arm32::transfer_multiple(C::Cs,true,R::R0,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb9654
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R1,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb9658
    c.emit(arm32::transfer_multiple(C::Cs,true,R::R0,R::new(3)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb965c
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R0,R::new(9)?,220)?)?;
    // ARM VMA 0xb9660
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R0,R::R0,1024)?)?;
    // ARM VMA 0xb9664
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R1,R::new(3)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb9668
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R1,R::R2,48)?)?;
    // ARM VMA 0xb966c
    c.emit(arm32::transfer_multiple(C::Cs,true,R::R0,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb9670
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R1,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb9674
    c.emit(arm32::transfer_multiple(C::Cs,true,R::R0,R::new(3)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb9678
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R1,R::new(3)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb967c
    c.emit(arm32::mov_register(C::Cs,R::new(6)?,R::new(4)?))?;
    // ARM VMA 0xb9680
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::SP,72)?)?;
    // ARM VMA 0xb9684
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R0,76)?)?;
    // ARM VMA 0xb9688
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb968c
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb9690
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R2,R::R2,76)?)?;
    // ARM VMA 0xb9694
    c.emit(arm32::data_register(C::Cs,Op::Add,R::new(6)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9698
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2452)?)?;
    // ARM VMA 0xb969c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb96a0
    c.b(C::Ne,0xb96e0,false)?;
    // ARM VMA 0xb96a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,24)?)?;
    // ARM VMA 0xb96a8
    c.b(C::Cc,0xb96e0,false)?;
    // ARM VMA 0xb96ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // ARM VMA 0xb96b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb96b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb96b8
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::new(3)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb96bc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,8)?)?;
    // ARM VMA 0xb96c0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,12)?)?;
    // ARM VMA 0xb96c4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,16)?)?;
    // ARM VMA 0xb96c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,24)?)?;
    // ARM VMA 0xb96cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,108)?)?;
    // ARM VMA 0xb96d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,64)?)?;
    // ARM VMA 0xb96d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,108)?)?;
    // ARM VMA 0xb96d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,60)?)?;
    // ARM VMA 0xb96dc
    c.b(C::Al,0x205c,true)?;
    // ARM VMA 0xb96e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,212)?)?;
    // ARM VMA 0xb96e4
    c.b(C::Al,0x7eb40,true)?;
    // ARM VMA 0xb96e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb96ec
    c.b(C::Ne,0xb986c,false)?;
    // ARM VMA 0xb96f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,100)?)?;
    // ARM VMA 0xb96f4
    c.b(C::Al,0x2bf4,true)?;
    // ARM VMA 0xb96f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,420)?)?;
    // ARM VMA 0xb96fc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9700
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,420)?)?;
    // ARM VMA 0xb9704
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,100)?)?;
    // ARM VMA 0xb9708
    c.b(C::Al,0x2c34,true)?;
    // ARM VMA 0xb970c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // ARM VMA 0xb9710
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,76)?)?;
    // ARM VMA 0xb9714
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb9718
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,0)?)?;
    // ARM VMA 0xb971c
    c.b(C::Al,0x26754,true)?;
    // ARM VMA 0xb9720
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // ARM VMA 0xb9724
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,168)?)?;
    // ARM VMA 0xb9728
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,108)?)?;
    // ARM VMA 0xb972c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,152)?)?;
    // ARM VMA 0xb9730
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,160)?)?;
    // ARM VMA 0xb9734
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,128)?)?;
    // ARM VMA 0xb9738
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb973c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // ARM VMA 0xb9740
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // ARM VMA 0xb9744
    c.literal(R::R1,0xb98f4)?;
    // ARM VMA 0xb9748
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // ARM VMA 0xb974c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // ARM VMA 0xb9750
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // ARM VMA 0xb9754
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9758
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb975c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // ARM VMA 0xb9760
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9764
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM VMA 0xb9768
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb976c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,17)?)?;
    // ARM VMA 0xb9770
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // ARM VMA 0xb9774
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM VMA 0xb9778
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // ARM VMA 0xb977c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM VMA 0xb9780
    c.b(C::Al,0xb9220,false)?;
    // ARM VMA 0xb9784
    c.literal(R::R1,0xb98dc)?;
    // ARM VMA 0xb9788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb978c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9790
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb9794
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb9798
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb979c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8)?)?;
    // ARM VMA 0xb97a0
    c.b(C::Al,0xb8864,false)?;
    // ARM VMA 0xb97a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,208)?)?;
    // ARM VMA 0xb97a8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb97ac
    c.b(C::Eq,0xb9050,false)?;
    // ARM VMA 0xb97b0
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb97b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb97b8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb97bc
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,208)?)?;
    // ARM VMA 0xb97c0
    c.b(C::Al,0xb9050,false)?;
    // ARM VMA 0xb97c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb97c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,172)?)?;
    // ARM VMA 0xb97cc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,168)?)?;
    // ARM VMA 0xb97d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,164)?)?;
    // ARM VMA 0xb97d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,160)?)?;
    // ARM VMA 0xb97d8
    c.b(C::Al,0xb8ff8,false)?;
    // ARM VMA 0xb97dc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R1,8)?)?;
    // ARM VMA 0xb97e0
    c.literal(R::R1,0xb98e8)?;
    // ARM VMA 0xb97e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,108)?)?;
    // ARM VMA 0xb97e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb97ec
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb97f0
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb97f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb97f8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb97fc
    c.b(C::Al,0xb89bc,false)?;
    c.check(0xb9800)?;
    c.emit(0x0022_1008)?; // non-instruction controller feature word
    c.check(0xb9804)?;
    // ARM VMA 0xb9804
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,108)?)?;
    // ARM VMA 0xb9808
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // ARM VMA 0xb980c
    c.b(C::Al,0xb8ff4,false)?;
    // ARM VMA 0xb9810
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,212)?)?;
    // ARM VMA 0xb9814
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9818
    c.b(C::Eq,0xb9764,false)?;
    // ARM VMA 0xb981c
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb9820
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9824
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9828
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,212)?)?;
    // ARM VMA 0xb982c
    c.b(C::Al,0xb9764,false)?;
    c.check(0xb9830)?;
    const FRAME_ALLOCATION_LOGS:[u32;2]=[0x000f_984c,0x000e_2788];
    for word in FRAME_ALLOCATION_LOGS {c.emit(word)?;}
    c.check(0xb9838)?;
    // ARM VMA 0xb9838
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb983c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,172)?)?;
    // ARM VMA 0xb9840
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,168)?)?;
    // ARM VMA 0xb9844
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,164)?)?;
    // ARM VMA 0xb9848
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,160)?)?;
    // ARM VMA 0xb984c
    c.b(C::Al,0xb970c,false)?;
    // ARM VMA 0xb9850
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,108)?)?;
    // ARM VMA 0xb9854
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8)?)?;
    // ARM VMA 0xb9858
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,108)?)?;
    // ARM VMA 0xb985c
    c.b(C::Al,0xb9118,false)?;
    c.check(0xb9860)?;
    const CODEC_MMIO_MODES:[u32;3]=[0x7fe0_0002,0x7f10_0002,0x0101_0200];
    for word in CODEC_MMIO_MODES {c.emit(word)?;}
    c.check(0xb986c)?;
    // ARM VMA 0xb986c
    c.literal(R::R1,0xb98fc)?;
    // ARM VMA 0xb9870
    c.literal(R::R2,0xb98e4)?;
    // ARM VMA 0xb9874
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,153)?)?;
    // ARM VMA 0xb9878
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb987c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM VMA 0xb9880
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9884
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb9888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb988c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9890
    c.literal(R::R2,0xb9900)?;
    // ARM VMA 0xb9894
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,154)?)?;
    // ARM VMA 0xb9898
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb989c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // ARM VMA 0xb98a0
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb98a4
    c.b(C::Al,0xb970c,false)?;
    // ARM VMA 0xb98a8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(7)?))?;
    // ARM VMA 0xb98ac
    c.b(C::Al,0xb8d2c,false)?;
    // ARM VMA 0xb98b0
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // ARM VMA 0xb98b4
    c.b(C::Al,0xb8ea4,false)?;
    // ARM VMA 0xb98b8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // ARM VMA 0xb98bc
    c.b(C::Al,0xb95c4,false)?;
    // ARM VMA 0xb98c0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,108)?)?;
    // ARM VMA 0xb98c4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,32)?)?;
    // ARM VMA 0xb98c8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,108)?)?;
    // ARM VMA 0xb98cc
    c.b(C::Al,0xb910c,false)?;
    c.check(0xb98d0)?;
    const FRAME_DIAGNOSTICS:[u32;13]=[
        0x000f_98c3,0x000f_98e0,0x000e_2788,0x000f_9892,
        0x000f_98c3,0x000f_98e0,0x000f_98f7,0x000f_690d,
        0x000f_6864,0x000f_9928,0x000f_9987,0x000f_99bf,
        0x000f_99f9,
    ];
    for word in FRAME_DIAGNOSTICS {c.emit(word)?;}
    c.check(0xb9904)
}
