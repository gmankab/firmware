
/// Continue the preceding controller routine across the 0xa8000 ownership boundary, record codec status and command completion.
fn continue_prior_codec_release(c:&mut Code)->Result<(),String>{
    c.check(0xa8000)?;
    // VMA 0xa8000
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,28)?)?;
    // VMA 0xa8004
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,904)?)?;
    // VMA 0xa8008
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,32)?)?;
    // VMA 0xa800c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xa8010
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa8014
    c.b(C::Al,0xa8ea0,true)?;
    // VMA 0xa8018
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa801c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8020
    c.b(C::Al,0xb40ac,true)?;
    // VMA 0xa8024
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3968)?)?;
    // VMA 0xa8028
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // VMA 0xa802c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xa8030
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // VMA 0xa8034
    c.b(C::Al,0xd98b8,true)?;
    // VMA 0xa8038
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa803c
    c.b(C::Eq,0xa8078,false)?;
    // VMA 0xa8040
    c.literal(R::R1,0xa83c4)?;
    // VMA 0xa8044
    c.literal(R::R2,0xa83c8)?;
    // VMA 0xa8048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // VMA 0xa804c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8050
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xa8054
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8058
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa805c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8060
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8064
    c.literal(R::R2,0xa8398)?;
    // VMA 0xa8068
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // VMA 0xa806c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8070
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xa8074
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa8078
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,764)?)?;
    // VMA 0xa807c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa8080
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,764)?)?;
    // VMA 0xa8084
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xa8088
    c.b(C::Al,0xd9a34,true)?;
    // VMA 0xa808c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // VMA 0xa8090
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,740)?)?;
    // VMA 0xa8094
    c.b(C::Al,0xa71d0,false)?;
    // VMA 0xa8098
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,732)?)?;
    // VMA 0xa809c
    c.literal(R::R1,0xa83ec)?;
    // VMA 0xa80a0
    c.literal(R::R2,0xa83c8)?;
    // VMA 0xa80a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,39)?)?;
    // VMA 0xa80a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xa80ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xa80b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa80b4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa80b8
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa80bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa80c0
    c.b(C::Al,0xa71cc,false)?;
    // VMA 0xa80c4
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2828)?)?;
    // VMA 0xa80c8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa80cc
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,1096)?)?;
    // VMA 0xa80d0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xa80d4
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,1896)?)?;
    // VMA 0xa80d8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,592)?)?;
    // VMA 0xa80dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xa80e0
    c.b(C::Ne,0xa80fc,false)?;
    // VMA 0xa80e4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1096)?)?;
    // VMA 0xa80e8
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,44)?)?;
    // VMA 0xa80ec
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // VMA 0xa80f0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa80f4
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,1100)?)?;
    // VMA 0xa80f8
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R0,1104)?)?;
    // VMA 0xa80fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1100)?)?;
    // VMA 0xa8100
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8104
    c.b(C::Eq,0xa8124,false)?;
    // VMA 0xa8108
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xa810c
    c.b(C::Al,0xa8ee8,true)?;
    // VMA 0xa8110
    c.b(C::Al,0xa8134,false)?;
    c.check(0xa8114)
}

// This four-word inline literal pool is jumped over by the source routine.
const CONTROLLER_STATUS_DIAGNOSTICS:[u32;4]=[
    0x000f_60d0, 0x000f_6074, 0x000f_7fd1, 0x000f_611a,
];
/// Account event buffers, synchronize state flags and issue the firmware trace callbacks.
fn account_codec_state_and_notify(c:&mut Code)->Result<(),String>{
    c.check(0xa8124)?;
    // VMA 0xa8124
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2476)?)?;
    // VMA 0xa8128
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xa812c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xa8130
    c.b(C::Al,0xb48bc,true)?;
    // VMA 0xa8134
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,776)?)?;
    // VMA 0xa8138
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // VMA 0xa813c
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,572)?)?;
    // VMA 0xa8140
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8144
    c.b(C::Eq,0xa8154,false)?;
    // VMA 0xa8148
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xa814c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8150
    c.b(C::Al,0xb40ac,true)?;
    // VMA 0xa8154
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xa8158
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3556)?)?;
    // VMA 0xa815c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,1096)?)?;
    // VMA 0xa8160
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,1904)?)?;
    // VMA 0xa8164
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa8168
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1904)?)?;
    // VMA 0xa816c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,1100)?)?;
    // VMA 0xa8170
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xa8174
    c.b(C::Ne,0xa8204,false)?;
    // VMA 0xa8178
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,2,true)?)?;
    // VMA 0xa817c
    c.b(C::Eq,0xa8204,false)?;
    // VMA 0xa8180
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,44)?)?;
    // VMA 0xa8184
    c.literal(R::R2,0xa8398)?;
    // VMA 0xa8188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa818c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xa8190
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,206)?)?;
    // VMA 0xa8194
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8198
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xa819c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa81a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3056)?)?;
    // VMA 0xa81a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa81a8
    c.b(C::Ne,0xa8204,false)?;
    // VMA 0xa81ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xa81b0
    c.literal(R::R1,0xa83d8)?;
    // VMA 0xa81b4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // VMA 0xa81b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa81bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa81c0
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa81c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa81c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa81cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2444)?)?;
    // VMA 0xa81d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,110)?)?;
    // VMA 0xa81d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa81d8
    c.b(C::Ne,0xa81e8,false)?;
    // VMA 0xa81dc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,111)?)?;
    // VMA 0xa81e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa81e4
    c.b(C::Eq,0xa8390,false)?;
    // VMA 0xa81e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,588)?)?;
    // VMA 0xa81ec
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xa81f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xa81f4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2147483648)?)?;
    // VMA 0xa81f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,120)?)?;
    // VMA 0xa81fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,402653184)?)?;
    // VMA 0xa8200
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,252)?)?;
    // VMA 0xa8204
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3056)?)?;
    // VMA 0xa8208
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xa820c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa8210
    c.b(C::Ne,0xa824c,false)?;
    // VMA 0xa8214
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1096)?)?;
    // VMA 0xa8218
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,44)?)?;
    // VMA 0xa821c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xa8220
    c.b(C::Ne,0xa824c,false)?;
    // VMA 0xa8224
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // VMA 0xa8228
    c.literal(R::R1,0xa83dc)?;
    // VMA 0xa822c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa8230
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8234
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa8238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa823c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8240
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xa8244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,134217728)?)?;
    // VMA 0xa8248
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,252)?)?;
    // VMA 0xa824c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,776)?)?;
    // VMA 0xa8250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xa8254
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,20)?)?;
    // VMA 0xa8258
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,896)?)?;
    // VMA 0xa825c
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::R2,R::R1,Shift::Lsr,7,false)?)?;
    // VMA 0xa8260
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,28)?)?;
    // VMA 0xa8264
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,1096)?)?;
    // VMA 0xa8268
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2452)?)?;
    // VMA 0xa826c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1912)?)?;
    // VMA 0xa8270
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa8274
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa8278
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // VMA 0xa827c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1912)?)?;
    // VMA 0xa8280
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2496)?)?;
    // VMA 0xa8284
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8288
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,2452)?)?;
    // VMA 0xa828c
    c.emit(arm32::mov_shifted_register(C::Ne,R::R1,R::R1,Shift::Lsl,20)?)?;
    // VMA 0xa8290
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8294
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // VMA 0xa8298
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // VMA 0xa829c
    c.b(C::Al,0xd98b8,true)?;
    // VMA 0xa82a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa82a4
    c.b(C::Eq,0xa82e0,false)?;
    // VMA 0xa82a8
    c.literal(R::R1,0xa83c4)?;
    // VMA 0xa82ac
    c.literal(R::R2,0xa83c8)?;
    // VMA 0xa82b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,250)?)?;
    // VMA 0xa82b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa82b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xa82bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa82c0
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa82c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa82c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa82cc
    c.literal(R::R2,0xa8398)?;
    // VMA 0xa82d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,251)?)?;
    // VMA 0xa82d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa82d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xa82dc
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa82e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,764)?)?;
    // VMA 0xa82e4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa82e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,764)?)?;
    // VMA 0xa82ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa82f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2436)?)?;
    // VMA 0xa82f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xa82f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xa82fc
    c.b(C::Al,0xd9a34,true)?;
    // VMA 0xa8300
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,776)?)?;
    // VMA 0xa8304
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8308
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,9)?)?;
    // VMA 0xa830c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,740)?)?;
    // VMA 0xa8310
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xa8314
    c.b(C::Al,0xd9a54,true)?;
    // VMA 0xa8318
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa831c
    c.b(C::Ne,0xa8328,false)?;
    // VMA 0xa8320
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xa8324
    c.b(C::Al,0x292c,true)?;
    // VMA 0xa8328
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,880)?)?;
    // VMA 0xa832c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,740)?)?;
    // VMA 0xa8330
    c.literal(R::R1,0xa83f4)?;
    // VMA 0xa8334
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xa8338
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa833c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa8340
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xa8344
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8348
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa834c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa8350
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8354
    c.b(C::Al,0x2a158,true)?;
    // VMA 0xa8358
    c.b(C::Al,0xa71d0,false)?;
    c.check(0xa835c)
}

const CODEC_STATUS_MESSAGE: u32=0x000f_806b;
/// Check the codec status and post callbacks before returning to the caller.
fn notify_on_codec_status(c:&mut Code)->Result<(),String>{
    c.check(0xa8360)?;
    // VMA 0xa8360
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3968)?)?;
    // VMA 0xa8364
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xa8368
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // VMA 0xa836c
    c.b(C::Al,0xe1568,true)?;
    // VMA 0xa8370
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3968)?)?;
    // VMA 0xa8374
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,11)?)?;
    // VMA 0xa8378
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa837c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xa8380
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xa8384
    c.b(C::Al,0xaba80,true)?;
    // VMA 0xa8388
    c.b(C::Al,0xa82f8,false)?;
    c.check(0xa838c)
}

const CONTROLLER_ERROR_CLASS:u32=0x0100_1005;
/// Finalize the exceptional codec recovery path from the earlier function.
fn final_codec_recovery_branch(c:&mut Code)->Result<(),String>{
    c.check(0xa8390)?;
    // VMA 0xa8390
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,584)?)?;
    // VMA 0xa8394
    c.b(C::Al,0xa81ec,false)?;
    c.check(0xa8398)
}

// 24 mixed references in the caller's trailing read-only diagnostic table.
const CONTROLLER_MESSAGE_CONSTANTS:[u32;24]=[
    0x000f_7e7b, 0x000e_2788, 0x0021_100e, 0x000f_655d,
    0x000f_549b, 0x0010_dd80, 0x000f_8163, 0x000f_649c,
    0x000f_541d, 0x000f_8116, 0x000f_642a, 0x000f_61cb,
    0x000f_8021, 0x000f_8144, 0x000f_80bc, 0x000f_80eb,
    0x000f_62f1, 0x000f_6310, 0x000f_7d9b, 0x000f_60fe,
    0x000f_7ff4, 0x000f_8035, 0x000f_61ac, 0x000f_63d1,
];
