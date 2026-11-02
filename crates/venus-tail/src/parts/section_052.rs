

/// Populate codec input/output register descriptors and compute ready flags
/// for the frame controller. Original function ends in a tail-call branch
/// after restoring LR, with a separate diagnostic pointer word.
fn initialize_frame_registers(c:&mut Code)->Result<(),String>{
    c.check(0xb9c64)?;
    // ARM VMA 0xb9c64
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb9c68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM VMA 0xb9c6c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,56)?)?;
    // ARM VMA 0xb9c70
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,4)?)?;
    // ARM VMA 0xb9c74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,104)?)?;
    // ARM VMA 0xb9c78
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,1024)?)?;
    // ARM VMA 0xb9c7c
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,4)?)?;
    // ARM VMA 0xb9c80
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1136)?)?;
    // ARM VMA 0xb9c84
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,52)?)?;
    // ARM VMA 0xb9c88
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,4)?)?;
    // ARM VMA 0xb9c8c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1140)?)?;
    // ARM VMA 0xb9c90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // ARM VMA 0xb9c94
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1160)?)?;
    // ARM VMA 0xb9c98
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1164)?)?;
    // ARM VMA 0xb9c9c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,724)?)?;
    // ARM VMA 0xb9ca0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb9ca4
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(6)?,972)?)?;
    // ARM VMA 0xb9ca8
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb9cac
    c.b(C::Ne,0xb9cc4,false)?;
    // ARM VMA 0xb9cb0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,200)?)?;
    // ARM VMA 0xb9cb4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb9cb8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // ARM VMA 0xb9cbc
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM VMA 0xb9cc0
    c.b(C::Al,0xb9ccc,false)?;
    // ARM VMA 0xb9cc4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,200)?)?;
    // ARM VMA 0xb9cc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM VMA 0xb9ccc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb9cd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb9cd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM VMA 0xb9cd8
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(6)?,1172)?)?;
    // ARM VMA 0xb9cdc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9ce0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb9ce4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1168)?)?;
    // ARM VMA 0xb9ce8
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(6)?,1175)?)?;
    // ARM VMA 0xb9cec
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(6)?,1176)?)?;
    // ARM VMA 0xb9cf0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,296)?)?;
    // ARM VMA 0xb9cf4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb9cf8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb9cfc
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,1181)?)?;
    // ARM VMA 0xb9d00
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,368)?)?;
    // ARM VMA 0xb9d04
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb9d08
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb9d0c
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,1182)?)?;
    // ARM VMA 0xb9d10
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,292)?)?;
    // ARM VMA 0xb9d14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb9d18
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb9d1c
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,1173)?)?;
    // ARM VMA 0xb9d20
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,364)?)?;
    // ARM VMA 0xb9d24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb9d28
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb9d2c
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,1174)?)?;
    // ARM VMA 0xb9d30
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,984)?)?;
    // ARM VMA 0xb9d34
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1144)?)?;
    // ARM VMA 0xb9d38
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,988)?)?;
    // ARM VMA 0xb9d3c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1148)?)?;
    // ARM VMA 0xb9d40
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,308)?)?;
    // ARM VMA 0xb9d44
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1152)?)?;
    // ARM VMA 0xb9d48
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,320)?)?;
    // ARM VMA 0xb9d4c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1156)?)?;
    // ARM VMA 0xb9d50
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,1296)?)?;
    // ARM VMA 0xb9d54
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb9d58
    c.b(C::Eq,0xb9e28,false)?;
    // ARM VMA 0xb9d5c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2008)?)?;
    // ARM VMA 0xb9d60
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1716)?)?;
    // ARM VMA 0xb9d64
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2012)?)?;
    // ARM VMA 0xb9d68
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1748)?)?;
    // ARM VMA 0xb9d6c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,2724)?)?;
    // ARM VMA 0xb9d70
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb9d74
    c.b(C::Eq,0xb9dc0,false)?;
    // ARM VMA 0xb9d78
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,868)?)?;
    // ARM VMA 0xb9d7c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,2016)?)?;
    // ARM VMA 0xb9d80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM VMA 0xb9d84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // ARM VMA 0xb9d88
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,0)?)?;
    // ARM VMA 0xb9d8c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // ARM VMA 0xb9d90
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,-112)?)?;
    // ARM VMA 0xb9d94
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,8)?)?;
    // ARM VMA 0xb9d98
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,-96)?)?;
    // ARM VMA 0xb9d9c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,24)?)?;
    // ARM VMA 0xb9da0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,-16)?)?;
    // ARM VMA 0xb9da4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,32)?)?;
    // ARM VMA 0xb9da8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    // ARM VMA 0xb9dac
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // ARM VMA 0xb9db0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // ARM VMA 0xb9db4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,2724)?)?;
    // ARM VMA 0xb9db8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9dbc
    c.b(C::Cc,0xb9d88,false)?;
    // ARM VMA 0xb9dc0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2032)?)?;
    // ARM VMA 0xb9dc4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1812)?)?;
    // ARM VMA 0xb9dc8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2036)?)?;
    // ARM VMA 0xb9dcc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1844)?)?;
    // ARM VMA 0xb9dd0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2664)?)?;
    // ARM VMA 0xb9dd4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1720)?)?;
    // ARM VMA 0xb9dd8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1752)?)?;
    // ARM VMA 0xb9ddc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2668)?)?;
    // ARM VMA 0xb9de0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1784)?)?;
    // ARM VMA 0xb9de4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1800)?)?;
    // ARM VMA 0xb9de8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2676)?)?;
    // ARM VMA 0xb9dec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1816)?)?;
    // ARM VMA 0xb9df0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1848)?)?;
    // ARM VMA 0xb9df4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2680)?)?;
    // ARM VMA 0xb9df8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1880)?)?;
    // ARM VMA 0xb9dfc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1896)?)?;
    // ARM VMA 0xb9e00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2684)?)?;
    // ARM VMA 0xb9e04
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // ARM VMA 0xb9e08
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,4)?)?;
    // ARM VMA 0xb9e0c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2136)?)?;
    // ARM VMA 0xb9e10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2144)?)?;
    // ARM VMA 0xb9e14
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2688)?)?;
    // ARM VMA 0xb9e18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // ARM VMA 0xb9e1c
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,4)?)?;
    // ARM VMA 0xb9e20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2160)?)?;
    // ARM VMA 0xb9e24
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2168)?)?;
    // ARM VMA 0xb9e28
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM VMA 0xb9e2c
    c.b(C::Al,0x31b80,true)?;
    // ARM VMA 0xb9e30
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,280)?)?;
    // ARM VMA 0xb9e34
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1024)?)?;
    // ARM VMA 0xb9e38
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM VMA 0xb9e3c
    c.b(C::Al,0x2fcbc,true)?;
    // ARM VMA 0xb9e40
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,1136)?)?;
    // ARM VMA 0xb9e44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,127)?)?;
    // ARM VMA 0xb9e48
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,3,false)?)?;
    // ARM VMA 0xb9e4c
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Asr,7)?)?;
    // ARM VMA 0xb9e50
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::R2,Shift::Lsl,16,false)?)?;
    // ARM VMA 0xb9e54
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Asr,7,false)?)?;
    // ARM VMA 0xb9e58
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R2,Shift::Lsl,16,false)?)?;
    // ARM VMA 0xb9e5c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::R1,Shift::Asr,7,false)?)?;
    // ARM VMA 0xb9e60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb9e64
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,1508)?)?;
    // ARM VMA 0xb9e68
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,1476)?)?;
    // ARM VMA 0xb9e6c
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,2276)?)?;
    // ARM VMA 0xb9e70
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,2280)?)?;
    // ARM VMA 0xb9e74
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,2284)?)?;
    // ARM VMA 0xb9e78
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,2288)?)?;
    // ARM VMA 0xb9e7c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2260)?)?;
    // ARM VMA 0xb9e80
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2264)?)?;
    // ARM VMA 0xb9e84
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2268)?)?;
    // ARM VMA 0xb9e88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9e8c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1292)?)?;
    // ARM VMA 0xb9e90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb9e94
    c.b(C::Al,0x311c0,true)?;
    // ARM VMA 0xb9e98
    c.literal(R::R2,0xb9eb4)?;
    // ARM VMA 0xb9e9c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9ea0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,124)?)?;
    // ARM VMA 0xb9ea4
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // ARM VMA 0xb9ea8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb9eac
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb9eb0
    c.b(C::Al,0x17b0,false)?;
    c.check(0xb9eb4)?;
    c.emit(0x000f_9ad3)?; // controller log pointer
    c.check(0xb9eb8)
}

/// Open the optional internal frame source, send attach-notifications, and
/// return a success/failure code through the original POP/PC epilogue.
fn attach_optional_input_source(c:&mut Code)->Result<(),String>{
    c.check(0xb9eb8)?;
    // ARM VMA 0xb9eb8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb9ebc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // ARM VMA 0xb9ec0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // ARM VMA 0xb9ec4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb9ec8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // ARM VMA 0xb9ecc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,300)?)?;
    // ARM VMA 0xb9ed0
    c.b(C::Al,0x80994,true)?;
    // ARM VMA 0xb9ed4
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // ARM VMA 0xb9ed8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9edc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM VMA 0xb9ee0
    c.b(C::Ne,0xb9f28,false)?;
    // ARM VMA 0xb9ee4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,4)?)?;
    // ARM VMA 0xb9ee8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9eec
    c.literal(R::R1,0xb9f78)?;
    // ARM VMA 0xb9ef0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,364)?)?;
    // ARM VMA 0xb9ef4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,720)?)?;
    // ARM VMA 0xb9ef8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb9efc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // ARM VMA 0xb9f00
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9f04
    c.literal(R::R1,0xb9f7c)?;
    // ARM VMA 0xb9f08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb9f0c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9f10
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,564)?)?;
    // ARM VMA 0xb9f14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9f18
    c.b(C::Eq,0xb9f30,false)?;
    // ARM VMA 0xb9f1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb9f20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,232)?)?;
    // ARM VMA 0xb9f24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9f28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // ARM VMA 0xb9f2c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb9f30
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,532)?)?;
    // ARM VMA 0xb9f34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9f38
    c.b(C::Eq,0xb9f1c,false)?;
    // ARM VMA 0xb9f3c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,520)?)?;
    // ARM VMA 0xb9f40
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9f44
    c.b(C::Eq,0xb9f64,false)?;
    // ARM VMA 0xb9f48
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1028)?)?;
    // ARM VMA 0xb9f4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9f50
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb9f54
    c.b(C::Ne,0xb9f20,false)?;
    // ARM VMA 0xb9f58
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,496)?)?;
    // ARM VMA 0xb9f5c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb9f60
    c.b(C::Ne,0xb9f20,false)?;
    // ARM VMA 0xb9f64
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,552)?)?;
    // ARM VMA 0xb9f68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9f6c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb9f70
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb9f74
    c.b(C::Al,0xb9f20,false)?;
    c.check(0xb9f78)?;
    const ATTACH_MESSAGE_POINTERS:[u32;2]=[0x000f_9b0b,0x000e_2788];
    for word in ATTACH_MESSAGE_POINTERS {c.emit(word)?;}
    c.check(0xb9f80)
}
