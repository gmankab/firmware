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
pub const END: usize=0xc65ac;
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

// Firmware routines and literal pools remain in their original item namespace.
include!("parts/section_001.rs");
include!("parts/section_002.rs");
include!("parts/section_003.rs");
include!("parts/section_004.rs");
include!("parts/section_005.rs");
include!("parts/section_006.rs");
include!("parts/section_007.rs");
include!("parts/section_008.rs");
include!("parts/section_009.rs");
include!("parts/section_010.rs");
include!("parts/section_011.rs");
include!("parts/section_012.rs");
include!("parts/section_013.rs");
include!("parts/section_014.rs");
include!("parts/section_015.rs");
include!("parts/section_016.rs");
include!("parts/section_017.rs");
include!("parts/section_018.rs");
include!("parts/section_019.rs");
include!("parts/section_020.rs");
include!("parts/section_021.rs");
include!("parts/section_022.rs");
include!("parts/section_023.rs");
include!("parts/section_024.rs");
include!("parts/section_025.rs");
include!("parts/section_026.rs");
include!("parts/section_027.rs");
include!("parts/section_028.rs");
include!("parts/section_029.rs");
include!("parts/section_030.rs");
include!("parts/section_031.rs");
include!("parts/section_032.rs");
include!("parts/section_033.rs");
include!("parts/section_034.rs");
include!("parts/section_035.rs");
include!("parts/section_036.rs");
include!("parts/section_037.rs");
include!("parts/section_038.rs");
include!("parts/section_039.rs");
include!("parts/section_040.rs");
include!("parts/section_041.rs");
include!("parts/section_042.rs");
include!("parts/section_043.rs");
include!("parts/section_044.rs");
include!("parts/section_045.rs");
include!("parts/section_046.rs");
include!("parts/section_047.rs");
include!("parts/section_048.rs");
include!("parts/section_049.rs");
include!("parts/section_050.rs");
include!("parts/section_051.rs");
include!("parts/section_052.rs");
include!("parts/section_053.rs");
include!("parts/section_054.rs");
include!("parts/section_055.rs");
include!("parts/section_056.rs");
include!("parts/section_057.rs");
include!("parts/section_058.rs");
include!("parts/section_059.rs");
include!("parts/section_060.rs");
include!("parts/section_061.rs");
include!("parts/section_062.rs");
include!("parts/section_063.rs");
include!("parts/section_064.rs");
include!("parts/section_065.rs");
include!("parts/section_066.rs");
include!("parts/section_067.rs");
include!("parts/section_068.rs");
include!("parts/section_069.rs");
include!("parts/section_070.rs");
include!("parts/section_071.rs");
include!("parts/section_072.rs");
include!("parts/section_073.rs");
include!("parts/section_074.rs");
include!("parts/section_075.rs");
include!("parts/section_076.rs");
include!("parts/section_077.rs");
include!("parts/section_078.rs");
include!("parts/section_079.rs");
include!("parts/section_080.rs");
include!("parts/section_081.rs");
include!("parts/section_082.rs");
include!("parts/section_083.rs");
include!("parts/section_084.rs");
include!("parts/section_085.rs");
include!("parts/section_086.rs");
include!("parts/section_087.rs");
include!("parts/section_088.rs");
include!("parts/section_089.rs");
include!("parts/section_090.rs");
include!("parts/section_091.rs");
include!("parts/section_092.rs");
