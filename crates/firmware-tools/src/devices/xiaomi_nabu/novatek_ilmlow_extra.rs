// SPDX-License-Identifier: GPL-3.0-only
//! Continuation of lower ILM after complete startup at file 0xb1a.
//! Cross-profile code layouts diverge; all PCs are runtime=file-0x200.
use super::{Located,Profile};
use super::isa::{Insn,LP,R0};

/// Set the controller's active-operation flag to one and return.
/// The global field is at 0x2c584 (postmarketOS) or 0x2c55c (map220v).
pub fn instructions(profile:Profile)->Vec<Located>{
    use Insn::*;
    let (pc,flag)=match profile{
        Profile::Postmarketos=>(0x91a,0x2c584),
        Profile::Map220v=>(0x91c,0x2c55c),
    };
    vec![
        Located{pc,instruction:MoveSmall{dest:R0,value:1}},
        Located{pc:pc+2,instruction:StoreWordGp{source:R0,address:flag}},
        Located{pc:pc+6,instruction:Return{reg:LP}},
    ]
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
            assert_eq!(seq.len(),3);
            assert_eq!(seq.iter().map(|x|x.pc).collect::<Vec<_>>(),[start,start+2,start+6]);
            assert!(matches!(seq[1].instruction,Insn::StoreWordGp{address, ..} if address==global));
            let bytes:Vec<u8>=seq.iter().flat_map(|x|x.instruction.encode(x.pc,profile.gp()).unwrap()).collect();
            assert_eq!(bytes.len(),8);
            assert_eq!(&bytes[..2], &[0x84,0x01]);
            assert_eq!(&bytes[6..8], &[0xdd,0x9e]);
        }
    }
}
