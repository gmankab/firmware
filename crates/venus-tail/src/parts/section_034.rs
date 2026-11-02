
/// Publish control flags and log failures in hardware channel state.
fn set_codec_channel_state(c:&mut Code)->Result<(),String>{
    c.check(0xb4488)?;
    // VMA 0xb4488
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb448c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb4490
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // VMA 0xb4494
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // VMA 0xb4498
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xb449c
    c.b(C::Al,0xb41c4,true)?;
    // VMA 0xb44a0
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(8)?,248)?)?;
    // VMA 0xb44a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb44a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb44ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb44b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xb44b4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xb44b8
    c.b(C::Al,0xb4518,false)?;
    // VMA 0xb44bc
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // VMA 0xb44c0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R1,1)?)?;
    // VMA 0xb44c4
    c.emit(arm32::transfer_word_postindexed_scaled_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,2)?)?;
    // VMA 0xb44c8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,176)?)?;
    // VMA 0xb44cc
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(5)?,R::new(3)?,R::R1)?)?;
    // VMA 0xb44d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,1)?)?;
    // VMA 0xb44d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb44d8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xb44dc
    c.b(C::Ne,0xb4518,false)?;
    // VMA 0xb44e0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,48)?)?;
    // VMA 0xb44e4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb44e8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xb44ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xb44f0
    c.b(C::Lt,0xb4518,false)?;
    // VMA 0xb44f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,36)?)?;
    // VMA 0xb44f8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,44)?)?;
    // VMA 0xb44fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // VMA 0xb4500
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4504
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb4508
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb450c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xb4510
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xb4514
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xb4518
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb451c
    c.b(C::Ge,0xb44bc,false)?;
    // VMA 0xb4520
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,1)?)?;
    // VMA 0xb4524
    c.b(C::Lt,0xb4674,false)?;
    // VMA 0xb4528
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,176)?)?;
    // VMA 0xb452c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,1)?)?;
    // VMA 0xb4530
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb4534
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // VMA 0xb4538
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xb453c
    c.b(C::Al,0xb457c,false)?;
    // VMA 0xb4540
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,260)?)?;
    // VMA 0xb4544
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // VMA 0xb4548
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb454c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4550
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4554
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb4558
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb455c
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4560
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,true)?)?;
    // VMA 0xb4564
    c.b(C::Eq,0xb4674,false)?;
    // VMA 0xb4568
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb456c
    c.b(C::Le,0xb4674,false)?;
    // VMA 0xb4570
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xb4574
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,0)?)?;
    // VMA 0xb4578
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xb457c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(9)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb4580
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // VMA 0xb4584
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::new(9)?,1)?)?;
    // VMA 0xb4588
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,1)?)?;
    // VMA 0xb458c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xb4590
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R2,0)?)?;
    // VMA 0xb4594
    c.emit(arm32::multiply_accumulate(C::Al,R::LR,R::R12,R::new(3)?,R::R1)?)?;
    // VMA 0xb4598
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::LR))?;
    // VMA 0xb459c
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::new(4)?,R::new(7)?,1)?)?;
    // VMA 0xb45a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb45a4
    c.b(C::Ne,0xb45c8,false)?;
    // VMA 0xb45a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,44)?)?;
    // VMA 0xb45ac
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb45b0
    c.emit(arm32::transfer_word(C::Le,true,R::R0,R::SP,8)?)?;
    // VMA 0xb45b4
    c.emit(arm32::data_immediate(C::Le,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb45b8
    c.b(C::Gt,0xb45e4,false)?;
    // VMA 0xb45bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,260)?)?;
    // VMA 0xb45c0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb45c4
    c.b(C::Gt,0xb45e4,false)?;
    // VMA 0xb45c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(9)?,1)?)?;
    // VMA 0xb45cc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(9)?,1)?)?;
    // VMA 0xb45d0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,4)?)?;
    // VMA 0xb45d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // VMA 0xb45d8
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // VMA 0xb45dc
    c.b(C::Gt,0xb4590,false)?;
    // VMA 0xb45e0
    c.b(C::Al,0xb4674,false)?;
    // VMA 0xb45e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,252)?)?;
    // VMA 0xb45e8
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R0,0)?)?;
    // VMA 0xb45ec
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb45f0
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R0,1)?)?;
    // VMA 0xb45f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,252)?)?;
    // VMA 0xb45f8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,4)?)?;
    // VMA 0xb45fc
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,0)?)?;
    // VMA 0xb4600
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb4604
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4608
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,1)?)?;
    // VMA 0xb460c
    c.emit(arm32::transfer_byte_postindexed_register(C::Al,false,R::new(5)?,R::R1,R::R2)?)?;
    // VMA 0xb4610
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,18)?)?;
    // VMA 0xb4614
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb4618
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb461c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,4)?)?;
    // VMA 0xb4620
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,48)?)?;
    // VMA 0xb4624
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb4628
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb462c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb4630
    c.b(C::Le,0xb4658,false)?;
    // VMA 0xb4634
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,44)?)?;
    // VMA 0xb4638
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,36)?)?;
    // VMA 0xb463c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4640
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // VMA 0xb4644
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4648
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // VMA 0xb464c
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xb4650
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xb4654
    c.b(C::Al,0xb465c,false)?;
    // VMA 0xb4658
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,44)?)?;
    // VMA 0xb465c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xb4660
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,1)?)?;
    // VMA 0xb4664
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4668
    c.b(C::Le,0xb4540,false)?;
    // VMA 0xb466c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xb4670
    c.b(C::Ge,0xb4570,false)?;
    // VMA 0xb4674
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xb4678
    c.b(C::Al,0xb41c4,true)?;
    // VMA 0xb467c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,248)?)?;
    // VMA 0xb4680
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,260)?)?;
    // VMA 0xb4684
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4688
    c.emit(arm32::data_immediate(C::Le,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb468c
    c.emit(arm32::transfer_multiple(C::Le,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb4690
    c.literal(R::R1,0xb46c0)?;
    // VMA 0xb4694
    c.literal(R::R2,0xb46c4)?;
    // VMA 0xb4698
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb469c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255&0xffffffff)?)?;
    // VMA 0xb46a0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb46a4
    c.literal(R::R1,0xb46c8)?;
    // VMA 0xb46a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb46ac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb46b0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xb46b4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb46b8
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb46bc
    c.b(C::Al,0xb46cc,false)?;
    c.check(0xb46c0)
}

const CHANNEL_STATE_REPORTS:[u32;3]=[
    0x000f_8cc9,0x000f_8d13,0x000e_2788,
];

/// Retire an indexed controller record and preserve the error-logging tail.
fn shift_codec_record_indices(c:&mut Code)->Result<(),String>{
    c.check(0xb46cc)?;
    // VMA 0xb46cc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb46d0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xb46d4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xb46d8
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xb46dc
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(8)?,248)?)?;
    // VMA 0xb46e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // VMA 0xb46e4
    c.b(C::Lt,0xb4764,false)?;
    // VMA 0xb46e8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xb46ec
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R12,1)?)?;
    // VMA 0xb46f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb46f4
    c.emit(arm32::transfer_word_postindexed_scaled_register(C::Al,true,R::R1,R::R0,R::R12,Shift::Lsl,2)?)?;
    // VMA 0xb46f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,176)?)?;
    // VMA 0xb46fc
    c.b(C::Al,0xb4710,false)?;
    // VMA 0xb4700
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,5)?)?;
    // VMA 0xb4704
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb4708
    c.b(C::Ne,0xb4748,false)?;
    // VMA 0xb470c
    c.b(C::Al,0xb4738,false)?;
    // VMA 0xb4710
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    // VMA 0xb4714
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xb4718
    c.emit(arm32::multiply_register(C::Al,R::new(4)?,R::R0,R::new(3)?)?)?;
    // VMA 0xb471c
    c.emit(arm32::transfer_byte_register_preindexed_writeback(C::Al,true,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xb4720
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb4724
    c.b(C::Eq,0xb4738,false)?;
    // VMA 0xb4728
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::R0,R::new(3)?,R::R1)?)?;
    // VMA 0xb472c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,8)?)?;
    // VMA 0xb4730
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb4734
    c.b(C::Eq,0xb4700,false)?;
    // VMA 0xb4738
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::R0,R::new(3)?,R::R1)?)?;
    // VMA 0xb473c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,4)?)?;
    // VMA 0xb4740
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb4744
    c.b(C::Eq,0xb47a4,false)?;
    // VMA 0xb4748
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,1)?)?;
    // VMA 0xb474c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(5)?,1)?)?;
    // VMA 0xb4750
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,4)?)?;
    // VMA 0xb4754
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // VMA 0xb4758
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb475c
    c.b(C::Gt,0xb4710,false)?;
    // VMA 0xb4760
    c.b(C::Al,0xb4768,false)?;
    // VMA 0xb4764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4768
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R12,Shift::Lsl,2,false)?)?;
    // VMA 0xb476c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R12,1)?)?;
    // VMA 0xb4770
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,176)?)?;
    // VMA 0xb4774
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,84&0xffffffff)?)?;
    // VMA 0xb4778
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // VMA 0xb477c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,1)?)?;
    // VMA 0xb4780
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb4784
    c.b(C::Lt,0xb47a4,false)?;
    // VMA 0xb4788
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // VMA 0xb478c
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R1,-4)?)?;
    // VMA 0xb4790
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(3)?,R::R0,R::R2)?)?;
    // VMA 0xb4794
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(5)?,1)?)?;
    // VMA 0xb4798
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,5)?)?;
    // VMA 0xb479c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb47a0
    c.b(C::Ne,0xb4778,false)?;
    // VMA 0xb47a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(5)?,1)?)?;
    // VMA 0xb47a8
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb47ac
    c.b(C::Eq,0xb4890,false)?;
    // VMA 0xb47b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xb47b4
    c.literal(R::R1,0xb48ac)?;
    // VMA 0xb47b8
    c.literal(R::R2,0xb48b0)?;
    // VMA 0xb47bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,49&0xffffffff)?)?;
    // VMA 0xb47c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // VMA 0xb47c4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xb47c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xb47cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb47d0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb47d4
    c.literal(R::R1,0xb48b4)?;
    // VMA 0xb47d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb47dc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb47e0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,1)?)?;
    // VMA 0xb47e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb47e8
    c.b(C::Ne,0xb482c,false)?;
    // VMA 0xb47ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,252)?)?;
    // VMA 0xb47f0
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R0,0)?)?;
    // VMA 0xb47f4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb47f8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(8)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb47fc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,180)?)?;
    // VMA 0xb4800
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1)?)?;
    // VMA 0xb4804
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,252)?)?;
    // VMA 0xb4808
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb480c
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,0)?)?;
    // VMA 0xb4810
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb4814
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4818
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // VMA 0xb481c
    c.emit(arm32::transfer_byte_postindexed_register(C::Al,false,R::new(3)?,R::R1,R::R2)?)?;
    // VMA 0xb4820
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,18)?)?;
    // VMA 0xb4824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb4828
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,1)?)?;
    // VMA 0xb482c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb4830
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb4834
    c.b(C::Eq,0xb4874,false)?;
    // VMA 0xb4838
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb483c
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb4840
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb4844
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,252)?)?;
    // VMA 0xb4848
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,180)?)?;
    // VMA 0xb484c
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,35)?)?;
    // VMA 0xb4850
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // VMA 0xb4854
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,35)?)?;
    // VMA 0xb4858
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb485c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,36)?)?;
    // VMA 0xb4860
    c.b(C::Al,0xb4874,false)?;
    // VMA 0xb4864
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb4868
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xb486c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,184)?)?;
    // VMA 0xb4870
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,180)?)?;
    // VMA 0xb4874
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,248)?)?;
    // VMA 0xb4878
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xb487c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4880
    c.b(C::Lt,0xb4864,false)?;
    // VMA 0xb4884
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,248)?)?;
    // VMA 0xb4888
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xb488c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb4890
    c.literal(R::R2,0xb48b8)?;
    // VMA 0xb4894
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,42&0xffffffff)?)?;
    // VMA 0xb4898
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb489c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb48a0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xb48a4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb48a8
    c.b(C::Al,0x17b0,false)?;
    c.check(0xb48ac)
}

const CODEC_RECORD_SHIFT_DIAGNOSTICS:[u32;4]=[
    0x000f_8d24,0x000f_8d52,0x000e_2788,0x000f_8c8d,
];

/// Validate and register a stream record with the checked buffer queue.
fn register_stream_record(c:&mut Code)->Result<(),String>{
    c.check(0xb48bc)?;
    // VMA 0xb48bc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb48c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xb48c4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xb48c8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb48cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xb48d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb48d4
    c.b(C::Ne,0xb48f8,false)?;
    // VMA 0xb48d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,7)?)?;
    // VMA 0xb48dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb48e0
    c.b(C::Ne,0xb48f8,false)?;
    // VMA 0xb48e4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,6)?)?;
    // VMA 0xb48e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb48ec
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(4)?,22)?)?;
    // VMA 0xb48f0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb48f4
    c.b(C::Eq,0xb4904,false)?;
    // VMA 0xb48f8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb48fc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xb4900
    c.b(C::Al,0xb42d0,true)?;
    // VMA 0xb4904
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,12)?)?;
    // VMA 0xb4908
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb490c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xb4910
    c.b(C::Al,0xb43c8,true)?;
    // VMA 0xb4914
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb4918
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xb491c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb4920
    c.b(C::Al,0xb4488,false)?;
    c.check(0xb4924)
}

/// Sum the selected codec plane sizes from its descriptor bounds.
fn sum_codec_plane_sizes(c:&mut Code)->Result<(),String>{
    c.check(0xb4924)?;
    // VMA 0xb4924
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R2,24)?)?;
    // VMA 0xb4928
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb492c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,184)?)?;
    // VMA 0xb4930
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R2,15)?)?;
    // VMA 0xb4934
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,16)?)?;
    // VMA 0xb4938
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,14)?)?;
    // VMA 0xb493c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4940
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4944
    c.emit(arm32::data_register(C::Al,Op::Cmn,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4948
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // VMA 0xb494c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4950
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,84)?)?;
    // VMA 0xb4954
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4958
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb495c
    c.emit(arm32::transfer_byte_register_offset(C::Al,true,R::R1,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4960
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb4964
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xb4968
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb496c
    c.b(C::Lt,0xb495c,false)?;
    // VMA 0xb4970
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xb4974)
}

/// Calculate the combined signed byte-offset corrections.
fn sum_codec_line_offsets(c:&mut Code)->Result<(),String>{
    c.check(0xb4974)?;
    // VMA 0xb4974
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,76&0xffffffff)?)?;
    // VMA 0xb4978
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,77&0xffffffff)?)?;
    // VMA 0xb497c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // VMA 0xb4980
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // VMA 0xb4984
    c.emit(arm32::load_signed_byte_register(C::Al,R::R1,R::R0,R::R1)?)?;
    // VMA 0xb4988
    c.emit(arm32::load_signed_byte_register(C::Al,R::R2,R::R0,R::R2)?)?;
    // VMA 0xb498c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4990
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,79&0xffffffff)?)?;
    // VMA 0xb4994
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // VMA 0xb4998
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::R0,R::R2)?)?;
    // VMA 0xb499c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb49a0
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xb49a4)
}
