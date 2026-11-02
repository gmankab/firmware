// SPDX-License-Identifier: GPL-3.0-only
//! Worker ownership decision after the atomic lock/HTID handoff.
//! Typed V66 encodings for a duplex stack reload, predicate checks, and
//! packet-relative branches. No OEM executable bytes are embedded.
use super::cdsp_boot::{GeneralRegister as Gpr, PacketMarker as P};
use super::cdsp_early_worker_lock;

pub const FILE_OFFSET: usize = 0x4e04;
pub const REGION_END: usize = 0x4e28;
pub const FUNCTION_BYTES: usize = REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS: u32 = 0x96c0_1e04;
fn put(output:&mut Vec<u8>,opcode:u32,marker:P){output.extend_from_slice(&marker.apply(opcode).to_le_bytes());}

/// One V66 duplex pair: `Rdst=Rsrc ; Reven+1:Reven=memd(SP+offset)`.
/// Only the independently LLVM-checked compact SP forms are supported.
fn duplex_assign_and_stack_reload(dst:Gpr,src:Gpr,even_low:Gpr,offset:u8)->Result<u32,String>{
    if !(8..=23).contains(&dst.number()) || src.number()>7 || even_low.number()>6
        || even_low.number()%2!=0 || offset>16 || offset%8!=0 {
        return Err("V66 duplex register/stack addressing beyond checked subset".into());
    }
    Ok(0x5000_3e00 | (u32::from(src.number())<<20)
       | (u32::from(dst.number()-8)<<16) | u32::from(even_low.number()/2)
       |u32::from(offset))
}
/// `Rd=memw(Rbase+#0)`; zero offset is the only used form.
fn load_word(dst:Gpr,base:Gpr)->u32{
    0x9180_c000|(u32::from(base.number())<<16)|u32::from(dst.number())
}
/// Hexagon signed 32-bit extract of a width-bit slice at bit offset.
fn signed_extract(dst:Gpr,src:Gpr,width:u8,offset:u8)->Result<u32,String>{
    if width==0 || width>31 || offset>31 || u16::from(width)+u16::from(offset)>32 {
        return Err("V66 signed extract width/bit offset outside 32-bit source".into());
    }
    Ok(0x8d80_c000|(u32::from(src.number())<<16)|(u32::from(width)<<8)
       |(u32::from(offset&15)<<5)|u32::from(dst.number())
       |(u32::from(offset&16)<<18))
}
fn predicate_testbit(pred:u8,src:Gpr,bit:u8)->Result<u32,String>{
    if pred>3||bit>31{return Err("V66 predicate bit test operands out of bounds".into());}
    Ok(0x8500_c000|(u32::from(src.number())<<16)|(u32::from(bit)<<8)|u32::from(pred))
}
fn predicate_equal_regs(pred:u8,left:Gpr,right:Gpr)->Result<u32,String>{
    if pred>3{return Err("V66 equality predicate out of bounds".into());}
    Ok(0xf200_c000|(u32::from(left.number())<<16)|(u32::from(right.number())<<8)|u32::from(pred))
}
/// Predicate combination, with the second source intentionally inverted.
fn predicate_and_not(dst:u8,left:u8,right:u8)->Result<u32,String>{
    if [dst,left,right].iter().any(|v|*v>3) {return Err("V66 predicate out of range".into());}
    Ok(0x6b60_c000|(u32::from(right)<<16)|(u32::from(left)<<8)|u32::from(dst))
}
fn halfword_forward(packet:u32,target:u32,max:u32)->Result<u32,String>{
    if packet%4!=0||target%4!=0||target<=packet||target-packet>max {
        return Err("Hexagon relative forward branch exceeds checked range".into());
    }
    Ok((target-packet)/2)
}
fn branch_new_predicate_not_taken(pred:u8,packet:u32,target:u32)->Result<u32,String>{
    if pred>3 {return Err("V66 new-predicate index out of range".into());}
    let half=halfword_forward(packet,target,0x3fe)?;
    Ok(0x5c00_c800|(u32::from(pred)<<8)|((half&0x100)<<5)|(half&0xff))
}
fn branch_predicate_not_taken(pred:u8,packet:u32,target:u32)->Result<u32,String>{
    if pred>3{return Err("V66 ordinary predicate out of range".into());}
    let half=halfword_forward(packet,target,0x1fe)?;
    Ok(0x5c00_c000|(u32::from(pred)<<8)|(half&0xff))
}
/// Compound P0=(R1==0); if(!P0.new) jump:t to target.
fn inverted_zero_compare_branch(src:Gpr,packet:u32,target:u32)->Result<u32,String>{
    if src.number()>7{return Err("V66 compact cmp source out of range".into());}
    let half=halfword_forward(packet,target,0x1fe)?;
    Ok(0x1040_e000|(u32::from(src.number())<<16)|(half&0xff))
}

pub fn build_worker_decision()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,duplex_assign_and_stack_reload(r(23)?,r(0)?,r(0)?,0)?,P::ContinuationB);
    put(&mut out,load_word(r(18)?,r(0)?),P::Final);
    put(&mut out,signed_extract(r(19)?,r(18)?,3,0)?,P::ContinuationA);
    put(&mut out,predicate_testbit(0,r(18)?,28)?,P::Final);
    put(&mut out,predicate_equal_regs(1,r(19)?,r(1)?)?,P::Final);
    put(&mut out,predicate_and_not(2,0,1)?,P::ContinuationA);
    put(&mut out,branch_new_predicate_not_taken(2,0x96c0_1e18,0x96c0_2004)?,P::Final);
    put(&mut out,branch_predicate_not_taken(1,0x96c0_1e20,0x96c0_1e28)?,P::Final);
    put(&mut out,inverted_zero_compare_branch(r(1)?,0x96c0_1e24,0x96c0_2004)?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("worker decision byte count not 36".into());}
    Ok(out)
}
pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_worker_lock::verify_reference(original)?;
    if cdsp_early_worker_lock::REGION_END!=FILE_OFFSET {
        return Err("CDSP worker decision must follow original lock packet".into());
    }
    let expected=original.get(FILE_OFFSET..REGION_END).ok_or("short CDSP reference")?;
    let generated=build_worker_decision()?;
    if expected!=generated {return Err("source-built CDSP worker decision differs from pinned reference".into());}
    Ok(())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn worker_predicates_duplex_and_branches_checked(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_worker_decision().unwrap().len(),36);
        assert_eq!(duplex_assign_and_stack_reload(r(23),r(0),r(0),0).unwrap(),0x500f_3e00);
        assert_eq!(duplex_assign_and_stack_reload(r(22),r(0),r(2),8).unwrap(),0x500e_3e09);
        assert!(duplex_assign_and_stack_reload(r(23),r(0),r(1),0).is_err());
        assert!(duplex_assign_and_stack_reload(r(23),r(0),r(0),24).is_err());
        assert_eq!(signed_extract(r(19),r(18),3,0).unwrap(),0x8d92_c313);
        assert!(signed_extract(r(19),r(18),31,6).is_err());
        assert_eq!(predicate_and_not(2,0,1).unwrap(),0x6b61_c002);
        assert!(predicate_and_not(4,0,1).is_err());
        assert_eq!(branch_new_predicate_not_taken(2,0x96c0_1e18,0x96c0_2004).unwrap(),0x5c00_caf6);
        assert_eq!(branch_predicate_not_taken(1,0x96c0_1e20,0x96c0_1e28).unwrap(),0x5c00_c104);
        assert_eq!(inverted_zero_compare_branch(r(1),0x96c0_1e24,0x96c0_2004).unwrap(),0x1041_e0f0);
        assert!(branch_new_predicate_not_taken(2,0x96c0_1e18,0x96c0_3004).is_err());
    }
}
