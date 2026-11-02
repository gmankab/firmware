// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 TLB snapshot: publish saved TLB entries through TLBW, read
//! entries through TLBR, then clean the saved cache lines using hardware loops.
//! Packet-alignment and paired scalar/TLB instructions use checked ISA fields.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::{cdsp_early_hvx_init as previous,cdsp_early_worker_command as worker};

pub const FILE_OFFSET:usize=0x5990;
pub const REGION_END:usize=0x59f0;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2990;
fn put(out:&mut Vec<u8>,opcode:u32,parse:P){out.extend_from_slice(&parse.apply(opcode).to_le_bytes());}

/// Compact `R1 = ##address; R0 = #0` used immediately after an extender.
/// The source/destination constraints are deliberately narrow until further
/// independent duplex field evidence is available.
fn paired_base_and_zero(first:Gpr,second:Gpr,constant:u8)->Result<u32,String>{
    if first.number()!=1 || second.number()!=0 || constant!=0 {
        return Err("paired extended R1 address/R0 zero only supports original compact fields".into());
    }
    Ok(0x2800_2800|(u32::from(first.number())<<16))
}
/// V66 TLB read is privileged and unsupported by the LLVM6 assembler.
/// Rizin V66 verifies its register operands and packet placement.
fn read_tlb(low:Gpr,index:Gpr)->Result<u32,String>{
    if low.number()%2!=0 || low.number()>30 {
        return Err("TLBR output must be a properly aligned register pair".into());
    }
    Ok(0x6c40_c000|(u32::from(index.number())<<16)|u32::from(low.number()))
}
/// P0 predicate compares the updated R0 with R4, then backward branches
/// inside a two-instruction Hexagon packet. Independent LLVM6 packet sample
/// verifies split backward offset and both implicit predicate fields.
fn retry_entry_compare(packet_start:u32,target:u32,left:Gpr,right:Gpr)->Result<u32,String>{
    if left.number()!=0 || right.number()!=4 || target>=packet_start || target%4!=0
       || packet_start%4!=0 || packet_start-target>0x1fc {
        return Err("TLB copy retry requires original R0/R4 registers and short backward jump".into());
    }
    let half=((i64::from(target)-i64::from(packet_start))/2) as i32 as u32;
    Ok(0x1470_2400 | (half&0xff))
}

fn write_tlb_entries(out:&mut Vec<u8>)->Result<(),String>{
    let r=|n|Gpr::new(n);
    put(out,H::AssignSmallImmediate{dst:r(4)?,value:127}.encode()?,P::ContinuationA);
    let address=cdsp_boot::encode_extended_assignment(r(1)?,0xfe02_4900u32 as i32,false);
    put(out,address[0],P::ContinuationA);
    // Producer of paired R1/R0 assignments occupies a 32-bit compact word.
    out.extend_from_slice(&paired_base_and_zero(r(1)?,r(0)?,0)?.to_le_bytes());
    put(out,H::LoadDoublePostIncrement{low:r(2)?,base:r(1)?,increment:8}.encode()?,P::Final);
    put(out,H::TlbWriteIndexedR0{low:r(2)?}.encode()?,P::Final);
    put(out,retry_entry_compare(0x96c0_29a4,0x96c0_299c,r(0)?,r(4)?)?,P::ContinuationA);
    put(out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:1}.encode()?,P::Final);
    put(out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    Ok(())
}
fn snapshot_tlb_entries(out:&mut Vec<u8>)->Result<(),String>{
    let r=|n|Gpr::new(n);
    put(out,H::AssignUnsignedImmediate{dst:r(0)?,value:128}.encode()?,P::ContinuationA);
    let memory=cdsp_boot::encode_extended_assignment(r(1)?,0xfe02_4900u32 as i32,true);
    put(out,memory[0],P::ContinuationA);
    put(out,memory[1],P::Final);
    put(out,H::LogicalShiftRight{dst:r(4)?,src:r(0)?,amount:2}.encode()?,P::Final);
    put(out,H::HardwareLoop{loop_index:0,counter:r(0)?,displacement_bytes:8}.encode()?,P::ContinuationA);
    put(out,H::AssignSmallImmediate{dst:r(0)?,value:0}.encode()?,P::Final);
    put(out,read_tlb(r(2)?,r(0)?)?,P::Final);
    put(out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:1}.encode()?,P::LoopEnd);
    put(out,H::MemDoubleStorePostIncrement{low:r(2)?,base:r(1)?,increment:8}.encode()?,P::Final);
    let save=cdsp_boot::encode_extended_assignment(r(1)?,0xfe02_4900u32 as i32,true);
    put(out,save[0],P::ContinuationA);
    put(out,save[1],P::Final);
    put(out,H::HardwareLoop{loop_index:0,counter:r(4)?,displacement_bytes:4}.encode()?,P::Final);
    put(out,H::AddSmallImmediate{dst:r(1)?,src:r(1)?,value:32}.encode()?,P::LoopEnd);
    put(out,worker::cache_clean(r(1)?),P::Final);
    put(out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    Ok(())
}
/// Unconditional jump-register stub immediately after the two TLB helpers.
fn empty_tlb_handler(out:&mut Vec<u8>)->Result<(),String>{
    put(out,H::JumpRegister{source:Gpr::new(31)?}.encode()?,P::Final);
    Ok(())
}

pub fn build_tlb_state()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    write_tlb_entries(&mut out)?;
    if out.len()!=0x20{return Err("TLB writer must end at 0x59b0".into());}
    snapshot_tlb_entries(&mut out)?;
    if out.len()!=0x5c{return Err("TLB snapshot must end at 0x59ec".into());}
    empty_tlb_handler(&mut out)?;
    if out.len()!=FUNCTION_BYTES{return Err("CDSP TLB helper length mismatch".into());}
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("CDSP TLB save not adjacent to HVX init".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_tlb_state()?.as_slice()){
        return Err("source-built TLB writer/snapshot differs from pinned reference".into());
    }
    Ok(())
}

#[cfg(test)] mod tests{
    use super::*;
    #[test]fn tlb_writer_and_snapshot_are_checked(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,96);
        assert_eq!(paired_base_and_zero(r(1),r(0),0).unwrap(),0x2801_2800);
        assert!(paired_base_and_zero(r(2),r(0),0).is_err());
        assert_eq!(read_tlb(r(2),r(0)).unwrap(),0x6c40_c002);
        assert!(read_tlb(r(3),r(0)).is_err());
        assert_eq!(retry_entry_compare(0x96c0_29a4,0x96c0_299c,r(0),r(4)).unwrap(),0x1470_24fc);
        assert!(retry_entry_compare(0x96c0_29a4,0x96c0_29a8,r(0),r(4)).is_err());
        assert_eq!(build_tlb_state().unwrap().len(),FUNCTION_BYTES);
    }
}
