// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 lock-free shared counter and caller pointer update, 0x56464..0x564b8.
//!
//! Uses memw_locked + atomic conditional store, packet-relative branches and
//! one combined immediate/data-load duplex. Original packet boundaries are
//! explicit and validated with an independent Hexagon disassembler. This is
//! typed per-instruction encoding, not a copied OEM instruction array.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_seed_handoff;

pub const FILE_OFFSET:usize=0x56464;
pub const END_FILE_OFFSET:usize=0x564b8;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc8464;
pub const RWE_LAYOUT_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_LAYOUT_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_seed_handoff::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_seed_handoff::RWE_LAYOUT_NOP_LEN;

fn emit(out:&mut Vec<u8>,w:u32){out.extend_from_slice(&w.to_le_bytes());}

/// Load the global lock-free counter from `0xc0363e00` with a V66
/// `R2=#absolute` + `R3=memw(R2)` fused instruction word and immext.
/// LLVM6 emits two standalone instructions for this logical operation;
/// the fused V66 packet was decoded independently by Rizin.
pub fn encode_counter_init_pointer(dst:G,base:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=3||base.number()!=2||address!=0xc036_3e00{
        return Err("Counter uses R3=memw(R2=0xc0363e00)".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(base,address as i32,false)[0];
    let fused=M::ContinuationA.apply(0x9b80_1000 | (u32::from(base.number())<<16)
                                    | (u32::from(dst.number())) |0x4000);
    Ok([ext,fused])
}

/// If newly loaded R3 is zero, jump to the caller's empty-state return.
/// The branch immediate is relative to the V66 packet start `0x56464`.
pub fn encode_counter_empty_branch(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start!=PHYSICAL_ADDRESS||target!=0x96cc_849c{
        return Err("Unverified CDSP shared counter empty-state packet/target".into());
    }
    let d=target.checked_sub(packet_start).ok_or("Target precedes counter")?;
    if d>0x1fe||d&1!=0{return Err("Counter branch outside packet displacement".into());}
    Ok(0x2402_c000|(d/2))
}

pub fn encode_minus_one(dst:G,value:i32)->Result<u32,String>{
    if dst.number()!=4||value!=-1{return Err("Lock decrement requires R4=-1".into());}
    Ok(0x78df_c000|(((value as u32)&0x1ff)<<5)|u32::from(dst.number()))
}

/// Read a word under atomic lock (`R3/R4=memw_locked(R2)`).
pub fn encode_atomic_load(dst:G,base:G)->Result<u32,String>{
    if base.number()!=2||![3,4].contains(&dst.number()){
        return Err("Counter atomic word load only supports R3/R4 from R2".into());
    }
    Ok(0x9200_c000|(u32::from(base.number())<<16)|u32::from(dst.number()))
}

/// Store R3 or R4 as conditional lock-free result; the instruction also
/// produces P0 indicating whether atomic compare/store succeeded.
pub fn encode_atomic_store(base:G,value:G)->Result<u32,String>{
    if base.number()!=2||![3,4].contains(&value.number()){
        return Err("Counter locked conditional store needs R2 and R3/R4".into());
    }
    Ok(0xa0a0_c000|(u32::from(base.number())<<16)|(u32::from(value.number())<<8))
}

/// If P0 is false retry the appropriate atomic compare-and-swap loop.
pub fn encode_retry(packet_start:u32,target:u32)->Result<u32,String>{
    if ![(0x96cc_8480,0x96cc_8474),(0x96cc_84ac,0x96cc_84a0)].contains(&(packet_start,target)){
        return Err("Unverified CDSP counter retry loop".into());
    }
    let d=(target as i64)-(packet_start as i64);
    if d>=0||d< -256||d&1!=0{return Err("Counter retry displacement invalid".into());}
    Ok(0x5cff_e000|(((d/2) as i32 as u32)&0xff))
}

/// In a P0.new packet, set R3=1 when the atomic result needs a fixup.
pub fn encode_if_not_new_set_one(dst:G,value:u8)->Result<u32,String>{
    if dst.number()!=3||value!=1{return Err("Expected if(!P0.new) R3=1".into());}
    Ok(M::ContinuationA.apply(0x7e80_2000|(u32::from(value)<<5)|u32::from(dst.number())))
}

/// Compare `R3 > -1` and conditionally skip to the second atomic loop;
/// branch offset is in halfwords from packet start `0x56484`.
pub fn encode_signed_counter_second_loop(src:G,value:i32,packet_start:u32,target:u32)->Result<u32,String>{
    if src.number()!=3||value!=-1||packet_start!=0x96cc_8484||target!=0x96cc_84a0{
        return Err("Unverified counter signed-bound check and forward branch".into());
    }
    let disp=target.checked_sub(packet_start).ok_or("Forward counter branch negative")?;
    if disp&1!=0 || disp>0x1fe{return Err("Counter cmp+branch out of range".into());}
    Ok(M::Final.apply(0x11c0_0100|(u32::from(src.number())<<16)|(disp/2)))
}

/// V66 extended two-source register-indexed load `R2=memw(R3<<2+
/// ##0xc0363e08)` after a compatible immext. Format confirmed in
/// independent Rizin decoding; all other offset/register forms are refused.
pub fn encode_indexed_counter_read(dst:G,index:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=2||index.number()!=3||address!=0xc036_3e08{
        return Err("Indexed counter reads R2 from R3<<2 + 0xc0363e08".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let body=M::ContinuationA.apply(0x9d80_3000|(u32::from(index.number())<<16)|0x200|u32::from(dst.number()));
    Ok([ext,body])
}

/// Store recently read R2.new into R0's output pointer, using implicit
/// V66 new-value producer distance one packet slot.
pub fn encode_new_output_store(base:G,value:G,producer_distance:u8)->Result<u32,String>{
    if (base.number(),value.number(),producer_distance)!=(0,2,1){
        return Err("Counter output expects memw(R0)=R2.new".into());
    }
    Ok(0xa1a0_d000|(u32::from(producer_distance)<<9))
}

pub fn build_atomic_counter()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    for word in encode_counter_init_pointer(r(3)?,r(2)?,0xc036_3e00)? {emit(&mut out,word);}
    emit(&mut out,encode_counter_empty_branch(PHYSICAL_ADDRESS,0x96cc_849c)?);
    emit(&mut out,encode_minus_one(r(4)?,-1)?);
    emit(&mut out,encode_atomic_load(r(3)?,r(2)?)?);
    emit(&mut out,S::AddRegisters{dst:r(3)?,left:r(3)?,right:r(4)?}.encode()?);
    emit(&mut out,encode_atomic_store(r(2)?,r(3)?)?);
    emit(&mut out,encode_retry(0x96cc_8480,0x96cc_8474)?);
    emit(&mut out,encode_if_not_new_set_one(r(3)?,1)?);
    emit(&mut out,encode_signed_counter_second_loop(r(3)?,-1,0x96cc_8484,0x96cc_84a0)?);
    emit(&mut out,M::ContinuationA.apply(S::JumpRegister{source:r(31)?}.encode()?));
    for word in encode_indexed_counter_read(r(2)?,r(3)?,0xc036_3e08)? {emit(&mut out,word);}
    emit(&mut out,encode_new_output_store(r(0)?,r(2)?,1)?);
    emit(&mut out,S::JumpRegister{source:r(31)?}.encode()?);
    emit(&mut out,encode_atomic_load(r(4)?,r(2)?)?);
    emit(&mut out,S::AddRegisters{dst:r(4)?,left:r(4)?,right:r(3)?}.encode()?);
    emit(&mut out,encode_atomic_store(r(2)?,r(4)?)?);
    emit(&mut out,encode_retry(0x96cc_84ac,0x96cc_84a0)?);
    emit(&mut out,S::JumpRegister{source:r(31)?}.encode()?);
    emit(&mut out,S::JumpRegister{source:r(31)?}.encode()?);
    if out.len()!=EXECUTABLE_LEN{return Err(format!("CDSP atomics emitted {} !=84 bytes",out.len()));}
    Ok(out)
}

pub fn build_rwe_prefix_through_atomic()->Result<Vec<u8>,String>{
    let mut result=cdsp_late_seed_handoff::build_rwe_prefix_through_handoff()?;
    result.extend(build_atomic_counter()?);
    if result.len()!=FULL_RWE_LAYOUT_LEN{return Err("CDSP counter prefix length invalid".into());}
    Ok(result)
}

pub fn verify_atomic(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let code=build_atomic_counter()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(code.as_slice()){
        return Err("CDSP lock-free counter differs from pinned firmware".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let code=build_rwe_prefix_through_atomic()?;
    if reference.get(RWE_LAYOUT_START..END_FILE_OFFSET)!=Some(code.as_slice()){
        return Err("CDSP RWE prefix through atomic counter differs".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(x:u8)->G{G::new(x).unwrap()}
    #[test]fn atomic_counter_operands_and_branches_are_checked(){
        assert_eq!(encode_counter_init_pointer(r(3),r(2),0xc0363e00).unwrap(),[0x0c03_58f8,0x9b82_5003]);
        assert_eq!(encode_counter_empty_branch(0x96cc8464,0x96cc849c).unwrap(),0x2402_c01c);
        assert_eq!(encode_minus_one(r(4),-1).unwrap(),0x78df_ffe4);
        assert_eq!(encode_atomic_load(r(3),r(2)).unwrap(),0x9202_c003);
        assert_eq!(encode_atomic_store(r(2),r(4)).unwrap(),0xa0a2_c400);
        assert_eq!(encode_retry(0x96cc8480,0x96cc8474).unwrap(),0x5cff_e0fa);
        assert_eq!(encode_if_not_new_set_one(r(3),1).unwrap(),0x7e80_6023);
        assert_eq!(encode_signed_counter_second_loop(r(3),-1,0x96cc8484,0x96cc84a0).unwrap(),0x11c3_c10e);
        assert_eq!(encode_indexed_counter_read(r(2),r(3),0xc0363e08).unwrap(),[0x0c03_58f8,0x9d83_7202]);
        assert!(encode_retry(0x96cc8480,0x96cc8478).is_err());
        assert!(encode_atomic_load(r(5),r(2)).is_err());
    }
    #[test]fn full_counter_and_layout_boundaries(){
        assert_eq!(build_atomic_counter().unwrap().len(),EXECUTABLE_LEN);
        assert_eq!(build_rwe_prefix_through_atomic().unwrap().len(),0x4b8);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1176);
        assert_eq!(RWE_LAYOUT_NOP_LEN,32);
    }
}
