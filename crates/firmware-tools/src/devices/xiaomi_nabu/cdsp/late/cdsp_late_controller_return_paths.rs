// SPDX-License-Identifier: GPL-3.0-only
//! Restore CDSP controller frame and tail-call the common bit-plane helper
//! on the normal and zero-plane return paths. The original V66 instructions
//! and duplex pair restores are encoded from checked register/memory fields.
use super::cdsp_boot::{GeneralRegister as G,ScalarPacket as S,PacketMarker as M};
use super::cdsp_late_controller_secondary_completion;
use super::cdsp_late_runtime::encode_deallocframe;
use super::cdsp_boot;

pub const FILE_OFFSET:usize=0x56848;
pub const END_FILE_OFFSET:usize=0x56890;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8848;
pub const RWE_START:usize=cdsp_late_controller_secondary_completion::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_secondary_completion::RWE_PREFIX_LAYOUT_LEN;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_secondary_completion::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
fn r(i:u8)->Result<G,String>{G::new(i)}
fn emit(o:&mut Vec<u8>,w:u32){o.extend_from_slice(&w.to_le_bytes());}

pub fn encode_return_result_pair(low:G,high:u8,lowest:u8)->Result<u32,String>{
    if (low.number(),high,lowest)!=(0,0,1){return Err("controller return requires R1:0=combine(0,1)".into());}
    Ok(0x7c00_e000 | u32::from(low.number()))
}

/// V66 duplex load of R2 from R16 and R17:16 from FP+40. Verified with
/// independently assembled LLVM6 code for these exact registers and sizes.
pub fn encode_entry_duplex(value:G,word_base:G,pair_low:G,fp:G,frame_offset:u8)->Result<u32,String>{
    if (value.number(),word_base.number(),pair_low.number(),fp.number(),frame_offset)!=(2,16,16,29,40){
        return Err("unsupported controller success-frame load tuple".into());
    }
    Ok(0x0080_3e2c | (u32::from(value.number())<<16))
}

/// Duplex paired frame loads: for each two-register step the original pair
/// position advances 2 GPRs and the memory offsets decrease by eight.
pub fn encode_restore_adjacent_pairs(fp:G,first:G,second:G,first_offset:u8,second_offset:u8)->Result<u32,String>{
    if fp.number()!=29 || first.number()%2!=0 || first.number()<16 || first.number()>20 ||
       second.number()!=first.number()+2 {return Err("invalid frame double-restore registers".into());}
    let index=(first.number()-16)/2;
    if first_offset!=40-index*8 || second_offset!=32-index*8{
        return Err("controller FP pair restore uses unexpected offsets".into());
    }
    Ok(0x3e2c_1e25 - u32::from(index)*0x0007_0007)
}

pub fn encode_stack_pair_restore(low:G,base:G,offset:u8)->Result<u32,String>{
    if base.number()!=29 || ![(22,16),(24,8),(26,0)].contains(&(low.number(),offset)){
        return Err("unsupported controller stack-pair restore".into());
    }
    Ok(0x91dd_c000 | (u32::from(offset)/8<<5) | u32::from(low.number()))
}

/// Hexagon direct call to the shared bitplane helper from the already
/// validated success path. End-of-packet marker is applied by the caller.
pub fn encode_bitplane_helper_call(from:u32,target:u32)->Result<u32,String>{
    if (from,target)!=(0x96cc_8848,0x96dc_0aac){return Err("unverified bitplane helper source/target".into());}
    let d=target.checked_sub(from).ok_or("bitplane helper call must be forward")?;
    if d&1!=0||d>0x1fff_fe{return Err("helper call distance out of range".into());}
    Ok(0x5a00_4000|(((d>>15)&0x3f)<<16)|((d>>1)&0x3fff))
}

pub fn encode_bitplane_tail_jump(from:u32,target:u32)->Result<u32,String>{
    if ![(0x96cc_8864,0x96dc_0aac),(0x96cc_8888,0x96dc_0aac)].contains(&(from,target)){
        return Err("unsupported bitplane tail call source/target".into());
    }
    let d=target.checked_sub(from).ok_or("tail jump is not forward")?;
    if d&1!=0 || d>0x1fff_fe{return Err("Hexagon tail jump offset is not encodable".into());}
    Ok(0x5800_4000 | (((d>>15)&0x3f)<<16) | ((d>>1)&0x3fff))
}

pub fn build_return_paths()->Result<Vec<u8>,String>{
    let mut v=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut v,M::Final.apply(encode_bitplane_helper_call(0x96cc_8848,0x96dc_0aac)?));
    emit(&mut v,M::ContinuationA.apply(encode_return_result_pair(r(0)?,0,1)?));
    emit(&mut v,encode_entry_duplex(r(2)?,r(16)?,r(16)?,r(29)?,40)?);
    emit(&mut v,M::ContinuationA.apply(S::ArithmeticShiftLeft{dst:r(2)?,src:r(2)?,amount:12}.encode()?));
    emit(&mut v,encode_restore_adjacent_pairs(r(29)?,r(18)?,r(20)?,32,24)?);
    emit(&mut v,M::ContinuationA.apply(encode_stack_pair_restore(r(22)?,r(29)?,16)?));
    emit(&mut v,encode_stack_pair_restore(r(24)?,r(29)?,8)?);
    emit(&mut v,encode_bitplane_tail_jump(0x96cc_8864,0x96dc_0aac)?);
    emit(&mut v,M::ContinuationA.apply(encode_stack_pair_restore(r(26)?,r(29)?,0)?));
    emit(&mut v,encode_deallocframe());
    emit(&mut v,M::ContinuationA.apply(encode_return_result_pair(r(0)?,0,1)?));
    emit(&mut v,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(2)?,value:0}.encode()?));
    emit(&mut v,encode_restore_adjacent_pairs(r(29)?,r(16)?,r(18)?,40,32)?);
    emit(&mut v,encode_restore_adjacent_pairs(r(29)?,r(20)?,r(22)?,24,16)?);
    emit(&mut v,M::ContinuationA.apply(encode_stack_pair_restore(r(24)?,r(29)?,8)?));
    emit(&mut v,encode_stack_pair_restore(r(26)?,r(29)?,0)?);
    emit(&mut v,encode_bitplane_tail_jump(0x96cc_8888,0x96dc_0aac)?);
    emit(&mut v,encode_deallocframe());
    if v.len()!=EXECUTABLE_LEN{return Err("controller return paths must span exactly 72 bytes".into());}
    Ok(v)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut prefix=cdsp_late_controller_secondary_completion::build_prefix()?;
    prefix.extend(build_return_paths()?);
    if prefix.len()!=FULL_RWE_LAYOUT_LEN || prefix.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("invalid controller return source/layout accounting".into());
    }
    Ok(prefix)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_return_paths()?.as_slice()){
        return Err("controller return paths differ from pinned CDSP".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined return-path RWE prefix differs from pinned CDSP".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_return_paths_validate_registers_and_offsets(){
        let b=build_return_paths().unwrap();assert_eq!(b.len(),72);
        let words:Vec<u32>=b.chunks_exact(4).map(|x|u32::from_le_bytes(x.try_into().unwrap())).collect();
        assert_eq!(words[0],0x5a1f_c132);
        assert_eq!(words[2],0x0082_3e2c);
        assert_eq!(words[4],0x3e25_1e1e);
        assert_eq!(words[9],0x901e_c01e);
        assert_eq!(words[12],0x3e2c_1e25);
        assert_eq!(words[16],0x581f_4112);
        assert!(encode_stack_pair_restore(r(28).unwrap(),r(29).unwrap(),0).is_err());
        assert!(encode_bitplane_tail_jump(0x96cc_8868,0x96dc_0aac).is_err());
    }
    #[test]fn controller_returns_extend_exact_rwe_layout(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,0x890);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2148);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,44);
        assert_eq!(build_prefix().unwrap().len(),0x890);
    }
}
