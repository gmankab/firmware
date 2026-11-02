
/// Continue the codec stream class dispatch through the original
/// error/retry paths and shared command-queue completion. This function
/// ends with eighteen explicitly classified controller globals and
/// diagnostic-address words, not executable opcodes.
fn finish_controller_stream_classification(c:&mut Code)->Result<(),String>{
    c.check(0xb5600)?;
    // VMA 0xb5600
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(4)?,2608)?)?;
    // VMA 0xb5604
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xb5608
    c.emit(arm32::transfer_byte(C::Eq,true,R::R1,R::new(6)?,5)?)?;
    // VMA 0xb560c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5610
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(4)?,2688)?)?;
    // VMA 0xb5614
    c.b(C::Al,0xb5750,false)?;
    // VMA 0xb5618
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb561c
    c.b(C::Eq,0xb5644,false)?;
    // VMA 0xb5620
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5624
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb5628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8&0xffffffff)?)?;
    // VMA 0xb562c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5630
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xb5634
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5638
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2740)?)?;
    // VMA 0xb563c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5640
    c.b(C::Cc,0xb562c,false)?;
    // VMA 0xb5644
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2452)?)?;
    // VMA 0xb5648
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb564c
    c.b(C::Ne,0xb5f74,false)?;
    // VMA 0xb5650
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3700)?)?;
    // VMA 0xb5654
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // VMA 0xb5658
    c.b(C::Al,0xb05bc,true)?;
    // VMA 0xb565c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5660
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5664
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,2564)?)?;
    // VMA 0xb5668
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,2560)?)?;
    // VMA 0xb566c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xb5670
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2444)?)?;
    // VMA 0xb5674
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,940)?)?;
    // VMA 0xb5678
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb567c
    c.b(C::Eq,0xb56dc,false)?;
    // VMA 0xb5680
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2564)?)?;
    // VMA 0xb5684
    c.literal(R::R1,0xb60dc)?;
    // VMA 0xb5688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb568c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5690
    c.literal(R::new(5)?,0xb60a8)?;
    // VMA 0xb5694
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5698
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xb569c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb56a0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb56a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb56a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb56ac
    c.b(C::Al,0xac9f0,true)?;
    // VMA 0xb56b0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2564)?)?;
    // VMA 0xb56b4
    c.literal(R::R1,0xb60e0)?;
    // VMA 0xb56b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb56bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb56c0
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R2,R::R2,1)?)?;
    // VMA 0xb56c4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2564)?)?;
    // VMA 0xb56c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb56cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb56d0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xb56d4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb56d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,940)?)?;
    // VMA 0xb56dc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2593)?)?;
    // VMA 0xb56e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb56e4
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(4)?,724)?)?;
    // VMA 0xb56e8
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb56ec
    c.b(C::Eq,0xb5f74,false)?;
    // VMA 0xb56f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,944)?)?;
    // VMA 0xb56f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb56f8
    c.b(C::Eq,0xb5f60,false)?;
    // VMA 0xb56fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5700
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,944)?)?;
    // VMA 0xb5704
    c.b(C::Al,0xb5f74,false)?;
    // VMA 0xb5708
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(4)?,2608)?)?;
    // VMA 0xb570c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xb5710
    c.b(C::Ne,0xb5750,false)?;
    // VMA 0xb5714
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // VMA 0xb5718
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb571c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb5720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5724
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5728
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb572c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb5730
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,240)?)?;
    // VMA 0xb5734
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2688)?)?;
    // VMA 0xb5738
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // VMA 0xb573c
    c.b(C::Cc,0xb5750,false)?;
    // VMA 0xb5740
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5744
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2688)?)?;
    // VMA 0xb5748
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,69&0xffffffff)?)?;
    // VMA 0xb574c
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5750
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5754
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xb5758
    c.b(C::Lt,0xb57c0,false)?;
    // VMA 0xb575c
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(6)?))?;
    // VMA 0xb5760
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R0,1)?)?;
    // VMA 0xb5764
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,31&0xffffffff)?)?;
    // VMA 0xb576c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5770
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::new(5)?,Shift::Lsl,R::new(6)?,false)?)?;
    // VMA 0xb5774
    c.b(C::Ne,0xb5788,false)?;
    // VMA 0xb5778
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb577c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb5780
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,31)?)?;
    // VMA 0xb5784
    c.b(C::Lt,0xb5770,false)?;
    // VMA 0xb5788
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // VMA 0xb578c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb5790
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,40)?)?;
    // VMA 0xb5794
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R12,R::R2,32)?)?;
    // VMA 0xb5798
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb579c
    c.b(C::Eq,0xb57c0,false)?;
    // VMA 0xb57a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xb57a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xb57a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb57ac
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(6)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb57b0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // VMA 0xb57b4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb57b8
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,2689)?)?;
    // VMA 0xb57bc
    c.b(C::Al,0xb57c4,false)?;
    // VMA 0xb57c0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,2689)?)?;
    // VMA 0xb57c4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,255)?)?;
    // VMA 0xb57c8
    c.b(C::Eq,0xb58e0,false)?;
    // VMA 0xb57cc
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(6)?))?;
    // VMA 0xb57d0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,2687)?)?;
    // VMA 0xb57d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,65280&0xffffffff)?)?;
    // VMA 0xb57d8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xb57dc
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R12,Shift::Lsl,8,false)?)?;
    // VMA 0xb57e0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb57e4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xb57e8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb57ec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(3)?,268435456)?)?;
    // VMA 0xb57f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xb57f4
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(6)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb57f8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(9)?,120)?)?;
    // VMA 0xb57fc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,2048)?)?;
    // VMA 0xb5800
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5804
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,64)?)?;
    // VMA 0xb5808
    c.b(C::Eq,0xb5800,false)?;
    // VMA 0xb580c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(9)?,124)?)?;
    // VMA 0xb5810
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xb5814
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,64&0xffffffff)?)?;
    // VMA 0xb5818
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31&0xffffffff)?)?;
    // VMA 0xb581c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xb5820
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(5)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5824
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(9)?,116)?)?;
    // VMA 0xb5828
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xb582c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(5)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5830
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(5)?,Shift::Lsr,11,false)?)?;
    // VMA 0xb5834
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,2687)?)?;
    // VMA 0xb5838
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb583c
    c.b(C::Cs,0xb58a4,false)?;
    // VMA 0xb5840
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5844
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,68&0xffffffff)?)?;
    // VMA 0xb5848
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,4)?)?;
    // VMA 0xb584c
    c.b(C::Ne,0xb5164,false)?;
    // VMA 0xb5850
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb5854
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5858
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb585c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // VMA 0xb5860
    c.b(C::Eq,0xb5854,false)?;
    // VMA 0xb5864
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5868
    c.literal(R::R0,0xb60c0)?;
    // VMA 0xb586c
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::LR,180)?)?;
    // VMA 0xb5870
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // VMA 0xb5874
    c.emit(arm32::mov_register(C::Al,R::R1,R::LR))?;
    // VMA 0xb5878
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb587c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,0)?)?;
    // VMA 0xb5880
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb5884
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(6)?)?)?;
    // VMA 0xb5888
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,24)?)?;
    // VMA 0xb588c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // VMA 0xb5890
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5894
    c.b(C::Ne,0xb5164,false)?;
    // VMA 0xb5898
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,180)?)?;
    // VMA 0xb589c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2616)?)?;
    // VMA 0xb58a0
    c.b(C::Al,0xb59e0,false)?;
    // VMA 0xb58a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb58a8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // VMA 0xb58ac
    c.b(C::Eq,0xb58e0,false)?;
    // VMA 0xb58b0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,131)?)?;
    // VMA 0xb58b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb58b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xb58bc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2560)?)?;
    // VMA 0xb58c0
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb58c4
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::R2,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb58c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb58cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2687)?)?;
    // VMA 0xb58d0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb58d4
    c.b(C::Cc,0xb58c0,false)?;
    // VMA 0xb58d8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(4)?,2608)?)?;
    // VMA 0xb58dc
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // VMA 0xb58e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xb58e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb58e8
    c.b(C::Eq,0xb5a30,false)?;
    // VMA 0xb58ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xb58f0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2723)?)?;
    // VMA 0xb58f4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb58f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb58fc
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2724)?)?;
    // VMA 0xb5900
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,3)?)?;
    // VMA 0xb5904
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb5908
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // VMA 0xb590c
    c.b(C::Eq,0xb5928,false)?;
    // VMA 0xb5910
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xb5914
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb5918
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb591c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5920
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5924
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2724)?)?;
    // VMA 0xb5928
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2725)?)?;
    // VMA 0xb592c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2726)?)?;
    // VMA 0xb5930
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2686)?)?;
    // VMA 0xb5934
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb5938
    c.b(C::Eq,0xb5984,false)?;
    // VMA 0xb593c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5940
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xb5944
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2725)?)?;
    // VMA 0xb5948
    c.b(C::Ne,0xb5964,false)?;
    // VMA 0xb594c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xb5950
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb5954
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5958
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb595c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5960
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2725)?)?;
    // VMA 0xb5964
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // VMA 0xb5968
    c.b(C::Eq,0xb5ac0,false)?;
    // VMA 0xb596c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,255)?)?;
    // VMA 0xb5970
    c.b(C::Eq,0xb5a60,false)?;
    // VMA 0xb5974
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,2687)?)?;
    // VMA 0xb5978
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // VMA 0xb597c
    c.b(C::Hi,0xb5a70,false)?;
    // VMA 0xb5980
    c.b(C::Al,0xb5a88,false)?;
    // VMA 0xb5984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5988
    c.b(C::Al,0xb5a8c,false)?;
    // VMA 0xb598c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,58&0xffffffff)?)?;
    // VMA 0xb5990
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5994
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R12,1)?)?;
    // VMA 0xb5998
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb599c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,31&0xffffffff)?)?;
    // VMA 0xb59a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb59a4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R0,Shift::Lsl,R::new(3)?,false)?)?;
    // VMA 0xb59a8
    c.b(C::Ne,0xb59bc,false)?;
    // VMA 0xb59ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb59b0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb59b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,32)?)?;
    // VMA 0xb59b8
    c.b(C::Cc,0xb59a4,false)?;
    // VMA 0xb59bc
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R1,32)?)?;
    // VMA 0xb59c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb59c4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xb59c8
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(8)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb59cc
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb59d0
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2616)?)?;
    // VMA 0xb59d4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xb59d8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xb59dc
    c.emit(arm32::transfer_byte(C::Cs,false,R::R12,R::new(4)?,2616)?)?;
    // VMA 0xb59e0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,444)?)?;
    // VMA 0xb59e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb59e8
    c.b(C::Eq,0xb5a10,false)?;
    // VMA 0xb59ec
    c.literal(R::R0,0xb60c4)?;
    // VMA 0xb59f0
    c.emit(arm32::mov_register(C::Al,R::R1,R::LR))?;
    // VMA 0xb59f4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // VMA 0xb59f8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xb59fc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb5a00
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(3)?)?)?;
    // VMA 0xb5a04
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,24)?)?;
    // VMA 0xb5a08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5a0c
    c.b(C::Ne,0xb5164,false)?;
    // VMA 0xb5a10
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,608)?)?;
    // VMA 0xb5a14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5a18
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5a1c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5a20
    c.emit(arm32::transfer_word_register_offset(C::Ne,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5a24
    c.emit(arm32::transfer_word_register_offset(C::Ne,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5a28
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,2686)?)?;
    // VMA 0xb5a2c
    c.b(C::Al,0xb54b4,false)?;
    // VMA 0xb5a30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xb5a34
    c.b(C::Lt,0xb5ebc,false)?;
    // VMA 0xb5a38
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,40)?)?;
    // VMA 0xb5a3c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5a40
    c.b(C::Eq,0xb5ebc,false)?;
    // VMA 0xb5a44
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // VMA 0xb5a48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5a4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5a50
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5a54
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5a58
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2690)?)?;
    // VMA 0xb5a5c
    c.b(C::Al,0xb5ec4,false)?;
    // VMA 0xb5a60
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,2688)?)?;
    // VMA 0xb5a64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // VMA 0xb5a68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5a6c
    c.b(C::Cc,0xb5a8c,false)?;
    // VMA 0xb5a70
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xb5a74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5a78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb5a7c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5a80
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5a84
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2726)?)?;
    // VMA 0xb5a88
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // VMA 0xb5a8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    // VMA 0xb5a90
    c.b(C::Eq,0xb5aa4,false)?;
    // VMA 0xb5a94
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xb5a98
    c.b(C::Ne,0xb5ac0,false)?;
    // VMA 0xb5a9c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,255)?)?;
    // VMA 0xb5aa0
    c.b(C::Eq,0xb5e68,false)?;
    // VMA 0xb5aa4
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2687)?)?;
    // VMA 0xb5aa8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // VMA 0xb5aac
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5ab0
    c.b(C::Ls,0xb5ac0,false)?;
    // VMA 0xb5ab4
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2726)?)?;
    // VMA 0xb5ab8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,67&0xffffffff)?)?;
    // VMA 0xb5abc
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5ac0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5ac4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2732)?)?;
    // VMA 0xb5ac8
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2731)?)?;
    // VMA 0xb5acc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,14)?)?;
    // VMA 0xb5ad0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5ad4
    c.b(C::Eq,0xb5ae0,false)?;
    // VMA 0xb5ad8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    // VMA 0xb5adc
    c.b(C::Eq,0xb5af4,false)?;
    // VMA 0xb5ae0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,15)?)?;
    // VMA 0xb5ae4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5ae8
    c.b(C::Eq,0xb5cbc,false)?;
    // VMA 0xb5aec
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Eor,R::R0,R::new(5)?,1,true)?)?;
    // VMA 0xb5af0
    c.b(C::Ne,0xb5cbc,false)?;
    // VMA 0xb5af4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // VMA 0xb5af8
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,12)?)?;
    // VMA 0xb5afc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,868&0xffffffff)?)?;
    // VMA 0xb5b00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5b04
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2688)?)?;
    // VMA 0xb5b08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // VMA 0xb5b0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xb5b10
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // VMA 0xb5b14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb5b18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb5b1c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xb5b20
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(4)?,2687)?)?;
    // VMA 0xb5b24
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,1124)?)?;
    // VMA 0xb5b28
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,96)?)?;
    // VMA 0xb5b2c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1164)?)?;
    // VMA 0xb5b30
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,0)?)?;
    // VMA 0xb5b34
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(10)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5b38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2912&0xffffffff)?)?;
    // VMA 0xb5b3c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb5b40
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R1,0)?)?;
    // VMA 0xb5b44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,12288)?)?;
    // VMA 0xb5b48
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5b4c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb5b50
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R12,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5b54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3040&0xffffffff)?)?;
    // VMA 0xb5b58
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,12288)?)?;
    // VMA 0xb5b5c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb5b60
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(10)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5b64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,988&0xffffffff)?)?;
    // VMA 0xb5b68
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,16)?)?;
    // VMA 0xb5b6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,14336)?)?;
    // VMA 0xb5b70
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb5b74
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xb5b78
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5b7c
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5b80
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(8)?,3)?)?;
    // VMA 0xb5b84
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R0,Shift::Lsl,15)?)?;
    // VMA 0xb5b88
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::R1,Shift::Lsl,12,false)?)?;
    // VMA 0xb5b8c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,8)?)?;
    // VMA 0xb5b90
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb5b94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,120&0xffffffff)?)?;
    // VMA 0xb5b98
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::LR,Shift::Lsl,3,false)?)?;
    // VMA 0xb5b9c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,120)?)?;
    // VMA 0xb5ba0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5ba4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1920&0xffffffff)?)?;
    // VMA 0xb5ba8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,7,false)?)?;
    // VMA 0xb5bac
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,1920)?)?;
    // VMA 0xb5bb0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5bb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,12&0xffffffff)?)?;
    // VMA 0xb5bb8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xb5bbc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb5bc0
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5bc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2048&0xffffffff)?)?;
    // VMA 0xb5bc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,65536&0xffffffff)?)?;
    // VMA 0xb5bcc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb5bd0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,201326592)?)?;
    // VMA 0xb5bd4
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5bd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,120&0xffffffff)?)?;
    // VMA 0xb5bdc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb5be0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb5be4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5be8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // VMA 0xb5bec
    c.b(C::Eq,0xb5be4,false)?;
    // VMA 0xb5bf0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,124)?)?;
    // VMA 0xb5bf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,3758096384)?)?;
    // VMA 0xb5bf8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4&0xffffffff)?)?;
    // VMA 0xb5bfc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb5c00
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5c04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4&0xffffffff)?)?;
    // VMA 0xb5c08
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xb5c0c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5c10
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5c14
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsr,8)?)?;
    // VMA 0xb5c18
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsr,16,false)?)?;
    // VMA 0xb5c1c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,2731)?)?;
    // VMA 0xb5c20
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2732)?)?;
    // VMA 0xb5c24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5c28
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1132)?)?;
    // VMA 0xb5c2c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5c30
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1879048192)?)?;
    // VMA 0xb5c34
    c.b(C::Eq,0xb5c6c,false)?;
    // VMA 0xb5c38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192&0xffffffff)?)?;
    // VMA 0xb5c3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,49&0xffffffff)?)?;
    // VMA 0xb5c40
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5c44
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb5c48
    c.literal(R::R1,0xb60cc)?;
    // VMA 0xb5c4c
    c.literal(R::R2,0xb60d0)?;
    // VMA 0xb5c50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb5c54
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5c58
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb5c5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb5c60
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5c64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255&0xffffffff)?)?;
    // VMA 0xb5c68
    c.b(C::Al,0xb5c70,false)?;
    // VMA 0xb5c6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5c70
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,116)?)?;
    // VMA 0xb5c74
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,12)?)?;
    // VMA 0xb5c78
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,24)?)?;
    // VMA 0xb5c7c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb5c80
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5c84
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // VMA 0xb5c88
    c.b(C::Ne,0xb5ca0,false)?;
    // VMA 0xb5c8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb5c90
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5c94
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5c98
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,4)?)?;
    // VMA 0xb5c9c
    c.b(C::Eq,0xb5c90,false)?;
    // VMA 0xb5ca0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2432)?)?;
    // VMA 0xb5ca4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,70&0xffffffff)?)?;
    // VMA 0xb5ca8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb5cac
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // VMA 0xb5cb0
    c.b(C::Ne,0xb5164,false)?;
    // VMA 0xb5cb4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(4)?,2608)?)?;
    // VMA 0xb5cb8
    c.b(C::Al,0xb5cc0,false)?;
    // VMA 0xb5cbc
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,24)?)?;
    // VMA 0xb5cc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5cc4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // VMA 0xb5cc8
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2727)?)?;
    // VMA 0xb5ccc
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,16)?)?;
    // VMA 0xb5cd0
    c.b(C::Eq,0xb5d1c,false)?;
    // VMA 0xb5cd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb5cd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb5cdc
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5ce0
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5ce4
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R1,5)?)?;
    // VMA 0xb5ce8
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2727)?)?;
    // VMA 0xb5cec
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xb5cf0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // VMA 0xb5cf4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // VMA 0xb5cf8
    c.b(C::Cc,0xb5d1c,false)?;
    // VMA 0xb5cfc
    c.literal(R::R1,0xb60d4)?;
    // VMA 0xb5d00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb5d04
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5d08
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb5d0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb5d10
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5d14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,127&0xffffffff)?)?;
    // VMA 0xb5d18
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5d1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb5d20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33554432&0xffffffff)?)?;
    // VMA 0xb5d24
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(6)?))?;
    // VMA 0xb5d28
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5d2c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5d30
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,6)?)?;
    // VMA 0xb5d34
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5d38
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(5)?))?;
    // VMA 0xb5d3c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,2728)?)?;
    // VMA 0xb5d40
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2730)?)?;
    // VMA 0xb5d44
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2729)?)?;
    // VMA 0xb5d48
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,13)?)?;
    // VMA 0xb5d4c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5d50
    c.b(C::Eq,0xb5da8,false)?;
    // VMA 0xb5d54
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xb5d58
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xb5d5c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // VMA 0xb5d60
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xb5d64
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xb5d68
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2729)?)?;
    // VMA 0xb5d6c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,12)?)?;
    // VMA 0xb5d70
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xb5d74
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xb5d78
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,24)?)?;
    // VMA 0xb5d7c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2730)?)?;
    // VMA 0xb5d80
    c.b(C::Hi,0xb5d94,false)?;
    // VMA 0xb5d84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,201326592&0xffffffff)?)?;
    // VMA 0xb5d88
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,24,false)?)?;
    // VMA 0xb5d8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,402653185)?)?;
    // VMA 0xb5d90
    c.b(C::Cc,0xb5da8,false)?;
    // VMA 0xb5d94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5d98
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2730)?)?;
    // VMA 0xb5d9c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2729)?)?;
    // VMA 0xb5da0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72&0xffffffff)?)?;
    // VMA 0xb5da4
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5da8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5dac
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xb5db0
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::R1,2733)?)?;
    // VMA 0xb5db4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,3)?)?;
    // VMA 0xb5db8
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,2)?)?;
    // VMA 0xb5dbc
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,1)?)?;
    // VMA 0xb5dc0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,34)?)?;
    // VMA 0xb5dc4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb5dc8
    c.b(C::Eq,0xb5e88,false)?;
    // VMA 0xb5dcc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,35)?)?;
    // VMA 0xb5dd0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5dd4
    c.b(C::Eq,0xb5e70,false)?;
    // VMA 0xb5dd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5ddc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5de0
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5de4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5de8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,255)?)?;
    // VMA 0xb5dec
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2733)?)?;
    // VMA 0xb5df0
    c.b(C::Eq,0xb5e70,false)?;
    // VMA 0xb5df4
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5df8
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5dfc
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // VMA 0xb5e00
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2734)?)?;
    // VMA 0xb5e04
    c.b(C::Ne,0xb5e88,false)?;
    // VMA 0xb5e08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5e0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33554432&0xffffffff)?)?;
    // VMA 0xb5e10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xb5e14
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xb5e18
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xb5e1c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xb5e20
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xb5e24
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2735)?)?;
    // VMA 0xb5e28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // VMA 0xb5e2c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,6)?)?;
    // VMA 0xb5e30
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xb5e34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,12)?)?;
    // VMA 0xb5e38
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2736)?)?;
    // VMA 0xb5e3c
    c.b(C::Hi,0xb5e54,false)?;
    // VMA 0xb5e40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,100663296&0xffffffff)?)?;
    // VMA 0xb5e44
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,24,false)?)?;
    // VMA 0xb5e48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,201326593)?)?;
    // VMA 0xb5e4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5e50
    c.b(C::Cc,0xb5e88,false)?;
    // VMA 0xb5e54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5e58
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2736)?)?;
    // VMA 0xb5e5c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2735)?)?;
    // VMA 0xb5e60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,73&0xffffffff)?)?;
    // VMA 0xb5e64
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5e68
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2688)?)?;
    // VMA 0xb5e6c
    c.b(C::Al,0xb5aa8,false)?;
    // VMA 0xb5e70
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,36)?)?;
    // VMA 0xb5e74
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2734)?)?;
    // VMA 0xb5e78
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,37)?)?;
    // VMA 0xb5e7c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2735)?)?;
    // VMA 0xb5e80
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,38)?)?;
    // VMA 0xb5e84
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2736)?)?;
    // VMA 0xb5e88
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,33)?)?;
    // VMA 0xb5e8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb5e90
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2737)?)?;
    // VMA 0xb5e94
    c.b(C::Eq,0xb53cc,false)?;
    // VMA 0xb5e98
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2684)?)?;
    // VMA 0xb5e9c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb5ea0
    c.b(C::Eq,0xb608c,false)?;
    // VMA 0xb5ea4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5ea8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5eac
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5eb0
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5eb4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2737)?)?;
    // VMA 0xb5eb8
    c.b(C::Al,0xb53cc,false)?;
    // VMA 0xb5ebc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2690)?)?;
    // VMA 0xb5ec0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // VMA 0xb5ec4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // VMA 0xb5ec8
    c.b(C::Eq,0xb6060,false)?;
    // VMA 0xb5ecc
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2688)?)?;
    // VMA 0xb5ed0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65280&0xffffffff)?)?;
    // VMA 0xb5ed4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R12,Shift::Lsl,8,false)?)?;
    // VMA 0xb5ed8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // VMA 0xb5edc
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xb5ee0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5ee4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,268435456)?)?;
    // VMA 0xb5ee8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5eec
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5ef0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,120)?)?;
    // VMA 0xb5ef4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb5ef8
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5efc
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,64)?)?;
    // VMA 0xb5f00
    c.b(C::Eq,0xb5ef8,false)?;
    // VMA 0xb5f04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,124)?)?;
    // VMA 0xb5f08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64&0xffffffff)?)?;
    // VMA 0xb5f0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,31&0xffffffff)?)?;
    // VMA 0xb5f10
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,2048)?)?;
    // VMA 0xb5f14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5f18
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5f1c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,116)?)?;
    // VMA 0xb5f20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb5f24
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5f28
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R2,Shift::Lsr,11,false)?)?;
    // VMA 0xb5f2c
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2688)?)?;
    // VMA 0xb5f30
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5f34
    c.b(C::Cs,0xb6014,false)?;
    // VMA 0xb5f38
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5f3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,69&0xffffffff)?)?;
    // VMA 0xb5f40
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // VMA 0xb5f44
    c.b(C::Ne,0xb5164,false)?;
    // VMA 0xb5f48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb5f4c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5f50
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5f54
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,4)?)?;
    // VMA 0xb5f58
    c.b(C::Eq,0xb5f4c,false)?;
    // VMA 0xb5f5c
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5f60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5f64
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,944)?)?;
    // VMA 0xb5f68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2564)?)?;
    // VMA 0xb5f6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xb5f70
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2564)?)?;
    // VMA 0xb5f74
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,120)?)?;
    // VMA 0xb5f78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xb5f7c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb5f80
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5f84
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // VMA 0xb5f88
    c.b(C::Eq,0xb5fcc,false)?;
    // VMA 0xb5f8c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,124)?)?;
    // VMA 0xb5f90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192&0xffffffff)?)?;
    // VMA 0xb5f94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,167&0xffffffff)?)?;
    // VMA 0xb5f98
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb5f9c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xb5fa0
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5fa4
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb5fa8
    c.literal(R::R1,0xb60cc)?;
    // VMA 0xb5fac
    c.literal(R::R2,0xb60b8)?;
    // VMA 0xb5fb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb5fb4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5fb8
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb5fbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb5fc0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5fc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb5fc8
    c.b(C::Al,0xb5fd0,false)?;
    // VMA 0xb5fcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5fd0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,116)?)?;
    // VMA 0xb5fd4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb5fd8
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5fdc
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,5)?)?;
    // VMA 0xb5fe0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // VMA 0xb5fe4
    c.b(C::Eq,0xb5164,false)?;
    // VMA 0xb5fe8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xb5fec
    c.literal(R::R1,0xb60e4)?;
    // VMA 0xb5ff0
    c.literal(R::R2,0xb60b8)?;
    // VMA 0xb5ff4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb5ff8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,936&0xffffffff)?)?;
    // VMA 0xb5ffc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6000
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb6004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb6008
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb600c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb6010
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb6014
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb6018
    c.b(C::Eq,0xb6060,false)?;
    // VMA 0xb601c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,147)?)?;
    // VMA 0xb6020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb6028
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2560)?)?;
    // VMA 0xb602c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb6030
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::new(3)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb6034
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb6038
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,2688)?)?;
    // VMA 0xb603c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6040
    c.b(C::Cc,0xb602c,false)?;
    // VMA 0xb6044
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(4)?,2608)?)?;
    // VMA 0xb6048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb604c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6050
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(5)?,R::R0,2723)?)?;
    // VMA 0xb6054
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xb6058
    c.b(C::Ne,0xb58f8,false)?;
    // VMA 0xb605c
    c.b(C::Al,0xb606c,false)?;
    // VMA 0xb6060
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6064
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6068
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R1,R::R0,2723)?)?;
    // VMA 0xb606c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // VMA 0xb6070
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb6074
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6078
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb607c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(5)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb6080
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb6084
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xb6088
    c.b(C::Al,0xb58f8,false)?;
    // VMA 0xb608c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2685)?)?;
    // VMA 0xb6090
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb6094
    c.b(C::Ne,0xb5ea4,false)?;
    // VMA 0xb6098
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // VMA 0xb609c
    c.b(C::Ne,0xb53cc,false)?;
    // VMA 0xb60a0
    c.b(C::Al,0xb5ea4,false)?;
    c.check(0xb60a4)?;
    const STREAM_CLASS_CONTROLLER_WORDS:[u32;18]=[
        0x000f_79ef,0x000e_2788,0x000f_8e3e,0x000f_7a25,
        0x000f_8e6c,0x000f_8e29,0x000f_8d97,0x0010_dd88,
        0x0010_dd98,0x0010_dd94,0x000f_7058,0x000f_8eed,
        0x000f_8ea2,0x000f_8ecb,0x000f_7a4c,0x000f_7a7c,
        0x000f_7095,0x000f_8dfc,
    ];
    for word in STREAM_CLASS_CONTROLLER_WORDS {c.emit(word)?;}
    c.check(0xb60ec)
}
