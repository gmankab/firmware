
/// Update downstream codec stream status and signal completion.
fn update_codec_stream_feedback(c:&mut Code)->Result<(),String>{
    c.check(0xb4e24)?;
    // VMA 0xb4e24
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb4e28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb4e2c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // VMA 0xb4e30
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,445)?)?;
    // VMA 0xb4e34
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,36)?)?;
    // VMA 0xb4e38
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xb4e3c
    c.b(C::Eq,0xb4e68,false)?;
    // VMA 0xb4e40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048&0xffffffff)?)?;
    // VMA 0xb4e44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xb4e48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb4e4c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb4e50
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4e54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb4e58
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb4e5c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb4e60
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(10)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb4e64
    c.b(C::Al,0xb4e6c,false)?;
    // VMA 0xb4e68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4e6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048&0xffffffff)?)?;
    // VMA 0xb4e70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xb4e74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb4e78
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xb4e7c
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsl,15)?)?;
    // VMA 0xb4e80
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::R2,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb4e84
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(5)?,R::new(8)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4e88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4&0xffffffff)?)?;
    // VMA 0xb4e8c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,2048)?)?;
    // VMA 0xb4e90
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(5)?,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xb4e94
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4e98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,31&0xffffffff)?)?;
    // VMA 0xb4e9c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R12,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4ea0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4ea4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xb4ea8
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,R::new(5)?,false)?)?;
    // VMA 0xb4eac
    c.b(C::Ne,0xb4ec0,false)?;
    // VMA 0xb4eb0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb4eb4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xb4eb8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,31)?)?;
    // VMA 0xb4ebc
    c.b(C::Lt,0xb4ea8,false)?;
    // VMA 0xb4ec0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R12,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4ec4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb4ec8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // VMA 0xb4ecc
    c.b(C::Lt,0xb4f3c,false)?;
    // VMA 0xb4ed0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::LR,R::new(7)?,32)?)?;
    // VMA 0xb4ed4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4ed8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xb4edc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4ee0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,16777216&0xffffffff)?)?;
    // VMA 0xb4ee4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R12,R::LR,255)?)?;
    // VMA 0xb4ee8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4eec
    c.b(C::Ge,0xb4f00,false)?;
    // VMA 0xb4ef0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,1)?)?;
    // VMA 0xb4ef4
    c.emit(arm32::transfer_word_register_offset(C::Ge,false,R::R12,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4ef8
    c.emit(arm32::transfer_word_register_offset(C::Ge,true,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4efc
    c.b(C::Al,0xb4f1c,false)?;
    // VMA 0xb4f00
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,115)?)?;
    // VMA 0xb4f04
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xb4f08
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xb4f0c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb4f10
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xb4f14
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xb4f18
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xb4f1c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(5)?,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4f20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb4f24
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4f28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb4f2c
    c.emit(arm32::transfer_word_register_offset(C::Ne,false,R::new(9)?,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4f30
    c.emit(arm32::transfer_word_register_offset(C::Ne,true,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4f34
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4f38
    c.b(C::Ne,0xb4ee8,false)?;
    // VMA 0xb4f3c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // VMA 0xb4f40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xb4f44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,120)?)?;
    // VMA 0xb4f48
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xb4f4c
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4f50
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // VMA 0xb4f54
    c.b(C::Eq,0xb4f98,false)?;
    // VMA 0xb4f58
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,124)?)?;
    // VMA 0xb4f5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192&0xffffffff)?)?;
    // VMA 0xb4f60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,27&0xffffffff)?)?;
    // VMA 0xb4f64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xb4f68
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xb4f6c
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4f70
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb4f74
    c.literal(R::R1,0xb4ff8)?;
    // VMA 0xb4f78
    c.literal(R::R2,0xb4ffc)?;
    // VMA 0xb4f7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb4f80
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4f84
    c.literal(R::R1,0xb5000)?;
    // VMA 0xb4f88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb4f8c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4f90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb4f94
    c.b(C::Al,0xb4f9c,false)?;
    // VMA 0xb4f98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4f9c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,116)?)?;
    // VMA 0xb4fa0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,2048)?)?;
    // VMA 0xb4fa4
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4fa8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,5)?)?;
    // VMA 0xb4fac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // VMA 0xb4fb0
    c.b(C::Eq,0xb4fdc,false)?;
    // VMA 0xb4fb4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xb4fb8
    c.literal(R::R1,0xb5004)?;
    // VMA 0xb4fbc
    c.literal(R::R2,0xb4ffc)?;
    // VMA 0xb4fc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb4fc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,284&0xffffffff)?)?;
    // VMA 0xb4fc8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4fcc
    c.literal(R::R1,0xb5000)?;
    // VMA 0xb4fd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb4fd4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb4fd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255&0xffffffff)?)?;
    // VMA 0xb4fdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xb4fe0
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4fe4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // VMA 0xb4fe8
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb4fec
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb4ff0
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4ff4
    c.b(C::Al,0xb4fe0,false)?;
    c.check(0xb4ff8)
}

const STREAM_FEEDBACK_DIAGNOSTICS:[u32;4]=[
    0x000f_7058,0x000f_8de7,0x000e_2788,0x000f_7095,
];
