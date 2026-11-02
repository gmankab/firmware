
/// Establish input-buffer command coordinates, derive controller dimensions,
/// and update paired memory fields. The original routine branches back into
/// an internal sizing loop for one codec subtype and emits six distinct
/// failure/diagnostic addresses after its return path.
fn prepare_codec_input_records_and_feedback(c:&mut Code)->Result<(),String>{
    c.check(0xb84fc)?;
    // ARM VMA 0xb84fc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb8500
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM VMA 0xb8504
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // ARM VMA 0xb8508
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb850c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // ARM VMA 0xb8510
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,360)?)?;
    // ARM VMA 0xb8514
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb8518
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // ARM VMA 0xb851c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,280)?)?;
    // ARM VMA 0xb8520
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2320)?)?;
    // ARM VMA 0xb8524
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2048)?)?;
    // ARM VMA 0xb8528
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,724)?)?;
    // ARM VMA 0xb852c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,20)?)?;
    // ARM VMA 0xb8530
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // ARM VMA 0xb8534
    c.b(C::Al,0x471c,true)?;
    // ARM VMA 0xb8538
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,992)?)?;
    // ARM VMA 0xb853c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,224)?)?;
    // ARM VMA 0xb8540
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,996)?)?;
    // ARM VMA 0xb8544
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,228)?)?;
    // ARM VMA 0xb8548
    c.b(C::Al,0x2f8d0,true)?;
    // ARM VMA 0xb854c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8550
    c.b(C::Ne,0xb85bc,false)?;
    // ARM VMA 0xb8554
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,20)?)?;
    // ARM VMA 0xb8558
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb855c
    c.b(C::Ne,0xb85bc,false)?;
    // ARM VMA 0xb8560
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,972)?)?;
    // ARM VMA 0xb8564
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8568
    c.b(C::Eq,0xb85bc,false)?;
    // ARM VMA 0xb856c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // ARM VMA 0xb8570
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,128)?)?;
    // ARM VMA 0xb8574
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // ARM VMA 0xb8578
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM VMA 0xb857c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // ARM VMA 0xb8580
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // ARM VMA 0xb8584
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // ARM VMA 0xb8588
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // ARM VMA 0xb858c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // ARM VMA 0xb8590
    c.b(C::Al,0x2a990,true)?;
    // ARM VMA 0xb8594
    c.literal(R::R1,0xb86b0)?;
    // ARM VMA 0xb8598
    c.literal(R::R2,0xb86b4)?;
    // ARM VMA 0xb859c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,191)?)?;
    // ARM VMA 0xb85a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb85a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM VMA 0xb85a8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb85ac
    c.literal(R::R1,0xb86b8)?;
    // ARM VMA 0xb85b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb85b4
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb85b8
    c.b(C::Al,0xb8670,false)?;
    // ARM VMA 0xb85bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2132)?)?;
    // ARM VMA 0xb85c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb85c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb85c8
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(5)?,552)?)?;
    // ARM VMA 0xb85cc
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb85d0
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,2120)?)?;
    // ARM VMA 0xb85d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb85d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,712)?)?;
    // ARM VMA 0xb85dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb85e0
    c.b(C::Ne,0xb85f0,false)?;
    // ARM VMA 0xb85e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,552)?)?;
    // ARM VMA 0xb85e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb85ec
    c.b(C::Eq,0xb867c,false)?;
    // ARM VMA 0xb85f0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,300)?)?;
    // ARM VMA 0xb85f4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,12)?)?;
    // ARM VMA 0xb85f8
    c.b(C::Al,0x80928,true)?;
    // ARM VMA 0xb85fc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8600
    c.b(C::Eq,0xb863c,false)?;
    // ARM VMA 0xb8604
    c.literal(R::R1,0xb86c0)?;
    // ARM VMA 0xb8608
    c.literal(R::R2,0xb86b4)?;
    // ARM VMA 0xb860c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,218)?)?;
    // ARM VMA 0xb8610
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb8614
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM VMA 0xb8618
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb861c
    c.literal(R::R1,0xb86b8)?;
    // ARM VMA 0xb8620
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb8624
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8628
    c.literal(R::R2,0xb86c4)?;
    // ARM VMA 0xb862c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,219)?)?;
    // ARM VMA 0xb8630
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8634
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // ARM VMA 0xb8638
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb863c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // ARM VMA 0xb8640
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xb8644
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM VMA 0xb8648
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,360)?)?;
    // ARM VMA 0xb864c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,116)?)?;
    // ARM VMA 0xb8650
    c.b(C::Al,0x80b1c,true)?;
    // ARM VMA 0xb8654
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8658
    c.b(C::Ne,0xb8664,false)?;
    // ARM VMA 0xb865c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // ARM VMA 0xb8660
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xb8664
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb8668
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb866c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // ARM VMA 0xb8670
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb8674
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // ARM VMA 0xb8678
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb867c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,484)?)?;
    // ARM VMA 0xb8680
    c.literal(R::R1,0xb86bc)?;
    // ARM VMA 0xb8684
    c.literal(R::R2,0xb86b4)?;
    // ARM VMA 0xb8688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,211)?)?;
    // ARM VMA 0xb868c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM VMA 0xb8690
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb8694
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,484)?)?;
    // ARM VMA 0xb8698
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb869c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb86a0
    c.literal(R::R1,0xb86b8)?;
    // ARM VMA 0xb86a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb86a8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb86ac
    c.b(C::Al,0xb85f0,false)?;
    c.check(0xb86b0)?;
    for addr in CODEC_INPUT_RECORD_WARNINGS {c.emit(addr)?;}
    c.check(0xb86c8)
}

/// Allocate and initialize an active codec stream record, copy input plane
/// control fields and record the buffer's admission status in the session.
fn initialize_buffer_subscription(c:&mut Code)->Result<(),String>{
    c.check(0xb86c8)?;
    // ARM VMA 0xb86c8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb86cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // ARM VMA 0xb86d0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // ARM VMA 0xb86d4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb86d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
    // ARM VMA 0xb86dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,12)?)?;
    // ARM VMA 0xb86e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,356)?)?;
    // ARM VMA 0xb86e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb86e8
    c.b(C::Al,0x7f974,true)?;
    // ARM VMA 0xb86ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,8)?)?;
    // ARM VMA 0xb86f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // ARM VMA 0xb86f4
    c.b(C::Eq,0xb8794,false)?;
    // ARM VMA 0xb86f8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,12)?)?;
    // ARM VMA 0xb86fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb8700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1536)?)?;
    // ARM VMA 0xb8704
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM VMA 0xb8708
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,4)?)?;
    // ARM VMA 0xb870c
    c.b(C::Al,0x31ec,true)?;
    // ARM VMA 0xb8710
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,204)?)?;
    // ARM VMA 0xb8714
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,532)?)?;
    // ARM VMA 0xb8718
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,520)?)?;
    // ARM VMA 0xb871c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,0)?)?;
    // ARM VMA 0xb8720
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // ARM VMA 0xb8724
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,504)?)?;
    // ARM VMA 0xb8728
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,564)?)?;
    // ARM VMA 0xb872c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,508)?)?;
    // ARM VMA 0xb8730
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,376)?)?;
    // ARM VMA 0xb8734
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,300)?)?;
    // ARM VMA 0xb8738
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // ARM VMA 0xb873c
    c.b(C::Al,0x80928,true)?;
    // ARM VMA 0xb8740
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8744
    c.b(C::Eq,0xb8780,false)?;
    // ARM VMA 0xb8748
    c.literal(R::R1,0xb87a4)?;
    // ARM VMA 0xb874c
    c.literal(R::R2,0xb87a8)?;
    // ARM VMA 0xb8750
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,27)?)?;
    // ARM VMA 0xb8754
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb8758
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM VMA 0xb875c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8760
    c.literal(R::R1,0xb87ac)?;
    // ARM VMA 0xb8764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb8768
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb876c
    c.literal(R::R2,0xb87b0)?;
    // ARM VMA 0xb8770
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // ARM VMA 0xb8774
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8778
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // ARM VMA 0xb877c
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb8780
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,28)?)?;
    // ARM VMA 0xb8784
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xb8788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // ARM VMA 0xb878c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb8790
    c.b(C::Al,0xb8798,false)?;
    // ARM VMA 0xb8794
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // ARM VMA 0xb8798
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb879c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // ARM VMA 0xb87a0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb87a4)?;
    for addr in CODEC_CHANNEL_SETUP_WARNINGS {c.emit(addr)?;}
    c.check(0xb87b4)
}

/// Read whether a controller frame record is in the single-buffer fallback
/// state, yielding an enum-like status without a stack frame.
fn inspect_codec_fallback_mode(c:&mut Code)->Result<(),String>{
    c.check(0xb87b4)?;
    // ARM VMA 0xb87b4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    // ARM VMA 0xb87b8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,504)?)?;
    // ARM VMA 0xb87bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM VMA 0xb87c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb87c4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // ARM VMA 0xb87c8
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,228)?)?;
    // ARM VMA 0xb87cc
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb87d0
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // ARM VMA 0xb87d4
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xb87d8)
}
