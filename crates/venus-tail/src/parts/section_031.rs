
/// Reconcile active and queued codec-plane descriptors: bounded per-plane
/// traversals, hardware queue wakeups and error reporting. The routine has
/// several paths back to its entry before five diagnostic literal words.
fn reconcile_codec_frame_queue(c:&mut Code)->Result<(),String>{
    c.check(0xb2f08)?;
    // VMA 0xb2f08
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb2f0c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb2f10
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,76)?)?;
    // VMA 0xb2f14
    c.emit(arm32::mov_register(C::Al,R::LR,R::R0))?;
    // VMA 0xb2f18
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,8)?)?;
    // VMA 0xb2f1c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb2f20
    c.b(C::Eq,0xb2f5c,false)?;
    // VMA 0xb2f24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2f28
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::LR,272)?)?;
    // VMA 0xb2f2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::LR,268)?)?;
    // VMA 0xb2f30
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::LR,176)?)?;
    // VMA 0xb2f34
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,11)?)?;
    // VMA 0xb2f38
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xb2f3c
    c.b(C::Eq,0xb2f54,false)?;
    // VMA 0xb2f40
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,176)?)?;
    // VMA 0xb2f44
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::LR,676)?)?;
    // VMA 0xb2f48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb2f4c
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R1,R::R1,2)?)?;
    // VMA 0xb2f50
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::LR,676)?)?;
    // VMA 0xb2f54
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb2f58
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb2f5c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,248)?)?;
    // VMA 0xb2f60
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xb2f64
    c.b(C::Lt,0xb301c,false)?;
    // VMA 0xb2f68
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,52)?)?;
    // VMA 0xb2f6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb2f70
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2f74
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,115)?)?;
    // VMA 0xb2f78
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R2,R::new(7)?,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // VMA 0xb2f7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2f80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb2f84
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // VMA 0xb2f88
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,20)?)?;
    // VMA 0xb2f8c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb2f90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2f94
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R2,184)?)?;
    // VMA 0xb2f98
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // VMA 0xb2f9c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,40)?)?;
    // VMA 0xb2fa0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // VMA 0xb2fa4
    c.emit(arm32::mov_register(C::Al,R::R1,R::LR))?;
    // VMA 0xb2fa8
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R2,R::R1,180)?)?;
    // VMA 0xb2fac
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(5)?,R::R1,R::new(6)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb2fb0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb2fb4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb2fb8
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::new(5)?,R::new(7)?,R::R2)?)?;
    // VMA 0xb2fbc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,4)?)?;
    // VMA 0xb2fc0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,3)?)?;
    // VMA 0xb2fc4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,4)?)?;
    // VMA 0xb2fc8
    c.b(C::Lt,0xb2fac,false)?;
    // VMA 0xb2fcc
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,48)?)?;
    // VMA 0xb2fd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2fd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2fd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2fdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2fe0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb2fe4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R12,12)?)?;
    // VMA 0xb2fe8
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,52)?)?;
    // VMA 0xb2fec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb2ff0
    c.b(C::Eq,0xb336c,false)?;
    // VMA 0xb2ff4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // VMA 0xb2ff8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::LR,180)?)?;
    // VMA 0xb2ffc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb3000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3008
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb300c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3010
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,84)?)?;
    // VMA 0xb3014
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // VMA 0xb3018
    c.b(C::Al,0xb3044,false)?;
    // VMA 0xb301c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb3020
    c.b(C::Ne,0xb2f34,false)?;
    // VMA 0xb3024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3028
    c.b(C::Al,0xb2f30,false)?;
    // VMA 0xb302c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb3030
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R12,0)?)?;
    // VMA 0xb3034
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb3038
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb303c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb3040
    c.b(C::Al,0xb31c8,false)?;
    // VMA 0xb3044
    c.emit(arm32::transfer_byte_register_offset(C::Al,true,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb3048
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb304c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-44)?)?;
    // VMA 0xb3050
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,44)?)?;
    // VMA 0xb3054
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb3058
    c.b(C::Eq,0xb3140,false)?;
    // VMA 0xb305c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,16)?)?;
    // VMA 0xb3060
    c.b(C::Ge,0xb3308,false)?;
    // VMA 0xb3064
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-48)?)?;
    // VMA 0xb3068
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb306c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xb3070
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::LR,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3074
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::LR,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3078
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,-64)?)?;
    // VMA 0xb307c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb3084
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R1,R::R12,276)?)?;
    // VMA 0xb3088
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,36)?)?;
    // VMA 0xb308c
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::new(8)?,596)?)?;
    // VMA 0xb3090
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,248)?)?;
    // VMA 0xb3094
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3098
    c.b(C::Lt,0xb30e4,false)?;
    // VMA 0xb309c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::LR,0)?)?;
    // VMA 0xb30a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb30a4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(7)?,R::new(10)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb30a8
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(7)?,R::new(9)?,R::R2)?)?;
    // VMA 0xb30ac
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,3)?)?;
    // VMA 0xb30b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb30b4
    c.b(C::Eq,0xb30d8,false)?;
    // VMA 0xb30b8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(6)?))?;
    // VMA 0xb30bc
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::new(5)?,R::new(4)?,5)?)?;
    // VMA 0xb30c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb30c4
    c.b(C::Ne,0xb30d8,false)?;
    // VMA 0xb30c8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,12)?)?;
    // VMA 0xb30cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,0)?)?;
    // VMA 0xb30d0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb30d4
    c.b(C::Eq,0xb3270,false)?;
    // VMA 0xb30d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb30dc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb30e0
    c.b(C::Lt,0xb30a4,false)?;
    // VMA 0xb30e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // VMA 0xb30e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // VMA 0xb30ec
    c.b(C::Ge,0xb331c,false)?;
    // VMA 0xb30f0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // VMA 0xb30f4
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // VMA 0xb30f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb30fc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,255)?)?;
    // VMA 0xb3100
    c.b(C::Ne,0xb32d0,false)?;
    // VMA 0xb3104
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R12,0)?)?;
    // VMA 0xb3108
    c.literal(R::R1,0xb3bc8)?;
    // VMA 0xb310c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb3110
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xb3114
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb3118
    c.literal(R::R1,0xb3bcc)?;
    // VMA 0xb311c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb3120
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb3124
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // VMA 0xb3128
    c.b(C::Lt,0xb31e0,false)?;
    // VMA 0xb312c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // VMA 0xb3130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,83&0xffffffff)?)?;
    // VMA 0xb3134
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xb3138
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::R1,R::R0)?)?;
    // VMA 0xb313c
    c.b(C::Al,0xb31e4,false)?;
    // VMA 0xb3140
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // VMA 0xb3144
    c.b(C::Ge,0xb3348,false)?;
    // VMA 0xb3148
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb314c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xb3150
    c.emit(arm32::mov_register(C::Al,R::R0,R::LR))?;
    // VMA 0xb3154
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,-64)?)?;
    // VMA 0xb3158
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::R0,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb315c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3160
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb3164
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R1,R::LR,404)?)?;
    // VMA 0xb3168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255&0xffffffff)?)?;
    // VMA 0xb316c
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R1,R::R12,628)?)?;
    // VMA 0xb3170
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,248)?)?;
    // VMA 0xb3174
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb3178
    c.b(C::Lt,0xb31c8,false)?;
    // VMA 0xb317c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // VMA 0xb3180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3184
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xb3188
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(7)?,R::new(10)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb318c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(7)?,R::new(9)?,R::new(3)?)?)?;
    // VMA 0xb3190
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,3)?)?;
    // VMA 0xb3194
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb3198
    c.b(C::Eq,0xb31bc,false)?;
    // VMA 0xb319c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(6)?))?;
    // VMA 0xb31a0
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::R1,R::new(4)?,5)?)?;
    // VMA 0xb31a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb31a8
    c.b(C::Ne,0xb31bc,false)?;
    // VMA 0xb31ac
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // VMA 0xb31b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // VMA 0xb31b4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb31b8
    c.b(C::Eq,0xb302c,false)?;
    // VMA 0xb31bc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xb31c0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb31c4
    c.b(C::Lt,0xb3188,false)?;
    // VMA 0xb31c8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb31cc
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb31d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // VMA 0xb31d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb31d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb31dc
    c.b(C::Al,0xb32e8,false)?;
    // VMA 0xb31e0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb31e4
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb31e8
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xb31ec
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,676)?)?;
    // VMA 0xb31f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xb31f4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::LR,676)?)?;
    // VMA 0xb31f8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,248)?)?;
    // VMA 0xb31fc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3200
    c.b(C::Lt,0xb3234,false)?;
    // VMA 0xb3204
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::LR,0)?)?;
    // VMA 0xb3208
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb320c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(10)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb3210
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::new(3)?,R::new(9)?,R::R2)?)?;
    // VMA 0xb3214
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,12)?)?;
    // VMA 0xb3218
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-32)?)?;
    // VMA 0xb321c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3220
    c.b(C::Le,0xb3238,false)?;
    // VMA 0xb3224
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xb3228
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb322c
    c.b(C::Lt,0xb320c,false)?;
    // VMA 0xb3230
    c.b(C::Al,0xb3238,false)?;
    // VMA 0xb3234
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3238
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb323c
    c.b(C::Ge,0xb3290,false)?;
    // VMA 0xb3240
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::LR,0)?)?;
    // VMA 0xb3244
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xb3248
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::LR,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb324c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,180)?)?;
    // VMA 0xb3250
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::R2,R::new(9)?,R::new(7)?)?)?;
    // VMA 0xb3254
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,4)?)?;
    // VMA 0xb3258
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,3)?)?;
    // VMA 0xb325c
    c.b(C::Eq,0xb3288,false)?;
    // VMA 0xb3260
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb3264
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3268
    c.b(C::Lt,0xb3248,false)?;
    // VMA 0xb326c
    c.b(C::Al,0xb3290,false)?;
    // VMA 0xb3270
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb3274
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(8)?,0)?)?;
    // VMA 0xb3278
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb327c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3280
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb3284
    c.b(C::Al,0xb30e4,false)?;
    // VMA 0xb3288
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // VMA 0xb328c
    c.b(C::Ne,0xb32cc,false)?;
    // VMA 0xb3290
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb3294
    c.b(C::Lt,0xb32c8,false)?;
    // VMA 0xb3298
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::LR,0)?)?;
    // VMA 0xb329c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(10)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb32a0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xb32a4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xb32a8
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::R2,R::new(9)?,R::new(3)?)?)?;
    // VMA 0xb32ac
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,4)?)?;
    // VMA 0xb32b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xb32b4
    c.b(C::Eq,0xb32cc,false)?;
    // VMA 0xb32b8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xb32bc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,4)?)?;
    // VMA 0xb32c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb32c4
    c.b(C::Gt,0xb32a4,false)?;
    // VMA 0xb32c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255&0xffffffff)?)?;
    // VMA 0xb32cc
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // VMA 0xb32d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-48)?)?;
    // VMA 0xb32d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb32d8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb32dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xb32e0
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,160)?)?;
    // VMA 0xb32e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb32e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // VMA 0xb32ec
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // VMA 0xb32f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xb32f4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,12)?)?;
    // VMA 0xb32f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb32fc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3300
    c.b(C::Lt,0xb3044,false)?;
    // VMA 0xb3304
    c.b(C::Al,0xb336c,false)?;
    // VMA 0xb3308
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // VMA 0xb330c
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb3310
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3314
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,53&0xffffffff)?)?;
    // VMA 0xb3318
    c.b(C::Al,0xb3358,false)?;
    // VMA 0xb331c
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb3320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,78&0xffffffff)?)?;
    // VMA 0xb3328
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb332c
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3330
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-48)?)?;
    // VMA 0xb3334
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb3338
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // VMA 0xb333c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // VMA 0xb3340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3344
    c.b(C::Al,0xb336c,false)?;
    // VMA 0xb3348
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // VMA 0xb334c
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb3350
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3354
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,103&0xffffffff)?)?;
    // VMA 0xb3358
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // VMA 0xb335c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb3360
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3364
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xb3368
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xb336c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-48)?)?;
    // VMA 0xb3370
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::LR,268)?)?;
    // VMA 0xb3374
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,48)?)?;
    // VMA 0xb3378
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // VMA 0xb337c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,13)?)?;
    // VMA 0xb3380
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3384
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3388
    c.b(C::Ge,0xb36c0,false)?;
    // VMA 0xb338c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::LR,180)?)?;
    // VMA 0xb3390
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb3394
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3398
    c.b(C::Al,0xb33cc,false)?;
    // VMA 0xb339c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb33a0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(9)?,0)?)?;
    // VMA 0xb33a4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb33a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb33ac
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb33b0
    c.b(C::Al,0xb346c,false)?;
    // VMA 0xb33b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb33b8
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::R12,0)?)?;
    // VMA 0xb33bc
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb33c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb33c4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb33c8
    c.b(C::Al,0xb354c,false)?;
    // VMA 0xb33cc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb33d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,84)?)?;
    // VMA 0xb33d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,44)?)?;
    // VMA 0xb33d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb33dc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-44)?)?;
    // VMA 0xb33e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb33e4
    c.b(C::Eq,0xb34c4,false)?;
    // VMA 0xb33e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,16)?)?;
    // VMA 0xb33ec
    c.b(C::Ge,0xb3668,false)?;
    // VMA 0xb33f0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb33f4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xb33f8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::LR,R::new(4)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb33fc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::LR,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3400
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,20)?)?;
    // VMA 0xb3404
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3408
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb340c
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R1,R::R12,340)?)?;
    // VMA 0xb3410
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,40)?)?;
    // VMA 0xb3414
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::new(9)?,612)?)?;
    // VMA 0xb3418
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,248)?)?;
    // VMA 0xb341c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3420
    c.b(C::Lt,0xb346c,false)?;
    // VMA 0xb3424
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::LR,0)?)?;
    // VMA 0xb3428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb342c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(4)?,R::new(8)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3430
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(4)?,R::new(10)?,R::R2)?)?;
    // VMA 0xb3434
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,3)?)?;
    // VMA 0xb3438
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb343c
    c.b(C::Eq,0xb3460,false)?;
    // VMA 0xb3440
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // VMA 0xb3444
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::R0,R::new(7)?,5)?)?;
    // VMA 0xb3448
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb344c
    c.b(C::Ne,0xb3460,false)?;
    // VMA 0xb3450
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xb3454
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R12,0)?)?;
    // VMA 0xb3458
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb345c
    c.b(C::Eq,0xb339c,false)?;
    // VMA 0xb3460
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb3464
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3468
    c.b(C::Lt,0xb342c,false)?;
    // VMA 0xb346c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // VMA 0xb3470
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // VMA 0xb3474
    c.b(C::Ge,0xb3684,false)?;
    // VMA 0xb3478
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,0)?)?;
    // VMA 0xb347c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xb3480
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,255)?)?;
    // VMA 0xb3484
    c.b(C::Ne,0xb3628,false)?;
    // VMA 0xb3488
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R12,0)?)?;
    // VMA 0xb348c
    c.literal(R::R1,0xb3bd4)?;
    // VMA 0xb3490
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb3494
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xb3498
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb349c
    c.literal(R::R1,0xb3bcc)?;
    // VMA 0xb34a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb34a4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb34a8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,1)?)?;
    // VMA 0xb34ac
    c.b(C::Lt,0xb3560,false)?;
    // VMA 0xb34b0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xb34b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,99&0xffffffff)?)?;
    // VMA 0xb34b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xb34bc
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::R1,R::R0)?)?;
    // VMA 0xb34c0
    c.b(C::Al,0xb3564,false)?;
    // VMA 0xb34c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // VMA 0xb34c8
    c.b(C::Ge,0xb36a4,false)?;
    // VMA 0xb34cc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb34d0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xb34d4
    c.emit(arm32::mov_register(C::Al,R::R0,R::LR))?;
    // VMA 0xb34d8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,20)?)?;
    // VMA 0xb34dc
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::R0,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb34e0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb34e4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb34e8
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R1,R::LR,404)?)?;
    // VMA 0xb34ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255&0xffffffff)?)?;
    // VMA 0xb34f0
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R1,R::R12,628)?)?;
    // VMA 0xb34f4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,248)?)?;
    // VMA 0xb34f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb34fc
    c.b(C::Lt,0xb354c,false)?;
    // VMA 0xb3500
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // VMA 0xb3504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3508
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xb350c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(4)?,R::new(8)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xb3510
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(4)?,R::new(10)?,R::new(3)?)?)?;
    // VMA 0xb3514
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,3)?)?;
    // VMA 0xb3518
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb351c
    c.b(C::Eq,0xb3540,false)?;
    // VMA 0xb3520
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // VMA 0xb3524
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::new(5)?,R::new(7)?,5)?)?;
    // VMA 0xb3528
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb352c
    c.b(C::Ne,0xb3540,false)?;
    // VMA 0xb3530
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,12)?)?;
    // VMA 0xb3534
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // VMA 0xb3538
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb353c
    c.b(C::Eq,0xb33b4,false)?;
    // VMA 0xb3540
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb3544
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb3548
    c.b(C::Lt,0xb350c,false)?;
    // VMA 0xb354c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb3550
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3554
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xb3558
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb355c
    c.b(C::Al,0xb3648,false)?;
    // VMA 0xb3560
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3564
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3568
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xb356c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3570
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,676)?)?;
    // VMA 0xb3574
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xb3578
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::LR,676)?)?;
    // VMA 0xb357c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,248)?)?;
    // VMA 0xb3580
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // VMA 0xb3584
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3588
    c.b(C::Lt,0xb35b8,false)?;
    // VMA 0xb358c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(8)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb3590
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::new(3)?,R::new(10)?,R::R0)?)?;
    // VMA 0xb3594
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,12)?)?;
    // VMA 0xb3598
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-32)?)?;
    // VMA 0xb359c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb35a0
    c.b(C::Le,0xb35b0,false)?;
    // VMA 0xb35a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb35a8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb35ac
    c.b(C::Lt,0xb358c,false)?;
    // VMA 0xb35b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb35b4
    c.b(C::Lt,0xb35f0,false)?;
    // VMA 0xb35b8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(8)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb35bc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,1)?)?;
    // VMA 0xb35c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,0)?)?;
    // VMA 0xb35c4
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::new(3)?,R::new(10)?,R::R0)?)?;
    // VMA 0xb35c8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,4)?)?;
    // VMA 0xb35cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xb35d0
    c.b(C::Eq,0xb3624,false)?;
    // VMA 0xb35d4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb35d8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(4)?,4)?)?;
    // VMA 0xb35dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb35e0
    c.b(C::Gt,0xb35c0,false)?;
    // VMA 0xb35e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xb35e8
    c.b(C::Al,0xb35f0,false)?;
    // VMA 0xb35ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb35f0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb35f4
    c.b(C::Ge,0xb361c,false)?;
    // VMA 0xb35f8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::LR,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb35fc
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,180)?)?;
    // VMA 0xb3600
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::new(3)?,R::new(10)?,R::R0)?)?;
    // VMA 0xb3604
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,4)?)?;
    // VMA 0xb3608
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xb360c
    c.b(C::Ne,0xb35ec,false)?;
    // VMA 0xb3610
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R2,1)?)?;
    // VMA 0xb3614
    c.emit(arm32::mov_register(C::Eq,R::new(3)?,R::R2))?;
    // VMA 0xb3618
    c.b(C::Al,0xb3628,false)?;
    // VMA 0xb361c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255&0xffffffff)?)?;
    // VMA 0xb3620
    c.b(C::Al,0xb3628,false)?;
    // VMA 0xb3624
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xb3628
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // VMA 0xb362c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xb3630
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3634
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb3638
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,160)?)?;
    // VMA 0xb363c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-48)?)?;
    // VMA 0xb3640
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,48)?)?;
    // VMA 0xb3644
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb3648
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // VMA 0xb364c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // VMA 0xb3650
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,13)?)?;
    // VMA 0xb3654
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb3658
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb365c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3660
    c.b(C::Lt,0xb33cc,false)?;
    // VMA 0xb3664
    c.b(C::Al,0xb36c0,false)?;
    // VMA 0xb3668
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb366c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3670
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,138&0xffffffff)?)?;
    // VMA 0xb3674
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb3678
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb367c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb3680
    c.b(C::Al,0xb36c0,false)?;
    // VMA 0xb3684
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb3688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb368c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,172&0xffffffff)?)?;
    // VMA 0xb3690
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb3694
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::SP,44)?)?;
    // VMA 0xb3698
    c.emit(arm32::transfer_multiple(C::Al,true,R::LR,R::new(4)?.bit()|R::new(9)?.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb369c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xb36a0
    c.b(C::Al,0xb36c0,false)?;
    // VMA 0xb36a4
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb36a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb36ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,197&0xffffffff)?)?;
    // VMA 0xb36b0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // VMA 0xb36b4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb36b8
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb36bc
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xb36c0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::LR,269)?)?;
    // VMA 0xb36c4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::LR,270)?)?;
    // VMA 0xb36c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb36cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,16)?)?;
    // VMA 0xb36d0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,14)?)?;
    // VMA 0xb36d4
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,12)?)?;
    // VMA 0xb36d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,13)?)?;
    // VMA 0xb36dc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,15)?)?;
    // VMA 0xb36e0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb36e4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb36e8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb36ec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb36f0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,44)?)?;
    // VMA 0xb36f4
    c.b(C::Ls,0xb3af0,false)?;
    // VMA 0xb36f8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // VMA 0xb36fc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::LR,660)?)?;
    // VMA 0xb3700
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::LR,532)?)?;
    // VMA 0xb3704
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3708
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb370c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3710
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // VMA 0xb3714
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // VMA 0xb3718
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb371c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xb3720
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xb3724
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::LR,180)?)?;
    // VMA 0xb3728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,83)?)?;
    // VMA 0xb372c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // VMA 0xb3730
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3734
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // VMA 0xb3738
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0&0xffffffff)?)?;
    // VMA 0xb373c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-44)?)?;
    // VMA 0xb3740
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xb3744
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::new(3)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3748
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xb374c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3750
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xb3754
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb3758
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsl,2)?)?;
    // VMA 0xb375c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,40)?)?;
    // VMA 0xb3760
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb3764
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xb3768
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,36)?)?;
    // VMA 0xb376c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xb3770
    c.emit(arm32::transfer_byte_register_offset(C::Al,true,R::R1,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb3774
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xb3778
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb377c
    c.b(C::Ne,0xb38e8,false)?;
    // VMA 0xb3780
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,16)?)?;
    // VMA 0xb3784
    c.b(C::Ge,0xb3b00,false)?;
    // VMA 0xb3788
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xb378c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::LR,Shift::Lsl,2,false)?)?;
    // VMA 0xb3790
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,-67)?)?;
    // VMA 0xb3794
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2147483648)?)?;
    // VMA 0xb3798
    c.b(C::Ne,0xb3820,false)?;
    // VMA 0xb379c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xb37a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // VMA 0xb37a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,36)?)?;
    // VMA 0xb37a8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb37ac
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::LR,Shift::Lsl,2,false)?)?;
    // VMA 0xb37b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,-4)?)?;
    // VMA 0xb37b4
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::new(8)?,R::LR,Shift::Lsl,2,true)?)?;
    // VMA 0xb37b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb37bc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // VMA 0xb37c0
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::R0,R::R1,R::LR,Shift::Lsl,0,true)?)?;
    // VMA 0xb37c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // VMA 0xb37c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // VMA 0xb37cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb37d0
    c.b(C::Lt,0xb38b8,false)?;
    // VMA 0xb37d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // VMA 0xb37d8
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(7)?,R::LR,0)?)?;
    // VMA 0xb37dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb37e0
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xb37e4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(5)?,R::new(9)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb37e8
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::new(5)?,R::new(10)?,R::R12)?)?;
    // VMA 0xb37ec
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3)?)?;
    // VMA 0xb37f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb37f4
    c.b(C::Eq,0xb3810,false)?;
    // VMA 0xb37f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // VMA 0xb37fc
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(8)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3800
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3804
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // VMA 0xb3808
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb380c
    c.b(C::Eq,0xb388c,false)?;
    // VMA 0xb3810
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb3814
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3818
    c.b(C::Lt,0xb37e4,false)?;
    // VMA 0xb381c
    c.b(C::Al,0xb38b8,false)?;
    // VMA 0xb3820
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xb3824
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3828
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb382c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(8)?,R::LR,Shift::Lsl,2,true)?)?;
    // VMA 0xb3830
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // VMA 0xb3834
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::R0,R::R1,R::LR,Shift::Lsl,0,true)?)?;
    // VMA 0xb3838
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // VMA 0xb383c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // VMA 0xb3840
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3844
    c.b(C::Lt,0xb38b8,false)?;
    // VMA 0xb3848
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // VMA 0xb384c
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(7)?,R::LR,0)?)?;
    // VMA 0xb3850
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3854
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xb3858
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(6)?,R::new(9)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb385c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(6)?,R::new(10)?,R::R2)?)?;
    // VMA 0xb3860
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,3)?)?;
    // VMA 0xb3864
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb3868
    c.b(C::Eq,0xb387c,false)?;
    // VMA 0xb386c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // VMA 0xb3870
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(8)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3874
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3878
    c.b(C::Eq,0xb38a4,false)?;
    // VMA 0xb387c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xb3880
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3884
    c.b(C::Lt,0xb3858,false)?;
    // VMA 0xb3888
    c.b(C::Al,0xb38b8,false)?;
    // VMA 0xb388c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-44)?)?;
    // VMA 0xb3890
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3894
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb3898
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // VMA 0xb389c
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb38a0
    c.b(C::Al,0xb38b8,false)?;
    // VMA 0xb38a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-44)?)?;
    // VMA 0xb38a8
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::new(6)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb38ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb38b0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // VMA 0xb38b4
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,4)?)?;
    // VMA 0xb38b8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,48)?)?;
    // VMA 0xb38bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xb38c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xb38c4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::LR,R::LR,1)?)?;
    // VMA 0xb38c8
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,12)?)?;
    // VMA 0xb38cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,13)?)?;
    // VMA 0xb38d0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb38d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xb38d8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb38dc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb38e0
    c.b(C::Gt,0xb376c,false)?;
    // VMA 0xb38e4
    c.b(C::Al,0xb3b14,false)?;
    // VMA 0xb38e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,24)?)?;
    // VMA 0xb38ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,16)?)?;
    // VMA 0xb38f0
    c.b(C::Ge,0xb3b84,false)?;
    // VMA 0xb38f4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xb38f8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::LR,Shift::Lsl,2,false)?)?;
    // VMA 0xb38fc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,-67)?)?;
    // VMA 0xb3900
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2147483648)?)?;
    // VMA 0xb3904
    c.b(C::Ne,0xb398c,false)?;
    // VMA 0xb3908
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xb390c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,44)?)?;
    // VMA 0xb3910
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,52)?)?;
    // VMA 0xb3914
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,36)?)?;
    // VMA 0xb3918
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::new(8)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb391c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R2,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3920
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3924
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::LR,Shift::Lsl,2,false)?)?;
    // VMA 0xb3928
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,-4)?)?;
    // VMA 0xb392c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,468)?)?;
    // VMA 0xb3930
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255&0xffffffff)?)?;
    // VMA 0xb3934
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R12,644)?)?;
    // VMA 0xb3938
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,248)?)?;
    // VMA 0xb393c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // VMA 0xb3940
    c.b(C::Lt,0xb3a1c,false)?;
    // VMA 0xb3944
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // VMA 0xb3948
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb394c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // VMA 0xb3950
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(9)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb3954
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::R1,R::new(10)?,R::new(7)?)?)?;
    // VMA 0xb3958
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,3)?)?;
    // VMA 0xb395c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb3960
    c.b(C::Eq,0xb397c,false)?;
    // VMA 0xb3964
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,12)?)?;
    // VMA 0xb3968
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,468)?)?;
    // VMA 0xb396c
    c.emit(arm32::data_register(C::Al,Op::Eor,R::new(5)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3970
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // VMA 0xb3974
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3978
    c.b(C::Eq,0xb39f8,false)?;
    // VMA 0xb397c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb3980
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3984
    c.b(C::Lt,0xb3950,false)?;
    // VMA 0xb3988
    c.b(C::Al,0xb3a1c,false)?;
    // VMA 0xb398c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // VMA 0xb3990
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // VMA 0xb3994
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(8)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3998
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb399c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb39a0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,468)?)?;
    // VMA 0xb39a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255&0xffffffff)?)?;
    // VMA 0xb39a8
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R12,644)?)?;
    // VMA 0xb39ac
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,248)?)?;
    // VMA 0xb39b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // VMA 0xb39b4
    c.b(C::Lt,0xb3a1c,false)?;
    // VMA 0xb39b8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,52)?)?;
    // VMA 0xb39bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb39c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // VMA 0xb39c4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(4)?,R::new(9)?,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb39c8
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(4)?,R::new(10)?,R::new(7)?)?)?;
    // VMA 0xb39cc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,3)?)?;
    // VMA 0xb39d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb39d4
    c.b(C::Eq,0xb39e8,false)?;
    // VMA 0xb39d8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,12)?)?;
    // VMA 0xb39dc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,468)?)?;
    // VMA 0xb39e0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb39e4
    c.b(C::Eq,0xb3a0c,false)?;
    // VMA 0xb39e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb39ec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb39f0
    c.b(C::Lt,0xb39c4,false)?;
    // VMA 0xb39f4
    c.b(C::Al,0xb3a1c,false)?;
    // VMA 0xb39f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb39fc
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R12,644)?)?;
    // VMA 0xb3a00
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // VMA 0xb3a04
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb3a08
    c.b(C::Al,0xb3a1c,false)?;
    // VMA 0xb3a0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb3a10
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::R12,644)?)?;
    // VMA 0xb3a14
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // VMA 0xb3a18
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,4)?)?;
    // VMA 0xb3a1c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xb3a20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,16)?)?;
    // VMA 0xb3a24
    c.b(C::Ge,0xb3ba0,false)?;
    // VMA 0xb3a28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xb3a2c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xb3a30
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb3a34
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3a38
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xb3a3c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // VMA 0xb3a40
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::LR,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3a44
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,644)?)?;
    // VMA 0xb3a48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,255)?)?;
    // VMA 0xb3a4c
    c.b(C::Eq,0xb3a6c,false)?;
    // VMA 0xb3a50
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // VMA 0xb3a54
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3a58
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,160)?)?;
    // VMA 0xb3a5c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // VMA 0xb3a60
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    // VMA 0xb3a64
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,8)?)?;
    // VMA 0xb3a68
    c.b(C::Al,0xb3ac4,false)?;
    // VMA 0xb3a6c
    c.literal(R::R1,0xb3bd8)?;
    // VMA 0xb3a70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb3a74
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xb3a78
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb3a7c
    c.literal(R::R1,0xb3bcc)?;
    // VMA 0xb3a80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb3a84
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb3a88
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    // VMA 0xb3a8c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,8)?)?;
    // VMA 0xb3a90
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    // VMA 0xb3a94
    c.b(C::Lt,0xb3aa8,false)?;
    // VMA 0xb3a98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,131&0xffffffff)?)?;
    // VMA 0xb3a9c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xb3aa0
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(7)?,R::R0)?)?;
    // VMA 0xb3aa4
    c.b(C::Al,0xb3aac,false)?;
    // VMA 0xb3aa8
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3aac
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3ab0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // VMA 0xb3ab4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,644)?)?;
    // VMA 0xb3ab8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,676)?)?;
    // VMA 0xb3abc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xb3ac0
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::LR,676)?)?;
    // VMA 0xb3ac4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // VMA 0xb3ac8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xb3acc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,12)?)?;
    // VMA 0xb3ad0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,13)?)?;
    // VMA 0xb3ad4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3ad8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // VMA 0xb3adc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3ae0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // VMA 0xb3ae4
    c.b(C::Gt,0xb3730,false)?;
    // VMA 0xb3ae8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3aec
    c.b(C::Al,0xb3af8,false)?;
    // VMA 0xb3af0
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-48)?)?;
    // VMA 0xb3af4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3af8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xb3afc
    c.b(C::Al,0xb3b24,false)?;
    // VMA 0xb3b00
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb3b04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,47&0xffffffff)?)?;
    // VMA 0xb3b08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3b0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb3b10
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb3b14
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3b18
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xb3b1c
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,24)?)?;
    // VMA 0xb3b20
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,20)?)?;
    // VMA 0xb3b24
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::LR,271)?)?;
    // VMA 0xb3b28
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::LR,272)?)?;
    // VMA 0xb3b2c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,248)?)?;
    // VMA 0xb3b30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xb3b34
    c.b(C::Lt,0xb2f30,false)?;
    // VMA 0xb3b38
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::LR))?;
    // VMA 0xb3b3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3b40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,84&0xffffffff)?)?;
    // VMA 0xb3b44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3b48
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R2,R::new(3)?,180)?)?;
    // VMA 0xb3b4c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(5)?,R::new(3)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3b50
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::new(5)?,R::R12,R::R2)?)?;
    // VMA 0xb3b54
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // VMA 0xb3b58
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::new(7)?,R::new(5)?,3)?)?;
    // VMA 0xb3b5c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb3b60
    c.b(C::Eq,0xb3b70,false)?;
    // VMA 0xb3b64
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,4)?)?;
    // VMA 0xb3b68
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb3b6c
    c.emit(arm32::transfer_byte(C::Eq,false,R::new(8)?,R::new(4)?,5)?)?;
    // VMA 0xb3b70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb3b74
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(5)?,0)?)?;
    // VMA 0xb3b78
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3b7c
    c.b(C::Lt,0xb3b4c,false)?;
    // VMA 0xb3b80
    c.b(C::Al,0xb2f30,false)?;
    // VMA 0xb3b84
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb3b88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3b8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,235&0xffffffff)?)?;
    // VMA 0xb3b90
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb3b94
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3b98
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xb3b9c
    c.b(C::Al,0xb3b20,false)?;
    // VMA 0xb3ba0
    c.literal(R::R2,0xb3bd0)?;
    // VMA 0xb3ba4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23&0xffffffff)?)?;
    // VMA 0xb3ba8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3bac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb3bb0
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb3bb4
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,52)?)?;
    // VMA 0xb3bb8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xb3bbc
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xb3bc0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(5)?))?;
    // VMA 0xb3bc4
    c.b(C::Al,0xb3b24,false)?;

    c.check(0xb3bc8)
}
