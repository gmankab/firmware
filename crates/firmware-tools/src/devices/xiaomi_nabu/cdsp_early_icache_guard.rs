// SPDX-License-Identifier: GPL-3.0-only
//! CDSP I-cache tag inspection revision gate, built from checked Hexagon V66
//! scalar fields and explicitly tagged packet boundaries. The following
//! hardware-loop function body starts at 0x5298; it is not included here.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::{cdsp_early_dcache_tags as previous,cdsp_early_worker_command as op};

pub const FILE_OFFSET:usize=0x5268;
pub const REGION_END:usize=0x5298;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2268;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
fn put(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}
fn read_revision(dst:Gpr)->Result<u32,String>{
    if dst.number()>7 {return Err("verified REV gate only accepts low GPR destinations".into());}
    Ok(0x6e9d_c000|u32::from(dst.number()))
}
fn mask_revision_low_byte(dst:Gpr,src:Gpr)->Result<u32,String>{
    if dst.number()>7||src.number()>7{return Err("revision byte mask only validated for low GPRs".into());}
    Ok(0x7600_c000|(u32::from(src.number())<<16)|(255<<5)|u32::from(dst.number()))
}
fn compare_greater_p0(src:Gpr,threshold:u8)->Result<u32,String>{
    if src.number()>7||threshold>127{return Err("I-cache revision predicate immediate/register out of range".into());}
    Ok(0x7540_c000|(u32::from(src.number())<<16)|(u32::from(threshold)<<5))
}
fn branch_return_if_invalid(source:Gpr)->Result<u32,String>{
    if source.number()!=31 {return Err("revision guard must return through LR".into());}
    Ok(0x5360_c800|(u32::from(source.number())<<16))
}
fn absolute_word_load(dst:Gpr,assign:Gpr,address:u32)->Result<[u32;2],String>{
    if dst.number()>7||assign.number()>7||address%4!=0 {
        return Err("absolute I-cache gate load needs low registers and aligned address".into());
    }
    Ok([((address>>20)<<16)|((address>>6)&0x3fff)|0x4000,
        0x9b80_d000|(u32::from(assign.number())<<16)|(((address&0x3f)>>2)<<8)|u32::from(dst.number())])
}
fn compact_skip_if_set(src:Gpr)->Result<u32,String>{
    if src.number()>7{return Err("compact I-cache gate predicate supports only tested register bank".into());}
    Ok(0x5900_3fc7|(u32::from(src.number())<<20))
}
fn store_flag(src:Gpr,base:Gpr)->Result<u32,String>{
    if src.number()>7||base.number()>7{return Err("I-cache state store requires low GPRs".into());}
    Ok(0xa180_c000|(u32::from(base.number())<<16)|(u32::from(src.number())<<8))
}
fn allocate_context(bytes:u16)->Result<u32,String>{
    if bytes==0 || bytes>0x100 || bytes%8!=0{return Err("I-cache context allocframe length invalid".into());}
    Ok(0xa09d_c000|(u32::from(bytes)>>3))
}

pub fn build_icache_guard()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,read_revision(r(0)?)?,P::Final);
    put(&mut out,mask_revision_low_byte(r(0)?,r(0)?)?,P::Final);
    put(&mut out,compare_greater_p0(r(0)?,0x55)?,P::ContinuationA);
    put(&mut out,branch_return_if_invalid(r(31)?)?,P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:1}.encode()?,P::ContinuationA);
    let state=absolute_word_load(r(2)?,r(1)?,0xfe01_6e20)?;
    put(&mut out,state[0],P::ContinuationA);
    put(&mut out,state[1],P::Final);
    out.extend_from_slice(&compact_skip_if_set(r(2)?)?.to_le_bytes());
    put(&mut out,store_flag(r(0)?,r(1)?)?,P::Final);
    put(&mut out,allocate_context(16)?,P::Final);
    put(&mut out,op::direct_call(0x96c0_2290,0x96c0_3388)?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0x108}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err(format!("I-cache revision guard emitted {} bytes, expected {}",out.len(),FUNCTION_BYTES));}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("I-cache guard is not adjacent to D-cache tag routine".into());}
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_icache_guard()?.as_slice()){
        return Err("I-cache revision gate does not match pinned CDSP".into());
    }
    Ok(())
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn icache_revision_and_context_are_bounded(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_icache_guard().unwrap().len(),48);
        assert_eq!(read_revision(r(0)).unwrap(),0x6e9d_c000);
        assert!(read_revision(r(8)).is_err());
        assert_eq!(mask_revision_low_byte(r(0),r(0)).unwrap(),0x7600_dfe0);
        assert_eq!(compare_greater_p0(r(0),0x55).unwrap(),0x7540_caa0);
        assert_eq!(branch_return_if_invalid(r(31)).unwrap(),0x537f_c800);
        assert!(branch_return_if_invalid(r(0)).is_err());
        assert_eq!(absolute_word_load(r(2),r(1),0xfe01_6e20).unwrap(),[0x0fe0_45b8,0x9b81_d802]);
        assert!(absolute_word_load(r(2),r(1),0xfe01_6e21).is_err());
        assert_eq!(compact_skip_if_set(r(2)).unwrap(),0x5920_3fc7);
        assert_eq!(store_flag(r(0),r(1)).unwrap(),0xa181_c000);
        assert_eq!(allocate_context(16).unwrap(),0xa09d_c002);
        assert!(allocate_context(11).is_err());
    }
}
