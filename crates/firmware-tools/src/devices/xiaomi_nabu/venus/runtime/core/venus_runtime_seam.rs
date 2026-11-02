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
pub const END: usize = 0x183e0; // Fully reconstructed seam; exclusive Agent 03 boundary.
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
    c.check(0x1815c)
}

/// Populate the Venus firmware command-ring headers and packed codec geometry.
/// The two recurring output channels are individually advanced in controller
/// memory; ranges and format selectors are validated before commands are
/// published. This routine continues the same frame begun at 0x17fd0.
fn publish_channel_descriptors(c:&mut Code)->Result<(),String>{
    c.check(0x1815c)?;
    // 0x1815c: mov r0, #564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0x234)?)?;
    // 0x18160: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x18164: mov r8, r4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(4)?))?;
    // 0x18168: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x4)?)?;
    // 0x1816c: mov r9, r4
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(4)?))?;
    // 0x18170: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,0x1c00)?)?;
    // 0x18174: orr r1, r1, #33792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x8400)?)?;
    // 0x18178: ldr r0, [r8, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,0)?)?;
    // 0x1817c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x18180: mov r1, #568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x238)?)?;
    // 0x18184: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,0x8)?)?;
    // 0x18188: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,0x1c00)?)?;
    // 0x1818c: ldr r1, [r9, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::new(9)?,R::R1,Shift::Lsl,0)?)?;
    // 0x18190: str r10, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R1,0)?)?;
    // 0x18194: str r3, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,0)?)?;
    // 0x18198: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0x8)?)?;
    // 0x1819c: orr r3, r3, #33792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,0x8400)?)?;
    // 0x181a0: str r3, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x181a4: add r0, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,0x8)?)?;
    // 0x181a8: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x181ac: orr r0, r5, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R2,Shift::Lsl,16,false)?)?;
    // 0x181b0: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x181b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0x1)?)?;
    // 0x181b8: str r0, [r4, #0x938]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2360)?)?;
    // 0x181bc: ldr r0, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2648)?)?;
    // 0x181c0: ldr r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,40)?)?;
    // 0x181c4: add r0, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x181c8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,0x1)?)?;
    // 0x181cc: blx 0x83c4c <entry+0x6bc7c> @ imm = #0x6ba78
    c.emit(arm32::branch_exchange_immediate(c.at() as u32, 0x83c4c)?)?;
    // 0x181d0: str r0, [r4, #0x93c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2364)?)?;
    // 0x181d4: ldr r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // 0x181d8: add r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x181dc: strb r6, [r0, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R0,972)?)?;
    // 0x181e0: ldr r0, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2648)?)?;
    // 0x181e4: ldr r1, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,88)?)?;
    // 0x181e8: ldr r2, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,40)?)?;
    // 0x181ec: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x181f0: bhs 0x182e0 <entry+0x310>   @ imm = #0xe8
    c.b(C::Cs, 0x182e0)?;
    // 0x181f4: ldr r1, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,68)?)?;
    // 0x181f8: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0x6)?)?;
    // 0x181fc: blo 0x18308 <entry+0x338>   @ imm = #0x104
    c.b(C::Cc, 0x18308)?;
    // 0x18200: ldr r2, [r4, #0x93c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2364)?)?;
    // 0x18204: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0x2)?)?;
    // 0x18208: blo 0x182e4 <entry+0x314>   @ imm = #0xd4
    c.b(C::Cc, 0x182e4)?;
    // 0x1820c: ldr r2, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // 0x18210: mov r3, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0xc)?)?;
    // 0x18214: orr r3, r3, #33792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,0x8400)?)?;
    // 0x18218: add r1, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,0x4)?)?;
    // 0x1821c: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x18220: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // 0x18224: ldr r3, [r4, #0x938]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2360)?)?;
    // 0x18228: ldr r6, [r4, #0x93c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2364)?)?;
    // 0x1822c: ldr r5, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,0)?)?;
    // 0x18230: lsl r3, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,1)?)?;
    // 0x18234: add r2, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,0x4)?)?;
    // 0x18238: orr r3, r3, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x1823c: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x18240: orr r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,0x1)?)?;
    // 0x18244: str r3, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,0)?)?;
    // 0x18248: ldr r3, [r4, #0x93c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2364)?)?;
    // 0x1824c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0x0)?)?;
    // 0x18250: beq 0x1828c <entry+0x2bc>   @ imm = #0x34
    c.b(C::Eq, 0x1828c)?;
    // 0x18254: mov r5, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0x410)?)?;
    // 0x18258: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0x0)?)?;
    // 0x1825c: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0x0)?)?;
    // 0x18260: orr r5, r5, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,0x8000)?)?;
    // 0x18264: str r5, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(5)?,R::R1,4)?)?;
    // 0x18268: str r6, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(6)?,R::R2,4)?)?;
    // 0x1826c: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,0x1)?)?;
    // 0x18270: ldr r7, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,40)?)?;
    // 0x18274: add r6, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x18278: ldr r7, [r4, #0x93c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,2364)?)?;
    // 0x1827c: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x18280: blo 0x18264 <entry+0x294>   @ imm = #-0x24
    c.b(C::Cc, 0x18264)?;
    // 0x18284: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x18288: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x1828c: ldr r3, [r4, #0x938]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2360)?)?;
    // 0x18290: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0x0)?)?;
    // 0x18294: beq 0x182d0 <entry+0x300>   @ imm = #0x34
    c.b(C::Eq, 0x182d0)?;
    // 0x18298: mov r5, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0x14)?)?;
    // 0x1829c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0x0)?)?;
    // 0x182a0: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0x0)?)?;
    // 0x182a4: orr r5, r5, #33792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,0x8400)?)?;
    // 0x182a8: str r5, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(5)?,R::R1,4)?)?;
    // 0x182ac: str r6, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(6)?,R::R2,4)?)?;
    // 0x182b0: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,0x1)?)?;
    // 0x182b4: ldr r7, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,44)?)?;
    // 0x182b8: add r6, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x182bc: ldr r7, [r4, #0x938]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,2360)?)?;
    // 0x182c0: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x182c4: blo 0x182a8 <entry+0x2d8>   @ imm = #-0x24
    c.b(C::Cc, 0x182a8)?;
    // 0x182c8: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x182cc: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x182d0: ldr r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // 0x182d4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x1)?)?;
    // 0x182d8: add r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x182dc: strb r1, [r0, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,972)?)?;
    // 0x182e0: ldr r1, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,68)?)?;
    // 0x182e4: ldr r0, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2648)?)?;
    // 0x182e8: ldr r12, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,0)?)?;
    // 0x182ec: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0x7)?)?;
    // 0x182f0: bne 0x1830c <entry+0x33c>   @ imm = #0x14
    c.b(C::Ne, 0x1830c)?;
    // 0x182f4: ldr r1, [r4, #0x8fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2300)?)?;
    // 0x182f8: ldr r2, [r0, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,712)?)?;
    // 0x182fc: ldr r0, [r0, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,716)?)?;
    // 0x18300: lsl r1, r1, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,6)?)?;
    // 0x18304: b 0x18354 <entry+0x384>   @ imm = #0x48
    c.b(C::Al, 0x18354)?;
    // 0x18308: ldr r12, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,0)?)?;
    // 0x1830c: ldr r0, [r0, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,716)?)?;
    // 0x18310: sub r2, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,0x4)?)?;
    // 0x18314: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0x0)?)?;
    // 0x18318: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0x3)?)?;
    // 0x1831c: bhi 0x18364 <entry+0x394>   @ imm = #0x40
    c.b(C::Hi, 0x18364)?;
    // 0x18320: lsl r3, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // 0x18324: add r7, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::PC,0x4)?)?;
    // 0x18328: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0x0)?)?;
    // 0x1832c: ldr pc, [r3, r7]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    c.check(0x18330)
}

/// Dispatch packed stream geometry by codec format. The four-way branch
/// table consists of pointer words, *not* executable A32 instructions.
fn pack_codec_geometry_and_finish(c:&mut Code)->Result<(),String>{
    const GEOMETRY_FORMAT_CASES:[u32;4]=[
        0x18340, 0x18378, 0x18380, 0x18354,
    ];
    c.check(0x18330)?;
    for target in GEOMETRY_FORMAT_CASES {c.emit(target)?;}
    c.check(0x18340)?;
    // 0x18340: mov r4, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0x200)?)?;
    // 0x18344: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0x20)?)?;
    // 0x18348: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0x0)?)?;
    // 0x1834c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0x0)?)?;
    // 0x18350: b 0x18390 <entry+0x3c0>   @ imm = #0x38
    c.b(C::Al, 0x18390)?;
    // 0x18354: ldr r5, [r4, #0x2d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,728)?)?;
    // 0x18358: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0x0)?)?;
    // 0x1835c: movne r5, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,0x8)?)?;
    // 0x18360: b 0x1836c <entry+0x39c>   @ imm = #0x4
    c.b(C::Al, 0x1836c)?;
    // 0x18364: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0x0)?)?;
    // 0x18368: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0x0)?)?;
    // 0x1836c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0x0)?)?;
    // 0x18370: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0x0)?)?;
    // 0x18374: b 0x18390 <entry+0x3c0>   @ imm = #0x14
    c.b(C::Al, 0x18390)?;
    // 0x18378: mov r4, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0x200)?)?;
    // 0x1837c: b 0x18384 <entry+0x3b4>   @ imm = #0x0
    c.b(C::Al, 0x18384)?;
    // 0x18380: mov r4, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0x100)?)?;
    // 0x18384: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0x20)?)?;
    // 0x18388: mov r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0x8)?)?;
    // 0x1838c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0x0)?)?;
    // 0x18390: ldr r7, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,0)?)?;
    // 0x18394: orr r2, r2, r12, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R12,Shift::Lsl,10,false)?)?;
    // 0x18398: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x1839c: orr r0, r1, r0, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,18,false)?)?;
    // 0x183a0: orr r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x183a4: add r6, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,0x4)?)?;
    // 0x183a8: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x183ac: str r6, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,0)?)?;
    // 0x183b0: mov r6, #1088
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0x440)?)?;
    // 0x183b4: orr r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x183b8: orr r6, r6, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,0x8000)?)?;
    // 0x183bc: str r6, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,0)?)?;
    // 0x183c0: ldr r7, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,0)?)?;
    // 0x183c4: add r6, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,0x4)?)?;
    // 0x183c8: str r6, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,0)?)?;
    // 0x183cc: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x183d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0x1)?)?;
    // 0x183d4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,0x1c)?)?;
    // 0x183d8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x183dc)?;
    const CONTROLLER_DIAGNOSTIC:u32=0x000e_3d16;
    c.emit(CONTROLLER_DIAGNOSTIC)?; // Not an ARM instruction.
    c.check(END)
}

/// Emit only independently reconstructed bytes, with no opaque vendor payload.
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();
    choose_stream_geometry(&mut c)?;
    calculate_stream_layout(&mut c)?;
    publish_channel_descriptors(&mut c)?;
    pack_codec_geometry_and_finish(&mut c)?;
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
    #[test] fn complete_seam_has_two_dispatch_tables_and_a_literal(){
        let b=fragment().unwrap();
        assert_eq!((START,END,FILE_OFFSET,FILE_OFFSET+b.len()),(0x18000,0x183e0,0x1c000,0x1c3e0));
        assert_eq!(b.len(),992);
        for (idx,target) in [0x18340u32,0x18378,0x18380,0x18354].iter().enumerate(){
            assert_eq!(word_at(&b,0x18330+4*idx),*target);
        }
        assert_eq!(word_at(&b,0x183dc),0x000e_3d16);
        // Both PC-relative and immediate BLX branch forms must match A32.
        assert_eq!(word_at(&b,0x1802c),
            arm32::transfer_word(C::Al,true,R::R2,R::PC,0x3a8).unwrap());
        assert_eq!(word_at(&b,0x181cc),
            arm32::branch_exchange_immediate(0x181cc,0x83c4c).unwrap());
        // Original function ends with a restored PC; Agent 03 starts fresh.
        let saved=(4..=11).map(|n|1u16<<n).sum::<u16>()|R::PC.bit();
        assert_eq!(word_at(&b,0x183d8),
            arm32::transfer_multiple(C::Al,true,R::SP,saved,BlockMode::IncrementAfter,true).unwrap());
    }
    #[test] fn complete_seam_does_not_overrun_agent03(){
        let mut c=Code{bytes:vec![0;RESERVED_END-START]};
        assert_eq!(c.at(),RESERVED_END);
        assert!(c.emit(0).is_err());
    }
    #[test] fn no_implicit_padded_gaps(){
        let mut c=Code::new();
        assert!(c.check(0x18004).is_err());
        assert!(c.emit(arm32::mov_register(C::Al,R::R0,R::R0)).is_ok());
        assert!(c.check(0x18000).is_err());
        assert!(c.check(0x18004).is_ok());
    }
}
