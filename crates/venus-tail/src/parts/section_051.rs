

/// Reset frame-controller status and release queue pointers conditionally.
/// Original asynchronous errors are routed to logging and fail-closed return.
fn reconcile_controller_frame_status(c:&mut Code)->Result<(),String>{
    c.check(0xb9904)?;
    // ARM VMA 0xb9904
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb9908
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // ARM VMA 0xb990c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,520)?)?;
    // ARM VMA 0xb9910
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,1024)?)?;
    // ARM VMA 0xb9914
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // ARM VMA 0xb9918
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb991c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,552)?)?;
    // ARM VMA 0xb9920
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb9924
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,532)?)?;
    // ARM VMA 0xb9928
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,8)?)?;
    // ARM VMA 0xb992c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM VMA 0xb9930
    c.b(C::Ne,0xb9948,false)?;
    // ARM VMA 0xb9934
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9938
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,520)?)?;
    // ARM VMA 0xb993c
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9940
    c.b(C::Eq,0xb9964,false)?;
    // ARM VMA 0xb9944
    c.b(C::Al,0xb9950,false)?;
    // ARM VMA 0xb9948
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb994c
    c.b(C::Eq,0xb9994,false)?;
    // ARM VMA 0xb9950
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,520)?)?;
    // ARM VMA 0xb9954
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9958
    c.b(C::Eq,0xb9994,false)?;
    // ARM VMA 0xb995c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // ARM VMA 0xb9960
    c.b(C::Cs,0xb9990,false)?;
    // ARM VMA 0xb9964
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,516)?)?;
    // ARM VMA 0xb9968
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,380)?)?;
    // ARM VMA 0xb996c
    c.emit(arm32::mov_register(C::Al,R::R0,R::SP))?;
    // ARM VMA 0xb9970
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,1024)?)?;
    // ARM VMA 0xb9974
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb9978
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // ARM VMA 0xb997c
    c.b(C::Al,0x471c,true)?;
    // ARM VMA 0xb9980
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM VMA 0xb9984
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb9988
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // ARM VMA 0xb998c
    c.b(C::Al,0xb99a4,false)?;
    // ARM VMA 0xb9990
    c.b(C::Ne,0xb9b40,false)?;
    // ARM VMA 0xb9994
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,516)?)?;
    // ARM VMA 0xb9998
    c.emit(arm32::mov_register(C::Al,R::R0,R::SP))?;
    // ARM VMA 0xb999c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb99a0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // ARM VMA 0xb99a4
    c.b(C::Al,0x471c,true)?;
    // ARM VMA 0xb99a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,552)?)?;
    // ARM VMA 0xb99ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb99b0
    c.b(C::Ne,0xb99e0,false)?;
    // ARM VMA 0xb99b4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,588)?)?;
    // ARM VMA 0xb99b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb99bc
    c.b(C::Eq,0xb99e0,false)?;
    // ARM VMA 0xb99c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,484)?)?;
    // ARM VMA 0xb99c4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb99c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,484)?)?;
    // ARM VMA 0xb99cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,864)?)?;
    // ARM VMA 0xb99d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb99d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,864)?)?;
    // ARM VMA 0xb99d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb99dc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,932)?)?;
    // ARM VMA 0xb99e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // ARM VMA 0xb99e4
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::SP,556)?)?;
    // ARM VMA 0xb99e8
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb99ec
    c.b(C::Ne,0xb9a24,false)?;
    // ARM VMA 0xb99f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,588)?)?;
    // ARM VMA 0xb99f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb99f8
    c.b(C::Eq,0xb9a24,false)?;
    // ARM VMA 0xb99fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,484)?)?;
    // ARM VMA 0xb9a00
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,2097152)?)?;
    // ARM VMA 0xb9a04
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,484)?)?;
    // ARM VMA 0xb9a08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9a0c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,556)?)?;
    // ARM VMA 0xb9a10
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,864)?)?;
    // ARM VMA 0xb9a14
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2097152)?)?;
    // ARM VMA 0xb9a18
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,864)?)?;
    // ARM VMA 0xb9a1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9a20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,936)?)?;
    // ARM VMA 0xb9a24
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,532)?)?;
    // ARM VMA 0xb9a28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9a2c
    c.b(C::Eq,0xb9a40,false)?;
    // ARM VMA 0xb9a30
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,520)?)?;
    // ARM VMA 0xb9a34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9a38
    c.emit(arm32::mov_register(C::Ne,R::R1,R::SP))?;
    // ARM VMA 0xb9a3c
    c.b(C::Ne,0xb9b78,true)?;
    // ARM VMA 0xb9a40
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,208)?)?;
    // ARM VMA 0xb9a44
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9a48
    c.b(C::Eq,0xb9a90,false)?;
    // ARM VMA 0xb9a4c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,1028)?)?;
    // ARM VMA 0xb9a50
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9a54
    c.b(C::Eq,0xb9a68,false)?;
    // ARM VMA 0xb9a58
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // ARM VMA 0xb9a5c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM VMA 0xb9a60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb9a64
    c.b(C::Al,0xb9a8c,false)?;
    // ARM VMA 0xb9a68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,552)?)?;
    // ARM VMA 0xb9a6c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9a70
    c.b(C::Ne,0xb9a80,false)?;
    // ARM VMA 0xb9a74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,496)?)?;
    // ARM VMA 0xb9a78
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9a7c
    c.b(C::Eq,0xb9a90,false)?;
    // ARM VMA 0xb9a80
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // ARM VMA 0xb9a84
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM VMA 0xb9a88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb9a8c
    c.b(C::Al,0xb87d8,true)?;
    // ARM VMA 0xb9a90
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,932)?)?;
    // ARM VMA 0xb9a94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // ARM VMA 0xb9a98
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9a9c
    c.b(C::Ne,0xb9b34,false)?;
    // ARM VMA 0xb9aa0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,900)?)?;
    // ARM VMA 0xb9aa4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,380)?)?;
    // ARM VMA 0xb9aa8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9aac
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(7)?))?;
    // ARM VMA 0xb9ab0
    c.b(C::Ne,0xb9b78,true)?;
    // ARM VMA 0xb9ab4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1408)?)?;
    // ARM VMA 0xb9ab8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9abc
    c.b(C::Eq,0xb9ad0,false)?;
    // ARM VMA 0xb9ac0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM VMA 0xb9ac4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb9ac8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb9acc
    c.b(C::Al,0xb9ae8,false)?;
    // ARM VMA 0xb9ad0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,876)?)?;
    // ARM VMA 0xb9ad4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9ad8
    c.b(C::Eq,0xb9aec,false)?;
    // ARM VMA 0xb9adc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM VMA 0xb9ae0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb9ae4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb9ae8
    c.b(C::Al,0xb87d8,true)?;
    // ARM VMA 0xb9aec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,372)?)?;
    // ARM VMA 0xb9af0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb9af4
    c.b(C::Eq,0xb9b08,false)?;
    // ARM VMA 0xb9af8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // ARM VMA 0xb9afc
    c.b(C::Al,0xd88a0,true)?;
    // ARM VMA 0xb9b00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9b04
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,372)?)?;
    // ARM VMA 0xb9b08
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,376)?)?;
    // ARM VMA 0xb9b0c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb9b10
    c.b(C::Eq,0xb9b24,false)?;
    // ARM VMA 0xb9b14
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // ARM VMA 0xb9b18
    c.b(C::Al,0xd88a0,true)?;
    // ARM VMA 0xb9b1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9b20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,376)?)?;
    // ARM VMA 0xb9b24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9b28
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,932)?)?;
    // ARM VMA 0xb9b2c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,588)?)?;
    // ARM VMA 0xb9b30
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,864)?)?;
    // ARM VMA 0xb9b34
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM VMA 0xb9b38
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // ARM VMA 0xb9b3c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb9b40
    c.literal(R::R1,0xb9b6c)?;
    // ARM VMA 0xb9b44
    c.literal(R::R2,0xb9b70)?;
    // ARM VMA 0xb9b48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,237)?)?;
    // ARM VMA 0xb9b4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb9b50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM VMA 0xb9b54
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9b58
    c.literal(R::R1,0xb9b74)?;
    // ARM VMA 0xb9b5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb9b60
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9b64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM VMA 0xb9b68
    c.b(C::Al,0xb9b34,false)?;
    c.check(0xb9b6c)?;
    const FRAME_STATE_TRACE:[u32;3]=[0x000f_9a31,0x000f_9a5e,0x000e_2788];
    for word in FRAME_STATE_TRACE {c.emit(word)?;}
    c.check(0xb9b78)
}

/// Shut down optional frame sources and clear residual input slots; restore
/// preserved registers through the original POP/PC epilogue.
fn clear_optional_codec_sources(c:&mut Code)->Result<(),String>{
    c.check(0xb9b78)?;
    // ARM VMA 0xb9b78
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb9b7c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // ARM VMA 0xb9b80
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // ARM VMA 0xb9b84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // ARM VMA 0xb9b88
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // ARM VMA 0xb9b8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb9b90
    c.b(C::Cc,0xb9c4c,false)?;
    // ARM VMA 0xb9b94
    c.b(C::Ne,0xb9bb8,false)?;
    // ARM VMA 0xb9b98
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,216)?)?;
    // ARM VMA 0xb9b9c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9ba0
    c.b(C::Eq,0xb9be4,false)?;
    // ARM VMA 0xb9ba4
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb9ba8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9bac
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9bb0
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,216)?)?;
    // ARM VMA 0xb9bb4
    c.b(C::Al,0xb9c08,false)?;
    // ARM VMA 0xb9bb8
    c.literal(R::R1,0xb9c60)?;
    // ARM VMA 0xb9bbc
    c.literal(R::R2,0xb9c58)?;
    // ARM VMA 0xb9bc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,125)?)?;
    // ARM VMA 0xb9bc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb9bc8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM VMA 0xb9bcc
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9bd0
    c.literal(R::R1,0xb9c5c)?;
    // ARM VMA 0xb9bd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb9bd8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9bdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM VMA 0xb9be0
    c.b(C::Al,0xb9c4c,false)?;
    // ARM VMA 0xb9be4
    c.literal(R::R1,0xb9c54)?;
    // ARM VMA 0xb9be8
    c.literal(R::R2,0xb9c58)?;
    // ARM VMA 0xb9bec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,105)?)?;
    // ARM VMA 0xb9bf0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb9bf4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM VMA 0xb9bf8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9bfc
    c.literal(R::R1,0xb9c5c)?;
    // ARM VMA 0xb9c00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb9c04
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9c08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,220)?)?;
    // ARM VMA 0xb9c0c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9c10
    c.b(C::Eq,0xb9c28,false)?;
    // ARM VMA 0xb9c14
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb9c18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9c1c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9c20
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,220)?)?;
    // ARM VMA 0xb9c24
    c.b(C::Al,0xb9c4c,false)?;
    // ARM VMA 0xb9c28
    c.literal(R::R1,0xb9c54)?;
    // ARM VMA 0xb9c2c
    c.literal(R::R2,0xb9c58)?;
    // ARM VMA 0xb9c30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,117)?)?;
    // ARM VMA 0xb9c34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb9c38
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM VMA 0xb9c3c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9c40
    c.literal(R::R1,0xb9c5c)?;
    // ARM VMA 0xb9c44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb9c48
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9c4c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb9c50
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb9c54)?;
    const FRAME_SOURCE_TRACE:[u32;4]=[
        0x000f_9a7b,0x000f_9ab4,0x000e_2788,0x000f_9a31,
    ];
    for word in FRAME_SOURCE_TRACE {c.emit(word)?;}
    c.check(0xb9c64)
}
