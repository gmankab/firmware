// SPDX-License-Identifier: GPL-3.0-only
//! Two adjacent V66 TLB routines immediately following the previously
//! rebuilt supervisor/TLB cluster. No reference opcode arrays are imported.
//! LLVM6 independently assembles ordinary operations; for the two privileged
//! TLB writes Rizin V66 confirms operand decoding only.
use super::{cdsp_boot::{self, GeneralRegister as Gpr, ScalarPacket as H, PacketMarker as P}, cdsp_tlb};

pub const FILE_OFFSET: usize = 0x3e38;
pub const PHYSICAL_ADDRESS: u32 = 0x96c0_0e38;
pub const CLEAR_LOOP_BYTES: usize = 28;
pub const READ_TABLE_BYTES: usize = 36;
pub const TOTAL_BYTES: usize = CLEAR_LOOP_BYTES + READ_TABLE_BYTES;

fn put(out: &mut Vec<u8>, instruction: H, parse: P) -> Result<(), String> {
    out.extend_from_slice(&parse.apply(instruction.encode()?).to_le_bytes());
    Ok(())
}

/// Walk the 127-slot TLB, clearing R3:2 each time, returning once the slot
/// index reaches the inclusive upper bound. The conditional return uses
/// P0.new computed in the same 8-byte packet.
pub fn build_clear_loop() -> Result<Vec<u8>, String> {
    let reg=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(CLEAR_LOOP_BYTES);
    put(&mut out, H::AssignUnsignedImmediate{dst:reg(5)?,value:127},P::ContinuationA)?;
    // Hexagon duplex has its own parse encoding; do not overlay 0xc000.
    out.extend_from_slice(&H::DuplexClearR0AndPair32.encode()?.to_le_bytes());
    put(&mut out,H::PredicateP0EqualsRegisters{left:reg(0)?,right:reg(5)?},P::ContinuationA)?;
    put(&mut out,H::ReturnIfNewP0NotTaken,P::Final)?;
    put(&mut out,H::TlbWriteIndexedR0{low:reg(2)?},P::Final)?;
    put(&mut out,H::AddSmallImmediate{dst:reg(0)?,src:reg(0)?,value:1},P::ContinuationA)?;
    // Packet start 0x96c00e4c branches to 0x96c00e40, i.e. -12 bytes.
    put(&mut out,H::JumpBackward{displacement_bytes:-12},P::Final)?;
    if out.len()!=CLEAR_LOOP_BYTES{return Err("unexpected CDSP TLB clear-loop length".into());}
    Ok(out)
}

/// Clear one TLB mapping at the address computed from a fixed relocation
/// base and an index shifted by three. The extended signed immediate is
/// source-derived; `tlbw` itself has no matching LLVM6 assembler opcode.
pub fn build_clear_index() -> Result<Vec<u8>, String> {
    let reg=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(READ_TABLE_BYTES);
    put(&mut out,H::AssignUnsignedImmediate{dst:reg(0)?,value:127},P::ContinuationA)?;
    for w in cdsp_boot::encode_extended_assignment(reg(1)?,-0x1fdb700,true) {
        out.extend_from_slice(&w.to_le_bytes());
    }
    for (instr,parse) in [
        (H::ArithmeticShiftLeft{dst:reg(2)?,src:reg(0)?,amount:3},P::Final),
        (H::AddRegisters{dst:reg(1)?,left:reg(1)?,right:reg(2)?},P::Final),
        (H::AssignUnsignedImmediate{dst:reg(0)?,value:127},P::ContinuationA),
        (H::LoadDoublePostIncrement{low:reg(2)?,base:reg(1)?,increment:0},P::Final),
        (H::TlbWriteIndexedR0{low:reg(2)?},P::Final),
        (H::JumpRegister{source:reg(31)?},P::Final),
    ] { put(&mut out,instr,parse)?; }
    if out.len()!=READ_TABLE_BYTES{return Err("unexpected CDSP TLB indexed-clear length".into());}
    Ok(out)
}

/// The old 248-byte cluster and these adjacent 64 bytes are contiguous.
pub fn build_extended_cluster() -> Result<Vec<u8>,String> {
    if cdsp_tlb::COMPLETE_CLUSTER_OFFSET+cdsp_tlb::COMPLETE_CLUSTER_SIZE != FILE_OFFSET {
        return Err("CDSP privileged cluster has a gap before extended TLB".into());
    }
    let mut b=cdsp_tlb::build_complete_tlb_cluster()?;
    b.extend(build_clear_loop()?);
    b.extend(build_clear_index()?);
    if b.len()!=cdsp_tlb::COMPLETE_CLUSTER_SIZE+TOTAL_BYTES {
        return Err("CDSP extended cluster length mismatch".into());
    }
    Ok(b)
}

pub fn verify_extended_cluster(reference:&[u8])->Result<(),String> {
    cdsp_boot::validate_reference(reference)?;
    let b=build_extended_cluster()?;
    let start=cdsp_tlb::COMPLETE_CLUSTER_OFFSET;
    let source=reference.get(start..start+b.len()).ok_or("CDSP truncated supervisor/TLB cluster")?;
    if let Some((i,(&a,&v)))=source.iter().zip(&b).enumerate().find(|(_,(x,y))|x!=y) {
        return Err(format!("CDSP extended TLB reference mismatch at file {:#x}: reference {a:02x}, built {v:02x}",start+i));
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn neighboring_tlb_loops_join_without_holes(){
        let a=build_clear_loop().unwrap();let b=build_clear_index().unwrap();
        assert_eq!(a.len(),28);assert_eq!(b.len(),36);
        let joined=build_extended_cluster().unwrap();
        assert_eq!(joined.len(),312);
        assert_eq!(&joined[248..276],a.as_slice());
        assert_eq!(&joined[276..],b.as_slice());
        assert_eq!(PHYSICAL_ADDRESS,0x96c00000+(FILE_OFFSET-0x3000) as u32);
    }
}
