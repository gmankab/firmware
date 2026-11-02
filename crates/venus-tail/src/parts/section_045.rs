
/// Calculate a channel-data CRC against the platform table and close the
/// matching buffer handle. This is the original MSB-first CRC32 polynomial
/// 0x04c11db7, with its two table globals modeled as separate data words.
fn calculate_codec_buffer_crc32(c:&mut Code)->Result<(),String>{
    c.check(0xb7bd8)?;
    // ARM VMA 0xb7bd8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb7bdc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM VMA 0xb7be0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // ARM VMA 0xb7be4
    c.literal(R::R12,0xb7ca8)?;
    // ARM VMA 0xb7be8
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // ARM VMA 0xb7bec
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb7bf0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(8)?,3)?)?;
    // ARM VMA 0xb7bf4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R12,0)?)?;
    // ARM VMA 0xb7bf8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM VMA 0xb7bfc
    c.b(C::Ne,0xb7c50,false)?;
    // ARM VMA 0xb7c00
    c.literal(R::new(9)?,0xb7cac)?;
    // ARM VMA 0xb7c04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM VMA 0xb7c08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM VMA 0xb7c0c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(10)?))?;
    // ARM VMA 0xb7c10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM VMA 0xb7c14
    c.literal(R::new(5)?,0xb7cb0)?;
    // ARM VMA 0xb7c18
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(4)?,Shift::Lsl,1)?)?;
    // ARM VMA 0xb7c1c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb7c20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // ARM VMA 0xb7c24
    c.emit(arm32::data_register(C::Lt,Op::Eor,R::new(3)?,R::new(5)?,R::new(4)?,Shift::Lsl,1,false)?)?;
    // ARM VMA 0xb7c28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // ARM VMA 0xb7c2c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // ARM VMA 0xb7c30
    c.b(C::Ne,0xb7c14,false)?;
    // ARM VMA 0xb7c34
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // ARM VMA 0xb7c38
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::new(9)?,4)?)?;
    // ARM VMA 0xb7c3c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,16777216)?)?;
    // ARM VMA 0xb7c40
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,256)?)?;
    // ARM VMA 0xb7c44
    c.b(C::Ne,0xb7c0c,false)?;
    // ARM VMA 0xb7c48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM VMA 0xb7c4c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // ARM VMA 0xb7c50
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R12,R::LR,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb7c54
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb7c58
    c.b(C::Eq,0xb7c90,false)?;
    // ARM VMA 0xb7c5c
    c.literal(R::new(6)?,0xb7cac)?;
    // ARM VMA 0xb7c60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM VMA 0xb7c64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM VMA 0xb7c68
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb7c6c
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(3)?,R::new(8)?,R::new(3)?,Shift::Lsl,0)?)?;
    // ARM VMA 0xb7c70
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb7c74
    c.emit(arm32::data_register(C::Al,Op::Eor,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsr,24,false)?)?;
    // ARM VMA 0xb7c78
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,2)?)?;
    // ARM VMA 0xb7c7c
    c.emit(arm32::data_register(C::Al,Op::Eor,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,8,false)?)?;
    // ARM VMA 0xb7c80
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // ARM VMA 0xb7c84
    c.b(C::Cc,0xb7c68,false)?;
    // ARM VMA 0xb7c88
    c.emit(arm32::data_register(C::Al,Op::Mvn,R::new(4)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb7c8c
    c.b(C::Al,0xb7c94,false)?;
    // ARM VMA 0xb7c90
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(4)?,R::R0,0)?)?;
    // ARM VMA 0xb7c94
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // ARM VMA 0xb7c98
    c.b(C::Al,0x205c,true)?;
    // ARM VMA 0xb7c9c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7ca0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM VMA 0xb7ca4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb7ca8)?;
    for value in BUFFER_CRC_TABLE_AND_POLYNOMIAL {c.emit(value)?;}
    c.check(0xb7cb4)
}

/// Copy an active codec output descriptor to the hardware slot. Validate the
/// expected controller buffer sizes, then publish the record status.
fn publish_codec_active_record(c:&mut Code)->Result<(),String>{
    c.check(0xb7cb4)?;
    // ARM VMA 0xb7cb4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb7cb8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // ARM VMA 0xb7cbc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,64)?)?;
    // ARM VMA 0xb7cc0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb7cc4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // ARM VMA 0xb7cc8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2740)?)?;
    // ARM VMA 0xb7ccc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM VMA 0xb7cd0
    c.b(C::Ne,0xb7d08,false)?;
    // ARM VMA 0xb7cd4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,504)?)?;
    // ARM VMA 0xb7cd8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb7cdc
    c.b(C::Eq,0xb7d00,false)?;
    // ARM VMA 0xb7ce0
    c.literal(R::R1,0xb7d90)?;
    // ARM VMA 0xb7ce4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb7ce8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7cec
    c.literal(R::R1,0xb7d94)?;
    // ARM VMA 0xb7cf0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb7cf4
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7cf8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7cfc
    c.b(C::Al,0xb7d88,false)?;
    // ARM VMA 0xb7d00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb7d04
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2740)?)?;
    // ARM VMA 0xb7d08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,116)?)?;
    // ARM VMA 0xb7d0c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // ARM VMA 0xb7d10
    c.b(C::Al,0x80994,true)?;
    // ARM VMA 0xb7d14
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // ARM VMA 0xb7d18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7d1c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM VMA 0xb7d20
    c.b(C::Ne,0xb7d88,false)?;
    // ARM VMA 0xb7d24
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // ARM VMA 0xb7d28
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2120)?)?;
    // ARM VMA 0xb7d2c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // ARM VMA 0xb7d30
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2116)?)?;
    // ARM VMA 0xb7d34
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // ARM VMA 0xb7d38
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2112)?)?;
    // ARM VMA 0xb7d3c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // ARM VMA 0xb7d40
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2108)?)?;
    // ARM VMA 0xb7d44
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // ARM VMA 0xb7d48
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2104)?)?;
    // ARM VMA 0xb7d4c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // ARM VMA 0xb7d50
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2100)?)?;
    // ARM VMA 0xb7d54
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // ARM VMA 0xb7d58
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2096)?)?;
    // ARM VMA 0xb7d5c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // ARM VMA 0xb7d60
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2092)?)?;
    // ARM VMA 0xb7d64
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM VMA 0xb7d68
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2088)?)?;
    // ARM VMA 0xb7d6c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM VMA 0xb7d70
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2084)?)?;
    // ARM VMA 0xb7d74
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2128)?)?;
    // ARM VMA 0xb7d78
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2124)?)?;
    // ARM VMA 0xb7d7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7d80
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb7d84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7d88
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // ARM VMA 0xb7d8c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb7d90)?;
    for value in ACTIVE_BUFFER_TRACE {c.emit(value)?;}
    c.check(0xb7d98)
}

/// Allocate output descriptors, build the codec call packet and process the
/// deferred callback list, preserving the original small fallback loop.
fn initialize_codec_output_message_queue(c:&mut Code)->Result<(),String>{
    c.check(0xb7d98)?;
    // ARM VMA 0xb7d98
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb7d9c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // ARM VMA 0xb7da0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // ARM VMA 0xb7da4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // ARM VMA 0xb7da8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb7dac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // ARM VMA 0xb7db0
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,4)?)?;
    // ARM VMA 0xb7db4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,360)?)?;
    // ARM VMA 0xb7db8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,356)?)?;
    // ARM VMA 0xb7dbc
    c.b(C::Al,0x7f974,true)?;
    // ARM VMA 0xb7dc0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,4)?)?;
    // ARM VMA 0xb7dc4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // ARM VMA 0xb7dc8
    c.b(C::Eq,0xb7f28,false)?;
    // ARM VMA 0xb7dcc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,360)?)?;
    // ARM VMA 0xb7dd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,516)?)?;
    // ARM VMA 0xb7dd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb7dd8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // ARM VMA 0xb7ddc
    c.b(C::Al,0x31ec,true)?;
    // ARM VMA 0xb7de0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,360)?)?;
    // ARM VMA 0xb7de4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,204)?)?;
    // ARM VMA 0xb7de8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(5)?,552)?)?;
    // ARM VMA 0xb7dec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2120)?)?;
    // ARM VMA 0xb7df0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,532)?)?;
    // ARM VMA 0xb7df4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2116)?)?;
    // ARM VMA 0xb7df8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,524)?)?;
    // ARM VMA 0xb7dfc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2112)?)?;
    // ARM VMA 0xb7e00
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,520)?)?;
    // ARM VMA 0xb7e04
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2108)?)?;
    // ARM VMA 0xb7e08
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,516)?)?;
    // ARM VMA 0xb7e0c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2104)?)?;
    // ARM VMA 0xb7e10
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,504)?)?;
    // ARM VMA 0xb7e14
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb7e18
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2100)?)?;
    // ARM VMA 0xb7e1c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,500)?)?;
    // ARM VMA 0xb7e20
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2096)?)?;
    // ARM VMA 0xb7e24
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,496)?)?;
    // ARM VMA 0xb7e28
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2128)?)?;
    // ARM VMA 0xb7e2c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,512)?)?;
    // ARM VMA 0xb7e30
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2124)?)?;
    // ARM VMA 0xb7e34
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,508)?)?;
    // ARM VMA 0xb7e38
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2092)?)?;
    // ARM VMA 0xb7e3c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,484)?)?;
    // ARM VMA 0xb7e40
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2088)?)?;
    // ARM VMA 0xb7e44
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,492)?)?;
    // ARM VMA 0xb7e48
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2084)?)?;
    // ARM VMA 0xb7e4c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,488)?)?;
    // ARM VMA 0xb7e50
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    // ARM VMA 0xb7e54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R2,1)?)?;
    // ARM VMA 0xb7e58
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // ARM VMA 0xb7e5c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,0)?)?;
    // ARM VMA 0xb7e60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // ARM VMA 0xb7e64
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,448)?)?;
    // ARM VMA 0xb7e68
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,456)?)?;
    // ARM VMA 0xb7e6c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(6)?,1)?)?;
    // ARM VMA 0xb7e70
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(6)?,Shift::Lsr,7,false)?)?;
    // ARM VMA 0xb7e74
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(7)?,R::new(6)?,Shift::Lsr,21,false)?)?;
    // ARM VMA 0xb7e78
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit() | R::R1.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb7e7c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM VMA 0xb7e80
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2156)?)?;
    // ARM VMA 0xb7e84
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb7e88
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,716)?)?;
    // ARM VMA 0xb7e8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,784)?)?;
    // ARM VMA 0xb7e90
    c.b(C::Eq,0xb7ebc,false)?;
    // ARM VMA 0xb7e94
    c.literal(R::R1,0xb7f84)?;
    // ARM VMA 0xb7e98
    c.literal(R::R2,0xb7f88)?;
    // ARM VMA 0xb7e9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,73)?)?;
    // ARM VMA 0xb7ea0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb7ea4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // ARM VMA 0xb7ea8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7eac
    c.literal(R::R1,0xb7f8c)?;
    // ARM VMA 0xb7eb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb7eb4
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7eb8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // ARM VMA 0xb7ebc
    c.literal(R::R1,0xb7f90)?;
    // ARM VMA 0xb7ec0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7ec4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,564)?)?;
    // ARM VMA 0xb7ec8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,208)?)?;
    // ARM VMA 0xb7ecc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,212)?)?;
    // ARM VMA 0xb7ed0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,216)?)?;
    // ARM VMA 0xb7ed4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,220)?)?;
    // ARM VMA 0xb7ed8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb7edc
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,1028)?)?;
    // ARM VMA 0xb7ee0
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7ee4
    c.literal(R::R1,0xb7f8c)?;
    // ARM VMA 0xb7ee8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb7eec
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7ef0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,532)?)?;
    // ARM VMA 0xb7ef4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7ef8
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,520)?)?;
    // ARM VMA 0xb7efc
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7f00
    c.b(C::Ne,0xb7f30,false)?;
    // ARM VMA 0xb7f04
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,552)?)?;
    // ARM VMA 0xb7f08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7f0c
    c.b(C::Eq,0xb7f40,false)?;
    // ARM VMA 0xb7f10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb7f14
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb7f18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7f1c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,472)?)?;
    // ARM VMA 0xb7f20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2132)?)?;
    // ARM VMA 0xb7f24
    c.b(C::Al,0xb7f38,false)?;
    // ARM VMA 0xb7f28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7f2c
    c.b(C::Al,0xb7f7c,false)?;
    // ARM VMA 0xb7f30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb7f34
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb7f38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7f3c
    c.b(C::Al,0xb7f7c,false)?;
    // ARM VMA 0xb7f40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb7f44
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM VMA 0xb7f48
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // ARM VMA 0xb7f4c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,572)?)?;
    // ARM VMA 0xb7f50
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7f54
    c.b(C::Al,0xb7f94,true)?;
    // ARM VMA 0xb7f58
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,204)?)?;
    // ARM VMA 0xb7f5c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7f60
    c.b(C::Eq,0xb7f74,false)?;
    // ARM VMA 0xb7f64
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb7f68
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7f6c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7f70
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,204)?)?;
    // ARM VMA 0xb7f74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7f78
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb7f7c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // ARM VMA 0xb7f80
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb7f84)?;
    const OUTPUT_MESSAGE_LOGS:[u32;4]=[0x000f_95ba,0x000f_9621,0x000e_2788,0x000f_9638];
    for value in OUTPUT_MESSAGE_LOGS {c.emit(value)?;}
    c.check(0xb7f94)
}

/// Serialize active buffer identity, codec frame coordinates, and relay
/// descriptors into one outgoing record for platform tracing and handoff.
fn serialize_active_codec_output_record(c:&mut Code)->Result<(),String>{
    c.check(0xb7f94)?;
    // ARM VMA 0xb7f94
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb7f98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM VMA 0xb7f9c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,96)?)?;
    // ARM VMA 0xb7fa0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM VMA 0xb7fa4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // ARM VMA 0xb7fa8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // ARM VMA 0xb7fac
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,0)?)?;
    // ARM VMA 0xb7fb0
    c.b(C::Al,0x26754,true)?;
    // ARM VMA 0xb7fb4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,532)?)?;
    // ARM VMA 0xb7fb8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // ARM VMA 0xb7fbc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,524)?)?;
    // ARM VMA 0xb7fc0
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // ARM VMA 0xb7fc4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // ARM VMA 0xb7fc8
    c.literal(R::R1,0xb80c8)?;
    // ARM VMA 0xb7fcc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM VMA 0xb7fd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // ARM VMA 0xb7fd4
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7fd8
    c.literal(R::R1,0xb80cc)?;
    // ARM VMA 0xb7fdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // ARM VMA 0xb7fe0
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7fe4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,80)?)?;
    // ARM VMA 0xb7fe8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,13)?)?;
    // ARM VMA 0xb7fec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM VMA 0xb7ff0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // ARM VMA 0xb7ff4
    c.literal(R::R0,0xb80d0)?;
    // ARM VMA 0xb7ff8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // ARM VMA 0xb7ffc
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(5)?,4)?)?;
    // ARM VMA 0xb8000
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,0)?)?;
    // ARM VMA 0xb8004
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // ARM VMA 0xb8008
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,572)?)?;
    // ARM VMA 0xb800c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8010
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // ARM VMA 0xb8014
    c.b(C::Eq,0xb802c,false)?;
    // ARM VMA 0xb8018
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,1028)?)?;
    // ARM VMA 0xb801c
    c.literal(R::R2,0xb80d4)?;
    // ARM VMA 0xb8020
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM VMA 0xb8024
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb8028
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // ARM VMA 0xb802c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,8)?)?;
    // ARM VMA 0xb8030
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb8034
    c.b(C::Eq,0xb80b4,false)?;
    // ARM VMA 0xb8038
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb803c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,88)?)?;
    // ARM VMA 0xb8040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // ARM VMA 0xb8044
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // ARM VMA 0xb8048
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    // ARM VMA 0xb804c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,52)?)?;
    // ARM VMA 0xb8050
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2140)?)?;
    // ARM VMA 0xb8054
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // ARM VMA 0xb8058
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,2,false)?)?;
    // ARM VMA 0xb805c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,136)?)?;
    // ARM VMA 0xb8060
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1024)?)?;
    // ARM VMA 0xb8064
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,12)?)?;
    // ARM VMA 0xb8068
    c.emit(arm32::transfer_multiple(C::Al,true,R::R2,R::R0.bit() | R::new(5)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb806c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,44)?)?;
    // ARM VMA 0xb8070
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit() | R::new(5)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb8074
    c.emit(arm32::transfer_multiple(C::Al,true,R::R2,R::R0.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8078
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb807c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,1536)?)?;
    // ARM VMA 0xb8080
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // ARM VMA 0xb8084
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,512)?)?;
    // ARM VMA 0xb8088
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM VMA 0xb808c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,520)?)?;
    // ARM VMA 0xb8090
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM VMA 0xb8094
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,524)?)?;
    // ARM VMA 0xb8098
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // ARM VMA 0xb809c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,508)?)?;
    // ARM VMA 0xb80a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // ARM VMA 0xb80a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,128)?)?;
    // ARM VMA 0xb80a8
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb80ac
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // ARM VMA 0xb80b0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb80b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM VMA 0xb80b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,88)?)?;
    // ARM VMA 0xb80bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // ARM VMA 0xb80c0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,92)?)?;
    // ARM VMA 0xb80c4
    c.b(C::Al,0xb8040,false)?;
    c.check(0xb80c8)?;
    for value in CODEC_WORKER_TRACE {c.emit(value)?;}
    c.check(0xb80d8)
}

const CODEC_PLANE_SIZE_DISPATCH:[u32;4]=[0x000b_82dc,0x000b_8358,0x000b_8364,0x000b_84cc];
const CODEC_PLANE_CONFIGURATION_LOGS:[u32;8]=[
    0x000f_9661,0x000f_967a,0x000e_2788,0x000f_9560,
    0x000f_9810,0x000f_9685,0x000f_96b5,0x000f_96e1,
];
