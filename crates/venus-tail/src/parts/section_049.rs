

/// Continue frame descriptor initialization: transfer conditionally packed
/// ARM command records into channel memory, call platform callbacks, set
/// per-plane field offsets and select one of four input buffer layouts.
/// This remains within the same function, with its epilogue still ahead.
fn write_frame_descriptor_command_records(c:&mut Code)->Result<(),String>{
    c.check(0xb8c00)?;
    // ARM VMA 0xb8c00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,56)?)?;
    // ARM VMA 0xb8c04
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // ARM VMA 0xb8c08
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,32)?)?;
    // ARM VMA 0xb8c0c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // ARM VMA 0xb8c10
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // ARM VMA 0xb8c14
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,36)?)?;
    // ARM VMA 0xb8c18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,248)?)?;
    // ARM VMA 0xb8c1c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1024)?)?;
    // ARM VMA 0xb8c20
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM VMA 0xb8c24
    c.b(C::Al,0x471c,true)?;
    // ARM VMA 0xb8c28
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,20)?)?;
    // ARM VMA 0xb8c2c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,184)?)?;
    // ARM VMA 0xb8c30
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,728)?)?;
    // ARM VMA 0xb8c34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // ARM VMA 0xb8c38
    c.b(C::Ne,0xb8c4c,false)?;
    // ARM VMA 0xb8c3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb8c40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // ARM VMA 0xb8c44
    c.b(C::Al,0x31ec,true)?;
    // ARM VMA 0xb8c48
    c.b(C::Al,0xb8c5c,false)?;
    // ARM VMA 0xb8c4c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,380)?)?;
    // ARM VMA 0xb8c50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // ARM VMA 0xb8c54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // ARM VMA 0xb8c58
    c.b(C::Al,0x471c,true)?;
    // ARM VMA 0xb8c5c
    c.literal(R::new(4)?,0xb98ec)?;
    // ARM VMA 0xb8c60
    c.literal(R::new(6)?,0xb98d8)?;
    // ARM VMA 0xb8c64
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,32)?)?;
    // ARM VMA 0xb8c68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM VMA 0xb8c6c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(7)?,Shift::Lsl,5,false)?)?;
    // ARM VMA 0xb8c70
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // ARM VMA 0xb8c74
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,2)?)?;
    // ARM VMA 0xb8c78
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8c7c
    c.b(C::Eq,0xb8cd0,false)?;
    // ARM VMA 0xb8c80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // ARM VMA 0xb8c84
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::new(9)?,Shift::Lsl,2,false)?)?;
    // ARM VMA 0xb8c88
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,76)?)?;
    // ARM VMA 0xb8c8c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,108)?)?;
    // ARM VMA 0xb8c90
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,140)?)?;
    // ARM VMA 0xb8c94
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,172)?)?;
    // ARM VMA 0xb8c98
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8c9c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM VMA 0xb8ca0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8ca4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // ARM VMA 0xb8ca8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // ARM VMA 0xb8cac
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // ARM VMA 0xb8cb0
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8cb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8cb8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb8cbc
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8cc0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // ARM VMA 0xb8cc4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // ARM VMA 0xb8cc8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8ccc
    c.b(C::Cc,0xb8c84,false)?;
    // ARM VMA 0xb8cd0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // ARM VMA 0xb8cd4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,132)?)?;
    // ARM VMA 0xb8cd8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // ARM VMA 0xb8cdc
    c.b(C::Ne,0xb8c6c,false)?;
    // ARM VMA 0xb8ce0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,32)?)?;
    // ARM VMA 0xb8ce4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8ce8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,64)?)?;
    // ARM VMA 0xb8cec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,68)?)?;
    // ARM VMA 0xb8cf0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,0)?)?;
    // ARM VMA 0xb8cf4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8cf8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,16)?)?;
    // ARM VMA 0xb8cfc
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,24)?)?;
    // ARM VMA 0xb8d00
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,44)?)?;
    // ARM VMA 0xb8d04
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,40)?)?;
    // ARM VMA 0xb8d08
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,36)?)?;
    // ARM VMA 0xb8d0c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,56)?)?;
    // ARM VMA 0xb8d10
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8d14
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // ARM VMA 0xb8d18
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8d1c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8d20
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,0)?)?;
    // ARM VMA 0xb8d24
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8d28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1116)?)?;
    // ARM VMA 0xb8d2c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,33554432)?)?;
    // ARM VMA 0xb8d30
    c.b(C::Eq,0xb8dac,false)?;
    // ARM VMA 0xb8d34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,48)?)?;
    // ARM VMA 0xb8d38
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM VMA 0xb8d3c
    c.emit(arm32::transfer_word(C::Cs,false,R::new(10)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb8d40
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,8)?)?;
    // ARM VMA 0xb8d44
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,13)?)?;
    // ARM VMA 0xb8d48
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,12)?)?;
    // ARM VMA 0xb8d4c
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,28)?)?;
    // ARM VMA 0xb8d50
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,16)?)?;
    // ARM VMA 0xb8d54
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,48)?)?;
    // ARM VMA 0xb8d58
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,0)?)?;
    // ARM VMA 0xb8d5c
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,676)?)?;
    // ARM VMA 0xb8d60
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,20)?)?;
    // ARM VMA 0xb8d64
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,680)?)?;
    // ARM VMA 0xb8d68
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,24)?)?;
    // ARM VMA 0xb8d6c
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,684)?)?;
    // ARM VMA 0xb8d70
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,28)?)?;
    // ARM VMA 0xb8d74
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,688)?)?;
    // ARM VMA 0xb8d78
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,32)?)?;
    // ARM VMA 0xb8d7c
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,692)?)?;
    // ARM VMA 0xb8d80
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,36)?)?;
    // ARM VMA 0xb8d84
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,696)?)?;
    // ARM VMA 0xb8d88
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,40)?)?;
    // ARM VMA 0xb8d8c
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,700)?)?;
    // ARM VMA 0xb8d90
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,44)?)?;
    // ARM VMA 0xb8d94
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8d98
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R1,48)?)?;
    // ARM VMA 0xb8d9c
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8da0
    c.emit(arm32::transfer_word(C::Cs,true,R::R2,R::new(4)?,0)?)?;
    // ARM VMA 0xb8da4
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::new(4)?,R::new(4)?,48)?)?;
    // ARM VMA 0xb8da8
    c.emit(arm32::data_register(C::Cs,Op::Add,R::R12,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8dac
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1024)?)?;
    // ARM VMA 0xb8db0
    c.b(C::Eq,0xb8dfc,false)?;
    // ARM VMA 0xb8db4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,24)?)?;
    // ARM VMA 0xb8db8
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM VMA 0xb8dbc
    c.emit(arm32::transfer_word(C::Cs,false,R::new(10)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb8dc0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,8)?)?;
    // ARM VMA 0xb8dc4
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::new(5)?,1)?)?;
    // ARM VMA 0xb8dc8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,12)?)?;
    // ARM VMA 0xb8dcc
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM VMA 0xb8dd0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,16)?)?;
    // ARM VMA 0xb8dd4
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,24)?)?;
    // ARM VMA 0xb8dd8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,0)?)?;
    // ARM VMA 0xb8ddc
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,720)?)?;
    // ARM VMA 0xb8de0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,20)?)?;
    // ARM VMA 0xb8de4
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8de8
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R1,24)?)?;
    // ARM VMA 0xb8dec
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8df0
    c.emit(arm32::transfer_word(C::Cs,true,R::R2,R::new(4)?,0)?)?;
    // ARM VMA 0xb8df4
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::new(4)?,R::new(4)?,24)?)?;
    // ARM VMA 0xb8df8
    c.emit(arm32::data_register(C::Cs,Op::Add,R::R12,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8dfc
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,32768)?)?;
    // ARM VMA 0xb8e00
    c.b(C::Eq,0xb98b0,false)?;
    // ARM VMA 0xb8e04
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,40)?)?;
    // ARM VMA 0xb8e08
    c.b(C::Cc,0xb98b0,false)?;
    // ARM VMA 0xb8e0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb8e10
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb8e14
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // ARM VMA 0xb8e18
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // ARM VMA 0xb8e1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // ARM VMA 0xb8e20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // ARM VMA 0xb8e24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // ARM VMA 0xb8e28
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // ARM VMA 0xb8e2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // ARM VMA 0xb8e30
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb8e34
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1008)?)?;
    // ARM VMA 0xb8e38
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,20)?)?;
    // ARM VMA 0xb8e3c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,1012)?)?;
    // ARM VMA 0xb8e40
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,24)?)?;
    // ARM VMA 0xb8e44
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1016)?)?;
    // ARM VMA 0xb8e48
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // ARM VMA 0xb8e4c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1020)?)?;
    // ARM VMA 0xb8e50
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,32)?)?;
    // ARM VMA 0xb8e54
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,1024)?)?;
    // ARM VMA 0xb8e58
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,36)?)?;
    // ARM VMA 0xb8e5c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8e60
    c.literal(R::R1,0xb98f0)?;
    // ARM VMA 0xb8e64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8e68
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8e6c
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb8e70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8e74
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8e78
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb8e7c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8e80
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // ARM VMA 0xb8e84
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(6)?))?;
    // ARM VMA 0xb8e88
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8e8c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,20)?)?;
    // ARM VMA 0xb8e90
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8e94
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8e98
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb8e9c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8ea0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1116)?)?;
    // ARM VMA 0xb8ea4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,65536)?)?;
    // ARM VMA 0xb8ea8
    c.b(C::Eq,0xb8efc,false)?;
    // ARM VMA 0xb8eac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,28)?)?;
    // ARM VMA 0xb8eb0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM VMA 0xb8eb4
    c.emit(arm32::transfer_word(C::Cs,false,R::new(10)?,R::R2,4)?)?;
    // ARM VMA 0xb8eb8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,8)?)?;
    // ARM VMA 0xb8ebc
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,16)?)?;
    // ARM VMA 0xb8ec0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,12)?)?;
    // ARM VMA 0xb8ec4
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,8)?)?;
    // ARM VMA 0xb8ec8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,16)?)?;
    // ARM VMA 0xb8ecc
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,28)?)?;
    // ARM VMA 0xb8ed0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,0)?)?;
    // ARM VMA 0xb8ed4
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,996)?)?;
    // ARM VMA 0xb8ed8
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,20)?)?;
    // ARM VMA 0xb8edc
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::new(9)?,1000)?)?;
    // ARM VMA 0xb8ee0
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::R2,24)?)?;
    // ARM VMA 0xb8ee4
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8ee8
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R1,28)?)?;
    // ARM VMA 0xb8eec
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8ef0
    c.emit(arm32::transfer_word(C::Cs,true,R::new(3)?,R::R2,0)?)?;
    // ARM VMA 0xb8ef4
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R2,R::R2,28)?)?;
    // ARM VMA 0xb8ef8
    c.emit(arm32::data_register(C::Cs,Op::Add,R::R12,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8efc
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,8388608)?)?;
    // ARM VMA 0xb8f00
    c.b(C::Eq,0xb8f78,false)?;
    // ARM VMA 0xb8f04
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,76)?)?;
    // ARM VMA 0xb8f08
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(7)?,R::R0,76)?)?;
    // ARM VMA 0xb8f0c
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb8f10
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,25)?)?;
    // ARM VMA 0xb8f14
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(3)?,R::R0,56)?)?;
    // ARM VMA 0xb8f18
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::LR,R::R2,8)?)?;
    // ARM VMA 0xb8f1c
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R2,R::new(7)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8f20
    c.emit(arm32::transfer_multiple(C::Cs,false,R::LR,R::R0.bit() | R::R1.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8f24
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R0,R::new(9)?,136)?)?;
    // ARM VMA 0xb8f28
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R1,R::R2,20)?)?;
    // ARM VMA 0xb8f2c
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R0,R::R0,1024)?)?;
    // ARM VMA 0xb8f30
    c.emit(arm32::transfer_multiple(C::Cs,true,R::R0,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb8f34
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R1,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb8f38
    c.emit(arm32::transfer_multiple(C::Cs,true,R::R0,R::new(3)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8f3c
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R0,R::new(9)?,164)?)?;
    // ARM VMA 0xb8f40
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R0,R::R0,1024)?)?;
    // ARM VMA 0xb8f44
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R1,R::new(3)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8f48
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R1,R::R2,48)?)?;
    // ARM VMA 0xb8f4c
    c.emit(arm32::transfer_multiple(C::Cs,true,R::R0,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb8f50
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R1,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb8f54
    c.emit(arm32::transfer_multiple(C::Cs,true,R::R0,R::new(3)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8f58
    c.emit(arm32::transfer_multiple(C::Cs,false,R::R1,R::new(3)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8f5c
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::SP,72)?)?;
    // ARM VMA 0xb8f60
    c.emit(arm32::transfer_word(C::Cs,true,R::new(5)?,R::SP,56)?)?;
    // ARM VMA 0xb8f64
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R1,R::R0,76)?)?;
    // ARM VMA 0xb8f68
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb8f6c
    c.emit(arm32::transfer_word(C::Cs,true,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb8f70
    c.emit(arm32::data_immediate(C::Cs,Op::Add,R::R2,R::R2,76)?)?;
    // ARM VMA 0xb8f74
    c.emit(arm32::data_register(C::Cs,Op::Add,R::R12,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8f78
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2448)?)?;
    // ARM VMA 0xb8f7c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8f80
    c.b(C::Ne,0xb8fc0,false)?;
    // ARM VMA 0xb8f84
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,24)?)?;
    // ARM VMA 0xb8f88
    c.b(C::Cc,0xb8fc0,false)?;
    // ARM VMA 0xb8f8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // ARM VMA 0xb8f90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb8f94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb8f98
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::new(3)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb8f9c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,8)?)?;
    // ARM VMA 0xb8fa0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,12)?)?;
    // ARM VMA 0xb8fa4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,16)?)?;
    // ARM VMA 0xb8fa8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,24)?)?;
    // ARM VMA 0xb8fac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,108)?)?;
    // ARM VMA 0xb8fb0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,64)?)?;
    // ARM VMA 0xb8fb4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,108)?)?;
    // ARM VMA 0xb8fb8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // ARM VMA 0xb8fbc
    c.b(C::Al,0x205c,true)?;
    // ARM VMA 0xb8fc0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,208)?)?;
    // ARM VMA 0xb8fc4
    c.b(C::Al,0x7eb40,true)?;
    // ARM VMA 0xb8fc8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8fcc
    c.b(C::Ne,0xb9804,false)?;
    // ARM VMA 0xb8fd0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,100)?)?;
    // ARM VMA 0xb8fd4
    c.b(C::Al,0x2bf4,true)?;
    // ARM VMA 0xb8fd8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,408)?)?;
    // ARM VMA 0xb8fdc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8fe0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,408)?)?;
    // ARM VMA 0xb8fe4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,100)?)?;
    // ARM VMA 0xb8fe8
    c.b(C::Al,0x2c34,true)?;
    // ARM VMA 0xb8fec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,108)?)?;
    // ARM VMA 0xb8ff0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,512)?)?;
    // ARM VMA 0xb8ff4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,108)?)?;
    // ARM VMA 0xb8ff8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // ARM VMA 0xb8ffc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,76)?)?;

    // ARM VMA 0xb9000
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb9004
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,0)?)?;
    // ARM VMA 0xb9008
    c.b(C::Al,0x26754,true)?;
    // ARM VMA 0xb900c
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // ARM VMA 0xb9010
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,168)?)?;
    // ARM VMA 0xb9014
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,108)?)?;
    // ARM VMA 0xb9018
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,152)?)?;
    // ARM VMA 0xb901c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,160)?)?;
    // ARM VMA 0xb9020
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,128)?)?;
    // ARM VMA 0xb9024
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb9028
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // ARM VMA 0xb902c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // ARM VMA 0xb9030
    c.literal(R::R1,0xb98f4)?;
    // ARM VMA 0xb9034
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // ARM VMA 0xb9038
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // ARM VMA 0xb903c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // ARM VMA 0xb9040
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9044
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb9048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // ARM VMA 0xb904c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb9050
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,972)?)?;
    // ARM VMA 0xb9054
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9058
    c.b(C::Ne,0xb9810,false)?;
    // ARM VMA 0xb905c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb9060
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // ARM VMA 0xb9064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9068
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,120)?)?;
    // ARM VMA 0xb906c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,368)?)?;
    // ARM VMA 0xb9070
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,540)?)?;
    // ARM VMA 0xb9074
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb9078
    c.b(C::Eq,0xb90a0,false)?;
    // ARM VMA 0xb907c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb9080
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(9)?,332)?)?;
    // ARM VMA 0xb9084
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(9)?,336)?)?;
    // ARM VMA 0xb9088
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(9)?,356)?)?;
    // ARM VMA 0xb908c
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(9)?,360)?)?;
    // ARM VMA 0xb9090
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9094
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9098
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb909c
    c.b(C::Al,0xb90a4,false)?;
    // ARM VMA 0xb90a0
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // ARM VMA 0xb90a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,128)?)?;
    // ARM VMA 0xb90a8
    c.literal(R::R1,0xb98f8)?;
    // ARM VMA 0xb90ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb90b0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,224)?)?;
    // ARM VMA 0xb90b4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,228)?)?;
    // ARM VMA 0xb90b8
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb90bc
    c.literal(R::R1,0xb98d8)?;
    // ARM VMA 0xb90c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb90c4
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb90c8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,212)?)?;
    // ARM VMA 0xb90cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb90d0
    c.b(C::Eq,0xb9838,false)?;
    // ARM VMA 0xb90d4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,20)?)?;
    // ARM VMA 0xb90d8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,124)?)?;
    // ARM VMA 0xb90dc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // ARM VMA 0xb90e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,168)?)?;
    // ARM VMA 0xb90e4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,32)?)?;
    // ARM VMA 0xb90e8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,160)?)?;
    // ARM VMA 0xb90ec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,24)?)?;
    // ARM VMA 0xb90f0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,172)?)?;
    // ARM VMA 0xb90f4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,164)?)?;
    // ARM VMA 0xb90f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb90fc
    c.b(C::Ne,0xb910c,false)?;
    // ARM VMA 0xb9100
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,520)?)?;
    // ARM VMA 0xb9104
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb9108
    c.b(C::Ne,0xb98c0,false)?;
    // ARM VMA 0xb910c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,572)?)?;
    // ARM VMA 0xb9110
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb9114
    c.b(C::Ne,0xb9850,false)?;
    // ARM VMA 0xb9118
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,48)?)?;
    // ARM VMA 0xb911c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb9120
    c.b(C::Ne,0xb96e4,false)?;
    // ARM VMA 0xb9124
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,464)?)?;
    // ARM VMA 0xb9128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb912c
    c.b(C::Eq,0xb9168,false)?;
    // ARM VMA 0xb9130
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,2452)?)?;
    // ARM VMA 0xb9134
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb9138
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(9)?,800)?)?;
    // ARM VMA 0xb913c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // ARM VMA 0xb9140
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9144
    c.emit(arm32::transfer_word(C::Ne,false,R::new(10)?,R::new(3)?,4)?)?;
    // ARM VMA 0xb9148
    c.emit(arm32::transfer_word(C::Ne,false,R::new(7)?,R::new(3)?,8)?)?;
    // ARM VMA 0xb914c
    c.emit(arm32::transfer_word(C::Ne,true,R::new(7)?,R::SP,28)?)?;
    // ARM VMA 0xb9150
    c.emit(arm32::transfer_word(C::Ne,false,R::new(7)?,R::new(3)?,12)?)?;
    // ARM VMA 0xb9154
    c.emit(arm32::transfer_word(C::Ne,true,R::new(7)?,R::R0,28)?)?;
    // ARM VMA 0xb9158
    c.emit(arm32::data_register(C::Ne,Op::Sub,R::new(7)?,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb915c
    c.emit(arm32::transfer_word_register(C::Ne,false,R::new(7)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // ARM VMA 0xb9160
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::R1,R::new(7)?,24)?)?;
    // ARM VMA 0xb9164
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(3)?,16)?)?;
    // ARM VMA 0xb9168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,68)?)?;
    // ARM VMA 0xb916c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,72)?)?;
    // ARM VMA 0xb9170
    c.b(C::Al,0x80134,true)?;
    // ARM VMA 0xb9174
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,212)?)?;
    // ARM VMA 0xb9178
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,64)?)?;
    // ARM VMA 0xb917c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,60)?)?;
    // ARM VMA 0xb9180
    c.b(C::Al,0x801a8,true)?;
    // ARM VMA 0xb9184
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,800)?)?;
    // ARM VMA 0xb9188
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // ARM VMA 0xb918c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,60)?)?;
    // ARM VMA 0xb9190
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9194
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // ARM VMA 0xb919c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // ARM VMA 0xb91a0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb91a4
    c.b(C::Al,0x205c,true)?;
    // ARM VMA 0xb91a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1116)?)?;
    // ARM VMA 0xb91ac
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM VMA 0xb91b0
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb91b4
    c.b(C::Eq,0xb9240,false)?;
    // ARM VMA 0xb91b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,28)?)?;
    // ARM VMA 0xb91bc
    c.b(C::Cc,0xb9240,false)?;
    // ARM VMA 0xb91c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,28)?)?;
    // ARM VMA 0xb91c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // ARM VMA 0xb91c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // ARM VMA 0xb91cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // ARM VMA 0xb91d0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(4)?,8)?)?;
    // ARM VMA 0xb91d4
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(4)?,R::new(7)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb91d8
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb91dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,20)?)?;
    // ARM VMA 0xb91e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,732)?)?;
    // ARM VMA 0xb91e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // ARM VMA 0xb91e8
    c.b(C::Hi,0xb9208,false)?;
    // ARM VMA 0xb91ec
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // ARM VMA 0xb91f0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::PC,0)?)?;
    // ARM VMA 0xb91f4
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0xb91f8)?;
    const FRAME_INPUT_LAYOUT_TARGETS:[u32;4]=[0xb9208,0xb9210,0xb9218,0xb976c];
    for value in FRAME_INPUT_LAYOUT_TARGETS {c.emit(value)?;}
    c.check(0xb9208)
}
