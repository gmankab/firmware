// SPDX-License-Identifier: GPL-3.0-only
//! Independent Venus A32 controller functions in Agent 20-04's exclusive
//! runtime address reservation `[0x40000,0x68000)`.
//!
//! The first complete routine calculates normalized signed 16-bit buffer
//! dimensions, invokes the shared clipping/check helpers and returns a
//! two-bit status mask indicating inadequate width/height. The original
//! function is referenced by the preceding large controller routine at
//! address `0x40008` and has a genuine saved-register return at `0x40c08`.
//!
//! **Source provenance:** the generator below emits public A32 operations
//! using typed registers, conditions, immediate constants and memory modes;
//! no original OEM opcodes are stored here. Code identity is independently
//! verified against a *privately supplied* pinned `venus.mbn` reference.
//! This is ARM instruction-level reconstruction, not proof of recovered
//! high-level source semantics or a bootable/signable firmware image.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};

pub const FILE_BASE:usize=0x4000;
pub const OWNED_START:usize=0x40000;
pub const OWNED_END:usize=0x68000;
pub const FIRST_START:usize=0x40a2c;
pub const FIRST_END:usize=0x40c14;

/// Strict output-position tracker. No implicit padding or opaque reference
/// fragments are emitted between independently rebuilt instructions.
struct Code { at:usize, bytes:Vec<u8> }
impl Code {
    fn new(at:usize)->Self { Self {at, bytes:Vec::new()} }
    fn check(&self,expected:usize)->Result<(),String> {
        if self.at!=expected {return Err(format!("Venus middle layout 0x{:x} != 0x{expected:x}",self.at));}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String> {
        if !(OWNED_START..OWNED_END).contains(&self.at) || self.at & 3 != 0 {
            return Err(format!("Venus middle emitter outside owned range at 0x{:x}",self.at));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at+=4;
        Ok(())
    }
    fn call(&mut self,target:usize)->Result<(),String> {
        self.emit(arm32::branch(C::Al,self.at as u32,target as u32,true)?)
    }
    fn branch(&mut self,condition:C,target:usize)->Result<(),String> {
        self.emit(arm32::branch(condition,self.at as u32,target as u32,false)?)
    }
}

/// Given signed packet size and desired clip limits, normalize two axes via
/// runtime helpers at `0x53e8c` and `0x53f98`, then return an 8-bit mask
/// encoding which axis falls below the hard lower bound (1). The nested
/// branch at `0x40ae0` skips optional clipping; the branch at `0x40bfc`
/// returns success for a zero-sized input rather than fabricating a value.
fn classify_dimensions(c:&mut Code)->Result<(),String>{
    c.check(0x40a2c)?;
    // 0x40a2c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x40a30: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x40a34: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // 0x40a38: mov r10, r2
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R2))?;
    // 0x40a3c: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x40a40: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x40a44: mov r6, r3
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // 0x40a48: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x40a4c: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x40a50: bl 0x81838 <entry+0x41938>
    c.call(0x81838)?;
    // 0x40a54: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x40a58: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x40a5c: beq 0x40bfc <entry+0xcfc>
    c.branch(C::Eq,0x40bfc)?;
    // 0x40a60: sub r9, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(9)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x40a64: ldr r1, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // 0x40a68: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x40a6c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x40a70: add r0, r9, r9, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(9)?,Shift::Lsr,31,false)?)?;
    // 0x40a74: asr r8, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::R0,Shift::Asr,1)?)?;
    // 0x40a78: add r0, r5, r5, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(5)?,Shift::Lsr,31,false)?)?;
    // 0x40a7c: asr r7, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R0,Shift::Asr,1)?)?;
    // 0x40a80: beq 0x40ae0 <entry+0xbe0>
    c.branch(C::Eq,0x40ae0)?;
    // 0x40a84: mov r6, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,16711680)?)?;
    // 0x40a88: add r3, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,24)?)?;
    // 0x40a8c: orr r6, r6, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4278190080)?)?;
    // 0x40a90: add r0, r6, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x40a94: add r4, sp, #26
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::SP,26)?)?;
    // 0x40a98: asr r1, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Asr,16)?)?;
    // 0x40a9c: lsl r0, r9, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(9)?,Shift::Lsl,16)?)?;
    // 0x40aa0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x40aa4: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x40aa8: bl 0x53e8c <entry+0x13f8c>
    c.call(0x53e8c)?;
    // 0x40aac: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x40ab0: add r0, r7, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x40ab4: ldrsh r9, [sp, #26]
    c.emit(arm32::load_signed_halfword(C::Al,R::new(9)?,R::SP,26)?)?;
    // 0x40ab8: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x40abc: add r3, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,24)?)?;
    // 0x40ac0: add r0, r6, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x40ac4: asr r1, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Asr,16)?)?;
    // 0x40ac8: lsl r0, r8, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(8)?,Shift::Lsl,16)?)?;
    // 0x40acc: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x40ad0: bl 0x53e8c <entry+0x13f8c>
    c.call(0x53e8c)?;
    // 0x40ad4: ldr r1, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // 0x40ad8: ldrsh r8, [sp, #26]
    c.emit(arm32::load_signed_halfword(C::Al,R::new(8)?,R::SP,26)?)?;
    // 0x40adc: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x40ae0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x40ae4: beq 0x40be4 <entry+0xce4>
    c.branch(C::Eq,0x40be4)?;
    // 0x40ae8: ldr r1, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,12)?)?;
    // 0x40aec: ldr r0, [r11, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,16)?)?;
    // 0x40af0: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // 0x40af4: add r2, sp, #22
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,22)?)?;
    // 0x40af8: add r3, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,20)?)?;
    // 0x40afc: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x40b00: mov r1, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16711680)?)?;
    // 0x40b04: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x40b08: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x40b0c: add r0, r5, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x40b10: orr r1, r1, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4278190080)?)?;
    // 0x40b14: add r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x40b18: asr r5, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R0,Shift::Asr,16)?)?;
    // 0x40b1c: lsl r0, r9, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(9)?,Shift::Lsl,16)?)?;
    // 0x40b20: asr r4, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R0,Shift::Asr,16)?)?;
    // 0x40b24: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x40b28: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x40b2c: bl 0x53edc <entry+0x13fdc>
    c.call(0x53edc)?;
    // 0x40b30: add r0, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,24)?)?;
    // 0x40b34: add r3, sp, #26
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,26)?)?;
    // 0x40b38: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x40b3c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x40b40: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x40b44: mov r4, r6
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(6)?))?;
    // 0x40b48: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x40b4c: bl 0x53f98 <entry+0x14098>
    c.call(0x53f98)?;
    // 0x40b50: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x40b54: add r0, r10, r10, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::new(10)?,Shift::Lsr,31,false)?)?;
    // 0x40b58: mov r1, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16711680)?)?;
    // 0x40b5c: ldrsh r9, [sp, #26]
    c.emit(arm32::load_signed_halfword(C::Al,R::new(9)?,R::SP,26)?)?;
    // 0x40b60: mov r10, r4
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(4)?))?;
    // 0x40b64: add r2, sp, #22
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,22)?)?;
    // 0x40b68: add r3, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,20)?)?;
    // 0x40b6c: asr r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,1)?)?;
    // 0x40b70: orr r1, r1, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4278190080)?)?;
    // 0x40b74: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x40b78: ldr r0, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,12)?)?;
    // 0x40b7c: stmib sp, {r0, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(4)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x40b80: add r0, r7, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x40b84: add r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x40b88: asr r4, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R0,Shift::Asr,16)?)?;
    // 0x40b8c: lsl r0, r8, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(8)?,Shift::Lsl,16)?)?;
    // 0x40b90: asr r7, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R0,Shift::Asr,16)?)?;
    // 0x40b94: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x40b98: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x40b9c: bl 0x53edc <entry+0x13fdc>
    c.call(0x53edc)?;
    // 0x40ba0: ldr r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,16)?)?;
    // 0x40ba4: add r0, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,24)?)?;
    // 0x40ba8: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x40bac: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x40bb0: add r3, sp, #26
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,26)?)?;
    // 0x40bb4: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x40bb8: add r0, r9, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x40bbc: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x40bc0: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x40bc4: subgt r5, r6, r9
    c.emit(arm32::data_register(C::Gt,Op::Sub,R::new(5)?,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x40bc8: bl 0x53f98 <entry+0x14098>
    c.call(0x53f98)?;
    // 0x40bcc: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x40bd0: ldrsh r0, [sp, #26]
    c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::SP,26)?)?;
    // 0x40bd4: add r2, r6, r6, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::new(6)?,Shift::Lsr,31,false)?)?;
    // 0x40bd8: add r1, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x40bdc: cmp r1, r2, asr #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Asr,1,false)?)?;
    // 0x40be0: rsbgt r7, r0, r2, asr #1
    c.emit(arm32::data_register(C::Gt,Op::Rsb,R::new(7)?,R::R0,R::R2,Shift::Asr,1,false)?)?;
    // 0x40be4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x40be8: cmp r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // 0x40bec: movlt r0, #1
    c.emit(arm32::data_immediate(C::Lt,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x40bf0: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x40bf4: orrlt r0, r0, #2
    c.emit(arm32::data_immediate(C::Lt,Op::Orr,R::R0,R::R0,2)?)?;
    // 0x40bf8: b 0x40c00 <entry+0xd00>
    c.branch(C::Al,0x40c00)?;
    // 0x40bfc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x40c00: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x40c04: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x40c08: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x40c0c)
}

/// Small immediately adjacent stub: return zero through an ARM BX LR.
fn return_zero(c:&mut Code)->Result<(),String>{
    c.check(0x40c0c)?;
    // 0x40c0c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x40c10: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(FIRST_END)
}

/// Build the complete source-derived `0x40a2c–0x40c14` A32 interval.
pub fn first_fragment()->Result<Vec<u8>,String>{
    let mut code=Code::new(FIRST_START);
    classify_dimensions(&mut code)?;
    return_zero(&mut code)?;
    Ok(code.bytes)
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn segment_and_stub_match_checked_sizes(){
        let bytes=first_fragment().unwrap();
        assert_eq!(bytes.len(),FIRST_END-FIRST_START);
        assert_eq!(FIRST_START+FILE_BASE,0x44a2c);
        assert_eq!(FIRST_END+FILE_BASE,0x44c14);
        assert!(FIRST_START>=OWNED_START && FIRST_END<=OWNED_END);
        // These are independently re-encoded operations, not OEM bytes.
        let at=|vma:usize| u32::from_le_bytes(bytes[vma-FIRST_START..vma-FIRST_START+4].try_into().unwrap());
        assert_eq!(at(FIRST_START),arm32::push_registers(C::Al,
            R::new(4).unwrap().bit()|R::new(5).unwrap().bit()|
            R::new(6).unwrap().bit()|R::new(7).unwrap().bit()|
            R::new(8).unwrap().bit()|R::new(9).unwrap().bit()|
            R::new(10).unwrap().bit()|R::R11.bit()|R::LR.bit()).unwrap());
        assert_eq!(at(0x40c0c),arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0).unwrap());
        assert_eq!(at(0x40c10),arm32::bx(C::Al,R::LR));
    }
}
