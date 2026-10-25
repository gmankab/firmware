// SPDX-License-Identifier: GPL-3.0-only
//! First 80 bytes of the CDSP Hexagon V66 cache-controller setup.
//! The routine is incomplete: the cache-clean loop and return follow this
//! prefix. All words are assembled from typed operands, packet parse markers
//! and named offsets, not extracted reference opcode arrays.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P,
                      ScalarPacket as H, SystemRegister};

pub const FILE_OFFSET: usize=0x3f24;
pub const PHYSICAL_ADDRESS: u32=0x96c0_0f24;
pub const PREFIX_SIZE:usize=80;

fn put(out:&mut Vec<u8>, op:H, marker:P)->Result<(),String>{
    out.extend_from_slice(&marker.apply(op.encode()?).to_le_bytes());
    Ok(())
}

/// Read three physical configuration words using CFGBASE-scaled addressing;
/// each value is shifted into the appropriate device register field and
/// written through the controller's negative-offset register window.
pub fn build_prefix()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut bytes=Vec::with_capacity(PREFIX_SIZE);
    put(&mut bytes,H::ReadSystem{system:SystemRegister::Cfgbase,dst:r(21)?},P::Final)?;
    put(&mut bytes,H::ArithmeticShiftLeft{dst:r(21)?,src:r(21)?,amount:5},P::ContinuationA)?;
    for (address,shift,write_offset) in [(76u32,16u8,-0x1fe92a4i32),
                                                 (56,16,-0x1fe92a0),
                                                 (60,10,-0x1fe929c)]
                                              .into_iter(){
        // The original package explicitly uses immext even for small
        // constants; forcing the two-word form preserves relocation width.
        for word in cdsp_boot::encode_extended_assignment(r(22)?,address as i32,true) {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        put(&mut bytes,H::ReadPhysicalWord{dst:r(23)?,offset:r(22)?,base:r(21)?},P::Final)?;
        put(&mut bytes,H::ArithmeticShiftLeft{dst:r(23)?,src:r(23)?,amount:shift},P::ContinuationA)?;
        for word in cdsp_boot::encode_extended_new_value_store(r(25)?,write_offset)? {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
    }
    if bytes.len()!=PREFIX_SIZE {
        return Err(format!("CDSP CFGBASE setup length {} != {PREFIX_SIZE}",bytes.len()));
    }
    Ok(bytes)
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_prefix()?;
    let src=reference.get(FILE_OFFSET..FILE_OFFSET+PREFIX_SIZE)
        .ok_or("CDSP reference too short for CFGBASE setup")?;
    if let Some((i,(x,y)))=out.iter().zip(src).enumerate().find(|(_, (x,y))|x!=y) {
        return Err(format!("CDSP CFGBASE mismatch at file {:#x}, generated {x:02x}, reference {y:02x}",FILE_OFFSET+i));
    }
    Ok(())
}

pub fn build_joined_cache_controller_prefix()->Result<Vec<u8>,String>{
    let mut code=super::cdsp_copyfast::build_cluster_with_copyfast()?;
    if super::cdsp_copyfast::FILE_OFFSET + super::cdsp_copyfast::TOTAL_BYTES != FILE_OFFSET {
        return Err("gap before CDSP cache-controller setup".into());
    }
    code.extend(build_prefix()?);
    if code.len()!=484+PREFIX_SIZE { return Err("joined CFGBASE/fastcopy span length incorrect".into()); }
    Ok(code)
}

pub fn verify_joined_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_joined_cache_controller_prefix()?;
    let begin=super::cdsp_tlb::COMPLETE_CLUSTER_OFFSET;
    if reference.get(begin..begin+out.len())!=Some(out.as_slice()) {
        return Err("CDSP contiguous supervisor/TLB/cache/CFGBASE bytes differ".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test] fn typed_cache_controller_setup_has_explicit_packets(){
        assert_eq!(build_prefix().unwrap().len(),PREFIX_SIZE);
        assert_eq!(build_joined_cache_controller_prefix().unwrap().len(),564);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn new_value_store_checks_alignment_and_preserves_system_registers(){
        let r=|x|Gpr::new(x).unwrap();
        assert_eq!(H::ReadSystem{system:SystemRegister::Cfgbase,dst:r(21)}.encode().unwrap(),0x6e9b_c015);
        assert_eq!(H::ReadPhysicalWord{dst:r(23),offset:r(22),base:r(21)}.encode().unwrap(),0x9216_f517);
        assert_eq!(cdsp_boot::encode_extended_new_value_store(r(25),-0x1fe92a4).unwrap(),[0x0fe0_45b5,0xa1b9_d21c]);
        assert!(cdsp_boot::encode_extended_new_value_store(r(25),-3).is_err());
    }
}

/// Invalidate the full indexed cache window with nested hardware loops and
/// synchronize global caches before disabling SYSCFG bit two. This is the
/// exact successor to `build_prefix`, not an unrelated stand-alone routine.
pub const CACHE_MAINTENANCE_SIZE:usize=88;
pub fn build_cache_maintenance()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(CACHE_MAINTENANCE_SIZE);
    let operations:[(H,P);22]=[
        (H::AssignUnsignedImmediate{dst:r(22)?,value:0},P::Final),
        (H::HardwareLoopImmediate{loop_index:1,iterations:128,displacement_bytes:4},P::Final),
        (H::AssignRegister{dst:r(21)?,src:r(22)?},P::Final),
        (H::HardwareLoopImmediate{loop_index:0,iterations:8,displacement_bytes:4},P::Final),
        (H::AddSmallImmediate{dst:r(21)?,src:r(21)?,value:1},P::LoopEnd),
        (H::CleanInvalidateDcacheIndex{address:r(21)?},P::Final),
        (H::AddSmallImmediate{dst:r(22)?,src:r(22)?,value:32},P::ContinuationA),
        (H::Nop,P::LoopEnd),
        (H::Nop,P::Final),
        (H::Syncht,P::Final),
        (H::GlobalL2CleanInvalidate,P::Final),
        (H::Syncht,P::Final),
        (H::ReadSyscfg{dst:r(21)?},P::Final),
        (H::TestBitPredicate{predicate:1,src:r(21)?,bit:31},P::ContinuationA),
        (H::JumpIfNewP1TakenBackward{displacement_bytes:-4},P::Final),
        (H::Syncht,P::Final),
        (H::ClearBit{dst:r(21)?,src:r(21)?,position:2},P::Final),
        (H::WriteSystem{system:SystemRegister::Syscfg,src:r(21)?},P::Final),
        (H::Isync,P::Final),
        (H::Dckill,P::Final),
        (H::L2Kill,P::Final),
        (H::Syncht,P::Final),
    ];
    for (op,parse) in operations { put(&mut out,op,parse)?; }
    if out.len()!=CACHE_MAINTENANCE_SIZE {
        return Err("cache maintenance instructions not contiguous".into());
    }
    Ok(out)
}

pub fn build_complete()->Result<Vec<u8>,String>{
    let mut out=build_prefix()?;
    out.extend(build_cache_maintenance()?);
    assert_eq!(out.len(),PREFIX_SIZE+CACHE_MAINTENANCE_SIZE);
    Ok(out)
}

pub fn build_joined_cache_cluster()->Result<Vec<u8>,String>{
    let mut out=super::cdsp_copyfast::build_cluster_with_copyfast()?;
    if super::cdsp_copyfast::FILE_OFFSET+super::cdsp_copyfast::TOTAL_BYTES!=FILE_OFFSET {
        return Err("CDSP cache controller is not adjacent to cache-copy".into());
    }
    out.extend(build_complete()?);
    if out.len()!=484+PREFIX_SIZE+CACHE_MAINTENANCE_SIZE {
        return Err("CDSP cache-control cluster has an uncovered hole".into());
    }
    Ok(out)
}

pub fn verify_cache_maintenance(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_cache_maintenance()?;
    let start=FILE_OFFSET+PREFIX_SIZE;
    let original=reference.get(start..start+out.len()).ok_or("CDSP cache-maintenance reference truncated")?;
    if original!=out {return Err("CDSP cache-maintenance bytes not identical".into());}
    Ok(())
}

pub fn verify_joined_cache_cluster(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_joined_cache_cluster()?;
    let start=super::cdsp_tlb::COMPLETE_CLUSTER_OFFSET;
    if reference.get(start..start+out.len())!=Some(out.as_slice()) {
        return Err("CDSP supervisor/TLB/cache-controller cluster differs".into());
    }
    Ok(())
}

#[cfg(test)]mod cache_loop_tests{
    use super::*;
    #[test] fn cache_loop_has_explicit_endloop_and_predicate(){
        let r=|n|Gpr::new(n).unwrap();
        let out=build_cache_maintenance().unwrap();
        assert_eq!(out.len(),88);
        assert_eq!(u32::from_le_bytes(out[4..8].try_into().unwrap()),0x6924_c008);
        assert_eq!(u32::from_le_bytes(out[12..16].try_into().unwrap()),0x6900_c048);
        assert_eq!(u32::from_le_bytes(out[24..28].try_into().unwrap())&0xc000,0x4000);
        assert_eq!(u32::from_le_bytes(out[28..32].try_into().unwrap())&0xc000,0x8000);
        assert_eq!(u32::from_le_bytes(out[14*4..15*4].try_into().unwrap()),0x5cdf_f9fe);
        assert_eq!(H::TestBitPredicate{predicate:1,src:r(21),bit:31}.encode().unwrap(),0x8515_df01);
        assert!(H::TestBitPredicate{predicate:4,src:r(21),bit:31}.encode().is_err());
        assert!(H::HardwareLoopImmediate{loop_index:2,iterations:1,displacement_bytes:4}.encode().is_err());
        assert!(H::JumpIfNewP1TakenBackward{displacement_bytes:4}.encode().is_err());
        assert_eq!(build_joined_cache_cluster().unwrap().len(),652);
    }
}

/// Zero a contiguous controller table in 64-bit strides, decrementing the
/// word-count pair until its unsigned count reaches zero. LLVM6 independently
/// encodes all eight instruction words, including the conditional back edge.
pub const TABLE_CLEAR_SIZE:usize=32;
pub fn build_controller_table_clear()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(TABLE_CLEAR_SIZE);
    for word in cdsp_boot::encode_extended_word_load(r(21)?,r(25)?,-0x1fe92a0)? {
        out.extend_from_slice(&word.to_le_bytes());
    }
    for word in cdsp_boot::encode_extended_word_load(r(19)?,r(25)?,-0x1fe929c)? {
        out.extend_from_slice(&word.to_le_bytes());
    }
    put(&mut out,H::CombineZeroPair{low:r(22)?},P::Final)?;
    put(&mut out,H::MemDoubleStorePostIncrement{low:r(22)?,base:r(21)?,increment:8},P::Final)?;
    put(&mut out,H::AddSignedSmallImmediate{dst:r(19)?,src:r(19)?,value:-8},P::ContinuationA)?;
    // Back edge from the packet beginning at file 0x3fe4 to 0x3fe0: -4.
    put(&mut out,H::BranchIfNewGreaterThanZeroBackward{displacement_bytes:-4},P::Final)?;
    if out.len()!=TABLE_CLEAR_SIZE {return Err("CDSP controller table loop has invalid length".into());}
    Ok(out)
}

pub fn build_joined_cache_and_table_clear()->Result<Vec<u8>,String>{
    let mut out=build_joined_cache_cluster()?;
    if FILE_OFFSET+PREFIX_SIZE+CACHE_MAINTENANCE_SIZE!=0x3fcc {
        return Err("gap at CDSP cache-to-table transition".into());
    }
    out.extend(build_controller_table_clear()?);
    if out.len()!=684 {return Err("CDSP cache/controller cluster gap".into());}
    Ok(out)
}

pub fn verify_table_clear(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let src=reference.get(0x3fcc..0x3fec).ok_or("CDSP reference has no controller table-clear loop")?;
    if src!=build_controller_table_clear()?.as_slice(){
        return Err("CDSP controller table-clear loop differs from reference".into());
    }
    Ok(())
}

pub fn verify_joined_table_clear(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_joined_cache_and_table_clear()?;
    if reference.get(0x3d40..0x3d40+out.len())!=Some(out.as_slice()) {
        return Err("CDSP 684-byte supervisor/TLB/cache/table-clear region differs".into());
    }
    Ok(())
}

#[cfg(test)]mod controller_table_tests{
    use super::*;
    #[test]fn controller_table_uses_aligned_32_bit_offsets(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_controller_table_clear().unwrap().len(),32);
        assert_eq!(build_joined_cache_and_table_clear().unwrap().len(),684);
        assert!(cdsp_boot::encode_extended_word_load(r(21),r(25),-3).is_err());
        assert!(H::AddSignedSmallImmediate{dst:r(19),src:r(19),value:0}.encode().is_err());
        assert!(H::BranchIfNewGreaterThanZeroBackward{displacement_bytes:2}.encode().is_err());
        assert!(H::MemDoubleStorePostIncrement{low:r(21),base:r(21),increment:8}.encode().is_err());
    }
}

/// First descriptor of the controller's static four-entry configuration
/// table. The controller writes two class/header words, queries three
/// independent feature flags, and computes `memw(base+0x28)` from flags.
/// This is a partial initialization routine until the other entries follow.
pub const FIRST_DESCRIPTOR_BYTES:usize=108;
pub fn build_first_controller_descriptor()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FIRST_DESCRIPTOR_BYTES);
    for word in cdsp_boot::encode_extended_word_load(r(21)?,r(25)?,-0x1fe92a4)?{
        out.extend_from_slice(&word.to_le_bytes());
    }
    for (val,off) in [(0xaa,0x2c),(0xa,0x30)] {
        put(&mut out,H::AssignUnsignedImmediate{dst:r(23)?,value:val},P::ContinuationA)?;
        for word in cdsp_boot::encode_v66_new_word_store(r(21)?,off)? {
            out.extend_from_slice(&word.to_le_bytes());
        }
    }
    put(&mut out,H::AssignUnsignedImmediate{dst:r(23)?,value:5},P::ContinuationA)?;
    for word in cdsp_boot::encode_extended_word_load(r(22)?,r(25)?,-0x1fd92bc)? {
        out.extend_from_slice(&P::ContinuationA.apply(word).to_le_bytes());
    }
    put(&mut out,H::BranchIfNewLoadZeroForward{displacement_bytes:20},P::Final)?;
    put(&mut out,H::AssignUnsignedImmediate{dst:r(23)?,value:10},P::Final)?;
    for word in cdsp_boot::encode_v66_plain_word_store(r(21)?,r(23)?,0)?{
        out.extend_from_slice(&word.to_le_bytes());
    }
    put(&mut out,H::AssignUnsignedImmediate{dst:r(23)?,value:5},P::ContinuationA)?;
    for word in cdsp_boot::encode_extended_word_load(r(22)?,r(25)?,-0x1fd92b8)? {
        out.extend_from_slice(&P::ContinuationA.apply(word).to_le_bytes());
    }
    put(&mut out,H::BranchIfNewLoadZeroForward{displacement_bytes:20},P::Final)?;
    put(&mut out,H::AssignUnsignedImmediate{dst:r(23)?,value:10},P::Final)?;
    for word in cdsp_boot::encode_extended_word_load(r(22)?,r(25)?,-0x1fd92b4)? {
        out.extend_from_slice(&word.to_le_bytes());
    }
    put(&mut out,H::ArithmeticShiftLeft{dst:r(22)?,src:r(22)?,amount:4},P::Final)?;
    put(&mut out,H::AddRegisters{dst:r(23)?,left:r(23)?,right:r(22)?},P::Final)?;
    for word in cdsp_boot::encode_v66_plain_word_store(r(21)?,r(23)?,0x28)? {
        out.extend_from_slice(&word.to_le_bytes());
    }
    put(&mut out,H::AddUnsignedImmediate12{dst:r(21)?,src:r(21)?,value:256},P::Final)?;
    if out.len()!=FIRST_DESCRIPTOR_BYTES {
        return Err(format!("CDSP descriptor emitted {} instead of 108 bytes",out.len()));
    }
    Ok(out)
}

pub fn build_joined_first_descriptor()->Result<Vec<u8>,String>{
    let mut output=build_joined_cache_and_table_clear()?;
    if 0x3fcc+TABLE_CLEAR_SIZE!=0x3fec {return Err("table clear gap".into());}
    output.extend(build_first_controller_descriptor()?);
    if output.len()!=792 {return Err("CDSP controller first descriptor gap".into());}
    Ok(output)
}

pub fn verify_first_descriptor(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let built=build_first_controller_descriptor()?;
    if reference.get(0x3fec..0x3fec+built.len())!=Some(built.as_slice()) {
        return Err("CDSP controller first static descriptor mismatch".into());
    }
    Ok(())
}

pub fn verify_joined_first_descriptor(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let built=build_joined_first_descriptor()?;
    if reference.get(0x3d40..0x3d40+built.len())!=Some(built.as_slice()) {
        return Err("CDSP entire supervisor/TLB/cache/static-descriptor cluster mismatch".into());
    }
    Ok(())
}

#[cfg(test)]mod descriptor_tests{
    use super::*;
    #[test]fn controller_descriptor_uses_checked_offsets(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_first_controller_descriptor().unwrap().len(),108);
        assert_eq!(build_joined_first_descriptor().unwrap().len(),792);
        assert_eq!(H::AddUnsignedImmediate12{dst:r(21),src:r(21),value:256}.encode().unwrap(),0xb015_e015);
        assert!(cdsp_boot::encode_v66_new_word_store(r(21),3).is_err());
        assert!(cdsp_boot::encode_v66_plain_word_store(r(21),r(23),64).is_err());
        assert!(H::BranchIfNewLoadZeroForward{displacement_bytes:0}.encode().is_err());
    }
}
