
/// Transition active codec contexts between close/reset states. Walk per-slot
/// completion queues, update global controller flags, and release the active
/// buffer callbacks. Both codec resource paths preserve the source's
/// conditional notifications and branch back to shared cleanup logic.
fn drain_codec_slot_contexts_and_close(c:&mut Code)->Result<(),String>{
    c.check(0xb71b8)?;
    // ARM VMA 0xb71b8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb71bc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM VMA 0xb71c0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,76)?)?;
    // ARM VMA 0xb71c4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM VMA 0xb71c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb71cc
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,4)?)?;
    // ARM VMA 0xb71d0
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(4)?,R::R1.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb71d4
    c.b(C::Al,0x3084,true)?;
    // ARM VMA 0xb71d8
    c.literal(R::R2,0xb76d4)?;
    // ARM VMA 0xb71dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // ARM VMA 0xb71e0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb71e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1072)?)?;
    // ARM VMA 0xb71e8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb71ec
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb71f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // ARM VMA 0xb71f4
    c.b(C::Al,0x293c,true)?;
    // ARM VMA 0xb71f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // ARM VMA 0xb71fc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // ARM VMA 0xb7200
    c.b(C::Eq,0xb7220,false)?;
    // ARM VMA 0xb7204
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,10)?)?;
    // ARM VMA 0xb7208
    c.b(C::Ne,0xb7244,false)?;
    // ARM VMA 0xb720c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,236)?)?;
    // ARM VMA 0xb7210
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // ARM VMA 0xb7214
    c.b(C::Ne,0xb7234,false)?;
    // ARM VMA 0xb7218
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // ARM VMA 0xb721c
    c.b(C::Al,0xb7230,false)?;
    // ARM VMA 0xb7220
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,236)?)?;
    // ARM VMA 0xb7224
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb7228
    c.b(C::Ne,0xb7234,false)?;
    // ARM VMA 0xb722c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // ARM VMA 0xb7230
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,236)?)?;
    // ARM VMA 0xb7234
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // ARM VMA 0xb7238
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xb723c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM VMA 0xb7240
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb7244
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // ARM VMA 0xb7248
    c.b(C::Cc,0xb723c,false)?;
    // ARM VMA 0xb724c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,244)?)?;
    // ARM VMA 0xb7250
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // ARM VMA 0xb7254
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM VMA 0xb7258
    c.b(C::Al,0x80994,true)?;
    // ARM VMA 0xb725c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7260
    c.b(C::Ne,0xb73ec,false)?;
    // ARM VMA 0xb7264
    c.literal(R::new(9)?,0xb76e0)?;
    // ARM VMA 0xb7268
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // ARM VMA 0xb726c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R11,32)?)?;
    // ARM VMA 0xb7270
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,16777216)?)?;
    // ARM VMA 0xb7274
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-32)?)?;
    // ARM VMA 0xb7278
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,564)?)?;
    // ARM VMA 0xb727c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7280
    c.b(C::Ne,0xb74ac,false)?;
    // ARM VMA 0xb7284
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // ARM VMA 0xb7288
    c.literal(R::R1,0xb76e8)?;
    // ARM VMA 0xb728c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7290
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7294
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7298
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM VMA 0xb729c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb72a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // ARM VMA 0xb72a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1064)?)?;
    // ARM VMA 0xb72a8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb72ac
    c.b(C::Eq,0xb7308,false)?;
    // ARM VMA 0xb72b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM VMA 0xb72b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16777216)?)?;
    // ARM VMA 0xb72b8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb72bc
    c.b(C::Ne,0xb737c,false)?;
    // ARM VMA 0xb72c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,532)?)?;
    // ARM VMA 0xb72c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb72c8
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,520)?)?;
    // ARM VMA 0xb72cc
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb72d0
    c.b(C::Eq,0xb72e0,false)?;
    // ARM VMA 0xb72d4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb72d8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb72dc
    c.b(C::Al,0xb9b78,true)?;
    // ARM VMA 0xb72e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,208)?)?;
    // ARM VMA 0xb72e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb72e8
    c.b(C::Eq,0xb737c,false)?;
    // ARM VMA 0xb72ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1028)?)?;
    // ARM VMA 0xb72f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb72f4
    c.b(C::Eq,0xb7354,false)?;
    // ARM VMA 0xb72f8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb72fc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb7300
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb7304
    c.b(C::Al,0xb7378,false)?;
    // ARM VMA 0xb7308
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb730c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb7310
    c.b(C::Al,0xb9904,true)?;
    // ARM VMA 0xb7314
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7318
    c.b(C::Eq,0xb737c,false)?;
    // ARM VMA 0xb731c
    c.literal(R::R1,0xb76ec)?;
    // ARM VMA 0xb7320
    c.literal(R::R2,0xb76dc)?;
    // ARM VMA 0xb7324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // ARM VMA 0xb7328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb732c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM VMA 0xb7330
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7334
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb7338
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM VMA 0xb733c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7340
    c.literal(R::R2,0xb76d4)?;
    // ARM VMA 0xb7344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1280)?)?;
    // ARM VMA 0xb734c
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb7350
    c.b(C::Al,0xb737c,false)?;
    // ARM VMA 0xb7354
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,552)?)?;
    // ARM VMA 0xb7358
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb735c
    c.b(C::Ne,0xb736c,false)?;
    // ARM VMA 0xb7360
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,496)?)?;
    // ARM VMA 0xb7364
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7368
    c.b(C::Eq,0xb737c,false)?;
    // ARM VMA 0xb736c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7370
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb7374
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb7378
    c.b(C::Al,0xb87d8,true)?;
    // ARM VMA 0xb737c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,444)?)?;
    // ARM VMA 0xb7380
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb7384
    c.b(C::Eq,0xb73d0,false)?;
    // ARM VMA 0xb7388
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // ARM VMA 0xb738c
    c.b(C::Al,0xd88a0,true)?;
    // ARM VMA 0xb7390
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7394
    c.b(C::Eq,0xb73d0,false)?;
    // ARM VMA 0xb7398
    c.literal(R::R1,0xb76f0)?;
    // ARM VMA 0xb739c
    c.literal(R::R2,0xb76dc)?;
    // ARM VMA 0xb73a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    // ARM VMA 0xb73a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb73a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // ARM VMA 0xb73ac
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb73b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb73b4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM VMA 0xb73b8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb73bc
    c.literal(R::R2,0xb76d4)?;
    // ARM VMA 0xb73c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // ARM VMA 0xb73c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb73c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // ARM VMA 0xb73cc
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb73d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,204)?)?;
    // ARM VMA 0xb73d4
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb73d8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM VMA 0xb73dc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // ARM VMA 0xb73e0
    c.b(C::Al,0x80994,true)?;
    // ARM VMA 0xb73e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb73e8
    c.b(C::Eq,0xb7274,false)?;
    // ARM VMA 0xb73ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // ARM VMA 0xb73f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // ARM VMA 0xb73f4
    c.b(C::Cc,0xb7234,false)?;
    // ARM VMA 0xb73f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,236)?)?;
    // ARM VMA 0xb73fc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb7400
    c.b(C::Ne,0xb7234,false)?;
    // ARM VMA 0xb7404
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,912)?)?;
    // ARM VMA 0xb7408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(5)?,380)?)?;
    // ARM VMA 0xb740c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7410
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,900)?)?;
    // ARM VMA 0xb7414
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7418
    c.b(C::Eq,0xb7468,false)?;
    // ARM VMA 0xb741c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb7420
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb7424
    c.b(C::Al,0xb9b78,true)?;
    // ARM VMA 0xb7428
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb742c
    c.b(C::Eq,0xb7468,false)?;
    // ARM VMA 0xb7430
    c.literal(R::R1,0xb76d8)?;
    // ARM VMA 0xb7434
    c.literal(R::R2,0xb76dc)?;
    // ARM VMA 0xb7438
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,94)?)?;
    // ARM VMA 0xb743c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb7440
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM VMA 0xb7444
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7448
    c.literal(R::R1,0xb76e0)?;
    // ARM VMA 0xb744c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb7450
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7454
    c.literal(R::R2,0xb76d4)?;
    // ARM VMA 0xb7458
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,95)?)?;
    // ARM VMA 0xb745c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7460
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // ARM VMA 0xb7464
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb7468
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,588)?)?;
    // ARM VMA 0xb746c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7470
    c.b(C::Eq,0xb7684,false)?;
    // ARM VMA 0xb7474
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,380)?)?;
    // ARM VMA 0xb7478
    c.literal(R::R1,0xb76e4)?;
    // ARM VMA 0xb747c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7480
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7484
    c.literal(R::R1,0xb76e0)?;
    // ARM VMA 0xb7488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb748c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7490
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1408)?)?;
    // ARM VMA 0xb7494
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7498
    c.b(C::Eq,0xb765c,false)?;
    // ARM VMA 0xb749c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb74a0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb74a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb74a8
    c.b(C::Al,0xb7680,false)?;
    // ARM VMA 0xb74ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,912)?)?;
    // ARM VMA 0xb74b0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,380)?)?;
    // ARM VMA 0xb74b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb74b8
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,900)?)?;
    // ARM VMA 0xb74bc
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb74c0
    c.b(C::Eq,0xb7510,false)?;
    // ARM VMA 0xb74c4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb74c8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb74cc
    c.b(C::Al,0xb9b78,true)?;
    // ARM VMA 0xb74d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb74d4
    c.b(C::Eq,0xb7510,false)?;
    // ARM VMA 0xb74d8
    c.literal(R::R1,0xb76d8)?;
    // ARM VMA 0xb74dc
    c.literal(R::R2,0xb76dc)?;
    // ARM VMA 0xb74e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,151)?)?;
    // ARM VMA 0xb74e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb74e8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM VMA 0xb74ec
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb74f0
    c.literal(R::R1,0xb76e0)?;
    // ARM VMA 0xb74f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb74f8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb74fc
    c.literal(R::R2,0xb76d4)?;
    // ARM VMA 0xb7500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,152)?)?;
    // ARM VMA 0xb7504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7508
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // ARM VMA 0xb750c
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb7510
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,588)?)?;
    // ARM VMA 0xb7514
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7518
    c.b(C::Eq,0xb757c,false)?;
    // ARM VMA 0xb751c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,380)?)?;
    // ARM VMA 0xb7520
    c.literal(R::R1,0xb76e4)?;
    // ARM VMA 0xb7524
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7528
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb752c
    c.literal(R::R1,0xb76e0)?;
    // ARM VMA 0xb7530
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7534
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7538
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1408)?)?;
    // ARM VMA 0xb753c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7540
    c.b(C::Eq,0xb7554,false)?;
    // ARM VMA 0xb7544
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7548
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb754c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb7550
    c.b(C::Al,0xb7578,false)?;
    // ARM VMA 0xb7554
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,932)?)?;
    // ARM VMA 0xb7558
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb755c
    c.b(C::Ne,0xb756c,false)?;
    // ARM VMA 0xb7560
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,876)?)?;
    // ARM VMA 0xb7564
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7568
    c.b(C::Eq,0xb757c,false)?;
    // ARM VMA 0xb756c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7570
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb7574
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb7578
    c.b(C::Al,0xb87d8,true)?;
    // ARM VMA 0xb757c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,372)?)?;
    // ARM VMA 0xb7580
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb7584
    c.b(C::Eq,0xb7598,false)?;
    // ARM VMA 0xb7588
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // ARM VMA 0xb758c
    c.b(C::Al,0xd88a0,true)?;
    // ARM VMA 0xb7590
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7594
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,372)?)?;
    // ARM VMA 0xb7598
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,376)?)?;
    // ARM VMA 0xb759c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb75a0
    c.b(C::Eq,0xb75b4,false)?;
    // ARM VMA 0xb75a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // ARM VMA 0xb75a8
    c.b(C::Al,0xd88a0,true)?;
    // ARM VMA 0xb75ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb75b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,376)?)?;
    // ARM VMA 0xb75b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb75b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // ARM VMA 0xb75bc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,852)?)?;
    // ARM VMA 0xb75c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,588)?)?;
    // ARM VMA 0xb75c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,864)?)?;
    // ARM VMA 0xb75c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,912)?)?;
    // ARM VMA 0xb75cc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,900)?)?;
    // ARM VMA 0xb75d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // ARM VMA 0xb75d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM VMA 0xb75d8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,40)?)?;
    // ARM VMA 0xb75dc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // ARM VMA 0xb75e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,44)?)?;
    // ARM VMA 0xb75e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // ARM VMA 0xb75e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // ARM VMA 0xb75ec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // ARM VMA 0xb75f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2716)?)?;
    // ARM VMA 0xb75f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb75f8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb75fc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // ARM VMA 0xb7600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb7604
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // ARM VMA 0xb7608
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // ARM VMA 0xb760c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // ARM VMA 0xb7610
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // ARM VMA 0xb7614
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // ARM VMA 0xb7618
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,376)?)?;
    // ARM VMA 0xb761c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,564)?)?;
    // ARM VMA 0xb7620
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,136)?)?;
    // ARM VMA 0xb7624
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,192)?)?;
    // ARM VMA 0xb7628
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,16)?)?;
    // ARM VMA 0xb762c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(5)?.bit() | R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb7630
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // ARM VMA 0xb7634
    c.b(C::Al,0xd893c,true)?;
    // ARM VMA 0xb7638
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,2740)?)?;
    // ARM VMA 0xb763c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,204)?)?;
    // ARM VMA 0xb7640
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7644
    c.b(C::Eq,0xb7234,false)?;
    // ARM VMA 0xb7648
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb764c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7650
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7654
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(6)?,204)?)?;
    // ARM VMA 0xb7658
    c.b(C::Al,0xb7234,false)?;
    // ARM VMA 0xb765c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,932)?)?;
    // ARM VMA 0xb7660
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7664
    c.b(C::Ne,0xb7674,false)?;
    // ARM VMA 0xb7668
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,876)?)?;
    // ARM VMA 0xb766c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7670
    c.b(C::Eq,0xb7684,false)?;
    // ARM VMA 0xb7674
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7678
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb767c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb7680
    c.b(C::Al,0xb87d8,true)?;
    // ARM VMA 0xb7684
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,372)?)?;
    // ARM VMA 0xb7688
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb768c
    c.b(C::Eq,0xb76a0,false)?;
    // ARM VMA 0xb7690
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // ARM VMA 0xb7694
    c.b(C::Al,0xd88a0,true)?;
    // ARM VMA 0xb7698
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb769c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,372)?)?;
    // ARM VMA 0xb76a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,376)?)?;
    // ARM VMA 0xb76a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb76a8
    c.b(C::Eq,0xb76bc,false)?;
    // ARM VMA 0xb76ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // ARM VMA 0xb76b0
    c.b(C::Al,0xd88a0,true)?;
    // ARM VMA 0xb76b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb76b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,376)?)?;
    // ARM VMA 0xb76bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb76c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,852)?)?;
    // ARM VMA 0xb76c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,588)?)?;
    // ARM VMA 0xb76c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,864)?)?;
    // ARM VMA 0xb76cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb76d0
    c.b(C::Al,0xb7230,false)?;
    c.check(0xb76d4)?;
    for value in CODEC_CONTEXT_CLOSE_DIAGNOSTICS { c.emit(value)?; }
    c.check(0xb76f4)
}

const RESTART_CODEC_LOGS:[u32;5]=[
    0x0022_1007,0x000f_94f6,0x000f_9529,0x000e_2788,0x000f_9560,
];
const CODEC_EVENT_READY:u32=0x0022_1007;
const CODEC_EVENT_STREAM_ON:u32=0x0022_1003;
const CODEC_EVENT_STREAM_OFF:u32=0x0022_1004;
