
/// Initialize codec-controller stream state, choose active format,
/// and calculate bounded frame dimensions. The first routine's original
/// return is followed by three constant words; the next short function
/// resets default surface/control fields without copying OEM code.
fn initialize_codec_plane_status(c:&mut Code)->Result<(),String>{
    c.check(0xb60ec)?;
    // VMA 0xb60ec
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb60f0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xb60f4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xb60f8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xb60fc
    c.b(C::Al,0xd1480,true)?;
    // VMA 0xb6100
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb6104
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,140)?)?;
    // VMA 0xb6108
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xb610c
    c.b(C::Ne,0xb61ac,false)?;
    // VMA 0xb6110
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6114
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256&0xffffffff)?)?;
    // VMA 0xb6118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb611c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6120
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,304)?)?;
    // VMA 0xb6124
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,308)?)?;
    // VMA 0xb6128
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,312)?)?;
    // VMA 0xb612c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,316)?)?;
    // VMA 0xb6130
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,292)?)?;
    // VMA 0xb6134
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xb6138
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb613c
    c.b(C::Eq,0xb6190,false)?;
    // VMA 0xb6140
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,988)?)?;
    // VMA 0xb6144
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,468)?)?;
    // VMA 0xb6148
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,472)?)?;
    // VMA 0xb614c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,984)?)?;
    // VMA 0xb6150
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(4)?,1004)?)?;
    // VMA 0xb6154
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xb6158
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xb615c
    c.b(C::Al,0xd7a48,true)?;
    // VMA 0xb6160
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,984)?)?;
    // VMA 0xb6164
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,988)?)?;
    // VMA 0xb6168
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,468)?)?;
    // VMA 0xb616c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,472)?)?;
    // VMA 0xb6170
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb6174
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,612)?)?;
    // VMA 0xb6178
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2048)?)?;
    // VMA 0xb617c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xb6180
    c.b(C::Al,0xd7bf8,true)?;
    // VMA 0xb6184
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,292)?)?;
    // VMA 0xb6188
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb618c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,292)?)?;
    // VMA 0xb6190
    c.literal(R::R1,0xb6258)?;
    // VMA 0xb6194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6198
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,296)?)?;
    // VMA 0xb619c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb61a0
    c.literal(R::R1,0xb625c)?;
    // VMA 0xb61a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb61a8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb61ac
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,148)?)?;
    // VMA 0xb61b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20992&0xffffffff)?)?;
    // VMA 0xb61b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,131072)?)?;
    // VMA 0xb61b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb61bc
    c.b(C::Ne,0xb620c,false)?;
    // VMA 0xb61c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb61c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256&0xffffffff)?)?;
    // VMA 0xb61c8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,148)?)?;
    // VMA 0xb61cc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,360)?)?;
    // VMA 0xb61d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,364)?)?;
    // VMA 0xb61d4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,368)?)?;
    // VMA 0xb61d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,768&0xffffffff)?)?;
    // VMA 0xb61dc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,372)?)?;
    // VMA 0xb61e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,348)?)?;
    // VMA 0xb61e4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2308)?)?;
    // VMA 0xb61e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb61ec
    c.b(C::Ne,0xb6204,false)?;
    // VMA 0xb61f0
    c.literal(R::R1,0xb6260)?;
    // VMA 0xb61f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,348)?)?;
    // VMA 0xb61f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,40192&0xffffffff)?)?;
    // VMA 0xb61fc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,62652416)?)?;
    // VMA 0xb6200
    c.b(C::Al,0xb6208,false)?;
    // VMA 0xb6204
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,36096)?)?;
    // VMA 0xb6208
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,348)?)?;
    // VMA 0xb620c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6210
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256&0xffffffff)?)?;
    // VMA 0xb6214
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,172)?)?;
    // VMA 0xb6218
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,176)?)?;
    // VMA 0xb621c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,396)?)?;
    // VMA 0xb6220
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,168)?)?;
    // VMA 0xb6224
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,468)?)?;
    // VMA 0xb6228
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,352)?)?;
    // VMA 0xb622c
    c.b(C::Cc,0xb6244,false)?;
    // VMA 0xb6230
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,472)?)?;
    // VMA 0xb6234
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::R2,R::new(3)?)?)?;
    // VMA 0xb6238
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,288)?)?;
    // VMA 0xb623c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // VMA 0xb6240
    c.emit(arm32::mov_shifted_register(C::Cs,R::R0,R::R2,Shift::Lsr,1)?)?;
    // VMA 0xb6244
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,152)?)?;
    // VMA 0xb6248
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,156)?)?;
    // VMA 0xb624c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb6250
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xb6254
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb6258)?;
    const PLANE_STATUS_CONSTANTS:[u32;3]=[0x000f_8f04,0x000e_2788,0x03ba_db00];
    for word in PLANE_STATUS_CONSTANTS {c.emit(word)?;}
    c.check(0xb6264)?;
    // VMA 0xb6264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1920&0xffffffff)?)?;
    // VMA 0xb6268
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,60&0xffffffff)?)?;
    // VMA 0xb626c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5&0xffffffff)?)?;
    // VMA 0xb6270
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xb6274
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1088&0xffffffff)?)?;
    // VMA 0xb6278
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // VMA 0xb627c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6280
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,176)?)?;
    // VMA 0xb6284
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    // VMA 0xb6288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,80&0xffffffff)?)?;
    // VMA 0xb628c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // VMA 0xb6290
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6294
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,164)?)?;
    // VMA 0xb6298
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,144)?)?;
    // VMA 0xb629c
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,145)?)?;
    // VMA 0xb62a0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,148)?)?;
    // VMA 0xb62a4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,152)?)?;
    // VMA 0xb62a8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,156)?)?;
    // VMA 0xb62ac
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,184)?)?;
    // VMA 0xb62b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,188)?)?;
    // VMA 0xb62b4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,192)?)?;
    // VMA 0xb62b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,196)?)?;
    // VMA 0xb62bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,200)?)?;
    // VMA 0xb62c0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,204)?)?;
    // VMA 0xb62c4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,168)?)?;
    // VMA 0xb62c8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,172)?)?;
    // VMA 0xb62cc
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xb62d0)
}

/// Allocate a codec descriptor, initialize it with callback slots and
/// dispatch its controller setup operations. The original register restore
/// and return precede a twelve-word global/diagnostic constant pool.
fn create_controller_codec_descriptor(c:&mut Code)->Result<(),String>{
    c.check(0xb62d0)?;
    // VMA 0xb62d0
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb62d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xb62d8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xb62dc
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xb62e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2192&0xffffffff)?)?;
    // VMA 0xb62e4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(3)?))?;
    // VMA 0xb62e8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // VMA 0xb62ec
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xb62f0
    c.b(C::Al,0x2efc,true)?;
    // VMA 0xb62f4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xb62f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb62fc
    c.b(C::Eq,0xb636c,false)?;
    // VMA 0xb6300
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6308
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2192&0xffffffff)?)?;
    // VMA 0xb630c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6310
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xb6314
    c.literal(R::R0,0xb65b4)?;
    // VMA 0xb6318
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xb631c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xb6320
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // VMA 0xb6324
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xb6328
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb632c
    c.b(C::Al,0xd0fd0,true)?;
    // VMA 0xb6330
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb6334
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb6338
    c.b(C::Eq,0xb6394,false)?;
    // VMA 0xb633c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6340
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,16)?)?;
    // VMA 0xb6344
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R0,1292)?)?;
    // VMA 0xb6348
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,148)?)?;
    // VMA 0xb634c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2720)?)?;
    // VMA 0xb6350
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2724)?)?;
    // VMA 0xb6354
    c.b(C::Eq,0xb63cc,false)?;
    // VMA 0xb6358
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,8)?)?;
    // VMA 0xb635c
    c.b(C::Ne,0xb64dc,false)?;
    // VMA 0xb6360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3&0xffffffff)?)?;
    // VMA 0xb6364
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6368
    c.b(C::Al,0xb63d4,false)?;
    // VMA 0xb636c
    c.literal(R::R1,0xb65e0)?;
    // VMA 0xb6370
    c.literal(R::R2,0xb65c4)?;
    // VMA 0xb6374
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,47&0xffffffff)?)?;
    // VMA 0xb6378
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb637c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xb6380
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6384
    c.literal(R::R1,0xb65c8)?;
    // VMA 0xb6388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb638c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6390
    c.b(C::Al,0xb65a8,false)?;
    // VMA 0xb6394
    c.literal(R::R1,0xb65dc)?;
    // VMA 0xb6398
    c.literal(R::R2,0xb65c4)?;
    // VMA 0xb639c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb63a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,316&0xffffffff)?)?;
    // VMA 0xb63a4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb63a8
    c.literal(R::R1,0xb65c8)?;
    // VMA 0xb63ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb63b0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb63b4
    c.literal(R::R2,0xb65d4)?;
    // VMA 0xb63b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,61&0xffffffff)?)?;
    // VMA 0xb63bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb63c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb63c4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb63c8
    c.b(C::Al,0xb65a0,false)?;
    // VMA 0xb63cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4&0xffffffff)?)?;
    // VMA 0xb63d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb63d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2716)?)?;
    // VMA 0xb63d8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // VMA 0xb63dc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1108)?)?;
    // VMA 0xb63e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1112)?)?;
    // VMA 0xb63e4
    c.literal(R::R1,0xb65b8)?;
    // VMA 0xb63e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb63ec
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb63f0
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xb63f4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // VMA 0xb63f8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xb63fc
    c.b(C::Al,0x804fc,true)?;
    // VMA 0xb6400
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb6404
    c.literal(R::R1,0xb65bc)?;
    // VMA 0xb6408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(4)?,356)?)?;
    // VMA 0xb640c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6410
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xb6414
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,112)?)?;
    // VMA 0xb6418
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,516&0xffffffff)?)?;
    // VMA 0xb641c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10&0xffffffff)?)?;
    // VMA 0xb6420
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xb6424
    c.b(C::Al,0x7e544,true)?;
    // VMA 0xb6428
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,356)?)?;
    // VMA 0xb642c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb6430
    c.b(C::Eq,0xb6598,false)?;
    // VMA 0xb6434
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6438
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,228)?)?;
    // VMA 0xb643c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,232)?)?;
    // VMA 0xb6440
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,236)?)?;
    // VMA 0xb6444
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2164)?)?;
    // VMA 0xb6448
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2168)?)?;
    // VMA 0xb644c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb6450
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,28)?)?;
    // VMA 0xb6454
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb6458
    c.b(C::Eq,0xb6504,false)?;
    // VMA 0xb645c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,300)?)?;
    // VMA 0xb6460
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6468
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4&0xffffffff)?)?;
    // VMA 0xb646c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10&0xffffffff)?)?;
    // VMA 0xb6470
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xb6474
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // VMA 0xb6478
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // VMA 0xb647c
    c.b(C::Al,0x8080c,true)?;
    // VMA 0xb6480
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xb6484
    c.b(C::Ne,0xb653c,false)?;
    // VMA 0xb6488
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,244)?)?;
    // VMA 0xb648c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6490
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4&0xffffffff)?)?;
    // VMA 0xb6494
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10&0xffffffff)?)?;
    // VMA 0xb6498
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // VMA 0xb649c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // VMA 0xb64a0
    c.b(C::Al,0x8080c,true)?;
    // VMA 0xb64a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xb64a8
    c.b(C::Ne,0xb6564,false)?;
    // VMA 0xb64ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb64b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,372)?)?;
    // VMA 0xb64b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,376)?)?;
    // VMA 0xb64b8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,2160)?)?;
    // VMA 0xb64bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb64c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,2308)?)?;
    // VMA 0xb64c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,8)?)?;
    // VMA 0xb64c8
    c.b(C::Al,0xb6264,true)?;
    // VMA 0xb64cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb64d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,16)?)?;
    // VMA 0xb64d4
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::new(4)?,164)?)?;
    // VMA 0xb64d8
    c.b(C::Al,0xb65ac,false)?;
    // VMA 0xb64dc
    c.literal(R::R1,0xb65d8)?;
    // VMA 0xb64e0
    c.literal(R::R2,0xb65c4)?;
    // VMA 0xb64e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,85&0xffffffff)?)?;
    // VMA 0xb64e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb64ec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xb64f0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb64f4
    c.literal(R::R1,0xb65c8)?;
    // VMA 0xb64f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb64fc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6500
    c.b(C::Al,0xb6598,false)?;
    // VMA 0xb6504
    c.literal(R::R1,0xb65d0)?;
    // VMA 0xb6508
    c.literal(R::R2,0xb65c4)?;
    // VMA 0xb650c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb6510
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,376&0xffffffff)?)?;
    // VMA 0xb6514
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6518
    c.literal(R::R1,0xb65c8)?;
    // VMA 0xb651c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb6520
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6524
    c.literal(R::R2,0xb65d4)?;
    // VMA 0xb6528
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,121&0xffffffff)?)?;
    // VMA 0xb652c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6530
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb6534
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6538
    c.b(C::Al,0xb6590,false)?;
    // VMA 0xb653c
    c.literal(R::R1,0xb65c0)?;
    // VMA 0xb6540
    c.literal(R::R2,0xb65c4)?;
    // VMA 0xb6544
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,134&0xffffffff)?)?;
    // VMA 0xb6548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb654c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xb6550
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6554
    c.literal(R::R1,0xb65c8)?;
    // VMA 0xb6558
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb655c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6560
    c.b(C::Al,0xb6590,false)?;
    // VMA 0xb6564
    c.literal(R::R1,0xb65cc)?;
    // VMA 0xb6568
    c.literal(R::R2,0xb65c4)?;
    // VMA 0xb656c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,146&0xffffffff)?)?;
    // VMA 0xb6570
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb6574
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xb6578
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb657c
    c.literal(R::R1,0xb65c8)?;
    // VMA 0xb6580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb6584
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6588
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xb658c
    c.b(C::Al,0x808d0,true)?;
    // VMA 0xb6590
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xb6594
    c.b(C::Al,0x7e7d4,true)?;
    // VMA 0xb6598
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb659c
    c.b(C::Al,0xd3b78,true)?;
    // VMA 0xb65a0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb65a4
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xb65a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb65ac
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xb65b0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb65b4)?;
    const CODEC_DESCRIPTOR_GLOBALS:[u32;12]=[
        0x0010_b6dc,0x0010_b6ec,0x000b_60ec,0x000f_904f,
        0x000f_8f74,0x000e_2788,0x000f_9085,0x000f_9020,
        0x000f_8fbc,0x000f_8ff5,0x000f_8f88,0x000f_8f34,
    ];
    for word in CODEC_DESCRIPTOR_GLOBALS {c.emit(word)?;}
    c.check(0xb65e4)
}

/// Lightweight codec context forwarding to the existing private platform
/// entry points. Each wrapper preserves the original tail-call branch.
fn forward_codec_context_calls(c:&mut Code)->Result<(),String>{
    c.check(0xb65e4)?;
    // VMA 0xb65e4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb65e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xb65ec
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb65f0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xb65f4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xb65f8
    c.b(C::Al,0xb60ec,true)?;
    // VMA 0xb65fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xb6600
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xb6604
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb6608
    c.b(C::Al,0xd4bf8,false)?;
    // VMA 0xb660c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6610
    c.b(C::Al,0xd23f8,false)?;
    // VMA 0xb6614
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6618
    c.b(C::Al,0xd3244,false)?;
    // VMA 0xb661c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6620
    c.b(C::Al,0xd3580,false)?;
    // VMA 0xb6624
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6628
    c.b(C::Al,0xd370c,false)?;
    // VMA 0xb662c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6630
    c.b(C::Al,0xd38e0,false)?;
    // VMA 0xb6634
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6638
    c.b(C::Al,0xd33d4,false)?;
    // VMA 0xb663c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6640
    c.b(C::Al,0xd30c0,false)?;
    // VMA 0xb6644
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6648
    c.b(C::Al,0xd52e0,false)?;
    // VMA 0xb664c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6650
    c.b(C::Al,0xd7254,false)?;
    c.check(0xb6654)
}
