

/// Initiate a frame-descriptor payload, classify the input codec family, and
/// write format-specific descriptor fields. This is a partial function: its
/// subsequent processing paths continue at and above `0xb8c00`.
/// The two four-way branches are typed address *data*, not executable opcodes.
fn begin_frame_descriptor_dispatch(c:&mut Code)->Result<(),String>{
    c.check(0xb87d8)?;
    // ARM VMA 0xb87d8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb87dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM VMA 0xb87e0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,180)?)?;
    // ARM VMA 0xb87e4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,48)?)?;
    // ARM VMA 0xb87e8
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // ARM VMA 0xb87ec
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // ARM VMA 0xb87f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM VMA 0xb87f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,128)?)?;
    // ARM VMA 0xb87f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // ARM VMA 0xb87fc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,52)?)?;
    // ARM VMA 0xb8800
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,104)?)?;
    // ARM VMA 0xb8804
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,76)?)?;
    // ARM VMA 0xb8808
    c.literal(R::R1,0xb9800)?;
    // ARM VMA 0xb880c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,80)?)?;
    // ARM VMA 0xb8810
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // ARM VMA 0xb8814
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,92)?)?;
    // ARM VMA 0xb8818
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,488)?)?;
    // ARM VMA 0xb881c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,100)?)?;
    // ARM VMA 0xb8820
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,492)?)?;
    // ARM VMA 0xb8824
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,104)?)?;
    // ARM VMA 0xb8828
    c.b(C::Al,0x26754,true)?;
    // ARM VMA 0xb882c
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // ARM VMA 0xb8830
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,492)?)?;
    // ARM VMA 0xb8834
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,488)?)?;
    // ARM VMA 0xb8838
    c.literal(R::R1,0xb9830)?;
    // ARM VMA 0xb883c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM VMA 0xb8840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8844
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8848
    c.literal(R::R1,0xb9834)?;
    // ARM VMA 0xb884c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8850
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8854
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,568)?)?;
    // ARM VMA 0xb8858
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,484)?)?;
    // ARM VMA 0xb885c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb8860
    c.b(C::Ne,0xb9784,false)?;
    // ARM VMA 0xb8864
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,108)?)?;
    // ARM VMA 0xb8868
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,496)?)?;
    // ARM VMA 0xb886c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,112)?)?;
    // ARM VMA 0xb8870
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,500)?)?;
    // ARM VMA 0xb8874
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,116)?)?;
    // ARM VMA 0xb8878
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,524)?)?;
    // ARM VMA 0xb887c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,152)?)?;
    // ARM VMA 0xb8880
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,528)?)?;
    // ARM VMA 0xb8884
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,156)?)?;
    // ARM VMA 0xb8888
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,96)?)?;
    // ARM VMA 0xb888c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // ARM VMA 0xb8890
    c.literal(R::R0,0xb9860)?;
    // ARM VMA 0xb8894
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM VMA 0xb8898
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // ARM VMA 0xb889c
    c.literal(R::new(5)?,0xb9864)?;
    // ARM VMA 0xb88a0
    c.literal(R::new(10)?,0xb9868)?;
    // ARM VMA 0xb88a4
    c.b(C::Hi,0xb88d0,false)?;
    // ARM VMA 0xb88a8
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // ARM VMA 0xb88ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,4)?)?;
    // ARM VMA 0xb88b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb88b4
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
    c.check(0xb88b8)?;
    const FRAME_MODE_TARGETS:[u32;4]=[0xb88c8,0xb8910,0xb88e4,0xb88ec];
    for word in FRAME_MODE_TARGETS {c.emit(word)?;}
    c.check(0xb88c8)?;
    // ARM VMA 0xb88c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb88cc
    c.b(C::Al,0xb8910,false)?;
    // ARM VMA 0xb88d0
    c.literal(R::R1,0xb98d0)?;
    // ARM VMA 0xb88d4
    c.literal(R::R2,0xb98d4)?;
    // ARM VMA 0xb88d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb88dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,100)?)?;
    // ARM VMA 0xb88e0
    c.b(C::Al,0xb88fc,false)?;
    // ARM VMA 0xb88e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb88e8
    c.b(C::Al,0xb8910,false)?;
    // ARM VMA 0xb88ec
    c.literal(R::R1,0xb98e0)?;
    // ARM VMA 0xb88f0
    c.literal(R::R2,0xb98e4)?;
    // ARM VMA 0xb88f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb88f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,94)?)?;
    // ARM VMA 0xb88fc
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8900
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb8904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb8908
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb890c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8910
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb8914
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,164)?)?;
    // ARM VMA 0xb8918
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,132)?)?;
    // ARM VMA 0xb891c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,44)?)?;
    // ARM VMA 0xb8920
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,140)?)?;
    // ARM VMA 0xb8924
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,40)?)?;
    // ARM VMA 0xb8928
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,136)?)?;
    // ARM VMA 0xb892c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,144)?)?;
    // ARM VMA 0xb8930
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,148)?)?;
    // ARM VMA 0xb8934
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,136)?)?;
    // ARM VMA 0xb8938
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,40)?)?;
    // ARM VMA 0xb893c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,36)?)?;
    // ARM VMA 0xb8940
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,56)?)?;
    // ARM VMA 0xb8944
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,44)?)?;
    // ARM VMA 0xb8948
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb894c
    c.b(C::Ne,0xb97a4,false)?;
    // ARM VMA 0xb8950
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,88)?)?;
    // ARM VMA 0xb8954
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,120)?)?;
    // ARM VMA 0xb8958
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,536)?)?;
    // ARM VMA 0xb895c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,128)?)?;
    // ARM VMA 0xb8960
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,176)?)?;
    // ARM VMA 0xb8964
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,208)?)?;
    // ARM VMA 0xb8968
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb896c
    c.b(C::Eq,0xb97c4,false)?;
    // ARM VMA 0xb8970
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,20)?)?;
    // ARM VMA 0xb8974
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8978
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,124)?)?;
    // ARM VMA 0xb897c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,16)?)?;
    // ARM VMA 0xb8980
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,168)?)?;
    // ARM VMA 0xb8984
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,32)?)?;
    // ARM VMA 0xb8988
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,160)?)?;
    // ARM VMA 0xb898c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,24)?)?;
    // ARM VMA 0xb8990
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,172)?)?;
    // ARM VMA 0xb8994
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,512)?)?;
    // ARM VMA 0xb8998
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,108)?)?;
    // ARM VMA 0xb899c
    c.b(C::Ne,0xb89b0,false)?;
    // ARM VMA 0xb89a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,520)?)?;
    // ARM VMA 0xb89a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb89a8
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R1,R::new(4)?,544)?)?;
    // ARM VMA 0xb89ac
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::SP,108)?)?;
    // ARM VMA 0xb89b0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,572)?)?;
    // ARM VMA 0xb89b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb89b8
    c.b(C::Ne,0xb97dc,false)?;
    // ARM VMA 0xb89bc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,1173)?)?;
    // ARM VMA 0xb89c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb89c4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb89c8
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::SP,176)?)?;
    // ARM VMA 0xb89cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // ARM VMA 0xb89d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb89d4
    c.b(C::Ne,0xb8fc0,false)?;
    // ARM VMA 0xb89d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,464)?)?;
    // ARM VMA 0xb89dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb89e0
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(8)?,2448)?)?;
    // ARM VMA 0xb89e4
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb89e8
    c.b(C::Eq,0xb8a2c,false)?;
    // ARM VMA 0xb89ec
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,208)?)?;
    // ARM VMA 0xb89f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,800)?)?;
    // ARM VMA 0xb89f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // ARM VMA 0xb89f8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,24)?)?;
    // ARM VMA 0xb89fc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8a00
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,4)?)?;
    // ARM VMA 0xb8a04
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    // ARM VMA 0xb8a08
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,28)?)?;
    // ARM VMA 0xb8a0c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,12)?)?;
    // ARM VMA 0xb8a10
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,28)?)?;
    // ARM VMA 0xb8a14
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8a18
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // ARM VMA 0xb8a1c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R2,20)?)?;
    // ARM VMA 0xb8a20
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // ARM VMA 0xb8a24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // ARM VMA 0xb8a28
    c.b(C::Al,0x205c,true)?;
    // ARM VMA 0xb8a2c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,208)?)?;
    // ARM VMA 0xb8a30
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,68)?)?;
    // ARM VMA 0xb8a34
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,72)?)?;
    // ARM VMA 0xb8a38
    c.b(C::Al,0x80134,true)?;
    // ARM VMA 0xb8a3c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,208)?)?;
    // ARM VMA 0xb8a40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,60)?)?;
    // ARM VMA 0xb8a44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,64)?)?;
    // ARM VMA 0xb8a48
    c.b(C::Al,0x801a8,true)?;
    // ARM VMA 0xb8a4c
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(9)?,800)?)?;
    // ARM VMA 0xb8a50
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // ARM VMA 0xb8a54
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8a58
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // ARM VMA 0xb8a5c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8a60
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,28)?)?;
    // ARM VMA 0xb8a64
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8a68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1116)?)?;
    // ARM VMA 0xb8a6c
    c.b(C::Cc,0xb8b1c,false)?;
    // ARM VMA 0xb8a70
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R0,4,true)?)?;
    // ARM VMA 0xb8a74
    c.b(C::Eq,0xb8b1c,false)?;
    // ARM VMA 0xb8a78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,28)?)?;
    // ARM VMA 0xb8a7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM VMA 0xb8a80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // ARM VMA 0xb8a84
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(7)?,R::new(3)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8a88
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,8)?)?;
    // ARM VMA 0xb8a8c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,12)?)?;
    // ARM VMA 0xb8a90
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,16)?)?;
    // ARM VMA 0xb8a94
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,20)?)?;
    // ARM VMA 0xb8a98
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,732)?)?;
    // ARM VMA 0xb8a9c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // ARM VMA 0xb8aa0
    c.b(C::Hi,0xb8ac0,false)?;
    // ARM VMA 0xb8aa4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // ARM VMA 0xb8aa8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::PC,0)?)?;
    // ARM VMA 0xb8aac
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0xb8ab0)?;
    const DESCRIPTOR_LAYOUT_TARGETS:[u32;4]=[0xb8ac0,0xb8ac8,0xb8ad0,0xb8ad8];
    for word in DESCRIPTOR_LAYOUT_TARGETS {c.emit(word)?;}
    c.check(0xb8ac0)?;
    // ARM VMA 0xb8ac0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM VMA 0xb8ac4
    c.b(C::Al,0xb8ae8,false)?;
    // ARM VMA 0xb8ac8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // ARM VMA 0xb8acc
    c.b(C::Al,0xb8ae8,false)?;
    // ARM VMA 0xb8ad0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // ARM VMA 0xb8ad4
    c.b(C::Al,0xb8ae8,false)?;
    // ARM VMA 0xb8ad8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,17)?)?;
    // ARM VMA 0xb8adc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // ARM VMA 0xb8ae0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // ARM VMA 0xb8ae4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,8)?)?;
    // ARM VMA 0xb8ae8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM VMA 0xb8aec
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,200)?)?;
    // ARM VMA 0xb8af0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM VMA 0xb8af4
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R1,R::R1,32)?)?;
    // ARM VMA 0xb8af8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,20)?)?;
    // ARM VMA 0xb8afc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,232)?)?;
    // ARM VMA 0xb8b00
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,24)?)?;
    // ARM VMA 0xb8b04
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8b08
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,28)?)?;
    // ARM VMA 0xb8b0c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8b10
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,0)?)?;
    // ARM VMA 0xb8b14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R2,8)?)?;
    // ARM VMA 0xb8b18
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8b1c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2048)?)?;
    // ARM VMA 0xb8b20
    c.b(C::Eq,0xb8b90,false)?;
    // ARM VMA 0xb8b24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,44)?)?;
    // ARM VMA 0xb8b28
    c.b(C::Cc,0xb8b90,false)?;
    // ARM VMA 0xb8b2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb8b30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // ARM VMA 0xb8b34
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,4)?)?;
    // ARM VMA 0xb8b38
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // ARM VMA 0xb8b3c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,12)?)?;
    // ARM VMA 0xb8b40
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,16)?)?;
    // ARM VMA 0xb8b44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,44)?)?;
    // ARM VMA 0xb8b48
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // ARM VMA 0xb8b4c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // ARM VMA 0xb8b50
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,20)?)?;
    // ARM VMA 0xb8b54
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,32)?)?;
    // ARM VMA 0xb8b58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // ARM VMA 0xb8b5c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,24)?)?;
    // ARM VMA 0xb8b60
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,28)?)?;
    // ARM VMA 0xb8b64
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,668)?)?;
    // ARM VMA 0xb8b68
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,40)?)?;
    // ARM VMA 0xb8b6c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,664)?)?;
    // ARM VMA 0xb8b70
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,36)?)?;
    // ARM VMA 0xb8b74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // ARM VMA 0xb8b78
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,44)?)?;
    // ARM VMA 0xb8b7c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8b80
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // ARM VMA 0xb8b84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,44)?)?;
    // ARM VMA 0xb8b88
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8b8c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1116)?)?;
    // ARM VMA 0xb8b90
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2097152)?)?;
    // ARM VMA 0xb8b94
    c.b(C::Eq,0xb98a8,false)?;
    // ARM VMA 0xb8b98
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,336)?)?;
    // ARM VMA 0xb8b9c
    c.b(C::Cc,0xb98a8,false)?;
    // ARM VMA 0xb8ba0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb8ba4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,316)?)?;
    // ARM VMA 0xb8ba8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,336)?)?;
    // ARM VMA 0xb8bac
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,24)?)?;
    // ARM VMA 0xb8bb0
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(7)?,R::R2.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8bb4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // ARM VMA 0xb8bb8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,12)?)?;
    // ARM VMA 0xb8bbc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,16)?)?;
    // ARM VMA 0xb8bc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // ARM VMA 0xb8bc4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,32)?)?;
    // ARM VMA 0xb8bc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // ARM VMA 0xb8bcc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,117440512)?)?;
    // ARM VMA 0xb8bd0
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R1,R::new(7)?,20)?)?;
    // ARM VMA 0xb8bd4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,12)?)?;
    // ARM VMA 0xb8bd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,312)?)?;
    // ARM VMA 0xb8bdc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(7)?,52)?)?;
    // ARM VMA 0xb8be0
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(7)?,R::R0.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // ARM VMA 0xb8be4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8be8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,20)?)?;
    // ARM VMA 0xb8bec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,16)?)?;
    // ARM VMA 0xb8bf0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,40)?)?;
    // ARM VMA 0xb8bf4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,24)?)?;
    // ARM VMA 0xb8bf8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,44)?)?;
    // ARM VMA 0xb8bfc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,28)?)?;
    c.check(0xb8c00)
}
