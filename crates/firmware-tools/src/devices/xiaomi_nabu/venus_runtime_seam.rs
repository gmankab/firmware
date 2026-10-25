// SPDX-License-Identifier: GPL-3.0-only
//! Byte-exact ARM32 reconstruction of the Xiaomi Nabu Venus codec controller
//! seam between Agent 01's forward range and Agent 03's late range.
//!
//! This implementation emits typed ARM A32 instruction encodings, separately
//! identified switch-target words and diagnostic metadata. It does not import
//! executable bytes or signatures from the original MBN, and by itself does
//! not establish high-level semantic equivalence or a bootable firmware.
use arm32::{BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START: usize = 0x18000;
pub const END: usize = 0x1815c; // Checkpoint 01 verified prefix of full 0x183e0 seam.
pub const FILE_BASE: usize = 0x4000;
pub const FILE_OFFSET: usize = FILE_BASE + START;
pub const RESERVED_END:usize = 0x183e0;

/// Guarded, no-padding output: the writer advances monotonically from START
/// and rejects attempts to overlap Agent 01 or Agent 03's address ranges.
struct Code {bytes:Vec<u8>}
impl Code {
    fn new()->Self {Self{bytes:Vec::new()}}
    fn at(&self)->usize {START+self.bytes.len()}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at()!=addr {return Err(format!("Venus seam: emitted address 0x{:x} != expected 0x{addr:x}",self.at()));}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at()<START || self.at()>=RESERVED_END || self.at()%4!=0 {
            return Err(format!("attempted to write outside Venus seam at 0x{:x}",self.at()));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());Ok(())
    }
    fn b(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(cond,self.at() as u32,target as u32,false)?)
    }
    fn bl(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(cond,self.at() as u32,target as u32,true)?)
    }
    fn literal(&mut self,rd:R,target:usize)->Result<(),String>{
        let delta=target as isize-self.at() as isize-8;
        let offset=i32::try_from(delta).map_err(|_| "PC-relative A32 displacement overflow")?;
        self.emit(arm32::transfer_word(C::Al,true,rd,R::PC,offset)?)
    }
}

const INPUT_GEOMETRY_TARGETS:[u32;7]=[
    0x1802c,0x1802c,0x1815c,0x18050,0x18088,0x180c0,0x18104,
];

/// Resume the already-open frame beginning at 0x17fd0. Map codec format IDs
/// through a seven-way jump table, separating the pointer words from A32 code.
fn choose_stream_geometry(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x18000: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0x0)?)?;
    // 0x18004: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0x0)?)?;
    // 0x18008: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0x0)?)?;
    // 0x1800c: ldr pc, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0x18010)?;
    for target in INPUT_GEOMETRY_TARGETS {c.emit(target)?;}
    c.check(0x1802c)
}

/// Derive plane pitch and logical buffer dimensions using hardware register
/// descriptions, then enter the shared stream-command builder at 0x1815c.
fn calculate_stream_layout(c:&mut Code)->Result<(),String>{
    c.check(0x1802c)?;
    // 0x1802c: ldr r2, [pc, #0x3a8]        @ 0x40c <entry+0x40c>
    c.literal(R::R2,0x183dc)?;
    // 0x18030: mov r1, #117
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x75)?)?;
    // 0x18034: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0x0)?)?;
    // 0x18038: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0x0)?)?;
    // 0x1803c: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1300)?)?;
    // 0x18040: bl 0x17b0 <entry+0xfffffffffffe97e0> @ imm = #-0x16898
    c.bl(C::Al, 0x17b0)?;
    // 0x18044: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0x0)?)?;
    // 0x18048: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0x0)?)?;
    // 0x1804c: b 0x1815c <entry+0x18c>   @ imm = #0x108
    c.b(C::Al, 0x1815c)?;
    // 0x18050: ldr r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x18054: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x18058: mov r1, #332
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x14c)?)?;
    // 0x1805c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1000)?)?;
    // 0x18060: ldr r1, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x18064: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,0x7)?)?;
    // 0x18068: lsr r7, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R1,Shift::Lsr,4)?)?;
    // 0x1806c: mov r1, #348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x15c)?)?;
    // 0x18070: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1000)?)?;
    // 0x18074: ldr r1, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x18078: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,0x7)?)?;
    // 0x1807c: bic r5, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::R1,0x7)?)?;
    // 0x18080: mov r1, #336
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x150)?)?;
    // 0x18084: b 0x180f4 <entry+0x124>   @ imm = #0x68
    c.b(C::Al, 0x180f4)?;
    // 0x18088: ldr r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x1808c: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x18090: mov r1, #336
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x150)?)?;
    // 0x18094: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1000)?)?;
    // 0x18098: ldr r1, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x1809c: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,0x7)?)?;
    // 0x180a0: lsr r7, r1, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R1,Shift::Lsr,3)?)?;
    // 0x180a4: mov r1, #344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x158)?)?;
    // 0x180a8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1000)?)?;
    // 0x180ac: ldr r1, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x180b0: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,0x7)?)?;
    // 0x180b4: bic r5, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::R1,0x7)?)?;
    // 0x180b8: mov r1, #332
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x14c)?)?;
    // 0x180bc: b 0x180f4 <entry+0x124>   @ imm = #0x30
    c.b(C::Al, 0x180f4)?;
    // 0x180c0: ldr r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x180c4: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x180c8: mov r1, #332
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x14c)?)?;
    // 0x180cc: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1000)?)?;
    // 0x180d0: ldr r1, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x180d4: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,0x7)?)?;
    // 0x180d8: lsr r7, r1, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R1,Shift::Lsr,3)?)?;
    // 0x180dc: mov r1, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x154)?)?;
    // 0x180e0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1000)?)?;
    // 0x180e4: ldr r1, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x180e8: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,0x7)?)?;
    // 0x180ec: bic r5, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::R1,0x7)?)?;
    // 0x180f0: mov r1, #328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x148)?)?;
    // 0x180f4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1000)?)?;
    // 0x180f8: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0x0)?)?;
    // 0x180fc: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x18100: b 0x18154 <entry+0x184>   @ imm = #0x4c
    c.b(C::Al, 0x18154)?;
    // 0x18104: mov r1, #684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x2ac)?)?;
    // 0x18108: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1c00)?)?;
    // 0x1810c: ldr r1, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x18110: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0x0)?)?;
    // 0x18114: beq 0x18128 <entry+0x158>   @ imm = #0xc
    c.b(C::Eq, 0x18128)?;
    // 0x18118: add r0, r4, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,0x58)?)?;
    // 0x1811c: b 0x18138 <entry+0x168>   @ imm = #0x14
    c.b(C::Al, 0x18138)?;
    // 0x18120: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0x0)?)?;
    // 0x18124: b 0x18044 <entry+0x74>    @ imm = #-0xe8
    c.b(C::Al, 0x18044)?;
    // 0x18128: ldr r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x1812c: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x18130: add r0, r0, #328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,0x148)?)?;
    // 0x18134: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,0x1000)?)?;
    // 0x18138: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x1813c: ldr r1, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,92)?)?;
    // 0x18140: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,0x7)?)?;
    // 0x18144: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,0x7)?)?;
    // 0x18148: lsr r7, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R0,Shift::Lsr,3)?)?;
    // 0x1814c: ldr r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,88)?)?;
    // 0x18150: bic r5, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::R1,0x7)?)?;
    // 0x18154: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,0x7)?)?;
    // 0x18158: bic r2, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R0,0x7)?)?;
    c.check(END)
}

/// Emit only independently reconstructed bytes, with no opaque vendor payload.
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();
    choose_stream_geometry(&mut c)?;
    calculate_stream_layout(&mut c)?;
    c.check(END)?;
    Ok(c.bytes)
}

/// Optional strict comparison; original reference must be passed explicitly,
/// and may never be embedded into the source-built generator.
pub fn verify_reference(reference:&[u8])->Result<usize,String>{
    let fragment=fragment()?;
    let cmp=reference.get(FILE_OFFSET..FILE_OFFSET+fragment.len())
        .ok_or_else(||"reference Venus.mbn too short".to_string())?;
    if let Some((offset,(expected,actual)))=cmp.iter().zip(&fragment).enumerate().find(|(_, (a,b))|a!=b){
        return Err(format!("Venus seam byte mismatch at runtime 0x{:x}, file 0x{:x}: reference={expected:02x} generated={actual:02x}",START+offset,FILE_OFFSET+offset));
    }
    Ok(fragment.len())
}

#[cfg(test)] mod tests {
    use super::*;
    fn word_at(data:&[u8],addr:usize)->u32{
        let off=addr-START;
        u32::from_le_bytes(data[off..off+4].try_into().unwrap())
    }
    #[test] fn prefix_has_exact_bounds_and_explicit_jump_table(){
        let data=fragment().unwrap();
        assert_eq!(data.len(),END-START);
        assert_eq!(FILE_OFFSET,0x1c000);
        for (idx,target) in INPUT_GEOMETRY_TARGETS.iter().enumerate(){
            assert_eq!(word_at(&data,0x18010+idx*4),*target);
        }
        assert_eq!(word_at(&data,0x1800c),
            arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::new(3).unwrap(),Shift::Lsl,0).unwrap());
    }
    #[test] fn no_implicit_padded_gaps(){
        let mut c=Code::new();
        assert!(c.check(0x18004).is_err());
        assert!(c.emit(arm32::mov_register(C::Al,R::R0,R::R0)).is_ok());
        assert!(c.check(0x18000).is_err());
        assert!(c.check(0x18004).is_ok());
    }
}
