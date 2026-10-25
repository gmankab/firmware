// SPDX-License-Identifier: GPL-3.0-only
//! Copy/convert Venus codec pixel-plane buffers after validating dimensions.
//! A32 instructions are encoded from ARM operand constructors and conditions;
//! error-message and runtime constant addresses are typed as data words.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

const BUFFER_COPY_DIAGNOSTICS:[u32;10]=[
    0x000e7d3f,0x000e7d86,0x000e2788,0x000e7d9b,0x000e7de4,
    0x000e7e06,0x000e7e29,0x000e8210,0x000e7eab,0x000e7e4c,
];

/// Verify destination and input sizes, read indexed codec byte-layout
/// descriptions and marshal packed descriptor data in permitted word widths.
fn copy_codec_plane_records(c:&mut Code)->Result<(),String>{
    c.check(0x24cd4)?;
    // 0x24cd4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x24cd8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x24cdc
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x24ce0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x24ce4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x24ce8
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x24cec
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x24cf0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x24cf4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x24cf8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24cfc
    c.b(C::Ls,0x24d2c)?;
    // 0x24d00
    c.literal(R::R1,0x24f68)?;
    // 0x24d04
    c.literal(R::R2,0x24f6c)?;
    // 0x24d08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,205)?)?;
    // 0x24d0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24d10
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24d14
    c.bl(0x2ec94)?;
    // 0x24d18
    c.literal(R::R1,0x24f70)?;
    // 0x24d1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24d20
    c.bl(0x2ec94)?;
    // 0x24d24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x24d28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x24d2c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(4)?,Shift::Lsl,3,false)?)?;
    // 0x24d30
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x24d34
    c.b(C::Ls,0x24d64)?;
    // 0x24d38
    c.literal(R::R1,0x24f74)?;
    // 0x24d3c
    c.literal(R::R2,0x24f6c)?;
    // 0x24d40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,210)?)?;
    // 0x24d44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24d48
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24d4c
    c.bl(0x2ec94)?;
    // 0x24d50
    c.literal(R::R1,0x24f70)?;
    // 0x24d54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24d58
    c.bl(0x2ec94)?;
    // 0x24d5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x24d60
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x24d64
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,12)?)?;
    // 0x24d68
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,8)?)?;
    // 0x24d6c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,16)?)?;
    // 0x24d70
    c.b(C::Eq,0x24da0)?;
    // 0x24d74
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8)?)?;
    // 0x24d78
    c.b(C::Eq,0x24dd8)?;
    // 0x24d7c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,4)?)?;
    // 0x24d80
    c.b(C::Ne,0x24e24)?;
    // 0x24d84
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,4)?)?;
    // 0x24d88
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(4)?,8)?)?;
    // 0x24d8c
    c.b(C::Eq,0x24e50)?;
    // 0x24d90
    c.literal(R::R1,0x24f78)?;
    // 0x24d94
    c.literal(R::R2,0x24f6c)?;
    // 0x24d98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,219)?)?;
    // 0x24d9c
    c.b(C::Al,0x24e30)?;
    // 0x24da0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,8)?)?;
    // 0x24da4
    c.b(C::Eq,0x24ef8)?;
    // 0x24da8
    c.literal(R::R1,0x24f78)?;
    // 0x24dac
    c.literal(R::R2,0x24f6c)?;
    // 0x24db0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,233)?)?;
    // 0x24db4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24db8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24dbc
    c.bl(0x2ec94)?;
    // 0x24dc0
    c.literal(R::R1,0x24f70)?;
    // 0x24dc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24dc8
    c.bl(0x2ec94)?;
    // 0x24dcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x24dd0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x24dd4
    c.b(C::Al,0x24f20)?;
    // 0x24dd8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,16)?)?;
    // 0x24ddc
    c.b(C::Hi,0x24df4)?;
    // 0x24de0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,272)?)?;
    // 0x24de4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x24de8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65536)?)?;
    // 0x24dec
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::new(4)?,false)?)?;
    // 0x24df0
    c.b(C::Ne,0x24eb8)?;
    // 0x24df4
    c.literal(R::R1,0x24f7c)?;
    // 0x24df8
    c.literal(R::R2,0x24f6c)?;
    // 0x24dfc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,226)?)?;
    // 0x24e00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24e04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24e08
    c.bl(0x2ec94)?;
    // 0x24e0c
    c.literal(R::R1,0x24f70)?;
    // 0x24e10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24e14
    c.bl(0x2ec94)?;
    // 0x24e18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x24e1c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x24e20
    c.b(C::Al,0x24eb8)?;
    // 0x24e24
    c.literal(R::R1,0x24f80)?;
    // 0x24e28
    c.literal(R::R2,0x24f6c)?;
    // 0x24e2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,239)?)?;
    // 0x24e30
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24e34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24e38
    c.bl(0x2ec94)?;
    // 0x24e3c
    c.literal(R::R1,0x24f70)?;
    // 0x24e40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24e44
    c.bl(0x2ec94)?;
    // 0x24e48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x24e4c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x24e50
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,4)?)?;
    // 0x24e54
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(4)?,4)?)?;
    // 0x24e58
    c.b(C::Ne,0x24e84)?;
    // 0x24e5c
    c.literal(R::R2,0x24f84)?;
    // 0x24e60
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x24e64
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x24e68
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x24e6c
    c.b(C::Cs,0x24f64)?;
    // 0x24e70
    c.literal(R::R1,0x24f8c)?;
    // 0x24e74
    c.literal(R::R2,0x24f6c)?;
    // 0x24e78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,249)?)?;
    // 0x24e7c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x24e80
    c.b(C::Al,0x24f48)?;
    // 0x24e84
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,4)?)?;
    // 0x24e88
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(4)?,8)?)?;
    // 0x24e8c
    c.b(C::Ne,0x24eb8)?;
    // 0x24e90
    c.literal(R::R2,0x24f84)?;
    // 0x24e94
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x24e98
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24e9c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,2)?)?;
    // 0x24ea0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x24ea4
    c.b(C::Cs,0x24f64)?;
    // 0x24ea8
    c.literal(R::R1,0x24f8c)?;
    // 0x24eac
    c.literal(R::R2,0x24f6c)?;
    // 0x24eb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x24eb4
    c.b(C::Al,0x24f44)?;
    // 0x24eb8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8)?)?;
    // 0x24ebc
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(4)?,4)?)?;
    // 0x24ec0
    c.b(C::Ne,0x24eec)?;
    // 0x24ec4
    c.literal(R::R2,0x24f84)?;
    // 0x24ec8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x24ecc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24ed0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,1)?)?;
    // 0x24ed4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x24ed8
    c.b(C::Cs,0x24f64)?;
    // 0x24edc
    c.literal(R::R1,0x24f88)?;
    // 0x24ee0
    c.literal(R::R2,0x24f6c)?;
    // 0x24ee4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,9)?)?;
    // 0x24ee8
    c.b(C::Al,0x24f44)?;
    // 0x24eec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8)?)?;
    // 0x24ef0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(4)?,16)?)?;
    // 0x24ef4
    c.b(C::Ne,0x24f20)?;
    // 0x24ef8
    c.literal(R::R2,0x24f84)?;
    // 0x24efc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x24f00
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24f04
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,4)?)?;
    // 0x24f08
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x24f0c
    c.emit(arm32::transfer_multiple(C::Cs,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x24f10
    c.literal(R::R1,0x24f88)?;
    // 0x24f14
    c.literal(R::R2,0x24f6c)?;
    // 0x24f18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,17)?)?;
    // 0x24f1c
    c.b(C::Al,0x24f44)?;
    // 0x24f20
    c.literal(R::R2,0x24f84)?;
    // 0x24f24
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x24f28
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x24f2c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,3)?)?;
    // 0x24f30
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x24f34
    c.b(C::Cs,0x24f64)?;
    // 0x24f38
    c.literal(R::R1,0x24f88)?;
    // 0x24f3c
    c.literal(R::R2,0x24f6c)?;
    // 0x24f40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,25)?)?;
    // 0x24f44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2560)?)?;
    // 0x24f48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24f4c
    c.bl(0x2ec94)?;
    // 0x24f50
    c.literal(R::R1,0x24f70)?;
    // 0x24f54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x24f58
    c.bl(0x2ec94)?;
    // 0x24f5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x24f60
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x24f64
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x24f68)?;
    for word in BUFFER_COPY_DIAGNOSTICS{ c.emit(word)?; }
    c.check(0x24f90)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    copy_codec_plane_records(c)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn copy_plane_widths_are_checked(){
        let mut c=Code::new(0x24cd4);
        emit(&mut c).unwrap();
        assert_eq!(c.at(),0x24f90);
        assert_eq!(c.bytes.len(),0x2bc);
        assert_eq!(BUFFER_COPY_DIAGNOSTICS.len(),10);
    }
}
