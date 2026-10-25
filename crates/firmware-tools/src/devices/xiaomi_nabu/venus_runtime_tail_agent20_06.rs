// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-06: source-generated ARM32 Venus executable tail. This module
//! reconstructs a bounded part of the *first* ARM executable PT_LOAD.
//!
//! It begins at runtime 0xa8000 and file 0xac000. Address 0xa8000 is
//! in the middle of a preexisting controller routine (some entry instructions
//! belong to Agent 20-05); this fragment is not independently callable.
//! Non-instruction literals are explicitly distinguished from A32 code.
//! All instruction words are generated from architecture operands, not
//! embedded original machine-code bytes. Reference firmware is used only by
//! the opt-in verification CLI, and never redistributed in this crate.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};

pub const FILE_BASE: usize=0x4000;
pub const START: usize=0xa8000;
pub const END: usize=0xa8cbc;
pub const MAX_OWNED_END: usize=0x1029f4;
pub const FILE_OFFSET: usize=FILE_BASE+START;

struct Code { bytes: Vec<u8> }
impl Code {
    fn new() -> Self { Self { bytes: Vec::new() } }
    fn at(&self) -> usize { START + self.bytes.len() }
    fn check(&self, addr: usize) -> Result<(), String> {
        if addr!=self.at() {return Err(format!("Venus agent06 layout: at 0x{:x}, expected 0x{addr:x}",self.at()));}
        Ok(())
    }
    fn emit(&mut self, word: u32) -> Result<(), String> {
        let at=self.at();
        if at % 4 !=0 || at>=MAX_OWNED_END || at>=END {
            return Err(format!("agent06 A32 emitted outside owned interval 0x{at:x}"));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        Ok(())
    }
    fn b(&mut self, condition: C, target: usize, link: bool) -> Result<(),String> {
        self.emit(arm32::branch(condition,self.at() as u32,target as u32,link)?)
    }
    fn literal(&mut self, rd:R, absolute_address:usize) -> Result<(),String> {
        let delta=absolute_address as i64-self.at() as i64-8;
        let signed=i32::try_from(delta).map_err(|_|"literal displacement overflow")?;
        self.emit(arm32::transfer_word(C::Al,true,rd,R::PC,signed)?)
    }
}

/// Continue the preceding controller routine across the 0xa8000 ownership boundary, record codec status and command completion.
fn continue_prior_codec_release(c:&mut Code)->Result<(),String>{
    c.check(0xa8000)?;
    // VMA 0xa8000
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,28)?)?;
    // VMA 0xa8004
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,904)?)?;
    // VMA 0xa8008
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,32)?)?;
    // VMA 0xa800c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xa8010
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa8014
    c.b(C::Al,0xa8ea0,true)?;
    // VMA 0xa8018
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa801c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8020
    c.b(C::Al,0xb40ac,true)?;
    // VMA 0xa8024
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3968)?)?;
    // VMA 0xa8028
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // VMA 0xa802c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xa8030
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // VMA 0xa8034
    c.b(C::Al,0xd98b8,true)?;
    // VMA 0xa8038
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa803c
    c.b(C::Eq,0xa8078,false)?;
    // VMA 0xa8040
    c.literal(R::R1,0xa83c4)?;
    // VMA 0xa8044
    c.literal(R::R2,0xa83c8)?;
    // VMA 0xa8048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // VMA 0xa804c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8050
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xa8054
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8058
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa805c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8060
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8064
    c.literal(R::R2,0xa8398)?;
    // VMA 0xa8068
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // VMA 0xa806c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8070
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xa8074
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa8078
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,764)?)?;
    // VMA 0xa807c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa8080
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,764)?)?;
    // VMA 0xa8084
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xa8088
    c.b(C::Al,0xd9a34,true)?;
    // VMA 0xa808c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // VMA 0xa8090
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,740)?)?;
    // VMA 0xa8094
    c.b(C::Al,0xa71d0,false)?;
    // VMA 0xa8098
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,732)?)?;
    // VMA 0xa809c
    c.literal(R::R1,0xa83ec)?;
    // VMA 0xa80a0
    c.literal(R::R2,0xa83c8)?;
    // VMA 0xa80a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,39)?)?;
    // VMA 0xa80a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xa80ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xa80b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa80b4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa80b8
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa80bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa80c0
    c.b(C::Al,0xa71cc,false)?;
    // VMA 0xa80c4
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2828)?)?;
    // VMA 0xa80c8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa80cc
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,1096)?)?;
    // VMA 0xa80d0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xa80d4
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,1896)?)?;
    // VMA 0xa80d8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,592)?)?;
    // VMA 0xa80dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xa80e0
    c.b(C::Ne,0xa80fc,false)?;
    // VMA 0xa80e4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1096)?)?;
    // VMA 0xa80e8
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,44)?)?;
    // VMA 0xa80ec
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // VMA 0xa80f0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa80f4
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,1100)?)?;
    // VMA 0xa80f8
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R0,1104)?)?;
    // VMA 0xa80fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1100)?)?;
    // VMA 0xa8100
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8104
    c.b(C::Eq,0xa8124,false)?;
    // VMA 0xa8108
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xa810c
    c.b(C::Al,0xa8ee8,true)?;
    // VMA 0xa8110
    c.b(C::Al,0xa8134,false)?;
    c.check(0xa8114)
}

// This four-word inline literal pool is jumped over by the source routine.
const CONTROLLER_STATUS_DIAGNOSTICS:[u32;4]=[
    0x000f_60d0, 0x000f_6074, 0x000f_7fd1, 0x000f_611a,
];
/// Account event buffers, synchronize state flags and issue the firmware trace callbacks.
fn account_codec_state_and_notify(c:&mut Code)->Result<(),String>{
    c.check(0xa8124)?;
    // VMA 0xa8124
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2476)?)?;
    // VMA 0xa8128
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xa812c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xa8130
    c.b(C::Al,0xb48bc,true)?;
    // VMA 0xa8134
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,776)?)?;
    // VMA 0xa8138
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // VMA 0xa813c
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,572)?)?;
    // VMA 0xa8140
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8144
    c.b(C::Eq,0xa8154,false)?;
    // VMA 0xa8148
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xa814c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8150
    c.b(C::Al,0xb40ac,true)?;
    // VMA 0xa8154
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xa8158
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3556)?)?;
    // VMA 0xa815c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,1096)?)?;
    // VMA 0xa8160
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,1904)?)?;
    // VMA 0xa8164
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa8168
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1904)?)?;
    // VMA 0xa816c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,1100)?)?;
    // VMA 0xa8170
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xa8174
    c.b(C::Ne,0xa8204,false)?;
    // VMA 0xa8178
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,2,true)?)?;
    // VMA 0xa817c
    c.b(C::Eq,0xa8204,false)?;
    // VMA 0xa8180
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,44)?)?;
    // VMA 0xa8184
    c.literal(R::R2,0xa8398)?;
    // VMA 0xa8188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa818c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xa8190
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,206)?)?;
    // VMA 0xa8194
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8198
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xa819c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa81a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3056)?)?;
    // VMA 0xa81a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa81a8
    c.b(C::Ne,0xa8204,false)?;
    // VMA 0xa81ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xa81b0
    c.literal(R::R1,0xa83d8)?;
    // VMA 0xa81b4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // VMA 0xa81b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa81bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa81c0
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa81c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa81c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa81cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2444)?)?;
    // VMA 0xa81d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,110)?)?;
    // VMA 0xa81d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa81d8
    c.b(C::Ne,0xa81e8,false)?;
    // VMA 0xa81dc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,111)?)?;
    // VMA 0xa81e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa81e4
    c.b(C::Eq,0xa8390,false)?;
    // VMA 0xa81e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,588)?)?;
    // VMA 0xa81ec
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xa81f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xa81f4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2147483648)?)?;
    // VMA 0xa81f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,120)?)?;
    // VMA 0xa81fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,402653184)?)?;
    // VMA 0xa8200
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,252)?)?;
    // VMA 0xa8204
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3056)?)?;
    // VMA 0xa8208
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xa820c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa8210
    c.b(C::Ne,0xa824c,false)?;
    // VMA 0xa8214
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1096)?)?;
    // VMA 0xa8218
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,44)?)?;
    // VMA 0xa821c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xa8220
    c.b(C::Ne,0xa824c,false)?;
    // VMA 0xa8224
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // VMA 0xa8228
    c.literal(R::R1,0xa83dc)?;
    // VMA 0xa822c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa8230
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8234
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa8238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa823c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8240
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xa8244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,134217728)?)?;
    // VMA 0xa8248
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,252)?)?;
    // VMA 0xa824c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,776)?)?;
    // VMA 0xa8250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xa8254
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,20)?)?;
    // VMA 0xa8258
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,896)?)?;
    // VMA 0xa825c
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::R2,R::R1,Shift::Lsr,7,false)?)?;
    // VMA 0xa8260
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,28)?)?;
    // VMA 0xa8264
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,1096)?)?;
    // VMA 0xa8268
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2452)?)?;
    // VMA 0xa826c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1912)?)?;
    // VMA 0xa8270
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa8274
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa8278
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // VMA 0xa827c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1912)?)?;
    // VMA 0xa8280
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2496)?)?;
    // VMA 0xa8284
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8288
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,2452)?)?;
    // VMA 0xa828c
    c.emit(arm32::mov_shifted_register(C::Ne,R::R1,R::R1,Shift::Lsl,20)?)?;
    // VMA 0xa8290
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8294
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // VMA 0xa8298
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // VMA 0xa829c
    c.b(C::Al,0xd98b8,true)?;
    // VMA 0xa82a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa82a4
    c.b(C::Eq,0xa82e0,false)?;
    // VMA 0xa82a8
    c.literal(R::R1,0xa83c4)?;
    // VMA 0xa82ac
    c.literal(R::R2,0xa83c8)?;
    // VMA 0xa82b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,250)?)?;
    // VMA 0xa82b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa82b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xa82bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa82c0
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa82c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa82c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa82cc
    c.literal(R::R2,0xa8398)?;
    // VMA 0xa82d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,251)?)?;
    // VMA 0xa82d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa82d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xa82dc
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa82e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,764)?)?;
    // VMA 0xa82e4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa82e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,764)?)?;
    // VMA 0xa82ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa82f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2436)?)?;
    // VMA 0xa82f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xa82f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xa82fc
    c.b(C::Al,0xd9a34,true)?;
    // VMA 0xa8300
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,776)?)?;
    // VMA 0xa8304
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8308
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,9)?)?;
    // VMA 0xa830c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,740)?)?;
    // VMA 0xa8310
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xa8314
    c.b(C::Al,0xd9a54,true)?;
    // VMA 0xa8318
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa831c
    c.b(C::Ne,0xa8328,false)?;
    // VMA 0xa8320
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xa8324
    c.b(C::Al,0x292c,true)?;
    // VMA 0xa8328
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,880)?)?;
    // VMA 0xa832c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,740)?)?;
    // VMA 0xa8330
    c.literal(R::R1,0xa83f4)?;
    // VMA 0xa8334
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xa8338
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa833c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa8340
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xa8344
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8348
    c.literal(R::R1,0xa839c)?;
    // VMA 0xa834c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xa8350
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8354
    c.b(C::Al,0x2a158,true)?;
    // VMA 0xa8358
    c.b(C::Al,0xa71d0,false)?;
    c.check(0xa835c)
}

const CODEC_STATUS_MESSAGE: u32=0x000f_806b;
/// Check the codec status and post callbacks before returning to the caller.
fn notify_on_codec_status(c:&mut Code)->Result<(),String>{
    c.check(0xa8360)?;
    // VMA 0xa8360
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3968)?)?;
    // VMA 0xa8364
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xa8368
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // VMA 0xa836c
    c.b(C::Al,0xe1568,true)?;
    // VMA 0xa8370
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3968)?)?;
    // VMA 0xa8374
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,11)?)?;
    // VMA 0xa8378
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa837c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xa8380
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xa8384
    c.b(C::Al,0xaba80,true)?;
    // VMA 0xa8388
    c.b(C::Al,0xa82f8,false)?;
    c.check(0xa838c)
}

const CONTROLLER_ERROR_CLASS:u32=0x0100_1005;
/// Finalize the exceptional codec recovery path from the earlier function.
fn final_codec_recovery_branch(c:&mut Code)->Result<(),String>{
    c.check(0xa8390)?;
    // VMA 0xa8390
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,584)?)?;
    // VMA 0xa8394
    c.b(C::Al,0xa81ec,false)?;
    c.check(0xa8398)
}

// 24 mixed references in the caller's trailing read-only diagnostic table.
const CONTROLLER_MESSAGE_CONSTANTS:[u32;24]=[
    0x000f_7e7b, 0x000e_2788, 0x0021_100e, 0x000f_655d,
    0x000f_549b, 0x0010_dd80, 0x000f_8163, 0x000f_649c,
    0x000f_541d, 0x000f_8116, 0x000f_642a, 0x000f_61cb,
    0x000f_8021, 0x000f_8144, 0x000f_80bc, 0x000f_80eb,
    0x000f_62f1, 0x000f_6310, 0x000f_7d9b, 0x000f_60fe,
    0x000f_7ff4, 0x000f_8035, 0x000f_61ac, 0x000f_63d1,
];


/// Initialize controller buffers, populate per-codec callback function tables,
/// and generate diagnostic error branches for allocation failures. The
/// controller state stores registers and addresses into mapped data structs.
/// Runtime `0xa83f8–0xa8bc4`; the subsequent three pools are literal data.
fn initialize_codec_resource_layout(c:&mut Code)->Result<(),String>{
    c.check(0xa83f8)?;
    // VMA 0xa83f8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa83fc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xa8400
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xa8404
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // VMA 0xa8408
    c.literal(R::R2,0xa8bc4)?;
    // VMA 0xa840c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xa8410
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // VMA 0xa8414
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,250)?)?;
    // VMA 0xa8418
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xa841c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    // VMA 0xa8420
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8424
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa8428
    c.b(C::Al,0x26754,true)?;
    // VMA 0xa842c
    c.literal(R::R1,0xa8bc8)?;
    // VMA 0xa8430
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xa8434
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xa8438
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xa843c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8440
    c.literal(R::R1,0xa8bcc)?;
    // VMA 0xa8444
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xa8448
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa844c
    c.literal(R::R0,0xa8bd0)?;
    // VMA 0xa8450
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa8454
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // VMA 0xa8458
    c.b(C::Ne,0xa8744,false)?;
    // VMA 0xa845c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4032)?)?;
    // VMA 0xa8460
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8464
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // VMA 0xa8468
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // VMA 0xa846c
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // VMA 0xa8470
    c.b(C::Eq,0xa8754,false)?;
    // VMA 0xa8474
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // VMA 0xa8478
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa847c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4032)?)?;
    // VMA 0xa8480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa8484
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa8488
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa848c
    c.literal(R::R1,0xa8bdc)?;
    // VMA 0xa8490
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa8494
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xa8498
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // VMA 0xa849c
    c.b(C::Al,0x804fc,true)?;
    // VMA 0xa84a0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,2147483648)?)?;
    // VMA 0xa84a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // VMA 0xa84a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2488)?)?;
    // VMA 0xa84ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa84b0
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,2560)?)?;
    // VMA 0xa84b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,8)?)?;
    // VMA 0xa84b8
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::new(6)?.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa84bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,700)?)?;
    // VMA 0xa84c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,696)?)?;
    // VMA 0xa84c4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,16)?)?;
    // VMA 0xa84c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,720)?)?;
    // VMA 0xa84cc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,684)?)?;
    // VMA 0xa84d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,688)?)?;
    // VMA 0xa84d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xa84d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,20)?)?;
    // VMA 0xa84dc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // VMA 0xa84e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,68)?)?;
    // VMA 0xa84e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,592)?)?;
    // VMA 0xa84e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xa84ec
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,72)?)?;
    // VMA 0xa84f0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,96)?)?;
    // VMA 0xa84f4
    c.literal(R::R2,0xa8be0)?;
    // VMA 0xa84f8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,588)?)?;
    // VMA 0xa84fc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,584)?)?;
    // VMA 0xa8500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xa8504
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,24)?)?;
    // VMA 0xa8508
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,28)?)?;
    // VMA 0xa850c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,48)?)?;
    // VMA 0xa8510
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,712)?)?;
    // VMA 0xa8514
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,716)?)?;
    // VMA 0xa8518
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,692)?)?;
    // VMA 0xa851c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,120)?)?;
    // VMA 0xa8520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xa8524
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(5)?,724)?)?;
    // VMA 0xa8528
    c.b(C::Al,0xddad4,true)?;
    // VMA 0xa852c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,168)?)?;
    // VMA 0xa8530
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xa8534
    c.b(C::Al,0xddad4,true)?;
    // VMA 0xa8538
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // VMA 0xa853c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa8540
    c.b(C::Al,0x26d0,true)?;
    // VMA 0xa8544
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // VMA 0xa8548
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(5)?,728)?)?;
    // VMA 0xa854c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,732)?)?;
    // VMA 0xa8550
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,740)?)?;
    // VMA 0xa8554
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,744)?)?;
    // VMA 0xa8558
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,752)?)?;
    // VMA 0xa855c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,748)?)?;
    // VMA 0xa8560
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,764)?)?;
    // VMA 0xa8564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // VMA 0xa856c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,768)?)?;
    // VMA 0xa8570
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,772)?)?;
    // VMA 0xa8574
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,332)?)?;
    // VMA 0xa8578
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,944)?)?;
    // VMA 0xa857c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,2424)?)?;
    // VMA 0xa8580
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,568)?)?;
    // VMA 0xa8584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // VMA 0xa8588
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa858c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,0)?)?;
    // VMA 0xa8590
    c.literal(R::R2,0xa8be4)?;
    // VMA 0xa8594
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,780)?)?;
    // VMA 0xa8598
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa859c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,4004)?)?;
    // VMA 0xa85a0
    c.b(C::Al,0x30d0,true)?;
    // VMA 0xa85a4
    c.literal(R::R2,0xa8be8)?;
    // VMA 0xa85a8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,788)?)?;
    // VMA 0xa85ac
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa85b0
    c.b(C::Al,0x30d0,true)?;
    // VMA 0xa85b4
    c.literal(R::R2,0xa8bec)?;
    // VMA 0xa85b8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,796)?)?;
    // VMA 0xa85bc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa85c0
    c.b(C::Al,0x30d0,true)?;
    // VMA 0xa85c4
    c.literal(R::R2,0xa8bf0)?;
    // VMA 0xa85c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,804)?)?;
    // VMA 0xa85cc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa85d0
    c.b(C::Al,0x30d0,true)?;
    // VMA 0xa85d4
    c.literal(R::R0,0xa8bf4)?;
    // VMA 0xa85d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,844)?)?;
    // VMA 0xa85dc
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xa85e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa85e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,864)?)?;
    // VMA 0xa85e8
    c.b(C::Al,0x2aa0c,true)?;
    // VMA 0xa85ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(9)?,1168)?)?;
    // VMA 0xa85f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,100)?)?;
    // VMA 0xa85f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1024)?)?;
    // VMA 0xa85f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa85fc
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // VMA 0xa8600
    c.b(C::Al,0x7e544,true)?;
    // VMA 0xa8604
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1168)?)?;
    // VMA 0xa8608
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(9)?,948)?)?;
    // VMA 0xa860c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8610
    c.b(C::Eq,0xa877c,false)?;
    // VMA 0xa8614
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // VMA 0xa8618
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1152)?)?;
    // VMA 0xa861c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa8620
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa8624
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,308)?)?;
    // VMA 0xa8628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1152)?)?;
    // VMA 0xa862c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,352)?)?;
    // VMA 0xa8630
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,148)?)?;
    // VMA 0xa8634
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,1024)?)?;
    // VMA 0xa8638
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // VMA 0xa863c
    c.b(C::Al,0x7e544,true)?;
    // VMA 0xa8640
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1172)?)?;
    // VMA 0xa8644
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8648
    c.b(C::Eq,0xa878c,false)?;
    // VMA 0xa864c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,120)?)?;
    // VMA 0xa8650
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,1024)?)?;
    // VMA 0xa8654
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa8658
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xa865c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8660
    c.b(C::Ne,0xa87a0,false)?;
    // VMA 0xa8664
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,8)?)?;
    // VMA 0xa8668
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xa866c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8670
    c.b(C::Ne,0xa87b4,false)?;
    // VMA 0xa8674
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xa8678
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // VMA 0xa867c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // VMA 0xa8680
    c.b(C::Ls,0xa8654,false)?;
    // VMA 0xa8684
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,136)?)?;
    // VMA 0xa8688
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xa868c
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xa8690
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8694
    c.b(C::Ne,0xa87c4,false)?;
    // VMA 0xa8698
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,140)?)?;
    // VMA 0xa869c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xa86a0
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xa86a4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa86a8
    c.b(C::Ne,0xa87d4,false)?;
    // VMA 0xa86ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,972)?)?;
    // VMA 0xa86b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    // VMA 0xa86b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa86b8
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa86bc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa86c0
    c.b(C::Ne,0xa87e4,false)?;
    // VMA 0xa86c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,976)?)?;
    // VMA 0xa86c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xa86cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa86d0
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa86d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa86d8
    c.b(C::Ne,0xa87f4,false)?;
    // VMA 0xa86dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,980)?)?;
    // VMA 0xa86e0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xa86e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa86e8
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa86ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa86f0
    c.b(C::Ne,0xa8804,false)?;
    // VMA 0xa86f4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,984)?)?;
    // VMA 0xa86f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xa86fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa8700
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa8704
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8708
    c.b(C::Ne,0xa8814,false)?;
    // VMA 0xa870c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,988)?)?;
    // VMA 0xa8710
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xa8714
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa8718
    c.b(C::Al,0xd970c,true)?;
    // VMA 0xa871c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa8720
    c.b(C::Ne,0xa8824,false)?;
    // VMA 0xa8724
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,992)?)?;
    // VMA 0xa8728
    c.b(C::Al,0x2b8c,true)?;
    // VMA 0xa872c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8730
    c.b(C::Eq,0xa8948,false)?;
    // VMA 0xa8734
    c.literal(R::R1,0xa8c1c)?;
    // VMA 0xa8738
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa873c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,217)?)?;
    // VMA 0xa8740
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8744
    c.literal(R::R1,0xa8bd4)?;
    // VMA 0xa8748
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa874c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa8750
    c.b(C::Al,0xa8760,false)?;
    // VMA 0xa8754
    c.literal(R::R1,0xa8cb8)?;
    // VMA 0xa8758
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa875c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    // VMA 0xa8760
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2560)?)?;
    // VMA 0xa8764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8768
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa876c
    c.literal(R::R1,0xa8bcc)?;
    // VMA 0xa8770
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8774
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8778
    c.b(C::Al,0xa893c,false)?;
    // VMA 0xa877c
    c.literal(R::R1,0xa8cb4)?;
    // VMA 0xa8780
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,123)?)?;
    // VMA 0xa8788
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa878c
    c.literal(R::R1,0xa8cb0)?;
    // VMA 0xa8790
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8794
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,648)?)?;
    // VMA 0xa8798
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xa879c
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa87a0
    c.literal(R::R1,0xa8bf8)?;
    // VMA 0xa87a4
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa87ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2704)?)?;
    // VMA 0xa87b0
    c.b(C::Al,0xa8838,false)?;
    // VMA 0xa87b4
    c.literal(R::R1,0xa8bfc)?;
    // VMA 0xa87b8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,150)?)?;
    // VMA 0xa87c0
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa87c4
    c.literal(R::R1,0xa8c00)?;
    // VMA 0xa87c8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,157)?)?;
    // VMA 0xa87d0
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa87d4
    c.literal(R::R1,0xa8c04)?;
    // VMA 0xa87d8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,163)?)?;
    // VMA 0xa87e0
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa87e4
    c.literal(R::R1,0xa8c08)?;
    // VMA 0xa87e8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,171)?)?;
    // VMA 0xa87f0
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa87f4
    c.literal(R::R1,0xa8c0c)?;
    // VMA 0xa87f8
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa87fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,179)?)?;
    // VMA 0xa8800
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8804
    c.literal(R::R1,0xa8c10)?;
    // VMA 0xa8808
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa880c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,195)?)?;
    // VMA 0xa8810
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8814
    c.literal(R::R1,0xa8c14)?;
    // VMA 0xa8818
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa881c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,203)?)?;
    // VMA 0xa8820
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8824
    c.literal(R::R1,0xa8c18)?;
    // VMA 0xa8828
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa882c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,211)?)?;
    // VMA 0xa8830
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2560)?)?;
    // VMA 0xa8834
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8838
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa883c
    c.literal(R::R1,0xa8bcc)?;
    // VMA 0xa8840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8844
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8848
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,840)?)?;
    // VMA 0xa884c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // VMA 0xa8850
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8854
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,836)?)?;
    // VMA 0xa8858
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // VMA 0xa885c
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8860
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,2880)?)?;
    // VMA 0xa8864
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8868
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,900)?)?;
    // VMA 0xa886c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // VMA 0xa8870
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8874
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,3968)?)?;
    // VMA 0xa8878
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa887c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,952)?)?;
    // VMA 0xa8880
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa8884
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa8888
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa888c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,992)?)?;
    // VMA 0xa8890
    c.b(C::Al,0x2bcc,true)?;
    // VMA 0xa8894
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,984)?)?;
    // VMA 0xa8898
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa889c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,988)?)?;
    // VMA 0xa88a0
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa88a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,980)?)?;
    // VMA 0xa88a8
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa88ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,976)?)?;
    // VMA 0xa88b0
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa88b4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,972)?)?;
    // VMA 0xa88b8
    c.b(C::Al,0xd97f0,true)?;
    // VMA 0xa88bc
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // VMA 0xa88c0
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,1164)?)?;
    // VMA 0xa88c4
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xa88c8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa88cc
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xa88d0
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,1160)?)?;
    // VMA 0xa88d4
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xa88d8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa88dc
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xa88e0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(5)?,8)?)?;
    // VMA 0xa88e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa88e8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // VMA 0xa88ec
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // VMA 0xa88f0
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,-8)?)?;
    // VMA 0xa88f4
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xa88f8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa88fc
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xa8900
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(7)?,4)?)?;
    // VMA 0xa8904
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xa8908
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa890c
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xa8910
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xa8914
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(7)?))?;
    // VMA 0xa8918
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // VMA 0xa891c
    c.b(C::Ne,0xa88ec,false)?;
    // VMA 0xa8920
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,148)?)?;
    // VMA 0xa8924
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xa8928
    c.b(C::Al,0x7e7d4,true)?;
    // VMA 0xa892c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xa8930
    c.b(C::Al,0x7e7d4,true)?;
    // VMA 0xa8934
    c.emit(arm32::mov_register(C::Al,R::R0,R::SP))?;
    // VMA 0xa8938
    c.b(C::Al,0x3040,true)?;
    // VMA 0xa893c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8940
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xa8944
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa8948
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1000)?)?;
    // VMA 0xa894c
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8950
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8954
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xa8958
    c.b(C::Eq,0xa8b24,false)?;
    // VMA 0xa895c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1280)?)?;
    // VMA 0xa8960
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8964
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8968
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,952)?)?;
    // VMA 0xa896c
    c.b(C::Eq,0xa8b38,false)?;
    // VMA 0xa8970
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,256)?)?;
    // VMA 0xa8974
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,512)?)?;
    // VMA 0xa8978
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,768)?)?;
    // VMA 0xa897c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(5)?,956)?)?;
    // VMA 0xa8980
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xa8984
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R1.bit()|R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa8988
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,968)?)?;
    // VMA 0xa898c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8990
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1188)?)?;
    // VMA 0xa8994
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1152)?)?;
    // VMA 0xa8998
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa899c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa89a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,3968)?)?;
    // VMA 0xa89a4
    c.b(C::Eq,0xa8b48,false)?;
    // VMA 0xa89a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa89ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1152)?)?;
    // VMA 0xa89b0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa89b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // VMA 0xa89b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xa89bc
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa89c0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa89c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,3972)?)?;
    // VMA 0xa89c8
    c.b(C::Eq,0xa8b58,false)?;
    // VMA 0xa89cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,360)?)?;
    // VMA 0xa89d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa89d4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xa89d8
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa89dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,3968)?)?;
    // VMA 0xa89e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,3972)?)?;
    // VMA 0xa89e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1096)?)?;
    // VMA 0xa89e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2352)?)?;
    // VMA 0xa89ec
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa89f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa89f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,2880)?)?;
    // VMA 0xa89f8
    c.b(C::Eq,0xa8b6c,false)?;
    // VMA 0xa89fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8a00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2352)?)?;
    // VMA 0xa8a04
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8a08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1888)?)?;
    // VMA 0xa8a0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // VMA 0xa8a10
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8a14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8a18
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,2884)?)?;
    // VMA 0xa8a1c
    c.b(C::Eq,0xa8b80,false)?;
    // VMA 0xa8a20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1888)?)?;
    // VMA 0xa8a24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8a28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65536)?)?;
    // VMA 0xa8a2c
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8a30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1504)?)?;
    // VMA 0xa8a34
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xa8a38
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xa8a3c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8a40
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,2888)?)?;
    // VMA 0xa8a44
    c.b(C::Eq,0xa8b94,false)?;
    // VMA 0xa8a48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1504)?)?;
    // VMA 0xa8a4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8a50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // VMA 0xa8a54
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8a58
    c.literal(R::R0,0xa8c20)?;
    // VMA 0xa8a5c
    c.literal(R::R1,0xa8c24)?;
    // VMA 0xa8a60
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a64
    c.literal(R::R0,0xa8c28)?;
    // VMA 0xa8a68
    c.literal(R::R1,0xa8c2c)?;
    // VMA 0xa8a6c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a70
    c.literal(R::R0,0xa8c30)?;
    // VMA 0xa8a74
    c.literal(R::R1,0xa8c34)?;
    // VMA 0xa8a78
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a7c
    c.literal(R::R0,0xa8c38)?;
    // VMA 0xa8a80
    c.literal(R::R1,0xa8c3c)?;
    // VMA 0xa8a84
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a88
    c.literal(R::R0,0xa8c40)?;
    // VMA 0xa8a8c
    c.literal(R::R1,0xa8c44)?;
    // VMA 0xa8a90
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8a94
    c.literal(R::R0,0xa8c48)?;
    // VMA 0xa8a98
    c.literal(R::R1,0xa8c4c)?;
    // VMA 0xa8a9c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8aa0
    c.literal(R::R0,0xa8c50)?;
    // VMA 0xa8aa4
    c.literal(R::R1,0xa8c54)?;
    // VMA 0xa8aa8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8aac
    c.literal(R::R0,0xa8c58)?;
    // VMA 0xa8ab0
    c.literal(R::R1,0xa8c5c)?;
    // VMA 0xa8ab4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ab8
    c.literal(R::R0,0xa8c60)?;
    // VMA 0xa8abc
    c.literal(R::R1,0xa8c64)?;
    // VMA 0xa8ac0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ac4
    c.literal(R::R0,0xa8c68)?;
    // VMA 0xa8ac8
    c.literal(R::R1,0xa8c6c)?;
    // VMA 0xa8acc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ad0
    c.literal(R::R0,0xa8c70)?;
    // VMA 0xa8ad4
    c.literal(R::R1,0xa8c74)?;
    // VMA 0xa8ad8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8adc
    c.literal(R::R0,0xa8c78)?;
    // VMA 0xa8ae0
    c.literal(R::R1,0xa8c7c)?;
    // VMA 0xa8ae4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ae8
    c.literal(R::R0,0xa8c80)?;
    // VMA 0xa8aec
    c.literal(R::R1,0xa8c84)?;
    // VMA 0xa8af0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8af4
    c.literal(R::R0,0xa8c88)?;
    // VMA 0xa8af8
    c.literal(R::R1,0xa8c8c)?;
    // VMA 0xa8afc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8b00
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa8b04
    c.b(C::Al,0xb02b0,true)?;
    // VMA 0xa8b08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8b0c
    c.b(C::Eq,0xa8ba8,false)?;
    // VMA 0xa8b10
    c.literal(R::R1,0xa8c90)?;
    // VMA 0xa8b14
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,67)?)?;
    // VMA 0xa8b1c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xa8b20
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b24
    c.literal(R::R1,0xa8cac)?;
    // VMA 0xa8b28
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,744)?)?;
    // VMA 0xa8b30
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xa8b34
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b38
    c.literal(R::R1,0xa8ca8)?;
    // VMA 0xa8b3c
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,239)?)?;
    // VMA 0xa8b44
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8b48
    c.literal(R::R1,0xa8ca4)?;
    // VMA 0xa8b4c
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,253)?)?;
    // VMA 0xa8b54
    c.b(C::Al,0xa8830,false)?;
    // VMA 0xa8b58
    c.literal(R::R1,0xa8ca0)?;
    // VMA 0xa8b5c
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // VMA 0xa8b64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xa8b68
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b6c
    c.literal(R::R1,0xa8c9c)?;
    // VMA 0xa8b70
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,26)?)?;
    // VMA 0xa8b78
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xa8b7c
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b80
    c.literal(R::R1,0xa8c98)?;
    // VMA 0xa8b84
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,35)?)?;
    // VMA 0xa8b8c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xa8b90
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8b94
    c.literal(R::R1,0xa8c94)?;
    // VMA 0xa8b98
    c.literal(R::R2,0xa8bd8)?;
    // VMA 0xa8b9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,812)?)?;
    // VMA 0xa8ba0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xa8ba4
    c.b(C::Al,0xa8834,false)?;
    // VMA 0xa8ba8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,2444)?)?;
    // VMA 0xa8bac
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa8bb0
    c.b(C::Al,0xaf5c0,true)?;
    // VMA 0xa8bb4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa8bb8
    c.b(C::Al,0xad70c,true)?;
    // VMA 0xa8bbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8bc0
    c.b(C::Al,0xa8940,false)?;
    c.check(0xa8bc4)
}

// Runtime data 0xa8bc4..0xa8c24: 24 pointers and hardware constants.
const RESOURCE_INIT_LOGGING:[u32;24]=[
    0x000f7e7b, 0x000f5c38, 0x000e2788, 0xe00e0008,
    0x000f8181, 0x000f81ae, 0x0010b608, 0x08020080,
    0x000a6ca8, 0x000a6900, 0x000a5f6c, 0x000a582c,
    0x0010ceac, 0x000f59a4, 0x000f59d8, 0x000f5a0c,
    0x000f5c8d, 0x000f5cbd, 0x000f5cf5, 0x000f5d31,
    0x000f5d68, 0x000f5d98, 0x000f5dc9, 0x0010dd70,
];

// Runtime data 0xa8c24..0xa8c90: 27 pointers and hardware constants.
const RESOURCE_INITIALIZER_CALLBACKS:[u32;27]=[
    0x000a8cbc, 0x0010dd80, 0x000ad728, 0x0010dd8c,
    0x000b1b44, 0x0010dd84, 0x000b11b4, 0x0010dd88,
    0x000b06dc, 0x0010dd98, 0x000b49a4, 0x0010dd94,
    0x000b4924, 0x0010dd90, 0x000b2f08, 0x0010dd64,
    0x000a9038, 0x0010dd68, 0x000a91c4, 0x0010dd6c,
    0x000a9224, 0x0010dd78, 0x000a9518, 0x0010dd7c,
    0x000a9534, 0x0010dd74, 0x000a9284,
];

// Runtime data 0xa8c90..0xa8cbc: 11 pointers and hardware constants.
const RESOURCE_ERROR_MESSAGES:[u32;11]=[
    0x000f5a98, 0x000f8246, 0x000f821c, 0x000f81f1,
    0x000f5e51, 0x000f5e21, 0x000f5a6b, 0x000f5df8,
    0x000f81c1, 0x000f5c61, 0x000f4f2d,
];

/// Produce a partial executable function + explicitly distinguished data.
/// Only 4-byte-aligned, independently re-encoded instructions are emitted.
pub fn fragment() -> Result<Vec<u8>,String> {
    let mut c=Code::new();
    continue_prior_codec_release(&mut c)?;
    for value in CONTROLLER_STATUS_DIAGNOSTICS { c.emit(value)?; }
    account_codec_state_and_notify(&mut c)?;
    c.check(0xa835c)?;
    c.emit(CODEC_STATUS_MESSAGE)?;
    notify_on_codec_status(&mut c)?;
    c.check(0xa838c)?;
    c.emit(CONTROLLER_ERROR_CLASS)?;
    final_codec_recovery_branch(&mut c)?;
    c.check(0xa8398)?;
    for value in CONTROLLER_MESSAGE_CONSTANTS { c.emit(value)?; }
    initialize_codec_resource_layout(&mut c)?;
    for word in RESOURCE_INIT_LOGGING {c.emit(word)?;}
    for word in RESOURCE_INITIALIZER_CALLBACKS {c.emit(word)?;}
    for word in RESOURCE_ERROR_MESSAGES {c.emit(word)?;}
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn validates_exclusive_agent06_span() {
        assert_eq!(START,0xa8000);assert_eq!(END,0xa8cbc);
        assert_eq!(FILE_OFFSET,0xac000);
        assert_eq!(fragment().unwrap().len(),END-START);
    }
    #[test] fn literal_pools_are_not_treated_as_opcodes() {
        let bytes=fragment().unwrap();
        let word=|addr:usize|->u32 {let i=addr-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())};
        for (i,val) in CONTROLLER_STATUS_DIAGNOSTICS.iter().enumerate(){assert_eq!(word(0xa8114+4*i),*val);}
        assert_eq!(word(0xa835c),CODEC_STATUS_MESSAGE);
        assert_eq!(word(0xa838c),CONTROLLER_ERROR_CLASS);
        for (i,val) in CONTROLLER_MESSAGE_CONSTANTS.iter().enumerate(){assert_eq!(word(0xa8398+4*i),*val);}
        for (i,val) in RESOURCE_INIT_LOGGING.iter().enumerate(){assert_eq!(word(0xa8bc4+4*i),*val);}
        for (i,val) in RESOURCE_INITIALIZER_CALLBACKS.iter().enumerate(){assert_eq!(word(0xa8c24+4*i),*val);}
        for (i,val) in RESOURCE_ERROR_MESSAGES.iter().enumerate(){assert_eq!(word(0xa8c90+4*i),*val);}
    }
}
