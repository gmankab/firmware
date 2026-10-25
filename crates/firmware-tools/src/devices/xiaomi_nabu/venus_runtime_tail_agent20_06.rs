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
pub const END: usize=0xace00;
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

/// The codec-plane session's longer error/fallback path: accumulation over
/// per-plane halfword tables, controller bounds checks and diagnostics. An
/// independently classified fourteen-word literal pool follows at 0xaa1f4.
fn handle_codec_plane_blending_failures(c:&mut Code)->Result<(),String>{
    c.check(0xa9cf0)?;
    // VMA 0xa9cf0
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(9)?,24)?)?;
    // VMA 0xa9cf4
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,22)?)?;
    // VMA 0xa9cf8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xa9cfc
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // VMA 0xa9d00
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,28)?)?;
    // VMA 0xa9d04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9d08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // VMA 0xa9d0c
    c.literal(R::R2,0xaa218)?;
    // VMA 0xa9d10
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xa9d14
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // VMA 0xa9d18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xa9d1c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xa9d20
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa9d24
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xa9d28
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,2492)?)?;
    // VMA 0xa9d2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,0)?)?;
    // VMA 0xa9d30
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,24)?)?;
    // VMA 0xa9d34
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(9)?,22)?)?;
    // VMA 0xa9d38
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa9d3c
    c.b(C::Eq,0xa9d90,false)?;
    // VMA 0xa9d40
    c.emit(arm32::mov_shifted_register(C::Al,R::LR,R::new(6)?,Shift::Lsl,2)?)?;
    // VMA 0xa9d44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R12,4)?)?;
    // VMA 0xa9d48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xa9d4c
    c.b(C::Eq,0xa9d80,false)?;
    // VMA 0xa9d50
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xa9d54
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9d58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xa9d5c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xa9d60
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::R0,0)?)?;
    // VMA 0xa9d64
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // VMA 0xa9d68
    c.emit(arm32::load_halfword_postindexed(C::Al,R::R2,R::R0,2)?)?;
    // VMA 0xa9d6c
    c.emit(arm32::multiply_register(C::Al,R::new(4)?,R::new(7)?,R::R2)?)?;
    // VMA 0xa9d70
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::new(5)?,Shift::Lsl,2)?)?;
    // VMA 0xa9d74
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xa9d78
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9d7c
    c.b(C::Lt,0xa9d68,false)?;
    // VMA 0xa9d80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xa9d84
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xa9d88
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9d8c
    c.b(C::Lt,0xa9d48,false)?;
    // VMA 0xa9d90
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xa9d94
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::new(6)?)?)?;
    // VMA 0xa9d98
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(10)?))?;
    // VMA 0xa9d9c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9da0
    c.b(C::Eq,0xa9e04,false)?;
    // VMA 0xa9da4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,0)?)?;
    // VMA 0xa9da8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R0,1)?)?;
    // VMA 0xa9dac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xa9db0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xa9db4
    c.b(C::Eq,0xa9df4,false)?;
    // VMA 0xa9db8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9dbc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R12,4)?)?;
    // VMA 0xa9dc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // VMA 0xa9dc4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xa9dc8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,2)?)?;
    // VMA 0xa9dcc
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::new(7)?,R::new(7)?,2,true)?)?;
    // VMA 0xa9dd0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9dd4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xa9dd8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,4)?)?;
    // VMA 0xa9ddc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xa9de0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,4)?)?;
    // VMA 0xa9de4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // VMA 0xa9de8
    c.b(C::Ne,0xa9dc4,false)?;
    // VMA 0xa9dec
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::new(5)?,1)?)?;
    // VMA 0xa9df0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xa9df4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xa9df8
    c.emit(arm32::transfer_word_register(C::Ne,true,R::R0,R::R12,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xa9dfc
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9e00
    c.emit(arm32::transfer_word_register(C::Ne,false,R::R0,R::R12,R::new(7)?,Shift::Lsl,2)?)?;
    // VMA 0xa9e04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,106)?)?;
    // VMA 0xa9e08
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(5)?))?;
    // VMA 0xa9e0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xa9e10
    c.emit(arm32::load_halfword_register_preindexed_writeback(C::Al,R::R2,R::new(10)?,R::R0)?)?;
    // VMA 0xa9e14
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xa9e18
    c.emit(arm32::load_halfword_preindexed_writeback(C::Al,R::new(3)?,R::R0,-2)?)?;
    // VMA 0xa9e1c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // VMA 0xa9e20
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(3)?,R::R2)?)?;
    // VMA 0xa9e24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xa9e28
    c.b(C::Eq,0xaa08c,false)?;
    // VMA 0xa9e2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xa9e30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xa9e34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xa9e38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xa9e3c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,32)?)?;
    // VMA 0xa9e40
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // VMA 0xa9e44
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xa9e48
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // VMA 0xa9e4c
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    // VMA 0xa9e50
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xa9e54
    c.b(C::Al,0x81838,true)?;
    // VMA 0xa9e58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,255)?)?;
    // VMA 0xa9e5c
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xa9e60
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // VMA 0xa9e64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,65280)?)?;
    // VMA 0xa9e68
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9e6c
    c.b(C::Eq,0xa9ea8,false)?;
    // VMA 0xa9e70
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xa9e74
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9e78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9e7c
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(9)?))?;
    // VMA 0xa9e80
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::R2,0)?)?;
    // VMA 0xa9e84
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(10)?))?;
    // VMA 0xa9e88
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(10)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9e8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xa9e90
    c.b(C::Lt,0xa9eb0,false)?;
    // VMA 0xa9e94
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa9e98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,2)?)?;
    // VMA 0xa9e9c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ea0
    c.b(C::Lt,0xa9e80,false)?;
    // VMA 0xa9ea4
    c.b(C::Al,0xa9eb4,false)?;
    // VMA 0xa9ea8
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(9)?))?;
    // VMA 0xa9eac
    c.b(C::Al,0xa9eb4,false)?;
    // VMA 0xa9eb0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // VMA 0xa9eb4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // VMA 0xa9eb8
    c.emit(arm32::load_halfword(C::Al,R::R1,R::R0,24)?)?;
    // VMA 0xa9ebc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xa9ec0
    c.b(C::Eq,0xa9efc,false)?;
    // VMA 0xa9ec4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xa9ec8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ecc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xa9ed0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9ed4
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::R2,0)?)?;
    // VMA 0xa9ed8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(7)?))?;
    // VMA 0xa9edc
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ee0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xa9ee4
    c.b(C::Lt,0xa9f04,false)?;
    // VMA 0xa9ee8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xa9eec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,2)?)?;
    // VMA 0xa9ef0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xa9ef4
    c.b(C::Lt,0xa9ed4,false)?;
    // VMA 0xa9ef8
    c.b(C::Al,0xa9f08,false)?;
    // VMA 0xa9efc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9f00
    c.b(C::Al,0xa9f08,false)?;
    // VMA 0xa9f04
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xa9f08
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9f0c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(7)?,R::R2,R::new(10)?)?)?;
    // VMA 0xa9f10
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // VMA 0xa9f14
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9f18
    c.b(C::Le,0xa9f80,false)?;
    // VMA 0xa9f1c
    c.literal(R::R1,0xaa21c)?;
    // VMA 0xa9f20
    c.literal(R::R2,0xaa220)?;
    // VMA 0xa9f24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,54)?)?;
    // VMA 0xa9f28
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa9f2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // VMA 0xa9f30
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,12)?)?;
    // VMA 0xa9f34
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xa9f38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9f3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xa9f40
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9f44
    c.literal(R::R1,0xaa208)?;
    // VMA 0xa9f48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9f4c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9f50
    c.literal(R::R2,0xaa218)?;
    // VMA 0xa9f54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,55)?)?;
    // VMA 0xa9f58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xa9f5c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xa9f60
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xa9f64
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xa9f68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,255)?)?;
    // VMA 0xa9f6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,65280)?)?;
    // VMA 0xa9f70
    c.emit(arm32::load_halfword(C::Al,R::LR,R::R0,0)?)?;
    // VMA 0xa9f74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xa9f78
    c.emit(arm32::load_halfword(C::Al,R::R0,R::R0,0)?)?;
    // VMA 0xa9f7c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // VMA 0xa9f80
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,44)?)?;
    // VMA 0xa9f84
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-32)?)?;
    // VMA 0xa9f88
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(10)?,Shift::Lsl,1)?)?;
    // VMA 0xa9f8c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,2492)?)?;
    // VMA 0xa9f90
    c.emit(arm32::load_halfword_register_preindexed_writeback(C::Al,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xa9f94
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(6)?,Shift::Lsl,2)?)?;
    // VMA 0xa9f98
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xa9f9c
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(4)?,R::R0,R::R1)?)?;
    // VMA 0xa9fa0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // VMA 0xa9fa4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::LR,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9fa8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xa9fac
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // VMA 0xa9fb0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xa9fb4
    c.b(C::Lt,0xaa038,false)?;
    // VMA 0xa9fb8
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xa9fbc
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // VMA 0xa9fc0
    c.literal(R::new(7)?,0xaa220)?;
    // VMA 0xa9fc4
    c.literal(R::R1,0xaa224)?;
    // VMA 0xa9fc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,61)?)?;
    // VMA 0xa9fcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9fd0
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,12)?)?;
    // VMA 0xa9fd4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xa9fd8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xa9fdc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9fe0
    c.literal(R::new(10)?,0xaa208)?;
    // VMA 0xa9fe4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xa9fe8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xa9fec
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xa9ff0
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(9)?,0)?)?;
    // VMA 0xa9ff4
    c.literal(R::R1,0xaa228)?;
    // VMA 0xa9ff8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,62)?)?;
    // VMA 0xa9ffc
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // VMA 0xaa000
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xaa004
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xaa008
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xaa00c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaa010
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaa014
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaa018
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xaa01c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaa020
    c.literal(R::R2,0xaa218)?;
    // VMA 0xaa024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // VMA 0xaa028
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaa02c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xaa030
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xaa034
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,44)?)?;
    // VMA 0xaa038
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,1196)?)?;
    // VMA 0xaa03c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,1,false)?)?;
    // VMA 0xaa040
    c.emit(arm32::store_halfword(C::Al,R::new(8)?,R::R0,0)?)?;
    // VMA 0xaa044
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xaa048
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,1192)?)?;
    // VMA 0xaa04c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xaa050
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa054
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // VMA 0xaa058
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaa05c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,32)?)?;
    // VMA 0xaa060
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,36)?)?;
    // VMA 0xaa064
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xaa068
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(10)?,0)?)?;
    // VMA 0xaa06c
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(7)?,0)?)?;
    // VMA 0xaa070
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(9)?,22)?)?;
    // VMA 0xaa074
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(3)?,R::R2)?)?;
    // VMA 0xaa078
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa07c
    c.b(C::Lt,0xa9e40,false)?;
    // VMA 0xaa080
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,24)?)?;
    // VMA 0xaa084
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,24)?)?;
    // VMA 0xaa088
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xaa08c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,254)?)?;
    // VMA 0xaa090
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,768)?)?;
    // VMA 0xaa094
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32512)?)?;
    // VMA 0xaa098
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xaa09c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa0a0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xaa0a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,256)?)?;
    // VMA 0xaa0a8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,21)?)?;
    // VMA 0xaa0ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa0b0
    c.b(C::Eq,0xaa154,false)?;
    // VMA 0xaa0b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xaa0b8
    c.b(C::Eq,0xaa11c,false)?;
    // VMA 0xaa0bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaa0c0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xaa0c4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(5)?,167)?)?;
    // VMA 0xaa0c8
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa0cc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,2)?)?;
    // VMA 0xaa0d0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // VMA 0xaa0d4
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,22)?)?;
    // VMA 0xaa0d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,768)?)?;
    // VMA 0xaa0dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xaa0e0
    c.b(C::Cc,0xaa110,false)?;
    // VMA 0xaa0e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,36)?)?;
    // VMA 0xaa0e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xaa0ec
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaa0f0
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::R2)?)?;
    // VMA 0xaa0f4
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xaa0f8
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(8)?)?)?;
    // VMA 0xaa0fc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xaa100
    c.emit(arm32::store_halfword_postindexed(C::Al,R::R1,R::new(4)?,2)?)?;
    // VMA 0xaa104
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,22)?)?;
    // VMA 0xaa108
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa10c
    c.b(C::Lt,0xaa0ec,false)?;
    // VMA 0xaa110
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,24)?)?;
    // VMA 0xaa114
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xaa118
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(7)?))?;
    // VMA 0xaa11c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa120
    c.b(C::Eq,0xa9cd8,false)?;
    // VMA 0xaa124
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(5)?,167)?)?;
    // VMA 0xaa128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaa12c
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(10)?,0)?)?;
    // VMA 0xaa130
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::R2)?)?;
    // VMA 0xaa134
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xaa138
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(5)?)?)?;
    // VMA 0xaa13c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xaa140
    c.emit(arm32::store_halfword_postindexed(C::Al,R::R1,R::new(4)?,2)?)?;
    // VMA 0xaa144
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(9)?,24)?)?;
    // VMA 0xaa148
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa14c
    c.b(C::Lt,0xaa12c,false)?;
    // VMA 0xaa150
    c.b(C::Al,0xa9cd8,false)?;
    // VMA 0xaa154
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xaa158
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,2)?)?;
    // VMA 0xaa15c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // VMA 0xaa160
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(4)?,0)?)?;
    // VMA 0xaa164
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,768)?)?;
    // VMA 0xaa168
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xaa16c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // VMA 0xaa170
    c.b(C::Cc,0xaa1a8,false)?;
    // VMA 0xaa174
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,167)?)?;
    // VMA 0xaa178
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xaa17c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaa180
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::R0,-2)?)?;
    // VMA 0xaa184
    c.emit(arm32::load_halfword(C::Al,R::new(4)?,R::new(9)?,22)?)?;
    // VMA 0xaa188
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xaa18c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // VMA 0xaa190
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa194
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::new(6)?,R::new(3)?)?)?;
    // VMA 0xaa198
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(5)?,R::R1,2)?)?;
    // VMA 0xaa19c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::new(4)?,1)?)?;
    // VMA 0xaa1a0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa1a4
    c.b(C::Lt,0xaa180,false)?;
    // VMA 0xaa1a8
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(9)?,24)?)?;
    // VMA 0xaa1ac
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R1,0)?)?;
    // VMA 0xaa1b0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xaa1b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // VMA 0xaa1b8
    c.b(C::Cc,0xa9cd8,false)?;
    // VMA 0xaa1bc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,167)?)?;
    // VMA 0xaa1c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // VMA 0xaa1c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xaa1c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xaa1cc
    c.emit(arm32::load_halfword_postindexed(C::Al,R::new(6)?,R::R0,2)?)?;
    // VMA 0xaa1d0
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::new(9)?,24)?)?;
    // VMA 0xaa1d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xaa1d8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa1dc
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::new(7)?,R::R2)?)?;
    // VMA 0xaa1e0
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(6)?,R::R1,2)?)?;
    // VMA 0xaa1e4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(5)?,1)?)?;
    // VMA 0xaa1e8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa1ec
    c.b(C::Lt,0xaa1cc,false)?;
    // VMA 0xaa1f0
    c.b(C::Al,0xa9cd8,false)?;
    c.check(0xaa1f4)
}

const CODEC_BLEND_LITERALS:[u32;14]=[
    0x3fff_3fff, // packed signed 14-bit mask
    0x0030_6090, // codec controller callback address
    0x0010_dd64, // codec data region
    0x000f_84c5, // diagnostic string pointers
    0x000f_850a,
    0x000e_2788,
    0x0010_dd78, // controller configuration pointer
    0x7777_7777, // VLQ pattern (data, not an instruction)
    0x0077_7777,
    0x000f_8334,
    0x000f_836d,
    0x000f_839b,
    0x000f_83ab,
    0x000f_83d4,
];

/// Prepare the controller codec output descriptor: pack per-field mode
/// bits, fill a packed quantization table, and invoke the installed callback
/// with the finished record. Two constant global addresses follow the BX.
fn encode_codec_controller_output_record(c:&mut Code)->Result<(),String>{
    c.check(0xaa22c)?;
    // VMA 0xaa22c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xaa230
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xaa234
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // VMA 0xaa238
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xaa23c
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // VMA 0xaa240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // VMA 0xaa244
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1160)?)?;
    // VMA 0xaa248
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1164)?)?;
    // VMA 0xaa24c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,2444)?)?;
    // VMA 0xaa250
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // VMA 0xaa254
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,12)?)?;
    // VMA 0xaa258
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xaa25c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xaa260
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xaa264
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2452)?)?;
    // VMA 0xaa268
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2604)?)?;
    // VMA 0xaa26c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa270
    c.b(C::Ne,0xaa284,false)?;
    // VMA 0xaa274
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // VMA 0xaa278
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1048576)?)?;
    // VMA 0xaa27c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,42)?)?;
    // VMA 0xaa280
    c.b(C::Eq,0xaa2bc,false)?;
    // VMA 0xaa284
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,616)?)?;
    // VMA 0xaa288
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(4)?,R::R1)?)?;
    // VMA 0xaa28c
    c.b(C::Al,0x81838,true)?;
    // VMA 0xaa290
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,167)?)?;
    // VMA 0xaa294
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    // VMA 0xaa298
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,261120)?)?;
    // VMA 0xaa29c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,786432)?)?;
    // VMA 0xaa2a0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,7,false)?)?;
    // VMA 0xaa2a4
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // VMA 0xaa2a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xaa2ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // VMA 0xaa2b0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsr,3,false)?)?;
    // VMA 0xaa2b4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa2b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,1048576)?)?;
    // VMA 0xaa2bc
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,16)?)?;
    // VMA 0xaa2c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24576)?)?;
    // VMA 0xaa2c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,532676608)?)?;
    // VMA 0xaa2c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(5)?,2880)?)?;
    // VMA 0xaa2cc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // VMA 0xaa2d0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,32)?)?;
    // VMA 0xaa2d4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2468)?)?;
    // VMA 0xaa2d8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa2dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,8)?)?;
    // VMA 0xaa2e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xaa2e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2468)?)?;
    // VMA 0xaa2e8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,2504)?)?;
    // VMA 0xaa2ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4096)?)?;
    // VMA 0xaa2f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2508)?)?;
    // VMA 0xaa2f4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2686)?)?;
    // VMA 0xaa2f8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,12,false)?)?;
    // VMA 0xaa2fc
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2734)?)?;
    // VMA 0xaa300
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,13,false)?)?;
    // VMA 0xaa304
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // VMA 0xaa308
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xaa30c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xaa310
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xaa314
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3840)?)?;
    // VMA 0xaa318
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa31c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2735)?)?;
    // VMA 0xaa320
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,8,false)?)?;
    // VMA 0xaa324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,240)?)?;
    // VMA 0xaa328
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa32c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2736)?)?;
    // VMA 0xaa330
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,4,false)?)?;
    // VMA 0xaa334
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // VMA 0xaa338
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa33c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2724)?)?;
    // VMA 0xaa340
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xaa344
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa348
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2608)?)?;
    // VMA 0xaa34c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,3)?)?;
    // VMA 0xaa350
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xaa354
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa358
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,8388608)?)?;
    // VMA 0xaa35c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // VMA 0xaa360
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,2728)?)?;
    // VMA 0xaa364
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2688)?)?;
    // VMA 0xaa368
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2687)?)?;
    // VMA 0xaa36c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,2731)?)?;
    // VMA 0xaa370
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,22,false)?)?;
    // VMA 0xaa374
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2031616)?)?;
    // VMA 0xaa378
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xaa37c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,2031616)?)?;
    // VMA 0xaa380
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,63488)?)?;
    // VMA 0xaa388
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,11,false)?)?;
    // VMA 0xaa38c
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,63488)?)?;
    // VMA 0xaa390
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa394
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // VMA 0xaa398
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,3,false)?)?;
    // VMA 0xaa39c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa3a0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2732)?)?;
    // VMA 0xaa3a4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,7)?)?;
    // VMA 0xaa3a8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa3ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3932160)?)?;
    // VMA 0xaa3b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // VMA 0xaa3b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8388608)?)?;
    // VMA 0xaa3b8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // VMA 0xaa3bc
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(5)?,2725)?)?;
    // VMA 0xaa3c0
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,2723)?)?;
    // VMA 0xaa3c4
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,2684)?)?;
    // VMA 0xaa3c8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,2685)?)?;
    // VMA 0xaa3cc
    c.emit(arm32::data_register(C::Ne,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,23,false)?)?;
    // VMA 0xaa3d0
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2727)?)?;
    // VMA 0xaa3d4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,18,false)?)?;
    // VMA 0xaa3d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2601)?)?;
    // VMA 0xaa3dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4194304)?)?;
    // VMA 0xaa3e0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,22,false)?)?;
    // VMA 0xaa3e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    // VMA 0xaa3e8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa3ec
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa3f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,131072)?)?;
    // VMA 0xaa3f4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,17,false)?)?;
    // VMA 0xaa3f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // VMA 0xaa3fc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65536)?)?;
    // VMA 0xaa404
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xaa408
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa40c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32768)?)?;
    // VMA 0xaa410
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa414
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa418
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2737)?)?;
    // VMA 0xaa41c
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,14,false)?)?;
    // VMA 0xaa420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15872)?)?;
    // VMA 0xaa424
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa428
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2730)?)?;
    // VMA 0xaa42c
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,9,false)?)?;
    // VMA 0xaa430
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,496)?)?;
    // VMA 0xaa434
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa438
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2729)?)?;
    // VMA 0xaa43c
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,4,false)?)?;
    // VMA 0xaa440
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa444
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2726)?)?;
    // VMA 0xaa448
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,15)?)?;
    // VMA 0xaa44c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa450
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,20)?)?;
    // VMA 0xaa454
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,24)?)?;
    // VMA 0xaa458
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2512)?)?;
    // VMA 0xaa45c
    c.literal(R::R0,0xaa6e8)?;
    // VMA 0xaa460
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,12)?)?;
    // VMA 0xaa464
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,8)?)?;
    // VMA 0xaa468
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xaa46c
    c.emit(arm32::mov_register(C::Al,R::R0,R::R12))?;
    // VMA 0xaa470
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,4)?)?;
    // VMA 0xaa474
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xaa478
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // VMA 0xaa47c
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(5)?))?;
    // VMA 0xaa480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // VMA 0xaa484
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(9)?,R::new(10)?,32)?)?;
    // VMA 0xaa488
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xaa48c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,13)?)?;
    // VMA 0xaa490
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,9)?)?;
    // VMA 0xaa494
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,5)?)?;
    // VMA 0xaa498
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,3)?)?;
    // VMA 0xaa49c
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,2)?)?;
    // VMA 0xaa4a0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(10)?,1)?)?;
    // VMA 0xaa4a4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // VMA 0xaa4a8
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(9)?,R::R1,12)?)?;
    // VMA 0xaa4ac
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,3)?)?;
    // VMA 0xaa4b0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,2)?)?;
    // VMA 0xaa4b4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xaa4b8
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(9)?,R::R1,8)?)?;
    // VMA 0xaa4bc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,3)?)?;
    // VMA 0xaa4c0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,2)?)?;
    // VMA 0xaa4c4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xaa4c8
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(9)?,R::R1,4)?)?;
    // VMA 0xaa4cc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,3)?)?;
    // VMA 0xaa4d0
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,2)?)?;
    // VMA 0xaa4d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,984)?)?;
    // VMA 0xaa4d8
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // VMA 0xaa4dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,2048)?)?;
    // VMA 0xaa4e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,2896)?)?;
    // VMA 0xaa4e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // VMA 0xaa4e8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa4ec
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // VMA 0xaa4f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // VMA 0xaa4f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xaa4f8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,0)?)?;
    // VMA 0xaa4fc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(4)?,3)?)?;
    // VMA 0xaa500
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xaa504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xaa508
    c.b(C::Lt,0xaa5b8,false)?;
    // VMA 0xaa50c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,32)?)?;
    // VMA 0xaa510
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xaa514
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // VMA 0xaa518
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,0)?)?;
    // VMA 0xaa51c
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::R2.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xaa520
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,112)?)?;
    // VMA 0xaa524
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(6)?))?;
    // VMA 0xaa528
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,12)?)?;
    // VMA 0xaa52c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4)?)?;
    // VMA 0xaa530
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,16)?)?;
    // VMA 0xaa534
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // VMA 0xaa538
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa53c
    c.emit(arm32::data_register(C::Gt,Op::Add,R::new(6)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa540
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xaa544
    c.emit(arm32::load_signed_byte(C::Al,R::new(4)?,R::new(6)?,0)?)?;
    // VMA 0xaa548
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // VMA 0xaa54c
    c.emit(arm32::data_register(C::Gt,Op::Add,R::new(6)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa550
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::new(6)?,0)?)?;
    // VMA 0xaa554
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa558
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R2,R::R2,R::R2,Shift::Asr,31,false)?)?;
    // VMA 0xaa55c
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::R2,Shift::Lsl,4)?)?;
    // VMA 0xaa560
    c.emit(arm32::data_register(C::Gt,Op::Orr,R::new(6)?,R::new(4)?,R::R2,Shift::Lsl,4,false)?)?;
    // VMA 0xaa564
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa568
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // VMA 0xaa56c
    c.emit(arm32::data_register(C::Gt,Op::Add,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa570
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa574
    c.emit(arm32::data_register(C::Gt,Op::Add,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa578
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R2,0)?)?;
    // VMA 0xaa57c
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::new(3)?,0)?)?;
    // VMA 0xaa580
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // VMA 0xaa584
    c.emit(arm32::transfer_byte_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(3)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xaa588
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::LR,2)?)?;
    // VMA 0xaa58c
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::R1,R::R1,Shift::Asr,31,false)?)?;
    // VMA 0xaa590
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R2,R::R2,R::R2,Shift::Asr,31,false)?)?;
    // VMA 0xaa594
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(6)?,R::R2,Shift::Lsl,8,false)?)?;
    // VMA 0xaa598
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // VMA 0xaa59c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,12,false)?)?;
    // VMA 0xaa5a0
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,8)?)?;
    // VMA 0xaa5a4
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,1)?)?;
    // VMA 0xaa5a8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,0)?)?;
    // VMA 0xaa5ac
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(4)?,3)?)?;
    // VMA 0xaa5b0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa5b4
    c.b(C::Lt,0xaa518,false)?;
    // VMA 0xaa5b8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xaa5bc
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(7)?,R::new(4)?,3,true)?)?;
    // VMA 0xaa5c0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R1,8)?)?;
    // VMA 0xaa5c4
    c.b(C::Eq,0xaa630,false)?;
    // VMA 0xaa5c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,24)?)?;
    // VMA 0xaa5cc
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // VMA 0xaa5d0
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(5)?,Shift::Lsl,2)?)?;
    // VMA 0xaa5d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xaa5d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // VMA 0xaa5dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xaa5e0
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,32)?)?;
    // VMA 0xaa5e4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa5e8
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xaa5ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,1)?)?;
    // VMA 0xaa5f0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xaa5f4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaa5f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa5fc
    c.emit(arm32::data_register(C::Gt,Op::Add,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaa600
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa604
    c.emit(arm32::load_signed_byte(C::Al,R::R1,R::R2,112)?)?;
    // VMA 0xaa608
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::R1,R::R1,Shift::Asr,31,false)?)?;
    // VMA 0xaa60c
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::new(8)?,R::new(8)?,R::R1,Shift::Lsl,R::new(10)?,false)?)?;
    // VMA 0xaa610
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,4)?)?;
    // VMA 0xaa614
    c.b(C::Lt,0xaa5e4,false)?;
    // VMA 0xaa618
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::LR,1)?)?;
    // VMA 0xaa61c
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(8)?,Shift::Lsr,8)?)?;
    // VMA 0xaa620
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(8)?,R::new(3)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xaa624
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaa628
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,20)?)?;
    // VMA 0xaa62c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // VMA 0xaa630
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xaa634
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xaa638
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // VMA 0xaa63c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // VMA 0xaa640
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,2)?)?;
    // VMA 0xaa644
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(10)?,R::new(10)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,8)?)?;
    // VMA 0xaa64c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,68)?)?;
    // VMA 0xaa650
    c.b(C::Ne,0xaa4e4,false)?;
    // VMA 0xaa654
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // VMA 0xaa658
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // VMA 0xaa65c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2686)?)?;
    // VMA 0xaa660
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa664
    c.b(C::Eq,0xaa6b0,false)?;
    // VMA 0xaa668
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2608)?)?;
    // VMA 0xaa66c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xaa670
    c.b(C::Eq,0xaa6a0,false)?;
    // VMA 0xaa674
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa678
    c.b(C::Ne,0xaa6b0,false)?;
    // VMA 0xaa67c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2725)?)?;
    // VMA 0xaa680
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xaa684
    c.b(C::Hi,0xaa6b0,false)?;
    // VMA 0xaa688
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2726)?)?;
    // VMA 0xaa68c
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,1)?)?;
    // VMA 0xaa690
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // VMA 0xaa694
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xaa698
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xaa69c
    c.b(C::Al,0xaa6a8,false)?;
    // VMA 0xaa6a0
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2726)?)?;
    // VMA 0xaa6a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xaa6a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2896)?)?;
    // VMA 0xaa6ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,136)?)?;
    // VMA 0xaa6b0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,132)?)?;
    // VMA 0xaa6b4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xaa6b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa6bc
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::R1,48)?)?;
    // VMA 0xaa6c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,132)?)?;
    // VMA 0xaa6c4
    c.b(C::Al,0x20d0,true)?;
    // VMA 0xaa6c8
    c.literal(R::R0,0xaa6ec)?;
    // VMA 0xaa6cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2608)?)?;
    // VMA 0xaa6d0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xaa6d4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xaa6d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xaa6dc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xaa6e0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xaa6e4
    c.emit(arm32::bx(C::Al,R::new(3)?))?;
    c.check(0xaa6e8)
}

const CODEC_OUTPUT_GLOBALS:[u32;2]=[
    0x0010_dd64, // shared codec state
    0x0010_dd74, // controller event listener
];

/// Program the per-channel codec MMIO command list and auxiliary quality
/// tables from initialized buffer descriptors. The original routine ends in
/// a POP {...,PC}, followed by seven non-instruction pointers/constants.
fn encode_codec_channel_command_queue(c:&mut Code)->Result<(),String>{
    c.check(0xaa6f0)?;
    // VMA 0xaa6f0
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xaa6f4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xaa6f8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,108)?)?;
    // VMA 0xaa6fc
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // VMA 0xaa700
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // VMA 0xaa704
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // VMA 0xaa708
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // VMA 0xaa70c
    c.literal(R::R1,0xab030)?;
    // VMA 0xaa710
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1164)?)?;
    // VMA 0xaa714
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xaa718
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xaa71c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,4)?)?;
    // VMA 0xaa720
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // VMA 0xaa724
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // VMA 0xaa728
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xaa72c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaa730
    c.literal(R::R1,0xab034)?;
    // VMA 0xaa734
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xaa738
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaa73c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,852)?)?;
    // VMA 0xaa740
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2896)?)?;
    // VMA 0xaa744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xaa748
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xaa74c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xaa750
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa754
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xaa758
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R1,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa75c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-44)?)?;
    // VMA 0xaa760
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(8)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xaa764
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,608)?)?;
    // VMA 0xaa768
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-48)?)?;
    // VMA 0xaa76c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(5)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xaa770
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3024)?)?;
    // VMA 0xaa774
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xaa778
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R1,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa77c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,64)?)?;
    // VMA 0xaa780
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xaa784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,972)?)?;
    // VMA 0xaa788
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xaa78c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa790
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,612)?)?;
    // VMA 0xaa794
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,60)?)?;
    // VMA 0xaa798
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa79c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xaa7a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xaa7a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa7a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa7ac
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,608)?)?;
    // VMA 0xaa7b0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa7b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa7b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,612)?)?;
    // VMA 0xaa7bc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa7c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa7c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // VMA 0xaa7c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa7cc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,616)?)?;
    // VMA 0xaa7d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa7d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa7d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,620)?)?;
    // VMA 0xaa7dc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa7e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa7e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // VMA 0xaa7e8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa7ec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa7f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,616)?)?;
    // VMA 0xaa7f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa7f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa7fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,620)?)?;
    // VMA 0xaa800
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa804
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xaa80c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa810
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,624)?)?;
    // VMA 0xaa814
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa818
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa81c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,628)?)?;
    // VMA 0xaa820
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa824
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa828
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xaa82c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa830
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa834
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,624)?)?;
    // VMA 0xaa838
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa83c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa840
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,628)?)?;
    // VMA 0xaa844
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa848
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa84c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // VMA 0xaa850
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa854
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,632)?)?;
    // VMA 0xaa858
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa85c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa860
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,636)?)?;
    // VMA 0xaa864
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa868
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa86c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // VMA 0xaa870
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa874
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa878
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,632)?)?;
    // VMA 0xaa87c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa880
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa884
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xaa888
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,636)?)?;
    // VMA 0xaa88c
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa890
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa894
    c.literal(R::R0,0xab038)?;
    // VMA 0xaa898
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xaa89c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xaa8a0
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xaa8a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,98)?)?;
    // VMA 0xaa8a8
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,255)?)?;
    // VMA 0xaa8ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xaa8b0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaa8b4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // VMA 0xaa8b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xaa8bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xaa8c0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,0)?)?;
    // VMA 0xaa8c4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // VMA 0xaa8c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xaa8cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1024)?)?;
    // VMA 0xaa8d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xaa8d4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::new(6)?,Shift::Lsl,15)?)?;
    // VMA 0xaa8d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa8dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // VMA 0xaa8e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,316)?)?;
    // VMA 0xaa8e4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaa8e8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa8ec
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaa8f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaa8f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,308)?)?;
    // VMA 0xaa8f8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaa8fc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa900
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaa904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1440)?)?;
    // VMA 0xaa908
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,312)?)?;
    // VMA 0xaa90c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa910
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa914
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaa918
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,40)?)?;
    // VMA 0xaa91c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,256)?)?;
    // VMA 0xaa920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa924
    c.b(C::Eq,0xaaa88,false)?;
    // VMA 0xaa928
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,324)?)?;
    // VMA 0xaa92c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // VMA 0xaa930
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65280)?)?;
    // VMA 0xaa934
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R1,Shift::Lsr,15,false)?)?;
    // VMA 0xaa938
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(3)?,3)?)?;
    // VMA 0xaa93c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // VMA 0xaa940
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // VMA 0xaa944
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,64)?)?;
    // VMA 0xaa948
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xaa94c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // VMA 0xaa950
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xaa954
    c.b(C::Cc,0xaa9a8,false)?;
    // VMA 0xaa958
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,768)?)?;
    // VMA 0xaa95c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(9)?,2)?)?;
    // VMA 0xaa960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xaa964
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(6)?,R::new(9)?,R::new(6)?)?)?;
    // VMA 0xaa968
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,768)?)?;
    // VMA 0xaa96c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa970
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(7)?,0)?)?;
    // VMA 0xaa974
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa978
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa97c
    c.b(C::Cc,0xaaa30,false)?;
    // VMA 0xaa980
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xaa984
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // VMA 0xaa988
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa98c
    c.b(C::Cc,0xaa96c,false)?;
    // VMA 0xaa990
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,40)?)?;
    // VMA 0xaa994
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xaa998
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xaa99c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // VMA 0xaa9a0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xaa9a4
    c.b(C::Al,0xaa9c0,false)?;
    // VMA 0xaa9a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,40)?)?;
    // VMA 0xaa9ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa9b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xaa9b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // VMA 0xaa9b8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xaa9bc
    c.b(C::Eq,0xaa9e8,false)?;
    // VMA 0xaa9c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,1056)?)?;
    // VMA 0xaa9c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,768)?)?;
    // VMA 0xaa9c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xaa9cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xaa9d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaa9d4
    c.emit(arm32::load_halfword_postindexed(C::Al,R::new(6)?,R::R2,2)?)?;
    // VMA 0xaa9d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xaa9dc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa9e0
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xaa9e4
    c.b(C::Ne,0xaa9d4,false)?;
    // VMA 0xaa9e8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,256)?)?;
    // VMA 0xaa9ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // VMA 0xaa9f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16128)?)?;
    // VMA 0xaa9f4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsr,1,false)?)?;
    // VMA 0xaa9f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa9fc
    c.b(C::Eq,0xaaa88,false)?;
    // VMA 0xaaa00
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,36)?)?;
    // VMA 0xaaa04
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(9)?,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xaaa08
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaaa0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xaaa10
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // VMA 0xaaa14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xaaa18
    c.emit(arm32::load_halfword_postindexed(C::Al,R::new(6)?,R::new(3)?,2)?)?;
    // VMA 0xaaa1c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xaaa20
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaaa24
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R2,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xaaa28
    c.b(C::Cc,0xaaa18,false)?;
    // VMA 0xaaa2c
    c.b(C::Al,0xaaa88,false)?;
    // VMA 0xaaa30
    c.literal(R::R1,0xab03c)?;
    // VMA 0xaaa34
    c.literal(R::R2,0xab040)?;
    // VMA 0xaaa38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaaa3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,884)?)?;
    // VMA 0xaaa40
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaa44
    c.literal(R::R1,0xab034)?;
    // VMA 0xaaa48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaaa4c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaa50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,40)?)?;
    // VMA 0xaaa54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xaaa58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xaaa5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xaaa60
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaa64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,32768)?)?;
    // VMA 0xaaa68
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaaa6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,1056)?)?;
    // VMA 0xaaa70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xaaa74
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaaa78
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaaa7c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,36)?)?;
    // VMA 0xaaa80
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaa84
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaaa88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,1376)?)?;
    // VMA 0xaaa8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,36)?)?;
    // VMA 0xaaa90
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,352)?)?;
    // VMA 0xaaa94
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // VMA 0xaaa98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaaa9c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xaaaa0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,420)?)?;
    // VMA 0xaaaa4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,228)?)?;
    // VMA 0xaaaa8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaaac
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaab0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,424)?)?;
    // VMA 0xaaab4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,232)?)?;
    // VMA 0xaaab8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaabc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaac0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,428)?)?;
    // VMA 0xaaac4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,236)?)?;
    // VMA 0xaaac8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaacc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaad0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,8192)?)?;
    // VMA 0xaaad4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,340)?)?;
    // VMA 0xaaad8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaadc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,4)?)?;
    // VMA 0xaaae0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,344)?)?;
    // VMA 0xaaae4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xaaae8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaaec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,8)?)?;
    // VMA 0xaaaf0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,336)?)?;
    // VMA 0xaaaf4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xaaaf8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaafc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,16)?)?;
    // VMA 0xaab00
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,348)?)?;
    // VMA 0xaab04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xaab08
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,6144)?)?;
    // VMA 0xaab10
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,320)?)?;
    // VMA 0xaab14
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab18
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,4)?)?;
    // VMA 0xaab1c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,324)?)?;
    // VMA 0xaab20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // VMA 0xaab24
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,12)?)?;
    // VMA 0xaab2c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,328)?)?;
    // VMA 0xaab30
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // VMA 0xaab34
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab38
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,6272)?)?;
    // VMA 0xaab3c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,120)?)?;
    // VMA 0xaab40
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,2192)?)?;
    // VMA 0xaab48
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,4096)?)?;
    // VMA 0xaab4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaab50
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab54
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,32)?)?;
    // VMA 0xaab58
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,7168)?)?;
    // VMA 0xaab5c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,332)?)?;
    // VMA 0xaab60
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab64
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,24)?)?;
    // VMA 0xaab68
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,64)?)?;
    // VMA 0xaab6c
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,60)?)?;
    // VMA 0xaab70
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-44)?)?;
    // VMA 0xaab74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,166)?)?;
    // VMA 0xaab78
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::LR))?;
    // VMA 0xaab7c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xaab80
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaab84
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,16)?)?;
    // VMA 0xaab88
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaab8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,182)?)?;
    // VMA 0xaab90
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaab94
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab98
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,0)?)?;
    // VMA 0xaab9c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-48)?)?;
    // VMA 0xaaba0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::new(5)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaba4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // VMA 0xaaba8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaabac
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,612)?)?;
    // VMA 0xaabb0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // VMA 0xaabb4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(9)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaabb8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaabbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,167)?)?;
    // VMA 0xaabc0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // VMA 0xaabc4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaabc8
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,64)?)?;
    // VMA 0xaabcc
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,0)?)?;
    // VMA 0xaabd0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // VMA 0xaabd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xaabd8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,620)?)?;
    // VMA 0xaabdc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // VMA 0xaabe0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // VMA 0xaabe4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaabe8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // VMA 0xaabec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,128)?)?;
    // VMA 0xaabf0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaabf4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // VMA 0xaabf8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,628)?)?;
    // VMA 0xaabfc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xaac00
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // VMA 0xaac04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,183)?)?;
    // VMA 0xaac08
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaac0c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaac10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // VMA 0xaac14
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xaac18
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,192)?)?;
    // VMA 0xaac1c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // VMA 0xaac20
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaac24
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xaac28
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,636)?)?;
    // VMA 0xaac2c
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xaac30
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // VMA 0xaac34
    c.b(C::Ne,0xaab70,false)?;
    // VMA 0xaac38
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,32)?)?;
    // VMA 0xaac3c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,224)?)?;
    // VMA 0xaac40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xaac44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xaac48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xaac4c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,108)?)?;
    // VMA 0xaac50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xaac54
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaac58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xaac5c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,36)?)?;
    // VMA 0xaac60
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,712)?)?;
    // VMA 0xaac64
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xaac68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xaac6c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // VMA 0xaac70
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaac74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xaac78
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xaac7c
    c.b(C::Al,0x3027c,true)?;
    // VMA 0xaac80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,120)?)?;
    // VMA 0xaac84
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-44)?)?;
    // VMA 0xaac88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xaac8c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xaac90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // VMA 0xaac94
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,116)?)?;
    // VMA 0xaac98
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaac9c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // VMA 0xaaca0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,104)?)?;
    // VMA 0xaaca4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaca8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // VMA 0xaacac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,2400)?)?;
    // VMA 0xaacb0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::R0,12288)?)?;
    // VMA 0xaacb4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,356)?)?;
    // VMA 0xaacb8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xaacbc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xaacc0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1152)?)?;
    // VMA 0xaacc4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(10)?))?;
    // VMA 0xaacc8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // VMA 0xaaccc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,128)?)?;
    // VMA 0xaacd0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // VMA 0xaacd4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,172)?)?;
    // VMA 0xaacd8
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-48)?)?;
    // VMA 0xaacdc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-60)?)?;
    // VMA 0xaace0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,178)?)?;
    // VMA 0xaace4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaace8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-64)?)?;
    // VMA 0xaacec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaacf0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xaacf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaacf8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // VMA 0xaacfc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xaad00
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-40)?)?;
    // VMA 0xaad04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaad08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xaad0c
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaad10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaad14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,216)?)?;
    // VMA 0xaad18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R0,160)?)?;
    // VMA 0xaad1c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-52)?)?;
    // VMA 0xaad20
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // VMA 0xaad24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xaad28
    c.b(C::Ne,0xaad90,false)?;
    // VMA 0xaad2c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-56)?)?;
    // VMA 0xaad30
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-40)?)?;
    // VMA 0xaad34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaad38
    c.emit(arm32::mov_register(C::Al,R::R2,R::LR))?;
    // VMA 0xaad3c
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // VMA 0xaad40
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xaad44
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::R0,152)?)?;
    // VMA 0xaad48
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xaad4c
    c.emit(arm32::data_register(C::Eq,Op::Add,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaad50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xaad54
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaad58
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xaad5c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xaad60
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaad64
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xaad68
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaad6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::R2,4096)?)?;
    // VMA 0xaad70
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::new(9)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaad74
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-48)?)?;
    // VMA 0xaad78
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(8)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaad7c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // VMA 0xaad80
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-52)?)?;
    // VMA 0xaad84
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-44)?)?;
    // VMA 0xaad88
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::R0,208)?)?;
    // VMA 0xaad8c
    c.b(C::Al,0xaadbc,false)?;
    // VMA 0xaad90
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xaad94
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // VMA 0xaad98
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaad9c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaada0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,4096)?)?;
    // VMA 0xaada4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaada8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xaadac
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(8)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaadb0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xaadb4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-60)?)?;
    // VMA 0xaadb8
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::new(4)?,168)?)?;
    // VMA 0xaadbc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xaadc0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xaadc4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,16)?)?;
    // VMA 0xaadc8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // VMA 0xaadcc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(8)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaadd0
    c.b(C::Ne,0xaad20,false)?;
    // VMA 0xaadd4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-64)?)?;
    // VMA 0xaadd8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xaaddc
    c.emit(arm32::mov_register(C::Al,R::R12,R::LR))?;
    // VMA 0xaade0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(10)?))?;
    // VMA 0xaade4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaade8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R12,-8)?)?;
    // VMA 0xaadec
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-52)?)?;
    // VMA 0xaadf0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xaadf4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaadf8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,64)?)?;
    // VMA 0xaadfc
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae00
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(6)?,16)?)?;
    // VMA 0xaae04
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::new(6)?,255)?)?;
    // VMA 0xaae08
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,60)?)?;
    // VMA 0xaae0c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::LR,R::new(6)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R2,16)?)?;
    // VMA 0xaae14
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,0)?)?;
    // VMA 0xaae18
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,0)?)?;
    // VMA 0xaae1c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xaae20
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // VMA 0xaae24
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xaae28
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,16)?)?;
    // VMA 0xaae2c
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // VMA 0xaae30
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // VMA 0xaae34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xaae38
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,68)?)?;
    // VMA 0xaae3c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,56)?)?;
    // VMA 0xaae40
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Lsl,R::new(6)?,false)?)?;
    // VMA 0xaae44
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae48
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-52)?)?;
    // VMA 0xaae4c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,152)?)?;
    // VMA 0xaae50
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,0)?)?;
    // VMA 0xaae54
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaae58
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,48)?)?;
    // VMA 0xaae5c
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae60
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xaae64
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,160)?)?;
    // VMA 0xaae68
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaae6c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,52)?)?;
    // VMA 0xaae70
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae74
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,152)?)?;
    // VMA 0xaae78
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,160)?)?;
    // VMA 0xaae7c
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,208)?)?;
    // VMA 0xaae80
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,216)?)?;
    // VMA 0xaae84
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(7)?,0)?)?;
    // VMA 0xaae88
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xaae8c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xaae90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xaae94
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,12)?)?;
    // VMA 0xaae98
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xaae9c
    c.literal(R::R1,0xab044)?;
    // VMA 0xaaea0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xaaea4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xaaea8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaeac
    c.literal(R::R1,0xab034)?;
    // VMA 0xaaeb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xaaeb4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaeb8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xaaebc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // VMA 0xaaec0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xaaec4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // VMA 0xaaec8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,68)?)?;
    // VMA 0xaaecc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,65536)?)?;
    // VMA 0xaaed0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // VMA 0xaaed4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xaaed8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-64)?)?;
    // VMA 0xaaedc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,65536)?)?;
    // VMA 0xaaee0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-64)?)?;
    // VMA 0xaaee4
    c.b(C::Ne,0xaacfc,false)?;
    // VMA 0xaaee8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,56)?)?;
    // VMA 0xaaeec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xaaef0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xaaef4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaef8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,114)?)?;
    // VMA 0xaaefc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaaf00
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf04
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xaaf08
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaaf0c
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf10
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,64)?)?;
    // VMA 0xaaf14
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf18
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,16)?)?;
    // VMA 0xaaf1c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,60)?)?;
    // VMA 0xaaf20
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xaaf24
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf28
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,32)?)?;
    // VMA 0xaaf2c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaaf30
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,240)?)?;
    // VMA 0xaaf34
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(7)?,100)?)?;
    // VMA 0xaaf38
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaaf3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xaaf40
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf44
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,148)?)?;
    // VMA 0xaaf48
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,240)?)?;
    // VMA 0xaaf4c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,4)?)?;
    // VMA 0xaaf50
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaaf54
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaaf58
    c.b(C::Cc,0xaaf8c,false)?;
    // VMA 0xaaf5c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaaf60
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,16)?)?;
    // VMA 0xaaf64
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xaaf68
    c.literal(R::R1,0xab048)?;
    // VMA 0xaaf6c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xaaf70
    c.literal(R::R2,0xab040)?;
    // VMA 0xaaf74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xaaf78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1012)?)?;
    // VMA 0xaaf7c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaf80
    c.literal(R::R1,0xab034)?;
    // VMA 0xaaf84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xaaf88
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaf8c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,36)?)?;
    // VMA 0xaaf90
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xaaf94
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // VMA 0xaaf98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,672)?)?;
    // VMA 0xaaf9c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,1244)?)?;
    // VMA 0xaafa0
    c.b(C::Al,0xdff3c,true)?;
    // VMA 0xaafa4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,604)?)?;
    // VMA 0xaafa8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaafac
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,20)?)?;
    // VMA 0xaafb0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaafb4
    c.b(C::Eq,0xab004,false)?;
    // VMA 0xaafb8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,1246)?)?;
    // VMA 0xaafbc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaafc0
    c.b(C::Ne,0xab004,false)?;
    // VMA 0xaafc4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,3424)?)?;
    // VMA 0xaafc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // VMA 0xaafcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xaafd0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xaafd4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // VMA 0xaafd8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaafdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // VMA 0xaafe0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,324)?)?;
    // VMA 0xaafe4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,100)?)?;
    // VMA 0xaafe8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,3)?)?;
    // VMA 0xaafec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xaaff0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,32)?)?;
    // VMA 0xaaff4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xaaff8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xaaffc
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,64)?)?;
    // VMA 0xab000
    c.b(C::Al,0x31250,true)?;
    // VMA 0xab004
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,864)?)?;
    // VMA 0xab008
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,8)?)?;
    // VMA 0xab00c
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xab010
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,148)?)?;
    // VMA 0xab014
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,4)?)?;
    // VMA 0xab018
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // VMA 0xab01c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // VMA 0xab020
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xab024
    c.b(C::Al,0x31dc8,true)?;
    // VMA 0xab028
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xab02c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xab030)
}

const CODEC_CHANNEL_LITERALS:[u32;7]=[
    0x000f_83fb, // controller format diagnostics
    0x000e_2788, // common log prefix
    0x0010_dd7c, // callback/global record
    0x000f_842b, // linked configuration strings
    0x000f_8479,
    0x000f_6700,
    0x000f_662e,
];

/// Initialize the full codec controller's command FIFO, writes per-frame
/// register words across two DMA instances, and dispatches encoded packets.
/// The function has a natural POP {...,PC} return at 0xab764, followed by a
/// seven-word diagnostic/global-address pool, not executable instructions.
fn configure_codec_register_fifo(c:&mut Code)->Result<(),String>{
    c.check(0xab04c)?;
    // VMA 0xab04c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xab050
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xab054
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,60)?)?;
    // VMA 0xab058
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xab05c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,1160)?)?;
    // VMA 0xab060
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // VMA 0xab064
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(3)?))?;
    // VMA 0xab068
    c.literal(R::R1,0xab768)?;
    // VMA 0xab06c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xab070
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // VMA 0xab074
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,4)?)?;
    // VMA 0xab078
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // VMA 0xab07c
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // VMA 0xab080
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xab084
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab088
    c.literal(R::R1,0xab76c)?;
    // VMA 0xab08c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xab090
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab094
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,860)?)?;
    // VMA 0xab098
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xab09c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xab0a0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xab0a4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R12,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab0a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,81)?)?;
    // VMA 0xab0ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xab0b0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // VMA 0xab0b4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab0b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // VMA 0xab0bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,856)?)?;
    // VMA 0xab0c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xab0c4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab0c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // VMA 0xab0cc
    c.emit(arm32::mov_register(C::Al,R::R2,R::LR))?;
    // VMA 0xab0d0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab0d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,984)?)?;
    // VMA 0xab0d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xab0dc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-40)?)?;
    // VMA 0xab0e0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(10)?,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab0e4
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,44)?)?;
    // VMA 0xab0e8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab0ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,980)?)?;
    // VMA 0xab0f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xab0f4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab0f8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // VMA 0xab0fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // VMA 0xab100
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab104
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab108
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // VMA 0xab10c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,864)?)?;
    // VMA 0xab110
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // VMA 0xab114
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,4)?)?;
    // VMA 0xab118
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,292)?)?;
    // VMA 0xab11c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,4)?)?;
    // VMA 0xab120
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,144)?)?;
    // VMA 0xab124
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xab128
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab12c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // VMA 0xab130
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // VMA 0xab134
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::LR))?;
    // VMA 0xab138
    c.b(C::Al,0x31dc8,true)?;
    // VMA 0xab13c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,244)?)?;
    // VMA 0xab140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3072)?)?;
    // VMA 0xab144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xab148
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab14c
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xab154
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,248)?)?;
    // VMA 0xab158
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab15c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab160
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab164
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3696)?)?;
    // VMA 0xab168
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,260)?)?;
    // VMA 0xab16c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab170
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab174
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,628)?)?;
    // VMA 0xab178
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,264)?)?;
    // VMA 0xab17c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab180
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab184
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,632)?)?;
    // VMA 0xab18c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,268)?)?;
    // VMA 0xab190
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab194
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab198
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab19c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,636)?)?;
    // VMA 0xab1a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,272)?)?;
    // VMA 0xab1a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab1a8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab1ac
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab1b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xab1b4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,252)?)?;
    // VMA 0xab1b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab1bc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xab1c0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab1c4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xab1c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // VMA 0xab1cc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // VMA 0xab1d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // VMA 0xab1d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,1036)?)?;
    // VMA 0xab1d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xab1dc
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // VMA 0xab1e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // VMA 0xab1e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,36)?)?;
    // VMA 0xab1e8
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab1ec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xab1f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xab1f4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xab1f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xab1fc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,1036)?)?;
    // VMA 0xab200
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xab204
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xab208
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // VMA 0xab20c
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(7)?,Shift::Lsl,15)?)?;
    // VMA 0xab210
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,296)?)?;
    // VMA 0xab214
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab218
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xab21c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1040)?)?;
    // VMA 0xab220
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // VMA 0xab224
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xab228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33)?)?;
    // VMA 0xab22c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // VMA 0xab230
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // VMA 0xab234
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(5)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab238
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xab23c
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R0,4)?)?;
    // VMA 0xab240
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xab244
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(8)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab248
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(7)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab24c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,300)?)?;
    // VMA 0xab250
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,255)?)?;
    // VMA 0xab254
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(6)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab258
    c.b(C::Ne,0xab234,false)?;
    // VMA 0xab25c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,136)?)?;
    // VMA 0xab260
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab264
    c.b(C::Eq,0xab278,false)?;
    // VMA 0xab268
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,140)?)?;
    // VMA 0xab26c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab270
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xab274
    c.b(C::Al,0xab27c,false)?;
    // VMA 0xab278
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab27c
    c.literal(R::R2,0xab770)?;
    // VMA 0xab280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,69)?)?;
    // VMA 0xab284
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xab288
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xab28c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xab290
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,81)?)?;
    // VMA 0xab294
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xab298
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    // VMA 0xab29c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab2a0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,140)?)?;
    // VMA 0xab2a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,28)?)?;
    // VMA 0xab2a8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(8)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab2ac
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xab2b0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // VMA 0xab2b4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab2b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,304)?)?;
    // VMA 0xab2bc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab2c0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab2c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,49)?)?;
    // VMA 0xab2c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xab2cc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xab2d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,136)?)?;
    // VMA 0xab2d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,0)?)?;
    // VMA 0xab2d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xab2dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,304)?)?;
    // VMA 0xab2e0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab2e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xab2e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65)?)?;
    // VMA 0xab2ec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xab2f0
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xab2f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1048)?)?;
    // VMA 0xab2f8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // VMA 0xab2fc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xab300
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1052)?)?;
    // VMA 0xab304
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab308
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xab30c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,256)?)?;
    // VMA 0xab310
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab314
    c.b(C::Eq,0xab4e8,false)?;
    // VMA 0xab318
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,324)?)?;
    // VMA 0xab31c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xab320
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // VMA 0xab324
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,15,false)?)?;
    // VMA 0xab328
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(3)?,3)?)?;
    // VMA 0xab32c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // VMA 0xab330
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // VMA 0xab334
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,64)?)?;
    // VMA 0xab338
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xab33c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // VMA 0xab340
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // VMA 0xab344
    c.b(C::Cc,0xab380,false)?;
    // VMA 0xab348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,768)?)?;
    // VMA 0xab34c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(10)?,2)?)?;
    // VMA 0xab350
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xab354
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(6)?,R::new(10)?,R::new(6)?)?)?;
    // VMA 0xab358
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,768)?)?;
    // VMA 0xab35c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab360
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(7)?,0)?)?;
    // VMA 0xab364
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab368
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab36c
    c.b(C::Cc,0xab488,false)?;
    // VMA 0xab370
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xab374
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // VMA 0xab378
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xab37c
    c.b(C::Cc,0xab35c,false)?;
    // VMA 0xab380
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xab384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xab388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xab38c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3104)?)?;
    // VMA 0xab390
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xab394
    c.literal(R::R2,0xab770)?;
    // VMA 0xab398
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,256)?)?;
    // VMA 0xab39c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R1,R::R0,Shift::Lsr,15,false)?)?;
    // VMA 0xab3a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,101)?)?;
    // VMA 0xab3a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab3a8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,160)?)?;
    // VMA 0xab3ac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xab3b0
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xab3b4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xab3b8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,160)?)?;
    // VMA 0xab3bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,160)?)?;
    // VMA 0xab3c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,160)?)?;
    // VMA 0xab3c4
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xab3c8
    c.emit(arm32::mov_register(C::Cc,R::new(5)?,R::new(4)?))?;
    // VMA 0xab3cc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xab3d0
    c.b(C::Eq,0xab3f8,false)?;
    // VMA 0xab3d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xab3d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,768)?)?;
    // VMA 0xab3dc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,36)?)?;
    // VMA 0xab3e0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab3e4
    c.emit(arm32::load_halfword_postindexed(C::Al,R::R2,R::R1,2)?)?;
    // VMA 0xab3e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xab3ec
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab3f0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xab3f4
    c.b(C::Cc,0xab3e4,false)?;
    // VMA 0xab3f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,256)?)?;
    // VMA 0xab3fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xab400
    c.literal(R::R2,0xab770)?;
    // VMA 0xab404
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16128)?)?;
    // VMA 0xab408
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // VMA 0xab40c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,111)?)?;
    // VMA 0xab410
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab414
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab418
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xab41c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,160)?)?;
    // VMA 0xab420
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xab424
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xab428
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xab42c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,160)?)?;
    // VMA 0xab430
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xab434
    c.emit(arm32::mov_register(C::Cc,R::new(8)?,R::new(6)?))?;
    // VMA 0xab438
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab43c
    c.b(C::Cs,0xab4e8,false)?;
    // VMA 0xab440
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // VMA 0xab444
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xab448
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // VMA 0xab44c
    c.emit(arm32::data_register(C::Al,Op::Mvn,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab450
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,40)?)?;
    // VMA 0xab454
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,161)?)?;
    // VMA 0xab458
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab45c
    c.emit(arm32::data_immediate(C::Ls,Op::Mvn,R::R1,R::R0,160)?)?;
    // VMA 0xab460
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R2,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xab464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xab468
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(10)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab46c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,756)?)?;
    // VMA 0xab470
    c.emit(arm32::load_halfword_postindexed(C::Al,R::new(3)?,R::R1,2)?)?;
    // VMA 0xab474
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xab478
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab47c
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xab480
    c.b(C::Cc,0xab470,false)?;
    // VMA 0xab484
    c.b(C::Al,0xab4e8,false)?;
    // VMA 0xab488
    c.literal(R::R1,0xab774)?;
    // VMA 0xab48c
    c.literal(R::R2,0xab778)?;
    // VMA 0xab490
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,90)?)?;
    // VMA 0xab494
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xab498
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xab49c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab4a0
    c.literal(R::R1,0xab76c)?;
    // VMA 0xab4a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xab4a8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab4ac
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // VMA 0xab4b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xab4b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xab4b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xab4bc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,32768)?)?;
    // VMA 0xab4c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,3104)?)?;
    // VMA 0xab4c4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xab4c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,36)?)?;
    // VMA 0xab4cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xab4d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab4d4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xab4d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,40)?)?;
    // VMA 0xab4dc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab4e0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xab4e4
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xab4e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // VMA 0xab4ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,96)?)?;
    // VMA 0xab4f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,3504)?)?;
    // VMA 0xab4f4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab4f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,97)?)?;
    // VMA 0xab4fc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xab500
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xab504
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab508
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,32)?)?;
    // VMA 0xab50c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xab510
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab514
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // VMA 0xab518
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab51c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,36)?)?;
    // VMA 0xab520
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xab524
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab528
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab52c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,668)?)?;
    // VMA 0xab530
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,276)?)?;
    // VMA 0xab534
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xab538
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xab53c
    c.literal(R::R0,0xab77c)?;
    // VMA 0xab540
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,8)?)?;
    // VMA 0xab544
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xab548
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,2880)?)?;
    // VMA 0xab54c
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xab550
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,3440)?)?;
    // VMA 0xab554
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xab558
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xab55c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3758096384)?)?;
    // VMA 0xab560
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xab564
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,304)?)?;
    // VMA 0xab568
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(7)?,R::new(3)?,4)?)?;
    // VMA 0xab56c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xab570
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // VMA 0xab574
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(7)?,R::R1,4)?)?;
    // VMA 0xab578
    c.b(C::Ne,0xab568,false)?;
    // VMA 0xab57c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,168)?)?;
    // VMA 0xab580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xab584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab588
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,36)?)?;
    // VMA 0xab58c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xab590
    c.emit(arm32::load_signed_byte(C::Al,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xab594
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // VMA 0xab598
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xab59c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // VMA 0xab5a0
    c.b(C::Ne,0xab58c,false)?;
    // VMA 0xab5a4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xab5a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xab5ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,97)?)?;
    // VMA 0xab5b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xab5b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,664)?)?;
    // VMA 0xab5b8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // VMA 0xab5bc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab5c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,120)?)?;
    // VMA 0xab5c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,1024)?)?;
    // VMA 0xab5c8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,360)?)?;
    // VMA 0xab5cc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R0,14336)?)?;
    // VMA 0xab5d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,364)?)?;
    // VMA 0xab5d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // VMA 0xab5d8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::R0,14336)?)?;
    // VMA 0xab5dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xab5e0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // VMA 0xab5e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xab5e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1152)?)?;
    // VMA 0xab5ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // VMA 0xab5f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xab5f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xab5f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xab5fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // VMA 0xab600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xab604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xab608
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xab60c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-36)?)?;
    // VMA 0xab610
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab614
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xab618
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(10)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab61c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,160)?)?;
    // VMA 0xab620
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xab624
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // VMA 0xab628
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xab62c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-32)?)?;
    // VMA 0xab630
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xab634
    c.emit(arm32::data_register(C::Eq,Op::Add,R::new(4)?,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab638
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(3)?,R::new(10)?,152)?)?;
    // VMA 0xab63c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // VMA 0xab640
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,0)?)?;
    // VMA 0xab644
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xab648
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab64c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,16)?)?;
    // VMA 0xab650
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,0)?)?;
    // VMA 0xab654
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::LR,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab658
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab65c
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R12,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab660
    c.b(C::Ne,0xab620,false)?;
    // VMA 0xab664
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // VMA 0xab668
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,32)?)?;
    // VMA 0xab66c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-36)?)?;
    // VMA 0xab670
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xab674
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::R12,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab678
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,-8)?)?;
    // VMA 0xab67c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // VMA 0xab680
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    // VMA 0xab684
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab688
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // VMA 0xab68c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::R2,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xab690
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::LR))?;
    // VMA 0xab694
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // VMA 0xab698
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::R1,255)?)?;
    // VMA 0xab69c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xab6a0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::LR,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xab6a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,16)?)?;
    // VMA 0xab6a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // VMA 0xab6ac
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(8)?,0)?)?;
    // VMA 0xab6b0
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R12,0)?)?;
    // VMA 0xab6b4
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xab6b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xab6bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R12,16)?)?;
    // VMA 0xab6c0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xab6c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xab6c8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xab6cc
    c.b(C::Al,0xd9e60,true)?;
    // VMA 0xab6d0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xab6d4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // VMA 0xab6d8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xab6dc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // VMA 0xab6e0
    c.b(C::Al,0xd9e60,true)?;
    // VMA 0xab6e4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // VMA 0xab6e8
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // VMA 0xab6ec
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xab6f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,-8)?)?;
    // VMA 0xab6f4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // VMA 0xab6f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,160)?)?;
    // VMA 0xab6fc
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,152)?)?;
    // VMA 0xab700
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xab704
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xab708
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xab70c
    c.literal(R::R1,0xab780)?;
    // VMA 0xab710
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xab714
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xab718
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xab71c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab720
    c.literal(R::R1,0xab76c)?;
    // VMA 0xab724
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xab728
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xab72c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xab730
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(6)?))?;
    // VMA 0xab734
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,12)?)?;
    // VMA 0xab738
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xab73c
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(5)?))?;
    // VMA 0xab740
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xab744
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,4)?)?;
    // VMA 0xab748
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,32)?)?;
    // VMA 0xab74c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,2)?)?;
    // VMA 0xab750
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,65536)?)?;
    // VMA 0xab754
    c.b(C::Ne,0xab5f8,false)?;
    // VMA 0xab758
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xab75c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,292)?)?;
    // VMA 0xab760
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xab764
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xab768)
}

const CODEC_FIFO_LITERALS:[u32;7]=[
    0x000f_8487, // codec trace message
    0x000e_2788, // log format
    0x000f_8334, // controller command diagnostics
    0x000f_842b,
    0x000f_84b7,
    0x0010_dd64, // shared codec state global
    0x000f_65d4, // firmware callback name
];

/// Normalize up to sixteen runtime controller codec buffer pointer slots
/// after an MMIO rebind. Includes two separately classified global addresses.
fn remap_codec_channel_slots(c:&mut Code)->Result<(),String>{
    c.check(0xab784)?;
    // VMA 0xab784
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xab788
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xab78c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xab790
    c.literal(R::new(9)?,0xab8e4)?;
    // VMA 0xab794
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xab798
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xab79c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // VMA 0xab7a0
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,8)?)?;
    // VMA 0xab7a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(7)?,2880)?)?;
    // VMA 0xab7a8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xab7ac
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,0)?)?;
    // VMA 0xab7b0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xab7b4
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(3)?)?)?;
    // VMA 0xab7b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // VMA 0xab7bc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // VMA 0xab7c0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab7c4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,608)?)?;
    // VMA 0xab7c8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,64)?)?;
    // VMA 0xab7cc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab7d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,616)?)?;
    // VMA 0xab7d4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,624)?)?;
    // VMA 0xab7d8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,72)?)?;
    // VMA 0xab7dc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab7e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,632)?)?;
    // VMA 0xab7e4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab7e8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // VMA 0xab7ec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,80)?)?;
    // VMA 0xab7f0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xab7f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,620)?)?;
    // VMA 0xab7f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,628)?)?;
    // VMA 0xab7fc
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab800
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,636)?)?;
    // VMA 0xab804
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,604)?)?;
    // VMA 0xab808
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab80c
    c.b(C::Eq,0xab870,false)?;
    // VMA 0xab810
    c.literal(R::R0,0xab8e8)?;
    // VMA 0xab814
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,656)?)?;
    // VMA 0xab818
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xab81c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xab820
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xab824
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(3)?)?)?;
    // VMA 0xab828
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,656)?)?;
    // VMA 0xab82c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,60)?)?;
    // VMA 0xab830
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab834
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,640)?)?;
    // VMA 0xab838
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,68)?)?;
    // VMA 0xab83c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab840
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,648)?)?;
    // VMA 0xab844
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,76)?)?;
    // VMA 0xab848
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab84c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,664)?)?;
    // VMA 0xab850
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab854
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,644)?)?;
    // VMA 0xab858
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,84)?)?;
    // VMA 0xab85c
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xab860
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,652)?)?;
    // VMA 0xab864
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab868
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,660)?)?;
    // VMA 0xab86c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,668)?)?;
    // VMA 0xab870
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,352)?)?;
    // VMA 0xab874
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xab878
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xab87c
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R2,128)?)?;
    // VMA 0xab880
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xab884
    c.b(C::Eq,0xab8a0,false)?;
    // VMA 0xab888
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,0)?)?;
    // VMA 0xab88c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xab890
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(3)?)?)?;
    // VMA 0xab894
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,128)?)?;
    // VMA 0xab898
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab89c
    c.b(C::Ne,0xab8a8,false)?;
    // VMA 0xab8a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xab8a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,128)?)?;
    // VMA 0xab8a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,56)?)?;
    // VMA 0xab8ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xab8b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,16)?)?;
    // VMA 0xab8b4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab8b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xab8bc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,64)?)?;
    // VMA 0xab8c0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab8c4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,64)?)?;
    // VMA 0xab8c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,72)?)?;
    // VMA 0xab8cc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab8d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,192)?)?;
    // VMA 0xab8d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,4)?)?;
    // VMA 0xab8d8
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xab8dc
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xab8e0
    c.b(C::Al,0xab878,false)?;
    c.check(0xab8e4)
}

const SLOT_REMAP_GLOBALS:[u32;2]=[0x0010_dd68,0x0010_dd6c];

/// Bind a codec slot by tag and store the resolved controller pointer.
fn link_codec_slot_runtime(c:&mut Code)->Result<(),String>{
    c.check(0xab8ec)?;
    // VMA 0xab8ec
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xab8f0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xab8f4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xab8f8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xab8fc
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xab900
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // VMA 0xab904
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xab908
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,2880)?)?;
    // VMA 0xab90c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xab910
    c.b(C::Al,0xa9138,true)?;
    // VMA 0xab914
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xab918
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // VMA 0xab91c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,140)?)?;
    // VMA 0xab920
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::new(5)?,136)?)?;
    // VMA 0xab924
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xab928
    c.b(C::Eq,0xab958,false)?;
    // VMA 0xab92c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xab930
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xab934
    c.b(C::Al,0xa92c8,true)?;
    // VMA 0xab938
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,136)?)?;
    // VMA 0xab93c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xab940
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xab944
    c.b(C::Al,0xa9138,true)?;
    // VMA 0xab948
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,136)?)?;
    // VMA 0xab94c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab950
    c.b(C::Ne,0xab95c,false)?;
    // VMA 0xab954
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xab958
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xab95c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xab960
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xab964)
}

/// Publish per-session display metadata and process mode-specific parameter
/// defaults. This helper terminates at its original POP {...,PC}.
fn publish_codec_parameter_modes(c:&mut Code)->Result<(),String>{
    c.check(0xab964)?;
    // VMA 0xab964
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xab968
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xab96c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2448)?)?;
    // VMA 0xab970
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2816)?)?;
    // VMA 0xab974
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xab978
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,19)?)?;
    // VMA 0xab97c
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2821)?)?;
    // VMA 0xab980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,616)?)?;
    // VMA 0xab984
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,2444)?)?;
    // VMA 0xab988
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::R12,R::R2)?)?;
    // VMA 0xab98c
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::R0,R::new(3)?)?)?;
    // VMA 0xab990
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,106)?)?;
    // VMA 0xab994
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xab998
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // VMA 0xab99c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xab9a0
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::R12,R::R2)?)?;
    // VMA 0xab9a4
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::R0,R::new(3)?)?)?;
    // VMA 0xab9a8
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R12,167)?)?;
    // VMA 0xab9ac
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2820)?)?;
    // VMA 0xab9b0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2592)?)?;
    // VMA 0xab9b4
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2822)?)?;
    // VMA 0xab9b8
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::R0,2594)?)?;
    // VMA 0xab9bc
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::R0,2823)?)?;
    // VMA 0xab9c0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2593)?)?;
    // VMA 0xab9c4
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2824)?)?;
    // VMA 0xab9c8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,2596)?)?;
    // VMA 0xab9cc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,2825)?)?;
    // VMA 0xab9d0
    c.b(C::Eq,0xaba04,false)?;
    // VMA 0xab9d4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::LR,Shift::Lsl,0,true)?)?;
    // VMA 0xab9d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2599)?)?;
    // VMA 0xab9dc
    c.b(C::Ne,0xaba00,false)?;
    // VMA 0xab9e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xab9e4
    c.b(C::Eq,0xab9fc,false)?;
    // VMA 0xab9e8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2488)?)?;
    // VMA 0xab9ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xab9f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R2,2147483649)?)?;
    // VMA 0xab9f4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xab9f8
    c.b(C::Al,0xaba00,false)?;
    // VMA 0xab9fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xaba00
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2838)?)?;
    // VMA 0xaba04
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2600)?)?;
    // VMA 0xaba08
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,131)?)?;
    // VMA 0xaba0c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,572)?)?;
    // VMA 0xaba10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,2560)?)?;
    // VMA 0xaba14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xaba18
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2826)?)?;
    // VMA 0xaba1c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2608)?)?;
    // VMA 0xaba20
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2827)?)?;
    // VMA 0xaba24
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2609)?)?;
    // VMA 0xaba28
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2828)?)?;
    // VMA 0xaba2c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2612)?)?;
    // VMA 0xaba30
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2832)?)?;
    // VMA 0xaba34
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2616)?)?;
    // VMA 0xaba38
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2836)?)?;
    // VMA 0xaba3c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2686)?)?;
    // VMA 0xaba40
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2839)?)?;
    // VMA 0xaba44
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2687)?)?;
    // VMA 0xaba48
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2840)?)?;
    // VMA 0xaba4c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2688)?)?;
    // VMA 0xaba50
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2841)?)?;
    // VMA 0xaba54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,129)?)?;
    // VMA 0xaba58
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2560)?)?;
    // VMA 0xaba5c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2844)?)?;
    // VMA 0xaba60
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,2848)?)?;
    // VMA 0xaba64
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2852)?)?;
    // VMA 0xaba68
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R12,131)?)?;
    // VMA 0xaba6c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2860)?)?;
    // VMA 0xaba70
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,160)?)?;
    // VMA 0xaba74
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2864)?)?;
    // VMA 0xaba78
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,2868)?)?;
    // VMA 0xaba7c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xaba80)
}

/// Emit one codec notification from the selected channel buffer, including
/// mode flags and command metadata. The real function ends in a tail branch
/// to the logging routine; five literal words follow at 0xabbe4.
fn dispatch_controller_codec_events(c:&mut Code)->Result<(),String>{
    c.check(0xaba80)?;
    // VMA 0xaba80
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xaba84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xaba88
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xaba8c
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xaba90
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xaba94
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xaba98
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // VMA 0xaba9c
    c.literal(R::new(7)?,0xabbe4)?;
    // VMA 0xabaa0
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // VMA 0xabaa4
    c.b(C::Eq,0xabb44,false)?;
    // VMA 0xabaa8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabaac
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xabab0
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xabab4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // VMA 0xabab8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // VMA 0xababc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,8)?)?;
    // VMA 0xabac0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,8)?)?;
    // VMA 0xabac4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,4)?)?;
    // VMA 0xabac8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,12)?)?;
    // VMA 0xabacc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,40)?)?;
    // VMA 0xabad0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,24)?)?;
    // VMA 0xabad4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // VMA 0xabad8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,44)?)?;
    // VMA 0xabadc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xabae0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // VMA 0xabae4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1096)?)?;
    // VMA 0xabae8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1000)?)?;
    // VMA 0xabaec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xabaf0
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xabaf4
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xabaf8
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabafc
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabb00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1096)?)?;
    // VMA 0xabb04
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2400)?)?;
    // VMA 0xabb08
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,72)?)?;
    // VMA 0xabb0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xabb10
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,76)?)?;
    // VMA 0xabb14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // VMA 0xabb18
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,40)?)?;
    // VMA 0xabb1c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,5,false)?)?;
    // VMA 0xabb20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // VMA 0xabb24
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,720)?)?;
    // VMA 0xabb28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,864)?)?;
    // VMA 0xabb2c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xabb30
    c.b(C::Eq,0xabba0,false)?;
    // VMA 0xabb34
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xabb38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xabb3c
    c.b(C::Al,0x2a18c,true)?;
    // VMA 0xabb40
    c.b(C::Al,0xabba8,false)?;
    // VMA 0xabb44
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,720)?)?;
    // VMA 0xabb48
    c.literal(R::R2,0xabbe8)?;
    // VMA 0xabb4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,217)?)?;
    // VMA 0xabb50
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xabb54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // VMA 0xabb58
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xabb5c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xabb60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,6)?)?;
    // VMA 0xabb64
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xabb68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,8)?)?;
    // VMA 0xabb6c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // VMA 0xabb70
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,40)?)?;
    // VMA 0xabb74
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // VMA 0xabb78
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,864)?)?;
    // VMA 0xabb7c
    c.b(C::Al,0x2a878,true)?;
    // VMA 0xabb80
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,8)?)?;
    // VMA 0xabb84
    c.b(C::Al,0x26754,true)?;
    // VMA 0xabb88
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xabb8c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // VMA 0xabb90
    c.literal(R::R1,0xabbec)?;
    // VMA 0xabb94
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xabb98
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xabb9c
    c.b(C::Al,0xabbc8,false)?;
    // VMA 0xabba0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xabba4
    c.b(C::Al,0x2a878,true)?;
    // VMA 0xabba8
    c.b(C::Al,0x26754,true)?;
    // VMA 0xabbac
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,24)?)?;
    // VMA 0xabbb0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xabbb4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // VMA 0xabbb8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,28)?)?;
    // VMA 0xabbbc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xabbc0
    c.literal(R::R1,0xabbf4)?;
    // VMA 0xabbc4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xabbc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xabbcc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xabbd0
    c.literal(R::R1,0xabbf0)?;
    // VMA 0xabbd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xabbd8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xabbdc
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xabbe0
    c.b(C::Al,0x2ec94,false)?;
    c.check(0xabbe4)
}

const CODEC_RELAY_DIAGNOSTICS:[u32;5]=[
    0x0022_1007, // codec status command tag
    0x000f_8519, // diagnostic string
    0x000f_681d,
    0x000e_2788, // common logger context
    0x000f_683c,
];

/// Write mode-specific codec control registers, initialize per-plane
/// structures and route controller error conditions through designated
/// platform callbacks. Includes a separate nine-word literal pool.
fn program_codec_register_packets(c:&mut Code)->Result<(),String>{
    c.check(0xabbf8)?;
    // VMA 0xabbf8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xabbfc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xabc00
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // VMA 0xabc04
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // VMA 0xabc08
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,12)?)?;
    // VMA 0xabc0c
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // VMA 0xabc10
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,8)?)?;
    // VMA 0xabc14
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // VMA 0xabc18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xabc1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // VMA 0xabc20
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // VMA 0xabc24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,984)?)?;
    // VMA 0xabc28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xabc2c
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::new(10)?,988)?)?;
    // VMA 0xabc30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xabc34
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,0)?)?;
    // VMA 0xabc38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // VMA 0xabc3c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xabc40
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xabc44
    c.literal(R::R0,0xac72c)?;
    // VMA 0xabc48
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xabc4c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // VMA 0xabc50
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // VMA 0xabc54
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xabc58
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,12)?)?;
    // VMA 0xabc5c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,76)?)?;
    // VMA 0xabc60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xabc64
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,20)?)?;
    // VMA 0xabc68
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,48)?)?;
    // VMA 0xabc6c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,52)?)?;
    // VMA 0xabc70
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,56)?)?;
    // VMA 0xabc74
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xabc78
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,24)?)?;
    // VMA 0xabc7c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,28)?)?;
    // VMA 0xabc80
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,32)?)?;
    // VMA 0xabc84
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,36)?)?;
    // VMA 0xabc88
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,40)?)?;
    // VMA 0xabc8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xabc90
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xabc94
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,92)?)?;
    // VMA 0xabc98
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xabc9c
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,96)?)?;
    // VMA 0xabca0
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xabca4
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,48)?)?;
    // VMA 0xabca8
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xabcac
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,84)?)?;
    // VMA 0xabcb0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // VMA 0xabcb4
    c.b(C::Eq,0xac200,false)?;
    // VMA 0xabcb8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,28)?)?;
    // VMA 0xabcbc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,36)?)?;
    // VMA 0xabcc0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // VMA 0xabcc4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // VMA 0xabcc8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,48)?)?;
    // VMA 0xabccc
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // VMA 0xabcd0
    c.b(C::Ne,0xac584,false)?;
    // VMA 0xabcd4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1900)?)?;
    // VMA 0xabcd8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabcdc
    c.b(C::Ne,0xac5f0,false)?;
    // VMA 0xabce0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,48)?)?;
    // VMA 0xabce4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    // VMA 0xabce8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xabcec
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,32)?)?;
    // VMA 0xabcf0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,36)?)?;
    // VMA 0xabcf4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // VMA 0xabcf8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,20)?)?;
    // VMA 0xabcfc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // VMA 0xabd00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,24)?)?;
    // VMA 0xabd04
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // VMA 0xabd08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,44)?)?;
    // VMA 0xabd0c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,88)?)?;
    // VMA 0xabd10
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // VMA 0xabd14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabd18
    c.b(C::Eq,0xac200,false)?;
    // VMA 0xabd1c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1900)?)?;
    // VMA 0xabd20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabd24
    c.b(C::Ne,0xac200,false)?;
    // VMA 0xabd28
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,12)?)?;
    // VMA 0xabd2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,24)?)?;
    // VMA 0xabd30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1888)?)?;
    // VMA 0xabd34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xabd38
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(7)?,R::new(9)?,R::R0)?)?;
    // VMA 0xabd3c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabd40
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xabd44
    c.b(C::Ne,0xac640,false)?;
    // VMA 0xabd48
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsl,1)?)?;
    // VMA 0xabd4c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // VMA 0xabd50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,868)?)?;
    // VMA 0xabd54
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xabd58
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xabd5c
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsl,1)?)?;
    // VMA 0xabd60
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,72)?)?;
    // VMA 0xabd64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,98)?)?;
    // VMA 0xabd68
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xabd6c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,4)?)?;
    // VMA 0xabd70
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xabd74
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xabd78
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R2,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xabd7c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,60)?)?;
    // VMA 0xabd80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,102)?)?;
    // VMA 0xabd84
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xabd88
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,8)?)?;
    // VMA 0xabd8c
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xabd90
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xabd94
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(10)?,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xabd98
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,64)?)?;
    // VMA 0xabd9c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // VMA 0xabda0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,576)?)?;
    // VMA 0xabda4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabda8
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,96)?)?;
    // VMA 0xabdac
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xabdb0
    c.b(C::Eq,0xac510,false)?;
    // VMA 0xabdb4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(6)?,64)?)?;
    // VMA 0xabdb8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,16)?)?;
    // VMA 0xabdbc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,32)?)?;
    // VMA 0xabdc0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1876)?)?;
    // VMA 0xabdc4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xabdc8
    c.literal(R::R1,0xac730)?;
    // VMA 0xabdcc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // VMA 0xabdd0
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xabdd4
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,4)?)?;
    // VMA 0xabdd8
    c.b(C::Eq,0xabe2c,false)?;
    // VMA 0xabddc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,92)?)?;
    // VMA 0xabde0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xabde4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabde8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // VMA 0xabdec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xabdf0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xabdf4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xabdf8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xabdfc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xabe00
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xabe04
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabe08
    c.literal(R::R2,0xac734)?;
    // VMA 0xabe0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xabe10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,272)?)?;
    // VMA 0xabe14
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xabe18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,20)?)?;
    // VMA 0xabe1c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,64)?)?;
    // VMA 0xabe20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xabe24
    c.b(C::Al,0x471c,true)?;
    // VMA 0xabe28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,92)?)?;
    // VMA 0xabe2c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xabe30
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,524288)?)?;
    // VMA 0xabe34
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xabe38
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,64)?)?;
    // VMA 0xabe3c
    c.b(C::Eq,0xabec4,false)?;
    // VMA 0xabe40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,60)?)?;
    // VMA 0xabe44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xabe48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabe4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,21)?)?;
    // VMA 0xabe50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // VMA 0xabe54
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xabe58
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xabe5c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xabe60
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xabe64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xabe68
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabe6c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,148)?)?;
    // VMA 0xabe70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,20)?)?;
    // VMA 0xabe74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xabe78
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,-12)?)?;
    // VMA 0xabe7c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xabe80
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // VMA 0xabe84
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // VMA 0xabe88
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xabe8c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // VMA 0xabe90
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,12)?)?;
    // VMA 0xabe94
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // VMA 0xabe98
    c.b(C::Ne,0xabe78,false)?;
    // VMA 0xabe9c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,160)?)?;
    // VMA 0xabea0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,44)?)?;
    // VMA 0xabea4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,164)?)?;
    // VMA 0xabea8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,48)?)?;
    // VMA 0xabeac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,168)?)?;
    // VMA 0xabeb0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xabeb4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,172)?)?;
    // VMA 0xabeb8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,56)?)?;
    // VMA 0xabebc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,60)?)?;
    // VMA 0xabec0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xabec4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,1048576)?)?;
    // VMA 0xabec8
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xabecc
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,128)?)?;
    // VMA 0xabed0
    c.b(C::Eq,0xabf18,false)?;
    // VMA 0xabed4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xabed8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabedc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // VMA 0xabee0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xabee4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xabee8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xabeec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // VMA 0xabef0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // VMA 0xabef4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xabef8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xabefc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xabf00
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xabf04
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1932)?)?;
    // VMA 0xabf08
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xabf0c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1936)?)?;
    // VMA 0xabf10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xabf14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,28)?)?;
    // VMA 0xabf18
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,4194304)?)?;
    // VMA 0xabf1c
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,61)?)?;
    // VMA 0xabf20
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,1)?)?;
    // VMA 0xabf24
    c.b(C::Eq,0xabf84,false)?;
    // VMA 0xabf28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1712)?)?;
    // VMA 0xabf2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,668)?)?;
    // VMA 0xabf30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xabf34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabf38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // VMA 0xabf3c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // VMA 0xabf40
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xabf44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1024)?)?;
    // VMA 0xabf48
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // VMA 0xabf4c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,4)?)?;
    // VMA 0xabf50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,8)?)?;
    // VMA 0xabf54
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabf58
    c.literal(R::R2,0xac734)?;
    // VMA 0xabf5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,90)?)?;
    // VMA 0xabf60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xabf64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xabf68
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xabf6c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,20)?)?;
    // VMA 0xabf70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,176)?)?;
    // VMA 0xabf74
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xabf78
    c.b(C::Al,0x471c,true)?;
    // VMA 0xabf7c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xabf80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1712)?)?;
    // VMA 0xabf84
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,262144)?)?;
    // VMA 0xabf88
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xabf8c
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,32)?)?;
    // VMA 0xabf90
    c.b(C::Eq,0xabfe4,false)?;
    // VMA 0xabf94
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1872)?)?;
    // VMA 0xabf98
    c.literal(R::R2,0xac734)?;
    // VMA 0xabf9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,230)?)?;
    // VMA 0xabfa0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xabfa4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabfa8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xabfac
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xabfb0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1872)?)?;
    // VMA 0xabfb4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xabfb8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xabfbc
    c.b(C::Al,0xe159c,true)?;
    // VMA 0xabfc0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xabfc4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1872)?)?;
    // VMA 0xabfc8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1184)?)?;
    // VMA 0xabfcc
    c.b(C::Al,0x7fd1c,true)?;
    // VMA 0xabfd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xabfd4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1872)?)?;
    // VMA 0xabfd8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xabfdc
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xabfe0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,20)?)?;
    // VMA 0xabfe4
    c.literal(R::new(10)?,0xac738)?;
    // VMA 0xabfe8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,32)?)?;
    // VMA 0xabfec
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,12)?)?;
    // VMA 0xabff0
    c.b(C::Eq,0xac070,false)?;
    // VMA 0xabff4
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,2008)?)?;
    // VMA 0xabff8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,2012)?)?;
    // VMA 0xabffc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,267386880)?)?;
    // VMA 0xac000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xac004
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2952790016)?)?;
    // VMA 0xac008
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xac00c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xac010
    c.b(C::Al,0x816f8,true)?;
    // VMA 0xac014
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac018
    c.b(C::Ne,0xac070,false)?;
    // VMA 0xac01c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // VMA 0xac020
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,12)?)?;
    // VMA 0xac024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac028
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xac02c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xac030
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xac034
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xac038
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xac03c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac040
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xac044
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xac048
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac04c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15728640)?)?;
    // VMA 0xac050
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xac054
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xac058
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xac05c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1073741824)?)?;
    // VMA 0xac060
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83018)?)?;
    // VMA 0xac064
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82d30)?)?;
    // VMA 0xac068
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac06c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,24)?)?;
    // VMA 0xac070
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,1024)?)?;
    // VMA 0xac074
    c.b(C::Eq,0xac710,false)?;
    // VMA 0xac078
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,12)?)?;
    // VMA 0xac07c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xac080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac084
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac088
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac08c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac090
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac094
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(10)?,1)?)?;
    // VMA 0xac098
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // VMA 0xac09c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xac0a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xac0a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // VMA 0xac0a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xac0ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1908)?)?;
    // VMA 0xac0b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac0b4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,24)?)?;
    // VMA 0xac0b8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,65536)?)?;
    // VMA 0xac0bc
    c.b(C::Eq,0xac104,false)?;
    // VMA 0xac0c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac0c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac0c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac0cc
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac0d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac0d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xac0d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // VMA 0xac0dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xac0e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xac0e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xac0e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xac0ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2016)?)?;
    // VMA 0xac0f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xac0f4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2020)?)?;
    // VMA 0xac0f8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,3)?)?;
    // VMA 0xac0fc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac100
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,28)?)?;
    // VMA 0xac104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,964)?)?;
    // VMA 0xac108
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xac10c
    c.emit(arm32::load_signed_halfword_register(C::Al,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xac110
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // VMA 0xac114
    c.emit(arm32::data_immediate_with_flags(C::Ne,Op::And,R::R1,R::new(7)?,2048,true)?)?;
    // VMA 0xac118
    c.b(C::Eq,0xac17c,false)?;
    // VMA 0xac11c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xac120
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // VMA 0xac128
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac12c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xac130
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,8)?)?;
    // VMA 0xac134
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,12)?)?;
    // VMA 0xac138
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,16)?)?;
    // VMA 0xac13c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // VMA 0xac140
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // VMA 0xac144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(10)?,1)?)?;
    // VMA 0xac148
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,20)?)?;
    // VMA 0xac14c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,32)?)?;
    // VMA 0xac150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xac154
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,24)?)?;
    // VMA 0xac158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,198)?)?;
    // VMA 0xac15c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,28)?)?;
    // VMA 0xac160
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xac164
    c.emit(arm32::load_signed_halfword_register(C::Al,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xac168
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,40)?)?;
    // VMA 0xac16c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,36)?)?;
    // VMA 0xac170
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,44)?)?;
    // VMA 0xac174
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xac178
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,576)?)?;
    // VMA 0xac17c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,131072)?)?;
    // VMA 0xac180
    c.b(C::Eq,0xac294,false)?;
    // VMA 0xac184
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,44)?)?;
    // VMA 0xac188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xac18c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xac190
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xac194
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1948)?)?;
    // VMA 0xac198
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,28)?)?;
    // VMA 0xac19c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1952)?)?;
    // VMA 0xac1a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,32)?)?;
    // VMA 0xac1a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1968)?)?;
    // VMA 0xac1a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,60)?)?;
    // VMA 0xac1ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1972)?)?;
    // VMA 0xac1b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,64)?)?;
    // VMA 0xac1b4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1940)?)?;
    // VMA 0xac1b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac1bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1944)?)?;
    // VMA 0xac1c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xac1c4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1941)?)?;
    // VMA 0xac1c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,36)?)?;
    // VMA 0xac1cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1942)?)?;
    // VMA 0xac1d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac1d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,40)?)?;
    // VMA 0xac1d8
    c.b(C::Eq,0xac218,false)?;
    // VMA 0xac1dc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1956)?)?;
    // VMA 0xac1e0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,44)?)?;
    // VMA 0xac1e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1964)?)?;
    // VMA 0xac1e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xac1ec
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,61)?)?;
    // VMA 0xac1f0
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,2)?)?;
    // VMA 0xac1f4
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(9)?,1960)?)?;
    // VMA 0xac1f8
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(9)?,1868)?)?;
    // VMA 0xac1fc
    c.b(C::Al,0xac22c,false)?;
    // VMA 0xac200
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // VMA 0xac204
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac208
    c.b(C::Eq,0xac584,false)?;
    // VMA 0xac20c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,580)?)?;
    // VMA 0xac210
    c.b(C::Al,0x2038,true)?;
    // VMA 0xac214
    c.b(C::Al,0xac584,false)?;
    // VMA 0xac218
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac21c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,18)?)?;
    // VMA 0xac220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xac224
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,44)?)?;
    // VMA 0xac228
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xac22c
    c.literal(R::R1,0xac73c)?;
    // VMA 0xac230
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,48)?)?;
    // VMA 0xac234
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xac238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xac23c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac240
    c.literal(R::R1,0xac740)?;
    // VMA 0xac244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xac248
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac24c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1943)?)?;
    // VMA 0xac250
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,76)?)?;
    // VMA 0xac258
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,4)?)?;
    // VMA 0xac25c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // VMA 0xac260
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,56)?)?;
    // VMA 0xac264
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1976)?)?;
    // VMA 0xac268
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,68)?)?;
    // VMA 0xac26c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1980)?)?;
    // VMA 0xac270
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,72)?)?;
    // VMA 0xac274
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac278
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac27c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xac280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,76)?)?;
    // VMA 0xac284
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac288
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac28c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xac290
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,576)?)?;
    // VMA 0xac294
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,32768)?)?;
    // VMA 0xac298
    c.b(C::Eq,0xac318,false)?;
    // VMA 0xac29c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac2a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac2a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac2a8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac2ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac2b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // VMA 0xac2b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // VMA 0xac2b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // VMA 0xac2bc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xac2c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // VMA 0xac2c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xac2c8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1912)?)?;
    // VMA 0xac2cc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,20)?)?;
    // VMA 0xac2d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,1916)?)?;
    // VMA 0xac2d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,24)?)?;
    // VMA 0xac2d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1920)?)?;
    // VMA 0xac2dc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,28)?)?;
    // VMA 0xac2e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1924)?)?;
    // VMA 0xac2e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,32)?)?;
    // VMA 0xac2e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,1928)?)?;
    // VMA 0xac2ec
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,36)?)?;
    // VMA 0xac2f0
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac2f4
    c.literal(R::R1,0xac744)?;
    // VMA 0xac2f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac2fc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac300
    c.literal(R::R1,0xac740)?;
    // VMA 0xac304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac308
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac30c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xac310
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,40)?)?;
    // VMA 0xac314
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,576)?)?;
    // VMA 0xac318
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,2097152)?)?;
    // VMA 0xac31c
    c.b(C::Eq,0xac71c,false)?;
    // VMA 0xac320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac324
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,316)?)?;
    // VMA 0xac32c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac330
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac334
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac338
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac33c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,12)?)?;
    // VMA 0xac340
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,16)?)?;
    // VMA 0xac344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,336)?)?;
    // VMA 0xac348
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // VMA 0xac34c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // VMA 0xac350
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,117440512)?)?;
    // VMA 0xac354
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,20)?)?;
    // VMA 0xac358
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,32)?)?;
    // VMA 0xac35c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,312)?)?;
    // VMA 0xac360
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xac364
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,28)?)?;
    // VMA 0xac368
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,72)?)?;
    // VMA 0xac36c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,40)?)?;
    // VMA 0xac370
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // VMA 0xac374
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,36)?)?;
    // VMA 0xac378
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,60)?)?;
    // VMA 0xac37c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,44)?)?;
    // VMA 0xac380
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,64)?)?;
    // VMA 0xac384
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,48)?)?;
    // VMA 0xac388
    c.b(C::Eq,0xac39c,false)?;
    // VMA 0xac38c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // VMA 0xac390
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xac394
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,16)?)?;
    // VMA 0xac398
    c.b(C::Al,0xac3a8,false)?;
    // VMA 0xac39c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // VMA 0xac3a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xac3a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // VMA 0xac3a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xac3ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,72)?)?;
    // VMA 0xac3b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,56)?)?;
    // VMA 0xac3b4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,40)?)?;
    // VMA 0xac3b8
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Asr,8)?)?;
    // VMA 0xac3bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,60)?)?;
    // VMA 0xac3c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xac3c4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,4,false)?)?;
    // VMA 0xac3c8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsr,6,false)?)?;
    // VMA 0xac3cc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,64)?)?;
    // VMA 0xac3d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,68)?)?;
    // VMA 0xac3d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,88)?)?;
    // VMA 0xac3d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // VMA 0xac3dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2048)?)?;
    // VMA 0xac3e0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xac3e4
    c.b(C::Al,0x471c,true)?;
    // VMA 0xac3e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,204)?)?;
    // VMA 0xac3ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xac3f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // VMA 0xac3f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xac3f8
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xac3fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,72)?)?;
    // VMA 0xac400
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac404
    c.b(C::Eq,0xac458,false)?;
    // VMA 0xac408
    c.literal(R::new(7)?,0xac748)?;
    // VMA 0xac40c
    c.literal(R::new(10)?,0xac740)?;
    // VMA 0xac410
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(6)?,76)?)?;
    // VMA 0xac414
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,32)?)?;
    // VMA 0xac418
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,64)?)?;
    // VMA 0xac41c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,96)?)?;
    // VMA 0xac420
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,0)?)?;
    // VMA 0xac424
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac42c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xac430
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xac434
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac438
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac43c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xac440
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac444
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xac448
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xac44c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    // VMA 0xac450
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xac454
    c.b(C::Cc,0xac414,false)?;
    // VMA 0xac458
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // VMA 0xac45c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,8)?)?;
    // VMA 0xac460
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,12)?)?;
    // VMA 0xac464
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xac468
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,336)?)?;
    // VMA 0xac46c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xac470
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,8388608)?)?;
    // VMA 0xac474
    c.b(C::Eq,0xac4dc,false)?;
    // VMA 0xac478
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac47c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac480
    c.b(C::Ne,0xac724,false)?;
    // VMA 0xac484
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,1000)?)?;
    // VMA 0xac488
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // VMA 0xac48c
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xac490
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xac494
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac498
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xac49c
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac4a0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,48)?)?;
    // VMA 0xac4a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // VMA 0xac4a8
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xac4ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,76)?)?;
    // VMA 0xac4b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac4b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac4b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,25)?)?;
    // VMA 0xac4bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // VMA 0xac4c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xac4c4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac4c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xac4cc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xac4d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac4d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,76)?)?;
    // VMA 0xac4d8
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac4dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac4e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac4e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // VMA 0xac4e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xac4ec
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac4f0
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R2.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac4f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac4f8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,12)?)?;
    // VMA 0xac4fc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,16)?)?;
    // VMA 0xac500
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,580)?)?;
    // VMA 0xac504
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // VMA 0xac508
    c.b(C::Al,0x205c,true)?;
    // VMA 0xac50c
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,12)?)?;
    // VMA 0xac510
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1908)?)?;
    // VMA 0xac514
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac518
    c.b(C::Eq,0xac52c,false)?;
    // VMA 0xac51c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac520
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // VMA 0xac524
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac528
    c.b(C::Al,0xac538,false)?;
    // VMA 0xac52c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1904)?)?;
    // VMA 0xac530
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac534
    c.b(C::Ne,0xac51c,false)?;
    // VMA 0xac538
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2516)?)?;
    // VMA 0xac53c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac540
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac544
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xac548
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xac54c
    c.b(C::Eq,0xac6e0,false)?;
    // VMA 0xac550
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,88)?)?;
    // VMA 0xac554
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xac558
    c.b(C::Eq,0xac6f0,false)?;
    // VMA 0xac55c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1896)?)?;
    // VMA 0xac560
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac564
    c.b(C::Ne,0xac700,false)?;
    // VMA 0xac568
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // VMA 0xac56c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xac570
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,52)?)?;
    // VMA 0xac574
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,52)?)?;
    // VMA 0xac578
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xac57c
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,27)?)?;
    // VMA 0xac580
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,44)?)?;
    // VMA 0xac584
    c.b(C::Al,0x26754,true)?;
    // VMA 0xac588
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xac58c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac590
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,52)?)?;
    // VMA 0xac594
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,92)?)?;
    // VMA 0xac598
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // VMA 0xac59c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xac5a0
    c.literal(R::R1,0xac74c)?;
    // VMA 0xac5a4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,12)?)?;
    // VMA 0xac5a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xac5ac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac5b0
    c.literal(R::R1,0xac740)?;
    // VMA 0xac5b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xac5b8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac5bc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,720)?)?;
    // VMA 0xac5c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,864)?)?;
    // VMA 0xac5c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xac5c8
    c.b(C::Eq,0xac5dc,false)?;
    // VMA 0xac5cc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xac5d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xac5d4
    c.b(C::Al,0x2a18c,true)?;
    // VMA 0xac5d8
    c.b(C::Al,0xac5e4,false)?;
    // VMA 0xac5dc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xac5e0
    c.b(C::Al,0x2a878,true)?;
    // VMA 0xac5e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac5e8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xac5ec
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xac5f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1872)?)?;
    // VMA 0xac5f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xac5f8
    c.b(C::Eq,0xac60c,false)?;
    // VMA 0xac5fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1184)?)?;
    // VMA 0xac600
    c.b(C::Al,0x7fd1c,true)?;
    // VMA 0xac604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xac608
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1872)?)?;
    // VMA 0xac60c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,1)?)?;
    // VMA 0xac610
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(9)?,28)?)?;
    // VMA 0xac614
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac618
    c.b(C::Ne,0xabce0,false)?;
    // VMA 0xac61c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // VMA 0xac620
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac624
    c.b(C::Ne,0xabce0,false)?;
    // VMA 0xac628
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,948)?)?;
    // VMA 0xac62c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xac630
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xac634
    c.b(C::Al,0xda564,true)?;
    // VMA 0xac638
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xac63c
    c.b(C::Al,0xac5e8,false)?;
    // VMA 0xac640
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,1884)?)?;
    // VMA 0xac644
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,4)?)?;
    // VMA 0xac648
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(5)?,R::new(7)?)?)?;
    // VMA 0xac64c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // VMA 0xac650
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xac654
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // VMA 0xac658
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // VMA 0xac65c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,868)?)?;
    // VMA 0xac660
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xac664
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,1880)?)?;
    // VMA 0xac668
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(4)?,R::new(9)?,R::R0)?)?;
    // VMA 0xac66c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,12)?)?;
    // VMA 0xac670
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,8)?)?;
    // VMA 0xac674
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(10)?,R::new(4)?)?)?;
    // VMA 0xac678
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xac67c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xac680
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // VMA 0xac684
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // VMA 0xac688
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,72)?)?;
    // VMA 0xac68c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,98)?)?;
    // VMA 0xac690
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1792)?)?;
    // VMA 0xac694
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xac698
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xac69c
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::new(5)?)?)?;
    // VMA 0xac6a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // VMA 0xac6a4
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xac6a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // VMA 0xac6ac
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xac6b0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,12)?)?;
    // VMA 0xac6b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,60)?)?;
    // VMA 0xac6b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,102)?)?;
    // VMA 0xac6bc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1792)?)?;
    // VMA 0xac6c0
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xac6c4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xac6c8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,8)?)?;
    // VMA 0xac6cc
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::new(10)?)?)?;
    // VMA 0xac6d0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xac6d4
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,12)?)?;
    // VMA 0xac6d8
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xac6dc
    c.b(C::Al,0xabd94,false)?;
    // VMA 0xac6e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac6e4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xac6e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac6ec
    c.b(C::Al,0xac550,false)?;
    // VMA 0xac6f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac6f4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32)?)?;
    // VMA 0xac6f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac6fc
    c.b(C::Al,0xac55c,false)?;
    // VMA 0xac700
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,536870912)?)?;
    // VMA 0xac708
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac70c
    c.b(C::Al,0xac568,false)?;
    // VMA 0xac710
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,12)?)?;
    // VMA 0xac714
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xac718
    c.b(C::Al,0xac0b8,false)?;
    // VMA 0xac71c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // VMA 0xac720
    c.b(C::Al,0xac470,false)?;
    // VMA 0xac724
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,2080)?)?;
    // VMA 0xac728
    c.b(C::Al,0xac48c,false)?;
    c.check(0xac72c)
}

const CODEC_REGISTER_PACKET_LITERALS:[u32;9]=[
    0x0022_1008, // codec capability mask/ID
    0x0101_0200, // packed buffer capability data
    0x000f_8519, // diagnostic string
    0x7f10_0002, // opcode/tag data
    0x000f_8551, // diagnostic
    0x000e_2788, // common logger
    0x000f_6864,
    0x000f_859f,
    0x000f_85d7,
];

/// Build codec command metadata, validate input dimensions, invoke release callbacks and publish the remaining frame-header fields.
/// Runtime slice 0xac750..0xac9f0, with 3 classified trailing data words.
fn serialize_codec_resource_release(c:&mut Code)->Result<(),String>{
    c.check(0xac750)?;
    // ARM 0xac750: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xac754: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xac758: sub sp, sp, #68
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,68)?)?;
    // ARM 0xac75c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xac760: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // ARM 0xac764: mov r0, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,156)?)?;
    // ARM 0xac768: ldr r6, [r5, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2444)?)?;
    // ARM 0xac76c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // ARM 0xac770: bl 0x2f8d0 <entry+0xfffffffffff83180>
    c.b(C::Al,0x2f8d0,true)?;
    // ARM 0xac774: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac778: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM 0xac77c: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // ARM 0xac780: movne r1, #24
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,24)?)?;
    // ARM 0xac784: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xac788: cmp r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    // ARM 0xac78c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // ARM 0xac790: bls 0xac7a8 <entry+0x58>
    c.b(C::Ls,0xac7a8,false)?;
    // ARM 0xac794: ldr r2, [pc, #0x248]
    c.literal(R::R2,0xac9e4)?;
    // ARM 0xac798: mov r1, #50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,50)?)?;
    // ARM 0xac79c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xac7a0: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // ARM 0xac7a4: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xac7a8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xac7ac: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // ARM 0xac7b0: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // ARM 0xac7b4: orr r0, r0, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // ARM 0xac7b8: orr r9, r9, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,16777216)?)?;
    // ARM 0xac7bc: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // ARM 0xac7c0: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // ARM 0xac7c4: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // ARM 0xac7c8: add r0, r9, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,2)?)?;
    // ARM 0xac7cc: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // ARM 0xac7d0: str r8, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,20)?)?;
    // ARM 0xac7d4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xac7d8: ldrb r1, [r6, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,110)?)?;
    // ARM 0xac7dc: ldr r7, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,76)?)?;
    // ARM 0xac7e0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xac7e4: bne 0xac7f4 <entry+0xa4>
    c.b(C::Ne,0xac7f4,false)?;
    // ARM 0xac7e8: ldrb r0, [r6, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,111)?)?;
    // ARM 0xac7ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac7f0: movne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xac7f4: str r0, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,76)?)?;
    // ARM 0xac7f8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xac7fc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xac800: bl 0xac9f0 <entry+0x2a0>
    c.b(C::Al,0xac9f0,true)?;
    // ARM 0xac804: ldr r1, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,76)?)?;
    // ARM 0xac808: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // ARM 0xac80c: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // ARM 0xac810: cmp r7, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xac814: movne r10, r8
    c.emit(arm32::mov_register(C::Ne,R::new(10)?,R::new(8)?))?;
    // ARM 0xac818: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac81c: beq 0xac828 <entry+0xd8>
    c.b(C::Eq,0xac828,false)?;
    // ARM 0xac820: cmp r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,1)?)?;
    // ARM 0xac824: beq 0xac8b8 <entry+0x168>
    c.b(C::Eq,0xac8b8,false)?;
    // ARM 0xac828: add r0, r5, #948
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,948)?)?;
    // ARM 0xac82c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xac830: ldr r0, [r5, #0x48c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1164)?)?;
    // ARM 0xac834: bl 0xd9e10 <entry+0x2d6c0>
    c.b(C::Al,0xd9e10,true)?;
    // ARM 0xac838: ldr r0, [r5, #0x488]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1160)?)?;
    // ARM 0xac83c: bl 0xd9e10 <entry+0x2d6c0>
    c.b(C::Al,0xd9e10,true)?;
    // ARM 0xac840: add r7, r5, #1152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,1152)?)?;
    // ARM 0xac844: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // ARM 0xac848: ldr r0, [r7, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,-8)?)?;
    // ARM 0xac84c: bl 0xd9e10 <entry+0x2d6c0>
    c.b(C::Al,0xd9e10,true)?;
    // ARM 0xac850: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // ARM 0xac854: bl 0xd9e10 <entry+0x2d6c0>
    c.b(C::Al,0xd9e10,true)?;
    // ARM 0xac858: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // ARM 0xac85c: add r7, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,4)?)?;
    // ARM 0xac860: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // ARM 0xac864: bne 0xac848 <entry+0xf8>
    c.b(C::Ne,0xac848,false)?;
    // ARM 0xac868: ldr r0, [r5, #0x494]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1172)?)?;
    // ARM 0xac86c: bl 0x7ecc0 <entry+0xfffffffffffd2570>
    c.b(C::Al,0x7ecc0,true)?;
    // ARM 0xac870: ldr r0, [r5, #0x49c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1180)?)?;
    // ARM 0xac874: bl 0x7ecc0 <entry+0xfffffffffffd2570>
    c.b(C::Al,0x7ecc0,true)?;
    // ARM 0xac878: ldr r3, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,32)?)?;
    // ARM 0xac87c: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // ARM 0xac880: bne 0xac9b8 <entry+0x268>
    c.b(C::Ne,0xac9b8,false)?;
    // ARM 0xac884: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xac888: beq 0xac9b8 <entry+0x268>
    c.b(C::Eq,0xac9b8,false)?;
    // ARM 0xac88c: ldr r2, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,0)?)?;
    // ARM 0xac890: add r0, r5, #728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,728)?)?;
    // ARM 0xac894: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xac898: bl 0xdb350 <entry+0x2ec00>
    c.b(C::Al,0xdb350,true)?;
    // ARM 0xac89c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac8a0: beq 0xac8b8 <entry+0x168>
    c.b(C::Eq,0xac8b8,false)?;
    // ARM 0xac8a4: ldr r2, [pc, #0x138]
    c.literal(R::R2,0xac9e4)?;
    // ARM 0xac8a8: mov r1, #95
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,95)?)?;
    // ARM 0xac8ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xac8b0: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // ARM 0xac8b4: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xac8b8: ldr r3, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,32)?)?;
    // ARM 0xac8bc: add r0, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,16)?)?;
    // ARM 0xac8c0: str r9, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R0,0)?)?;
    // ARM 0xac8c4: ldr r1, [pc, #0x11c]
    c.literal(R::R1,0xac9e8)?;
    // ARM 0xac8c8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xac8cc: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // ARM 0xac8d0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xac8d4: ldr r1, [pc, #0x110]
    c.literal(R::R1,0xac9ec)?;
    // ARM 0xac8d8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xac8dc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xac8e0: sub r1, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // ARM 0xac8e4: sub r2, r11, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,36)?)?;
    // ARM 0xac8e8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xac8ec: bl 0xaf440 <entry+0x2cf0>
    c.b(C::Al,0xaf440,true)?;
    // ARM 0xac8f0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // ARM 0xac8f4: str r0, [r5, #0x2ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,684)?)?;
    // ARM 0xac8f8: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // ARM 0xac8fc: str r0, [r5, #0x2b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,688)?)?;
    // ARM 0xac900: ldrh r0, [r6, #98]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(6)?,98)?)?;
    // ARM 0xac904: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM 0xac908: ldrh r1, [r6, #100]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(6)?,100)?)?;
    // ARM 0xac90c: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // ARM 0xac910: ldrh r2, [r6, #102]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(6)?,102)?)?;
    // ARM 0xac914: lsl r3, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsl,1)?)?;
    // ARM 0xac918: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // ARM 0xac91c: ldrh r3, [r6, #106]
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(6)?,106)?)?;
    // ARM 0xac920: lsl r7, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(3)?,Shift::Lsl,1)?)?;
    // ARM 0xac924: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // ARM 0xac928: ldrh r7, [r6, #104]
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::new(6)?,104)?)?;
    // ARM 0xac92c: add r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xac930: sub r0, r0, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,1,false)?)?;
    // ARM 0xac934: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // ARM 0xac938: ldrh r0, [r6, #108]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(6)?,108)?)?;
    // ARM 0xac93c: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xac940: sub r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xac944: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // ARM 0xac948: ldrb r0, [r6, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,110)?)?;
    // ARM 0xac94c: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM 0xac950: ldrb r0, [r6, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,111)?)?;
    // ARM 0xac954: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM 0xac958: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xac95c: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // ARM 0xac960: ldr r0, [r6, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1764)?)?;
    // ARM 0xac964: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // ARM 0xac968: ldrb r0, [r6, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,123)?)?;
    // ARM 0xac96c: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // ARM 0xac970: ldrb r0, [r6, #0x83]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,131)?)?;
    // ARM 0xac974: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // ARM 0xac978: bl 0x2f8d0 <entry+0xfffffffffff83180>
    c.b(C::Al,0x2f8d0,true)?;
    // ARM 0xac97c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac980: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // ARM 0xac984: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xac988: ldrbeq r0, [r6, #0x7b]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(6)?,123)?)?;
    // ARM 0xac98c: ldrne r0, [r5, #0xc48]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,3144)?)?;
    // ARM 0xac990: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // ARM 0xac994: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // ARM 0xac998: add r0, r4, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,24)?)?;
    // ARM 0xac99c: bl 0xe13b0 <entry+0x34c60>
    c.b(C::Al,0xe13b0,true)?;
    // ARM 0xac9a0: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // ARM 0xac9a4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // ARM 0xac9a8: ldr r0, [r5, #0x360]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,864)?)?;
    // ARM 0xac9ac: bl 0x2a878 <entry+0xfffffffffff7e128>
    c.b(C::Al,0x2a878,true)?;
    // ARM 0xac9b0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xac9b4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xac9b8: add r0, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,16)?)?;
    // ARM 0xac9bc: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // ARM 0xac9c0: bne 0xac8c0 <entry+0x170>
    c.b(C::Ne,0xac8c0,false)?;
    // ARM 0xac9c4: add r1, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,1)?)?;
    // ARM 0xac9c8: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // ARM 0xac9cc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xac9d0: str r0, [r5, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,324)?)?;
    // ARM 0xac9d4: str r0, [r5, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,332)?)?;
    // ARM 0xac9d8: str r0, [r5, #0x21c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,540)?)?;
    // ARM 0xac9dc: str r0, [r5, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,532)?)?;
    // ARM 0xac9e0: b 0xac8c4 <entry+0x174>
    c.b(C::Al,0xac8c4,false)?;
    c.check(0xac9e4)?;
    // ARM 0xac9e4: literal or constant data (NOT executable)
    c.emit(0x000f8519)?;
    // ARM 0xac9e8: literal or constant data (NOT executable)
    c.emit(0x000f8615)?;
    // ARM 0xac9ec: literal or constant data (NOT executable)
    c.emit(0x000e2788)?;
    c.check(0xac9f0)
}

/// Initialize the next codec allocator, combine configured pixel dimensions, compute scaled plane sizes and perform checked 64-bit arithmetic.
/// Runtime slice 0xac9f0..0xace00, with 0 classified trailing data words.
fn begin_codec_allocation_and_scaled_dimensions(c:&mut Code)->Result<(),String>{
    c.check(0xac9f0)?;
    // ARM 0xac9f0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xac9f4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xac9f8: sub sp, sp, #116
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,116)?)?;
    // ARM 0xac9fc: str r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-56)?)?;
    // ARM 0xaca00: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xaca04: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // ARM 0xaca08: mov r0, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,616)?)?;
    // ARM 0xaca0c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xaca10: mvn r5, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(5)?,R::R0,127)?)?;
    // ARM 0xaca14: ldr r3, [r4, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2444)?)?;
    // ARM 0xaca18: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xaca1c: ldrh r0, [r3, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(3)?,R::R0)?)?;
    // ARM 0xaca20: ldrh r1, [r3, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(3)?,R::R1)?)?;
    // ARM 0xaca24: mul r7, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::R1,R::R0)?)?;
    // ARM 0xaca28: str r7, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,216)?)?;
    // ARM 0xaca2c: ldrb r0, [r3, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,165)?)?;
    // ARM 0xaca30: subs r1, r0, #4
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R1,R::R0,4,true)?)?;
    // ARM 0xaca34: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // ARM 0xaca38: movlo r1, #0
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xaca3c: str r1, [r4, #0x208]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,520)?)?;
    // ARM 0xaca40: mov r1, #942080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,942080)?)?;
    // ARM 0xaca44: cmp r7, r1, lsr r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsr,R::R0,false)?)?;
    // ARM 0xaca48: ldrh r9, [r3, #100]
    c.emit(arm32::load_halfword(C::Al,R::new(9)?,R::new(3)?,100)?)?;
    // ARM 0xaca4c: ldrh r8, [r3, #98]
    c.emit(arm32::load_halfword(C::Al,R::new(8)?,R::new(3)?,98)?)?;
    // ARM 0xaca50: movle r2, #1
    c.emit(arm32::data_immediate(C::Le,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM 0xaca54: str r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,20)?)?;
    // ARM 0xaca58: ldr r0, [r4, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,696)?)?;
    // ARM 0xaca5c: add r10, r8, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(8)?,63)?)?;
    // ARM 0xaca60: add lr, r9, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(9)?,63)?)?;
    // ARM 0xaca64: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM 0xaca68: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xaca6c: streq r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,20)?)?;
    // ARM 0xaca70: mov r0, #53504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,53504)?)?;
    // ARM 0xaca74: ldrb r2, [r3, #0xa7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(3)?,167)?)?;
    // ARM 0xaca78: ldr r6, [pc, #0xc70]
    c.literal(R::new(6)?,0xad6f0)?;
    // ARM 0xaca7c: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // ARM 0xaca80: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // ARM 0xaca84: mov r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,64)?)?;
    // ARM 0xaca88: cmp r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,64)?)?;
    // ARM 0xaca8c: blo 0xacaa0 <entry+0x350>
    c.b(C::Cc,0xacaa0,false)?;
    // ARM 0xaca90: strb r6, [r3, #0xa7]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(3)?,167)?)?;
    // ARM 0xaca94: mov r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256)?)?;
    // ARM 0xaca98: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xaca9c: b 0xacad4 <entry+0x384>
    c.b(C::Al,0xacad4,false)?;
    // ARM 0xacaa0: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xacaa4: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // ARM 0xacaa8: bne 0xacab4 <entry+0x364>
    c.b(C::Ne,0xacab4,false)?;
    // ARM 0xacaac: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // ARM 0xacab0: b 0xacad4 <entry+0x384>
    c.b(C::Al,0xacad4,false)?;
    // ARM 0xacab4: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // ARM 0xacab8: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xacabc: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xacac0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xacac4: movls r12, #16
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xacac8: movls r6, #64
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(6)?,R::R0,64)?)?;
    // ARM 0xacacc: movls r2, #16
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R2,R::R0,16)?)?;
    // ARM 0xacad0: strbls r12, [r3, #0xa7]
    c.emit(arm32::transfer_byte(C::Ls,false,R::R12,R::new(3)?,167)?)?;
    // ARM 0xacad4: add r1, r0, #11968
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,11968)?)?;
    // ARM 0xacad8: add r0, r9, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,15)?)?;
    // ARM 0xacadc: str r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-44)?)?;
    // ARM 0xacae0: add r0, r8, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,15)?)?;
    // ARM 0xacae4: str r0, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-68)?)?;
    // ARM 0xacae8: and r0, r5, r10, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::new(10)?,Shift::Lsl,1,false)?)?;
    // ARM 0xacaec: lsr r5, lr, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::LR,Shift::Lsr,6)?)?;
    // ARM 0xacaf0: str r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-64)?)?;
    // ARM 0xacaf4: mul r0, r2, r7
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(7)?)?)?;
    // ARM 0xacaf8: str r8, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,220)?)?;
    // ARM 0xacafc: str r9, [r4, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,224)?)?;
    // ARM 0xacb00: add r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,255)?)?;
    // ARM 0xacb04: mul r0, r8, r9
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(8)?,R::new(9)?)?)?;
    // ARM 0xacb08: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xacb0c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // ARM 0xacb10: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,76)?)?;
    // ARM 0xacb14: str r3, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,56)?)?;
    // ARM 0xacb18: str r7, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,52)?)?;
    // ARM 0xacb1c: str r6, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,48)?)?;
    // ARM 0xacb20: str r12, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,44)?)?;
    // ARM 0xacb24: str r9, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R11,-52)?)?;
    // ARM 0xacb28: str r1, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-60)?)?;
    // ARM 0xacb2c: str r5, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,64)?)?;
    // ARM 0xacb30: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // ARM 0xacb34: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM 0xacb38: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // ARM 0xacb3c: bne 0xacb9c <entry+0x44c>
    c.b(C::Ne,0xacb9c,false)?;
    // ARM 0xacb40: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // ARM 0xacb44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xacb48: beq 0xacbc4 <entry+0x474>
    c.b(C::Eq,0xacbc4,false)?;
    // ARM 0xacb4c: str r10, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R11,-48)?)?;
    // ARM 0xacb50: add r0, r8, #191
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,191)?)?;
    // ARM 0xacb54: ldr r1, [pc, #0xb9c]
    c.literal(R::R1,0xad6f8)?;
    // ARM 0xacb58: umull r2, r3, r0, r1
    c.emit(arm32::multiply_long(C::Al,false,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // ARM 0xacb5c: ldr r1, [pc, #0xb98]
    c.literal(R::R1,0xad6fc)?;
    // ARM 0xacb60: lsr r0, r3, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,7)?)?;
    // ARM 0xacb64: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xacb68: lsl r0, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,8)?)?;
    // ARM 0xacb6c: smull r2, r3, r0, r1
    c.emit(arm32::multiply_long(C::Al,true,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // ARM 0xacb70: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,120)?)?;
    // ARM 0xacb74: add r7, r3, r3, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::new(3)?,Shift::Lsr,31,false)?)?;
    // ARM 0xacb78: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacb7c: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // ARM 0xacb80: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacb84: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacb88: beq 0xacce4 <entry+0x594>
    c.b(C::Eq,0xacce4,false)?;
    // ARM 0xacb8c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacb90: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacb94: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacb98: b 0xaccec <entry+0x59c>
    c.b(C::Al,0xaccec,false)?;
    // ARM 0xacb9c: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,120)?)?;
    // ARM 0xacba0: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacba4: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // ARM 0xacba8: add r5, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacbac: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacbb0: beq 0xacbec <entry+0x49c>
    c.b(C::Eq,0xacbec,false)?;
    // ARM 0xacbb4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacbb8: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacbbc: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacbc0: b 0xacbf4 <entry+0x4a4>
    c.b(C::Al,0xacbf4,false)?;
    // ARM 0xacbc4: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,120)?)?;
    // ARM 0xacbc8: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacbcc: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // ARM 0xacbd0: add r5, r0, r8, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(8)?,Shift::Lsl,1,false)?)?;
    // ARM 0xacbd4: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacbd8: beq 0xacdb8 <entry+0x668>
    c.b(C::Eq,0xacdb8,false)?;
    // ARM 0xacbdc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacbe0: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacbe4: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacbe8: b 0xacdc0 <entry+0x670>
    c.b(C::Al,0xacdc0,false)?;
    // ARM 0xacbec: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacbf0: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacbf4: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // ARM 0xacbf8: ldr r1, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,124)?)?;
    // ARM 0xacbfc: add r0, r9, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc00: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // ARM 0xacc04: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacc08: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc0c: beq 0xacc20 <entry+0x4d0>
    c.b(C::Eq,0xacc20,false)?;
    // ARM 0xacc10: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacc14: bl 0x81850 <entry+0xfffffffffffd5100>
    c.b(C::Al,0x81850,true)?;
    // ARM 0xacc18: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc1c: b 0xacc28 <entry+0x4d8>
    c.b(C::Al,0xacc28,false)?;
    // ARM 0xacc20: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacc24: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacc28: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xacc2c: ldr r1, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,132)?)?;
    // ARM 0xacc30: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacc34: add r5, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacc38: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc3c: beq 0xacc50 <entry+0x500>
    c.b(C::Eq,0xacc50,false)?;
    // ARM 0xacc40: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacc44: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacc48: sub r9, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(9)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc4c: b 0xacc58 <entry+0x508>
    c.b(C::Al,0xacc58,false)?;
    // ARM 0xacc50: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacc54: and r9, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacc58: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // ARM 0xacc5c: ldr r1, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,136)?)?;
    // ARM 0xacc60: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xacc64: add r0, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // ARM 0xacc68: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // ARM 0xacc6c: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacc70: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc74: beq 0xacc88 <entry+0x538>
    c.b(C::Eq,0xacc88,false)?;
    // ARM 0xacc78: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacc7c: bl 0x81850 <entry+0xfffffffffffd5100>
    c.b(C::Al,0x81850,true)?;
    // ARM 0xacc80: sub r3, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc84: b 0xacc90 <entry+0x540>
    c.b(C::Al,0xacc90,false)?;
    // ARM 0xacc88: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacc8c: and r3, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacc90: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // ARM 0xacc94: ldr r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // ARM 0xacc98: ldr r12, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-60)?)?;
    // ARM 0xacc9c: ldr r6, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,64)?)?;
    // ARM 0xacca0: mul r7, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::R1,R::R0)?)?;
    // ARM 0xacca4: ldr r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,88)?)?;
    // ARM 0xacca8: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // ARM 0xaccac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaccb0: beq 0xaccd8 <entry+0x588>
    c.b(C::Eq,0xaccd8,false)?;
    // ARM 0xaccb4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xaccb8: orr r1, r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3840)?)?;
    // ARM 0xaccbc: mla r2, r3, r9, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::new(3)?,R::new(9)?,R::R1)?)?;
    // ARM 0xaccc0: ldr r3, [pc, #0xa2c]
    c.literal(R::new(3)?,0xad6f4)?;
    // ARM 0xaccc4: add r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaccc8: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacccc: and r7, r1, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaccd0: add r5, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaccd4: b 0xace9c <entry+0x74c>
    c.b(C::Al,0xace9c,false)?;
    // ARM 0xaccd8: mla r5, r3, r9, r7
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(3)?,R::new(9)?,R::new(7)?)?)?;
    // ARM 0xaccdc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xacce0: b 0xace9c <entry+0x74c>
    c.b(C::Al,0xace9c,false)?;
    // ARM 0xacce4: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacce8: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaccec: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // ARM 0xaccf0: ldr r1, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,124)?)?;
    // ARM 0xaccf4: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xaccf8: add r5, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaccfc: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd00: beq 0xacd14 <entry+0x5c4>
    c.b(C::Eq,0xacd14,false)?;
    // ARM 0xacd04: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacd08: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacd0c: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd10: b 0xacd1c <entry+0x5cc>
    c.b(C::Al,0xacd1c,false)?;
    // ARM 0xacd14: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacd18: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacd1c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xacd20: ldr r1, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,132)?)?;
    // ARM 0xacd24: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacd28: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacd2c: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd30: beq 0xacd44 <entry+0x5f4>
    c.b(C::Eq,0xacd44,false)?;
    // ARM 0xacd34: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacd38: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacd3c: sub r9, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(9)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd40: b 0xacd4c <entry+0x5fc>
    c.b(C::Al,0xacd4c,false)?;
    // ARM 0xacd44: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacd48: and r9, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacd4c: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // ARM 0xacd50: ldr r1, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,136)?)?;
    // ARM 0xacd54: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // ARM 0xacd58: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xacd5c: add r5, r2, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsr,1,false)?)?;
    // ARM 0xacd60: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd64: beq 0xacd78 <entry+0x628>
    c.b(C::Eq,0xacd78,false)?;
    // ARM 0xacd68: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacd6c: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacd70: sub r2, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd74: b 0xacd80 <entry+0x630>
    c.b(C::Al,0xacd80,false)?;
    // ARM 0xacd78: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacd7c: and r2, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacd80: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xacd84: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // ARM 0xacd88: ldr r10, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-48)?)?;
    // ARM 0xacd8c: ldr r12, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-60)?)?;
    // ARM 0xacd90: ldr r6, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,64)?)?;
    // ARM 0xacd94: orr r1, r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3840)?)?;
    // ARM 0xacd98: mla r0, r2, r9, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R2,R::new(9)?,R::R1)?)?;
    // ARM 0xacd9c: ldr r2, [pc, #0x950]
    c.literal(R::R2,0xad6f4)?;
    // ARM 0xacda0: ldr r7, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,68)?)?;
    // ARM 0xacda4: ldr r5, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-40)?)?;
    // ARM 0xacda8: mla r3, r5, r7, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::new(5)?,R::new(7)?,R::R1)?)?;
    // ARM 0xacdac: and r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xacdb0: and r7, r3, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xacdb4: b 0xace94 <entry+0x744>
    c.b(C::Al,0xace94,false)?;
    // ARM 0xacdb8: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacdbc: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacdc0: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // ARM 0xacdc4: lsl r7, r8, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(8)?,Shift::Lsl,1)?)?;
    // ARM 0xacdc8: ldr r1, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,124)?)?;
    // ARM 0xacdcc: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacdd0: add r5, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacdd4: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacdd8: beq 0xacdec <entry+0x69c>
    c.b(C::Eq,0xacdec,false)?;
    // ARM 0xacddc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacde0: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacde4: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacde8: b 0xacdf4 <entry+0x6a4>
    c.b(C::Al,0xacdf4,false)?;
    // ARM 0xacdec: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacdf0: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacdf4: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xacdf8: ldr r1, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,132)?)?;
    // ARM 0xacdfc: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    c.check(END)
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
    handle_codec_plane_blending_failures(&mut c)?;
    for data in CODEC_BLEND_LITERALS {c.emit(data)?;}
    encode_codec_controller_output_record(&mut c)?;
    for global in CODEC_OUTPUT_GLOBALS {c.emit(global)?;}
    encode_codec_channel_command_queue(&mut c)?;
    for data in CODEC_CHANNEL_LITERALS {c.emit(data)?;}
    configure_codec_register_fifo(&mut c)?;
    for data in CODEC_FIFO_LITERALS {c.emit(data)?;}
    remap_codec_channel_slots(&mut c)?;
    for addr in SLOT_REMAP_GLOBALS {c.emit(addr)?;}
    link_codec_slot_runtime(&mut c)?;
    publish_codec_parameter_modes(&mut c)?;
    dispatch_controller_codec_events(&mut c)?;
    for word in CODEC_RELAY_DIAGNOSTICS {c.emit(word)?;}
    program_codec_register_packets(&mut c)?;
    for data in CODEC_REGISTER_PACKET_LITERALS {c.emit(data)?;}
    serialize_codec_resource_release(&mut c)?;
    begin_codec_allocation_and_scaled_dimensions(&mut c)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn validates_exclusive_agent06_span() {
        assert_eq!(START,0xa8000);assert_eq!(END,0xace00);
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
        for (i,&val) in CODEC_BLEND_LITERALS.iter().enumerate(){
            assert_eq!(word(0xaa1f4+4*i),val);
        }
        for (i,&val) in CODEC_OUTPUT_GLOBALS.iter().enumerate(){
            assert_eq!(word(0xaa6e8+4*i),val);
        }
        for (i,&val) in CODEC_CHANNEL_LITERALS.iter().enumerate(){
            assert_eq!(word(0xab030+4*i),val);
        }
        for (i,&val) in CODEC_FIFO_LITERALS.iter().enumerate(){
            assert_eq!(word(0xab768+4*i),val);
        }
        for (i,&val) in SLOT_REMAP_GLOBALS.iter().enumerate(){
            assert_eq!(word(0xab8e4+4*i),val);
        }
        for (i,&val) in CODEC_RELAY_DIAGNOSTICS.iter().enumerate(){
            assert_eq!(word(0xabbe4+4*i),val);
        }
        for (i,&val) in CODEC_REGISTER_PACKET_LITERALS.iter().enumerate(){
            assert_eq!(word(0xac72c+4*i),val);
        }
    }
}
