
/// Produce a partial executable function + explicitly distinguished data.
/// Only 4-byte-aligned, independently re-encoded instructions are emitted.
/// ARM32 controller service function: parse and route fourteen stream
/// command types using the firmware's jump table; inspect controller state
/// and prepare channel flags and byte-sized record fields. This function
/// continues after our current boundary, so it is not yet complete.
fn begin_controller_stream_classification(c:&mut Code)->Result<(),String>{
    c.check(0xb5008)?;
    // VMA 0xb5008
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb500c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb5010
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // VMA 0xb5014
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xb5018
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R1,8)?)?;
    // VMA 0xb501c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5024
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // VMA 0xb5028
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2596)?)?;
    // VMA 0xb502c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5030
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(4)?))?;
    // VMA 0xb5034
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,13)?)?;
    // VMA 0xb5038
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R2,R::new(10)?,2592)?)?;
    // VMA 0xb503c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,5)?)?;
    // VMA 0xb5040
    c.b(C::Hi,0xb50d0,false)?;
    // VMA 0xb5044
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xb5048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::PC,4)?)?;
    // VMA 0xb504c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5050
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::new(7)?,R::new(6)?,Shift::Lsl,0)?)?;
    c.check(0xb5054)?;
    const STREAM_CLASS_JUMP_TARGETS:[u32;14]=[
        0x000b_50a0,0x000b_50a0,
        0x000b_50d0,0x000b_50d0,0x000b_50d0,0x000b_50d0,
        0x000b_50d0,0x000b_50d0,
        0x000b_508c,0x000b_508c,0x000b_508c,
        0x000b_50ac,0x000b_50ac,0x000b_50c4,
    ];
    for target in STREAM_CLASS_JUMP_TARGETS { c.emit(target)?; }
    c.check(0xb508c)?;
    // VMA 0xb508c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5090
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5094
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,2592)?)?;
    // VMA 0xb5098
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,2594)?)?;
    // VMA 0xb509c
    c.b(C::Al,0xb50d0,false)?;
    // VMA 0xb50a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb50a4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2595)?)?;
    // VMA 0xb50a8
    c.b(C::Al,0xb50d0,false)?;
    // VMA 0xb50ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb50b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2592&0xffffffff)?)?;
    // VMA 0xb50b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb50b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,256)?)?;
    // VMA 0xb50bc
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::new(6)?,R::new(4)?,R::new(7)?)?)?;
    // VMA 0xb50c0
    c.b(C::Al,0xb50d0,false)?;
    // VMA 0xb50c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb50c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb50cc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(10)?,0)?)?;
    // VMA 0xb50d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2048&0xffffffff)?)?;
    // VMA 0xb50d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xb50d8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb50dc
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::R0,Shift::Lsl,15)?)?;
    // VMA 0xb50e0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::new(7)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb50e4
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb50e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4&0xffffffff)?)?;
    // VMA 0xb50ec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xb50f0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xb50f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb50f8
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb50fc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,2598)?)?;
    // VMA 0xb5100
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,2599)?)?;
    // VMA 0xb5104
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5108
    c.emit(arm32::transfer_word_register_offset(C::Eq,false,R::R2,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb510c
    c.emit(arm32::transfer_word_register_offset(C::Eq,true,R::R2,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5110
    c.emit(arm32::transfer_byte(C::Eq,false,R::R2,R::new(4)?,2599)?)?;
    // VMA 0xb5114
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,21)?)?;
    // VMA 0xb5118
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb511c
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(4)?,2599)?)?;
    // VMA 0xb5120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb5124
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5128
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb512c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2600)?)?;
    // VMA 0xb5130
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // VMA 0xb5134
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // VMA 0xb5138
    c.b(C::Cc,0xb51a4,false)?;
    // VMA 0xb513c
    c.literal(R::R1,0xb60e8)?;
    // VMA 0xb5140
    c.literal(R::R2,0xb60b8)?;
    // VMA 0xb5144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,113&0xffffffff)?)?;
    // VMA 0xb5148
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb514c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xb5150
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5154
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb5158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb515c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3&0xffffffff)?)?;
    // VMA 0xb5164
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,116)?)?;
    // VMA 0xb5168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb516c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb5170
    c.b(C::Al,0xb5178,false)?;
    // VMA 0xb5174
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb5178
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb517c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,4)?)?;
    // VMA 0xb5180
    c.b(C::Eq,0xb5174,false)?;
    // VMA 0xb5184
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // VMA 0xb5188
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2452)?)?;
    // VMA 0xb518c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5190
    c.b(C::Ne,0xb5288,false)?;
    // VMA 0xb5194
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb5198
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb519c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb51a0
    c.b(C::Al,0xb0f50,false)?;
    // VMA 0xb51a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb51a8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,3704)?)?;
    // VMA 0xb51ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb51b0
    c.b(C::Ne,0xb51d0,false)?;
    // VMA 0xb51b4
    c.literal(R::R1,0xb60a4)?;
    // VMA 0xb51b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb51bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb51c0
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb51c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb51c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb51cc
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2448)?)?;
    // VMA 0xb51d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2452)?)?;
    // VMA 0xb51d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb51d8
    c.emit(arm32::transfer_word(C::Ne,true,R::new(5)?,R::new(4)?,2448)?)?;
    // VMA 0xb51dc
    c.emit(arm32::data_register(C::Ne,Op::Cmp,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb51e0
    c.b(C::Eq,0xb5200,false)?;
    // VMA 0xb51e4
    c.literal(R::R1,0xb60ac)?;
    // VMA 0xb51e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb51ec
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb51f0
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb51f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb51f8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb51fc
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(5)?))?;
    // VMA 0xb5200
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,1)?)?;
    // VMA 0xb5204
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // VMA 0xb5208
    c.b(C::Hi,0xb521c,false)?;
    // VMA 0xb520c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb5210
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,3632)?)?;
    // VMA 0xb5214
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb5218
    c.b(C::Ne,0xb5238,false)?;
    // VMA 0xb521c
    c.literal(R::R1,0xb60b0)?;
    // VMA 0xb5220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb5224
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5228
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb522c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb5230
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5234
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2444)?)?;
    // VMA 0xb5238
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2452)?)?;
    // VMA 0xb523c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5240
    c.emit(arm32::transfer_word(C::Ne,true,R::new(5)?,R::new(4)?,2444)?)?;
    // VMA 0xb5244
    c.emit(arm32::data_register(C::Ne,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb5248
    c.b(C::Eq,0xb5274,false)?;
    // VMA 0xb524c
    c.literal(R::R1,0xb60b4)?;
    // VMA 0xb5250
    c.literal(R::R2,0xb60b8)?;
    // VMA 0xb5254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,141&0xffffffff)?)?;
    // VMA 0xb5258
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb525c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xb5260
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5264
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb5268
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb526c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb5270
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xb5274
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,644)?)?;
    // VMA 0xb5278
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb527c
    c.b(C::Eq,0xb5290,false)?;
    // VMA 0xb5280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,19&0xffffffff)?)?;
    // VMA 0xb5284
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // VMA 0xb5288
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb528c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb5290
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5294
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,2448)?)?;
    // VMA 0xb5298
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2440)?)?;
    // VMA 0xb529c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2601)?)?;
    // VMA 0xb52a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2604)?)?;
    // VMA 0xb52a4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2598)?)?;
    // VMA 0xb52a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // VMA 0xb52ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb52b0
    c.b(C::Eq,0xb530c,false)?;
    // VMA 0xb52b4
    c.literal(R::R2,0xb60bc)?;
    // VMA 0xb52b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,193&0xffffffff)?)?;
    // VMA 0xb52bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb52c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb52c4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb52c8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb52cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,42)?)?;
    // VMA 0xb52d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb52d4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xb52d8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xb52dc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xb52e0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xb52e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb52e8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xb52ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xb52f0
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2608)?)?;
    // VMA 0xb52f4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // VMA 0xb52f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xb52fc
    c.b(C::Cc,0xb5468,false)?;
    // VMA 0xb5300
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2608)?)?;
    // VMA 0xb5304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,57&0xffffffff)?)?;
    // VMA 0xb5308
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb530c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // VMA 0xb5310
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // VMA 0xb5314
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5318
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,18)?)?;
    // VMA 0xb531c
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // VMA 0xb5320
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5324
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5328
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb532c
    c.emit(arm32::transfer_word_register_offset(C::Ne,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5330
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106&0xffffffff)?)?;
    // VMA 0xb5334
    c.emit(arm32::transfer_word_register_offset(C::Ne,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5338
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xb533c
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,2601)?)?;
    // VMA 0xb5340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,616&0xffffffff)?)?;
    // VMA 0xb5344
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::R2,R::R0)?)?;
    // VMA 0xb5348
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::R2,R::R1)?)?;
    // VMA 0xb534c
    c.literal(R::R2,0xb60bc)?;
    // VMA 0xb5350
    c.emit(arm32::multiply_register(C::Al,R::new(8)?,R::R1,R::R0)?)?;
    // VMA 0xb5354
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,173&0xffffffff)?)?;
    // VMA 0xb5358
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb535c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    // VMA 0xb5360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb5364
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5368
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb536c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,1)?)?;
    // VMA 0xb5370
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31&0xffffffff)?)?;
    // VMA 0xb5374
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5378
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R0,R::R2,Shift::Lsl,R::R1,false)?)?;
    // VMA 0xb537c
    c.b(C::Ne,0xb5390,false)?;
    // VMA 0xb5380
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // VMA 0xb5384
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // VMA 0xb5388
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,32)?)?;
    // VMA 0xb538c
    c.b(C::Cc,0xb5378,false)?;
    // VMA 0xb5390
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::new(9)?,32)?)?;
    // VMA 0xb5394
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb5398
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xb539c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb53a0
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb53a4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb53a8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2604)?)?;
    // VMA 0xb53ac
    c.b(C::Cs,0xb5584,false)?;
    // VMA 0xb53b0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2601)?)?;
    // VMA 0xb53b4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(6)?))?;
    // VMA 0xb53b8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // VMA 0xb53bc
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,24)?)?;
    // VMA 0xb53c0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(5)?))?;
    // VMA 0xb53c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb53c8
    c.b(C::Eq,0xb52b4,false)?;
    // VMA 0xb53cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,19)?)?;
    // VMA 0xb53d0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(6)?))?;
    // VMA 0xb53d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb53d8
    c.b(C::Ne,0xb53e8,false)?;
    // VMA 0xb53dc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,20)?)?;
    // VMA 0xb53e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb53e4
    c.b(C::Eq,0xb5428,false)?;
    // VMA 0xb53e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb53ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb53f0
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb53f4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb53f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb53fc
    c.b(C::Lt,0xb5428,false)?;
    // VMA 0xb5400
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5404
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5408
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb540c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb5410
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,255)?)?;
    // VMA 0xb5414
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5418
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb541c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(6)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5420
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5424
    c.b(C::Ne,0xb5414,false)?;
    // VMA 0xb5428
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,43)?)?;
    // VMA 0xb542c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5430
    c.b(C::Eq,0xb5644,false)?;
    // VMA 0xb5434
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb5438
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb543c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5440
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb5444
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    // VMA 0xb5448
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2740)?)?;
    // VMA 0xb544c
    c.b(C::Ls,0xb5618,false)?;
    // VMA 0xb5450
    c.literal(R::R1,0xb60d8)?;
    // VMA 0xb5454
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb5458
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb545c
    c.literal(R::R1,0xb60a8)?;
    // VMA 0xb5460
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb5464
    c.b(C::Al,0xb6008,false)?;
    // VMA 0xb5468
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // VMA 0xb546c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5470
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb5474
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,2&0xffffffff)?)?;
    // VMA 0xb5478
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,2608)?)?;
    // VMA 0xb547c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5480
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2609)?)?;
    // VMA 0xb5484
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,16)?)?;
    // VMA 0xb5488
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,24)?)?;
    // VMA 0xb548c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb5490
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb5494
    c.emit(arm32::transfer_word_register_offset(C::Ne,false,R::R0,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5498
    c.emit(arm32::transfer_word_register_offset(C::Ne,true,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb549c
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,2609)?)?;
    // VMA 0xb54a0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,2612)?)?;
    // VMA 0xb54a4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2686)?)?;
    // VMA 0xb54a8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2593)?)?;
    // VMA 0xb54ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb54b0
    c.b(C::Eq,0xb5594,false)?;
    // VMA 0xb54b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,636&0xffffffff)?)?;
    // VMA 0xb54b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb54bc
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::new(5)?,R::new(4)?,R::R0)?)?;
    // VMA 0xb54c0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,178)?)?;
    // VMA 0xb54c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb54c8
    c.b(C::Eq,0xb54f4,false)?;
    // VMA 0xb54cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb54d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1&0xffffffff)?)?;
    // VMA 0xb54d4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xb54d8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xb54dc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xb54e0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xb54e4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xb54e8
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2684)?)?;
    // VMA 0xb54ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // VMA 0xb54f0
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2685)?)?;
    // VMA 0xb54f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb54f8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb54fc
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // VMA 0xb5500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,2&0xffffffff)?)?;
    // VMA 0xb5504
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(5)?,R::R0,2687)?)?;
    // VMA 0xb5508
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R0,3)?)?;
    // VMA 0xb550c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R0,2)?)?;
    // VMA 0xb5510
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R0,1)?)?;
    // VMA 0xb5514
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,-79)?)?;
    // VMA 0xb5518
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xb551c
    c.b(C::Eq,0xb58ec,false)?;
    // VMA 0xb5520
    c.literal(R::R0,0xb60c8)?;
    // VMA 0xb5524
    c.emit(arm32::mov_register(C::Al,R::R1,R::LR))?;
    // VMA 0xb5528
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // VMA 0xb552c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xb5530
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb5534
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(3)?)?)?;
    // VMA 0xb5538
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // VMA 0xb553c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb5540
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb5544
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5548
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb554c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb5550
    c.b(C::Eq,0xb55f8,false)?;
    // VMA 0xb5554
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb5558
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb555c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb5560
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb5564
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2687)?)?;
    // VMA 0xb5568
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,240)?)?;
    // VMA 0xb556c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // VMA 0xb5570
    c.b(C::Cc,0xb5708,false)?;
    // VMA 0xb5574
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb5578
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2687)?)?;
    // VMA 0xb557c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,68&0xffffffff)?)?;
    // VMA 0xb5580
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5584
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2604)?)?;
    // VMA 0xb5588
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,48&0xffffffff)?)?;
    // VMA 0xb558c
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(5)?))?;
    // VMA 0xb5590
    c.b(C::Al,0xb5164,false)?;
    // VMA 0xb5594
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::LR,115)?)?;
    // VMA 0xb5598
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xb559c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb55a0
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb55a4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb55a8
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,R::R1,false)?)?;
    // VMA 0xb55ac
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,2612)?)?;
    // VMA 0xb55b0
    c.b(C::Cc,0xb55c4,false)?;
    // VMA 0xb55b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // VMA 0xb55b8
    c.b(C::Eq,0xb598c,false)?;
    // VMA 0xb55bc
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R2,Shift::Lsl,R::R1,false)?)?;
    // VMA 0xb55c0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2612)?)?;
    // VMA 0xb55c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb55c8
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb55cc
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2616)?)?;
    // VMA 0xb55d0
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb55d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb55d8
    c.b(C::Eq,0xb5868,false)?;
    // VMA 0xb55dc
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::LR,180)?)?;
    // VMA 0xb55e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // VMA 0xb55e4
    c.b(C::Eq,0xb59e0,false)?;
    // VMA 0xb55e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // VMA 0xb55ec
    c.b(C::Ne,0xb5994,false)?;
    // VMA 0xb55f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb55f4
    c.b(C::Al,0xb589c,false)?;
    // VMA 0xb55f8
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,4)?)?;
    // VMA 0xb55fc
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,2687)?)?;
    c.check(0xb5600)
}
