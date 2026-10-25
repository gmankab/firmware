// SPDX-License-Identifier: GPL-3.0-only
//! Supervisor stop/restart and TLB-context preservation helpers.
//! Source instruction encoders preserve literal-free Hexagon packets and
//! branch references to earlier independently authored CDSP routines.
use super::cdsp_boot::{self,GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H};
use super::cdsp_early_phys_dump;
use super::cdsp_early_supervisor_vectors;

pub const FILE_OFFSET:usize=0x4ab0;
pub const REGION_END:usize=0x4b1c;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1ab0;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;

fn put(out:&mut Vec<u8>, word:u32, p:P){out.extend_from_slice(&p.apply(word).to_le_bytes());}
fn short_call(out:&mut Vec<u8>,at:u32,target:u32)->Result<(),String>{
    put(out,cdsp_early_supervisor_vectors::short_call(at,target)?,P::Final);Ok(())
}
fn stop_controller(src:Gpr)->u32{0x6460_c000|(u32::from(src.number())<<16)}
fn read_s54(dst:Gpr)->u32{0x6e80_c000 | (54<<16) | u32::from(dst.number())}
fn read_tlb_pair(low:Gpr,index:Gpr)->Result<u32,String>{
    if low.number()%2!=0||low.number()>30{return Err("Hexagon TLB read requires even register pair".into());}
    Ok(0x6c40_c000|(u32::from(index.number())<<16)|u32::from(low.number()))
}
fn data_clean(address:Gpr)->u32{0xa000_c000|(u32::from(address.number())<<16)}

/// Restart wrapper: copy physical state, clean cache, then enter STOP.
/// A second entry also performs global L2 synchronization first.
fn build_supervisor_stop_wrappers()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(0x2c);
    short_call(&mut out,0x96c0_1ab0,0x96c0_1a38)?;
    short_call(&mut out,0x96c0_1ab4,0x96c0_1a84)?;
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:0}.encode()?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0x18}.encode()?,P::Final);
    short_call(&mut out,0x96c0_1ac0,0x96c0_1a38)?;
    short_call(&mut out,0x96c0_1ac4,0x96c0_1a08)?;
    short_call(&mut out,0x96c0_1ac8,0x96c0_1a84)?;
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:0}.encode()?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:4}.encode()?,P::Final);
    put(&mut out,stop_controller(r(0)?),P::Final);
    put(&mut out,H::JumpBackward{displacement_bytes:-4}.encode()?,P::Final);
    if out.len()!=0x2c{return Err("CDSP stop wrappers length mismatch".into());}
    Ok(out)
}

/// The debugger exports TLB entries as 64-bit pairs to an S54-relative
/// buffer, then flushes each cache line of that buffer.
fn build_tlb_snapshot_and_clean()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(0x40);
    put(&mut out,read_s54(r(23)?),P::Final);
    put(&mut out,H::AssignUnsignedImmediate{dst:r(0)?,value:128}.encode()?,P::ContinuationA);
    let base=cdsp_boot::encode_extended_add(r(1)?,r(23)?,-0x1fdb700,true);
    put(&mut out,base[0],P::ContinuationA);
    put(&mut out,base[1],P::Final);
    put(&mut out,H::LogicalShiftRight{dst:r(4)?,src:r(0)?,amount:2}.encode()?,P::Final);
    put(&mut out,H::HardwareLoop{loop_index:0,counter:r(0)?,displacement_bytes:8}.encode()?,P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:0}.encode()?,P::Final);
    put(&mut out,read_tlb_pair(r(2)?,r(0)?)?,P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:1}.encode()?,P::LoopEnd);
    put(&mut out,H::MemDoubleStorePostIncrement{low:r(2)?,base:r(1)?,increment:8}.encode()?,P::Final);
    let base2=cdsp_boot::encode_extended_add(r(1)?,r(23)?,-0x1fdb700,true);
    put(&mut out,base2[0],P::ContinuationA);
    put(&mut out,base2[1],P::Final);
    put(&mut out,H::HardwareLoop{loop_index:0,counter:r(4)?,displacement_bytes:4}.encode()?,P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(1)?,src:r(1)?,value:32}.encode()?,P::LoopEnd);
    put(&mut out,data_clean(r(1)?),P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=0x40{return Err("CDSP TLB snapshot/flush length mismatch".into());}
    Ok(out)
}

pub fn build_maintenance_helpers()->Result<Vec<u8>,String>{
    let mut out=build_supervisor_stop_wrappers()?;
    out.extend(build_tlb_snapshot_and_clean()?);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP stop/TLB helper gap".into());}
    Ok(out)
}
pub fn build_supervisor_with_maintenance()->Result<Vec<u8>,String>{
    if cdsp_early_phys_dump::REGION_END!=FILE_OFFSET{return Err("CDSP L1 flush to maintenance gap".into());}
    let mut out=cdsp_early_phys_dump::build_supervisor_with_physical_dump()?;
    out.extend(build_maintenance_helpers()?);
    Ok(out)
}
pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_phys_dump::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_maintenance_helpers()?.as_slice()){
        return Err("CDSP supervisor maintenance helpers differ from pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn stop_wrappers_and_tlb_snapshot_are_adjacent(){
        assert_eq!(FUNCTION_BYTES,108);
        assert_eq!(build_supervisor_stop_wrappers().unwrap().len(),44);
        assert_eq!(build_tlb_snapshot_and_clean().unwrap().len(),64);
        assert_eq!(build_maintenance_helpers().unwrap().len(),108);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn typed_tlb_and_stop_reject_invalid_pairs(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(read_tlb_pair(r(2),r(0)).unwrap(),0x6c40_c002);
        assert!(read_tlb_pair(r(3),r(0)).is_err());
        assert_eq!(stop_controller(r(0)),0x6460_c000);
        assert_ne!(stop_controller(r(1)),stop_controller(r(0)));
    }
}
