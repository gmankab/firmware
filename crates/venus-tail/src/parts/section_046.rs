
/// Configure codec input planes and channel resource counts. Four codec
/// families require different stride and buffer-size calculations; the
/// dispatch data is an explicit four-entry function-address table, not A32.
/// Notify the platform of invalid descriptors without mutating unknown data.
fn configure_codec_input_plane_dimensions(c:&mut Code)->Result<(),String>{
    c.check(0xb80d8)?;
    // ARM VMA 0xb80d8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb80dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM VMA 0xb80e0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb80e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // ARM VMA 0xb80e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb80ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,360)?)?;
    // ARM VMA 0xb80f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // ARM VMA 0xb80f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,12)?)?;
    // ARM VMA 0xb80f8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,464)?)?;
    // ARM VMA 0xb80fc
    c.b(C::Al,0x27718,true)?;
    // ARM VMA 0xb8100
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,456)?)?;
    // ARM VMA 0xb8104
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb8108
    c.b(C::Hi,0xb814c,false)?;
    // ARM VMA 0xb810c
    c.literal(R::R1,0xb84dc)?;
    // ARM VMA 0xb8110
    c.literal(R::R2,0xb84e0)?;
    // ARM VMA 0xb8114
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,145)?)?;
    // ARM VMA 0xb8118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb811c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // ARM VMA 0xb8120
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8124
    c.literal(R::R1,0xb84e4)?;
    // ARM VMA 0xb8128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb812c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8134
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,520)?)?;
    // ARM VMA 0xb8138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb813c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,568)?)?;
    // ARM VMA 0xb8140
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,472)?)?;
    // ARM VMA 0xb8144
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2132)?)?;
    // ARM VMA 0xb8148
    c.b(C::Al,0xb8158,false)?;
    // ARM VMA 0xb814c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb8150
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb8154
    c.b(C::Al,0xbd000,true)?;
    // ARM VMA 0xb8158
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // ARM VMA 0xb815c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // ARM VMA 0xb8160
    c.b(C::Al,0x27840,true)?;
    // ARM VMA 0xb8164
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,200)?)?;
    // ARM VMA 0xb8168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb816c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,680)?)?;
    // ARM VMA 0xb8170
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,196)?)?;
    // ARM VMA 0xb8174
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,676)?)?;
    // ARM VMA 0xb8178
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,204)?)?;
    // ARM VMA 0xb817c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,684)?)?;
    // ARM VMA 0xb8180
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,208)?)?;
    // ARM VMA 0xb8184
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,688)?)?;
    // ARM VMA 0xb8188
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,212)?)?;
    // ARM VMA 0xb818c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,692)?)?;
    // ARM VMA 0xb8190
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,152)?)?;
    // ARM VMA 0xb8194
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,188)?)?;
    // ARM VMA 0xb8198
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,153)?)?;
    // ARM VMA 0xb819c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,189)?)?;
    // ARM VMA 0xb81a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb81a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb81a8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb81ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // ARM VMA 0xb81b0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb81b4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,24)?)?;
    // ARM VMA 0xb81b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // ARM VMA 0xb81bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,60)?)?;
    // ARM VMA 0xb81c0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,88)?)?;
    // ARM VMA 0xb81c4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,124)?)?;
    // ARM VMA 0xb81c8
    c.b(C::Ne,0xb81a4,false)?;
    // ARM VMA 0xb81cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,232)?)?;
    // ARM VMA 0xb81d0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM VMA 0xb81d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // ARM VMA 0xb81d8
    c.b(C::Al,0xd8be4,true)?;
    // ARM VMA 0xb81dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,972)?)?;
    // ARM VMA 0xb81e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb81e4
    c.b(C::Ne,0xb81f4,false)?;
    // ARM VMA 0xb81e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,2320)?)?;
    // ARM VMA 0xb81ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb81f0
    c.b(C::Eq,0xb8204,false)?;
    // ARM VMA 0xb81f4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,304)?)?;
    // ARM VMA 0xb81f8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM VMA 0xb81fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // ARM VMA 0xb8200
    c.b(C::Al,0xd8be4,true)?;
    // ARM VMA 0xb8204
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,20)?)?;
    // ARM VMA 0xb8208
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // ARM VMA 0xb820c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,156)?)?;
    // ARM VMA 0xb8210
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,192)?)?;
    // ARM VMA 0xb8214
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,488)?)?;
    // ARM VMA 0xb8218
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,52)?)?;
    // ARM VMA 0xb821c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,484)?)?;
    // ARM VMA 0xb8220
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // ARM VMA 0xb8224
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,56)?)?;
    // ARM VMA 0xb8228
    c.literal(R::R2,0xb84e8)?;
    // ARM VMA 0xb822c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,48)?)?;
    // ARM VMA 0xb8230
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // ARM VMA 0xb8234
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,40)?)?;
    // ARM VMA 0xb8238
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // ARM VMA 0xb823c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,44)?)?;
    // ARM VMA 0xb8240
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,172)?)?;
    // ARM VMA 0xb8244
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,196)?)?;
    // ARM VMA 0xb8248
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,3)?)?;
    // ARM VMA 0xb824c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8250
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM VMA 0xb8254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,724)?)?;
    // ARM VMA 0xb8258
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb825c
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb8260
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8264
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,664)?)?;
    // ARM VMA 0xb8268
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,668)?)?;
    // ARM VMA 0xb826c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,1117)?)?;
    // ARM VMA 0xb8270
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb8274
    c.b(C::Eq,0xb8374,false)?;
    // ARM VMA 0xb8278
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // ARM VMA 0xb827c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,8)?)?;
    // ARM VMA 0xb8280
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb8284
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,12)?)?;
    // ARM VMA 0xb8288
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::new(4)?,176)?)?;
    // ARM VMA 0xb828c
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::new(4)?,180)?)?;
    // ARM VMA 0xb8290
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // ARM VMA 0xb8294
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8298
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // ARM VMA 0xb829c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb82a0
    c.b(C::Eq,0xb82ec,false)?;
    // ARM VMA 0xb82a4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,4)?)?;
    // ARM VMA 0xb82a8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,184)?)?;
    // ARM VMA 0xb82ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4)?)?;
    // ARM VMA 0xb82b0
    c.b(C::Ne,0xb831c,false)?;
    // ARM VMA 0xb82b4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // ARM VMA 0xb82b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // ARM VMA 0xb82bc
    c.b(C::Hi,0xb8340,false)?;
    // ARM VMA 0xb82c0
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,2)?)?;
    // ARM VMA 0xb82c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::PC,0)?)?;
    // ARM VMA 0xb82c8
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0xb82cc)?;
    for target in CODEC_PLANE_SIZE_DISPATCH {c.emit(target)?;}
    c.check(0xb82dc)?;
    // ARM VMA 0xb82dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb82e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,664)?)?;
    // ARM VMA 0xb82e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,668)?)?;
    // ARM VMA 0xb82e8
    c.b(C::Al,0xb8374,false)?;
    // ARM VMA 0xb82ec
    c.literal(R::R1,0xb84f4)?;
    // ARM VMA 0xb82f0
    c.literal(R::R2,0xb84e0)?;
    // ARM VMA 0xb82f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,25)?)?;
    // ARM VMA 0xb82f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // ARM VMA 0xb82fc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,188)?)?;
    // ARM VMA 0xb8300
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM VMA 0xb8304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb8308
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb830c
    c.literal(R::R1,0xb84e4)?;
    // ARM VMA 0xb8310
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb8314
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8318
    c.b(C::Al,0xb8374,false)?;
    // ARM VMA 0xb831c
    c.literal(R::new(3)?,0xb84ec)?;
    // ARM VMA 0xb8320
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // ARM VMA 0xb8324
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // ARM VMA 0xb8328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1808)?)?;
    // ARM VMA 0xb832c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // ARM VMA 0xb8330
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // ARM VMA 0xb8334
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,664)?)?;
    // ARM VMA 0xb8338
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,668)?)?;
    // ARM VMA 0xb833c
    c.b(C::Al,0xb8374,false)?;
    // ARM VMA 0xb8340
    c.literal(R::R1,0xb84f0)?;
    // ARM VMA 0xb8344
    c.literal(R::R2,0xb84e0)?;
    // ARM VMA 0xb8348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // ARM VMA 0xb834c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,776)?)?;
    // ARM VMA 0xb8350
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,188)?)?;
    // ARM VMA 0xb8354
    c.b(C::Al,0xb8304,false)?;
    // ARM VMA 0xb8358
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // ARM VMA 0xb835c
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // ARM VMA 0xb8360
    c.b(C::Al,0xb836c,false)?;
    // ARM VMA 0xb8364
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,3,false)?)?;
    // ARM VMA 0xb8368
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // ARM VMA 0xb836c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,664)?)?;
    // ARM VMA 0xb8370
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,668)?)?;
    // ARM VMA 0xb8374
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // ARM VMA 0xb8378
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb837c
    c.b(C::Eq,0xb8394,false)?;
    // ARM VMA 0xb8380
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,176)?)?;
    // ARM VMA 0xb8384
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,696)?)?;
    // ARM VMA 0xb8388
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,180)?)?;
    // ARM VMA 0xb838c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,700)?)?;
    // ARM VMA 0xb8390
    c.b(C::Al,0xb83b0,false)?;
    // ARM VMA 0xb8394
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // ARM VMA 0xb8398
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb839c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb83a0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,196)?)?;
    // ARM VMA 0xb83a4
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit() | R::R1.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb83a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,208)?)?;
    // ARM VMA 0xb83ac
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,212)?)?;
    // ARM VMA 0xb83b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,560)?)?;
    // ARM VMA 0xb83b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb83b8
    c.b(C::Ne,0xb83d4,false)?;
    // ARM VMA 0xb83bc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb83c0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb83c4
    c.b(C::Al,0xb7f94,true)?;
    // ARM VMA 0xb83c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb83cc
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,764)?)?;
    // ARM VMA 0xb83d0
    c.b(C::Al,0xb84c0,false)?;
    // ARM VMA 0xb83d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,1060)?)?;
    // ARM VMA 0xb83d8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb83dc
    c.b(C::Eq,0xb8484,false)?;
    // ARM VMA 0xb83e0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // ARM VMA 0xb83e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM VMA 0xb83e8
    c.b(C::Eq,0xb8444,false)?;
    // ARM VMA 0xb83ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM VMA 0xb83f0
    c.b(C::Eq,0xb8454,false)?;
    // ARM VMA 0xb83f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb83f8
    c.emit(arm32::data_immediate(C::Eq,Op::Tst,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb83fc
    c.b(C::Ne,0xb8484,false)?;
    // ARM VMA 0xb8400
    c.literal(R::R1,0xb84f8)?;
    // ARM VMA 0xb8404
    c.literal(R::R2,0xb84e0)?;
    // ARM VMA 0xb8408
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,70)?)?;
    // ARM VMA 0xb840c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8410
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1028)?)?;
    // ARM VMA 0xb8414
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM VMA 0xb8418
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb841c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb8420
    c.literal(R::R1,0xb84e4)?;
    // ARM VMA 0xb8424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb8428
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb842c
    c.literal(R::R2,0xb84e8)?;
    // ARM VMA 0xb8430
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,71)?)?;
    // ARM VMA 0xb8434
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8438
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // ARM VMA 0xb843c
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb8440
    c.b(C::Al,0xb8484,false)?;
    // ARM VMA 0xb8444
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb8448
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb844c
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(6)?,1028)?)?;
    // ARM VMA 0xb8450
    c.b(C::Al,0xb8484,false)?;
    // ARM VMA 0xb8454
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb8458
    c.b(C::Ne,0xb8484,false)?;
    // ARM VMA 0xb845c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,4)?)?;
    // ARM VMA 0xb8460
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1024)?)?;
    // ARM VMA 0xb8464
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb8468
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::R0,2144)?)?;
    // ARM VMA 0xb846c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM VMA 0xb8470
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb8474
    c.emit(arm32::mov_register(C::Ne,R::R0,R::R1))?;
    // ARM VMA 0xb8478
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // ARM VMA 0xb847c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb8480
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb8484
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,572)?)?;
    // ARM VMA 0xb8488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // ARM VMA 0xb848c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // ARM VMA 0xb8490
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb8494
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb8498
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(6)?,520)?)?;
    // ARM VMA 0xb849c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,504)?)?;
    // ARM VMA 0xb84a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb84a4
    c.b(C::Eq,0xb84bc,false)?;
    // ARM VMA 0xb84a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb84ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb84b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb84b4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,764)?)?;
    // ARM VMA 0xb84b8
    c.b(C::Al,0xb84c4,false)?;
    // ARM VMA 0xb84bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb84c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb84c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb84c8
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb84cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,100)?)?;
    // ARM VMA 0xb84d0
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // ARM VMA 0xb84d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,221)?)?;
    // ARM VMA 0xb84d8
    c.b(C::Al,0xb8330,false)?;
    c.check(0xb84dc)?;
    for pointer in CODEC_PLANE_CONFIGURATION_LOGS {c.emit(pointer)?;}
    c.check(0xb84fc)
}

const CODEC_INPUT_RECORD_WARNINGS:[u32;6]=[
    0x000f_9716,0x000f_9744,0x000e_2788,
    0x000f_975a,0x000f_9792,0x000f_9560,
];
const CODEC_CHANNEL_SETUP_WARNINGS:[u32;4]=[
    0x000f_9792,0x000f_97bb,0x000e_2788,0x000f_9560,
];
