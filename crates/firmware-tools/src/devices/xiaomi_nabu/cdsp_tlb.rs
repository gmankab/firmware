// SPDX-License-Identifier: GPL-3.0-only
//! Two disjoint-from-entry but mutually adjacent CDSP TLB reset functions.
//! Physical address is determined by executable segment p_paddr and file
//! offset; no reference words or vendor image slices are embedded here.
use super::cdsp_boot::{self,GeneralRegister as Gpr,ScalarPacket as Hexagon,PacketMarker as Parse};

pub const FILE_OFFSET:usize=0x3dfc;
pub const PHYSICAL_ADDRESS:u32=0x96c0_0dfc;
pub const SETUP_SIZE:usize=44;
pub const CLEAR_SIZE:usize=16;
pub const REGION_SIZE:usize=SETUP_SIZE+CLEAR_SIZE;

/// Configure one MMU mapping using two 16-bit packed attributes, then
/// invalidate/synchronize the TLB and return. Packet groups were checked
/// against the Hexagon assembler for supported arithmetic and Rizin V66
/// for the tlbw system write (which LLVM6 does not assemble).
pub fn build_tlb_setup() -> Result<Vec<u8>,String> {
    let r=|n| Gpr::new(n);
    let source = [
        (Hexagon::LogicalShiftRight {dst:r(2)?,src:r(24)?,amount:11}, Parse::ContinuationA),
        (Hexagon::LogicalShiftRight {dst:r(3)?,src:r(26)?,amount:12}, Parse::ContinuationA),
        (Hexagon::AssignUnsignedImmediate {dst:r(4)?,value:0x7c0}, Parse::ContinuationA),
        (Hexagon::AssignUnsignedImmediate {dst:r(5)?,value:0xc00}, Parse::Final),
        (Hexagon::InsertBitField {dst:r(2)?,src:r(4)?,width:11,offset:21}, Parse::ContinuationA),
        (Hexagon::InsertBitField {dst:r(3)?,src:r(5)?,width:12,offset:20}, Parse::Final),
        (Hexagon::SetBit {dst:r(2)?,src:r(2)?,position:1}, Parse::ContinuationA),
        (Hexagon::AssignUnsignedImmediate {dst:r(0)?,value:0x7e}, Parse::Final),
        (Hexagon::TlbWriteIndexedR0 {low:r(2)?}, Parse::Final),
        (Hexagon::Isync, Parse::Final),
        (Hexagon::JumpRegister {source:r(31)?}, Parse::Final),
    ];
    let mut out=Vec::with_capacity(SETUP_SIZE);
    for (op,marker) in source {out.extend_from_slice(&marker.apply(op.encode()?).to_le_bytes());}
    if out.len()!=SETUP_SIZE {return Err("CDSP setup isn't 44 bytes".into());}
    Ok(out)
}

/// Clear one indexed TLB slot, synchronize, then return; the pair zero
/// initializer can be independently assembled with LLVM6.
pub fn build_tlb_clear() -> Result<Vec<u8>,String> {
    let r=|n|Gpr::new(n);
    let ops = [
        Hexagon::CombineZeroPair{low:r(2)?},
        Hexagon::TlbWriteIndexedR0{low:r(2)?},
        Hexagon::Isync,
        Hexagon::JumpRegister{source:r(31)?},
    ];
    let mut out=Vec::with_capacity(CLEAR_SIZE);
    for op in ops {out.extend_from_slice(&op.encode()?.to_le_bytes());}
    Ok(out)
}

pub fn build_tlb_maintenance()->Result<Vec<u8>,String>{
    let mut out=build_tlb_setup()?;
    out.extend(build_tlb_clear()?);
    if out.len()!=REGION_SIZE {return Err("CDSP TLB span size mismatch".into());}
    Ok(out)
}

pub fn verify_tlb_maintenance(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let rebuilt=build_tlb_maintenance()?;
    let target=reference.get(FILE_OFFSET..FILE_OFFSET+REGION_SIZE)
        .ok_or("CDSP reference missing TLB maintenance span")?;
    if let Some((i,(a,b)))=target.iter().zip(&rebuilt).enumerate().find(|(_,(a,b))|a!=b) {
        return Err(format!("CDSP TLB byte mismatch at file {:#x}: ref {a:02x}, built {b:02x}",FILE_OFFSET+i));
    }
    Ok(())
}

/// Guard and program an additional 7-bit TLB slot only when R25 is nonzero.
/// A two-instruction Hexagon packet first compares R25 and branches via
/// P0.new; LLVM 6 independently confirms the two operation bit fields,
/// although LLVM's packet scheduler chooses the opposite instruction order.
pub const CONDITIONAL_OFFSET:usize=0x3dc8;
pub const CONDITIONAL_SIZE:usize=13*4;
pub fn build_conditional_tlb() -> Result<Vec<u8>,String> {
    let r=|n|Gpr::new(n);
    let ops=[
        (Hexagon::PredicateP0EqualsZero{src:r(25)?},Parse::ContinuationA),
        (Hexagon::JumpIfNewP0NotTaken{displacement_bytes:0x30},Parse::Final),
        (Hexagon::LogicalShiftRight{dst:r(2)?,src:r(24)?,amount:11},Parse::ContinuationA),
        (Hexagon::LogicalShiftRight{dst:r(3)?,src:r(24)?,amount:12},Parse::ContinuationA),
        (Hexagon::AssignUnsignedImmediate{dst:r(4)?,value:0x7c0},Parse::ContinuationA),
        (Hexagon::AssignUnsignedImmediate{dst:r(5)?,value:0xc00},Parse::Final),
        (Hexagon::InsertBitField{dst:r(2)?,src:r(4)?,width:11,offset:21},Parse::ContinuationA),
        (Hexagon::InsertBitField{dst:r(3)?,src:r(5)?,width:12,offset:20},Parse::Final),
        (Hexagon::SetBit{dst:r(2)?,src:r(2)?,position:1},Parse::ContinuationA),
        (Hexagon::AssignUnsignedImmediate{dst:r(0)?,value:0x7f},Parse::Final),
        (Hexagon::TlbWriteIndexedR0{low:r(2)?},Parse::Final),
        (Hexagon::Isync,Parse::Final),
        (Hexagon::JumpRegister{source:r(31)?},Parse::Final),
    ];
    let mut data=Vec::with_capacity(CONDITIONAL_SIZE);
    for (inst,parse) in ops {data.extend_from_slice(&parse.apply(inst.encode()?).to_le_bytes());}
    if data.len()!=CONDITIONAL_SIZE{return Err("CDSP conditional tlb should be 52 bytes".into());}
    Ok(data)
}

pub const FULL_TLB_SIZE:usize=CONDITIONAL_SIZE+REGION_SIZE;
pub fn build_full_tlb_span() -> Result<Vec<u8>,String> {
    if CONDITIONAL_OFFSET+CONDITIONAL_SIZE!=FILE_OFFSET {
        return Err("CDSP TLB fragment gap".into());
    }
    let mut data=build_conditional_tlb()?;
    data.extend(build_tlb_maintenance()?);
    if data.len()!=FULL_TLB_SIZE {return Err("CDSP TLB span length mismatch".into());}
    Ok(data)
}

pub fn verify_full_tlb_span(reference:&[u8])->Result<(),String> {
    cdsp_boot::validate_reference(reference)?;
    let got=build_full_tlb_span()?;
    let found=reference.get(CONDITIONAL_OFFSET..CONDITIONAL_OFFSET+FULL_TLB_SIZE)
        .ok_or("CDSP reference missing conditional tlb span")?;
    if let Some((i,(a,b)))=found.iter().zip(&got).enumerate().find(|(_,(a,b))|a!=b) {
        return Err(format!("CDSP conditional TLB mismatch at file {:#x}: {a:02x} != {b:02x}",CONDITIONAL_OFFSET+i));
    }
    Ok(())
}

/// CDSP TLB walker at file 0x3d94; matches a 13-instruction Hexagon
/// sequence ending at the already reconstructed conditional TLB function.
/// Three grouped instructions initialize a pointer with immext; then a
/// loop advances a packed entry, builds TLB attributes and returns.
pub const WALK_OFFSET:usize=0x3d94;
pub const WALK_SIZE:usize=13*4;
pub fn build_tlb_walk() -> Result<Vec<u8>,String> {
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(WALK_SIZE);
    let start=Hexagon::AssignSmallImmediate{dst:r(0)?,value:0};
    out.extend_from_slice(&Parse::ContinuationA.apply(start.encode()?).to_le_bytes());
    for ext_word in cdsp_boot::encode_extended_add(r(1)?,r(25)?,-0x1fdb700,true) {
        out.extend_from_slice(&ext_word.to_le_bytes());
    }
    let body=[
        (Hexagon::LoadDoublePostIncrement{low:r(2)?,base:r(1)?,increment:8},Parse::Final),
        (Hexagon::ExtractUnsigned{dst:r(4)?,src:r(3)?,width:7,offset:20},Parse::Final),
        (Hexagon::ReverseSubtractImmediate{dst:r(4)?,src:r(4)?,value:125},Parse::ContinuationA),
        (Hexagon::BranchOnNewValueNonzero{displacement_bytes:12},Parse::Final),
        (Hexagon::InsertBitField{dst:r(3)?,src:r(4)?,width:7,offset:20},Parse::Final),
        (Hexagon::TlbWriteIndexedR0{low:r(2)?},Parse::Final),
        (Hexagon::LoopOnNotP0New{src:r(2)?,displacement_bytes:-24},Parse::ContinuationA),
        (Hexagon::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:1},Parse::Final),
        (Hexagon::Isync,Parse::Final),
        (Hexagon::JumpRegister{source:r(31)?},Parse::Final),
    ];
    for (inst,mark) in body {out.extend_from_slice(&mark.apply(inst.encode()?).to_le_bytes());}
    if out.len()!=WALK_SIZE {return Err("CDSP TLB walker not 52 bytes".into());}
    Ok(out)
}

pub const COMPLETE_CLUSTER_OFFSET:usize=0x3d40;
pub const COMPLETE_CLUSTER_SIZE:usize=248;
/// Combine the complete reset+relocator+disable+walker+conditional-TLB+TLB
/// source-authored span only after explicit adjacency checks.
pub fn build_complete_tlb_cluster() -> Result<Vec<u8>,String> {
    use super::cdsp_privilege as privileged;
    if privileged::FILE_OFFSET+privileged::SUPERVISOR_SPAN_SIZE != WALK_OFFSET ||
       WALK_OFFSET+WALK_SIZE != CONDITIONAL_OFFSET ||
       CONDITIONAL_OFFSET+FULL_TLB_SIZE != COMPLETE_CLUSTER_OFFSET+COMPLETE_CLUSTER_SIZE {
        return Err("CDSP reconstructed supervisor/TLB regions not adjacent".into());
    }
    let mut out=privileged::build_supervisor_span()?;
    out.extend(build_tlb_walk()?);
    out.extend(build_full_tlb_span()?);
    if out.len()!=COMPLETE_CLUSTER_SIZE {return Err("CDSP cluster size mismatch".into());}
    Ok(out)
}

pub fn verify_complete_tlb_cluster(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let actual=build_complete_tlb_cluster()?;
    let given=reference.get(COMPLETE_CLUSTER_OFFSET..COMPLETE_CLUSTER_OFFSET+COMPLETE_CLUSTER_SIZE)
        .ok_or("CDSP reference missing TLB cluster")?;
    if let Some((i,(a,b)))=given.iter().zip(&actual).enumerate().find(|(_,(a,b))|a!=b) {
        return Err(format!("CDSP cluster byte differs at {:#x}: reference {a:02x}, generated {b:02x}",COMPLETE_CLUSTER_OFFSET+i));
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn supervisor_tlb_cluster_contains_all_known_adjacent_routines(){
        let data=build_complete_tlb_cluster().unwrap();
        assert_eq!(data.len(),COMPLETE_CLUSTER_SIZE);
        assert_eq!(&data[84..84+WALK_SIZE],build_tlb_walk().unwrap());
        assert_eq!(&data[84+WALK_SIZE..],build_full_tlb_span().unwrap());
    }
    #[test]fn conditional_tlb_programs_writes_without_holes(){
        let b=build_conditional_tlb().unwrap();
        assert_eq!(b.len(),CONDITIONAL_SIZE);
        assert_eq!(&build_full_tlb_span().unwrap()[..CONDITIONAL_SIZE],b.as_slice());
        assert_eq!(FULL_TLB_SIZE,112);
    }
    #[test]fn tlb_maintenance_has_exact_coverage(){
        assert_eq!(build_tlb_setup().unwrap().len(),SETUP_SIZE);
        assert_eq!(build_tlb_clear().unwrap().len(),CLEAR_SIZE);
        assert_eq!(build_tlb_maintenance().unwrap().len(),REGION_SIZE);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+0xdfc);
    }
}
