
const CODEC_FRAME_QUEUE_DIAGNOSTICS:[u32;5]=[
    0x000f_8c24,0x000e_2788,0x000f_8be8,0x000f_8c4b,0x000f_8c72,
];

/// Collapse previous output stream state into the active per-frame status
/// arrays and publish the resulting controller field values.
fn collapse_buffer_response_slots(c:&mut Code)->Result<(),String>{
    c.check(0xb3bdc)?;
    // VMA 0xb3bdc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb3be0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xb3be4
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,8)?)?;
    // VMA 0xb3be8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb3bec
    c.b(C::Eq,0xb3c00,false)?;
    // VMA 0xb3bf0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3bf4
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,272)?)?;
    // VMA 0xb3bf8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,268)?)?;
    // VMA 0xb3bfc
    c.b(C::Al,0xb3cfc,false)?;
    // VMA 0xb3c00
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,248)?)?;
    // VMA 0xb3c04
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3c08
    c.b(C::Lt,0xb3c40,false)?;
    // VMA 0xb3c0c
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xb3c10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3c14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,84&0xffffffff)?)?;
    // VMA 0xb3c18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3c1c
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R2,180)?)?;
    // VMA 0xb3c20
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(5)?,R::R2,R::new(4)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3c24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xb3c28
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3c2c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(5)?,R::LR,R::new(3)?)?)?;
    // VMA 0xb3c30
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xb3c34
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(6)?,3)?)?;
    // VMA 0xb3c38
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(6)?,4)?)?;
    // VMA 0xb3c3c
    c.b(C::Lt,0xb3c20,false)?;
    // VMA 0xb3c40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,268&0xffffffff)?)?;
    // VMA 0xb3c44
    c.emit(arm32::load_signed_byte_register(C::Al,R::R1,R::R0,R::R2)?)?;
    // VMA 0xb3c48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3c4c
    c.b(C::Lt,0xb3c78,false)?;
    // VMA 0xb3c50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,596)?)?;
    // VMA 0xb3c54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3c58
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xb3c5c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xb3c60
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(3)?,-436)?)?;
    // VMA 0xb3c64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb3c68
    c.emit(arm32::load_signed_byte_register(C::Al,R::new(6)?,R::R0,R::R2)?)?;
    // VMA 0xb3c6c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3c70
    c.b(C::Lt,0xb3c58,false)?;
    // VMA 0xb3c74
    c.b(C::Al,0xb3c7c,false)?;
    // VMA 0xb3c78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3c7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,13&0xffffffff)?)?;
    // VMA 0xb3c80
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,256)?)?;
    // VMA 0xb3c84
    c.emit(arm32::load_signed_byte_register(C::Al,R::R2,R::R0,R::R12)?)?;
    // VMA 0xb3c88
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb3c8c
    c.b(C::Lt,0xb3cbc,false)?;
    // VMA 0xb3c90
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3c94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3c98
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3c9c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3ca0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb3ca4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(5)?,612)?)?;
    // VMA 0xb3ca8
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(6)?,160)?)?;
    // VMA 0xb3cac
    c.emit(arm32::load_signed_byte_register(C::Al,R::new(6)?,R::R0,R::R12)?)?;
    // VMA 0xb3cb0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3cb4
    c.b(C::Lt,0xb3c98,false)?;
    // VMA 0xb3cb8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3cbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,15&0xffffffff)?)?;
    // VMA 0xb3cc0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,256)?)?;
    // VMA 0xb3cc4
    c.emit(arm32::load_signed_byte_register(C::Al,R::R2,R::R0,R::R12)?)?;
    // VMA 0xb3cc8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xb3ccc
    c.b(C::Lt,0xb3cfc,false)?;
    // VMA 0xb3cd0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3cd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3cd8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3cdc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3ce0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb3ce4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(5)?,644)?)?;
    // VMA 0xb3ce8
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(6)?,160)?)?;
    // VMA 0xb3cec
    c.emit(arm32::load_signed_byte_register(C::Al,R::new(6)?,R::R0,R::R12)?)?;
    // VMA 0xb3cf0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3cf4
    c.b(C::Lt,0xb3cd8,false)?;
    // VMA 0xb3cf8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3cfc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,176)?)?;
    // VMA 0xb3d00
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb3d04)
}

/// Initialize the next live output descriptor and synchronize both
/// controller slot arrays, including missing-slot reporting.
fn initialize_live_codec_buffers(c:&mut Code)->Result<(),String>{
    c.check(0xb3d04)?;
    // VMA 0xb3d04
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb3d08
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb3d0c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // VMA 0xb3d10
    c.literal(R::new(7)?,0xb40a0)?;
    // VMA 0xb3d14
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xb3d18
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // VMA 0xb3d1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3d20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,17&0xffffffff)?)?;
    // VMA 0xb3d24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3d28
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xb3d2c
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xb3d30
    c.literal(R::new(6)?,0xb40a4)?;
    // VMA 0xb3d34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3d38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,17&0xffffffff)?)?;
    // VMA 0xb3d3c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xb3d40
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xb3d44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13&0xffffffff)?)?;
    // VMA 0xb3d48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15&0xffffffff)?)?;
    // VMA 0xb3d4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,268&0xffffffff)?)?;
    // VMA 0xb3d50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb3d54
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // VMA 0xb3d58
    c.emit(arm32::load_signed_byte_register(C::Al,R::R2,R::new(9)?,R::R2)?)?;
    // VMA 0xb3d5c
    c.emit(arm32::load_signed_byte_register(C::Al,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xb3d60
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xb3d64
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(9)?,152)?)?;
    // VMA 0xb3d68
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(9)?,156)?)?;
    // VMA 0xb3d6c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb3d70
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R1,R::R0,Shift::Lsl,0,true)?)?;
    // VMA 0xb3d74
    c.b(C::Eq,0xb4098,false)?;
    // VMA 0xb3d78
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,25)?)?;
    // VMA 0xb3d7c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // VMA 0xb3d80
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,248)?)?;
    // VMA 0xb3d84
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3d88
    c.emit(arm32::mov_register(C::Ge,R::new(5)?,R::new(8)?))?;
    // VMA 0xb3d8c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3d90
    c.emit(arm32::mov_register(C::Lt,R::new(8)?,R::R0))?;
    // VMA 0xb3d94
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3d98
    c.b(C::Lt,0xb407c,false)?;
    // VMA 0xb3d9c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,11)?)?;
    // VMA 0xb3da0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xb3da4
    c.b(C::Eq,0xb407c,false)?;
    // VMA 0xb3da8
    c.literal(R::R2,0xb40a8)?;
    // VMA 0xb3dac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,238&0xffffffff)?)?;
    // VMA 0xb3db0
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // VMA 0xb3db4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,152)?)?;
    // VMA 0xb3db8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,18)?)?;
    // VMA 0xb3dbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3dc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3dc4
    c.emit(arm32::data_immediate(C::Lt,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb3dc8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb3dcc
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb3dd0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    // VMA 0xb3dd4
    c.b(C::Lt,0xb3ec8,false)?;
    // VMA 0xb3dd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,268&0xffffffff)?)?;
    // VMA 0xb3ddc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,644)?)?;
    // VMA 0xb3de0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(9)?,596)?)?;
    // VMA 0xb3de4
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xb3de8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // VMA 0xb3dec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,612)?)?;
    // VMA 0xb3df0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // VMA 0xb3df4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3df8
    c.b(C::Ge,0xb3e30,false)?;
    // VMA 0xb3dfc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xb3e00
    c.b(C::Lt,0xb3e30,false)?;
    // VMA 0xb3e04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb3e08
    c.emit(arm32::mov_register(C::Al,R::R1,R::LR))?;
    // VMA 0xb3e0c
    c.emit(arm32::load_signed_byte_postindexed(C::Al,R::new(3)?,R::R1,1)?)?;
    // VMA 0xb3e10
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::new(7)?,R::new(10)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3e14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xb3e18
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3e1c
    c.b(C::Ge,0xb3e30,false)?;
    // VMA 0xb3e20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // VMA 0xb3e24
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3e28
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // VMA 0xb3e2c
    c.b(C::Lt,0xb3e0c,false)?;
    // VMA 0xb3e30
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3e34
    c.b(C::Ge,0xb3e78,false)?;
    // VMA 0xb3e38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13&0xffffffff)?)?;
    // VMA 0xb3e3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb3e40
    c.emit(arm32::load_signed_byte_register(C::Al,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xb3e44
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3e48
    c.b(C::Lt,0xb3e78,false)?;
    // VMA 0xb3e4c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,4)?)?;
    // VMA 0xb3e50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1&0xffffffff)?)?;
    // VMA 0xb3e54
    c.emit(arm32::load_signed_byte_postindexed(C::Al,R::new(3)?,R::R2,1)?)?;
    // VMA 0xb3e58
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::new(7)?,R::new(10)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3e5c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xb3e60
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3e64
    c.b(C::Ge,0xb3e78,false)?;
    // VMA 0xb3e68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R12,1)?)?;
    // VMA 0xb3e6c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3e70
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(3)?))?;
    // VMA 0xb3e74
    c.b(C::Lt,0xb3e54,false)?;
    // VMA 0xb3e78
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3e7c
    c.b(C::Ge,0xb3ec0,false)?;
    // VMA 0xb3e80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15&0xffffffff)?)?;
    // VMA 0xb3e84
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb3e88
    c.emit(arm32::load_signed_byte_register(C::Al,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xb3e8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3e90
    c.b(C::Lt,0xb3ec0,false)?;
    // VMA 0xb3e94
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // VMA 0xb3e98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb3e9c
    c.emit(arm32::load_signed_byte_postindexed(C::Al,R::new(4)?,R::R2,1)?)?;
    // VMA 0xb3ea0
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(4)?,R::new(7)?,R::new(10)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3ea4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xb3ea8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3eac
    c.b(C::Ge,0xb3ec0,false)?;
    // VMA 0xb3eb0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(3)?,1)?)?;
    // VMA 0xb3eb4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3eb8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // VMA 0xb3ebc
    c.b(C::Lt,0xb3e9c,false)?;
    // VMA 0xb3ec0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3ec4
    c.b(C::Lt,0xb3df4,false)?;
    // VMA 0xb3ec8
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,0)?)?;
    // VMA 0xb3ecc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R12,24)?)?;
    // VMA 0xb3ed0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb3ed4
    c.b(C::Eq,0xb3f0c,false)?;
    // VMA 0xb3ed8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,28)?)?;
    // VMA 0xb3edc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,16)?)?;
    // VMA 0xb3ee0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3ee4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R2,0)?)?;
    // VMA 0xb3ee8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb3eec
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // VMA 0xb3ef0
    c.emit(arm32::transfer_word(C::Ne,true,R::new(4)?,R::R12,32)?)?;
    // VMA 0xb3ef4
    c.emit(arm32::transfer_byte_register_offset(C::Ne,true,R::new(4)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3ef8
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(4)?,R::new(7)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3efc
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(4)?,R::R1,R::new(3)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3f00
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xb3f04
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3f08
    c.b(C::Lt,0xb3ee4,false)?;
    // VMA 0xb3f0c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R12,11)?)?;
    // VMA 0xb3f10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb3f14
    c.b(C::Ne,0xb407c,false)?;
    // VMA 0xb3f18
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R12,25)?)?;
    // VMA 0xb3f1c
    c.literal(R::R2,0xb40a8)?;
    // VMA 0xb3f20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14&0xffffffff)?)?;
    // VMA 0xb3f24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,18)?)?;
    // VMA 0xb3f28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3f2c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xb3f30
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,156)?)?;
    // VMA 0xb3f34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb3f38
    c.emit(arm32::data_immediate(C::Lt,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb3f3c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb3f40
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xb3f44
    c.b(C::Lt,0xb4030,false)?;
    // VMA 0xb3f48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13&0xffffffff)?)?;
    // VMA 0xb3f4c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(9)?,644)?)?;
    // VMA 0xb3f50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(9)?,596)?)?;
    // VMA 0xb3f54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(9)?,612)?)?;
    // VMA 0xb3f58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,268&0xffffffff)?)?;
    // VMA 0xb3f5c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // VMA 0xb3f60
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xb3f64
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3f68
    c.b(C::Ge,0xb3fa0,false)?;
    // VMA 0xb3f6c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xb3f70
    c.b(C::Lt,0xb3fa0,false)?;
    // VMA 0xb3f74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1&0xffffffff)?)?;
    // VMA 0xb3f78
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xb3f7c
    c.emit(arm32::load_signed_byte_postindexed(C::Al,R::new(3)?,R::R1,1)?)?;
    // VMA 0xb3f80
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::new(6)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3f84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb3f88
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3f8c
    c.b(C::Ge,0xb3fa0,false)?;
    // VMA 0xb3f90
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // VMA 0xb3f94
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb3f98
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // VMA 0xb3f9c
    c.b(C::Lt,0xb3f7c,false)?;
    // VMA 0xb3fa0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3fa4
    c.b(C::Ge,0xb3fe0,false)?;
    // VMA 0xb3fa8
    c.emit(arm32::load_signed_byte_register(C::Al,R::R1,R::new(9)?,R::R12)?)?;
    // VMA 0xb3fac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3fb0
    c.b(C::Lt,0xb3fe0,false)?;
    // VMA 0xb3fb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb3fb8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xb3fbc
    c.emit(arm32::load_signed_byte_postindexed(C::Al,R::new(4)?,R::R2,1)?)?;
    // VMA 0xb3fc0
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(4)?,R::new(6)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb3fc4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb3fc8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3fcc
    c.b(C::Ge,0xb3fe0,false)?;
    // VMA 0xb3fd0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(3)?,1)?)?;
    // VMA 0xb3fd4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb3fd8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // VMA 0xb3fdc
    c.b(C::Lt,0xb3fbc,false)?;
    // VMA 0xb3fe0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb3fe4
    c.b(C::Ge,0xb4028,false)?;
    // VMA 0xb3fe8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15&0xffffffff)?)?;
    // VMA 0xb3fec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xb3ff0
    c.emit(arm32::load_signed_byte_register(C::Al,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xb3ff4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xb3ff8
    c.b(C::Lt,0xb4028,false)?;
    // VMA 0xb3ffc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb4000
    c.emit(arm32::mov_register(C::Al,R::R2,R::LR))?;
    // VMA 0xb4004
    c.emit(arm32::load_signed_byte_postindexed(C::Al,R::new(4)?,R::R2,1)?)?;
    // VMA 0xb4008
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(4)?,R::new(6)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb400c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb4010
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb4014
    c.b(C::Ge,0xb4028,false)?;
    // VMA 0xb4018
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(3)?,1)?)?;
    // VMA 0xb401c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb4020
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // VMA 0xb4024
    c.b(C::Lt,0xb4004,false)?;
    // VMA 0xb4028
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb402c
    c.b(C::Lt,0xb3f64,false)?;
    // VMA 0xb4030
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,0)?)?;
    // VMA 0xb4034
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,25)?)?;
    // VMA 0xb4038
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb403c
    c.b(C::Eq,0xb407c,false)?;
    // VMA 0xb4040
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,28)?)?;
    // VMA 0xb4044
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,84)?)?;
    // VMA 0xb4048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16&0xffffffff)?)?;
    // VMA 0xb404c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R2,1)?)?;
    // VMA 0xb4050
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xb4054
    c.emit(arm32::transfer_word(C::Ne,true,R::new(7)?,R::new(5)?,32)?)?;
    // VMA 0xb4058
    c.emit(arm32::transfer_byte_register_offset(C::Ne,true,R::new(7)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb405c
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::new(7)?,R::new(3)?,16)?)?;
    // VMA 0xb4060
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xb4064
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(7)?,R::R1,4)?)?;
    // VMA 0xb4068
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(3)?,1)?)?;
    // VMA 0xb406c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,15)?)?;
    // VMA 0xb4070
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb4074
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // VMA 0xb4078
    c.b(C::Lt,0xb404c,false)?;
    // VMA 0xb407c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,152)?)?;
    // VMA 0xb4080
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,16)?)?;
    // VMA 0xb4084
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb4088
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb408c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,156)?)?;
    // VMA 0xb4090
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xb4094
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,68)?)?;
    // VMA 0xb4098
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb409c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb40a0)
}

const LIVE_CODEC_BUFFER_GLOBALS:[u32;3]=[
    0x0010_b654,0x0010_b698,0x000f_8be8,
];
