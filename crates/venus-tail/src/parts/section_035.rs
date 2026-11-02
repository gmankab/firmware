
/// Flush hardware codec plane descriptors and publish queue acknowledgments.
fn flush_controller_stream_buffer(c:&mut Code)->Result<(),String>{
    c.check(0xb49a4)?;
    // VMA 0xb49a4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb49a8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb49ac
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,68)?)?;
    // VMA 0xb49b0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // VMA 0xb49b4
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,24)?)?;
    // VMA 0xb49b8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,36)?)?;
    // VMA 0xb49bc
    c.emit(arm32::mov_register(C::Al,R::LR,R::R2))?;
    // VMA 0xb49c0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xb49c4
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,184)?)?;
    // VMA 0xb49c8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb49cc
    c.b(C::Eq,0xb4d2c,false)?;
    // VMA 0xb49d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,115)?)?;
    // VMA 0xb49d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb49d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // VMA 0xb49dc
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(7)?,15)?)?;
    // VMA 0xb49e0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,445)?)?;
    // VMA 0xb49e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb49e8
    c.b(C::Eq,0xb4a2c,false)?;
    // VMA 0xb49ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048&0xffffffff)?)?;
    // VMA 0xb49f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb49f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb49f8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb49fc
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4a00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb4a04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb4a08
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb4a0c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4a10
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R1,255)?)?;
    // VMA 0xb4a14
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(7)?,15)?)?;
    // VMA 0xb4a18
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4a1c
    c.b(C::Ls,0xb4a2c,false)?;
    // VMA 0xb4a20
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(7)?,15)?)?;
    // VMA 0xb4a24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65&0xffffffff)?)?;
    // VMA 0xb4a28
    c.b(C::Al,0xb4d30,false)?;
    // VMA 0xb4a2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048&0xffffffff)?)?;
    // VMA 0xb4a30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb4a34
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb4a38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb4a3c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4a40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4&0xffffffff)?)?;
    // VMA 0xb4a44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xb4a48
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb4a4c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(10)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4a50
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(7)?,14)?)?;
    // VMA 0xb4a54
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R12,R::new(10)?,255)?)?;
    // VMA 0xb4a58
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,12)?)?;
    // VMA 0xb4a5c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(9)?,123)?)?;
    // VMA 0xb4a60
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4a64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65&0xffffffff)?)?;
    // VMA 0xb4a68
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4a6c
    c.b(C::Ge,0xb4d30,false)?;
    // VMA 0xb4a70
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-40)?)?;
    // VMA 0xb4a74
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::new(4)?,Shift::Lsl,15)?)?;
    // VMA 0xb4a78
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb4a7c
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4a80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,31&0xffffffff)?)?;
    // VMA 0xb4a84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4a88
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // VMA 0xb4a8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // VMA 0xb4a90
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,32)?)?;
    // VMA 0xb4a94
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // VMA 0xb4a98
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,R::new(5)?,false)?)?;
    // VMA 0xb4a9c
    c.b(C::Ne,0xb4ab0,false)?;
    // VMA 0xb4aa0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xb4aa4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xb4aa8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,31)?)?;
    // VMA 0xb4aac
    c.b(C::Lt,0xb4a98,false)?;
    // VMA 0xb4ab0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,1)?)?;
    // VMA 0xb4ab4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R1,255)?)?;
    // VMA 0xb4ab8
    c.emit(arm32::data_register(C::Al,Op::Cmn,R::R0,R::R12,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4abc
    c.b(C::Ne,0xb4ae0,false)?;
    // VMA 0xb4ac0
    c.literal(R::R1,0xb4e0c)?;
    // VMA 0xb4ac4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4ac8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4acc
    c.literal(R::R1,0xb4e10)?;
    // VMA 0xb4ad0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4ad4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4ad8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(10)?,R::new(7)?,14)?)?;
    // VMA 0xb4adc
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,15)?)?;
    // VMA 0xb4ae0
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R1,255)?)?;
    // VMA 0xb4ae4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(10)?,255)?)?;
    // VMA 0xb4ae8
    c.emit(arm32::data_register(C::Al,Op::Cmn,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4aec
    c.b(C::Eq,0xb4cd4,false)?;
    // VMA 0xb4af0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(7)?,16)?)?;
    // VMA 0xb4af4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4af8
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(4)?,R::new(8)?,32)?)?;
    // VMA 0xb4afc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4b00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4b04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4b08
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,36)?)?;
    // VMA 0xb4b0c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // VMA 0xb4b10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4b14
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,28)?)?;
    // VMA 0xb4b18
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4b1c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-44)?)?;
    // VMA 0xb4b20
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // VMA 0xb4b24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,83)?)?;
    // VMA 0xb4b28
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,44)?)?;
    // VMA 0xb4b2c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4b30
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4b34
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(7)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xb4b38
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,16)?)?;
    // VMA 0xb4b3c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,40)?)?;
    // VMA 0xb4b40
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(3)?,Shift::Lsl,2)?)?;
    // VMA 0xb4b44
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb4b48
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-40)?)?;
    // VMA 0xb4b4c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb4b50
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,32)?)?;
    // VMA 0xb4b54
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4b58
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xb4b5c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,24)?)?;
    // VMA 0xb4b60
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xb4b64
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(4)?,255)?)?;
    // VMA 0xb4b68
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xb4b6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb4b70
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-44)?)?;
    // VMA 0xb4b74
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(4)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4b78
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,16)?)?;
    // VMA 0xb4b7c
    c.b(C::Cs,0xb4dbc,false)?;
    // VMA 0xb4b80
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4b84
    c.b(C::Ge,0xb4ba8,false)?;
    // VMA 0xb4b88
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // VMA 0xb4b8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // VMA 0xb4b90
    c.b(C::Lt,0xb4bd8,false)?;
    // VMA 0xb4b94
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,24)?)?;
    // VMA 0xb4b98
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4b9c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // VMA 0xb4ba0
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4ba4
    c.b(C::Al,0xb4bdc,false)?;
    // VMA 0xb4ba8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,115)?)?;
    // VMA 0xb4bac
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(6)?))?;
    // VMA 0xb4bb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4bb4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xb4bb8
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(5)?,0)?)?;
    // VMA 0xb4bbc
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // VMA 0xb4bc0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // VMA 0xb4bc4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(5)?,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xb4bc8
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-32)?)?;
    // VMA 0xb4bcc
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(5)?,R::new(9)?,R::new(8)?,Shift::Lsl,2,true)?)?;
    // VMA 0xb4bd0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xb4bd4
    c.b(C::Al,0xb4c08,false)?;
    // VMA 0xb4bd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4bdc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(9)?,445)?)?;
    // VMA 0xb4be0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4be4
    c.b(C::Ge,0xb4dd4,false)?;
    // VMA 0xb4be8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(9)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb4bec
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-32)?)?;
    // VMA 0xb4bf0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(9)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4bf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4bf8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,448)?)?;
    // VMA 0xb4bfc
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(5)?,R::new(4)?,R::new(8)?,Shift::Lsl,2,true)?)?;
    // VMA 0xb4c00
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(3)?,576)?)?;
    // VMA 0xb4c04
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // VMA 0xb4c08
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,44)?)?;
    // VMA 0xb4c0c
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::new(3)?,R::new(5)?,R::new(8)?,Shift::Lsl,0,true)?)?;
    // VMA 0xb4c10
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::LR,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c14
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb4c1c
    c.b(C::Eq,0xb4c78,false)?;
    // VMA 0xb4c20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb4c24
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c28
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,14)?)?;
    // VMA 0xb4c2c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(7)?,16)?)?;
    // VMA 0xb4c30
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(7)?,15)?)?;
    // VMA 0xb4c34
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c38
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(5)?,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c3c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,32)?)?;
    // VMA 0xb4c40
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c44
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c48
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-32)?)?;
    // VMA 0xb4c4c
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(5)?))?;
    // VMA 0xb4c50
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R12,R::R12,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c54
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c58
    c.emit(arm32::mov_register(C::Eq,R::R12,R::new(5)?))?;
    // VMA 0xb4c5c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(5)?,R::new(6)?,R::new(8)?,Shift::Lsl,2,true)?)?;
    // VMA 0xb4c60
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,20)?)?;
    // VMA 0xb4c64
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R12,255)?)?;
    // VMA 0xb4c68
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c6c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,48)?)?;
    // VMA 0xb4c70
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,R::new(6)?,false)?)?;
    // VMA 0xb4c74
    c.b(C::Al,0xb4ca0,false)?;
    // VMA 0xb4c78
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,14)?)?;
    // VMA 0xb4c7c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(7)?,16)?)?;
    // VMA 0xb4c80
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(7)?,15)?)?;
    // VMA 0xb4c84
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c88
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c8c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c90
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R12,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4c94
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4c98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2147483648)?)?;
    // VMA 0xb4c9c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R12,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4ca0
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,40)?)?;
    // VMA 0xb4ca4
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,36)?)?;
    // VMA 0xb4ca8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // VMA 0xb4cac
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // VMA 0xb4cb0
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(10)?,R::R12,255)?)?;
    // VMA 0xb4cb4
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::new(4)?,R::new(8)?,Shift::Lsl,2,true)?)?;
    // VMA 0xb4cb8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xb4cbc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(7)?,14)?)?;
    // VMA 0xb4cc0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,15)?)?;
    // VMA 0xb4cc4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-40)?)?;
    // VMA 0xb4cc8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4ccc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4cd0
    c.b(C::Lt,0xb4b70,false)?;
    // VMA 0xb4cd4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,16)?)?;
    // VMA 0xb4cd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xb4cdc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,120)?)?;
    // VMA 0xb4ce0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb4ce4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4ce8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // VMA 0xb4cec
    c.b(C::Eq,0xb4d70,false)?;
    // VMA 0xb4cf0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(6)?,124)?)?;
    // VMA 0xb4cf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192&0xffffffff)?)?;
    // VMA 0xb4cf8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,227&0xffffffff)?)?;
    // VMA 0xb4cfc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb4d00
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4d04
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb4d08
    c.literal(R::R1,0xb4e18)?;
    // VMA 0xb4d0c
    c.literal(R::R2,0xb4e1c)?;
    // VMA 0xb4d10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb4d14
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4d18
    c.literal(R::R1,0xb4e10)?;
    // VMA 0xb4d1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb4d20
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4d24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb4d28
    c.b(C::Al,0xb4d74,false)?;
    // VMA 0xb4d2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,76&0xffffffff)?)?;
    // VMA 0xb4d30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,116&0xffffffff)?)?;
    // VMA 0xb4d34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb4d38
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb4d3c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb4d40
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4d44
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,4)?)?;
    // VMA 0xb4d48
    c.b(C::Ne,0xb4d68,false)?;
    // VMA 0xb4d4c
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(4)?,Shift::Lsl,15)?)?;
    // VMA 0xb4d50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4)?)?;
    // VMA 0xb4d54
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xb4d58
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4d5c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(7)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4d60
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,4)?)?;
    // VMA 0xb4d64
    c.b(C::Eq,0xb4d58,false)?;
    // VMA 0xb4d68
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb4d6c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb4d70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4d74
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(6)?,116)?)?;
    // VMA 0xb4d78
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,20)?)?;
    // VMA 0xb4d7c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb4d80
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4d84
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,5)?)?;
    // VMA 0xb4d88
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // VMA 0xb4d8c
    c.b(C::Eq,0xb4d30,false)?;
    // VMA 0xb4d90
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xb4d94
    c.literal(R::R1,0xb4e20)?;
    // VMA 0xb4d98
    c.literal(R::R2,0xb4e1c)?;
    // VMA 0xb4d9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb4da0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,228&0xffffffff)?)?;
    // VMA 0xb4da4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4da8
    c.literal(R::R1,0xb4e10)?;
    // VMA 0xb4dac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb4db0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4db4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb4db8
    c.b(C::Al,0xb4d30,false)?;
    // VMA 0xb4dbc
    c.literal(R::R2,0xb4e14)?;
    // VMA 0xb4dc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4dc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,155&0xffffffff)?)?;
    // VMA 0xb4dc8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb4dcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,127&0xffffffff)?)?;
    // VMA 0xb4dd0
    c.b(C::Al,0xb4d68,false)?;
    // VMA 0xb4dd4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(3)?,1)?)?;
    // VMA 0xb4dd8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // VMA 0xb4ddc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(9)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xb4de0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4de4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,448)?)?;
    // VMA 0xb4de8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(8)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb4dec
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,24)?)?;
    // VMA 0xb4df0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xb4df4
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,576)?)?;
    // VMA 0xb4df8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4dfc
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,83)?)?;
    // VMA 0xb4e00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,66&0xffffffff)?)?;
    // VMA 0xb4e04
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,20)?)?;
    // VMA 0xb4e08
    c.b(C::Al,0xb4d30,false)?;
    c.check(0xb4e0c)
}

const STREAM_FLUSH_DIAGNOSTICS:[u32;6]=[
    0x000f_8d68,0x000e_2788,0x000f_8d97,
    0x000f_7058,0x000f_8dd2,0x000f_7095,
];
