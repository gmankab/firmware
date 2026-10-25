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
pub const END: usize=0xa9cf0;
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


/// Initialize three codec buffer classes and reset 28 fixed-size command slots.
fn clear_multichannel_codec_slots(c:&mut Code)->Result<(),String>{
    c.check(0xa8cbc)?;
    // VMA 0xa8cbc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8cc0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xa8cc4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xa8cc8
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // VMA 0xa8ccc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8cd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,668)?)?;
    // VMA 0xa8cd4
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa8cd8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,8)?)?;
    // VMA 0xa8cdc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,12)?)?;
    // VMA 0xa8ce0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8ce4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xa8ce8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8cec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2352)?)?;
    // VMA 0xa8cf0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8cf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1888)?)?;
    // VMA 0xa8cf8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa8cfc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8d00
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65536)?)?;
    // VMA 0xa8d04
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8d08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1504)?)?;
    // VMA 0xa8d0c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa8d10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8d14
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // VMA 0xa8d18
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8d1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xa8d20
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa8d24
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,8)?)?;
    // VMA 0xa8d28
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // VMA 0xa8d2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xa8d30
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,256)?)?;
    // VMA 0xa8d34
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // VMA 0xa8d38
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,260)?)?;
    // VMA 0xa8d3c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xa8d40
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::R2,R::R0,84)?)?;
    // VMA 0xa8d44
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,27)?)?;
    // VMA 0xa8d48
    c.b(C::Lt,0xa8d3c,false)?;
    // VMA 0xa8d4c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa8d50)
}

/// Initialize a single codec buffer and reset 28 command slot tags.
fn clear_single_codec_slots(c:&mut Code)->Result<(),String>{
    c.check(0xa8d50)?;
    // VMA 0xa8d50
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8d54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xa8d58
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xa8d5c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xa8d60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8d64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,676)?)?;
    // VMA 0xa8d68
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,0)?)?;
    // VMA 0xa8d6c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,4)?)?;
    // VMA 0xa8d70
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8d74
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xa8d78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8d7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2352)?)?;
    // VMA 0xa8d80
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa8d84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xa8d88
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,0)?)?;
    // VMA 0xa8d8c
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // VMA 0xa8d90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xa8d94
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,256)?)?;
    // VMA 0xa8d98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // VMA 0xa8d9c
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,260)?)?;
    // VMA 0xa8da0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xa8da4
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::R2,R::R0,84)?)?;
    // VMA 0xa8da8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,27)?)?;
    // VMA 0xa8dac
    c.b(C::Lt,0xa8da0,false)?;
    // VMA 0xa8db0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa8db4)
}

/// Find the first available codec slot and compute its entry address.
fn select_next_codec_slot(c:&mut Code)->Result<(),String>{
    c.check(0xa8db4)?;
    // VMA 0xa8db4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8db8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa8dbc
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xa8dc0
    c.b(C::Al,0xa8de4,true)?;
    // VMA 0xa8dc4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // VMA 0xa8dc8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // VMA 0xa8dcc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xa8dd0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // VMA 0xa8dd4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // VMA 0xa8dd8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xa8ddc
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // VMA 0xa8de0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa8de4)
}

/// Walk slot-state records, fill a selected entry or issue diagnostics when none is available.
fn find_codec_record_by_tag(c:&mut Code)->Result<(),String>{
    c.check(0xa8de4)?;
    // VMA 0xa8de4
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,256)?)?;
    // VMA 0xa8de8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // VMA 0xa8dec
    c.b(C::Lt,0xa8e24,false)?;
    // VMA 0xa8df0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa8df4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,81)?)?;
    // VMA 0xa8df8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8dfc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,-81)?)?;
    // VMA 0xa8e00
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xa8e04
    c.b(C::Ne,0xa8e14,false)?;
    // VMA 0xa8e08
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,-80)?)?;
    // VMA 0xa8e0c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // VMA 0xa8e10
    c.b(C::Eq,0xa8e64,false)?;
    // VMA 0xa8e14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa8e18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,84)?)?;
    // VMA 0xa8e1c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa8e20
    c.b(C::Lt,0xa8dfc,false)?;
    // VMA 0xa8e24
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8e28
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xa8e2c
    c.literal(R::R1,0xa8e90)?;
    // VMA 0xa8e30
    c.literal(R::R2,0xa8e94)?;
    // VMA 0xa8e34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8e38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,201)?)?;
    // VMA 0xa8e3c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8e40
    c.literal(R::R1,0xa8e98)?;
    // VMA 0xa8e44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa8e48
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa8e4c
    c.literal(R::R2,0xa8e9c)?;
    // VMA 0xa8e50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8e54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,202)?)?;
    // VMA 0xa8e58
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa8e5c
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // VMA 0xa8e60
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa8e64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // VMA 0xa8e68
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-77)?)?;
    // VMA 0xa8e6c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-81)?)?;
    // VMA 0xa8e70
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,40)?)?;
    // VMA 0xa8e74
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-69)?)?;
    // VMA 0xa8e78
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,9)?)?;
    // VMA 0xa8e7c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,-79)?)?;
    // VMA 0xa8e80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8e84
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xa8e88
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,-45)?)?;
    // VMA 0xa8e8c
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa8e90)
}

// Source diagnostic address pool, not executable ARM instructions.
const MISSING_CODEC_SLOT_MESSAGES:[u32;4]=[
    0x000f_5fa8,0x000f_82e9,0x000e_2788,0x000f_82fd,
];


/// Reset and validate the selected codec command slot, including byte-sized tag flags.
fn validate_slot_substate(c:&mut Code)->Result<(),String>{
    c.check(0xa8ea0)?;
    // VMA 0xa8ea0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,252)?)?;
    // VMA 0xa8ea4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8ea8
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,35)?)?;
    // VMA 0xa8eac
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xa8eb0
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa8eb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8eb8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xa8ebc
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa8ec0
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1)?)?;
    // VMA 0xa8ec4
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,4)?)?;
    // VMA 0xa8ec8
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,5)?)?;
    // VMA 0xa8ecc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // VMA 0xa8ed0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,20)?)?;
    // VMA 0xa8ed4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // VMA 0xa8ed8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,28)?)?;
    // VMA 0xa8edc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // VMA 0xa8ee0
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,80)?)?;
    // VMA 0xa8ee4
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa8ee8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,252)?)?;
    // VMA 0xa8eec
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,12)?)?;
    // VMA 0xa8ef0
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa8ef4
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::new(3)?,0)?)?;
    // VMA 0xa8ef8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa8efc
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,1)?)?;
    // VMA 0xa8f00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,252)?)?;
    // VMA 0xa8f04
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R0,0)?)?;
    // VMA 0xa8f08
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,1)?)?;
    // VMA 0xa8f0c
    c.emit(arm32::transfer_byte_postindexed_register(C::Al,false,R::new(3)?,R::R0,R::R1)?)?;
    // VMA 0xa8f10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8f14
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,18)?)?;
    // VMA 0xa8f18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,84)?)?;
    // VMA 0xa8f1c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R2,R::R0,R::R12)?)?;
    // VMA 0xa8f20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xa8f24
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(3)?,1)?)?;
    // VMA 0xa8f28
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,4)?)?;
    // VMA 0xa8f2c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,5)?)?;
    // VMA 0xa8f30
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,8)?)?;
    // VMA 0xa8f34
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,20)?)?;
    // VMA 0xa8f38
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,24)?)?;
    // VMA 0xa8f3c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,28)?)?;
    // VMA 0xa8f40
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,32)?)?;
    // VMA 0xa8f44
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa8f48
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8f4c
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xa8f50
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,256)?)?;
    // VMA 0xa8f54
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,1)?)?;
    // VMA 0xa8f58
    c.b(C::Lt,0xa8fa8,false)?;
    // VMA 0xa8f5c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // VMA 0xa8f60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa8f64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3)?)?;
    // VMA 0xa8f68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,80)?)?;
    // VMA 0xa8f6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8f70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xa8f74
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-80)?)?;
    // VMA 0xa8f78
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R2,-79)?)?;
    // VMA 0xa8f7c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-76)?)?;
    // VMA 0xa8f80
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,-75)?)?;
    // VMA 0xa8f84
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-72)?)?;
    // VMA 0xa8f88
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-60)?)?;
    // VMA 0xa8f8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-56)?)?;
    // VMA 0xa8f90
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-52)?)?;
    // VMA 0xa8f94
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,-48)?)?;
    // VMA 0xa8f98
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // VMA 0xa8f9c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,84)?)?;
    // VMA 0xa8fa0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xa8fa4
    c.b(C::Lt,0xa8f70,false)?;
    // VMA 0xa8fa8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa8fac
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,248)?)?;
    // VMA 0xa8fb0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa8fb4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa8fb8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xa8fbc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa8fc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa8fc4
    c.literal(R::new(4)?,0xa9034)?;
    // VMA 0xa8fc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa8fcc
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa8fd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,238)?)?;
    // VMA 0xa8fd4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xa8fd8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1)?)?;
    // VMA 0xa8fdc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xa8fe0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa8fe4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa8fe8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa8fec
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xa8ff0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,239)?)?;
    // VMA 0xa8ff4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xa8ff8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa8ffc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9000
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa9004
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa9008
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xa900c
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,20)?)?;
    // VMA 0xa9010
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa9014
    c.emit(arm32::transfer_word(C::Ne,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa9018
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // VMA 0xa901c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9020
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9024
    c.b(C::Al,0x7fed0,true)?;
    // VMA 0xa9028
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa902c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xa9030
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa9034)
}

/// Update an allocated slot and publish codec flags and resource pointer diagnostics.
fn publish_slot_allocator_pointer(c:&mut Code)->Result<(),String>{
    c.check(0xa9038)?;
    // VMA 0xa9038
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,8)?)?;
    // VMA 0xa903c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,200)?)?;
    // VMA 0xa9040
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R2)?)?;
    // VMA 0xa9044
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa9048
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa904c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa9050
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa9054
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,20)?)?;
    // VMA 0xa9058
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa905c
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::R0,8)?)?;
    // VMA 0xa9060
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // VMA 0xa9064
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa9068
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xa906c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xa9070
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,32)?)?;
    // VMA 0xa9074
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xa9078
    c.emit(arm32::mov_register(C::Al,R::R2,R::SP))?;
    // VMA 0xa907c
    c.b(C::Al,0x801a8,true)?;
    // VMA 0xa9080
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // VMA 0xa9084
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // VMA 0xa9088
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa908c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa9090
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa9094
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa9098
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa909c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::R1,R::new(3)?,R::R0)?)?;
    // VMA 0xa90a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    // VMA 0xa90a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // VMA 0xa90a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,8)?)?;
    // VMA 0xa90ac
    c.literal(R::R2,0xa90d8)?;
    // VMA 0xa90b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // VMA 0xa90b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa90b8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // VMA 0xa90bc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa90c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,308)?)?;
    // VMA 0xa90c4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa90c8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa90cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa90d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // VMA 0xa90d4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa90d8)
}

/// Confirm visibility of codec slot metadata and report its initialized state.
fn confirm_slot_visibility(c:&mut Code)->Result<(),String>{
    c.check(0xa90dc)?;
    // VMA 0xa90dc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa90e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa90e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa90e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa90ec
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa90f0
    c.literal(R::R2,0xa911c)?;
    // VMA 0xa90f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa90f8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // VMA 0xa90fc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa9100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,71)?)?;
    // VMA 0xa9104
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa9108
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xa910c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa9110
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa9114
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // VMA 0xa9118
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa911c)
}

/// Load and store indexed fields from fixed-width codec slot records, with explicit null checks.
fn read_write_slot_fields(c:&mut Code)->Result<(),String>{
    c.check(0xa9120)?;
    // VMA 0xa9120
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa9124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa9128
    c.emit(arm32::multiply_accumulate(C::Al,R::R12,R::R1,R::new(3)?,R::R0)?)?;
    // VMA 0xa912c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,4)?)?;
    // VMA 0xa9130
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,32)?)?;
    // VMA 0xa9134
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa9138
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa913c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa9140
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xa9144
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // VMA 0xa9148
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xa914c
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xa9150
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,0)?)?;
    // VMA 0xa9154
    c.b(C::Eq,0xa9184,false)?;
    // VMA 0xa9158
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa915c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa9160
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa9164
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,32)?)?;
    // VMA 0xa9168
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa916c
    c.b(C::Eq,0xa9188,false)?;
    // VMA 0xa9170
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xa9174
    c.b(C::Al,0x8019c,true)?;
    // VMA 0xa9178
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xa917c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xa9180
    c.b(C::Al,0xa9188,false)?;
    // VMA 0xa9184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xa9188
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa918c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // VMA 0xa9190
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9194
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa9198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa919c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa91a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,20)?)?;
    // VMA 0xa91a4
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa91a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa91ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa91b0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa91b4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,32)?)?;
    // VMA 0xa91b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa91bc
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa91c0
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa91c4
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa91c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa91cc
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa91d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,44)?)?;
    // VMA 0xa91d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xa91d8
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa91dc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa91e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa91e4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xa91e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa91ec
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xa91f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xa91f4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,0)?)?;
    // VMA 0xa91f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // VMA 0xa91fc
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // VMA 0xa9200
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,28)?)?;
    // VMA 0xa9204
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9208
    c.b(C::Eq,0xa921c,false)?;
    // VMA 0xa920c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xa9210
    c.b(C::Al,0x8019c,true)?;
    // VMA 0xa9214
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xa9218
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xa921c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // VMA 0xa9220
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9224
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa9228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa922c
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa9230
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,64)?)?;
    // VMA 0xa9234
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xa9238
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa923c
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // VMA 0xa9240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // VMA 0xa9244
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa9248
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    // VMA 0xa924c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xa9250
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa9254
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,260)?)?;
    // VMA 0xa9258
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa925c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa9260
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xa9264
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xa9268
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // VMA 0xa926c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,176)?)?;
    // VMA 0xa9270
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,160)?)?;
    // VMA 0xa9274
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // VMA 0xa9278
    c.b(C::Al,0x471c,true)?;
    // VMA 0xa927c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,176)?)?;
    // VMA 0xa9280
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9284
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,4)?)?;
    // VMA 0xa9288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,360)?)?;
    // VMA 0xa928c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xa9290
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R1,R::new(3)?,R::R12)?)?;
    // VMA 0xa9294
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,44)?)?;
    // VMA 0xa9298
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // VMA 0xa929c
    c.b(C::Eq,0xa92c0,false)?;
    // VMA 0xa92a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xa92a4
    c.b(C::Eq,0xa92b8,false)?;
    // VMA 0xa92a8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xa92ac
    c.emit(arm32::bx(C::Ne,R::LR))?;
    // VMA 0xa92b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xa92b4
    c.b(C::Al,0xa92bc,false)?;
    // VMA 0xa92b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xa92bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xa92c0
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa92c4
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa92c8)
}

/// Original repeated diagnostic pointer for indexed slot validation.
const CODEC_SLOT_CHECK_MESSAGE:u32=0x000f_82fd;


/// Derive codec record mappings and populate three bit-packed hardware
/// descriptor fields from per-record indices. Unknown mappings are kept
/// explicitly and do not become zero-filled stub instructions.
fn encode_slot_mapping_vectors(c:&mut Code)->Result<(),String>{
    c.check(0xa92c8)?;
    // VMA 0xa92c8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa92cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xa92d0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xa92d4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xa92d8
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xa92dc
    c.b(C::Eq,0xa950c,false)?;
    // VMA 0xa92e0
    c.literal(R::new(10)?,0xa9514)?;
    // VMA 0xa92e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xa92e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,2880)?)?;
    // VMA 0xa92ec
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R0,952)?)?;
    // VMA 0xa92f0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa92f4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,0)?)?;
    // VMA 0xa92f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(9)?,16)?)?;
    // VMA 0xa92fc
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xa9300
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xa9304
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,0)?)?;
    // VMA 0xa9308
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xa930c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xa9310
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xa9314
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xa9318
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // VMA 0xa931c
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xa9320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // VMA 0xa9324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,15)?)?;
    // VMA 0xa9328
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,13)?)?;
    // VMA 0xa932c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,9)?)?;
    // VMA 0xa9330
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,5)?)?;
    // VMA 0xa9334
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,3)?)?;
    // VMA 0xa9338
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,2)?)?;
    // VMA 0xa933c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,1)?)?;
    // VMA 0xa9340
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xa9344
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,12)?)?;
    // VMA 0xa9348
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,3)?)?;
    // VMA 0xa934c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,2)?)?;
    // VMA 0xa9350
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xa9354
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,8)?)?;
    // VMA 0xa9358
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,3)?)?;
    // VMA 0xa935c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,2)?)?;
    // VMA 0xa9360
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xa9364
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,4)?)?;
    // VMA 0xa9368
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,3)?)?;
    // VMA 0xa936c
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,2)?)?;
    // VMA 0xa9370
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,140)?)?;
    // VMA 0xa9374
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // VMA 0xa9378
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(8)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa937c
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xa9380
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // VMA 0xa9384
    c.b(C::Eq,0xa93f4,false)?;
    // VMA 0xa9388
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(8)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa938c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9390
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::new(3)?,112)?)?;
    // VMA 0xa9394
    c.emit(arm32::load_signed_byte(C::Al,R::new(7)?,R::new(7)?,112)?)?;
    // VMA 0xa9398
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xa939c
    c.b(C::Lt,0xa93b0,false)?;
    // VMA 0xa93a0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::new(3)?,Shift::Lsl,2)?)?;
    // VMA 0xa93a4
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::new(8)?,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xa93a8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa93ac
    c.b(C::Eq,0xa93e0,false)?;
    // VMA 0xa93b0
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::LR,Shift::Lsl,24)?)?;
    // VMA 0xa93b4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(3)?,Shift::Asr,24)?)?;
    // VMA 0xa93b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // VMA 0xa93bc
    c.b(C::Gt,0xa93e4,false)?;
    // VMA 0xa93c0
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(8)?,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xa93c4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::R12,R::new(3)?,Shift::Asr,24,false)?)?;
    // VMA 0xa93c8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(8)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa93cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::LR,1)?)?;
    // VMA 0xa93d0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,2)?)?;
    // VMA 0xa93d4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa93d8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,168)?)?;
    // VMA 0xa93dc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,168)?)?;
    // VMA 0xa93e0
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(3)?,R::new(5)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xa93e4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R2,140)?)?;
    // VMA 0xa93e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // VMA 0xa93ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,27)?)?;
    // VMA 0xa93f0
    c.b(C::Lt,0xa9374,false)?;
    // VMA 0xa93f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xa93f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa93fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa9400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa9404
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,280)?)?;
    // VMA 0xa9408
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,284)?)?;
    // VMA 0xa940c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,288)?)?;
    // VMA 0xa9410
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(7)?,R::new(5)?,1)?)?;
    // VMA 0xa9414
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xa9418
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,255)?)?;
    // VMA 0xa941c
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(7)?))?;
    // VMA 0xa9420
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,6)?)?;
    // VMA 0xa9424
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R1,15)?)?;
    // VMA 0xa9428
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,R::R2,false)?)?;
    // VMA 0xa942c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,5)?)?;
    // VMA 0xa9430
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,280)?)?;
    // VMA 0xa9434
    c.b(C::Ne,0xa9410,false)?;
    // VMA 0xa9438
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,22)?)?;
    // VMA 0xa943c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // VMA 0xa9440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xa9444
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa9448
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(5)?,R::R2,1)?)?;
    // VMA 0xa944c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xa9450
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,255)?)?;
    // VMA 0xa9454
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(5)?))?;
    // VMA 0xa9458
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,12)?)?;
    // VMA 0xa945c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,15)?)?;
    // VMA 0xa9460
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,R::new(7)?,false)?)?;
    // VMA 0xa9464
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,5)?)?;
    // VMA 0xa9468
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,284)?)?;
    // VMA 0xa946c
    c.b(C::Ne,0xa9448,false)?;
    // VMA 0xa9470
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,28)?)?;
    // VMA 0xa9474
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,12)?)?;
    // VMA 0xa9478
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xa947c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xa9480
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(5)?,R::R2,1)?)?;
    // VMA 0xa9484
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xa9488
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,255)?)?;
    // VMA 0xa948c
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(5)?))?;
    // VMA 0xa9490
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // VMA 0xa9494
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,15)?)?;
    // VMA 0xa9498
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,R::new(7)?,false)?)?;
    // VMA 0xa949c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,5)?)?;
    // VMA 0xa94a0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,288)?)?;
    // VMA 0xa94a4
    c.b(C::Ne,0xa9480,false)?;
    // VMA 0xa94a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,124)?)?;
    // VMA 0xa94ac
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,0)?)?;
    // VMA 0xa94b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa94b4
    c.b(C::Eq,0xa950c,false)?;
    // VMA 0xa94b8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,132)?)?;
    // VMA 0xa94bc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,1160)?)?;
    // VMA 0xa94c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa94c4
    c.b(C::Al,0xa94d0,false)?;
    // VMA 0xa94c8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // VMA 0xa94cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xa94d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xa94d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,284)?)?;
    // VMA 0xa94d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // VMA 0xa94dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,288)?)?;
    // VMA 0xa94e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,8)?)?;
    // VMA 0xa94e4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,64)?)?;
    // VMA 0xa94e8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,4)?)?;
    // VMA 0xa94ec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa94f0
    c.emit(arm32::transfer_word(C::Le,true,R::R0,R::R2,16)?)?;
    // VMA 0xa94f4
    c.emit(arm32::data_immediate(C::Le,Op::Bic,R::R0,R::R0,3)?)?;
    // VMA 0xa94f8
    c.emit(arm32::data_register(C::Le,Op::Sub,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa94fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,124)?)?;
    // VMA 0xa9500
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9504
    c.b(C::Cc,0xa94c8,false)?;
    // VMA 0xa9508
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,132)?)?;
    // VMA 0xa950c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xa9510
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa9514)
}

/// Global controller slot-table handle used by a single original PC load.
const SLOT_MANAGER_GLOBAL:u32=0x0010_dd64;

/// Publish the calculated slot pointer and copy command completion state.
/// The small helpers return through either original BX LR or stack epilogue.
fn resolve_codec_slot_hardware(c:&mut Code)->Result<(),String>{
    c.check(0xa9518)?;
    // VMA 0xa9518
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,60)?)?;
    // VMA 0xa951c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1092)?)?;
    // VMA 0xa9520
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // VMA 0xa9524
    c.b(C::Al,0x7ffc8,false)?;
    // VMA 0xa9528
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,60)?)?;
    // VMA 0xa952c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1092)?)?;
    // VMA 0xa9530
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xa9534
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa9538
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xa953c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xa9540
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xa9544
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,1092)?)?;
    // VMA 0xa9548
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa954c
    c.b(C::Eq,0xa9560,false)?;
    // VMA 0xa9550
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xa9554
    c.b(C::Al,0x8019c,true)?;
    // VMA 0xa9558
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xa955c
    c.b(C::Al,0xa9568,false)?;
    // VMA 0xa9560
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,1124)?)?;
    // VMA 0xa9564
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xa9568
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // VMA 0xa956c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xa9570
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,1092)?)?;
    // VMA 0xa9574
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa9578)
}


/// Begin preparing the next codec controller session: copy output-format
/// metadata to controller mapped addresses, compute input mode parameters,
/// and encode initial stream flags. This function continues after `0xa9800`.
fn begin_codec_controller_format(c:&mut Code)->Result<(),String>{
    c.check(0xa9578)?;
    // VMA 0xa9578
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xa957c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xa9580
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,60)?)?;
    // VMA 0xa9584
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xa9588
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,2448)?)?;
    // VMA 0xa958c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,2444)?)?;
    // VMA 0xa9590
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xa9594
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,604)?)?;
    // VMA 0xa9598
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa959c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,672)?)?;
    // VMA 0xa95a0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,44)?)?;
    // VMA 0xa95a4
    c.b(C::Eq,0xa9600,false)?;
    // VMA 0xa95a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,4000)?)?;
    // VMA 0xa95ac
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // VMA 0xa95b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,676)?)?;
    // VMA 0xa95b4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,3996)?)?;
    // VMA 0xa95b8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,992)?)?;
    // VMA 0xa95bc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,684)?)?;
    // VMA 0xa95c0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,996)?)?;
    // VMA 0xa95c4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,680)?)?;
    // VMA 0xa95c8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,256)?)?;
    // VMA 0xa95cc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,688)?)?;
    // VMA 0xa95d0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,264)?)?;
    // VMA 0xa95d4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,692)?)?;
    // VMA 0xa95d8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,76)?)?;
    // VMA 0xa95dc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(5)?,165)?)?;
    // VMA 0xa95e0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // VMA 0xa95e4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xa95e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,12)?)?;
    // VMA 0xa95ec
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::R2,1)?)?;
    // VMA 0xa95f0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,2,false)?)?;
    // VMA 0xa95f4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa95f8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // VMA 0xa95fc
    c.b(C::Al,0xa9604,false)?;
    // VMA 0xa9600
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(5)?,165)?)?;
    // VMA 0xa9604
    c.emit(arm32::load_halfword(C::Al,R::new(9)?,R::new(5)?,100)?)?;
    // VMA 0xa9608
    c.emit(arm32::load_halfword(C::Al,R::LR,R::new(5)?,98)?)?;
    // VMA 0xa960c
    c.literal(R::new(7)?,0xaa1f4)?;
    // VMA 0xa9610
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3670016)?)?;
    // VMA 0xa9614
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1536)?)?;
    // VMA 0xa9618
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::LR,R::new(9)?,Shift::Lsl,16,false)?)?;
    // VMA 0xa961c
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9620
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,320)?)?;
    // VMA 0xa9624
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,110)?)?;
    // VMA 0xa9628
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,111)?)?;
    // VMA 0xa962c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,96)?)?;
    // VMA 0xa9630
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xa9634
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,28)?)?;
    // VMA 0xa9638
    c.emit(arm32::transfer_byte(C::Al,true,R::new(10)?,R::new(5)?,609)?)?;
    // VMA 0xa963c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,64)?)?;
    // VMA 0xa9640
    c.emit(arm32::data_register(C::Al,Op::And,R::R12,R::new(4)?,R::R2,Shift::Lsl,19,false)?)?;
    // VMA 0xa9644
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // VMA 0xa9648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,458752)?)?;
    // VMA 0xa964c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsl,9,false)?)?;
    // VMA 0xa9650
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,16,false)?)?;
    // VMA 0xa9654
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9658
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa965c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,15)?)?;
    // VMA 0xa9660
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,14)?)?;
    // VMA 0xa9664
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,7)?)?;
    // VMA 0xa9668
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,6,false)?)?;
    // VMA 0xa966c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9670
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // VMA 0xa9674
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(10)?,Shift::Lsl,5,false)?)?;
    // VMA 0xa9678
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(4)?))?;
    // VMA 0xa967c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,24)?)?;
    // VMA 0xa9680
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // VMA 0xa9688
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,3,false)?)?;
    // VMA 0xa968c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,507904)?)?;
    // VMA 0xa9690
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9694
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xa9698
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xa969c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa96a0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,1)?)?;
    // VMA 0xa96a4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R1,3)?)?;
    // VMA 0xa96a8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa96ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,324)?)?;
    // VMA 0xa96b0
    c.literal(R::R0,0xaa1f8)?;
    // VMA 0xa96b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,328)?)?;
    // VMA 0xa96b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12582912)?)?;
    // VMA 0xa96bc
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsl,22,false)?)?;
    // VMA 0xa96c0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa96c4
    c.emit(arm32::mov_shifted_register(C::Al,R::R12,R::new(9)?,Shift::Lsl,16)?)?;
    // VMA 0xa96c8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa96cc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R1,36)?)?;
    // VMA 0xa96d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,332)?)?;
    // VMA 0xa96d4
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,12)?)?;
    // VMA 0xa96d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,11)?)?;
    // VMA 0xa96dc
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,35)?)?;
    // VMA 0xa96e0
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,14,false)?)?;
    // VMA 0xa96e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15872)?)?;
    // VMA 0xa96e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xa96ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xa96f0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,9,false)?)?;
    // VMA 0xa96f4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa96f8
    c.b(C::Ne,0xa9718,false)?;
    // VMA 0xa96fc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,36)?)?;
    // VMA 0xa9700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xa9704
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9708
    c.b(C::Eq,0xa9718,false)?;
    // VMA 0xa970c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,19)?)?;
    // VMA 0xa9710
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xa9714
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xa9718
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // VMA 0xa971c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,8,false)?)?;
    // VMA 0xa9720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,128)?)?;
    // VMA 0xa9724
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(4)?,480)?)?;
    // VMA 0xa9728
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(3)?,178)?)?;
    // VMA 0xa972c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(3)?,179)?)?;
    // VMA 0xa9730
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,7,false)?)?;
    // VMA 0xa9734
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,16)?)?;
    // VMA 0xa9738
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa973c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xa9740
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(5)?,Shift::Lsl,5,false)?)?;
    // VMA 0xa9744
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // VMA 0xa9748
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa974c
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,33)?)?;
    // VMA 0xa9750
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,4,false)?)?;
    // VMA 0xa9754
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1048576)?)?;
    // VMA 0xa9758
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa975c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xa9760
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9764
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,340)?)?;
    // VMA 0xa9768
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,44)?)?;
    // VMA 0xa976c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,32)?)?;
    // VMA 0xa9770
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,92)?)?;
    // VMA 0xa9774
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9778
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,19)?)?;
    // VMA 0xa977c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xa9780
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(6)?,R::R0,Shift::Lsl,20,false)?)?;
    // VMA 0xa9784
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,22,false)?)?;
    // VMA 0xa9788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,131072)?)?;
    // VMA 0xa978c
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,17,false)?)?;
    // VMA 0xa9790
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9794
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // VMA 0xa9798
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,344)?)?;
    // VMA 0xa979c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R12,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xa97a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,336)?)?;
    // VMA 0xa97a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R1,9)?)?;
    // VMA 0xa97a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xa97ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,348)?)?;
    // VMA 0xa97b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xa97b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,616)?)?;
    // VMA 0xa97b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,608)?)?;
    // VMA 0xa97bc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa97c0
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa97c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1160)?)?;
    // VMA 0xa97c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,12)?)?;
    // VMA 0xa97cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xa97d0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa97d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa97d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,292)?)?;
    // VMA 0xa97dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2788)?)?;
    // VMA 0xa97e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,276)?)?;
    // VMA 0xa97e4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,110)?)?;
    // VMA 0xa97e8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa97ec
    c.b(C::Ne,0xa97fc,false)?;
    // VMA 0xa97f0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,111)?)?;
    // VMA 0xa97f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa97f8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa97fc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,577)?)?;
    c.check(0xa9800)
}


/// Complete the fast-path plane scaling, VLC lookup and codec buffer
/// accounting of the session begun at 0xa9578. The function has an early
/// return at 0xa9cec, but additional error handlers follow at 0xa9cf0.
fn write_codec_plane_blending(c:&mut Code)->Result<(),String>{
    c.check(0xa9800)?;
    // VMA 0xa9800
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,23)?)?;
    // VMA 0xa9804
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,128)?)?;
    // VMA 0xa9808
    c.b(C::Eq,0xa9818,false)?;
    // VMA 0xa980c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1048)?)?;
    // VMA 0xa9810
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xa9814
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R1,R::R1,4194304)?)?;
    // VMA 0xa9818
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,111)?)?;
    // VMA 0xa981c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,458752)?)?;
    // VMA 0xa9820
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,96)?)?;
    // VMA 0xa9824
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(4)?))?;
    // VMA 0xa9828
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xa982c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3670016)?)?;
    // VMA 0xa9830
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(7)?,R::R0,Shift::Lsl,19,false)?)?;
    // VMA 0xa9834
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9838
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,49152)?)?;
    // VMA 0xa983c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9840
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,14,false)?)?;
    // VMA 0xa9844
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa9848
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa984c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9)?)?;
    // VMA 0xa9850
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,244)?)?;
    // VMA 0xa9854
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(5)?,98)?)?;
    // VMA 0xa9858
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,100)?)?;
    // VMA 0xa985c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xa9860
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,536870912)?)?;
    // VMA 0xa9864
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,248)?)?;
    // VMA 0xa9868
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,178)?)?;
    // VMA 0xa986c
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,608)?)?;
    // VMA 0xa9870
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsl,29,false)?)?;
    // VMA 0xa9874
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,35)?)?;
    // VMA 0xa9878
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,30)?)?;
    // VMA 0xa987c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9880
    c.b(C::Ne,0xa9894,false)?;
    // VMA 0xa9884
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,36)?)?;
    // VMA 0xa9888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xa988c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9890
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa9894
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,169)?)?;
    // VMA 0xa9898
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa989c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,234881024)?)?;
    // VMA 0xa98a0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,165)?)?;
    // VMA 0xa98a4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,168)?)?;
    // VMA 0xa98a8
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(5)?,164)?)?;
    // VMA 0xa98ac
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,28,false)?)?;
    // VMA 0xa98b0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,170)?)?;
    // VMA 0xa98b4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,25,false)?)?;
    // VMA 0xa98b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,131072)?)?;
    // VMA 0xa98bc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,29360128)?)?;
    // VMA 0xa98c4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(10)?,19)?)?;
    // VMA 0xa98c8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,22,false)?)?;
    // VMA 0xa98cc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,786432)?)?;
    // VMA 0xa98d4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,18,false)?)?;
    // VMA 0xa98d8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14336)?)?;
    // VMA 0xa98e0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,11,false)?)?;
    // VMA 0xa98e4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,224)?)?;
    // VMA 0xa98ec
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,5,false)?)?;
    // VMA 0xa98f0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(10)?,20)?)?;
    // VMA 0xa98f4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa98f8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,2)?)?;
    // VMA 0xa98fc
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,17,false)?)?;
    // VMA 0xa9900
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,114688)?)?;
    // VMA 0xa9908
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,14,false)?)?;
    // VMA 0xa990c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9910
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1792)?)?;
    // VMA 0xa9914
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R12,Shift::Lsl,8,false)?)?;
    // VMA 0xa9918
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,2)?)?;
    // VMA 0xa991c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9920
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R12,R::new(4)?,Shift::Lsl,1,false)?)?;
    // VMA 0xa9924
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,229376)?)?;
    // VMA 0xa9928
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa992c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(6)?,1)?)?;
    // VMA 0xa9930
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2097152)?)?;
    // VMA 0xa9934
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9938
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2147483648)?)?;
    // VMA 0xa993c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,260)?)?;
    // VMA 0xa9940
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,41)?)?;
    // VMA 0xa9944
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,173)?)?;
    // VMA 0xa9948
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,10)?)?;
    // VMA 0xa994c
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(10)?,9)?)?;
    // VMA 0xa9950
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,172)?)?;
    // VMA 0xa9954
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,22,false)?)?;
    // VMA 0xa9958
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,177)?)?;
    // VMA 0xa995c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xa9960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,768)?)?;
    // VMA 0xa9964
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,21,false)?)?;
    // VMA 0xa9968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1835008)?)?;
    // VMA 0xa996c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9970
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,174)?)?;
    // VMA 0xa9974
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,18,false)?)?;
    // VMA 0xa9978
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,171)?)?;
    // VMA 0xa997c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9980
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,112)?)?;
    // VMA 0xa9984
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9988
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::new(3)?,Shift::Lsl,8,false)?)?;
    // VMA 0xa998c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9990
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // VMA 0xa9994
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::LR,Shift::Lsl,7,false)?)?;
    // VMA 0xa9998
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa999c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // VMA 0xa99a0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,6,false)?)?;
    // VMA 0xa99a4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa99a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,56)?)?;
    // VMA 0xa99ac
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,3,false)?)?;
    // VMA 0xa99b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,33554432)?)?;
    // VMA 0xa99b4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa99b8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R2,7)?)?;
    // VMA 0xa99bc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa99c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,67108864)?)?;
    // VMA 0xa99c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,264)?)?;
    // VMA 0xa99c8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,17)?)?;
    // VMA 0xa99cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,8)?)?;
    // VMA 0xa99d0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,12)?)?;
    // VMA 0xa99d4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,11)?)?;
    // VMA 0xa99d8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,26,false)?)?;
    // VMA 0xa99dc
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,25,false)?)?;
    // VMA 0xa99e0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,7)?)?;
    // VMA 0xa99e4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa99e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33030144)?)?;
    // VMA 0xa99ec
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,19,false)?)?;
    // VMA 0xa99f0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa99f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,516096)?)?;
    // VMA 0xa99f8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,13,false)?)?;
    // VMA 0xa99fc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // VMA 0xa9a04
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,3,false)?)?;
    // VMA 0xa9a08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xa9a0c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a10
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,18)?)?;
    // VMA 0xa9a14
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xa9a18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,992)?)?;
    // VMA 0xa9a1c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a20
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,15)?)?;
    // VMA 0xa9a24
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R12,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xa9a28
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a2c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,14)?)?;
    // VMA 0xa9a30
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,1)?)?;
    // VMA 0xa9a34
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a38
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,268)?)?;
    // VMA 0xa9a3c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,113)?)?;
    // VMA 0xa9a40
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,114)?)?;
    // VMA 0xa9a44
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,5,false)?)?;
    // VMA 0xa9a48
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // VMA 0xa9a4c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a50
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,272)?)?;
    // VMA 0xa9a54
    c.b(C::Al,0x2f82c,true)?;
    // VMA 0xa9a58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,671088640)?)?;
    // VMA 0xa9a5c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9a60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3670016)?)?;
    // VMA 0xa9a64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,458752)?)?;
    // VMA 0xa9a68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,536870912)?)?;
    // VMA 0xa9a6c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,536870912)?)?;
    // VMA 0xa9a70
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,252)?)?;
    // VMA 0xa9a74
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(5)?,98)?)?;
    // VMA 0xa9a78
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,100)?)?;
    // VMA 0xa9a7c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xa9a80
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,316)?)?;
    // VMA 0xa9a84
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,110)?)?;
    // VMA 0xa9a88
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,111)?)?;
    // VMA 0xa9a8c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,96)?)?;
    // VMA 0xa9a90
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::R1,Shift::Lsl,19,false)?)?;
    // VMA 0xa9a94
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9a98
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xa9a9c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::R0,Shift::Lsl,8,false)?)?;
    // VMA 0xa9aa0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,255,true)?)?;
    // VMA 0xa9aa4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xa9aa8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,23,false)?)?;
    // VMA 0xa9aac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1073741824)?)?;
    // VMA 0xa9ab0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ab4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ab8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9)?)?;
    // VMA 0xa9abc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,308)?)?;
    // VMA 0xa9ac0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,165)?)?;
    // VMA 0xa9ac4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,168)?)?;
    // VMA 0xa9ac8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,113)?)?;
    // VMA 0xa9acc
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,114)?)?;
    // VMA 0xa9ad0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,30,false)?)?;
    // VMA 0xa9ad4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,7)?)?;
    // VMA 0xa9ad8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,29,false)?)?;
    // VMA 0xa9adc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ae0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,234881024)?)?;
    // VMA 0xa9ae4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,25,false)?)?;
    // VMA 0xa9ae8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9aec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2031616)?)?;
    // VMA 0xa9af0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xa9af4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9af8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63488)?)?;
    // VMA 0xa9afc
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,11,false)?)?;
    // VMA 0xa9b00
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9b04
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,312)?)?;
    // VMA 0xa9b08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,104)?)?;
    // VMA 0xa9b0c
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,100)?)?;
    // VMA 0xa9b10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xa9b14
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,224)?)?;
    // VMA 0xa9b18
    c.literal(R::R0,0xaa1fc)?;
    // VMA 0xa9b1c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // VMA 0xa9b20
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xa9b24
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xa9b28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,2880)?)?;
    // VMA 0xa9b2c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xa9b30
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xa9b34
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xa9b38
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xa9b3c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,140)?)?;
    // VMA 0xa9b40
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xa9b44
    c.b(C::Al,0xa925c,true)?;
    // VMA 0xa9b48
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xa9b4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xa9b50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xa9b54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // VMA 0xa9b58
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xa9b5c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,112)?)?;
    // VMA 0xa9b60
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa9b64
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xa9b68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa9b6c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,112)?)?;
    // VMA 0xa9b70
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xa9b74
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,9)?)?;
    // VMA 0xa9b78
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // VMA 0xa9b7c
    c.b(C::Lt,0xa9ba4,false)?;
    // VMA 0xa9b80
    c.literal(R::R1,0xaa200)?;
    // VMA 0xa9b84
    c.literal(R::R2,0xaa204)?;
    // VMA 0xa9b88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9b8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,143)?)?;
    // VMA 0xa9b90
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9b94
    c.literal(R::R1,0xaa208)?;
    // VMA 0xa9b98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9b9c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9ba0
    c.b(C::Al,0xa9bac,false)?;
    // VMA 0xa9ba4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xa9ba8
    c.b(C::Lt,0xa9bf4,false)?;
    // VMA 0xa9bac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9bb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // VMA 0xa9bb4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9bb8
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::R2,140)?)?;
    // VMA 0xa9bbc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9bc0
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xa9bc4
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,112)?)?;
    // VMA 0xa9bc8
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xa9bcc
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::new(7)?,R::new(3)?,R::R1,R::new(7)?)?)?;
    // VMA 0xa9bd0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,5)?)?;
    // VMA 0xa9bd4
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R2,168)?)?;
    // VMA 0xa9bd8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // VMA 0xa9bdc
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R2,R::new(3)?,R::R1,R::R2)?)?;
    // VMA 0xa9be0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,12)?)?;
    // VMA 0xa9be4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xa9be8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa9bec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9bf0
    c.b(C::Ne,0xa9bb4,false)?;
    // VMA 0xa9bf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xa9bf8
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(10)?))?;
    // VMA 0xa9bfc
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,228)?)?;
    // VMA 0xa9c00
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,232)?)?;
    // VMA 0xa9c04
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,236)?)?;
    // VMA 0xa9c08
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // VMA 0xa9c0c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,175)?)?;
    // VMA 0xa9c10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1088)?)?;
    // VMA 0xa9c14
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,1092)?)?;
    // VMA 0xa9c18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9c1c
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xa9c20
    c.b(C::Eq,0xa9cbc,false)?;
    // VMA 0xa9c24
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,39)?)?;
    // VMA 0xa9c28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9c2c
    c.b(C::Eq,0xa9c64,false)?;
    // VMA 0xa9c30
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,48)?)?;
    // VMA 0xa9c34
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xa9c38
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2147483648)?)?;
    // VMA 0xa9c3c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,228)?)?;
    // VMA 0xa9c40
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // VMA 0xa9c44
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,232)?)?;
    // VMA 0xa9c48
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,56)?)?;
    // VMA 0xa9c4c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,236)?)?;
    // VMA 0xa9c50
    c.literal(R::R0,0xaa20c)?;
    // VMA 0xa9c54
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xa9c58
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa9c5c
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xa9c60
    c.b(C::Al,0xa9cbc,false)?;
    // VMA 0xa9c64
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,176)?)?;
    // VMA 0xa9c68
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9c6c
    c.b(C::Eq,0xa9c9c,false)?;
    // VMA 0xa9c70
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,624)?)?;
    // VMA 0xa9c74
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2147483648)?)?;
    // VMA 0xa9c78
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,228)?)?;
    // VMA 0xa9c7c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,628)?)?;
    // VMA 0xa9c80
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,232)?)?;
    // VMA 0xa9c84
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,632)?)?;
    // VMA 0xa9c88
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,236)?)?;
    // VMA 0xa9c8c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,636)?)?;
    // VMA 0xa9c90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1092)?)?;
    // VMA 0xa9c94
    c.b(C::Al,0x7ffc8,true)?;
    // VMA 0xa9c98
    c.b(C::Al,0xa9cbc,false)?;
    // VMA 0xa9c9c
    c.literal(R::R0,0xaa210)?;
    // VMA 0xa9ca0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,150994944)?)?;
    // VMA 0xa9ca4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,228)?)?;
    // VMA 0xa9ca8
    c.literal(R::R1,0xaa214)?;
    // VMA 0xa9cac
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,232)?)?;
    // VMA 0xa9cb0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,236)?)?;
    // VMA 0xa9cb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9cb8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1092)?)?;
    // VMA 0xa9cbc
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,256)?)?;
    // VMA 0xa9cc0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,19)?)?;
    // VMA 0xa9cc4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9cc8
    c.b(C::Eq,0xa9cd8,false)?;
    // VMA 0xa9ccc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1100)?)?;
    // VMA 0xa9cd0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9cd4
    c.b(C::Eq,0xa9cf0,false)?;
    // VMA 0xa9cd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa9cdc
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // VMA 0xa9ce0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,132)?)?;
    // VMA 0xa9ce4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,136)?)?;
    // VMA 0xa9ce8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xa9cec
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa9cf0)
}

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
    clear_multichannel_codec_slots(&mut c)?;
    clear_single_codec_slots(&mut c)?;
    select_next_codec_slot(&mut c)?;
    find_codec_record_by_tag(&mut c)?;
    for addr in MISSING_CODEC_SLOT_MESSAGES {c.emit(addr)?;}
    validate_slot_substate(&mut c)?;
    c.check(0xa9034)?;
    c.emit(CODEC_SLOT_CHECK_MESSAGE)?;
    publish_slot_allocator_pointer(&mut c)?;
    c.check(0xa90d8)?;
    c.emit(CODEC_SLOT_CHECK_MESSAGE)?;
    confirm_slot_visibility(&mut c)?;
    c.check(0xa911c)?;
    c.emit(CODEC_SLOT_CHECK_MESSAGE)?;
    read_write_slot_fields(&mut c)?;
    encode_slot_mapping_vectors(&mut c)?;
    c.emit(SLOT_MANAGER_GLOBAL)?;
    resolve_codec_slot_hardware(&mut c)?;
    begin_codec_controller_format(&mut c)?;
    write_codec_plane_blending(&mut c)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn validates_exclusive_agent06_span() {
        assert_eq!(START,0xa8000);assert_eq!(END,0xa9cf0);
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
        for (i,val) in MISSING_CODEC_SLOT_MESSAGES.iter().enumerate(){assert_eq!(word(0xa8e90+4*i),*val);}
        for address in [0xa9034,0xa90d8,0xa911c] {assert_eq!(word(address),CODEC_SLOT_CHECK_MESSAGE);}
        assert_eq!(word(0xa9514),SLOT_MANAGER_GLOBAL);
    }
}
