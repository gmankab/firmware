// SPDX-License-Identifier: GPL-3.0-only
//! Agent 17 lower ILM reconstruction. Build functions never read firmware.
//! Runtime = file offset - 0x200. GP targets are absolute SRAM addresses.
//! EX9 table source remains owned by Agent 18 and is an explicit dependency.
#[path="novatek_ilmlow_isa.rs"]
pub mod isa;
#[path="novatek_ilmlow_helpers.rs"]
mod helpers;
use isa::{Csr,Insn,Reg,R0,R1,R2};
pub const FILE_START: usize = 0x66a;
pub const FILE_END: usize = 0x79c;
pub const ILM_FILE_BASE: usize = 0x200;
pub const ASSIGNED_END: usize = 0x10000;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Profile { Postmarketos, Map220v }
impl Profile {
    pub fn parse(s:&str)->Result<Self,String>{match s {"postmarketos"=>Ok(Self::Postmarketos),"map220v"=>Ok(Self::Map220v),_=>Err("profile must be postmarketos or map220v".into())}}
    pub fn name(self)->&'static str{match self{Self::Postmarketos=>"postmarketos",Self::Map220v=>"map220v"}}
    pub fn gp(self)->u32{match self{Self::Postmarketos=>0x2d9e0,Self::Map220v=>0x2d7f0}}
    pub fn itb(self)->u32{match self{Self::Postmarketos=>0x17bc0,Self::Map220v=>0x17010}}
    pub fn reference_sha256(self)->&'static str{match self{
        Self::Postmarketos=>"8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5",
        Self::Map220v=>"72364eca0a91855d59e043371aca774354e5ad0885df5024edcec8f976543ad2"}}
}

#[derive(Clone,Copy,Debug)]
pub struct Located { pub pc:u32, pub instruction:Insn }
pub fn program(profile:Profile)->Vec<Located>{
    use Insn::*;
    let (flag,mask2,mask3,psw_enable)=match profile{
        Profile::Postmarketos=>(0x2c412,0x7d,0x162,0x6a),
        Profile::Map220v=>(0x2c3f7,0x7a,0x16d,0x43),
    };
    // Complete the reset routine begun by Agent09 at file 0x5de. R0 is
    // initially zero, established by that routine's entry. EX9 entries are
    // identified by effects below, with per-profile table indices.
    let source=[
        (0x46a,StoreByteGp{source:R0,address:flag}),
        (0x46e,WriteSystem{source:R0,csr:Csr::IntMask}),
        (0x472,Ex9(mask2)), // MTSR R0, INT_MASK2
        (0x474,Ex9(mask3)), // MTSR R0, INT_MASK3
        (0x476,ReadSystem{dest:R1,csr:Csr::Ivb}),
        (0x47a,LowBit{dest:R1,source:R1}),
        (0x47c,Move{dest:R2,source:R0}),
        (0x47e,BranchZero{reg:R1,nonzero:false,target:0x484}),
        (0x480,WriteSystem{source:R0,csr:Csr::IntMask}),
        (0x484,WriteSystem{source:R2,csr:Csr::Ivb}),
        (0x488,ReadSystem{dest:R0,csr:Csr::Psw}),
        (0x48c,BitClearImmediate{dest:R0,source:R0,mask:6}),
        (0x490,WriteSystem{source:R0,csr:Csr::Psw}),
        (0x494,DataSync),
        (0x498,ReadSystem{dest:R0,csr:Csr::IntMask2}),
        (0x49c,SetHigh{dest:R0,value:0x29c10}),
        (0x4a0,AddImmediate{dest:R0,source:R0,value:0x7c}),
        (0x4a4,Ex9(mask2)), // INT_MASK2 = 0x29c1007c
        (0x4a6,ReadSystem{dest:R0,csr:Csr::IntMask3}),
        (0x4aa,MoveImmediate{dest:R0,value:0x633}),
        (0x4ae,Ex9(mask3)), // INT_MASK3 = 0x633
        (0x4b0,ReadSystem{dest:R0,csr:Csr::Psw}),
        (0x4b4,Ex9(psw_enable)), // ORI R0,R0,1
        (0x4b6,WriteSystem{source:R0,csr:Csr::Psw}),
        (0x4ba,Stack{pop:true,last:Reg::new(6),extra:0}),
    ];
    let mut instructions:Vec<_>=source.into_iter().map(|(pc,instruction)|Located{pc,instruction}).collect();
    instructions.extend(helpers::instructions(profile));
    instructions
}
pub fn build(profile:Profile)->Result<Vec<u8>,String>{
    let mut output=Vec::new();
    for instruction in program(profile){
        let expected=FILE_START-ILM_FILE_BASE+output.len();
        if instruction.pc as usize!=expected{return Err(format!("source gap/overlap at runtime 0x{expected:x}"));}
        output.extend(instruction.instruction.encode(instruction.pc,profile.gp())?);
    }
    if FILE_START+output.len()!=FILE_END || FILE_END>ASSIGNED_END{return Err("invalid source interval".into());}
    Ok(output)
}
pub fn compare(firmware:&[u8],profile:Profile)->Result<(),String>{
    let expected=firmware.get(FILE_START..FILE_END).ok_or("truncated Novatek reference")?;
    let actual=build(profile)?;
    if let Some(i)=actual.iter().zip(expected).position(|(a,b)|a!=b){return Err(format!("byte mismatch at file 0x{:x}",FILE_START+i));}
    Ok(())
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn source_intervals_and_profile_selection(){
        let p=build(Profile::Postmarketos).unwrap();let m=build(Profile::Map220v).unwrap();
        assert_eq!(p.len(),306);assert_eq!(m.len(),306);assert_ne!(p,m);
        assert!(Profile::parse("unknown").is_err());
        let mut f=vec![0;FILE_END];f[FILE_START..FILE_END].copy_from_slice(&p);
        assert!(compare(&f,Profile::Postmarketos).is_ok());
        assert!(compare(&f,Profile::Map220v).is_err());
        f[FILE_START+20]^=1;assert!(compare(&f,Profile::Postmarketos).is_err());
        assert!(compare(&f[..FILE_END-1],Profile::Postmarketos).is_err());
    }
}
