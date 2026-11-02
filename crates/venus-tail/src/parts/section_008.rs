
/// Prepare the controller codec output descriptor: pack per-field mode
/// bits, fill a packed quantization table, and invoke the installed callback
/// with the finished record. Two constant global addresses follow the BX.
fn encode_codec_controller_output_record(c:&mut Code)->Result<(),String>{
    c.check(0xaa22c)?;
    // VMA 0xaa22c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xaa230
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xaa234
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // VMA 0xaa238
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xaa23c
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // VMA 0xaa240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // VMA 0xaa244
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1160)?)?;
    // VMA 0xaa248
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1164)?)?;
    // VMA 0xaa24c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,2444)?)?;
    // VMA 0xaa250
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // VMA 0xaa254
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,12)?)?;
    // VMA 0xaa258
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xaa25c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xaa260
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xaa264
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2452)?)?;
    // VMA 0xaa268
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2604)?)?;
    // VMA 0xaa26c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa270
    c.b(C::Ne,0xaa284,false)?;
    // VMA 0xaa274
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // VMA 0xaa278
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1048576)?)?;
    // VMA 0xaa27c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,42)?)?;
    // VMA 0xaa280
    c.b(C::Eq,0xaa2bc,false)?;
    // VMA 0xaa284
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,616)?)?;
    // VMA 0xaa288
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(4)?,R::R1)?)?;
    // VMA 0xaa28c
    c.b(C::Al,0x81838,true)?;
    // VMA 0xaa290
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,167)?)?;
    // VMA 0xaa294
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    // VMA 0xaa298
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,261120)?)?;
    // VMA 0xaa29c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,786432)?)?;
    // VMA 0xaa2a0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,7,false)?)?;
    // VMA 0xaa2a4
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // VMA 0xaa2a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xaa2ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // VMA 0xaa2b0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsr,3,false)?)?;
    // VMA 0xaa2b4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa2b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,1048576)?)?;
    // VMA 0xaa2bc
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,16)?)?;
    // VMA 0xaa2c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24576)?)?;
    // VMA 0xaa2c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,532676608)?)?;
    // VMA 0xaa2c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(5)?,2880)?)?;
    // VMA 0xaa2cc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // VMA 0xaa2d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,32)?)?;
    // VMA 0xaa2d4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2468)?)?;
    // VMA 0xaa2d8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa2dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,8)?)?;
    // VMA 0xaa2e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xaa2e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2468)?)?;
    // VMA 0xaa2e8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,2504)?)?;
    // VMA 0xaa2ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4096)?)?;
    // VMA 0xaa2f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2508)?)?;
    // VMA 0xaa2f4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2686)?)?;
    // VMA 0xaa2f8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,12,false)?)?;
    // VMA 0xaa2fc
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2734)?)?;
    // VMA 0xaa300
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,13,false)?)?;
    // VMA 0xaa304
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // VMA 0xaa308
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xaa30c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xaa310
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xaa314
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3840)?)?;
    // VMA 0xaa318
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa31c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2735)?)?;
    // VMA 0xaa320
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,8,false)?)?;
    // VMA 0xaa324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,240)?)?;
    // VMA 0xaa328
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa32c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2736)?)?;
    // VMA 0xaa330
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,4,false)?)?;
    // VMA 0xaa334
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // VMA 0xaa338
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa33c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2724)?)?;
    // VMA 0xaa340
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xaa344
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa348
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2608)?)?;
    // VMA 0xaa34c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,3)?)?;
    // VMA 0xaa350
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xaa354
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa358
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,8388608)?)?;
    // VMA 0xaa35c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // VMA 0xaa360
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,2728)?)?;
    // VMA 0xaa364
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2688)?)?;
    // VMA 0xaa368
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2687)?)?;
    // VMA 0xaa36c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,2731)?)?;
    // VMA 0xaa370
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,22,false)?)?;
    // VMA 0xaa374
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2031616)?)?;
    // VMA 0xaa378
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xaa37c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,2031616)?)?;
    // VMA 0xaa380
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,63488)?)?;
    // VMA 0xaa388
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,11,false)?)?;
    // VMA 0xaa38c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,63488)?)?;
    // VMA 0xaa390
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa394
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // VMA 0xaa398
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,3,false)?)?;
    // VMA 0xaa39c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa3a0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2732)?)?;
    // VMA 0xaa3a4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,7)?)?;
    // VMA 0xaa3a8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa3ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3932160)?)?;
    // VMA 0xaa3b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // VMA 0xaa3b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8388608)?)?;
    // VMA 0xaa3b8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // VMA 0xaa3bc
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(5)?,2725)?)?;
    // VMA 0xaa3c0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,2723)?)?;
    // VMA 0xaa3c4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,2684)?)?;
    // VMA 0xaa3c8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,2685)?)?;
    // VMA 0xaa3cc
    c.emit(arm32::data_register(C::Ne,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,23,false)?)?;
    // VMA 0xaa3d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2727)?)?;
    // VMA 0xaa3d4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,18,false)?)?;
    // VMA 0xaa3d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2601)?)?;
    // VMA 0xaa3dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4194304)?)?;
    // VMA 0xaa3e0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,22,false)?)?;
    // VMA 0xaa3e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    // VMA 0xaa3e8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa3ec
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa3f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,131072)?)?;
    // VMA 0xaa3f4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,17,false)?)?;
    // VMA 0xaa3f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // VMA 0xaa3fc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65536)?)?;
    // VMA 0xaa404
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xaa408
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa40c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32768)?)?;
    // VMA 0xaa410
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa414
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa418
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2737)?)?;
    // VMA 0xaa41c
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,14,false)?)?;
    // VMA 0xaa420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15872)?)?;
    // VMA 0xaa424
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa428
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2730)?)?;
    // VMA 0xaa42c
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,9,false)?)?;
    // VMA 0xaa430
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,496)?)?;
    // VMA 0xaa434
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa438
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2729)?)?;
    // VMA 0xaa43c
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,4,false)?)?;
    // VMA 0xaa440
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa444
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2726)?)?;
    // VMA 0xaa448
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,15)?)?;
    // VMA 0xaa44c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa450
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,20)?)?;
    // VMA 0xaa454
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,24)?)?;
    // VMA 0xaa458
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2512)?)?;
    // VMA 0xaa45c
    c.literal(R::R0,0xaa6e8)?;
    // VMA 0xaa460
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,12)?)?;
    // VMA 0xaa464
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,8)?)?;
    // VMA 0xaa468
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xaa46c
    c.emit(arm32::mov_register(C::Al,R::R0,R::R12))?;
    // VMA 0xaa470
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,4)?)?;
    // VMA 0xaa474
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xaa478
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // VMA 0xaa47c
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(5)?))?;
    // VMA 0xaa480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // VMA 0xaa484
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(9)?,R::new(10)?,32)?)?;
    // VMA 0xaa488
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xaa48c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,13)?)?;
    // VMA 0xaa490
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,9)?)?;
    // VMA 0xaa494
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,5)?)?;
    // VMA 0xaa498
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,3)?)?;
    // VMA 0xaa49c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,2)?)?;
    // VMA 0xaa4a0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,1)?)?;
    // VMA 0xaa4a4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // VMA 0xaa4a8
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(9)?,R::R1,12)?)?;
    // VMA 0xaa4ac
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,3)?)?;
    // VMA 0xaa4b0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,2)?)?;
    // VMA 0xaa4b4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xaa4b8
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(9)?,R::R1,8)?)?;
    // VMA 0xaa4bc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,3)?)?;
    // VMA 0xaa4c0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,2)?)?;
    // VMA 0xaa4c4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xaa4c8
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(9)?,R::R1,4)?)?;
    // VMA 0xaa4cc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,3)?)?;
    // VMA 0xaa4d0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,2)?)?;
    // VMA 0xaa4d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,984)?)?;
    // VMA 0xaa4d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // VMA 0xaa4dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,2048)?)?;
    // VMA 0xaa4e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,2896)?)?;
    // VMA 0xaa4e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // VMA 0xaa4e8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa4ec
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // VMA 0xaa4f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // VMA 0xaa4f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xaa4f8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,0)?)?;
    // VMA 0xaa4fc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(4)?,3)?)?;
    // VMA 0xaa500
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xaa504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xaa508
    c.b(C::Lt,0xaa5b8,false)?;
    // VMA 0xaa50c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,32)?)?;
    // VMA 0xaa510
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xaa514
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // VMA 0xaa518
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,0)?)?;
    // VMA 0xaa51c
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::R2.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xaa520
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,112)?)?;
    // VMA 0xaa524
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(6)?))?;
    // VMA 0xaa528
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,12)?)?;
    // VMA 0xaa52c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4)?)?;
    // VMA 0xaa530
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,16)?)?;
    // VMA 0xaa534
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // VMA 0xaa538
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa53c
    c.emit(arm32::data_register(C::Gt,Op::Add,R::new(6)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa540
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xaa544
    c.emit(arm32::load_signed_byte(C::Al,R::new(4)?,R::new(6)?,0)?)?;
    // VMA 0xaa548
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // VMA 0xaa54c
    c.emit(arm32::data_register(C::Gt,Op::Add,R::new(6)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa550
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::new(6)?,0)?)?;
    // VMA 0xaa554
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa558
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R2,R::R2,R::R2,Shift::Asr,31,false)?)?;
    // VMA 0xaa55c
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::R2,Shift::Lsl,4)?)?;
    // VMA 0xaa560
    c.emit(arm32::data_register(C::Gt,Op::Orr,R::new(6)?,R::new(4)?,R::R2,Shift::Lsl,4,false)?)?;
    // VMA 0xaa564
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa568
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // VMA 0xaa56c
    c.emit(arm32::data_register(C::Gt,Op::Add,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa570
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa574
    c.emit(arm32::data_register(C::Gt,Op::Add,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa578
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R2,0)?)?;
    // VMA 0xaa57c
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::new(3)?,0)?)?;
    // VMA 0xaa580
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // VMA 0xaa584
    c.emit(arm32::transfer_byte_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(3)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xaa588
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::LR,2)?)?;
    // VMA 0xaa58c
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::R1,R::R1,Shift::Asr,31,false)?)?;
    // VMA 0xaa590
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R2,R::R2,R::R2,Shift::Asr,31,false)?)?;
    // VMA 0xaa594
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(6)?,R::R2,Shift::Lsl,8,false)?)?;
    // VMA 0xaa598
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // VMA 0xaa59c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,12,false)?)?;
    // VMA 0xaa5a0
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,8)?)?;
    // VMA 0xaa5a4
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,1)?)?;
    // VMA 0xaa5a8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,0)?)?;
    // VMA 0xaa5ac
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(4)?,3)?)?;
    // VMA 0xaa5b0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa5b4
    c.b(C::Lt,0xaa518,false)?;
    // VMA 0xaa5b8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xaa5bc
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(7)?,R::new(4)?,3,true)?)?;
    // VMA 0xaa5c0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R1,8)?)?;
    // VMA 0xaa5c4
    c.b(C::Eq,0xaa630,false)?;
    // VMA 0xaa5c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,24)?)?;
    // VMA 0xaa5cc
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // VMA 0xaa5d0
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(5)?,Shift::Lsl,2)?)?;
    // VMA 0xaa5d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xaa5d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // VMA 0xaa5dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xaa5e0
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,32)?)?;
    // VMA 0xaa5e4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa5e8
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xaa5ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,1)?)?;
    // VMA 0xaa5f0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xaa5f4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaa5f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa5fc
    c.emit(arm32::data_register(C::Gt,Op::Add,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa600
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa604
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R2,112)?)?;
    // VMA 0xaa608
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::R1,R::R1,Shift::Asr,31,false)?)?;
    // VMA 0xaa60c
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::new(8)?,R::new(8)?,R::R1,Shift::Lsl,R::new(10)?,false)?)?;
    // VMA 0xaa610
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,4)?)?;
    // VMA 0xaa614
    c.b(C::Lt,0xaa5e4,false)?;
    // VMA 0xaa618
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::LR,1)?)?;
    // VMA 0xaa61c
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(8)?,Shift::Lsr,8)?)?;
    // VMA 0xaa620
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(8)?,R::new(3)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xaa624
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaa628
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,20)?)?;
    // VMA 0xaa62c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // VMA 0xaa630
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xaa634
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xaa638
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // VMA 0xaa63c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // VMA 0xaa640
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,2)?)?;
    // VMA 0xaa644
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(10)?,R::new(10)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,8)?)?;
    // VMA 0xaa64c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,68)?)?;
    // VMA 0xaa650
    c.b(C::Ne,0xaa4e4,false)?;
    // VMA 0xaa654
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // VMA 0xaa658
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // VMA 0xaa65c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2686)?)?;
    // VMA 0xaa660
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa664
    c.b(C::Eq,0xaa6b0,false)?;
    // VMA 0xaa668
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2608)?)?;
    // VMA 0xaa66c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xaa670
    c.b(C::Eq,0xaa6a0,false)?;
    // VMA 0xaa674
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa678
    c.b(C::Ne,0xaa6b0,false)?;
    // VMA 0xaa67c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2725)?)?;
    // VMA 0xaa680
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xaa684
    c.b(C::Hi,0xaa6b0,false)?;
    // VMA 0xaa688
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2726)?)?;
    // VMA 0xaa68c
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,1)?)?;
    // VMA 0xaa690
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // VMA 0xaa694
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xaa698
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xaa69c
    c.b(C::Al,0xaa6a8,false)?;
    // VMA 0xaa6a0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2726)?)?;
    // VMA 0xaa6a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xaa6a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2896)?)?;
    // VMA 0xaa6ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,136)?)?;
    // VMA 0xaa6b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,132)?)?;
    // VMA 0xaa6b4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xaa6b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa6bc
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::R1,48)?)?;
    // VMA 0xaa6c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,132)?)?;
    // VMA 0xaa6c4
    c.b(C::Al,0x20d0,true)?;
    // VMA 0xaa6c8
    c.literal(R::R0,0xaa6ec)?;
    // VMA 0xaa6cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2608)?)?;
    // VMA 0xaa6d0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xaa6d4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xaa6d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xaa6dc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xaa6e0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xaa6e4
    c.emit(arm32::bx(C::Al,R::new(3)?))?;
    c.check(0xaa6e8)
}

const CODEC_OUTPUT_GLOBALS:[u32;2]=[
    0x0010_dd64, // shared codec state
    0x0010_dd74, // controller event listener
];
