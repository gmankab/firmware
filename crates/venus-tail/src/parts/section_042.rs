
const FIVE_WAY_CLEANUP_DISPATCH:[u32;5]=[
    0x000b6f5c,0x000b6f9c,0x000b6fa8,0x000b6fb4,0x000b6fc0,
];
const CONTROLLER_RELEASE_MESSAGES:[u32;4]=[
    0x000f_9355,0x000f_9384,0x000e_2788,0x000f_8fbc,
];

/// Release the active codec context, notifying its controller and traversing
/// registered response entries. A five-way state dispatch chooses between
/// distinct cleanup routines; unknown states emit a diagnostic and retry.
/// This routine also guards individually owned buffer and callback resources.
fn release_codec_context_and_reset_queue(c:&mut Code)->Result<(),String>{
    c.check(0xb6edc)?;
    // ARM VMA 0xb6edc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb6ee0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM VMA 0xb6ee4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // ARM VMA 0xb6ee8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb6eec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb6ef0
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb6ef4
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(6)?,R::R1.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM VMA 0xb6ef8
    c.b(C::Al,0x3084,true)?;
    // ARM VMA 0xb6efc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // ARM VMA 0xb6f00
    c.b(C::Al,0x293c,true)?;
    // ARM VMA 0xb6f04
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,104)?)?;
    // ARM VMA 0xb6f08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // ARM VMA 0xb6f0c
    c.b(C::Ne,0xb6fd4,false)?;
    // ARM VMA 0xb6f10
    c.literal(R::new(8)?,0xb71a8)?;
    // ARM VMA 0xb6f14
    c.literal(R::new(9)?,0xb71ac)?;
    // ARM VMA 0xb6f18
    c.literal(R::new(5)?,0xb71b0)?;
    // ARM VMA 0xb6f1c
    c.literal(R::new(7)?,0xb71b4)?;
    // ARM VMA 0xb6f20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,28)?)?;
    // ARM VMA 0xb6f24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,29)?)?;
    // ARM VMA 0xb6f28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,1024)?)?;
    // ARM VMA 0xb6f2c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1024)?)?;
    // ARM VMA 0xb6f30
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,232)?)?;
    // ARM VMA 0xb6f34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb6f38
    c.b(C::Hi,0xb6f68,false)?;
    // ARM VMA 0xb6f3c
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // ARM VMA 0xb6f40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // ARM VMA 0xb6f44
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0xb6f48)?;

    for target in FIVE_WAY_CLEANUP_DISPATCH { c.emit(target)?; }
    c.check(0xb6f5c)?;
    // ARM VMA 0xb6f5c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb6f60
    c.b(C::Al,0xb9eb8,true)?;
    // ARM VMA 0xb6f64
    c.b(C::Al,0xb6fc8,false)?;
    // ARM VMA 0xb6f68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb6f6c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM VMA 0xb6f70
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // ARM VMA 0xb6f74
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // ARM VMA 0xb6f78
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb6f7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb6f80
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM VMA 0xb6f84
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb6f88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb6f8c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM VMA 0xb6f90
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // ARM VMA 0xb6f94
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb6f98
    c.b(C::Al,0xb6f30,false)?;
    // ARM VMA 0xb6f9c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb6fa0
    c.b(C::Al,0xb9f80,true)?;
    // ARM VMA 0xb6fa4
    c.b(C::Al,0xb6fc8,false)?;
    // ARM VMA 0xb6fa8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb6fac
    c.b(C::Al,0xba2a0,true)?;
    // ARM VMA 0xb6fb0
    c.b(C::Al,0xb6fc8,false)?;
    // ARM VMA 0xb6fb4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb6fb8
    c.b(C::Al,0xba40c,true)?;
    // ARM VMA 0xb6fbc
    c.b(C::Al,0xb6fc8,false)?;
    // ARM VMA 0xb6fc0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb6fc4
    c.b(C::Al,0xbb570,true)?;
    // ARM VMA 0xb6fc8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb6fcc
    c.b(C::Eq,0xb6f30,false)?;
    // ARM VMA 0xb6fd0
    c.b(C::Al,0xb71a0,false)?;
    // ARM VMA 0xb6fd4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,236)?)?;
    // ARM VMA 0xb6fd8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // ARM VMA 0xb6fdc
    c.b(C::Eq,0xb702c,false)?;
    // ARM VMA 0xb6fe0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb6fe4
    c.b(C::Eq,0xb7034,false)?;
    // ARM VMA 0xb6fe8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb6fec
    c.b(C::Ne,0xb7198,false)?;
    // ARM VMA 0xb6ff0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,108)?)?;
    // ARM VMA 0xb6ff4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb6ff8
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb6ffc
    c.b(C::Ne,0xb7188,false)?;
    // ARM VMA 0xb7000
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,368)?)?;
    // ARM VMA 0xb7004
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // ARM VMA 0xb7008
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb700c
    c.b(C::Eq,0xb7098,false)?;
    // ARM VMA 0xb7010
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb7014
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,520)?)?;
    // ARM VMA 0xb7018
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,1028)?)?;
    // ARM VMA 0xb701c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb7020
    c.b(C::Eq,0xb703c,false)?;
    // ARM VMA 0xb7024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM VMA 0xb7028
    c.b(C::Al,0xb7058,false)?;
    // ARM VMA 0xb702c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM VMA 0xb7030
    c.b(C::Al,0xb7194,false)?;
    // ARM VMA 0xb7034
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // ARM VMA 0xb7038
    c.b(C::Al,0xb7194,false)?;
    // ARM VMA 0xb703c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,552)?)?;
    // ARM VMA 0xb7040
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb7044
    c.b(C::Ne,0xb7054,false)?;
    // ARM VMA 0xb7048
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,496)?)?;
    // ARM VMA 0xb704c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb7050
    c.b(C::Eq,0xb705c,false)?;
    // ARM VMA 0xb7054
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb7058
    c.b(C::Al,0xb87d8,true)?;
    // ARM VMA 0xb705c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,368)?)?;
    // ARM VMA 0xb7060
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,204)?)?;
    // ARM VMA 0xb7064
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7068
    c.b(C::Eq,0xb7080,false)?;
    // ARM VMA 0xb706c
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb7070
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7074
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,368)?)?;
    // ARM VMA 0xb7078
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb707c
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,204)?)?;
    // ARM VMA 0xb7080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7084
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,368)?)?;
    // ARM VMA 0xb7088
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2141)?)?;
    // ARM VMA 0xb708c
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2140)?)?;
    // ARM VMA 0xb7090
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2152)?)?;
    // ARM VMA 0xb7094
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // ARM VMA 0xb7098
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,104)?)?;
    // ARM VMA 0xb709c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb70a0
    c.emit(arm32::transfer_word(C::Ne,true,R::new(5)?,R::new(4)?,364)?)?;
    // ARM VMA 0xb70a4
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // ARM VMA 0xb70a8
    c.b(C::Eq,0xb7120,false)?;
    // ARM VMA 0xb70ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,472)?)?;
    // ARM VMA 0xb70b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb70b4
    c.b(C::Ne,0xb70c4,false)?;
    // ARM VMA 0xb70b8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb70bc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM VMA 0xb70c0
    c.b(C::Al,0xb7f94,true)?;
    // ARM VMA 0xb70c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,208)?)?;
    // ARM VMA 0xb70c8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb70cc
    c.b(C::Eq,0xb70e0,false)?;
    // ARM VMA 0xb70d0
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb70d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb70d8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb70dc
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,208)?)?;
    // ARM VMA 0xb70e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,212)?)?;
    // ARM VMA 0xb70e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb70e8
    c.b(C::Eq,0xb70fc,false)?;
    // ARM VMA 0xb70ec
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb70f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb70f4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb70f8
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,212)?)?;
    // ARM VMA 0xb70fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,204)?)?;
    // ARM VMA 0xb7100
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7104
    c.b(C::Eq,0xb7118,false)?;
    // ARM VMA 0xb7108
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb710c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7110
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7114
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,204)?)?;
    // ARM VMA 0xb7118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb711c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,364)?)?;
    // ARM VMA 0xb7120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,300)?)?;
    // ARM VMA 0xb7124
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // ARM VMA 0xb7128
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb712c
    c.b(C::Al,0x80994,true)?;
    // ARM VMA 0xb7130
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7134
    c.b(C::Ne,0xb7188,false)?;
    // ARM VMA 0xb7138
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::SP))?;
    // ARM VMA 0xb713c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // ARM VMA 0xb7140
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,0)?)?;
    // ARM VMA 0xb7144
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,472)?)?;
    // ARM VMA 0xb7148
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb714c
    c.b(C::Ne,0xb715c,false)?;
    // ARM VMA 0xb7150
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7154
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb7158
    c.b(C::Al,0xb7f94,true)?;
    // ARM VMA 0xb715c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,204)?)?;
    // ARM VMA 0xb7160
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7164
    c.b(C::Eq,0xb7174,false)?;
    // ARM VMA 0xb7168
    c.b(C::Al,0x7fed0,true)?;
    // ARM VMA 0xb716c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7170
    c.emit(arm32::transfer_word(C::Eq,false,R::new(9)?,R::new(7)?,204)?)?;
    // ARM VMA 0xb7174
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb7178
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM VMA 0xb717c
    c.b(C::Al,0x80994,true)?;
    // ARM VMA 0xb7180
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7184
    c.b(C::Eq,0xb7140,false)?;
    // ARM VMA 0xb7188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb718c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb7190
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,232)?)?;
    // ARM VMA 0xb7194
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,236)?)?;
    // ARM VMA 0xb7198
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,36)?)?;
    // ARM VMA 0xb719c
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xb71a0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM VMA 0xb71a4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb71a8)?;

    for value in CONTROLLER_RELEASE_MESSAGES { c.emit(value)?; }
    c.check(0xb71b8)
}

const CODEC_CONTEXT_CLOSE_DIAGNOSTICS:[u32;8]=[
    0x000f_8fbc,0x000f_9397,0x000f_93db,0x000e_2788,
    0x000f_93f0,0x000f_941f,0x000f_9448,0x000f_948a,
];
