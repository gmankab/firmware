

/// Complete the fast-path plane scaling, VLC lookup and codec buffer
/// accounting of the session begun at 0xa9578. The function has an early
/// return at 0xa9cec, but additional error handlers follow at 0xa9cf0.
fn write_codec_plane_blending(c:&mut Code)->Result<(),String>{
    c.check(0xa9800)?;
    // VMA 0xa9800
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,23)?)?;
    // VMA 0xa9804
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,128)?)?;
    // VMA 0xa9808
    c.b(C::Eq,0xa9818,false)?;
    // VMA 0xa980c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1048)?)?;
    // VMA 0xa9810
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xa9814
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R1,R::R1,4194304)?)?;
    // VMA 0xa9818
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,111)?)?;
    // VMA 0xa981c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,458752)?)?;
    // VMA 0xa9820
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,96)?)?;
    // VMA 0xa9824
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(4)?))?;
    // VMA 0xa9828
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xa982c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3670016)?)?;
    // VMA 0xa9830
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(7)?,R::R0,Shift::Lsl,19,false)?)?;
    // VMA 0xa9834
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9838
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,49152)?)?;
    // VMA 0xa983c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9840
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,14,false)?)?;
    // VMA 0xa9844
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa9848
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa984c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9)?)?;
    // VMA 0xa9850
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,244)?)?;
    // VMA 0xa9854
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(5)?,98)?)?;
    // VMA 0xa9858
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,100)?)?;
    // VMA 0xa985c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xa9860
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,536870912)?)?;
    // VMA 0xa9864
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,248)?)?;
    // VMA 0xa9868
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,178)?)?;
    // VMA 0xa986c
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,608)?)?;
    // VMA 0xa9870
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsl,29,false)?)?;
    // VMA 0xa9874
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,35)?)?;
    // VMA 0xa9878
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,30)?)?;
    // VMA 0xa987c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9880
    c.b(C::Ne,0xa9894,false)?;
    // VMA 0xa9884
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,36)?)?;
    // VMA 0xa9888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa988c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9890
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa9894
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,169)?)?;
    // VMA 0xa9898
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa989c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,234881024)?)?;
    // VMA 0xa98a0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,165)?)?;
    // VMA 0xa98a4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,168)?)?;
    // VMA 0xa98a8
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(5)?,164)?)?;
    // VMA 0xa98ac
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,28,false)?)?;
    // VMA 0xa98b0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,170)?)?;
    // VMA 0xa98b4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,25,false)?)?;
    // VMA 0xa98b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,131072)?)?;
    // VMA 0xa98bc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,29360128)?)?;
    // VMA 0xa98c4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(10)?,19)?)?;
    // VMA 0xa98c8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,22,false)?)?;
    // VMA 0xa98cc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,786432)?)?;
    // VMA 0xa98d4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,18,false)?)?;
    // VMA 0xa98d8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14336)?)?;
    // VMA 0xa98e0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,11,false)?)?;
    // VMA 0xa98e4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,224)?)?;
    // VMA 0xa98ec
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,5,false)?)?;
    // VMA 0xa98f0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(10)?,20)?)?;
    // VMA 0xa98f4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98f8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,2)?)?;
    // VMA 0xa98fc
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,17,false)?)?;
    // VMA 0xa9900
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,114688)?)?;
    // VMA 0xa9908
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,14,false)?)?;
    // VMA 0xa990c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9910
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1792)?)?;
    // VMA 0xa9914
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R12,Shift::Lsl,8,false)?)?;
    // VMA 0xa9918
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,2)?)?;
    // VMA 0xa991c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9920
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R12,R::new(4)?,Shift::Lsl,1,false)?)?;
    // VMA 0xa9924
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,229376)?)?;
    // VMA 0xa9928
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa992c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(6)?,1)?)?;
    // VMA 0xa9930
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2097152)?)?;
    // VMA 0xa9934
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9938
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2147483648)?)?;
    // VMA 0xa993c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,260)?)?;
    // VMA 0xa9940
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,41)?)?;
    // VMA 0xa9944
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,173)?)?;
    // VMA 0xa9948
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,10)?)?;
    // VMA 0xa994c
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(10)?,9)?)?;
    // VMA 0xa9950
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,172)?)?;
    // VMA 0xa9954
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,22,false)?)?;
    // VMA 0xa9958
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,177)?)?;
    // VMA 0xa995c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xa9960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,768)?)?;
    // VMA 0xa9964
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,21,false)?)?;
    // VMA 0xa9968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1835008)?)?;
    // VMA 0xa996c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9970
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,174)?)?;
    // VMA 0xa9974
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,18,false)?)?;
    // VMA 0xa9978
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,171)?)?;
    // VMA 0xa997c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9980
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,112)?)?;
    // VMA 0xa9984
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9988
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::new(3)?,Shift::Lsl,8,false)?)?;
    // VMA 0xa998c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9990
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // VMA 0xa9994
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::LR,Shift::Lsl,7,false)?)?;
    // VMA 0xa9998
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa999c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // VMA 0xa99a0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,6,false)?)?;
    // VMA 0xa99a4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa99a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,56)?)?;
    // VMA 0xa99ac
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,3,false)?)?;
    // VMA 0xa99b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,33554432)?)?;
    // VMA 0xa99b4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa99b8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R2,7)?)?;
    // VMA 0xa99bc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa99c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,67108864)?)?;
    // VMA 0xa99c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,264)?)?;
    // VMA 0xa99c8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,17)?)?;
    // VMA 0xa99cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,8)?)?;
    // VMA 0xa99d0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,12)?)?;
    // VMA 0xa99d4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,11)?)?;
    // VMA 0xa99d8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,26,false)?)?;
    // VMA 0xa99dc
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,25,false)?)?;
    // VMA 0xa99e0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,7)?)?;
    // VMA 0xa99e4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa99e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33030144)?)?;
    // VMA 0xa99ec
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,19,false)?)?;
    // VMA 0xa99f0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa99f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,516096)?)?;
    // VMA 0xa99f8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,13,false)?)?;
    // VMA 0xa99fc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // VMA 0xa9a04
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,3,false)?)?;
    // VMA 0xa9a08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xa9a0c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a10
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,18)?)?;
    // VMA 0xa9a14
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xa9a18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,992)?)?;
    // VMA 0xa9a1c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a20
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,15)?)?;
    // VMA 0xa9a24
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R12,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xa9a28
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a2c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,14)?)?;
    // VMA 0xa9a30
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,1)?)?;
    // VMA 0xa9a34
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a38
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,268)?)?;
    // VMA 0xa9a3c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,113)?)?;
    // VMA 0xa9a40
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,114)?)?;
    // VMA 0xa9a44
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,5,false)?)?;
    // VMA 0xa9a48
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // VMA 0xa9a4c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a50
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,272)?)?;
    // VMA 0xa9a54
    c.b(C::Al,0x2f82c,true)?;
    // VMA 0xa9a58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,671088640)?)?;
    // VMA 0xa9a5c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9a60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3670016)?)?;
    // VMA 0xa9a64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,458752)?)?;
    // VMA 0xa9a68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,536870912)?)?;
    // VMA 0xa9a6c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,536870912)?)?;
    // VMA 0xa9a70
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,252)?)?;
    // VMA 0xa9a74
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(5)?,98)?)?;
    // VMA 0xa9a78
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,100)?)?;
    // VMA 0xa9a7c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xa9a80
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,316)?)?;
    // VMA 0xa9a84
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,110)?)?;
    // VMA 0xa9a88
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,111)?)?;
    // VMA 0xa9a8c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,96)?)?;
    // VMA 0xa9a90
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::R1,Shift::Lsl,19,false)?)?;
    // VMA 0xa9a94
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a98
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xa9a9c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::R0,Shift::Lsl,8,false)?)?;
    // VMA 0xa9aa0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,255,true)?)?;
    // VMA 0xa9aa4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa9aa8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,23,false)?)?;
    // VMA 0xa9aac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1073741824)?)?;
    // VMA 0xa9ab0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ab4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ab8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9)?)?;
    // VMA 0xa9abc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,308)?)?;
    // VMA 0xa9ac0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,165)?)?;
    // VMA 0xa9ac4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,168)?)?;
    // VMA 0xa9ac8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,113)?)?;
    // VMA 0xa9acc
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,114)?)?;
    // VMA 0xa9ad0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,30,false)?)?;
    // VMA 0xa9ad4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,7)?)?;
    // VMA 0xa9ad8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,29,false)?)?;
    // VMA 0xa9adc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ae0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,234881024)?)?;
    // VMA 0xa9ae4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,25,false)?)?;
    // VMA 0xa9ae8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9aec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2031616)?)?;
    // VMA 0xa9af0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xa9af4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9af8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63488)?)?;
    // VMA 0xa9afc
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,11,false)?)?;
    // VMA 0xa9b00
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9b04
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,312)?)?;
    // VMA 0xa9b08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,104)?)?;
    // VMA 0xa9b0c
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,100)?)?;
    // VMA 0xa9b10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xa9b14
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,224)?)?;
    // VMA 0xa9b18
    c.literal(R::R0,0xaa1fc)?;
    // VMA 0xa9b1c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // VMA 0xa9b20
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xa9b24
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xa9b28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,2880)?)?;
    // VMA 0xa9b2c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xa9b30
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xa9b34
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xa9b38
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xa9b3c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,140)?)?;
    // VMA 0xa9b40
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa9b44
    c.b(C::Al,0xa925c,true)?;
    // VMA 0xa9b48
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xa9b4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xa9b50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xa9b54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // VMA 0xa9b58
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xa9b5c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,112)?)?;
    // VMA 0xa9b60
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa9b64
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa9b68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa9b6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,112)?)?;
    // VMA 0xa9b70
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa9b74
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,9)?)?;
    // VMA 0xa9b78
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // VMA 0xa9b7c
    c.b(C::Lt,0xa9ba4,false)?;
    // VMA 0xa9b80
    c.literal(R::R1,0xaa200)?;
    // VMA 0xa9b84
    c.literal(R::R2,0xaa204)?;
    // VMA 0xa9b88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9b8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,143)?)?;
    // VMA 0xa9b90
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9b94
    c.literal(R::R1,0xaa208)?;
    // VMA 0xa9b98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9b9c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9ba0
    c.b(C::Al,0xa9bac,false)?;
    // VMA 0xa9ba4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xa9ba8
    c.b(C::Lt,0xa9bf4,false)?;
    // VMA 0xa9bac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9bb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // VMA 0xa9bb4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9bb8
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::R2,140)?)?;
    // VMA 0xa9bbc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9bc0
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xa9bc4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,112)?)?;
    // VMA 0xa9bc8
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xa9bcc
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::new(7)?,R::new(3)?,R::R1,R::new(7)?)?)?;
    // VMA 0xa9bd0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,5)?)?;
    // VMA 0xa9bd4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R2,168)?)?;
    // VMA 0xa9bd8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // VMA 0xa9bdc
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R2,R::new(3)?,R::R1,R::R2)?)?;
    // VMA 0xa9be0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,12)?)?;
    // VMA 0xa9be4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xa9be8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa9bec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9bf0
    c.b(C::Ne,0xa9bb4,false)?;
    // VMA 0xa9bf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xa9bf8
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(10)?))?;
    // VMA 0xa9bfc
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,228)?)?;
    // VMA 0xa9c00
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,232)?)?;
    // VMA 0xa9c04
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,236)?)?;
    // VMA 0xa9c08
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // VMA 0xa9c0c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,175)?)?;
    // VMA 0xa9c10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1088)?)?;
    // VMA 0xa9c14
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,1092)?)?;
    // VMA 0xa9c18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9c1c
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xa9c20
    c.b(C::Eq,0xa9cbc,false)?;
    // VMA 0xa9c24
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,39)?)?;
    // VMA 0xa9c28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9c2c
    c.b(C::Eq,0xa9c64,false)?;
    // VMA 0xa9c30
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,48)?)?;
    // VMA 0xa9c34
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xa9c38
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2147483648)?)?;
    // VMA 0xa9c3c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,228)?)?;
    // VMA 0xa9c40
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // VMA 0xa9c44
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,232)?)?;
    // VMA 0xa9c48
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,56)?)?;
    // VMA 0xa9c4c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,236)?)?;
    // VMA 0xa9c50
    c.literal(R::R0,0xaa20c)?;
    // VMA 0xa9c54
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xa9c58
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa9c5c
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xa9c60
    c.b(C::Al,0xa9cbc,false)?;
    // VMA 0xa9c64
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,176)?)?;
    // VMA 0xa9c68
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9c6c
    c.b(C::Eq,0xa9c9c,false)?;
    // VMA 0xa9c70
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,624)?)?;
    // VMA 0xa9c74
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2147483648)?)?;
    // VMA 0xa9c78
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,228)?)?;
    // VMA 0xa9c7c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,628)?)?;
    // VMA 0xa9c80
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,232)?)?;
    // VMA 0xa9c84
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,632)?)?;
    // VMA 0xa9c88
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,236)?)?;
    // VMA 0xa9c8c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,636)?)?;
    // VMA 0xa9c90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1092)?)?;
    // VMA 0xa9c94
    c.b(C::Al,0x7ffc8,true)?;
    // VMA 0xa9c98
    c.b(C::Al,0xa9cbc,false)?;
    // VMA 0xa9c9c
    c.literal(R::R0,0xaa210)?;
    // VMA 0xa9ca0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,150994944)?)?;
    // VMA 0xa9ca4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,228)?)?;
    // VMA 0xa9ca8
    c.literal(R::R1,0xaa214)?;
    // VMA 0xa9cac
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,232)?)?;
    // VMA 0xa9cb0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,236)?)?;
    // VMA 0xa9cb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9cb8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1092)?)?;
    // VMA 0xa9cbc
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,256)?)?;
    // VMA 0xa9cc0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,19)?)?;
    // VMA 0xa9cc4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9cc8
    c.b(C::Eq,0xa9cd8,false)?;
    // VMA 0xa9ccc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1100)?)?;
    // VMA 0xa9cd0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9cd4
    c.b(C::Eq,0xa9cf0,false)?;
    // VMA 0xa9cd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa9cdc
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // VMA 0xa9ce0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,132)?)?;
    // VMA 0xa9ce4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,136)?)?;
    // VMA 0xa9ce8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xa9cec
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa9cf0)
}
