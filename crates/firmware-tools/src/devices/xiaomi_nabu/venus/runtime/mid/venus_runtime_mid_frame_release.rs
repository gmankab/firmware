// SPDX-License-Identifier: GPL-3.0-only
//! Venus active codec frame teardown, MMIO queue clearing and host callback
//! reconciliation. All executable A32 words are created from typed operands
//! rather than copied OEM bytes. Platform pointers remain explicit literals.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const CODEC_FRAME_RECONCILIATION:[u32;8]=[
  0x0010_40cc,0x0002_9608,0x000e_832c,0x0002_9698,
  0x0002_96f0,0x0002_9850,0x0002_6534,0x0010_40b4,
];
const CODEC_FRAME_RELEASE:[u32;7]=[
  0x0010_40b4,0xe00e_00b8,0x0010_dec4,0x000e_832c,
  0x0002_96f0,0x0010_40d4,0x0002_65e4,
];
const CODEC_OUTPUT_RELEASE:[u32;4]=[0x0002_96f0,0x000e_832c,0x0002_65e4,0x0010_40b4];

/// Check codec queue class, arm shared MMIO resources, unregister stale callbacks and notify controller.
fn reconcile_controller_frame(c:&mut Code)->Result<(),String>{
    c.check(0x28c2c)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.literal_cond(C::Al,R::R0,0x28e7c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,64)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.b(C::Ne,0x28cd8)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,168)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,128)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1392)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.b(C::Ne,0x28d38)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    c.b(C::Eq,0x28d58)?;
    c.literal_cond(C::Al,R::R1,0x28e88)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x28db8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,132)?)?;
    c.b(C::Al,0x28da8)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,168)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1392)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x28e80)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x28de4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,77)?)?;
    c.b(C::Al,0x28dd4)?;
    c.literal_cond(C::Al,R::R1,0x28e88)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x28de4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,94)?)?;
    c.b(C::Al,0x28dd4)?;
    c.literal_cond(C::Al,R::R1,0x28e8c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x28d84)?;
    c.literal_cond(C::Al,R::R2,0x28e84)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,114)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,256)?)?;
    c.b(C::Eq,0x28db8)?;
    c.literal_cond(C::Al,R::R1,0x28e88)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x28db8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,122)?)?;
    c.literal_cond(C::Al,R::R2,0x28e84)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.bl(0x17b0)?;
    c.literal_cond(C::Al,R::R1,0x28e90)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x28de4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,140)?)?;
    c.literal_cond(C::Al,R::R2,0x28e84)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,1)?)?;
    c.b(C::Eq,0x28e18)?;
    c.literal_cond(C::Al,R::R1,0x28e94)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x28e18)?;
    c.literal_cond(C::Al,R::R2,0x28e84)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,150)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3152)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4128768)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x26de0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    c.b(C::Ne,0x28e68)?;
    c.literal_cond(C::Al,R::R0,0x28e98)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.bl(0x2c34)?;
    c.literal_cond(C::Al,R::R0,0x28e98)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,172)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;
    c.check(0x28e7c)?;
    for word in CODEC_FRAME_RECONCILIATION{c.emit(word)?;}
    c.check(0x28e9c)
}

/// Close codec control-plane MMIO frame handles while synchronizing host resource flags.
fn release_controller_frame(c:&mut Code)->Result<(),String>{
    c.check(0x28e9c)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    c.literal_cond(C::Al,R::new(8)?,0x28fd4)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(3)?))?;
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x26af8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    c.literal_cond(C::Al,R::new(5)?,0x28fd8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,32)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x273a4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,199)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::new(7)?,0x28fdc)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(4)?,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,580)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,4)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(7)?))?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(9)?,R::new(6)?,8)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,584)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x28f70)?;
    c.literal_cond(C::Al,R::R2,0x28fe0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,183)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x28f80)?;
    c.literal_cond(C::Al,R::R1,0x28fe4)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,8)?)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.literal_cond(C::Al,R::R0,0x28fe8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,92)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x28fc0)?;
    c.literal_cond(C::Al,R::R2,0x28fe0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,194)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x17b0)?;
    c.literal_cond(C::Al,R::R1,0x28fec)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,92)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,88)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x28fd4)?;
    for word in CODEC_FRAME_RELEASE{c.emit(word)?;}
    c.check(0x28ff0)
}

/// Release output buffer queues and request codec queue-state normalization.
fn release_output_channel(c:&mut Code)->Result<(),String>{
    c.check(0x28ff0)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,584)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,199)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x29094)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x29044)?;
    c.literal_cond(C::Al,R::R2,0x29098)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,212)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,1)?)?;
    c.b(C::Eq,0x29078)?;
    c.literal_cond(C::Al,R::R1,0x2909c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x29078)?;
    c.literal_cond(C::Al,R::R2,0x29098)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,221)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x26de0)?;
    c.literal_cond(C::Al,R::R0,0x290a0)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;
    c.check(0x29094)?;
    for word in CODEC_OUTPUT_RELEASE{c.emit(word)?;}
    c.check(0x290a4)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    reconcile_controller_frame(c)?;
    release_controller_frame(c)?;
    release_output_channel(c)?;
    Ok(())
}
