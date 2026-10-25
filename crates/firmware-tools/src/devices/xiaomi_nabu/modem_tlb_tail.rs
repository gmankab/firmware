// SPDX-License-Identifier: GPL-3.0-only
//! The final Hexagon TLB and cache-maintenance paths at the initial boot.
//!
//! This is a **disconnected** verified code island at file `0x3584–0x35c4`,
//! not the missing instruction gap `0x3500–0x3584`. All instructions have
//! explicit Hexagon packet markers, architectural operands and no raw OEM
//! words. The physical address is file offset - 0x3000 + 0x8e000000.
use super::modem_hexagon::{encode,Operation,SpecialReg,CacheCommand,PacketMarker};

pub const FILE_START:usize=0x3584;
pub const CODE_LENGTH:usize=0x40;
pub const PHYSICAL_ADDRESS:u32=0x8e00_0584;

pub fn operations()->Vec<(Operation,PacketMarker)> {
    use Operation::*;
    use PacketMarker::*;
    vec![
        (WriteSystem{dst:SpecialReg::Syscfg,src:15},Continue),
        (ClearBit{dst:15,src:0,index:1},End),
        (InstructionSync,End),
        (Cache(CacheCommand::InstructionKill),End),
        (InstructionSync,End),
        (WriteSystem{dst:SpecialReg::Syscfg,src:15},End),
        (InstructionSync,End),
        (JumpRegister{src:14},End),
        (WriteTlb{pair_low:10,index:2},End),
        (InstructionSync,End),
        (JumpRegister{src:13},End),
        (WriteTlb{pair_low:8,index:3},End),
        (InstructionSync,End),
        (WriteSystem{dst:SpecialReg::Syscfg,src:0},End),
        (InstructionSync,End),
        (JumpRegister{src:31},End),
    ]
}

pub fn rebuild()->Result<Vec<u8>,String>{
    let mut image=Vec::with_capacity(CODE_LENGTH);
    for (op,marker) in operations(){image.extend_from_slice(&encode(op,marker)?.to_le_bytes());}
    if image.len()!=CODE_LENGTH {return Err("TLB bootstrap length mismatch".into());}
    Ok(image)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    let src=reference.get(FILE_START..FILE_START+CODE_LENGTH)
        .ok_or_else(||"truncated modem TLB routine reference".to_owned())?;
    let emitted=rebuild()?;
    if src!=emitted {
        let i=src.iter().zip(&emitted).position(|(a,b)|a!=b).unwrap();
        return Err(format!("TLB routine mismatch at file {:#x}: expected {:#04x}, got {:#04x}",
                           FILE_START+i,src[i],emitted[i]));
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn tlb_flush_and_exit_paths_are_exactly_sixteen_words(){
        let words=rebuild().unwrap();
        assert_eq!(words.len(),CODE_LENGTH);
        assert_eq!(operations().len(),16);
        assert!(matches!(operations()[0].1,PacketMarker::Continue));
        assert!(matches!(operations()[1].1,PacketMarker::End));
        assert_eq!(FILE_START+CODE_LENGTH,0x35c4);
    }
    #[test] fn fragment_rejects_missing_reference(){assert!(verify(&[]).is_err());}
}
