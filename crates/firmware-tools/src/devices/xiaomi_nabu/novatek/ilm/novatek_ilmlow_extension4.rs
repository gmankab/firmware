// SPDX-License-Identifier: GPL-3.0-only
//! Ten-way byte-offset jump table and its bounded indirect branch cases.
//! Table bytes are *computed* from named code targets, not OEM instruction
//! bytes. Its 10-byte data span is not counted as reconstructed NDS32 code.
use super::{Located,Profile};
use super::isa::{Insn,Reg,Width,CompactMisc,AluRegOp};
use Insn::*;
fn r(x:u8)->Reg{Reg::new(x)}
pub const FILE_START:usize=0x2724;
pub const FILE_END:usize=0x275c;
pub fn instructions(profile:Profile)->Vec<Located>{
    if profile==Profile::Map220v{return Vec::new();}
    vec![
        Located{pc:0x2524,instruction:JumpTableOffsets10{
            base:0x2524,targets:[
                0x252e,0x2536,0x253a,0x253e,0x2542,
                0x2554,0x2554,0x2546,0x254a,0x2550,
            ],
        }},
        Located{pc:0x252e,instruction:LoadWordGp{dest:r(2),address:0x2c4f4}}, // N32, file 0x272e
        Located{pc:0x2532,instruction:ShortMemory{store:true,width:Width::Byte,reg:r(0),base:r(2),offset:0}}, // N16, file 0x2732
        Located{pc:0x2534,instruction:JumpShort{target:0x2554}}, // N16, file 0x2734
        Located{pc:0x2536,instruction:ShortMemory{store:true,width:Width::Byte,reg:r(0),base:r(7),offset:0}}, // N16, file 0x2736
        Located{pc:0x2538,instruction:JumpShort{target:0x2554}}, // N16, file 0x2738
        Located{pc:0x253a,instruction:ShortMemory{store:true,width:Width::Byte,reg:r(0),base:r(6),offset:0}}, // N16, file 0x273a
        Located{pc:0x253c,instruction:JumpShort{target:0x2554}}, // N16, file 0x273c
        Located{pc:0x253e,instruction:ShortMemory{store:true,width:Width::Byte,reg:r(0),base:r(5),offset:0}}, // N16, file 0x273e
        Located{pc:0x2540,instruction:JumpShort{target:0x2554}}, // N16, file 0x2740
        Located{pc:0x2542,instruction:ShortMemory{store:true,width:Width::Byte,reg:r(0),base:r(4),offset:0}}, // N16, file 0x2742
        Located{pc:0x2544,instruction:JumpShort{target:0x2554}}, // N16, file 0x2744
        Located{pc:0x2546,instruction:Ex9(421)}, // N16, file 0x2746
        Located{pc:0x2548,instruction:JumpShort{target:0x2554}}, // N16, file 0x2748
        Located{pc:0x254a,instruction:StoreByteGp{source:r(0),address:0x2c129}}, // N32, file 0x274a
        Located{pc:0x254e,instruction:JumpShort{target:0x2554}}, // N16, file 0x274e
        Located{pc:0x2550,instruction:StoreWordGp{source:r(0),address:0x2c4c4}}, // N32, file 0x2750
        Located{pc:0x2554,instruction:AddSmall{reg:r(1),value:1,subtract:false}}, // N16, file 0x2754
        Located{pc:0x2556,instruction:BranchImmediate{reg:r(1),value:10,nonzero:true,target:0x2508}}, // N32, file 0x2756
        Located{pc:0x255a,instruction:Stack{pop:true,last:r(8),extra:0}}, // N16, file 0x275a
    ]
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn jump_table_is_data_and_case_instructions_are_adjacent(){
        let p=instructions(Profile::Postmarketos);
        assert_eq!(p.len(),20); // one data table, nineteen real instructions
        let mut f=FILE_START;
        for insn in p {
            assert_eq!(insn.pc as usize+0x200,f);
            f+=insn.instruction.encode(insn.pc,Profile::Postmarketos.gp()).unwrap().len();
        }
        assert_eq!(f,FILE_END);
        assert!(instructions(Profile::Map220v).is_empty());
    }
}
