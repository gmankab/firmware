
/// Copy existing session state into a new codec-source structure, attach it to
/// the active controller and conditionally reset codec frame streams.
fn clone_optional_codec_source(c:&mut Code)->Result<(),String>{
    c.check(0xb9f80)?;
    // ARM VMA 0xb9f80
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb9f84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // ARM VMA 0xb9f88
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // ARM VMA 0xb9f8c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb9f90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9f94
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb9f98
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,364)?)?;
    // ARM VMA 0xb9f9c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM VMA 0xb9fa0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // ARM VMA 0xb9fa4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,2141)?)?;
    // ARM VMA 0xb9fa8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9fac
    c.b(C::Eq,0xba074,false)?;
    // ARM VMA 0xb9fb0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // ARM VMA 0xb9fb4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb9fb8
    c.b(C::Eq,0xba0fc,false)?;
    // ARM VMA 0xb9fbc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,368)?)?;
    // ARM VMA 0xb9fc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,248)?)?;
    // ARM VMA 0xb9fc4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // ARM VMA 0xb9fc8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,208)?)?;
    // ARM VMA 0xb9fcc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,208)?)?;
    // ARM VMA 0xb9fd0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,524)?)?;
    // ARM VMA 0xb9fd4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9fd8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,528)?)?;
    // ARM VMA 0xb9fdc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9fe0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // ARM VMA 0xb9fe4
    c.b(C::Al,0x471c,true)?;
    // ARM VMA 0xb9fe8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,368)?)?;
    // ARM VMA 0xb9fec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,136)?)?;
    // ARM VMA 0xb9ff0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // ARM VMA 0xb9ff4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9ff8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM VMA 0xb9ffc
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit() | R::new(3)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xba000
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit() | R::new(3)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xba004
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit() | R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xba008
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1216)?)?;
    // ARM VMA 0xba00c
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit() | R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xba010
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,368)?)?;
    // ARM VMA 0xba014
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1216)?)?;
    // ARM VMA 0xba018
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xba01c
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xba020
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit() | R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xba024
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit() | R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xba028
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,972)?)?;
    // ARM VMA 0xba02c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xba030
    c.b(C::Ne,0xba0a8,false)?;
    // ARM VMA 0xba034
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,368)?)?;
    // ARM VMA 0xba038
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,212)?)?;
    // ARM VMA 0xba03c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,212)?)?;
    // ARM VMA 0xba040
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,224)?)?;
    // ARM VMA 0xba044
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,224)?)?;
    // ARM VMA 0xba048
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,228)?)?;
    // ARM VMA 0xba04c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,228)?)?;
    // ARM VMA 0xba050
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,308)?)?;
    // ARM VMA 0xba054
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,308)?)?;
    // ARM VMA 0xba058
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,320)?)?;
    // ARM VMA 0xba05c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,320)?)?;
    // ARM VMA 0xba060
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,312)?)?;
    // ARM VMA 0xba064
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,312)?)?;
    // ARM VMA 0xba068
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,324)?)?;
    // ARM VMA 0xba06c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,324)?)?;
    // ARM VMA 0xba070
    c.b(C::Al,0xba0a8,false)?;
    // ARM VMA 0xba074
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,436)?)?;
    // ARM VMA 0xba078
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
    // ARM VMA 0xba07c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,12)?)?;
    // ARM VMA 0xba080
    c.b(C::Al,0x7f974,true)?;
    // ARM VMA 0xba084
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // ARM VMA 0xba088
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xba08c
    c.b(C::Eq,0xba1e8,false)?;
    // ARM VMA 0xba090
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,208)?)?;
    // ARM VMA 0xba094
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,972)?)?;
    // ARM VMA 0xba098
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xba09c
    c.b(C::Ne,0xba0a8,false)?;
    // ARM VMA 0xba0a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xba0a4
    c.b(C::Al,0xba0f0,false)?;
    // ARM VMA 0xba0a8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // ARM VMA 0xba0ac
    c.literal(R::R1,0xba20c)?;
    // ARM VMA 0xba0b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xba0b4
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xba0b8
    c.literal(R::R1,0xba1f8)?;
    // ARM VMA 0xba0bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xba0c0
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xba0c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,552)?)?;
    // ARM VMA 0xba0c8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xba0cc
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,520)?)?;
    // ARM VMA 0xba0d0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xba0d4
    c.b(C::Eq,0xba0ec,false)?;
    // ARM VMA 0xba0d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1028)?)?;
    // ARM VMA 0xba0dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xba0e0
    c.b(C::Eq,0xba0ec,false)?;
    // ARM VMA 0xba0e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM VMA 0xba0e8
    c.b(C::Al,0xba0f0,false)?;
    // ARM VMA 0xba0ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xba0f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,232)?)?;
    // ARM VMA 0xba0f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xba0f8
    c.b(C::Al,0xba1ec,false)?;
    // ARM VMA 0xba0fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,368)?)?;
    // ARM VMA 0xba100
    c.literal(R::R1,0xba1f4)?;
    // ARM VMA 0xba104
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // ARM VMA 0xba108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xba10c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xba110
    c.literal(R::R1,0xba1f8)?;
    // ARM VMA 0xba114
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM VMA 0xba118
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xba11c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1092)?)?;
    // ARM VMA 0xba120
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xba124
    c.b(C::Ne,0xba170,false)?;
    // ARM VMA 0xba128
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xba12c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM VMA 0xba130
    c.b(C::Al,0xba210,true)?;
    // ARM VMA 0xba134
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xba138
    c.b(C::Eq,0xba170,false)?;
    // ARM VMA 0xba13c
    c.literal(R::R1,0xba1fc)?;
    // ARM VMA 0xba140
    c.literal(R::R2,0xba200)?;
    // ARM VMA 0xba144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // ARM VMA 0xba148
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xba14c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // ARM VMA 0xba150
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xba154
    c.literal(R::R1,0xba1f8)?;
    // ARM VMA 0xba158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xba15c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xba160
    c.literal(R::R2,0xba204)?;
    // ARM VMA 0xba164
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xba168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,260)?)?;
    // ARM VMA 0xba16c
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xba170
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,368)?)?;
    // ARM VMA 0xba174
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // ARM VMA 0xba178
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM VMA 0xba17c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,244)?)?;
    // ARM VMA 0xba180
    c.b(C::Al,0x80928,true)?;
    // ARM VMA 0xba184
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xba188
    c.b(C::Eq,0xba1c4,false)?;
    // ARM VMA 0xba18c
    c.literal(R::R1,0xba208)?;
    // ARM VMA 0xba190
    c.literal(R::R2,0xba200)?;
    // ARM VMA 0xba194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // ARM VMA 0xba198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xba19c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // ARM VMA 0xba1a0
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xba1a4
    c.literal(R::R1,0xba1f8)?;
    // ARM VMA 0xba1a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xba1ac
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xba1b0
    c.literal(R::R2,0xba204)?;
    // ARM VMA 0xba1b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // ARM VMA 0xba1b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xba1bc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // ARM VMA 0xba1c0
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xba1c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,36)?)?;
    // ARM VMA 0xba1c8
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xba1cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xba1d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM VMA 0xba1d4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2141)?)?;
    // ARM VMA 0xba1d8
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2140)?)?;
    // ARM VMA 0xba1dc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,368)?)?;
    // ARM VMA 0xba1e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,232)?)?;
    // ARM VMA 0xba1e4
    c.b(C::Al,0xba1ec,false)?;
    // ARM VMA 0xba1e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xba1ec
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // ARM VMA 0xba1f0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xba1f4)?;
    const SOURCE_RELAY_DIAGNOSTICS:[u32;7]=[
        0x000f_9b32,0x000e_2788,0x000f_9b5e,0x000f_9b9a,
        0x000f_9ad3,0x000f_9bab,0x000f_9bd8,
    ];
    for word in SOURCE_RELAY_DIAGNOSTICS {c.emit(word)?;}
    c.check(0xba210)
}

/// Preserve stream object selection: non-final buffers are pushed to the
/// auxiliary controller, while completed channels are released through the
/// existing runtime; the invalid-mode branch emits the original diagnostics.
fn select_codec_stream_end_state(c:&mut Code)->Result<(),String>{
    c.check(0xba210)?;
    // VMA 0xba210: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xba214: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xba218: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xba21c: ldrb r0, [r1, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,8)?)?;
    // VMA 0xba220: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xba224: bhs 0xba23c <entry+0x2c>    @ imm = #0x10
    c.b(C::Cs,0xba23c,false)?;
    // VMA 0xba228: ldr r1, [r1, #0xd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,208)?)?;
    // VMA 0xba22c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xba230: bl 0xbb654 <entry+0x1444>  @ imm = #0x141c
    c.b(C::Al,0xbb654,true)?;
    // VMA 0xba234: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xba238: b 0xba28c <entry+0x7c>    @ imm = #0x4c
    c.b(C::Al,0xba28c,false)?;
    // VMA 0xba23c: bne 0xba268 <entry+0x58>    @ imm = #0x24
    c.b(C::Ne,0xba268,false)?;
    // VMA 0xba240: ldr r0, [r4, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,376)?)?;
    // VMA 0xba244: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba248: beq 0xba250 <entry+0x40>    @ imm = #0x0
    c.b(C::Eq,0xba250,false)?;
    // VMA 0xba24c: bl 0x7ffc8 <entry+0xfffffffffffc5db8> @ imm = #-0x3a28c
    c.b(C::Al,0x7ffc8,true)?;
    // VMA 0xba250: ldr r0, [r4, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,372)?)?;
    // VMA 0xba254: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xba258: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba25c: beq 0xba28c <entry+0x7c>    @ imm = #0x28
    c.b(C::Eq,0xba28c,false)?;
    // VMA 0xba260: bl 0x7ffc8 <entry+0xfffffffffffc5db8> @ imm = #-0x3a2a0
    c.b(C::Al,0x7ffc8,true)?;
    // VMA 0xba264: b 0xba28c <entry+0x7c>    @ imm = #0x20
    c.b(C::Al,0xba28c,false)?;
    // VMA 0xba268: ldr r1, [pc, #0x24]         @ 0x84 <entry+0x84>
    c.literal(R::R1,0xba294)?;
    // VMA 0xba26c: ldr r2, [pc, #0x24]         @ 0x88 <entry+0x88>
    c.literal(R::R2,0xba298)?;
    // VMA 0xba270: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xba274: mov r3, #1120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1120)?)?;
    // VMA 0xba278: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b5ec
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba27c: ldr r1, [pc, #0x18]         @ 0x8c <entry+0x8c>
    c.literal(R::R1,0xba29c)?;
    // VMA 0xba280: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xba284: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b5f8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba288: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xba28c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xba290: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xba294)?;
    Ok(())
}
const STREAM_END_DIAGNOSTICS:[u32;3]=[0x000f_9a31,0x000f_9f75,0x000e_2788];

/// Drain or clear outstanding DMA state and set the state-machine result for
/// the active context, respecting conditional return and error paths.
fn settle_codec_dma_status(c:&mut Code)->Result<(),String>{
    c.check(0xba2a0)?;
    // VMA 0xba2a0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xba2a4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xba2a8: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xba2ac: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xba2b0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xba2b4: ldr r5, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,4)?)?;
    // VMA 0xba2b8: ldr r6, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,364)?)?;
    // VMA 0xba2bc: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // VMA 0xba2c0: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // VMA 0xba2c4: ldr r0, [r5, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,968)?)?;
    // VMA 0xba2c8: bl 0x2bf4 <entry+0xfffffffffff489e4> @ imm = #-0xb76dc
    c.b(C::Al,0x2bf4,true)?;
    // VMA 0xba2cc: ldr r0, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,972)?)?;
    // VMA 0xba2d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba2d4: beq 0xba33c <entry+0x12c>   @ imm = #0x60
    c.b(C::Eq,0xba33c,false)?;
    // VMA 0xba2d8: add r1, r6, #304
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,304)?)?;
    // VMA 0xba2dc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xba2e0: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xba2e4: bl 0xd8be4 <entry+0x1e9d4> @ imm = #0x1e8f8
    c.b(C::Al,0xd8be4,true)?;
    // VMA 0xba2e8: ldr r0, [r5, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,968)?)?;
    // VMA 0xba2ec: bl 0x2c34 <entry+0xfffffffffff48a24> @ imm = #-0xb76c0
    c.b(C::Al,0x2c34,true)?;
    // VMA 0xba2f0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xba2f4: mov r1, sp
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // VMA 0xba2f8: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // VMA 0xba2fc: ldr r0, [r0, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,448)?)?;
    // VMA 0xba300: bl 0x7f974 <entry+0xfffffffffffc5764> @ imm = #-0x3a994
    c.b(C::Al,0x7f974,true)?;
    // VMA 0xba304: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // VMA 0xba308: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba30c: beq 0xba358 <entry+0x148>   @ imm = #0x44
    c.b(C::Eq,0xba358,false)?;
    // VMA 0xba310: str r0, [r6, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,212)?)?;
    // VMA 0xba314: ldr r0, [r6, #0x228]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,552)?)?;
    // VMA 0xba318: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xba31c: ldreq r0, [r6, #0x208]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(6)?,520)?)?;
    // VMA 0xba320: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba324: beq 0xba334 <entry+0x124>   @ imm = #0x8
    c.b(C::Eq,0xba334,false)?;
    // VMA 0xba328: ldr r0, [r6, #0x404]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1028)?)?;
    // VMA 0xba32c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba330: bne 0xba348 <entry+0x138>   @ imm = #0x10
    c.b(C::Ne,0xba348,false)?;
    // VMA 0xba334: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xba338: b 0xba34c <entry+0x13c>   @ imm = #0xc
    c.b(C::Al,0xba34c,false)?;
    // VMA 0xba33c: ldr r0, [r5, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,968)?)?;
    // VMA 0xba340: bl 0x2c34 <entry+0xfffffffffff48a24> @ imm = #-0xb7714
    c.b(C::Al,0x2c34,true)?;
    // VMA 0xba344: str r7, [r6, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,212)?)?;
    // VMA 0xba348: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xba34c: str r0, [r4, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,232)?)?;
    // VMA 0xba350: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xba354: b 0xba35c <entry+0x14c>   @ imm = #0x0
    c.b(C::Al,0xba35c,false)?;
    // VMA 0xba358: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xba35c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xba360: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xba364)?;
    Ok(())
}

/// Release the registered codec object and its controller slots, including
/// callback notification before restoring the original ARM stack frame.
fn reset_codec_command_session(c:&mut Code)->Result<(),String>{
    c.check(0xba364)?;
    // VMA 0xba364: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xba368: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xba36c: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xba370: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xba374: ldr r6, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,4)?)?;
    // VMA 0xba378: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xba37c: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xba380: bl 0xbbcd4 <entry+0x1ac4>  @ imm = #0x194c
    c.b(C::Al,0xbbcd4,true)?;
    // VMA 0xba384: ldr r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // VMA 0xba388: bl 0x293c <entry+0xfffffffffff4872c> @ imm = #-0xb7a54
    c.b(C::Al,0x293c,true)?;
    // VMA 0xba38c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xba390: ldr r1, [pc, #0x64]         @ 0x1ec <entry+0x1ec>
    c.literal(R::R1,0xba3fc)?;
    // VMA 0xba394: str r7, [r6, #0xa88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,2696)?)?;
    // VMA 0xba398: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xba39c: str r7, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R1,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xba3a0: ldr r1, [pc, #0x58]         @ 0x1f0 <entry+0x1f0>
    c.literal(R::R1,0xba400)?;
    // VMA 0xba3a4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xba3a8: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xba3ac: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b720
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba3b0: ldr r1, [pc, #0x4c]         @ 0x1f4 <entry+0x1f4>
    c.literal(R::R1,0xba404)?;
    // VMA 0xba3b4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xba3b8: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b72c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba3bc: bl 0x3070 <entry+0xfffffffffff48e60> @ imm = #-0xb7354
    c.b(C::Al,0x3070,true)?;
    // VMA 0xba3c0: ldr r0, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,48)?)?;
    // VMA 0xba3c4: ldr r2, [pc, #0x3c]         @ 0x1f8 <entry+0x1f8>
    c.literal(R::R2,0xba408)?;
    // VMA 0xba3c8: mov r3, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,98)?)?;
    // VMA 0xba3cc: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xba3d0: mla r1, r0, r3, r2
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R0,R::new(3)?,R::R2)?)?;
    // VMA 0xba3d4: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xba3d8: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // VMA 0xba3dc: bl 0x266a8 <entry+0xfffffffffff6c498> @ imm = #-0x93d3c
    c.b(C::Al,0x266a8,true)?;
    // VMA 0xba3e0: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xba3e4: bl 0xbc7bc <entry+0x25ac>  @ imm = #0x23d0
    c.b(C::Al,0xbc7bc,true)?;
    // VMA 0xba3e8: bl 0x3074 <entry+0xfffffffffff48e64> @ imm = #-0xb737c
    c.b(C::Al,0x3074,true)?;
    // VMA 0xba3ec: ldr r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // VMA 0xba3f0: bl 0x294c <entry+0xfffffffffff4873c> @ imm = #-0xb7aac
    c.b(C::Al,0x294c,true)?;
    // VMA 0xba3f4: str r7, [r6, #0xa88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,2696)?)?;
    // VMA 0xba3f8: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xba3fc)?;
    Ok(())
}
const CODEC_RESET_GLOBALS:[u32;4]=[
    0x0010_4090,0x000f_9bfc,0x000e_2788,0x004c_4b40,
];
