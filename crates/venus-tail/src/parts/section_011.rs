
/// Initialize the full codec controller's command FIFO, writes per-frame
/// register words across two DMA instances, and dispatches encoded packets.
/// The function has a natural POP {...,PC} return at 0xab764, followed by a
/// seven-word diagnostic/global-address pool, not executable instructions.
fn configure_codec_register_fifo(c:&mut Code)->Result<(),String>{
    c.check(0xab04c)?;
    // VMA 0xab04c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xab050
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xab054
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,60)?)?;
    // VMA 0xab058
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xab05c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,1160)?)?;
    // VMA 0xab060
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // VMA 0xab064
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(3)?))?;
    // VMA 0xab068
    c.literal(R::R1,0xab768)?;
    // VMA 0xab06c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xab070
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // VMA 0xab074
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,4)?)?;
    // VMA 0xab078
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // VMA 0xab07c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // VMA 0xab080
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xab084
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab088
    c.literal(R::R1,0xab76c)?;
    // VMA 0xab08c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xab090
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab094
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,860)?)?;
    // VMA 0xab098
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xab09c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xab0a0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xab0a4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R12,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab0a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,81)?)?;
    // VMA 0xab0ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xab0b0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // VMA 0xab0b4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab0b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // VMA 0xab0bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,856)?)?;
    // VMA 0xab0c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xab0c4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab0c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // VMA 0xab0cc
    c.emit(arm32::mov_register(C::Al,R::R2,R::LR))?;
    // VMA 0xab0d0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab0d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,984)?)?;
    // VMA 0xab0d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xab0dc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-40)?)?;
    // VMA 0xab0e0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(10)?,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab0e4
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,44)?)?;
    // VMA 0xab0e8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab0ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,980)?)?;
    // VMA 0xab0f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xab0f4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab0f8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // VMA 0xab0fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // VMA 0xab100
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab104
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab108
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // VMA 0xab10c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,864)?)?;
    // VMA 0xab110
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // VMA 0xab114
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,4)?)?;
    // VMA 0xab118
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,292)?)?;
    // VMA 0xab11c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,4)?)?;
    // VMA 0xab120
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,144)?)?;
    // VMA 0xab124
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xab128
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab12c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // VMA 0xab130
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // VMA 0xab134
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::LR))?;
    // VMA 0xab138
    c.b(C::Al,0x31dc8,true)?;
    // VMA 0xab13c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,244)?)?;
    // VMA 0xab140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3072)?)?;
    // VMA 0xab144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xab148
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab14c
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xab154
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,248)?)?;
    // VMA 0xab158
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab15c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab160
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab164
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3696)?)?;
    // VMA 0xab168
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,260)?)?;
    // VMA 0xab16c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab170
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab174
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,628)?)?;
    // VMA 0xab178
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,264)?)?;
    // VMA 0xab17c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab180
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab184
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,632)?)?;
    // VMA 0xab18c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,268)?)?;
    // VMA 0xab190
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab194
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab198
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab19c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,636)?)?;
    // VMA 0xab1a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,272)?)?;
    // VMA 0xab1a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab1a8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab1ac
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab1b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xab1b4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,252)?)?;
    // VMA 0xab1b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab1bc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab1c0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab1c4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xab1c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // VMA 0xab1cc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // VMA 0xab1d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xab1d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,1036)?)?;
    // VMA 0xab1d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xab1dc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // VMA 0xab1e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // VMA 0xab1e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,36)?)?;
    // VMA 0xab1e8
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab1ec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xab1f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xab1f4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xab1f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xab1fc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,1036)?)?;
    // VMA 0xab200
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xab204
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xab208
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // VMA 0xab20c
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(7)?,Shift::Lsl,15)?)?;
    // VMA 0xab210
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,296)?)?;
    // VMA 0xab214
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab218
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xab21c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1040)?)?;
    // VMA 0xab220
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // VMA 0xab224
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xab228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33)?)?;
    // VMA 0xab22c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // VMA 0xab230
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // VMA 0xab234
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(5)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab238
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xab23c
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R0,4)?)?;
    // VMA 0xab240
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xab244
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(8)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab248
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(7)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab24c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,300)?)?;
    // VMA 0xab250
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,255)?)?;
    // VMA 0xab254
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(6)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab258
    c.b(C::Ne,0xab234,false)?;
    // VMA 0xab25c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,136)?)?;
    // VMA 0xab260
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab264
    c.b(C::Eq,0xab278,false)?;
    // VMA 0xab268
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,140)?)?;
    // VMA 0xab26c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab270
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xab274
    c.b(C::Al,0xab27c,false)?;
    // VMA 0xab278
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab27c
    c.literal(R::R2,0xab770)?;
    // VMA 0xab280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,69)?)?;
    // VMA 0xab284
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xab288
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xab28c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xab290
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,81)?)?;
    // VMA 0xab294
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xab298
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    // VMA 0xab29c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab2a0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,140)?)?;
    // VMA 0xab2a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,28)?)?;
    // VMA 0xab2a8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(8)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab2ac
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xab2b0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // VMA 0xab2b4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab2b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,304)?)?;
    // VMA 0xab2bc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab2c0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab2c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,49)?)?;
    // VMA 0xab2c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xab2cc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xab2d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,136)?)?;
    // VMA 0xab2d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,0)?)?;
    // VMA 0xab2d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xab2dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,304)?)?;
    // VMA 0xab2e0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab2e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xab2e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65)?)?;
    // VMA 0xab2ec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xab2f0
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xab2f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1048)?)?;
    // VMA 0xab2f8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // VMA 0xab2fc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xab300
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1052)?)?;
    // VMA 0xab304
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab308
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xab30c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,256)?)?;
    // VMA 0xab310
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab314
    c.b(C::Eq,0xab4e8,false)?;
    // VMA 0xab318
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,324)?)?;
    // VMA 0xab31c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xab320
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // VMA 0xab324
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,15,false)?)?;
    // VMA 0xab328
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(3)?,3)?)?;
    // VMA 0xab32c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // VMA 0xab330
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // VMA 0xab334
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,64)?)?;
    // VMA 0xab338
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xab33c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // VMA 0xab340
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // VMA 0xab344
    c.b(C::Cc,0xab380,false)?;
    // VMA 0xab348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,768)?)?;
    // VMA 0xab34c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(10)?,2)?)?;
    // VMA 0xab350
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xab354
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(6)?,R::new(10)?,R::new(6)?)?)?;
    // VMA 0xab358
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,768)?)?;
    // VMA 0xab35c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab360
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(7)?,0)?)?;
    // VMA 0xab364
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab368
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab36c
    c.b(C::Cc,0xab488,false)?;
    // VMA 0xab370
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xab374
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // VMA 0xab378
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xab37c
    c.b(C::Cc,0xab35c,false)?;
    // VMA 0xab380
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xab384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xab388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xab38c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3104)?)?;
    // VMA 0xab390
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xab394
    c.literal(R::R2,0xab770)?;
    // VMA 0xab398
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,256)?)?;
    // VMA 0xab39c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R1,R::R0,Shift::Lsr,15,false)?)?;
    // VMA 0xab3a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,101)?)?;
    // VMA 0xab3a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab3a8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,160)?)?;
    // VMA 0xab3ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xab3b0
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xab3b4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xab3b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,160)?)?;
    // VMA 0xab3bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,160)?)?;
    // VMA 0xab3c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,160)?)?;
    // VMA 0xab3c4
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xab3c8
    c.emit(arm32::mov_register(C::Cc,R::new(5)?,R::new(4)?))?;
    // VMA 0xab3cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xab3d0
    c.b(C::Eq,0xab3f8,false)?;
    // VMA 0xab3d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xab3d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,768)?)?;
    // VMA 0xab3dc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,36)?)?;
    // VMA 0xab3e0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab3e4
    c.emit(arm32::load_halfword_postindexed(C::Al,R::R2,R::R1,2)?)?;
    // VMA 0xab3e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xab3ec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab3f0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xab3f4
    c.b(C::Cc,0xab3e4,false)?;
    // VMA 0xab3f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,256)?)?;
    // VMA 0xab3fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xab400
    c.literal(R::R2,0xab770)?;
    // VMA 0xab404
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16128)?)?;
    // VMA 0xab408
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // VMA 0xab40c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,111)?)?;
    // VMA 0xab410
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab414
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab418
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xab41c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,160)?)?;
    // VMA 0xab420
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xab424
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xab428
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xab42c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,160)?)?;
    // VMA 0xab430
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xab434
    c.emit(arm32::mov_register(C::Cc,R::new(8)?,R::new(6)?))?;
    // VMA 0xab438
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab43c
    c.b(C::Cs,0xab4e8,false)?;
    // VMA 0xab440
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // VMA 0xab444
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xab448
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // VMA 0xab44c
    c.emit(arm32::data_register(C::Al,Op::Mvn,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab450
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,40)?)?;
    // VMA 0xab454
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,161)?)?;
    // VMA 0xab458
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab45c
    c.emit(arm32::data_immediate(C::Ls,Op::Mvn,R::R1,R::R0,160)?)?;
    // VMA 0xab460
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R2,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xab464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xab468
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(10)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab46c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,756)?)?;
    // VMA 0xab470
    c.emit(arm32::load_halfword_postindexed(C::Al,R::new(3)?,R::R1,2)?)?;
    // VMA 0xab474
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xab478
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab47c
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xab480
    c.b(C::Cc,0xab470,false)?;
    // VMA 0xab484
    c.b(C::Al,0xab4e8,false)?;
    // VMA 0xab488
    c.literal(R::R1,0xab774)?;
    // VMA 0xab48c
    c.literal(R::R2,0xab778)?;
    // VMA 0xab490
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,90)?)?;
    // VMA 0xab494
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xab498
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xab49c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab4a0
    c.literal(R::R1,0xab76c)?;
    // VMA 0xab4a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xab4a8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab4ac
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // VMA 0xab4b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xab4b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xab4b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xab4bc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,32768)?)?;
    // VMA 0xab4c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,3104)?)?;
    // VMA 0xab4c4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xab4c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,36)?)?;
    // VMA 0xab4cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xab4d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab4d4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xab4d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,40)?)?;
    // VMA 0xab4dc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab4e0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xab4e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xab4e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // VMA 0xab4ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,96)?)?;
    // VMA 0xab4f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,3504)?)?;
    // VMA 0xab4f4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab4f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,97)?)?;
    // VMA 0xab4fc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xab500
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xab504
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab508
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,32)?)?;
    // VMA 0xab50c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xab510
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab514
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // VMA 0xab518
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab51c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,36)?)?;
    // VMA 0xab520
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xab524
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab528
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab52c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,668)?)?;
    // VMA 0xab530
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,276)?)?;
    // VMA 0xab534
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab538
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab53c
    c.literal(R::R0,0xab77c)?;
    // VMA 0xab540
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,8)?)?;
    // VMA 0xab544
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xab548
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,2880)?)?;
    // VMA 0xab54c
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xab550
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,3440)?)?;
    // VMA 0xab554
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xab558
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xab55c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3758096384)?)?;
    // VMA 0xab560
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xab564
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,304)?)?;
    // VMA 0xab568
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(7)?,R::new(3)?,4)?)?;
    // VMA 0xab56c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xab570
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // VMA 0xab574
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(7)?,R::R1,4)?)?;
    // VMA 0xab578
    c.b(C::Ne,0xab568,false)?;
    // VMA 0xab57c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,168)?)?;
    // VMA 0xab580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xab584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab588
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,36)?)?;
    // VMA 0xab58c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xab590
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xab594
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // VMA 0xab598
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xab59c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // VMA 0xab5a0
    c.b(C::Ne,0xab58c,false)?;
    // VMA 0xab5a4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xab5a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xab5ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,97)?)?;
    // VMA 0xab5b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xab5b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,664)?)?;
    // VMA 0xab5b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // VMA 0xab5bc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab5c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,120)?)?;
    // VMA 0xab5c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,1024)?)?;
    // VMA 0xab5c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,360)?)?;
    // VMA 0xab5cc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R0,14336)?)?;
    // VMA 0xab5d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,364)?)?;
    // VMA 0xab5d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // VMA 0xab5d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::R0,14336)?)?;
    // VMA 0xab5dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xab5e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // VMA 0xab5e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xab5e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1152)?)?;
    // VMA 0xab5ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // VMA 0xab5f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xab5f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab5f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xab5fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // VMA 0xab600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xab604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xab608
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xab60c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-36)?)?;
    // VMA 0xab610
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab614
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xab618
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(10)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab61c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,160)?)?;
    // VMA 0xab620
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xab624
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // VMA 0xab628
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xab62c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-32)?)?;
    // VMA 0xab630
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xab634
    c.emit(arm32::data_register(C::Eq,Op::Add,R::new(4)?,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab638
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(3)?,R::new(10)?,152)?)?;
    // VMA 0xab63c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // VMA 0xab640
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,0)?)?;
    // VMA 0xab644
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xab648
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab64c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,16)?)?;
    // VMA 0xab650
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,0)?)?;
    // VMA 0xab654
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::LR,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab658
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab65c
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R12,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab660
    c.b(C::Ne,0xab620,false)?;
    // VMA 0xab664
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // VMA 0xab668
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,32)?)?;
    // VMA 0xab66c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-36)?)?;
    // VMA 0xab670
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xab674
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::R12,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab678
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,-8)?)?;
    // VMA 0xab67c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // VMA 0xab680
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    // VMA 0xab684
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab688
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // VMA 0xab68c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::R2,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab690
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::LR))?;
    // VMA 0xab694
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // VMA 0xab698
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::R1,255)?)?;
    // VMA 0xab69c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xab6a0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::LR,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xab6a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,16)?)?;
    // VMA 0xab6a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // VMA 0xab6ac
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(8)?,0)?)?;
    // VMA 0xab6b0
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R12,0)?)?;
    // VMA 0xab6b4
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xab6b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xab6bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,16)?)?;
    // VMA 0xab6c0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab6c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xab6c8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xab6cc
    c.b(C::Al,0xd9e60,true)?;
    // VMA 0xab6d0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xab6d4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // VMA 0xab6d8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xab6dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // VMA 0xab6e0
    c.b(C::Al,0xd9e60,true)?;
    // VMA 0xab6e4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // VMA 0xab6e8
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // VMA 0xab6ec
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xab6f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,-8)?)?;
    // VMA 0xab6f4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // VMA 0xab6f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,160)?)?;
    // VMA 0xab6fc
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,152)?)?;
    // VMA 0xab700
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xab704
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xab708
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xab70c
    c.literal(R::R1,0xab780)?;
    // VMA 0xab710
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xab714
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xab718
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xab71c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab720
    c.literal(R::R1,0xab76c)?;
    // VMA 0xab724
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xab728
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab72c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xab730
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(6)?))?;
    // VMA 0xab734
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,12)?)?;
    // VMA 0xab738
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xab73c
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(5)?))?;
    // VMA 0xab740
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xab744
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,4)?)?;
    // VMA 0xab748
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,32)?)?;
    // VMA 0xab74c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,2)?)?;
    // VMA 0xab750
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,65536)?)?;
    // VMA 0xab754
    c.b(C::Ne,0xab5f8,false)?;
    // VMA 0xab758
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xab75c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,292)?)?;
    // VMA 0xab760
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xab764
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xab768)
}
