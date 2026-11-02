
/// Release/shift completed codec ring entries and update the head index.
fn drain_controller_frame_slots(c:&mut Code)->Result<(),String>{
    c.check(0xb40ac)?;
    // VMA 0xb40ac
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb40b0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb40b4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xb40b8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xb40bc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,248)?)?;
    // VMA 0xb40c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb40c4
    c.b(C::Lt,0xb410c,false)?;
    // VMA 0xb40c8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xb40cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,84&0xffffffff)?)?;
    // VMA 0xb40d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb40d4
    c.emit(arm32::transfer_word_postindexed_scaled_register(C::Al,true,R::R0,R::R1,R::R2,Shift::Lsl,2)?)?;
    // VMA 0xb40d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb40dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,176)?)?;
    // VMA 0xb40e0
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // VMA 0xb40e4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // VMA 0xb40e8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,4)?)?;
    // VMA 0xb40ec
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(6)?,R::R12,R::R0)?)?;
    // VMA 0xb40f0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(5)?))?;
    // VMA 0xb40f4
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xb40f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb40fc
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xb4100
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(7)?,R::new(5)?,5)?)?;
    // VMA 0xb4104
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb4108
    c.b(C::Gt,0xb40e0,false)?;
    // VMA 0xb410c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb4110
    c.b(C::Al,0xb41c4,true)?;
    // VMA 0xb4114
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // VMA 0xb4118
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xb411c
    c.b(C::Lt,0xb41b8,false)?;
    // VMA 0xb4120
    c.literal(R::new(10)?,0xb41c0)?;
    // VMA 0xb4124
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb4128
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,1)?)?;
    // VMA 0xb412c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3&0xffffffff)?)?;
    // VMA 0xb4134
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,176)?)?;
    // VMA 0xb4138
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb413c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // VMA 0xb4140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84&0xffffffff)?)?;
    // VMA 0xb4144
    c.emit(arm32::multiply_accumulate(C::Al,R::new(8)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xb4148
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,4)?)?;
    // VMA 0xb414c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb4150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4154
    c.b(C::Ne,0xb4168,false)?;
    // VMA 0xb4158
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,1)?)?;
    // VMA 0xb415c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb4160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4164
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,77&0xffffffff)?)?;
    // VMA 0xb416c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // VMA 0xb4170
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb4174
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,252)?)?;
    // VMA 0xb4178
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb417c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // VMA 0xb4180
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R0,0)?)?;
    // VMA 0xb4184
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4188
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(6)?,-4)?)?;
    // VMA 0xb418c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1)?)?;
    // VMA 0xb4190
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,252)?)?;
    // VMA 0xb4194
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R0,0)?)?;
    // VMA 0xb4198
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1)?)?;
    // VMA 0xb419c
    c.emit(arm32::transfer_byte_postindexed_register(C::Al,false,R::R2,R::R0,R::R1)?)?;
    // VMA 0xb41a0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R0,18)?)?;
    // VMA 0xb41a4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(8)?,1)?)?;
    // VMA 0xb41a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // VMA 0xb41ac
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xb41b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,248)?)?;
    // VMA 0xb41b4
    c.b(C::Gt,0xb4138,false)?;
    // VMA 0xb41b8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb41bc
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb41c0)
}

const RESET_RECORD_WARNING:u32=0x000f_8c8d;

/// Remove completed records from the controller indexed ring.
fn compact_controller_frame_records(c:&mut Code)->Result<(),String>{
    c.check(0xb41c4)?;
    // VMA 0xb41c4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb41c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xb41cc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // VMA 0xb41d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb41d4
    c.emit(arm32::transfer_multiple(C::Lt,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb41d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,360&0xffffffff)?)?;
    // VMA 0xb41dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0&0xffffffff)?)?;
    // VMA 0xb41e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb41e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb41e8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,2048)?)?;
    // VMA 0xb41ec
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb41f0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,0)?)?;
    // VMA 0xb41f4
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(4)?,R::new(5)?,180)?)?;
    // VMA 0xb41f8
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(4)?,R::new(8)?,R::new(7)?)?)?;
    // VMA 0xb41fc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,4)?)?;
    // VMA 0xb4200
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb4204
    c.b(C::Ne,0xb42c0,false)?;
    // VMA 0xb4208
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R0,4)?)?;
    // VMA 0xb420c
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,1)?)?;
    // VMA 0xb4210
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // VMA 0xb4214
    c.b(C::Ne,0xb42c0,false)?;
    // VMA 0xb4218
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::new(6)?,8)?)?;
    // VMA 0xb421c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xb4220
    c.b(C::Eq,0xb4234,false)?;
    // VMA 0xb4224
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(4)?,R::new(8)?)?)?;
    // VMA 0xb4228
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::LR,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb422c
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(6)?,0)?)?;
    // VMA 0xb4230
    c.b(C::Al,0xb4288,false)?;
    // VMA 0xb4234
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(4)?,R::R12,R::new(9)?)?)?;
    // VMA 0xb4238
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1896)?)?;
    // VMA 0xb423c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb4240
    c.b(C::Eq,0xb426c,false)?;
    // VMA 0xb4244
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,252)?)?;
    // VMA 0xb4248
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,0)?)?;
    // VMA 0xb424c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4250
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::R1,1)?)?;
    // VMA 0xb4254
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,252)?)?;
    // VMA 0xb4258
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,0)?)?;
    // VMA 0xb425c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R2,1)?)?;
    // VMA 0xb4260
    c.emit(arm32::transfer_byte_postindexed_register(C::Al,false,R::new(4)?,R::R1,R::R2)?)?;
    // VMA 0xb4264
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::R1,18)?)?;
    // VMA 0xb4268
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,0)?)?;
    // VMA 0xb426c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,252)?)?;
    // VMA 0xb4270
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R1,35)?)?;
    // VMA 0xb4274
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,1)?)?;
    // VMA 0xb4278
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R1,35)?)?;
    // VMA 0xb427c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4280
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::R1,36)?)?;
    // VMA 0xb4284
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // VMA 0xb4288
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // VMA 0xb428c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4290
    c.b(C::Ge,0xb42b8,false)?;
    // VMA 0xb4294
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // VMA 0xb4298
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(4)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb429c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xb42a0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,184)?)?;
    // VMA 0xb42a4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,180)?)?;
    // VMA 0xb42a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // VMA 0xb42ac
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // VMA 0xb42b0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb42b4
    c.b(C::Lt,0xb4298,false)?;
    // VMA 0xb42b8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb42bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,248)?)?;
    // VMA 0xb42c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb42c4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb42c8
    c.b(C::Lt,0xb41ec,false)?;
    // VMA 0xb42cc
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb42d0)
}

/// Continue conditional record updates and refresh preserved slot indices.
fn update_remaining_codec_records(c:&mut Code)->Result<(),String>{
    c.check(0xb42d0)?;
    // VMA 0xb42d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,22)?)?;
    // VMA 0xb42d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb42d8
    c.b(C::Eq,0xb43c4,false)?;
    // VMA 0xb42dc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb42e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb42e4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,248)?)?;
    // VMA 0xb42e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb42ec
    c.b(C::Lt,0xb43b8,false)?;
    // VMA 0xb42f0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xb42f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,360&0xffffffff)?)?;
    // VMA 0xb42f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xb42fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,84&0xffffffff)?)?;
    // VMA 0xb4300
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3&0xffffffff)?)?;
    // VMA 0xb4304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4308
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb430c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,176)?)?;
    // VMA 0xb4310
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,2048)?)?;
    // VMA 0xb4314
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,0)?)?;
    // VMA 0xb4318
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // VMA 0xb431c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,4)?)?;
    // VMA 0xb4320
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::new(7)?,R::new(9)?,R::new(6)?)?)?;
    // VMA 0xb4324
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // VMA 0xb4328
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::new(5)?,R::new(6)?,1)?)?;
    // VMA 0xb432c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb4330
    c.b(C::Eq,0xb4344,false)?;
    // VMA 0xb4334
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(7)?,R::LR,R::new(4)?)?)?;
    // VMA 0xb4338
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,1896)?)?;
    // VMA 0xb433c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb4340
    c.b(C::Eq,0xb4384,false)?;
    // VMA 0xb4344
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,0)?)?;
    // VMA 0xb4348
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb434c
    c.b(C::Ne,0xb4360,false)?;
    // VMA 0xb4350
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(7)?,R::LR,R::new(4)?)?)?;
    // VMA 0xb4354
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(6)?,0)?)?;
    // VMA 0xb4358
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,1900)?)?;
    // VMA 0xb435c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // VMA 0xb4360
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,252)?)?;
    // VMA 0xb4364
    c.emit(arm32::load_signed_byte(C::Al,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xb4368
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb436c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1)?)?;
    // VMA 0xb4370
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,252)?)?;
    // VMA 0xb4374
    c.emit(arm32::load_signed_byte(C::Al,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xb4378
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(5)?,1)?)?;
    // VMA 0xb437c
    c.emit(arm32::transfer_byte_postindexed_register(C::Al,false,R::new(6)?,R::new(4)?,R::new(5)?)?)?;
    // VMA 0xb4380
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(4)?,18)?)?;
    // VMA 0xb4384
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(3)?,4)?)?;
    // VMA 0xb4388
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(3)?,5)?)?;
    // VMA 0xb438c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,12)?)?;
    // VMA 0xb4390
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // VMA 0xb4394
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,252)?)?;
    // VMA 0xb4398
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R1,-4)?)?;
    // VMA 0xb439c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb43a0
    c.emit(arm32::load_signed_byte(C::Al,R::new(5)?,R::new(4)?,35)?)?;
    // VMA 0xb43a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(5)?,1)?)?;
    // VMA 0xb43a8
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,35)?)?;
    // VMA 0xb43ac
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb43b0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,36)?)?;
    // VMA 0xb43b4
    c.b(C::Gt,0xb4314,false)?;
    // VMA 0xb43b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb43bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,248)?)?;
    // VMA 0xb43c0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb43c4
    c.b(C::Al,0xb40ac,false)?;
    c.check(0xb43c8)
}

/// Insert a new sorted codec record, preserving size and status ordering.
fn insert_codec_frame_record(c:&mut Code)->Result<(),String>{
    c.check(0xb43c8)?;
    // VMA 0xb43c8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb43cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xb43d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,12)?)?;
    // VMA 0xb43d4
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xb43d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,84&0xffffffff)?)?;
    // VMA 0xb43dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3&0xffffffff)?)?;
    // VMA 0xb43e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb43e4
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R2,R::LR,R::R12)?)?;
    // VMA 0xb43e8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb43ec
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(3)?,1)?)?;
    // VMA 0xb43f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // VMA 0xb43f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb43f8
    c.b(C::Lt,0xb4440,false)?;
    // VMA 0xb43fc
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(3)?,12)?)?;
    // VMA 0xb4400
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,180)?)?;
    // VMA 0xb4404
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4408
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(7)?,R::new(5)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb440c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(7)?,R::LR,R::R12)?)?;
    // VMA 0xb4410
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,12)?)?;
    // VMA 0xb4414
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4418
    c.b(C::Le,0xb4444,false)?;
    // VMA 0xb441c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,1)?)?;
    // VMA 0xb4420
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb4424
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb4428
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(6)?,36)?)?;
    // VMA 0xb442c
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb4430
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(6)?,36)?)?;
    // VMA 0xb4434
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4438
    c.b(C::Lt,0xb4408,false)?;
    // VMA 0xb443c
    c.b(C::Al,0xb4444,false)?;
    // VMA 0xb4440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4444
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4448
    c.b(C::Le,0xb4470,false)?;
    // VMA 0xb444c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xb4450
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,180)?)?;
    // VMA 0xb4454
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(6)?))?;
    // VMA 0xb4458
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(7)?,R::new(5)?,-4)?)?;
    // VMA 0xb445c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // VMA 0xb4460
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4464
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xb4468
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(5)?))?;
    // VMA 0xb446c
    c.b(C::Gt,0xb4458,false)?;
    // VMA 0xb4470
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb4474
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,180)?)?;
    // VMA 0xb4478
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // VMA 0xb447c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb4480
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,248)?)?;
    // VMA 0xb4484
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb4488)
}
