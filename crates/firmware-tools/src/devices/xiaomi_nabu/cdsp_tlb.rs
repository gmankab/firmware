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

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn tlb_maintenance_has_exact_coverage(){
        assert_eq!(build_tlb_setup().unwrap().len(),SETUP_SIZE);
        assert_eq!(build_tlb_clear().unwrap().len(),CLEAR_SIZE);
        assert_eq!(build_tlb_maintenance().unwrap().len(),REGION_SIZE);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+0xdfc);
    }
}
