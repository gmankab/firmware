// SPDX-License-Identifier: GPL-3.0-only
//! Prefix of a Hexagon V66 cache-prefetch / zero / doubleword memcpy loop.
//! Its two hardware loops and associated endloop markers are represented as
//! checked typed operations. Real execution is not tested on device.
//! This module emits typed instructions; it does not paste reference bytes.
use super::cdsp_boot::{self,GeneralRegister as Gpr,ScalarPacket as H,PacketMarker as P};
pub const FILE_OFFSET:usize=0x3eb8;
pub const PHYSICAL_ADDRESS:u32=0x96c0_0eb8;
pub const PREFIX_BYTES:usize=32;
pub const BODY_BYTES:usize=76;
pub const TOTAL_BYTES:usize=PREFIX_BYTES+BODY_BYTES;

fn push(out:&mut Vec<u8>,operation:H,parse:P)->Result<(),String>{
    out.extend_from_slice(&parse.apply(operation.encode()?).to_le_bytes());Ok(())
}

/// Check buffer length and global availability. The `GP+##` data pointer
/// remains a typed relocation-like input rather than literal opcode words.
pub fn build_prefix()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(PREFIX_BYTES);
    push(&mut out,H::SkipIfSourceZeroNotTaken{src:r(2)?,displacement_bytes:104},P::ContinuationA)?;
    for x in cdsp_boot::encode_extended_gp_word_load(r(0)?,0xc067_3260) {
        out.extend_from_slice(&x.to_le_bytes());
    }
    push(&mut out,H::PredicateP0R2GreaterThan32,P::ContinuationA)?;
    push(&mut out,H::JumpIfNotNewP0NotTaken{displacement_bytes:92},P::Final)?;
    push(&mut out,H::SkipIfSourceZeroNotTaken{src:r(0)?,displacement_bytes:84},P::ContinuationA)?;
    for word in cdsp_boot::encode_extended_assignment(r(3)?,128,true) {
        out.extend_from_slice(&word.to_le_bytes());
    }
    if out.len()!=PREFIX_BYTES {return Err("CDSP cache-copy entry prefix must be 32 bytes".into());}
    Ok(out)
}

/// Cache the next buffer line while copying two packed register pairs per
/// body iteration, with loop0 inside loop1 and a final endloop01 packet.
/// The special 0x8000 parse marker is the Hexagon hardware-loop end marker,
/// not a normal packet-terminating instruction.
pub fn build_body()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(BODY_BYTES);
    for (op,parse) in [
        (H::LogicalShiftRight{dst:r(6)?,src:r(2)?,amount:12},P::ContinuationA),
        (H::AddSmallImmediate{dst:r(4)?,src:r(0)?,value:8},P::ContinuationA),
        (H::AddSmallImmediate{dst:r(5)?,src:r(1)?,value:8},P::Final),
        (H::HardwareLoop{loop_index:1,counter:r(6)?,displacement_bytes:12},P::ContinuationA),
    ]{push(&mut out,op,parse)?;}
    for word in cdsp_boot::encode_extended_assignment(r(7)?,0x0080_8020,true) {
        out.extend_from_slice(&word.to_le_bytes());
    }
    for (op,parse) in [
        (H::L2Fetch{address:r(1)?,control:r(7)?},P::Final),
        (H::HardwareLoop{loop_index:0,counter:r(3)?,displacement_bytes:4},P::Final),
        (H::CacheZeroLine{address:r(1)?},P::Final),
        (H::LoadDoublePostIncrement{low:r(6)?,base:r(0)?,increment:16},P::ContinuationA),
        (H::LoadDoublePostIncrement{low:r(8)?,base:r(4)?,increment:16},P::Final),
        (H::MemDoubleStorePostIncrement{low:r(6)?,base:r(1)?,increment:16},P::ContinuationA),
        (H::MemDoubleStorePostIncrement{low:r(8)?,base:r(5)?,increment:16},P::Final),
        (H::LoadDoublePostIncrement{low:r(6)?,base:r(0)?,increment:16},P::ContinuationA),
        (H::LoadDoublePostIncrement{low:r(8)?,base:r(4)?,increment:16},P::Final),
        (H::Nop,P::LoopEnd),
        (H::MemDoubleStorePostIncrement{low:r(6)?,base:r(1)?,increment:16},P::LoopEnd),
        (H::MemDoubleStorePostIncrement{low:r(8)?,base:r(5)?,increment:16},P::Final),
        (H::JumpRegister{source:r(31)?},P::Final),
    ]{push(&mut out,op,parse)?;}
    if out.len()!=BODY_BYTES{return Err("CDSP copy-fast body length mismatch".into());}
    Ok(out)
}

pub fn build_complete() -> Result<Vec<u8>,String>{
    let mut out=build_prefix()?;
    out.extend(build_body()?);
    if out.len()!=TOTAL_BYTES{return Err("CDSP fast-copy function not contiguous".into());}
    Ok(out)
}

pub fn build_cluster_with_copyfast() -> Result<Vec<u8>,String>{
    use super::cdsp_tlb_extension as tlb;
    if tlb::FILE_OFFSET_TABLES+tlb::TABLE_WALK_BYTES!=FILE_OFFSET{
        return Err("gap between CDSP TLB tables and copyfast function".into());
    }
    let mut out=tlb::build_extended_cluster()?;
    out.extend(build_complete()?);
    if out.len()!=376+TOTAL_BYTES{return Err("unexpected CDSP cluster-and-copy length".into());}
    Ok(out)
}

pub fn verify_complete(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let b=build_complete()?;
    let source=reference.get(FILE_OFFSET..FILE_OFFSET+TOTAL_BYTES)
        .ok_or("CDSP missing fast-copy reference bytes")?;
    if let Some((i,(&want,&got)))=source.iter().zip(&b).enumerate().find(|(_,(x,y))|x!=y){
        return Err(format!("CDSP fast-copy reference mismatch at {:#x}: {want:02x}!={got:02x}",FILE_OFFSET+i));
    }
    Ok(())
}

pub fn verify_full_cluster(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let bytes=build_cluster_with_copyfast()?;
    let start=super::cdsp_tlb::COMPLETE_CLUSTER_OFFSET;
    let source=reference.get(start..start+bytes.len()).ok_or("CDSP incomplete cluster reference")?;
    if source!=bytes{return Err("CDSP 484-byte supervisor/TLB/cache span differs".into());}
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let built=build_prefix()?;
    let ref_bytes=reference.get(FILE_OFFSET..FILE_OFFSET+PREFIX_BYTES)
        .ok_or("CDSP reference cache-copy prefix missing")?;
    if let Some((i,(&want,&got)))=ref_bytes.iter().zip(&built).enumerate().find(|(_,(a,b))|a!=b) {
        return Err(format!("CDSP cache-copy prefix differs at file {:#x}: {want:02x}!={got:02x}",FILE_OFFSET+i));
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn complete_cache_copy_has_two_hardware_loops_and_explicit_endloop() {
        let body=build_body().unwrap();
        assert_eq!(body.len(),BODY_BYTES);
        let words:Vec<u32>=body.chunks_exact(4)
            .map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();
        assert_eq!(words[3]&0xffff_0000,0x6026_0000);
        assert_eq!(words[7]&0xffff_0000,0x6003_0000);
        assert_eq!(words[15]&0xc000,0x8000);
        assert_eq!(words[16]&0xc000,0x8000);
        assert_eq!(build_complete().unwrap().len(),108);
        assert_eq!(build_cluster_with_copyfast().unwrap().len(),484);
    }
    #[test]fn packed_cache_copy_entry_has_no_implicit_filler(){
        let got=build_prefix().unwrap();assert_eq!(got.len(),32);
        let words:Vec<u32>=got.chunks_exact(4)
            .map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();
        assert_eq!(words[0]&0xc000,0x4000);
        assert_eq!(words[7]&0xc000,0xc000);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
}
