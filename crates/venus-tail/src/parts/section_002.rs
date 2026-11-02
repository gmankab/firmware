

/// Initialize controller buffers, populate per-codec callback function tables,
/// and generate diagnostic error branches for allocation failures. The
/// controller state stores registers and addresses into mapped data structs.
/// Runtime `0xa83f8–0xa8bc4`; the subsequent three pools are literal data.
fn initialize_codec_resource_layout(c:&mut Code)->Result<(),String>{
    c.check(0xa83f8)?;
    // VMA 0xa83f8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa83fc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xa8400
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xa8404
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // VMA 0xa8408
    c.literal(R::R2,0xa8bc4)?;
    // VMA 0xa840c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xa8410
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // VMA 0xa8414
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,250)?)?;
    // VMA 0xa8418
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xa841c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    // VMA 0xa8420
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8424
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa8428
    c.b(C::Al,0x26754,true)?;
    // VMA 0xa842c
    c.literal(R::R1,0xa8bc8)?;
    // VMA 0xa8430
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xa8434
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xa8438
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xa843c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8440
    c.literal(R::R1,0xa8bcc)?;
    // VMA 0xa8444
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xa8448
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa844c
    c.literal(R::R0,0xa8bd0)?;
    // VMA 0xa8450
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa8454
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // VMA 0xa8458
    c.b(C::Ne,0xa8744,false)?;
    // VMA 0xa845c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4032)?)?;
    // VMA 0xa8460
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8464
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // VMA 0xa8468
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // VMA 0xa846c
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // VMA 0xa8470
    c.b(C::Eq,0xa8754,false)?;
    // VMA 0xa8474
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // VMA 0xa8478
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa847c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4032)?)?;
    // VMA 0xa8480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa8484
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa8488
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa848c
    c.literal(R::R1,0xa8bdc)?;
    // VMA 0xa8490
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa8494
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xa8498
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // VMA 0xa849c
    c.b(C::Al,0x804fc,true)?;
    // VMA 0xa84a0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,2147483648)?)?;
    // VMA 0xa84a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // VMA 0xa84a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2488)?)?;
    // VMA 0xa84ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa84b0
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,2560)?)?;
    // VMA 0xa84b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,8)?)?;
    // VMA 0xa84b8
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::new(6)?.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa84bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,700)?)?;
    // VMA 0xa84c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,696)?)?;
    // VMA 0xa84c4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,16)?)?;
    // VMA 0xa84c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,720)?)?;
    // VMA 0xa84cc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,684)?)?;
    // VMA 0xa84d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,688)?)?;
    // VMA 0xa84d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xa84d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,20)?)?;
    // VMA 0xa84dc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // VMA 0xa84e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,68)?)?;
    // VMA 0xa84e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,592)?)?;
    // VMA 0xa84e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xa84ec
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,72)?)?;
    // VMA 0xa84f0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,96)?)?;
    // VMA 0xa84f4
    c.literal(R::R2,0xa8be0)?;
    // VMA 0xa84f8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,588)?)?;
    // VMA 0xa84fc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,584)?)?;
    // VMA 0xa8500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xa8504
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,24)?)?;
    // VMA 0xa8508
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,28)?)?;
    // VMA 0xa850c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,48)?)?;
    // VMA 0xa8510
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,712)?)?;
    // VMA 0xa8514
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,716)?)?;
    // VMA 0xa8518
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,692)?)?;
    // VMA 0xa851c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,120)?)?;
    // VMA 0xa8520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xa8524
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(5)?,724)?)?;
    // VMA 0xa8528
    c.b(C::Al,0xddad4,true)?;
    // VMA 0xa852c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,168)?)?;
    // VMA 0xa8530
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xa8534
    c.b(C::Al,0xddad4,true)?;
    // VMA 0xa8538
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // VMA 0xa853c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa8540
    c.b(C::Al,0x26d0,true)?;
    // VMA 0xa8544
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // VMA 0xa8548
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(5)?,728)?)?;
    // VMA 0xa854c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,732)?)?;
    // VMA 0xa8550
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,740)?)?;
    // VMA 0xa8554
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,744)?)?;
    // VMA 0xa8558
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,752)?)?;
    // VMA 0xa855c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,748)?)?;
    // VMA 0xa8560
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,764)?)?;
    // VMA 0xa8564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // VMA 0xa856c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,768)?)?;
    // VMA 0xa8570
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,772)?)?;
    // VMA 0xa8574
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,332)?)?;
    // VMA 0xa8578
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,944)?)?;
    // VMA 0xa857c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,2424)?)?;
    // VMA 0xa8580
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,568)?)?;
    // VMA 0xa8584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // VMA 0xa8588
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa858c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,0)?)?;
    // VMA 0xa8590
    c.literal(R::R2,0xa8be4)?;
    // VMA 0xa8594
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,780)?)?;
    // VMA 0xa8598
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa859c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,4004)?)?;
    // VMA 0xa85a0
    c.b(C::Al,0x30d0,true)?;
    // VMA 0xa85a4
    c.literal(R::R2,0xa8be8)?;
    // VMA 0xa85a8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,788)?)?;
    // VMA 0xa85ac
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa85b0
    c.b(C::Al,0x30d0,true)?;
    // VMA 0xa85b4
    c.literal(R::R2,0xa8bec)?;
    // VMA 0xa85b8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,796)?)?;
    // VMA 0xa85bc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa85c0
    c.b(C::Al,0x30d0,true)?;
    // VMA 0xa85c4
    c.literal(R::R2,0xa8bf0)?;
    // VMA 0xa85c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,804)?)?;
    // VMA 0xa85cc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa85d0
    c.b(C::Al,0x30d0,true)?;
    // VMA 0xa85d4
    c.literal(R::R0,0xa8bf4)?;
    // VMA 0xa85d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,844)?)?;
    // VMA 0xa85dc
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xa85e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa85e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,864)?)?;
    // VMA 0xa85e8
    c.b(C::Al,0x2aa0c,true)?;
    // VMA 0xa85ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(9)?,1168)?)?;
    // VMA 0xa85f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,100)?)?;
    // VMA 0xa85f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1024)?)?;
    // VMA 0xa85f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa85fc
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // VMA 0xa8600
    c.b(C::Al,0x7e544,true)?;
    // VMA 0xa8604
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1168)?)?;
    // VMA 0xa8608
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(9)?,948)?)?;
    // VMA 0xa860c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8610
    c.b(C::Eq,0xa877c,false)?;
    // VMA 0xa8614
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // VMA 0xa8618
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1152)?)?;
    // VMA 0xa861c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa8620
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa8624
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,308)?)?;
    // VMA 0xa8628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1152)?)?;
    // VMA 0xa862c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,352)?)?;
    // VMA 0xa8630
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,148)?)?;
    // VMA 0xa8634
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,1024)?)?;
    // VMA 0xa8638
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // VMA 0xa863c
    c.b(C::Al,0x7e544,true)?;
    // VMA 0xa8640
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1172)?)?;
    // VMA 0xa8644
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8648
    c.b(C::Eq,0xa878c,false)?;
    // VMA 0xa864c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,120)?)?;
    // VMA 0xa8650
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,1024)?)?;
    // VMA 0xa8654
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa8658
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xa865c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8660
    c.b(C::Ne,0xa87a0,false)?;
    // VMA 0xa8664
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,8)?)?;
    // VMA 0xa8668
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xa866c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8670
    c.b(C::Ne,0xa87b4,false)?;
    // VMA 0xa8674
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xa8678
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // VMA 0xa867c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // VMA 0xa8680
    c.b(C::Ls,0xa8654,false)?;
    // VMA 0xa8684
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,136)?)?;
    // VMA 0xa8688
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xa868c
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xa8690
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8694
    c.b(C::Ne,0xa87c4,false)?;
    // VMA 0xa8698
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,140)?)?;
    // VMA 0xa869c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xa86a0
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xa86a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa86a8
    c.b(C::Ne,0xa87d4,false)?;
    // VMA 0xa86ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,972)?)?;
    // VMA 0xa86b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    // VMA 0xa86b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa86b8
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa86bc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa86c0
    c.b(C::Ne,0xa87e4,false)?;
    // VMA 0xa86c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,976)?)?;
    // VMA 0xa86c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xa86cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa86d0
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa86d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa86d8
    c.b(C::Ne,0xa87f4,false)?;
    // VMA 0xa86dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,980)?)?;
    // VMA 0xa86e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xa86e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa86e8
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa86ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa86f0
    c.b(C::Ne,0xa8804,false)?;
    // VMA 0xa86f4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,984)?)?;
    // VMA 0xa86f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xa86fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa8700
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa8704
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8708
    c.b(C::Ne,0xa8814,false)?;
    // VMA 0xa870c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,988)?)?;
    // VMA 0xa8710
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xa8714
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa8718
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa871c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8720
    c.b(C::Ne,0xa8824,false)?;
    // VMA 0xa8724
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,992)?)?;
    // VMA 0xa8728
    c.b(C::Al,0x2b8c,true)?;
    // VMA 0xa872c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8730
    c.b(C::Eq,0xa8948,false)?;
    // VMA 0xa8734
    c.literal(R::R1,0xa8c1c)?;
    // VMA 0xa8738
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa873c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,217)?)?;
    // VMA 0xa8740
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8744
    c.literal(R::R1,0xa8bd4)?;
    // VMA 0xa8748
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa874c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa8750
    c.b(C::Al,0xa8760,false)?;
    // VMA 0xa8754
    c.literal(R::R1,0xa8cb8)?;
    // VMA 0xa8758
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa875c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    // VMA 0xa8760
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2560)?)?;
    // VMA 0xa8764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8768
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa876c
    c.literal(R::R1,0xa8bcc)?;
    // VMA 0xa8770
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8774
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8778
    c.b(C::Al,0xa893c,false)?;
    // VMA 0xa877c
    c.literal(R::R1,0xa8cb4)?;
    // VMA 0xa8780
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,123)?)?;
    // VMA 0xa8788
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa878c
    c.literal(R::R1,0xa8cb0)?;
    // VMA 0xa8790
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8794
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,648)?)?;
    // VMA 0xa8798
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xa879c
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa87a0
    c.literal(R::R1,0xa8bf8)?;
    // VMA 0xa87a4
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa87ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2704)?)?;
    // VMA 0xa87b0
    c.b(C::Al,0xa8838,false)?;
    // VMA 0xa87b4
    c.literal(R::R1,0xa8bfc)?;
    // VMA 0xa87b8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,150)?)?;
    // VMA 0xa87c0
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa87c4
    c.literal(R::R1,0xa8c00)?;
    // VMA 0xa87c8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,157)?)?;
    // VMA 0xa87d0
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa87d4
    c.literal(R::R1,0xa8c04)?;
    // VMA 0xa87d8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,163)?)?;
    // VMA 0xa87e0
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa87e4
    c.literal(R::R1,0xa8c08)?;
    // VMA 0xa87e8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,171)?)?;
    // VMA 0xa87f0
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa87f4
    c.literal(R::R1,0xa8c0c)?;
    // VMA 0xa87f8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,179)?)?;
    // VMA 0xa8800
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8804
    c.literal(R::R1,0xa8c10)?;
    // VMA 0xa8808
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa880c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,195)?)?;
    // VMA 0xa8810
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8814
    c.literal(R::R1,0xa8c14)?;
    // VMA 0xa8818
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa881c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,203)?)?;
    // VMA 0xa8820
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8824
    c.literal(R::R1,0xa8c18)?;
    // VMA 0xa8828
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa882c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,211)?)?;
    // VMA 0xa8830
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2560)?)?;
    // VMA 0xa8834
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8838
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa883c
    c.literal(R::R1,0xa8bcc)?;
    // VMA 0xa8840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8844
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8848
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,840)?)?;
    // VMA 0xa884c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // VMA 0xa8850
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8854
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,836)?)?;
    // VMA 0xa8858
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // VMA 0xa885c
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8860
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,2880)?)?;
    // VMA 0xa8864
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8868
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,900)?)?;
    // VMA 0xa886c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // VMA 0xa8870
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8874
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,3968)?)?;
    // VMA 0xa8878
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa887c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,952)?)?;
    // VMA 0xa8880
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8884
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa8888
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa888c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,992)?)?;
    // VMA 0xa8890
    c.b(C::Al,0x2bcc,true)?;
    // VMA 0xa8894
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,984)?)?;
    // VMA 0xa8898
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa889c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,988)?)?;
    // VMA 0xa88a0
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa88a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,980)?)?;
    // VMA 0xa88a8
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa88ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,976)?)?;
    // VMA 0xa88b0
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa88b4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,972)?)?;
    // VMA 0xa88b8
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa88bc
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // VMA 0xa88c0
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,1164)?)?;
    // VMA 0xa88c4
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xa88c8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa88cc
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xa88d0
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,1160)?)?;
    // VMA 0xa88d4
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xa88d8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa88dc
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xa88e0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(5)?,8)?)?;
    // VMA 0xa88e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa88e8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // VMA 0xa88ec
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // VMA 0xa88f0
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,-8)?)?;
    // VMA 0xa88f4
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xa88f8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa88fc
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xa8900
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(7)?,4)?)?;
    // VMA 0xa8904
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xa8908
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa890c
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xa8910
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xa8914
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(7)?))?;
    // VMA 0xa8918
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // VMA 0xa891c
    c.b(C::Ne,0xa88ec,false)?;
    // VMA 0xa8920
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,148)?)?;
    // VMA 0xa8924
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xa8928
    c.b(C::Al,0x7e7d4,true)?;
    // VMA 0xa892c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xa8930
    c.b(C::Al,0x7e7d4,true)?;
    // VMA 0xa8934
    c.emit(arm32::mov_register(C::Al,R::R0,R::SP))?;
    // VMA 0xa8938
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa893c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8940
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xa8944
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa8948
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1000)?)?;
    // VMA 0xa894c
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8950
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8954
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xa8958
    c.b(C::Eq,0xa8b24,false)?;
    // VMA 0xa895c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1280)?)?;
    // VMA 0xa8960
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8964
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8968
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,952)?)?;
    // VMA 0xa896c
    c.b(C::Eq,0xa8b38,false)?;
    // VMA 0xa8970
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,256)?)?;
    // VMA 0xa8974
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,512)?)?;
    // VMA 0xa8978
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,768)?)?;
    // VMA 0xa897c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(5)?,956)?)?;
    // VMA 0xa8980
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xa8984
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R1.bit()|R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa8988
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,968)?)?;
    // VMA 0xa898c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8990
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1188)?)?;
    // VMA 0xa8994
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1152)?)?;
    // VMA 0xa8998
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa899c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa89a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,3968)?)?;
    // VMA 0xa89a4
    c.b(C::Eq,0xa8b48,false)?;
    // VMA 0xa89a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa89ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1152)?)?;
    // VMA 0xa89b0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa89b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // VMA 0xa89b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xa89bc
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa89c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa89c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,3972)?)?;
    // VMA 0xa89c8
    c.b(C::Eq,0xa8b58,false)?;
    // VMA 0xa89cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,360)?)?;
    // VMA 0xa89d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa89d4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xa89d8
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa89dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,3968)?)?;
    // VMA 0xa89e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,3972)?)?;
    // VMA 0xa89e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1096)?)?;
    // VMA 0xa89e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2352)?)?;
    // VMA 0xa89ec
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa89f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa89f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,2880)?)?;
    // VMA 0xa89f8
    c.b(C::Eq,0xa8b6c,false)?;
    // VMA 0xa89fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8a00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2352)?)?;
    // VMA 0xa8a04
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8a08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1888)?)?;
    // VMA 0xa8a0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // VMA 0xa8a10
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8a14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8a18
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,2884)?)?;
    // VMA 0xa8a1c
    c.b(C::Eq,0xa8b80,false)?;
    // VMA 0xa8a20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1888)?)?;
    // VMA 0xa8a24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8a28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65536)?)?;
    // VMA 0xa8a2c
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8a30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1504)?)?;
    // VMA 0xa8a34
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xa8a38
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8a3c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8a40
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,2888)?)?;
    // VMA 0xa8a44
    c.b(C::Eq,0xa8b94,false)?;
    // VMA 0xa8a48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1504)?)?;
    // VMA 0xa8a4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8a50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // VMA 0xa8a54
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8a58
    c.literal(R::R0,0xa8c20)?;
    // VMA 0xa8a5c
    c.literal(R::R1,0xa8c24)?;
    // VMA 0xa8a60
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a64
    c.literal(R::R0,0xa8c28)?;
    // VMA 0xa8a68
    c.literal(R::R1,0xa8c2c)?;
    // VMA 0xa8a6c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a70
    c.literal(R::R0,0xa8c30)?;
    // VMA 0xa8a74
    c.literal(R::R1,0xa8c34)?;
    // VMA 0xa8a78
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a7c
    c.literal(R::R0,0xa8c38)?;
    // VMA 0xa8a80
    c.literal(R::R1,0xa8c3c)?;
    // VMA 0xa8a84
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a88
    c.literal(R::R0,0xa8c40)?;
    // VMA 0xa8a8c
    c.literal(R::R1,0xa8c44)?;
    // VMA 0xa8a90
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a94
    c.literal(R::R0,0xa8c48)?;
    // VMA 0xa8a98
    c.literal(R::R1,0xa8c4c)?;
    // VMA 0xa8a9c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8aa0
    c.literal(R::R0,0xa8c50)?;
    // VMA 0xa8aa4
    c.literal(R::R1,0xa8c54)?;
    // VMA 0xa8aa8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8aac
    c.literal(R::R0,0xa8c58)?;
    // VMA 0xa8ab0
    c.literal(R::R1,0xa8c5c)?;
    // VMA 0xa8ab4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ab8
    c.literal(R::R0,0xa8c60)?;
    // VMA 0xa8abc
    c.literal(R::R1,0xa8c64)?;
    // VMA 0xa8ac0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ac4
    c.literal(R::R0,0xa8c68)?;
    // VMA 0xa8ac8
    c.literal(R::R1,0xa8c6c)?;
    // VMA 0xa8acc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ad0
    c.literal(R::R0,0xa8c70)?;
    // VMA 0xa8ad4
    c.literal(R::R1,0xa8c74)?;
    // VMA 0xa8ad8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8adc
    c.literal(R::R0,0xa8c78)?;
    // VMA 0xa8ae0
    c.literal(R::R1,0xa8c7c)?;
    // VMA 0xa8ae4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ae8
    c.literal(R::R0,0xa8c80)?;
    // VMA 0xa8aec
    c.literal(R::R1,0xa8c84)?;
    // VMA 0xa8af0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8af4
    c.literal(R::R0,0xa8c88)?;
    // VMA 0xa8af8
    c.literal(R::R1,0xa8c8c)?;
    // VMA 0xa8afc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8b00
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa8b04
    c.b(C::Al,0xb02b0,true)?;
    // VMA 0xa8b08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8b0c
    c.b(C::Eq,0xa8ba8,false)?;
    // VMA 0xa8b10
    c.literal(R::R1,0xa8c90)?;
    // VMA 0xa8b14
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,67)?)?;
    // VMA 0xa8b1c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xa8b20
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b24
    c.literal(R::R1,0xa8cac)?;
    // VMA 0xa8b28
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,744)?)?;
    // VMA 0xa8b30
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xa8b34
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b38
    c.literal(R::R1,0xa8ca8)?;
    // VMA 0xa8b3c
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,239)?)?;
    // VMA 0xa8b44
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8b48
    c.literal(R::R1,0xa8ca4)?;
    // VMA 0xa8b4c
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,253)?)?;
    // VMA 0xa8b54
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8b58
    c.literal(R::R1,0xa8ca0)?;
    // VMA 0xa8b5c
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // VMA 0xa8b64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xa8b68
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b6c
    c.literal(R::R1,0xa8c9c)?;
    // VMA 0xa8b70
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,26)?)?;
    // VMA 0xa8b78
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xa8b7c
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b80
    c.literal(R::R1,0xa8c98)?;
    // VMA 0xa8b84
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,35)?)?;
    // VMA 0xa8b8c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xa8b90
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b94
    c.literal(R::R1,0xa8c94)?;
    // VMA 0xa8b98
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,812)?)?;
    // VMA 0xa8ba0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xa8ba4
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8ba8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,2444)?)?;
    // VMA 0xa8bac
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa8bb0
    c.b(C::Al,0xaf5c0,true)?;
    // VMA 0xa8bb4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa8bb8
    c.b(C::Al,0xad70c,true)?;
    // VMA 0xa8bbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8bc0
    c.b(C::Al,0xa8940,false)?;
    c.check(0xa8bc4)
}

// Runtime data 0xa8bc4..0xa8c24: 24 pointers and hardware constants.
