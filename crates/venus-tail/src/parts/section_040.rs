
/// Register a codec input/output buffer with the controller, recording mode
/// and plane metadata; original error branches enter shared runtime code.
fn register_controller_descriptor_and_planes(c:&mut Code)->Result<(),String>{
    c.check(0xb6654)?;
    // VMA 0xb6654
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb6658
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb665c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,44)?)?;
    // VMA 0xb6660
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb6664
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xb6668
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,4)?)?;
    // VMA 0xb666c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6670
    c.b(C::Al,0xd503c,true)?;
    // VMA 0xb6674
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xb6678
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,16)?)?;
    // VMA 0xb667c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,28)?)?;
    // VMA 0xb6680
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(6)?,32)?)?;
    // VMA 0xb6684
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xb6688
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xb668c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6690
    c.b(C::Al,0xd411c,true)?;
    // VMA 0xb6694
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb6698
    c.b(C::Eq,0xb67cc,false)?;
    // VMA 0xb669c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // VMA 0xb66a0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // VMA 0xb66a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xb66a8
    c.b(C::Cc,0xb67cc,false)?;
    // VMA 0xb66ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2&0xffffffff)?)?;
    // VMA 0xb66b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb66b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // VMA 0xb66b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,69632)?)?;
    // VMA 0xb66bc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // VMA 0xb66c0
    c.b(C::Eq,0xb6724,false)?;
    // VMA 0xb66c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // VMA 0xb66c8
    c.b(C::Ne,0xb677c,false)?;
    // VMA 0xb66cc
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,16)?)?;
    // VMA 0xb66d0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,292)?)?;
    // VMA 0xb66d4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb66d8
    c.b(C::Cs,0xb67d8,false)?;
    // VMA 0xb66dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28&0xffffffff)?)?;
    // VMA 0xb66e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xb66e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xb66e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // VMA 0xb66ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb66f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xb66f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2&0xffffffff)?)?;
    // VMA 0xb66f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xb66fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9&0xffffffff)?)?;
    // VMA 0xb6700
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xb6704
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // VMA 0xb6708
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,36)?)?;
    // VMA 0xb670c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // VMA 0xb6710
    c.b(C::Al,0x2a878,true)?;
    // VMA 0xb6714
    c.literal(R::R1,0xb68b4)?;
    // VMA 0xb6718
    c.literal(R::R2,0xb68a8)?;
    // VMA 0xb671c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,55&0xffffffff)?)?;
    // VMA 0xb6720
    c.b(C::Al,0xb67ac,false)?;
    // VMA 0xb6724
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,16)?)?;
    // VMA 0xb6728
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,348)?)?;
    // VMA 0xb672c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6730
    c.b(C::Cs,0xb6840,false)?;
    // VMA 0xb6734
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28&0xffffffff)?)?;
    // VMA 0xb6738
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xb673c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xb6740
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // VMA 0xb6744
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb6748
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xb674c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2&0xffffffff)?)?;
    // VMA 0xb6750
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xb6754
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9&0xffffffff)?)?;
    // VMA 0xb6758
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xb675c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // VMA 0xb6760
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,36)?)?;
    // VMA 0xb6764
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // VMA 0xb6768
    c.b(C::Al,0x2a878,true)?;
    // VMA 0xb676c
    c.literal(R::R1,0xb68a4)?;
    // VMA 0xb6770
    c.literal(R::R2,0xb68a8)?;
    // VMA 0xb6774
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,170&0xffffffff)?)?;
    // VMA 0xb6778
    c.b(C::Al,0xb67ac,false)?;
    // VMA 0xb677c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // VMA 0xb6780
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // VMA 0xb6784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6788
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb678c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,8)?)?;
    // VMA 0xb6790
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8&0xffffffff)?)?;
    // VMA 0xb6794
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2&0xffffffff)?)?;
    // VMA 0xb6798
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xb679c
    c.b(C::Al,0x2a990,true)?;
    // VMA 0xb67a0
    c.literal(R::R1,0xb68b8)?;
    // VMA 0xb67a4
    c.literal(R::R2,0xb68a8)?;
    // VMA 0xb67a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,209&0xffffffff)?)?;
    // VMA 0xb67ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xb67b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb67b4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb67b8
    c.literal(R::R1,0xb68ac)?;
    // VMA 0xb67bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb67c0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb67c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb67c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // VMA 0xb67cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb67d0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb67d4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb67d8
    c.literal(R::new(10)?,0xb68b0)?;
    // VMA 0xb67dc
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(6)?,32)?)?;
    // VMA 0xb67e0
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,316)?)?;
    // VMA 0xb67e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb67e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,580&0xffffffff)?)?;
    // VMA 0xb67ec
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,424)?)?;
    // VMA 0xb67f0
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // VMA 0xb67f4
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,2072)?)?;
    // VMA 0xb67f8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb67fc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,28)?)?;
    // VMA 0xb6800
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,308)?)?;
    // VMA 0xb6804
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6808
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb680c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,69&0xffffffff)?)?;
    // VMA 0xb6810
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // VMA 0xb6814
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6818
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xb681c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6820
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb6824
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xb6828
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xb682c
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // VMA 0xb6830
    c.b(C::Al,0xb68bc,true)?;
    // VMA 0xb6834
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6838
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,456)?)?;
    // VMA 0xb683c
    c.b(C::Al,0xb67cc,false)?;
    // VMA 0xb6840
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,0)?)?;
    // VMA 0xb6844
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,432)?)?;
    // VMA 0xb6848
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,2080)?)?;
    // VMA 0xb684c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,768)?)?;
    // VMA 0xb6850
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2308)?)?;
    // VMA 0xb6854
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // VMA 0xb6858
    c.b(C::Ne,0xb6880,false)?;
    // VMA 0xb685c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2176)?)?;
    // VMA 0xb6860
    c.literal(R::R0,0xb6898)?;
    // VMA 0xb6864
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb6868
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2180)?)?;
    // VMA 0xb686c
    c.literal(R::R0,0xb689c)?;
    // VMA 0xb6870
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb6874
    c.literal(R::R2,0xb68a0)?;
    // VMA 0xb6878
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb687c
    c.b(C::Al,0xb6884,false)?;
    // VMA 0xb6880
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,768)?)?;
    // VMA 0xb6884
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2184)?)?;
    // VMA 0xb6888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb688c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,2188)?)?;
    // VMA 0xb6890
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,460)?)?;
    // VMA 0xb6894
    c.b(C::Al,0xb67cc,false)?;
    c.check(0xb6898)?;
    const REGISTER_DESCRIPTOR_LITERALS:[u32;9]=[
        0x02fd_0300,0x03ba_db00,0xfc45_2500,
        0x000f_912d,0x000f_9112,0x000e_2788,
        0x000f_8fbc,0x000f_90c0,0x000f_9190,
    ];
    for word in REGISTER_DESCRIPTOR_LITERALS {c.emit(word)?;}
    c.check(0xb68bc)
}
