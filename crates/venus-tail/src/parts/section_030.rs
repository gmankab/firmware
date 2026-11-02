
/// Descriptor command output: initialize codec report slots, drain a bounded
/// hardware response ring, and update completion/callback flags. The original
/// function returns before a six-word diagnostic literal pool.
fn enqueue_codec_output_and_status(c:&mut Code)->Result<(),String>{
    c.check(0xb2a80)?;
    // VMA 0xb2a80
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb2a84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb2a88
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // VMA 0xb2a8c
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xb2a90
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,3624)?)?;
    // VMA 0xb2a94
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,36)?)?;
    // VMA 0xb2a98
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb2a9c
    c.b(C::Ne,0xb2ac8,false)?;
    // VMA 0xb2aa0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,288&0xffffffff)?)?;
    // VMA 0xb2aa4
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xb2aa8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb2aac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb2ab0
    c.b(C::Eq,0xb2d28,false)?;
    // VMA 0xb2ab4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb2ab8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2abc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,288&0xffffffff)?)?;
    // VMA 0xb2ac0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xb2ac4
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,3624)?)?;
    // VMA 0xb2ac8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048&0xffffffff)?)?;
    // VMA 0xb2acc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,3758096384)?)?;
    // VMA 0xb2ad0
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xb2ad4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb2ad8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb2adc
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // VMA 0xb2ae0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(4)?,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xb2ae4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb2ae8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb2aec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb2af0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb2af4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb2af8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb2afc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb2b00
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb2b04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb2b08
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // VMA 0xb2b0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb2b10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(9)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb2b14
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // VMA 0xb2b18
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(6)?,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xb2b1c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,89)?)?;
    // VMA 0xb2b20
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb2b24
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,90)?)?;
    // VMA 0xb2b28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb2b2c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb2b30
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xb2b34
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,88)?)?;
    // VMA 0xb2b38
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // VMA 0xb2b3c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,255)?)?;
    // VMA 0xb2b40
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xb2b44
    c.b(C::Al,0xb2d48,true)?;
    // VMA 0xb2b48
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,88)?)?;
    // VMA 0xb2b4c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xb2b50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,92)?)?;
    // VMA 0xb2b54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2b58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2b5c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xb2b60
    c.b(C::Al,0xb1a2c,true)?;
    // VMA 0xb2b64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb2b68
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb2b6c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb2b70
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,88)?)?;
    // VMA 0xb2b74
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb2b78
    c.b(C::Eq,0xb2bd4,false)?;
    // VMA 0xb2b7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb2b80
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xb2b84
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xb2b88
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb2b8c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xb2b90
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xb2b94
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xb2b98
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2b9c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(6)?,R::new(3)?,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xb2ba0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb2ba4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R2,236)?)?;
    // VMA 0xb2ba8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb2bac
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R2,244)?)?;
    // VMA 0xb2bb0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb2bb4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xb2bb8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb2bbc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,252)?)?;
    // VMA 0xb2bc0
    c.b(C::Eq,0xb2bd4,false)?;
    // VMA 0xb2bc4
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,88)?)?;
    // VMA 0xb2bc8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb2bcc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb2bd0
    c.b(C::Lt,0xb2b80,false)?;
    // VMA 0xb2bd4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xb2bd8
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R0,Shift::Lsl,15)?)?;
    // VMA 0xb2bdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb2be0
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::new(8)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2be4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(9)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2be8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // VMA 0xb2bec
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,91)?)?;
    // VMA 0xb2bf0
    c.b(C::Eq,0xb2c2c,false)?;
    // VMA 0xb2bf4
    c.literal(R::R1,0xb2d30)?;
    // VMA 0xb2bf8
    c.literal(R::R2,0xb2d34)?;
    // VMA 0xb2bfc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,123&0xffffffff)?)?;
    // VMA 0xb2c00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb2c04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xb2c08
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb2c0c
    c.literal(R::R1,0xb2d38)?;
    // VMA 0xb2c10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb2c14
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb2c18
    c.literal(R::R2,0xb2d3c)?;
    // VMA 0xb2c1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,892&0xffffffff)?)?;
    // VMA 0xb2c20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2c24
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb2c28
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb2c2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,3758096384)?)?;
    // VMA 0xb2c30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb2c34
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::new(8)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2c38
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(9)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2c3c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,284)?)?;
    // VMA 0xb2c40
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,120)?)?;
    // VMA 0xb2c44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb2c48
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2c4c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,16)?)?;
    // VMA 0xb2c50
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // VMA 0xb2c54
    c.b(C::Eq,0xb2c98,false)?;
    // VMA 0xb2c58
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,124)?)?;
    // VMA 0xb2c5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192&0xffffffff)?)?;
    // VMA 0xb2c60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,130&0xffffffff)?)?;
    // VMA 0xb2c64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb2c68
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xb2c6c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2c70
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb2c74
    c.literal(R::R1,0xb2d40)?;
    // VMA 0xb2c78
    c.literal(R::R2,0xb2d34)?;
    // VMA 0xb2c7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb2c80
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb2c84
    c.literal(R::R1,0xb2d38)?;
    // VMA 0xb2c88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb2c8c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb2c90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb2c94
    c.b(C::Al,0xb2c9c,false)?;
    // VMA 0xb2c98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2c9c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,116)?)?;
    // VMA 0xb2ca0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::R1,2048)?)?;
    // VMA 0xb2ca4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(4)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2ca8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,5)?)?;
    // VMA 0xb2cac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // VMA 0xb2cb0
    c.b(C::Ne,0xb2cd8,false)?;
    // VMA 0xb2cb4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb2cb8
    c.b(C::Ne,0xb2d04,false)?;
    // VMA 0xb2cbc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,89)?)?;
    // VMA 0xb2cc0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb2cc4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,3560)?)?;
    // VMA 0xb2cc8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,3624)?)?;
    // VMA 0xb2ccc
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,3560)?)?;
    // VMA 0xb2cd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2cd4
    c.b(C::Al,0xb2d04,false)?;
    // VMA 0xb2cd8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xb2cdc
    c.literal(R::R1,0xb2d44)?;
    // VMA 0xb2ce0
    c.literal(R::R2,0xb2d34)?;
    // VMA 0xb2ce4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,131&0xffffffff)?)?;
    // VMA 0xb2ce8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb2cec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xb2cf0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb2cf4
    c.literal(R::R1,0xb2d38)?;
    // VMA 0xb2cf8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb2cfc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb2d00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb2d04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb2d08
    c.b(C::Al,0xb2d10,false)?;
    // VMA 0xb2d0c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb2d10
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb2d14
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // VMA 0xb2d18
    c.b(C::Eq,0xb2d0c,false)?;
    // VMA 0xb2d1c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,2432)?)?;
    // VMA 0xb2d20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb2d24
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb2d28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,45&0xffffffff)?)?;
    // VMA 0xb2d2c
    c.b(C::Al,0xb2d1c,false)?;
    c.check(0xb2d30)
}

const CODEC_OUTPUT_DIAGNOSTICS:[u32;6]=[
    0x000f_89a4,0x000f_89ce,0x000e_2788,
    0x000f_88c7,0x000f_7058,0x000f_7095,
];

/// Read back controller response counts and populate per-plane status arrays,
/// with the same conditional early return as the original firmware.
fn read_codec_output_plane_records(c:&mut Code)->Result<(),String>{
    c.check(0xb2d48)?;
    // VMA 0xb2d48
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb2d4c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb2d50
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,36)?)?;
    // VMA 0xb2d54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048&0xffffffff)?)?;
    // VMA 0xb2d58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3758096384)?)?;
    // VMA 0xb2d5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2&0xffffffff)?)?;
    // VMA 0xb2d60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb2d64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4&0xffffffff)?)?;
    // VMA 0xb2d68
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R12,R::R1,1,true)?)?;
    // VMA 0xb2d6c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb2d70
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xb2d74
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::new(6)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xb2d78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,5&0xffffffff)?)?;
    // VMA 0xb2d7c
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(6)?,0)?)?;
    // VMA 0xb2d80
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xb2d84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,32&0xffffffff)?)?;
    // VMA 0xb2d88
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xb2d8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,0)?)?;
    // VMA 0xb2d90
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xb2d94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,12&0xffffffff)?)?;
    // VMA 0xb2d98
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xb2d9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4&0xffffffff)?)?;
    // VMA 0xb2da0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // VMA 0xb2da4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::new(7)?,R::new(3)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb2da8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // VMA 0xb2dac
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(5)?,R::new(7)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xb2db0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xb2db4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,0)?)?;
    // VMA 0xb2db8
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R2,1)?)?;
    // VMA 0xb2dbc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,0)?)?;
    // VMA 0xb2dc0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R2,2)?)?;
    // VMA 0xb2dc4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,0)?)?;
    // VMA 0xb2dc8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,4)?)?;
    // VMA 0xb2dcc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,0)?)?;
    // VMA 0xb2dd0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,0)?)?;
    // VMA 0xb2dd4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,0)?)?;
    // VMA 0xb2dd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,8&0xffffffff)?)?;
    // VMA 0xb2ddc
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,0)?)?;
    // VMA 0xb2de0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,0)?)?;
    // VMA 0xb2de4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R2,3)?)?;
    // VMA 0xb2de8
    c.b(C::Eq,0xb2f04,false)?;
    // VMA 0xb2dec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,8)?)?;
    // VMA 0xb2df0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2df4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xb2df8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // VMA 0xb2dfc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb2e00
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(9)?,R::new(7)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xb2e04
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xb2e08
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(4)?,R::new(3)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xb2e0c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(5)?,0)?)?;
    // VMA 0xb2e10
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(7)?,0)?)?;
    // VMA 0xb2e14
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xb2e18
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(5)?,8)?)?;
    // VMA 0xb2e1c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xb2e20
    c.b(C::Ne,0xb2df4,false)?;
    // VMA 0xb2e24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xb2e28
    c.b(C::Cc,0xb2e54,false)?;
    // VMA 0xb2e2c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,7)?)?;
    // VMA 0xb2e30
    c.b(C::Gt,0xb2e54,false)?;
    // VMA 0xb2e34
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,2)?)?;
    // VMA 0xb2e38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xb2e3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2&0xffffffff)?)?;
    // VMA 0xb2e40
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(6)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2e44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb2e48
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(8)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2e4c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // VMA 0xb2e50
    c.b(C::Lt,0xb2e40,false)?;
    // VMA 0xb2e54
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // VMA 0xb2e58
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb2e5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2e60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xb2e64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,32&0xffffffff)?)?;
    // VMA 0xb2e68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,12&0xffffffff)?)?;
    // VMA 0xb2e6c
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb2e70
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::LR,8)?)?;
    // VMA 0xb2e74
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb2e78
    c.b(C::Eq,0xb2ee0,false)?;
    // VMA 0xb2e7c
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xb2e80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2&0xffffffff)?)?;
    // VMA 0xb2e84
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(3)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xb2e88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb2e8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb2e90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,5&0xffffffff)?)?;
    // VMA 0xb2e94
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb2e98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4&0xffffffff)?)?;
    // VMA 0xb2e9c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xb2ea0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb2ea4
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xb2ea8
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,0)?)?;
    // VMA 0xb2eac
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // VMA 0xb2eb0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(9)?,R::new(3)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xb2eb4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::LR,24)?)?;
    // VMA 0xb2eb8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb2ebc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::LR,32)?)?;
    // VMA 0xb2ec0
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb2ec4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::LR,40)?)?;
    // VMA 0xb2ec8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xb2ecc
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,0)?)?;
    // VMA 0xb2ed0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,56)?)?;
    // VMA 0xb2ed4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb2ed8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb2edc
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xb2ee0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::LR,16)?)?;
    // VMA 0xb2ee4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb2ee8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb2eec
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,8&0xffffffff)?)?;
    // VMA 0xb2ef0
    c.emit(arm32::transfer_word_register_offset(C::Ne,false,R::new(3)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2ef4
    c.emit(arm32::transfer_word_register_offset(C::Ne,true,R::new(3)?,R::new(8)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb2ef8
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(3)?,R::LR,48)?)?;
    // VMA 0xb2efc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xb2f00
    c.b(C::Ne,0xb2e6c,false)?;
    // VMA 0xb2f04
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb2f08)
}
