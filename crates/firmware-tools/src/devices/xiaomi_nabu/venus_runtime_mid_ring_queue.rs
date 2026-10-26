// SPDX-License-Identifier: GPL-3.0-only
//! Controller 32-entry ring: append and dequeue request descriptors and
//! toggle the owner synchronization callbacks. A32 STRD stores the paired
//! buffer descriptor words as a single 64-bit write. All bytes are encoded
//! from checked architecture operands, not proprietary opcode literals.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x35670;
const END:usize=0x3580c;
pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x35670: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35674: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x35678: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x3567c: ldr r7, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,8)?)?;
    // 0x35680: mov r5, r3
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // 0x35684: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x35688: ldr r0, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,396)?)?;
    // 0x3568c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35690: beq 0x35698 <entry+0x20c>   @ imm = #0x0
    c.b(C::Eq,0x35698)?;
    // 0x35694: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x32aa8
    c.bl(0x2bf4)?;
    // 0x35698: ldr r0, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,388)?)?;
    // 0x3569c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x356a0: str r6, [r1, r0, lsl #3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,3)?)?;
    // 0x356a4: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x356a8: str r5, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,4)?)?;
    // 0x356ac: str r7, [r0, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,256)?)?;
    // 0x356b0: ldr r0, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,388)?)?;
    // 0x356b4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x356b8: str r0, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,388)?)?;
    // 0x356bc: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x356c0: ldr r1, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,392)?)?;
    // 0x356c4: movhs r0, #0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x356c8: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x356cc: str r1, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,392)?)?;
    // 0x356d0: strhs r0, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,388)?)?;
    // 0x356d4: ldr r0, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,396)?)?;
    // 0x356d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x356dc: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x356e0: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x356e4: b 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x32ab8
    c.b(C::Al,0x2c34)?;
    // 0x356e8: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x356ec: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x356f0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x356f4: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x356f8: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x356fc: ldr r0, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,396)?)?;
    // 0x35700: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35704: beq 0x3570c <entry+0x280>   @ imm = #0x0
    c.b(C::Eq,0x3570c)?;
    // 0x35708: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x32b1c
    c.bl(0x2bf4)?;
    // 0x3570c: ldr r0, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,392)?)?;
    // 0x35710: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35714: beq 0x35764 <entry+0x2d8>   @ imm = #0x48
    c.b(C::Eq,0x35764)?;
    // 0x35718: ldr r0, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,384)?)?;
    // 0x3571c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x35720: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x35724: ldr r2, [r1, r0, lsl #3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,3)?)?;
    // 0x35728: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x3572c: ldr r3, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,4)?)?;
    // 0x35730: strd r2, r3, [r7]
    c.emit(arm32::transfer_doubleword(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x35734: ldr r0, [r0, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,256)?)?;
    // 0x35738: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x3573c: ldr r0, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,384)?)?;
    // 0x35740: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x35744: str r0, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,384)?)?;
    // 0x35748: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x3574c: ldr r1, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,392)?)?;
    // 0x35750: movhs r0, #0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35754: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x35758: str r1, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,392)?)?;
    // 0x3575c: strhs r0, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,384)?)?;
    // 0x35760: b 0x35774 <entry+0x2e8>   @ imm = #0xc
    c.b(C::Al,0x35774)?;
    // 0x35764: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x35768: str r6, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,0)?)?;
    // 0x3576c: str r6, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,4)?)?;
    // 0x35770: str r6, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,0)?)?;
    // 0x35774: ldr r0, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,396)?)?;
    // 0x35778: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3577c: beq 0x35784 <entry+0x2f8>   @ imm = #0x0
    c.b(C::Eq,0x35784)?;
    // 0x35780: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x32b54
    c.bl(0x2c34)?;
    // 0x35784: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x35788: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3578c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35790: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x35794: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x35798: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x3579c: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x357a0: ldr r0, [r5, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,396)?)?;
    // 0x357a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x357a8: beq 0x357b0 <entry+0x324>   @ imm = #0x0
    c.b(C::Eq,0x357b0)?;
    // 0x357ac: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x32bc0
    c.bl(0x2bf4)?;
    // 0x357b0: ldr r0, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,392)?)?;
    // 0x357b4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x357b8: beq 0x357e0 <entry+0x354>   @ imm = #0x20
    c.b(C::Eq,0x357e0)?;
    // 0x357bc: ldr r0, [r5, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,384)?)?;
    // 0x357c0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x357c4: ldr r2, [r1, r0, lsl #3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,3)?)?;
    // 0x357c8: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x357cc: ldr r3, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,4)?)?;
    // 0x357d0: strd r2, r3, [r6]
    c.emit(arm32::transfer_doubleword(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x357d4: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x357d8: ldr r0, [r0, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,256)?)?;
    // 0x357dc: b 0x357f0 <entry+0x364>   @ imm = #0xc
    c.b(C::Al,0x357f0)?;
    // 0x357e0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x357e4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x357e8: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x357ec: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x357f0: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x357f4: ldr r0, [r5, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,396)?)?;
    // 0x357f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x357fc: beq 0x35804 <entry+0x378>   @ imm = #0x0
    c.b(C::Eq,0x35804)?;
    // 0x35800: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x32bd4
    c.bl(0x2c34)?;
    // 0x35804: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x35808: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(END)
}

#[cfg(test)]mod tests {
 use super::*;
 #[test]fn compare_full_generated_fragment_to_private_reference_if_set(){
    let Some(path)=std::env::var_os("NABU_VENUS_REFERENCE") else {return;};
    let original=std::fs::read(&path).expect("read opted-in Venus reference");
    let generated=super::super::fragment().expect("emit complete typed region");
    let src=original.get(super::super::FILE_OFFSET..super::super::FILE_OFFSET+generated.len())
        .expect("firmware is shorter than emitted source range");
    assert_eq!(generated,src,"typed reconstruction differs from original MBN");
    if let Some(output)=std::env::var_os("NABU_VENUS_EMIT") {
        std::fs::write(output,&generated).expect("write generated region");
    }
 }
 #[test]fn queue_routines_are_contiguous_and_use_strd(){
   let mut c=Code::new(START);emit(&mut c).unwrap();
   assert_eq!(c.at(),END);
   let word=|v:usize|u32::from_le_bytes(c.bytes[v-START..v-START+4].try_into().unwrap());
   assert_eq!(word(0x35730),arm32::transfer_doubleword(C::Al,false,R::R2,R::new(7).unwrap(),0).unwrap());
   assert_eq!(word(0x357d0),arm32::transfer_doubleword(C::Al,false,R::R2,R::new(6).unwrap(),0).unwrap());
 }
}
