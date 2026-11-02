// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec metadata-plane copies, DMA buffer attachment and controller
//! queue-line reset. All instruction fields use typed A32 operands while
//! separately classified literal pools represent MMIO or diagnostic data.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const CODEC_PLANE_RESET:[u32;8]=[
  0x0002_9550,0x000e_832c,0x0002_6534,0xe000_3ce0,
  0x000e_8734,0x000e_2788,0x0010_4090,0x0010_40b4,
];
const CODEC_DMA_BINDINGS:[u32;10]=[
  0x0010_40b4,0xe00e_00b8,0x0010_dec4,0x000e_832c,
  0x0002_95b0,0x0002_65e4,0xe00e_10c4,0x000e_869e,
  0x000e_8766,0x000e_2788,
];
const CODEC_QUEUE_CHANNELS:[u32;4]=[0x0002_95b0,0x000e_832c,0x0002_65e4,0x0010_40b4];
const CODEC_REGISTER_DIAGNOSTICS:[u32;2]=[0x000e_8783,0x000e_2788];

/// Reset queued codec plane registers, copy the active three-plane metadata and notify the controller.
fn reset_codec_plane_registers(c:&mut Code)->Result<(),String>{
    c.check(0x280f0)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,168)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x28290)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x2815c)?;
    c.literal_cond(C::Al,R::R2,0x28294)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,460)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.bl(0x17b0)?;
    c.literal_cond(C::Al,R::R1,0x28298)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(4)?,Shift::Lsl,15)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x28190)?;
    c.literal_cond(C::Al,R::R2,0x28294)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,211)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    c.bl(0x17b0)?;
    c.literal_cond(C::Al,R::R0,0x2829c)?;
    c.literal_cond(C::Al,R::new(7)?,0x282a4)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,-48)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::new(8)?,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R0,-32)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::new(9)?.bit() | R::new(10)?.bit(),BlockMode::IncrementAfter,false)?)?;
    c.literal_cond(C::Al,R::R1,0x282a0)?;
    c.bl(0x2ec94)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    c.bl(0x2ec94)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::new(9)?,R::R0,68)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::new(10)?,R::R0,100)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,2)?)?;
    c.b(C::Ne,0x281a8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4128768)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,3152)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,12288)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    c.literal_cond(C::Al,R::R2,0x282a8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(5)?,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R2,R::new(3)?,2384)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R2,R::R2,12288)?)?;
    c.emit(arm32::transfer_word_register(C::Eq,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(3)?,8)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,96)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,100)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,104)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,108)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    c.bl(0x26de0)?;
    c.literal_cond(C::Al,R::new(4)?,0x282ac)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.bl(0x2c34)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,172)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;
    c.check(0x28290)?;
    for word in CODEC_PLANE_RESET{c.emit(word)?;}
    c.check(0x282b0)
}

/// Attach primary codec DMA buffer handles and update hardware/host descriptor statistics.
fn attach_codec_dma_buffers(c:&mut Code)->Result<(),String>{
    c.check(0x282b0)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    c.literal_cond(C::Al,R::new(9)?,0x28450)?;
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(3)?))?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::new(6)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    c.bl(0x26af8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    c.literal_cond(C::Al,R::new(10)?,0x28454)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,32)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    c.bl(0x273a4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.literal_cond(C::Al,R::new(4)?,0x28458)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,199)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8192)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(8)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,580)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,584)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mvn,R::R2,R::R0,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mvn,R::R2,R::R0,2)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::new(6)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x28390)?;
    c.literal_cond(C::Al,R::R2,0x2845c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,372)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x283a0)?;
    c.literal_cond(C::Al,R::R1,0x28460)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::new(6)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,8)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::new(6)?,Shift::Lsl,2)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(6)?,Shift::Lsl,15)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,92)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x283cc)?;
    c.literal_cond(C::Al,R::R2,0x2845c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,380)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x283dc)?;
    c.literal_cond(C::Al,R::R1,0x28464)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,92)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::new(6)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,88)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    c.bl(0x31998)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    c.bl(0x2fac8)?;
    c.literal_cond(C::Al,R::R0,0x28468)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x28418)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,3936)?)?;
    c.b(C::Al,0x28448)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    c.b(C::Ne,0x28448)?;
    c.literal_cond(C::Al,R::R1,0x2846c)?;
    c.literal_cond(C::Al,R::R2,0x28470)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2736)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x28474)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2ec94)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x28450)?;
    for word in CODEC_DMA_BINDINGS{c.emit(word)?;}
    c.check(0x28478)
}

/// Clear a hardware queue line and validate the queue selector and corresponding status channel.
fn clear_codec_controller_line(c:&mut Code)->Result<(),String>{
    c.check(0x28478)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,584)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,199)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x28510)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x284c8)?;
    c.literal_cond(C::Al,R::R2,0x28514)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,396)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.bl(0x17b0)?;
    c.literal_cond(C::Al,R::R1,0x28518)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x284f4)?;
    c.literal_cond(C::Al,R::R2,0x28514)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,147)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x26de0)?;
    c.literal_cond(C::Al,R::R0,0x2851c)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;
    c.check(0x28510)?;
    for word in CODEC_QUEUE_CHANNELS{c.emit(word)?;}
    c.check(0x28520)
}

/// Read hardware stream metadata and dispatch a two-message diagnostic trace.
fn read_codec_register_info(c:&mut Code)->Result<(),String>{
    c.check(0x28520)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,536)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x28560)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x28564)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.bl(0x2ec94)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x28560)?;
    for word in CODEC_REGISTER_DIAGNOSTICS{c.emit(word)?;}
    c.check(0x28568)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    reset_codec_plane_registers(c)?;
    attach_codec_dma_buffers(c)?;
    clear_codec_controller_line(c)?;
    read_codec_register_info(c)?;
    Ok(())
}
