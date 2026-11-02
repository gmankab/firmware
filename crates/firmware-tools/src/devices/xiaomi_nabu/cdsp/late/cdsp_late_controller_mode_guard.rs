// SPDX-License-Identifier: GPL-3.0-only
//! CDSP controller secondary-plane bit-size guards at file 0x56784..0x567b4.
//! The two packet-relative, predicated branches are narrow checked encodings
//! cross-checked with Rizin V66. The consumer of the second branch's R2.new
//! is not a standalone routine; the enclosing function continues at 0x567b4.
use super::cdsp_boot::{self,GeneralRegister as G,ScalarPacket as S,PacketMarker as M};
use super::cdsp_late_controller_mask_setup;

pub const FILE_OFFSET:usize=0x56784;
pub const END_FILE_OFFSET:usize=0x567b4;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8784;
pub const RWE_START:usize=cdsp_late_controller_mask_setup::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_mask_setup::RWE_PREFIX_LAYOUT_LEN;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_mask_setup::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;

fn r(n:u8)->Result<G,String>{G::new(n)}
fn put(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}

/// Compound `P0=cmp.eq(R23,0); if(P0.new) jump:nt` to the shared plane exit.
pub fn encode_guard_if_empty(src:G,from:u32,target:u32)->Result<u32,String>{
    if (src.number(),from,target)!=(23,0x96cc_8784,0x96cc_8810){
        return Err("controller empty-plane predicate/branch is not verified".into());
    }
    let delta=target.checked_sub(from).ok_or("plane exit behind condition")?;
    if delta%2!=0 || delta>0x1fe{return Err("invalid empty-plane skip displacement".into());}
    // The original V66 packet's source tuple fixes selector 0x100f; low byte
    // is the halfword branch offset, not a copied firmware word.
    Ok(0x100f_c000 | ((delta>>1)&0xff))
}

/// Early upper bound `if(cmp.gtu(R4.new,R3)) jump:nt`. The branch source
/// is the packet's *first* word, not the comparison's own word.
pub fn encode_guard_if_too_many(dst:G,src:G,packet:u32,target:u32)->Result<u32,String>{
    if (dst.number(),src.number(),packet,target)!=(4,3,0x96cc_8788,0x96cc_8810){
        return Err("controller R4.new guard uses unsupported operands/target".into());
    }
    let delta=target.checked_sub(packet).ok_or("unexpected bound exit target")?;
    if delta%2!=0 || delta>0x1fe{return Err("bound branch offset outside packet range".into());}
    Ok(0x2102_c300 | (delta>>1))
}

/// Signed short source register `R3=cl0(R23)`, established by independent
/// scalar assembly/decoder fields for an earlier controller loop.
pub fn encode_count_plane_bits(dst:G,src:G)->Result<u32,String>{
    if (dst.number(),src.number())!=(3,23){return Err("expected CL0(R23) -> R3".into());}
    Ok(0x8c00_40a0 | (u32::from(src.number())<<16) | u32::from(dst.number()))
}

/// Reject too-small plane capacity when R25 does not exceed the newest R2.
/// The branch is measured from the R2-producing packet and includes the
/// independently observed 4-byte Hexagon packet-relative adjustment.
pub fn encode_insufficient_plane_branch(plane:G,packet:u32,target:u32)->Result<u32,String>{
    if (plane.number(),packet,target)!=(25,0x96cc_87ac,0x96cc_88ac){
        return Err("secondary plane uses unsupported bit capacity branch".into());
    }
    let distance=target.checked_sub(packet).ok_or("capacity exit behind current packet")?;
    if distance%2!=0||distance>0x3fe {return Err("invalid secondary capacity distance".into());}
    Ok(0x2242_d800 | (u32::from(plane.number()-24)<<8) | ((distance+4)>>1))
}

pub fn build_mode_guard()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    put(&mut out,encode_guard_if_empty(r(23)?,PHYSICAL_ADDRESS,0x96cc_8810)?);
    put(&mut out,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(4)?,value:33}.encode()?));
    put(&mut out,encode_guard_if_too_many(r(4)?,r(3)?,PHYSICAL_ADDRESS+4,0x96cc_8810)?);
    put(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(24)?,src:r(22)?,amount:12}.encode()?));
    const SMALL_BANK_MASK:u32=0x000f_ff00;
    put(&mut out,cdsp_boot::encode_extended_assignment(r(19)?,SMALL_BANK_MASK as i32,false)[0]);
    put(&mut out,M::ContinuationA.apply(0x7600_c000 | (u32::from(r(19)?.number())<<16) |
                                    u32::from(r(19)?.number())));
    put(&mut out,S::AssignUnsignedImmediate{dst:r(17)?,value:32}.encode()?);
    put(&mut out,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(25)?,value:8}.encode()?));
    put(&mut out,S::AssignUnsignedImmediate{dst:r(26)?,value:1}.encode()?);
    put(&mut out,encode_count_plane_bits(r(3)?,r(23)?)?);
    put(&mut out,M::ContinuationA.apply(S::AddSignedSmallImmediate{dst:r(2)?,src:r(17)?,value:-31}.encode()?));
    put(&mut out,encode_insufficient_plane_branch(r(25)?,PHYSICAL_ADDRESS+0x28,0x96cc_88ac)?);
    if out.len()!=EXECUTABLE_LEN{return Err("controller mode guard must be 12 words".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_mask_setup::build_prefix()?;
    out.extend(build_mode_guard()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN || out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("controller mask-mode prefix has inconsistent code/fill accounting".into());
    }
    Ok(out)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_mode_guard()?.as_slice()){
        return Err("controller mask-mode guards differ from reference".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("controller mask-mode prefix differs from reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn secondary_mode_guards_are_checked(){
        let b=build_mode_guard().unwrap();assert_eq!(b.len(),48);
        let w:Vec<u32>=b.chunks_exact(4).map(|q|u32::from_le_bytes(q.try_into().unwrap())).collect();
        assert_eq!(w[0],0x100f_c046);
        assert_eq!(w[2],0x2102_c344);
        assert_eq!(w[5],0x7613_4013);
        assert_eq!(w[9],0x8c17_40a3);
        assert_eq!(w[11],0x2242_d982);
        assert!(encode_guard_if_empty(r(17).unwrap(),PHYSICAL_ADDRESS,0x96cc_8810).is_err());
        assert!(encode_guard_if_too_many(r(4).unwrap(),r(3).unwrap(),0x96cc_8788,0x96cc_8820).is_err());
        assert!(encode_insufficient_plane_branch(r(24).unwrap(),0x96cc_87ac,0x96cc_88ac).is_err());
    }
    #[test]fn joined_prefix_has_separate_layout_fill(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,0x7b4);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1928);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,44);
        assert_eq!(build_prefix().unwrap().len(),FULL_RWE_LAYOUT_LEN);
    }
}
