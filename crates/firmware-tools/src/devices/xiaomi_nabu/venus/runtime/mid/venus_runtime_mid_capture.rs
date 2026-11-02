// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec capture input state, buffer type transitions and mode event
//! notification, with checked ARM32 ISA operations and separate debug words.
use arm32::{Condition as C,DataOp as Op,Register as R,BlockMode};
use super::Code;
const STREAM_CAPTURE_DIAGNOSTICS:[u32;14]=[
    0x000e8092, 0x000e2788, 0x000e8082, 0x000e81ce, 0x000e81b7,
    0x000e819f, 0x000e810f, 0x000e8149, 0x000e816d, 0x000e80f7,
    0x000e80a1, 0x000e80c5, 0x000e6a43, 0x000e81eb,
];

fn update_stream_capture_state(c:&mut Code)->Result<(),String>{
    c.check(0x253fc)?;
    // 0x253fc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x25400
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x25404
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x25408
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x2540c
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x25410
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x25414
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x25418
    c.b(C::Eq,0x25440)?;
    // 0x2541c
    c.literal(R::R1,0x256c8)?;
    // 0x25420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25424
    c.bl(0x2ec94)?;
    // 0x25428
    c.literal(R::R1,0x256cc)?;
    // 0x2542c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25430
    c.bl(0x2ec94)?;
    // 0x25434
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,140)?)?;
    // 0x25438
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(6)?,192)?)?;
    // 0x2543c
    c.b(C::Al,0x25460)?;
    // 0x25440
    c.literal(R::R1,0x256d0)?;
    // 0x25444
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25448
    c.bl(0x2ec94)?;
    // 0x2544c
    c.literal(R::R1,0x256cc)?;
    // 0x25450
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25454
    c.bl(0x2ec94)?;
    // 0x25458
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,152)?)?;
    // 0x2545c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(6)?,204)?)?;
    // 0x25460
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,16)?)?;
    // 0x25464
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,8)?)?;
    // 0x25468
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,12)?)?;
    // 0x2546c
    c.literal(R::new(7)?,0x256cc)?;
    // 0x25470
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x25474
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x25478
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x2547c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,0)?)?;
    // 0x25480
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // 0x25484
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x25488
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2548c
    c.b(C::Eq,0x254cc)?;
    // 0x25490
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x25494
    c.b(C::Eq,0x25500)?;
    // 0x25498
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2549c
    c.b(C::Ne,0x25578)?;
    // 0x254a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x254a4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x254a8
    c.bl(0x80994)?;
    // 0x254ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x254b0
    c.b(C::Eq,0x2558c)?;
    // 0x254b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x254b8
    c.literal(R::R1,0x256ec)?;
    // 0x254bc
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(9)?,R::R0.bit() | R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x254c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x254c4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // 0x254c8
    c.b(C::Al,0x25528)?;
    // 0x254cc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x254d0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x254d4
    c.bl(0x80994)?;
    // 0x254d8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x254dc
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,0)?)?;
    // 0x254e0
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,4)?)?;
    // 0x254e4
    c.b(C::Ne,0x255e0)?;
    // 0x254e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x254ec
    c.literal(R::R1,0x256d8)?;
    // 0x254f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // 0x254f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // 0x254f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x254fc
    c.b(C::Al,0x25564)?;
    // 0x25500
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x25504
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x25508
    c.bl(0x80994)?;
    // 0x2550c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x25510
    c.b(C::Eq,0x25608)?;
    // 0x25514
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,0)?)?;
    // 0x25518
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,4)?)?;
    // 0x2551c
    c.literal(R::R1,0x256dc)?;
    // 0x25520
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x25524
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2)?)?;
    // 0x25528
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // 0x2552c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25530
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25534
    c.bl(0x2ec94)?;
    // 0x25538
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2553c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x25540
    c.bl(0x2ec94)?;
    // 0x25544
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x25548
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x2554c
    c.b(C::Ne,0x2557c)?;
    // 0x25550
    c.literal(R::R1,0x256e0)?;
    // 0x25554
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x25558
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x2555c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25560
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,4)?)?;
    // 0x25564
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25568
    c.bl(0x2ec94)?;
    // 0x2556c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25570
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x25574
    c.bl(0x2ec94)?;
    // 0x25578
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2557c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x25580
    c.b(C::Ne,0x25484)?;
    // 0x25584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x25588
    c.b(C::Al,0x25674)?;
    // 0x2558c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x25590
    c.literal(R::R1,0x256f0)?;
    // 0x25594
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25598
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x2559c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x255a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4)?)?;
    // 0x255a4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x255a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x255ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x255b0
    c.bl(0x2ec94)?;
    // 0x255b4
    c.literal(R::R1,0x256cc)?;
    // 0x255b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x255bc
    c.bl(0x2ec94)?;
    // 0x255c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x255c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x255c8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x255cc
    c.b(C::Ne,0x25674)?;
    // 0x255d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x255d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(9)?,0)?)?;
    // 0x255d8
    c.literal(R::R1,0x256f4)?;
    // 0x255dc
    c.b(C::Al,0x25654)?;
    // 0x255e0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x255e4
    c.literal(R::R1,0x256d4)?;
    // 0x255e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x255ec
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x255f0
    c.bl(0x2ec94)?;
    // 0x255f4
    c.literal(R::R1,0x256cc)?;
    // 0x255f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x255fc
    c.bl(0x2ec94)?;
    // 0x25600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x25604
    c.b(C::Al,0x25674)?;
    // 0x25608
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x2560c
    c.literal(R::R1,0x256e4)?;
    // 0x25610
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25614
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x25618
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4)?)?;
    // 0x2561c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x25620
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x25624
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25628
    c.bl(0x2ec94)?;
    // 0x2562c
    c.literal(R::R1,0x256cc)?;
    // 0x25630
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25634
    c.bl(0x2ec94)?;
    // 0x25638
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x2563c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x25640
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x25644
    c.b(C::Ne,0x25674)?;
    // 0x25648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2564c
    c.literal(R::R1,0x256e8)?;
    // 0x25650
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,0)?)?;
    // 0x25654
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,4)?)?;
    // 0x25658
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x2565c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25660
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x25664
    c.bl(0x2ec94)?;
    // 0x25668
    c.literal(R::R1,0x256cc)?;
    // 0x2566c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x25670
    c.bl(0x2ec94)?;
    // 0x25674
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x25678
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2567c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x25680
    c.b(C::Ne,0x256bc)?;
    // 0x25684
    c.literal(R::R2,0x256f8)?;
    // 0x25688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,118)?)?;
    // 0x2568c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x25690
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2816)?)?;
    // 0x25694
    c.bl(0x17b0)?;
    // 0x25698
    c.literal(R::R1,0x256fc)?;
    // 0x2569c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x256a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x256a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // 0x256a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x256ac
    c.bl(0x2ec94)?;
    // 0x256b0
    c.literal(R::R1,0x256cc)?;
    // 0x256b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x256b8
    c.bl(0x2ec94)?;
    // 0x256bc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x256c0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x256c4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x256c8)?;
    for value in STREAM_CAPTURE_DIAGNOSTICS {c.emit(value)?;}
    c.check(0x25700)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{update_stream_capture_state(c)}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn capture_mode_state_is_contiguous(){
        let mut c=Code::new(0x253fc);
        emit(&mut c).unwrap();
        assert_eq!(c.at(),0x25700);
        assert_eq!(c.bytes.len(),0x304);
        assert_eq!(STREAM_CAPTURE_DIAGNOSTICS.len(),14);
    }
}
