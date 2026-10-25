// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller input-record allocation and debug-message dispatch.
//! Instruction opcodes are rebuilt from typed ARM32 operations. The two
//! switch/diagnostic tables are non-executable data explicitly identified
//! from control-flow targets, not copied instruction templates.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

const FRAME_CLASS_TARGETS:[u32;7]=[
    0x238c8,0x23bb0,0x23c44,0x23874,0x23c44,0x23c44,0x23bd4,
];
const CONTROLLER_WORDS:[u32;18]=[
    0x0401_100f,0xfbfe_effd,0x0401_1010,0x0401_1012,
    0x0401_1016,0x000e_7275,0x000e_6ffa,0x000e_2788,
    0x000e_722e,0x000e_71d5,0x000e_7193,0x000e_6fc8,
    0x000e_7011,0x000e_704f,0x000e_70b5,0x000e_70f7,
    0x000e_712b,0x000e_6a43,
];

/// Runtime allocation state: check the input mode, dispatch seven format
/// cases, update buffer size accounting, and report invalid frame layout.
fn process_buffer_allocation(c:&mut Code)->Result<(),String>{
    c.check(0x23840)?;
    // 0x23840
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x23844
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x23848
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x2384c
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x23850
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x23854
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x23858
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // 0x2385c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x23860
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2960)?)?;
    // 0x23864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x23868
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x2386c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x23870
    c.b(C::Eq,0x23880)?;
    // 0x23874
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x23878
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2387c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x23880
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x23884
    c.literal(R::R1,0x23c60)?;
    // 0x23888
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2388c
    c.b(C::Gt,0x23b14)?;
    // 0x23890
    c.literal(R::R1,0x23c64)?;
    // 0x23894
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23898
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x2389c
    c.b(C::Hi,0x23c44)?;
    // 0x238a0
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x238a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x238a8
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x238ac)?;
    for address in FRAME_CLASS_TARGETS {c.emit(address)?;}
    c.check(0x238c8)?;
    // 0x238c8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x238cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x238d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x238d4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,32768)?)?;
    // 0x238d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,32768)?)?;
    // 0x238dc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x238e0
    c.b(C::Eq,0x23914)?;
    // 0x238e4
    c.literal(R::R1,0x23c8c)?;
    // 0x238e8
    c.literal(R::R2,0x23c78)?;
    // 0x238ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,218)?)?;
    // 0x238f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x238f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x238f8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x238fc
    c.bl(0x2ec94)?;
    // 0x23900
    c.literal(R::R1,0x23c7c)?;
    // 0x23904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23908
    c.bl(0x2ec94)?;
    // 0x2390c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x23910
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,8192)?)?;
    // 0x23914
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // 0x23918
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,40)?)?;
    // 0x2391c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x23920
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x23924
    c.b(C::Ls,0x23958)?;
    // 0x23928
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2392c
    c.literal(R::R1,0x23c90)?;
    // 0x23930
    c.literal(R::R2,0x23c78)?;
    // 0x23934
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,223)?)?;
    // 0x23938
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2393c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x23940
    c.bl(0x2ec94)?;
    // 0x23944
    c.literal(R::R1,0x23c7c)?;
    // 0x23948
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2394c
    c.bl(0x2ec94)?;
    // 0x23950
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x23954
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,8192)?)?;
    // 0x23958
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // 0x2395c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x23960
    c.b(C::Eq,0x23994)?;
    // 0x23964
    c.literal(R::R1,0x23c94)?;
    // 0x23968
    c.literal(R::R2,0x23c78)?;
    // 0x2396c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,740)?)?;
    // 0x23970
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x23974
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23978
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2397c
    c.bl(0x2ec94)?;
    // 0x23980
    c.literal(R::R1,0x23c7c)?;
    // 0x23984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23988
    c.bl(0x2ec94)?;
    // 0x2398c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x23990
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,8192)?)?;
    // 0x23994
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,72)?)?;
    // 0x23998
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,64)?)?;
    // 0x2399c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x239a0
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x239a4
    c.b(C::Ls,0x239dc)?;
    // 0x239a8
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x239ac
    c.literal(R::R1,0x23c98)?;
    // 0x239b0
    c.literal(R::R2,0x23c78)?;
    // 0x239b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,233)?)?;
    // 0x239b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x239bc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x239c0
    c.bl(0x2ec94)?;
    // 0x239c4
    c.literal(R::R1,0x23c7c)?;
    // 0x239c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x239cc
    c.bl(0x2ec94)?;
    // 0x239d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,64)?)?;
    // 0x239d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,5)?)?;
    // 0x239d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,8192)?)?;
    // 0x239dc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,36)?)?;
    // 0x239e0
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsl,3)?)?;
    // 0x239e4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x239e8
    c.b(C::Cc,0x23a10)?;
    // 0x239ec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x239f0
    c.b(C::Hi,0x23a10)?;
    // 0x239f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,72)?)?;
    // 0x239f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x239fc
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,3)?)?;
    // 0x23a00
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23a04
    c.b(C::Cc,0x23a10)?;
    // 0x23a08
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23a0c
    c.b(C::Ls,0x23a3c)?;
    // 0x23a10
    c.literal(R::R1,0x23c9c)?;
    // 0x23a14
    c.literal(R::R2,0x23c78)?;
    // 0x23a18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,239)?)?;
    // 0x23a1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23a20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x23a24
    c.bl(0x2ec94)?;
    // 0x23a28
    c.literal(R::R1,0x23c7c)?;
    // 0x23a2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23a30
    c.bl(0x2ec94)?;
    // 0x23a34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x23a38
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,8192)?)?;
    // 0x23a3c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x23a40
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x23a44
    c.b(C::Eq,0x23874)?;
    // 0x23a48
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,60)?)?;
    // 0x23a4c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x23a50
    c.b(C::Eq,0x23a84)?;
    // 0x23a54
    c.literal(R::R1,0x23ca0)?;
    // 0x23a58
    c.literal(R::R2,0x23c78)?;
    // 0x23a5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,760)?)?;
    // 0x23a60
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x23a64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23a68
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x23a6c
    c.bl(0x2ec94)?;
    // 0x23a70
    c.literal(R::R1,0x23c7c)?;
    // 0x23a74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23a78
    c.bl(0x2ec94)?;
    // 0x23a7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x23a80
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,8192)?)?;
    // 0x23a84
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,76)?)?;
    // 0x23a88
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // 0x23a8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x23a90
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,1920)?)?;
    // 0x23a94
    c.b(C::Ls,0x23acc)?;
    // 0x23a98
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x23a9c
    c.literal(R::R1,0x23c98)?;
    // 0x23aa0
    c.literal(R::R2,0x23c78)?;
    // 0x23aa4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,251)?)?;
    // 0x23aa8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23aac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x23ab0
    c.bl(0x2ec94)?;
    // 0x23ab4
    c.literal(R::R1,0x23c7c)?;
    // 0x23ab8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23abc
    c.bl(0x2ec94)?;
    // 0x23ac0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // 0x23ac4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,5)?)?;
    // 0x23ac8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,8192)?)?;
    // 0x23acc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,64)?)?;
    // 0x23ad0
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,3)?)?;
    // 0x23ad4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23ad8
    c.b(C::Cc,0x23b00)?;
    // 0x23adc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23ae0
    c.b(C::Hi,0x23b00)?;
    // 0x23ae4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,72)?)?;
    // 0x23ae8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,76)?)?;
    // 0x23aec
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsl,3)?)?;
    // 0x23af0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23af4
    c.b(C::Cc,0x23b00)?;
    // 0x23af8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23afc
    c.b(C::Ls,0x23874)?;
    // 0x23b00
    c.literal(R::R1,0x23c9c)?;
    // 0x23b04
    c.literal(R::R2,0x23c78)?;
    // 0x23b08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23b0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1792)?)?;
    // 0x23b10
    c.b(C::Al,0x23c28)?;
    // 0x23b14
    c.literal(R::R1,0x23c68)?;
    // 0x23b18
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23b1c
    c.b(C::Eq,0x23874)?;
    // 0x23b20
    c.literal(R::R1,0x23c6c)?;
    // 0x23b24
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23b28
    c.b(C::Eq,0x23c04)?;
    // 0x23b2c
    c.literal(R::R1,0x23c70)?;
    // 0x23b30
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23b34
    c.b(C::Ne,0x23c44)?;
    // 0x23b38
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,56)?)?;
    // 0x23b3c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x23b40
    c.b(C::Eq,0x23874)?;
    // 0x23b44
    c.literal(R::new(9)?,0x23c78)?;
    // 0x23b48
    c.literal(R::new(10)?,0x23c7c)?;
    // 0x23b4c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,160)?)?;
    // 0x23b50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,6)?)?;
    // 0x23b54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x23b58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x23b5c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,8192)?)?;
    // 0x23b60
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(7)?,4)?)?;
    // 0x23b64
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Ror,2)?)?;
    // 0x23b68
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x23b6c
    c.b(C::Cc,0x23ba0)?;
    // 0x23b70
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x23b74
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x23b78
    c.literal(R::R1,0x23c84)?;
    // 0x23b7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23b80
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x23b84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1840)?)?;
    // 0x23b88
    c.bl(0x2ec94)?;
    // 0x23b8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23b90
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x23b94
    c.bl(0x2ec94)?;
    // 0x23b98
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,56)?)?;
    // 0x23b9c
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x23ba0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x23ba4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x23ba8
    c.b(C::Cc,0x23b60)?;
    // 0x23bac
    c.b(C::Al,0x23874)?;
    // 0x23bb0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // 0x23bb4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Ror,2)?)?;
    // 0x23bb8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x23bbc
    c.b(C::Cc,0x23874)?;
    // 0x23bc0
    c.literal(R::R1,0x23c80)?;
    // 0x23bc4
    c.literal(R::R2,0x23c78)?;
    // 0x23bc8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x23bcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,62)?)?;
    // 0x23bd0
    c.b(C::Al,0x23bf8)?;
    // 0x23bd4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // 0x23bd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,239)?)?;
    // 0x23bdc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x23be0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x23be4
    c.b(C::Cc,0x23874)?;
    // 0x23be8
    c.literal(R::R1,0x23c88)?;
    // 0x23bec
    c.literal(R::R2,0x23c78)?;
    // 0x23bf0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,17)?)?;
    // 0x23bf4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x23bf8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // 0x23bfc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23c00
    c.b(C::Al,0x23c28)?;
    // 0x23c04
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // 0x23c08
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Ror,2)?)?;
    // 0x23c0c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x23c10
    c.b(C::Cc,0x23874)?;
    // 0x23c14
    c.literal(R::R1,0x23c74)?;
    // 0x23c18
    c.literal(R::R2,0x23c78)?;
    // 0x23c1c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x23c20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23c24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1872)?)?;
    // 0x23c28
    c.bl(0x2ec94)?;
    // 0x23c2c
    c.literal(R::R1,0x23c7c)?;
    // 0x23c30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x23c34
    c.bl(0x2ec94)?;
    // 0x23c38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x23c3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,8192)?)?;
    // 0x23c40
    c.b(C::Al,0x23874)?;
    // 0x23c44
    c.literal(R::R2,0x23ca4)?;
    // 0x23c48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,99)?)?;
    // 0x23c4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x23c50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x23c54
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x23c58
    c.bl(0x17b0)?;
    // 0x23c5c
    c.b(C::Al,0x23874)?;
    c.check(0x23c60)?;
    for value in CONTROLLER_WORDS {c.emit(value)?;}
    c.check(0x23ca8)
}

/// Publish a checked four-register message into the platform command queue.
fn publish_buffer_message(c:&mut Code)->Result<(),String>{
    c.check(0x23ca8)?;
    // 0x23ca8
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    // 0x23cac
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x23cb0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x23cb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x23cb8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,135168)?)?;
    // 0x23cbc
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,8)?)?;
    // 0x23cc0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::SP,12)?)?;
    // 0x23cc4
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x23cc8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // 0x23ccc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,12)?)?;
    // 0x23cd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x23cd4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x23cd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // 0x23cdc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x23ce0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // 0x23ce4
    c.bl(0x2a198)?;
    // 0x23ce8
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // 0x23cec
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x23cf0)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    process_buffer_allocation(c)?;
    publish_buffer_message(c)?;
    Ok(())
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn buffer_message_layout_and_literal_table(){
        assert_eq!(FRAME_CLASS_TARGETS.len(),7);
        assert_eq!(CONTROLLER_WORDS.len(),18);
        let mut c=Code::new(0x23840);
        emit(&mut c).unwrap();
        assert_eq!(c.at(),0x23cf0);
        assert_eq!(c.bytes.len(),0x4b0);
        for (idx,value) in FRAME_CLASS_TARGETS.iter().enumerate(){
            let offset=0x238ac-0x23840+idx*4;
            assert_eq!(u32::from_le_bytes(c.bytes[offset..offset+4].try_into().unwrap()),*value);
        }
    }
}
