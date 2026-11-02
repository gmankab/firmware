

/// Begin preparing the next codec controller session: copy output-format
/// metadata to controller mapped addresses, compute input mode parameters,
/// and encode initial stream flags. This function continues after `0xa9800`.
fn begin_codec_controller_format(c:&mut Code)->Result<(),String>{
    c.check(0xa9578)?;
    // VMA 0xa9578
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa957c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xa9580
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,60)?)?;
    // VMA 0xa9584
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xa9588
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,2448)?)?;
    // VMA 0xa958c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,2444)?)?;
    // VMA 0xa9590
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xa9594
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,604)?)?;
    // VMA 0xa9598
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa959c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,672)?)?;
    // VMA 0xa95a0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,44)?)?;
    // VMA 0xa95a4
    c.b(C::Eq,0xa9600,false)?;
    // VMA 0xa95a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,4000)?)?;
    // VMA 0xa95ac
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // VMA 0xa95b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,676)?)?;
    // VMA 0xa95b4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,3996)?)?;
    // VMA 0xa95b8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,992)?)?;
    // VMA 0xa95bc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,684)?)?;
    // VMA 0xa95c0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,996)?)?;
    // VMA 0xa95c4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,680)?)?;
    // VMA 0xa95c8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,256)?)?;
    // VMA 0xa95cc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,688)?)?;
    // VMA 0xa95d0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,264)?)?;
    // VMA 0xa95d4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,692)?)?;
    // VMA 0xa95d8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,76)?)?;
    // VMA 0xa95dc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(5)?,165)?)?;
    // VMA 0xa95e0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // VMA 0xa95e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xa95e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,12)?)?;
    // VMA 0xa95ec
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::R2,1)?)?;
    // VMA 0xa95f0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,2,false)?)?;
    // VMA 0xa95f4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa95f8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // VMA 0xa95fc
    c.b(C::Al,0xa9604,false)?;
    // VMA 0xa9600
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(5)?,165)?)?;
    // VMA 0xa9604
    c.emit(arm32::load_halfword(C::Al,R::new(9)?,R::new(5)?,100)?)?;
    // VMA 0xa9608
    c.emit(arm32::load_halfword(C::Al,R::LR,R::new(5)?,98)?)?;
    // VMA 0xa960c
    c.literal(R::new(7)?,0xaa1f4)?;
    // VMA 0xa9610
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3670016)?)?;
    // VMA 0xa9614
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1536)?)?;
    // VMA 0xa9618
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::LR,R::new(9)?,Shift::Lsl,16,false)?)?;
    // VMA 0xa961c
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9620
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,320)?)?;
    // VMA 0xa9624
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,110)?)?;
    // VMA 0xa9628
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,111)?)?;
    // VMA 0xa962c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,96)?)?;
    // VMA 0xa9630
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xa9634
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,28)?)?;
    // VMA 0xa9638
    c.emit(arm32::transfer_byte(C::Al,true,R::new(10)?,R::new(5)?,609)?)?;
    // VMA 0xa963c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,64)?)?;
    // VMA 0xa9640
    c.emit(arm32::data_register(C::Al,Op::And,R::R12,R::new(4)?,R::R2,Shift::Lsl,19,false)?)?;
    // VMA 0xa9644
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // VMA 0xa9648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,458752)?)?;
    // VMA 0xa964c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsl,9,false)?)?;
    // VMA 0xa9650
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,16,false)?)?;
    // VMA 0xa9654
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9658
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa965c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,15)?)?;
    // VMA 0xa9660
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,14)?)?;
    // VMA 0xa9664
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,7)?)?;
    // VMA 0xa9668
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,6,false)?)?;
    // VMA 0xa966c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9670
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // VMA 0xa9674
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(10)?,Shift::Lsl,5,false)?)?;
    // VMA 0xa9678
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(4)?))?;
    // VMA 0xa967c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,24)?)?;
    // VMA 0xa9680
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // VMA 0xa9688
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,3,false)?)?;
    // VMA 0xa968c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,507904)?)?;
    // VMA 0xa9690
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9694
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xa9698
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xa969c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa96a0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,1)?)?;
    // VMA 0xa96a4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R1,3)?)?;
    // VMA 0xa96a8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa96ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,324)?)?;
    // VMA 0xa96b0
    c.literal(R::R0,0xaa1f8)?;
    // VMA 0xa96b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,328)?)?;
    // VMA 0xa96b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12582912)?)?;
    // VMA 0xa96bc
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsl,22,false)?)?;
    // VMA 0xa96c0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa96c4
    c.emit(arm32::mov_shifted_register(C::Al,R::R12,R::new(9)?,Shift::Lsl,16)?)?;
    // VMA 0xa96c8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa96cc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R1,36)?)?;
    // VMA 0xa96d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,332)?)?;
    // VMA 0xa96d4
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,12)?)?;
    // VMA 0xa96d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,11)?)?;
    // VMA 0xa96dc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,35)?)?;
    // VMA 0xa96e0
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,14,false)?)?;
    // VMA 0xa96e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15872)?)?;
    // VMA 0xa96e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xa96ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xa96f0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,9,false)?)?;
    // VMA 0xa96f4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa96f8
    c.b(C::Ne,0xa9718,false)?;
    // VMA 0xa96fc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,36)?)?;
    // VMA 0xa9700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xa9704
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9708
    c.b(C::Eq,0xa9718,false)?;
    // VMA 0xa970c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,19)?)?;
    // VMA 0xa9710
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xa9714
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xa9718
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // VMA 0xa971c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,8,false)?)?;
    // VMA 0xa9720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,128)?)?;
    // VMA 0xa9724
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(4)?,480)?)?;
    // VMA 0xa9728
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(3)?,178)?)?;
    // VMA 0xa972c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(3)?,179)?)?;
    // VMA 0xa9730
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,7,false)?)?;
    // VMA 0xa9734
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,16)?)?;
    // VMA 0xa9738
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa973c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa9740
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(5)?,Shift::Lsl,5,false)?)?;
    // VMA 0xa9744
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // VMA 0xa9748
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa974c
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,33)?)?;
    // VMA 0xa9750
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,4,false)?)?;
    // VMA 0xa9754
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1048576)?)?;
    // VMA 0xa9758
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa975c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xa9760
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9764
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,340)?)?;
    // VMA 0xa9768
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,44)?)?;
    // VMA 0xa976c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,32)?)?;
    // VMA 0xa9770
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,92)?)?;
    // VMA 0xa9774
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9778
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,19)?)?;
    // VMA 0xa977c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xa9780
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(6)?,R::R0,Shift::Lsl,20,false)?)?;
    // VMA 0xa9784
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,22,false)?)?;
    // VMA 0xa9788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,131072)?)?;
    // VMA 0xa978c
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,17,false)?)?;
    // VMA 0xa9790
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9794
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // VMA 0xa9798
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,344)?)?;
    // VMA 0xa979c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R12,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xa97a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,336)?)?;
    // VMA 0xa97a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R1,9)?)?;
    // VMA 0xa97a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xa97ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,348)?)?;
    // VMA 0xa97b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xa97b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,616)?)?;
    // VMA 0xa97b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,608)?)?;
    // VMA 0xa97bc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa97c0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa97c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1160)?)?;
    // VMA 0xa97c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,12)?)?;
    // VMA 0xa97cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa97d0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa97d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa97d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,292)?)?;
    // VMA 0xa97dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2788)?)?;
    // VMA 0xa97e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,276)?)?;
    // VMA 0xa97e4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,110)?)?;
    // VMA 0xa97e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa97ec
    c.b(C::Ne,0xa97fc,false)?;
    // VMA 0xa97f0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,111)?)?;
    // VMA 0xa97f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa97f8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa97fc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,577)?)?;
    c.check(0xa9800)
}
