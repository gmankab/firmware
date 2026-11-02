
/// Load and store indexed fields from fixed-width codec slot records, with explicit null checks.
fn read_write_slot_fields(c:&mut Code)->Result<(),String>{
    c.check(0xa9120)?;
    // VMA 0xa9120
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa9124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa9128
    c.emit(arm32::multiply_accumulate(C::Al,R::R12,R::R1,R::new(3)?,R::R0)?)?;
    // VMA 0xa912c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,4)?)?;
    // VMA 0xa9130
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,32)?)?;
    // VMA 0xa9134
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa9138
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa913c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa9140
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xa9144
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // VMA 0xa9148
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xa914c
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xa9150
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,0)?)?;
    // VMA 0xa9154
    c.b(C::Eq,0xa9184,false)?;
    // VMA 0xa9158
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa915c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa9160
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa9164
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,32)?)?;
    // VMA 0xa9168
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa916c
    c.b(C::Eq,0xa9188,false)?;
    // VMA 0xa9170
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xa9174
    c.b(C::Al,0x8019c,true)?;
    // VMA 0xa9178
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xa917c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xa9180
    c.b(C::Al,0xa9188,false)?;
    // VMA 0xa9184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xa9188
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa918c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // VMA 0xa9190
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9194
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa9198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa919c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa91a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,20)?)?;
    // VMA 0xa91a4
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa91a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa91ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa91b0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa91b4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,32)?)?;
    // VMA 0xa91b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa91bc
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa91c0
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa91c4
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa91c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa91cc
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa91d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,44)?)?;
    // VMA 0xa91d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xa91d8
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa91dc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa91e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa91e4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xa91e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa91ec
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xa91f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xa91f4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,0)?)?;
    // VMA 0xa91f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa91fc
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa9200
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,28)?)?;
    // VMA 0xa9204
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9208
    c.b(C::Eq,0xa921c,false)?;
    // VMA 0xa920c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xa9210
    c.b(C::Al,0x8019c,true)?;
    // VMA 0xa9214
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xa9218
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xa921c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // VMA 0xa9220
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9224
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa9228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa922c
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa9230
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,64)?)?;
    // VMA 0xa9234
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xa9238
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa923c
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa9240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa9244
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa9248
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    // VMA 0xa924c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xa9250
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa9254
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,260)?)?;
    // VMA 0xa9258
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa925c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa9260
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa9264
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xa9268
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // VMA 0xa926c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,176)?)?;
    // VMA 0xa9270
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,160)?)?;
    // VMA 0xa9274
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // VMA 0xa9278
    c.b(C::Al,0x471c,true)?;
    // VMA 0xa927c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,176)?)?;
    // VMA 0xa9280
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9284
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,4)?)?;
    // VMA 0xa9288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,360)?)?;
    // VMA 0xa928c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xa9290
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa9294
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,44)?)?;
    // VMA 0xa9298
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // VMA 0xa929c
    c.b(C::Eq,0xa92c0,false)?;
    // VMA 0xa92a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xa92a4
    c.b(C::Eq,0xa92b8,false)?;
    // VMA 0xa92a8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xa92ac
    c.emit(arm32::bx(C::Ne,R::LR))?;
    // VMA 0xa92b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xa92b4
    c.b(C::Al,0xa92bc,false)?;
    // VMA 0xa92b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xa92bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa92c0
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa92c4
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa92c8)
}

/// Original repeated diagnostic pointer for indexed slot validation.
const CODEC_SLOT_CHECK_MESSAGE:u32=0x000f_82fd;


/// Derive codec record mappings and populate three bit-packed hardware
/// descriptor fields from per-record indices. Unknown mappings are kept
/// explicitly and do not become zero-filled stub instructions.
fn encode_slot_mapping_vectors(c:&mut Code)->Result<(),String>{
    c.check(0xa92c8)?;
    // VMA 0xa92c8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa92cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xa92d0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xa92d4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xa92d8
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xa92dc
    c.b(C::Eq,0xa950c,false)?;
    // VMA 0xa92e0
    c.literal(R::new(10)?,0xa9514)?;
    // VMA 0xa92e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xa92e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,2880)?)?;
    // VMA 0xa92ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R0,952)?)?;
    // VMA 0xa92f0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa92f4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,0)?)?;
    // VMA 0xa92f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(9)?,16)?)?;
    // VMA 0xa92fc
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xa9300
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xa9304
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,0)?)?;
    // VMA 0xa9308
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xa930c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa9310
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xa9314
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xa9318
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // VMA 0xa931c
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xa9320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // VMA 0xa9324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,15)?)?;
    // VMA 0xa9328
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,13)?)?;
    // VMA 0xa932c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,9)?)?;
    // VMA 0xa9330
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,5)?)?;
    // VMA 0xa9334
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,3)?)?;
    // VMA 0xa9338
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,2)?)?;
    // VMA 0xa933c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,1)?)?;
    // VMA 0xa9340
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xa9344
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,12)?)?;
    // VMA 0xa9348
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,3)?)?;
    // VMA 0xa934c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,2)?)?;
    // VMA 0xa9350
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xa9354
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,8)?)?;
    // VMA 0xa9358
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,3)?)?;
    // VMA 0xa935c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,2)?)?;
    // VMA 0xa9360
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xa9364
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,4)?)?;
    // VMA 0xa9368
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,3)?)?;
    // VMA 0xa936c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,2)?)?;
    // VMA 0xa9370
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,140)?)?;
    // VMA 0xa9374
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // VMA 0xa9378
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(8)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa937c
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xa9380
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // VMA 0xa9384
    c.b(C::Eq,0xa93f4,false)?;
    // VMA 0xa9388
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(8)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa938c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9390
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::new(3)?,112)?)?;
    // VMA 0xa9394
    c.emit(arm32::load_signed_byte(C::Al,R::new(7)?,R::new(7)?,112)?)?;
    // VMA 0xa9398
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xa939c
    c.b(C::Lt,0xa93b0,false)?;
    // VMA 0xa93a0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::new(3)?,Shift::Lsl,2)?)?;
    // VMA 0xa93a4
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(8)?,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xa93a8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa93ac
    c.b(C::Eq,0xa93e0,false)?;
    // VMA 0xa93b0
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::LR,Shift::Lsl,24)?)?;
    // VMA 0xa93b4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(3)?,Shift::Asr,24)?)?;
    // VMA 0xa93b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // VMA 0xa93bc
    c.b(C::Gt,0xa93e4,false)?;
    // VMA 0xa93c0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(8)?,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xa93c4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::R12,R::new(3)?,Shift::Asr,24,false)?)?;
    // VMA 0xa93c8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(8)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa93cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::LR,1)?)?;
    // VMA 0xa93d0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,2)?)?;
    // VMA 0xa93d4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa93d8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,168)?)?;
    // VMA 0xa93dc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,168)?)?;
    // VMA 0xa93e0
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xa93e4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R2,140)?)?;
    // VMA 0xa93e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // VMA 0xa93ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,27)?)?;
    // VMA 0xa93f0
    c.b(C::Lt,0xa9374,false)?;
    // VMA 0xa93f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xa93f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa93fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa9400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa9404
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,280)?)?;
    // VMA 0xa9408
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,284)?)?;
    // VMA 0xa940c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,288)?)?;
    // VMA 0xa9410
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(7)?,R::new(5)?,1)?)?;
    // VMA 0xa9414
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xa9418
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,255)?)?;
    // VMA 0xa941c
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(7)?))?;
    // VMA 0xa9420
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,6)?)?;
    // VMA 0xa9424
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R1,15)?)?;
    // VMA 0xa9428
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,R::R2,false)?)?;
    // VMA 0xa942c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,5)?)?;
    // VMA 0xa9430
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,280)?)?;
    // VMA 0xa9434
    c.b(C::Ne,0xa9410,false)?;
    // VMA 0xa9438
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,22)?)?;
    // VMA 0xa943c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // VMA 0xa9440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xa9444
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa9448
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(5)?,R::R2,1)?)?;
    // VMA 0xa944c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xa9450
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,255)?)?;
    // VMA 0xa9454
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(5)?))?;
    // VMA 0xa9458
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,12)?)?;
    // VMA 0xa945c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,15)?)?;
    // VMA 0xa9460
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,R::new(7)?,false)?)?;
    // VMA 0xa9464
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,5)?)?;
    // VMA 0xa9468
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,284)?)?;
    // VMA 0xa946c
    c.b(C::Ne,0xa9448,false)?;
    // VMA 0xa9470
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,28)?)?;
    // VMA 0xa9474
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,12)?)?;
    // VMA 0xa9478
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xa947c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa9480
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(5)?,R::R2,1)?)?;
    // VMA 0xa9484
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xa9488
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,255)?)?;
    // VMA 0xa948c
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(5)?))?;
    // VMA 0xa9490
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // VMA 0xa9494
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,15)?)?;
    // VMA 0xa9498
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,R::new(7)?,false)?)?;
    // VMA 0xa949c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,5)?)?;
    // VMA 0xa94a0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,288)?)?;
    // VMA 0xa94a4
    c.b(C::Ne,0xa9480,false)?;
    // VMA 0xa94a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,124)?)?;
    // VMA 0xa94ac
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,0)?)?;
    // VMA 0xa94b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa94b4
    c.b(C::Eq,0xa950c,false)?;
    // VMA 0xa94b8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,132)?)?;
    // VMA 0xa94bc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,1160)?)?;
    // VMA 0xa94c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa94c4
    c.b(C::Al,0xa94d0,false)?;
    // VMA 0xa94c8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // VMA 0xa94cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xa94d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xa94d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,284)?)?;
    // VMA 0xa94d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // VMA 0xa94dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,288)?)?;
    // VMA 0xa94e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,8)?)?;
    // VMA 0xa94e4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,64)?)?;
    // VMA 0xa94e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,4)?)?;
    // VMA 0xa94ec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa94f0
    c.emit(arm32::transfer_word(C::Le,true,R::R0,R::R2,16)?)?;
    // VMA 0xa94f4
    c.emit(arm32::data_immediate(C::Le,Op::Bic,R::R0,R::R0,3)?)?;
    // VMA 0xa94f8
    c.emit(arm32::data_register(C::Le,Op::Sub,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa94fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,124)?)?;
    // VMA 0xa9500
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9504
    c.b(C::Cc,0xa94c8,false)?;
    // VMA 0xa9508
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,132)?)?;
    // VMA 0xa950c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xa9510
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa9514)
}

/// Global controller slot-table handle used by a single original PC load.
const SLOT_MANAGER_GLOBAL:u32=0x0010_dd64;

/// Publish the calculated slot pointer and copy command completion state.
/// The small helpers return through either original BX LR or stack epilogue.
fn resolve_codec_slot_hardware(c:&mut Code)->Result<(),String>{
    c.check(0xa9518)?;
    // VMA 0xa9518
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,60)?)?;
    // VMA 0xa951c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1092)?)?;
    // VMA 0xa9520
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // VMA 0xa9524
    c.b(C::Al,0x7ffc8,false)?;
    // VMA 0xa9528
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,60)?)?;
    // VMA 0xa952c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1092)?)?;
    // VMA 0xa9530
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa9534
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa9538
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xa953c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xa9540
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xa9544
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,1092)?)?;
    // VMA 0xa9548
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa954c
    c.b(C::Eq,0xa9560,false)?;
    // VMA 0xa9550
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xa9554
    c.b(C::Al,0x8019c,true)?;
    // VMA 0xa9558
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xa955c
    c.b(C::Al,0xa9568,false)?;
    // VMA 0xa9560
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,1124)?)?;
    // VMA 0xa9564
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xa9568
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // VMA 0xa956c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9570
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,1092)?)?;
    // VMA 0xa9574
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa9578)
}
