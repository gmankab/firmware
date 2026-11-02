
/// The codec-plane session's longer error/fallback path: accumulation over
/// per-plane halfword tables, controller bounds checks and diagnostics. An
/// independently classified fourteen-word literal pool follows at 0xaa1f4.
fn handle_codec_plane_blending_failures(c:&mut Code)->Result<(),String>{
    c.check(0xa9cf0)?;
    // VMA 0xa9cf0
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(9)?,24)?)?;
    // VMA 0xa9cf4
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,22)?)?;
    // VMA 0xa9cf8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xa9cfc
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // VMA 0xa9d00
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,28)?)?;
    // VMA 0xa9d04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9d08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // VMA 0xa9d0c
    c.literal(R::R2,0xaa218)?;
    // VMA 0xa9d10
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa9d14
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // VMA 0xa9d18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xa9d1c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xa9d20
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa9d24
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xa9d28
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,2492)?)?;
    // VMA 0xa9d2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,0)?)?;
    // VMA 0xa9d30
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,24)?)?;
    // VMA 0xa9d34
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(9)?,22)?)?;
    // VMA 0xa9d38
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa9d3c
    c.b(C::Eq,0xa9d90,false)?;
    // VMA 0xa9d40
    c.emit(arm32::mov_shifted_register(C::Al,R::LR,R::new(6)?,Shift::Lsl,2)?)?;
    // VMA 0xa9d44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R12,4)?)?;
    // VMA 0xa9d48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xa9d4c
    c.b(C::Eq,0xa9d80,false)?;
    // VMA 0xa9d50
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xa9d54
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9d58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xa9d5c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xa9d60
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::R0,0)?)?;
    // VMA 0xa9d64
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // VMA 0xa9d68
    c.emit(arm32::load_halfword_postindexed(C::Al,R::R2,R::R0,2)?)?;
    // VMA 0xa9d6c
    c.emit(arm32::multiply_register(C::Al,R::new(4)?,R::new(7)?,R::R2)?)?;
    // VMA 0xa9d70
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::new(5)?,Shift::Lsl,2)?)?;
    // VMA 0xa9d74
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xa9d78
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9d7c
    c.b(C::Lt,0xa9d68,false)?;
    // VMA 0xa9d80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xa9d84
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xa9d88
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9d8c
    c.b(C::Lt,0xa9d48,false)?;
    // VMA 0xa9d90
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xa9d94
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::new(6)?)?)?;
    // VMA 0xa9d98
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(10)?))?;
    // VMA 0xa9d9c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9da0
    c.b(C::Eq,0xa9e04,false)?;
    // VMA 0xa9da4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,0)?)?;
    // VMA 0xa9da8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa9dac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xa9db0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa9db4
    c.b(C::Eq,0xa9df4,false)?;
    // VMA 0xa9db8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9dbc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R12,4)?)?;
    // VMA 0xa9dc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // VMA 0xa9dc4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xa9dc8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,2)?)?;
    // VMA 0xa9dcc
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::new(7)?,R::new(7)?,2,true)?)?;
    // VMA 0xa9dd0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9dd4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xa9dd8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,4)?)?;
    // VMA 0xa9ddc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9de0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,4)?)?;
    // VMA 0xa9de4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // VMA 0xa9de8
    c.b(C::Ne,0xa9dc4,false)?;
    // VMA 0xa9dec
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::new(5)?,1)?)?;
    // VMA 0xa9df0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xa9df4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xa9df8
    c.emit(arm32::transfer_word_register(C::Ne,true,R::R0,R::R12,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xa9dfc
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9e00
    c.emit(arm32::transfer_word_register(C::Ne,false,R::R0,R::R12,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xa9e04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,106)?)?;
    // VMA 0xa9e08
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(5)?))?;
    // VMA 0xa9e0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xa9e10
    c.emit(arm32::load_halfword_register_preindexed_writeback(C::Al,R::R2,R::new(10)?,R::R0)?)?;
    // VMA 0xa9e14
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xa9e18
    c.emit(arm32::load_halfword_preindexed_writeback(C::Al,R::new(3)?,R::R0,-2)?)?;
    // VMA 0xa9e1c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // VMA 0xa9e20
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(3)?,R::R2)?)?;
    // VMA 0xa9e24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9e28
    c.b(C::Eq,0xaa08c,false)?;
    // VMA 0xa9e2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa9e30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xa9e34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xa9e38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xa9e3c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,32)?)?;
    // VMA 0xa9e40
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // VMA 0xa9e44
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa9e48
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // VMA 0xa9e4c
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    // VMA 0xa9e50
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xa9e54
    c.b(C::Al,0x81838,true)?;
    // VMA 0xa9e58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,255)?)?;
    // VMA 0xa9e5c
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xa9e60
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // VMA 0xa9e64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,65280)?)?;
    // VMA 0xa9e68
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9e6c
    c.b(C::Eq,0xa9ea8,false)?;
    // VMA 0xa9e70
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xa9e74
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9e78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9e7c
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(9)?))?;
    // VMA 0xa9e80
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::R2,0)?)?;
    // VMA 0xa9e84
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(10)?))?;
    // VMA 0xa9e88
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(10)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9e8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xa9e90
    c.b(C::Lt,0xa9eb0,false)?;
    // VMA 0xa9e94
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa9e98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,2)?)?;
    // VMA 0xa9e9c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ea0
    c.b(C::Lt,0xa9e80,false)?;
    // VMA 0xa9ea4
    c.b(C::Al,0xa9eb4,false)?;
    // VMA 0xa9ea8
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(9)?))?;
    // VMA 0xa9eac
    c.b(C::Al,0xa9eb4,false)?;
    // VMA 0xa9eb0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // VMA 0xa9eb4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // VMA 0xa9eb8
    c.emit(arm32::load_halfword(C::Al,R::R1,R::R0,24)?)?;
    // VMA 0xa9ebc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa9ec0
    c.b(C::Eq,0xa9efc,false)?;
    // VMA 0xa9ec4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xa9ec8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ecc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xa9ed0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9ed4
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::R2,0)?)?;
    // VMA 0xa9ed8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(7)?))?;
    // VMA 0xa9edc
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ee0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xa9ee4
    c.b(C::Lt,0xa9f04,false)?;
    // VMA 0xa9ee8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa9eec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,2)?)?;
    // VMA 0xa9ef0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ef4
    c.b(C::Lt,0xa9ed4,false)?;
    // VMA 0xa9ef8
    c.b(C::Al,0xa9f08,false)?;
    // VMA 0xa9efc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9f00
    c.b(C::Al,0xa9f08,false)?;
    // VMA 0xa9f04
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xa9f08
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9f0c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(7)?,R::R2,R::new(10)?)?)?;
    // VMA 0xa9f10
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // VMA 0xa9f14
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9f18
    c.b(C::Le,0xa9f80,false)?;
    // VMA 0xa9f1c
    c.literal(R::R1,0xaa21c)?;
    // VMA 0xa9f20
    c.literal(R::R2,0xaa220)?;
    // VMA 0xa9f24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,54)?)?;
    // VMA 0xa9f28
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa9f2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // VMA 0xa9f30
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,12)?)?;
    // VMA 0xa9f34
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xa9f38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9f3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xa9f40
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9f44
    c.literal(R::R1,0xaa208)?;
    // VMA 0xa9f48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9f4c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9f50
    c.literal(R::R2,0xaa218)?;
    // VMA 0xa9f54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,55)?)?;
    // VMA 0xa9f58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9f5c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xa9f60
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa9f64
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xa9f68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,255)?)?;
    // VMA 0xa9f6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,65280)?)?;
    // VMA 0xa9f70
    c.emit(arm32::load_halfword(C::Al,R::LR,R::R0,0)?)?;
    // VMA 0xa9f74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xa9f78
    c.emit(arm32::load_halfword(C::Al,R::R0,R::R0,0)?)?;
    // VMA 0xa9f7c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // VMA 0xa9f80
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,44)?)?;
    // VMA 0xa9f84
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-32)?)?;
    // VMA 0xa9f88
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(10)?,Shift::Lsl,1)?)?;
    // VMA 0xa9f8c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,2492)?)?;
    // VMA 0xa9f90
    c.emit(arm32::load_halfword_register_preindexed_writeback(C::Al,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xa9f94
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(6)?,Shift::Lsl,2)?)?;
    // VMA 0xa9f98
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9f9c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(4)?,R::R0,R::R1)?)?;
    // VMA 0xa9fa0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // VMA 0xa9fa4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::LR,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9fa8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9fac
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // VMA 0xa9fb0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9fb4
    c.b(C::Lt,0xaa038,false)?;
    // VMA 0xa9fb8
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa9fbc
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // VMA 0xa9fc0
    c.literal(R::new(7)?,0xaa220)?;
    // VMA 0xa9fc4
    c.literal(R::R1,0xaa224)?;
    // VMA 0xa9fc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,61)?)?;
    // VMA 0xa9fcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9fd0
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,12)?)?;
    // VMA 0xa9fd4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xa9fd8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xa9fdc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9fe0
    c.literal(R::new(10)?,0xaa208)?;
    // VMA 0xa9fe4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9fe8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xa9fec
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9ff0
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(9)?,0)?)?;
    // VMA 0xa9ff4
    c.literal(R::R1,0xaa228)?;
    // VMA 0xa9ff8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,62)?)?;
    // VMA 0xa9ffc
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // VMA 0xaa000
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xaa004
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xaa008
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xaa00c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaa010
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaa014
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaa018
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xaa01c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaa020
    c.literal(R::R2,0xaa218)?;
    // VMA 0xaa024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // VMA 0xaa028
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaa02c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xaa030
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xaa034
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,44)?)?;
    // VMA 0xaa038
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,1196)?)?;
    // VMA 0xaa03c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,1,false)?)?;
    // VMA 0xaa040
    c.emit(arm32::store_halfword(C::Al,R::new(8)?,R::R0,0)?)?;
    // VMA 0xaa044
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xaa048
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,1192)?)?;
    // VMA 0xaa04c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xaa050
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa054
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // VMA 0xaa058
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaa05c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,32)?)?;
    // VMA 0xaa060
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,36)?)?;
    // VMA 0xaa064
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xaa068
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(10)?,0)?)?;
    // VMA 0xaa06c
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(7)?,0)?)?;
    // VMA 0xaa070
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(9)?,22)?)?;
    // VMA 0xaa074
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(3)?,R::R2)?)?;
    // VMA 0xaa078
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa07c
    c.b(C::Lt,0xa9e40,false)?;
    // VMA 0xaa080
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,24)?)?;
    // VMA 0xaa084
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,24)?)?;
    // VMA 0xaa088
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xaa08c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,254)?)?;
    // VMA 0xaa090
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,768)?)?;
    // VMA 0xaa094
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32512)?)?;
    // VMA 0xaa098
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xaa09c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa0a0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xaa0a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,256)?)?;
    // VMA 0xaa0a8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,21)?)?;
    // VMA 0xaa0ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa0b0
    c.b(C::Eq,0xaa154,false)?;
    // VMA 0xaa0b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xaa0b8
    c.b(C::Eq,0xaa11c,false)?;
    // VMA 0xaa0bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaa0c0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xaa0c4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(5)?,167)?)?;
    // VMA 0xaa0c8
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa0cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,2)?)?;
    // VMA 0xaa0d0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // VMA 0xaa0d4
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,22)?)?;
    // VMA 0xaa0d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,768)?)?;
    // VMA 0xaa0dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xaa0e0
    c.b(C::Cc,0xaa110,false)?;
    // VMA 0xaa0e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,36)?)?;
    // VMA 0xaa0e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xaa0ec
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaa0f0
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::R2)?)?;
    // VMA 0xaa0f4
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xaa0f8
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(8)?)?)?;
    // VMA 0xaa0fc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xaa100
    c.emit(arm32::store_halfword_postindexed(C::Al,R::R1,R::new(4)?,2)?)?;
    // VMA 0xaa104
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,22)?)?;
    // VMA 0xaa108
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa10c
    c.b(C::Lt,0xaa0ec,false)?;
    // VMA 0xaa110
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,24)?)?;
    // VMA 0xaa114
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xaa118
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(7)?))?;
    // VMA 0xaa11c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa120
    c.b(C::Eq,0xa9cd8,false)?;
    // VMA 0xaa124
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(5)?,167)?)?;
    // VMA 0xaa128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaa12c
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(10)?,0)?)?;
    // VMA 0xaa130
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::R2)?)?;
    // VMA 0xaa134
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xaa138
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(5)?)?)?;
    // VMA 0xaa13c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xaa140
    c.emit(arm32::store_halfword_postindexed(C::Al,R::R1,R::new(4)?,2)?)?;
    // VMA 0xaa144
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,24)?)?;
    // VMA 0xaa148
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa14c
    c.b(C::Lt,0xaa12c,false)?;
    // VMA 0xaa150
    c.b(C::Al,0xa9cd8,false)?;
    // VMA 0xaa154
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xaa158
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,2)?)?;
    // VMA 0xaa15c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // VMA 0xaa160
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(4)?,0)?)?;
    // VMA 0xaa164
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,768)?)?;
    // VMA 0xaa168
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xaa16c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // VMA 0xaa170
    c.b(C::Cc,0xaa1a8,false)?;
    // VMA 0xaa174
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,167)?)?;
    // VMA 0xaa178
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xaa17c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaa180
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::R0,-2)?)?;
    // VMA 0xaa184
    c.emit(arm32::load_halfword(C::Al,R::new(4)?,R::new(9)?,22)?)?;
    // VMA 0xaa188
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xaa18c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // VMA 0xaa190
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa194
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::new(6)?,R::new(3)?)?)?;
    // VMA 0xaa198
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(5)?,R::R1,2)?)?;
    // VMA 0xaa19c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::new(4)?,1)?)?;
    // VMA 0xaa1a0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa1a4
    c.b(C::Lt,0xaa180,false)?;
    // VMA 0xaa1a8
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(9)?,24)?)?;
    // VMA 0xaa1ac
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R1,0)?)?;
    // VMA 0xaa1b0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xaa1b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // VMA 0xaa1b8
    c.b(C::Cc,0xa9cd8,false)?;
    // VMA 0xaa1bc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,167)?)?;
    // VMA 0xaa1c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // VMA 0xaa1c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xaa1c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xaa1cc
    c.emit(arm32::load_halfword_postindexed(C::Al,R::new(6)?,R::R0,2)?)?;
    // VMA 0xaa1d0
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::new(9)?,24)?)?;
    // VMA 0xaa1d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xaa1d8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa1dc
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::new(7)?,R::R2)?)?;
    // VMA 0xaa1e0
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(6)?,R::R1,2)?)?;
    // VMA 0xaa1e4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(5)?,1)?)?;
    // VMA 0xaa1e8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa1ec
    c.b(C::Lt,0xaa1cc,false)?;
    // VMA 0xaa1f0
    c.b(C::Al,0xa9cd8,false)?;
    c.check(0xaa1f4)
}

const CODEC_BLEND_LITERALS:[u32;14]=[
    0x3fff_3fff, // packed signed 14-bit mask
    0x0030_6090, // codec controller callback address
    0x0010_dd64, // codec data region
    0x000f_84c5, // diagnostic string pointers
    0x000f_850a,
    0x000e_2788,
    0x0010_dd78, // controller configuration pointer
    0x7777_7777, // VLQ pattern (data, not an instruction)
    0x0077_7777,
    0x000f_8334,
    0x000f_836d,
    0x000f_839b,
    0x000f_83ab,
    0x000f_83d4,
];
