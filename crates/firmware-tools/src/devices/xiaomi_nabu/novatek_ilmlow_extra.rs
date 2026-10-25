// SPDX-License-Identifier: GPL-3.0-only
//! Continuation of lower ILM after complete startup at file 0xb1a.
//! Cross-profile code layouts diverge; all PCs are runtime=file-0x200.
use super::{Located,Profile};
use super::isa::{Insn,Reg,LP,R0};

/// Set the controller's active-operation flag to one and return.
/// The global field is at 0x2c584 (postmarketOS) or 0x2c55c (map220v).
pub fn instructions(profile:Profile)->Vec<Located>{
    use Insn::*;
    let (pc,flag)=match profile{
        Profile::Postmarketos=>(0x91a,0x2c584),
        Profile::Map220v=>(0x91c,0x2c55c),
    };
    let mut out=vec![
        Located{pc,instruction:MoveSmall{dest:R0,value:1}},
        Located{pc:pc+2,instruction:StoreWordGp{source:R0,address:flag}},
        Located{pc:pc+6,instruction:Return{reg:LP}},
    ];
    if profile==Profile::Postmarketos {
        // Only present at this location in the pinned postmarketOS image.
        // The older map220v build schedules the related state checks later.
        let checks=[
            (0x922,Stack{pop:false,last:Reg::new(6),extra:0}),
            (0x924,Ex9(171)), // GP controller word at 0x2c? from separate EX9 table
            (0x926,SubtractSmall3{dest:Reg::new(1),source:R0,value:1}),
            (0x928,LessThanSmallImmediate{reg:Reg::new(1),value:2}),
            (0x92a,BranchTa{nonzero:true,target:0x930}),
            (0x92c,BranchImmediate{reg:R0,value:4,nonzero:true,target:0x940}),
            (0x930,JumpRelative{target:0xab42,link:true}),
            (0x934,JumpRelative{target:0x15570,link:true}),
            (0x938,BranchZero{reg:R0,nonzero:false,target:0x940}),
            (0x93a,MoveSmall{dest:R0,value:0}),
            (0x93c,Ex9(46)), // separate GP instruction-table dependency
            (0x93e,Ex9(321)),
            (0x940,Stack{pop:true,last:Reg::new(6),extra:0}),
        ];
        out.extend(checks.into_iter().map(|(pc,instruction)|Located{pc,instruction}));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn setter_program_is_three_valid_instructions_on_both_profiles() {
        for (profile, start, global) in [
            (Profile::Postmarketos,0x91a,0x2c584),
            (Profile::Map220v,0x91c,0x2c55c),
        ] {
            let seq=instructions(profile);
            assert!(seq.len()>=3);
            assert_eq!(seq.iter().take(3).map(|x|x.pc).collect::<Vec<_>>(),[start,start+2,start+6]);
            assert!(matches!(seq[1].instruction,Insn::StoreWordGp{address, ..} if address==global));
            let bytes:Vec<u8>=seq.iter().take(3).flat_map(|x|x.instruction.encode(x.pc,profile.gp()).unwrap()).collect();
            assert_eq!(bytes.len(),8);
            assert_eq!(&bytes[..2], &[0x84,0x01]);
            assert_eq!(&bytes[6..8], &[0xdd,0x9e]);
        }
    }
}
