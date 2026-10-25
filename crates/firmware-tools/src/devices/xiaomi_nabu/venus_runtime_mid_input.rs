// SPDX-License-Identifier: GPL-3.0-only
//! Codec input vector packing, controller format record registration,
//! channel flag/state probes and diagnostic call paths.
//! Data words are kept separate from executable operations.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

pub(super) fn emit_next(c:&mut Code)->Result<(),String>{
    c.check(0x2265c)?;
    // 0x2265c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x22660
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x22664
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x22668
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,4)?)?;
    // 0x2266c
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::new(3)?,0)?)?;
    // 0x22670
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x22674
    c.emit(arm32::load_halfword(C::Al,R::R12,R::R2,0)?)?;
    // 0x22678
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,12)?)?;
    // 0x2267c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x22680
    c.b(C::Lt,0x22724)?;
    // 0x22684
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,16)?)?;
    // 0x22688
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,8)?)?;
    // 0x2268c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x22690
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x22694
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,32)?)?;
    // 0x22698
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x2269c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x226a0
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R2,R::R1,4)?)?;
    // 0x226a4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsl,16)?)?;
    // 0x226a8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(9)?,R::new(7)?,Shift::Asr,16,false)?)?;
    // 0x226ac
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x226b0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R2,R::LR,Shift::Lsl,0,false)?)?;
    // 0x226b4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R12,Shift::Lsl,16)?)?;
    // 0x226b8
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::R2,Shift::Asr,16)?)?;
    // 0x226bc
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x226c0
    c.b(C::Ge,0x226f0)?;
    // 0x226c4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,16)?)?;
    // 0x226c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,1)?)?;
    // 0x226cc
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(4)?,R::new(3)?,Shift::Asr,16,false)?)?;
    // 0x226d0
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Asr,16)?)?;
    // 0x226d4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x226d8
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R2,R::R2,R::new(6)?,Shift::Asr,R::new(7)?,false)?)?;
    // 0x226dc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x226e0
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R12,Shift::Lsl,16)?)?;
    // 0x226e4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,16)?)?;
    // 0x226e8
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x226ec
    c.b(C::Al,0x22718)?;
    // 0x226f0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,R::new(4)?,false)?)?;
    // 0x226f4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(7)?,Shift::Asr,16)?)?;
    // 0x226f8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x226fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x22700
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x22704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65280)?)?;
    // 0x22708
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2270c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // 0x22710
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x22714
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R12,R::R12,1)?)?;
    // 0x22718
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x2271c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x22720
    c.b(C::Ne,0x226a0)?;
    // 0x22724
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x22728
    c.emit(arm32::store_halfword(C::Al,R::R12,R::R0,0)?)?;
    // 0x2272c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x22730
    c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::R0,0)?)?;
    // 0x22734
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x22738
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2273c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,580)?)?;
    // 0x22740
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x22744
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x22748
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x2274c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7744)?)?;
    // 0x22750
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x22754
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,16)?)?;
    // 0x22758
    c.literal(R::R0,0x227dc)?;
    // 0x2275c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x22760
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x22764
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,6)?)?;
    // 0x22768
    c.b(C::Ne,0x22780)?;
    // 0x2276c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,37)?)?;
    // 0x22770
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x22774
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x22778
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,262144)?)?;
    // 0x2277c
    c.b(C::Al,0x2279c)?;
    // 0x22780
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x22784
    c.b(C::Eq,0x227b4)?;
    // 0x22788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,37)?)?;
    // 0x2278c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x22790
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x22794
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,252)?)?;
    // 0x22798
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,256)?)?;
    // 0x2279c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x227a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,53)?)?;
    // 0x227a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x227a8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x227ac
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x227b0
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x227b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x227b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x227bc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x227c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,262144)?)?;
    // 0x227c4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    // 0x227c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,21)?)?;
    // 0x227cc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x227d0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x227d4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x227d8
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x227dc)?;
    const INPUT_CONTROLLER_ID:u32=0xe000_9620;
    c.emit(INPUT_CONTROLLER_ID)?;
    c.check(0x227e0)?;
    // 0x227e0
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x227e4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x227e8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x227ec
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x227f0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x227f4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,532)?)?;
    // 0x227f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,3616)?)?;
    // 0x227fc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22800
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,7168)?)?;
    // 0x22804
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R0,4096)?)?;
    // 0x22808
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,540)?)?;
    // 0x2280c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22810
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R1,0)?)?;
    // 0x22814
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,3600)?)?;
    // 0x22818
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4096)?)?;
    // 0x2281c
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R0,7168)?)?;
    // 0x22820
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,552)?)?;
    // 0x22824
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // 0x22828
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,556)?)?;
    // 0x2282c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x22830
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x22834
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x22838
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // 0x2283c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x22840
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x22844
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x22848
    c.literal(R::R1,0x229d8)?;
    // 0x2284c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22850
    c.bl(0x2ec94)?;
    // 0x22854
    c.literal(R::R1,0x229dc)?;
    // 0x22858
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2285c
    c.bl(0x2ec94)?;
    // 0x22860
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,5)?)?;
    // 0x22864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,37)?)?;
    // 0x22868
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,38400)?)?;
    // 0x2286c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x22870
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x22874
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,4096)?)?;
    // 0x22878
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,4096)?)?;
    // 0x2287c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,262144)?)?;
    // 0x22880
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,3758096384)?)?;
    // 0x22884
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x22888
    c.emit(arm32::mov_register(C::Ne,R::R0,R::new(10)?))?;
    // 0x2288c
    c.b(C::Eq,0x228fc)?;
    // 0x22890
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,53)?)?;
    // 0x22894
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,4)?)?;
    // 0x22898
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,0)?)?;
    // 0x2289c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,132)?)?;
    // 0x228a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x228a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x228a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16384)?)?;
    // 0x228ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x228b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,128)?)?;
    // 0x228b4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,4)?)?;
    // 0x228b8
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,0)?)?;
    // 0x228bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,132)?)?;
    // 0x228c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,128)?)?;
    // 0x228c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,580)?)?;
    // 0x228c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x228cc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x228d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7744)?)?;
    // 0x228d4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x228d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,252)?)?;
    // 0x228dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x228e0
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,36)?)?;
    // 0x228e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x228e8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,262144)?)?;
    // 0x228ec
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(6)?,R::new(6)?,256)?)?;
    // 0x228f0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,32)?)?;
    // 0x228f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,36)?)?;
    // 0x228f8
    c.b(C::Al,0x229b0)?;
    // 0x228fc
    c.literal(R::R1,0x229e0)?;
    // 0x22900
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // 0x22904
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,8)?)?;
    // 0x22908
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,140)?)?;
    // 0x2290c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22910
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // 0x22914
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16384)?)?;
    // 0x22918
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,136)?)?;
    // 0x2291c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22920
    c.bl(0x2ec94)?;
    // 0x22924
    c.literal(R::new(7)?,0x229dc)?;
    // 0x22928
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2292c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x22930
    c.bl(0x2ec94)?;
    // 0x22934
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,21)?)?;
    // 0x22938
    c.literal(R::R1,0x229e0)?;
    // 0x2293c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // 0x22940
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x22944
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // 0x22948
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,8)?)?;
    // 0x2294c
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x22950
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22954
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16384)?)?;
    // 0x22958
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,140)?)?;
    // 0x2295c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,136)?)?;
    // 0x22960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22964
    c.bl(0x2ec94)?;
    // 0x22968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2296c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x22970
    c.bl(0x2ec94)?;
    // 0x22974
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,580)?)?;
    // 0x22978
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2297c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x22980
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x22984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7744)?)?;
    // 0x22988
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x2298c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x22990
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x22994
    c.b(C::Ne,0x229b8)?;
    // 0x22998
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,262144)?)?;
    // 0x2299c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,36)?)?;
    // 0x229a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,32)?)?;
    // 0x229a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,53)?)?;
    // 0x229a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x229ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,36)?)?;
    // 0x229b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,32)?)?;
    // 0x229b4
    c.b(C::Al,0x229cc)?;
    // 0x229b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,262144)?)?;
    // 0x229bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,44)?)?;
    // 0x229c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,40)?)?;
    // 0x229c4
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,44)?)?;
    // 0x229c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,40)?)?;
    // 0x229cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x229d0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x229d4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x229d8)?;
    const CHANNEL_DIAGNOSTICS:[u32;3]=[0x000e_6c6b,0x000e_2788,0x000e_3dfc];
    for value in CHANNEL_DIAGNOSTICS {c.emit(value)?;}
    c.check(0x229e4)?;
    // 0x229e4
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    // 0x229e8
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x229ec
    c.literal(R::R12,0x22a3c)?;
    // 0x229f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x229f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // 0x229f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x229fc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,256)?)?;
    // 0x22a00
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x22a04
    c.emit(arm32::mov_register(C::Eq,R::new(3)?,R::R2))?;
    // 0x22a08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,712)?)?;
    // 0x22a0c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x22a10
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x22a14
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R12,0)?)?;
    // 0x22a18
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x22a1c
    c.literal(R::R2,0x22a40)?;
    // 0x22a20
    c.emit(arm32::data_immediate(C::Eq,Op::Mvn,R::R2,R::R0,49)?)?;
    // 0x22a24
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::LR,4)?)?;
    // 0x22a28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x22a2c
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x22a30
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::R1,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x22a34
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x22a38
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x22a3c)?;
    const CODEC_LAYOUT_TABLE:[u32;2]=[0xe000_8060,0xffff_fcfd];
    for value in CODEC_LAYOUT_TABLE {c.emit(value)?;}
    c.check(0x22a44)
}
