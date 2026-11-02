// SPDX-License-Identifier: GPL-3.0-only
//! Independently generated Hexagon V66 zero-cache-line + 2x64-bit DMA-like
//! buffer copier at CDSP ELF file 0x42bc. The controller's zero-cache-line
//! operation and hardware loop are represented by named checked operations.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_controllers;

pub const FILE_OFFSET:usize=0x42bc;
pub const FUNCTION_LEN:usize=0x5c;
pub const REGION_END:usize=FILE_OFFSET+FUNCTION_LEN;
pub const PHYSICAL_ADDRESS:u32=0x96c0_12bc;

fn emit(out:&mut Vec<u8>,op:H,marker:P)->Result<(),String>{
    out.extend_from_slice(&marker.apply(op.encode()?).to_le_bytes());Ok(())
}
fn emit_gp(out:&mut Vec<u8>,dest:Gpr,addr:u32,final_packet:bool){
    let pair=cdsp_boot::encode_extended_gp_word_load(dest,addr);
    out.extend_from_slice(&pair[0].to_le_bytes());
    out.extend_from_slice(&(if final_packet {pair[1]} else {P::ContinuationA.apply(pair[1])}).to_le_bytes());
}

pub fn build_cache_zero_copy()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_LEN);
    emit_gp(&mut out,r(1)?,0xc067_3260,false);
    emit_gp(&mut out,r(2)?,0xc067_3264,true);
    emit(&mut out,H::SkipIfSourceZeroNotTaken{src:r(1)?,displacement_bytes:0x48},P::Final)?;
    emit(&mut out,H::SkipIfSourceZeroNotTaken{src:r(2)?,displacement_bytes:0x44},P::Final)?;
    emit(&mut out,H::PredicateP0GreaterThan32{src:r(2)?},P::ContinuationA)?;
    emit(&mut out,H::JumpIfNotNewP0NotTaken{displacement_bytes:0x40},P::Final)?;
    emit(&mut out,H::LogicalShiftRight{dst:r(3)?,src:r(2)?,amount:5},P::ContinuationA)?;
    // This conditional branch is relative to the preceding packet *start*,
    // not its own machine-word PC (0x42dc, rather than 0x42e0).
    emit(&mut out,H::SkipIfSourceZeroNotTaken{src:r(0)?,displacement_bytes:0x38},P::Final)?;
    emit(&mut out,H::HardwareLoop{loop_index:0,counter:r(3)?,displacement_bytes:12},P::ContinuationA)?;
    emit(&mut out,H::AddSmallImmediate{dst:r(4)?,src:r(0)?,value:8},P::ContinuationA)?;
    emit(&mut out,H::AddSmallImmediate{dst:r(5)?,src:r(1)?,value:8},P::Final)?;
    emit(&mut out,H::CacheZeroLine{address:r(1)?},P::Final)?;
    for (low,base,marker) in [(6,0,P::ContinuationA),(8,4,P::Final)]{
        emit(&mut out,H::LoadDoublePostIncrement{low:r(low)?,base:r(base)?,increment:16},marker)?;
    }
    for (low,base,marker) in [(6,1,P::ContinuationA),(8,5,P::Final)]{
        emit(&mut out,H::MemDoubleStorePostIncrement{low:r(low)?,base:r(base)?,increment:16},marker)?;
    }
    for (low,base,marker) in [(6,0,P::ContinuationA),(8,4,P::Final)]{
        emit(&mut out,H::LoadDoublePostIncrement{low:r(low)?,base:r(base)?,increment:16},marker)?;
    }
    for (low,base,marker) in [(6,1,P::LoopEnd),(8,5,P::Final)]{
        emit(&mut out,H::MemDoubleStorePostIncrement{low:r(low)?,base:r(base)?,increment:16},marker)?;
    }
    emit(&mut out,H::JumpRegister{source:r(31)?},P::Final)?;
    if out.len()!=FUNCTION_LEN {return Err("CDSP cache-zero copy instruction width mismatch".into());}
    Ok(out)
}

pub fn build_joined()->Result<Vec<u8>,String>{
    let mut bytes=cdsp_early_controllers::build_joined()?;
    if cdsp_early_controllers::REGION_END!=FILE_OFFSET {return Err("CDSP startup/copy is not adjacent".into());}
    bytes.extend(build_cache_zero_copy()?);
    if bytes.len()!=REGION_END-super::cdsp_early_tlb::FILE_OFFSET{return Err("CDSP controller buffer span size mismatch".into());}
    Ok(bytes)
}

pub fn verify_joined(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let generated=build_joined()?;
    let offset=super::cdsp_early_tlb::FILE_OFFSET;
    let expected=reference.get(offset..REGION_END).ok_or("CDSP pinned image truncated")?;
    if let Some((i,(x,y)))=generated.iter().zip(expected).enumerate().find(|(_, (x,y))|x!=y){
        return Err(format!("CDSP cache-copy differs at {:#x}: generated {x:02x}, reference {y:02x}",offset+i));
    }
    Ok(())
}

#[cfg(test)] mod tests{
    use super::*;
    #[test]fn cache_zero_and_dual_copy_is_fully_typed(){
        assert_eq!(build_cache_zero_copy().unwrap().len(),FUNCTION_LEN);
        assert_eq!(build_joined().unwrap().len(),0x138);
        assert_eq!(PHYSICAL_ADDRESS,0x96c00000+(FILE_OFFSET-0x3000) as u32);
    }
}
