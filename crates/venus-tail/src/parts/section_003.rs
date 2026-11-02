const RESOURCE_INIT_LOGGING:[u32;24]=[
    0x000f7e7b, 0x000f5c38, 0x000e2788, 0xe00e0008,
    0x000f8181, 0x000f81ae, 0x0010b608, 0x08020080,
    0x000a6ca8, 0x000a6900, 0x000a5f6c, 0x000a582c,
    0x0010ceac, 0x000f59a4, 0x000f59d8, 0x000f5a0c,
    0x000f5c8d, 0x000f5cbd, 0x000f5cf5, 0x000f5d31,
    0x000f5d68, 0x000f5d98, 0x000f5dc9, 0x0010dd70,
];

// Runtime data 0xa8c24..0xa8c90: 27 pointers and hardware constants.
const RESOURCE_INITIALIZER_CALLBACKS:[u32;27]=[
    0x000a8cbc, 0x0010dd80, 0x000ad728, 0x0010dd8c,
    0x000b1b44, 0x0010dd84, 0x000b11b4, 0x0010dd88,
    0x000b06dc, 0x0010dd98, 0x000b49a4, 0x0010dd94,
    0x000b4924, 0x0010dd90, 0x000b2f08, 0x0010dd64,
    0x000a9038, 0x0010dd68, 0x000a91c4, 0x0010dd6c,
    0x000a9224, 0x0010dd78, 0x000a9518, 0x0010dd7c,
    0x000a9534, 0x0010dd74, 0x000a9284,
];

// Runtime data 0xa8c90..0xa8cbc: 11 pointers and hardware constants.
const RESOURCE_ERROR_MESSAGES:[u32;11]=[
    0x000f5a98, 0x000f8246, 0x000f821c, 0x000f81f1,
    0x000f5e51, 0x000f5e21, 0x000f5a6b, 0x000f5df8,
    0x000f81c1, 0x000f5c61, 0x000f4f2d,
];


/// Initialize three codec buffer classes and reset 28 fixed-size command slots.
fn clear_multichannel_codec_slots(c:&mut Code)->Result<(),String>{
    c.check(0xa8cbc)?;
    // VMA 0xa8cbc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8cc0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xa8cc4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xa8cc8
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // VMA 0xa8ccc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8cd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,668)?)?;
    // VMA 0xa8cd4
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa8cd8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,8)?)?;
    // VMA 0xa8cdc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,12)?)?;
    // VMA 0xa8ce0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8ce4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xa8ce8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8cec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2352)?)?;
    // VMA 0xa8cf0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8cf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1888)?)?;
    // VMA 0xa8cf8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa8cfc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8d00
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65536)?)?;
    // VMA 0xa8d04
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8d08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1504)?)?;
    // VMA 0xa8d0c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa8d10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8d14
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // VMA 0xa8d18
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8d1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xa8d20
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa8d24
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,8)?)?;
    // VMA 0xa8d28
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // VMA 0xa8d2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xa8d30
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,256)?)?;
    // VMA 0xa8d34
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // VMA 0xa8d38
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,260)?)?;
    // VMA 0xa8d3c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xa8d40
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::R2,R::R0,84)?)?;
    // VMA 0xa8d44
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,27)?)?;
    // VMA 0xa8d48
    c.b(C::Lt,0xa8d3c,false)?;
    // VMA 0xa8d4c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa8d50)
}

/// Initialize a single codec buffer and reset 28 command slot tags.
fn clear_single_codec_slots(c:&mut Code)->Result<(),String>{
    c.check(0xa8d50)?;
    // VMA 0xa8d50
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8d54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xa8d58
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xa8d5c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xa8d60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8d64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,676)?)?;
    // VMA 0xa8d68
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,0)?)?;
    // VMA 0xa8d6c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,4)?)?;
    // VMA 0xa8d70
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8d74
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xa8d78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8d7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2352)?)?;
    // VMA 0xa8d80
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8d84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xa8d88
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,0)?)?;
    // VMA 0xa8d8c
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // VMA 0xa8d90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xa8d94
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,256)?)?;
    // VMA 0xa8d98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // VMA 0xa8d9c
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,260)?)?;
    // VMA 0xa8da0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xa8da4
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::R2,R::R0,84)?)?;
    // VMA 0xa8da8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,27)?)?;
    // VMA 0xa8dac
    c.b(C::Lt,0xa8da0,false)?;
    // VMA 0xa8db0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa8db4)
}

/// Find the first available codec slot and compute its entry address.
fn select_next_codec_slot(c:&mut Code)->Result<(),String>{
    c.check(0xa8db4)?;
    // VMA 0xa8db4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8db8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa8dbc
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xa8dc0
    c.b(C::Al,0xa8de4,true)?;
    // VMA 0xa8dc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // VMA 0xa8dc8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // VMA 0xa8dcc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xa8dd0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // VMA 0xa8dd4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // VMA 0xa8dd8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xa8ddc
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // VMA 0xa8de0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa8de4)
}

/// Walk slot-state records, fill a selected entry or issue diagnostics when none is available.
fn find_codec_record_by_tag(c:&mut Code)->Result<(),String>{
    c.check(0xa8de4)?;
    // VMA 0xa8de4
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,256)?)?;
    // VMA 0xa8de8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // VMA 0xa8dec
    c.b(C::Lt,0xa8e24,false)?;
    // VMA 0xa8df0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa8df4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,81)?)?;
    // VMA 0xa8df8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8dfc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,-81)?)?;
    // VMA 0xa8e00
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xa8e04
    c.b(C::Ne,0xa8e14,false)?;
    // VMA 0xa8e08
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,-80)?)?;
    // VMA 0xa8e0c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // VMA 0xa8e10
    c.b(C::Eq,0xa8e64,false)?;
    // VMA 0xa8e14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa8e18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,84)?)?;
    // VMA 0xa8e1c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa8e20
    c.b(C::Lt,0xa8dfc,false)?;
    // VMA 0xa8e24
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8e28
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xa8e2c
    c.literal(R::R1,0xa8e90)?;
    // VMA 0xa8e30
    c.literal(R::R2,0xa8e94)?;
    // VMA 0xa8e34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8e38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,201)?)?;
    // VMA 0xa8e3c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8e40
    c.literal(R::R1,0xa8e98)?;
    // VMA 0xa8e44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8e48
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8e4c
    c.literal(R::R2,0xa8e9c)?;
    // VMA 0xa8e50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8e54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,202)?)?;
    // VMA 0xa8e58
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa8e5c
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // VMA 0xa8e60
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa8e64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // VMA 0xa8e68
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-77)?)?;
    // VMA 0xa8e6c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-81)?)?;
    // VMA 0xa8e70
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,40)?)?;
    // VMA 0xa8e74
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-69)?)?;
    // VMA 0xa8e78
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,9)?)?;
    // VMA 0xa8e7c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,-79)?)?;
    // VMA 0xa8e80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8e84
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xa8e88
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,-45)?)?;
    // VMA 0xa8e8c
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa8e90)
}

// Source diagnostic address pool, not executable ARM instructions.
const MISSING_CODEC_SLOT_MESSAGES:[u32;4]=[
    0x000f_5fa8,0x000f_82e9,0x000e_2788,0x000f_82fd,
];


/// Reset and validate the selected codec command slot, including byte-sized tag flags.
fn validate_slot_substate(c:&mut Code)->Result<(),String>{
    c.check(0xa8ea0)?;
    // VMA 0xa8ea0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,252)?)?;
    // VMA 0xa8ea4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8ea8
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,35)?)?;
    // VMA 0xa8eac
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xa8eb0
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa8eb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8eb8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xa8ebc
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ec0
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1)?)?;
    // VMA 0xa8ec4
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,4)?)?;
    // VMA 0xa8ec8
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,5)?)?;
    // VMA 0xa8ecc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // VMA 0xa8ed0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,20)?)?;
    // VMA 0xa8ed4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // VMA 0xa8ed8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,28)?)?;
    // VMA 0xa8edc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // VMA 0xa8ee0
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,80)?)?;
    // VMA 0xa8ee4
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa8ee8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,252)?)?;
    // VMA 0xa8eec
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,12)?)?;
    // VMA 0xa8ef0
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa8ef4
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::new(3)?,0)?)?;
    // VMA 0xa8ef8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa8efc
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,1)?)?;
    // VMA 0xa8f00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,252)?)?;
    // VMA 0xa8f04
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R0,0)?)?;
    // VMA 0xa8f08
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,1)?)?;
    // VMA 0xa8f0c
    c.emit(arm32::transfer_byte_postindexed_register(C::Al,false,R::new(3)?,R::R0,R::R1)?)?;
    // VMA 0xa8f10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8f14
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,18)?)?;
    // VMA 0xa8f18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,84)?)?;
    // VMA 0xa8f1c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R2,R::R0,R::R12)?)?;
    // VMA 0xa8f20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xa8f24
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,1)?)?;
    // VMA 0xa8f28
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,4)?)?;
    // VMA 0xa8f2c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,5)?)?;
    // VMA 0xa8f30
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,8)?)?;
    // VMA 0xa8f34
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,20)?)?;
    // VMA 0xa8f38
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,24)?)?;
    // VMA 0xa8f3c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,28)?)?;
    // VMA 0xa8f40
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,32)?)?;
    // VMA 0xa8f44
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa8f48
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8f4c
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xa8f50
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,256)?)?;
    // VMA 0xa8f54
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,1)?)?;
    // VMA 0xa8f58
    c.b(C::Lt,0xa8fa8,false)?;
    // VMA 0xa8f5c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // VMA 0xa8f60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa8f64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3)?)?;
    // VMA 0xa8f68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,80)?)?;
    // VMA 0xa8f6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8f70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xa8f74
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-80)?)?;
    // VMA 0xa8f78
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R2,-79)?)?;
    // VMA 0xa8f7c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-76)?)?;
    // VMA 0xa8f80
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-75)?)?;
    // VMA 0xa8f84
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-72)?)?;
    // VMA 0xa8f88
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-60)?)?;
    // VMA 0xa8f8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-56)?)?;
    // VMA 0xa8f90
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-52)?)?;
    // VMA 0xa8f94
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-48)?)?;
    // VMA 0xa8f98
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // VMA 0xa8f9c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,84)?)?;
    // VMA 0xa8fa0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xa8fa4
    c.b(C::Lt,0xa8f70,false)?;
    // VMA 0xa8fa8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8fac
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,248)?)?;
    // VMA 0xa8fb0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa8fb4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8fb8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xa8fbc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa8fc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa8fc4
    c.literal(R::new(4)?,0xa9034)?;
    // VMA 0xa8fc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa8fcc
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa8fd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,238)?)?;
    // VMA 0xa8fd4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xa8fd8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1)?)?;
    // VMA 0xa8fdc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xa8fe0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8fe4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8fe8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa8fec
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xa8ff0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,239)?)?;
    // VMA 0xa8ff4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xa8ff8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8ffc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9000
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa9004
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa9008
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa900c
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,20)?)?;
    // VMA 0xa9010
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa9014
    c.emit(arm32::transfer_word(C::Ne,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa9018
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // VMA 0xa901c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9020
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9024
    c.b(C::Al,0x7fed0,true)?;
    // VMA 0xa9028
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa902c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xa9030
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa9034)
}

/// Update an allocated slot and publish codec flags and resource pointer diagnostics.
fn publish_slot_allocator_pointer(c:&mut Code)->Result<(),String>{
    c.check(0xa9038)?;
    // VMA 0xa9038
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,8)?)?;
    // VMA 0xa903c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,200)?)?;
    // VMA 0xa9040
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R2)?)?;
    // VMA 0xa9044
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa9048
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa904c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa9050
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa9054
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,20)?)?;
    // VMA 0xa9058
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa905c
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::R0,8)?)?;
    // VMA 0xa9060
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // VMA 0xa9064
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa9068
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xa906c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xa9070
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,32)?)?;
    // VMA 0xa9074
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xa9078
    c.emit(arm32::mov_register(C::Al,R::R2,R::SP))?;
    // VMA 0xa907c
    c.b(C::Al,0x801a8,true)?;
    // VMA 0xa9080
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // VMA 0xa9084
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // VMA 0xa9088
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa908c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa9090
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa9094
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa9098
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa909c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::R1,R::new(3)?,R::R0)?)?;
    // VMA 0xa90a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    // VMA 0xa90a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // VMA 0xa90a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,8)?)?;
    // VMA 0xa90ac
    c.literal(R::R2,0xa90d8)?;
    // VMA 0xa90b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // VMA 0xa90b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa90b8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // VMA 0xa90bc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa90c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,308)?)?;
    // VMA 0xa90c4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa90c8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa90cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa90d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // VMA 0xa90d4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa90d8)
}

/// Confirm visibility of codec slot metadata and report its initialized state.
fn confirm_slot_visibility(c:&mut Code)->Result<(),String>{
    c.check(0xa90dc)?;
    // VMA 0xa90dc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa90e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa90e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa90e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa90ec
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa90f0
    c.literal(R::R2,0xa911c)?;
    // VMA 0xa90f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa90f8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // VMA 0xa90fc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa9100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,71)?)?;
    // VMA 0xa9104
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa9108
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xa910c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa9110
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa9114
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // VMA 0xa9118
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa911c)
}
