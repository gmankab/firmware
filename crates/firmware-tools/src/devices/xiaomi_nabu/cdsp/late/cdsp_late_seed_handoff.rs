// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 seeded-controller handoff, file 0x56448..0x56464 (28 bytes).
//!
//! Two semantic Hexagon operands are duplexes, which the independent LLVM6
//! assembler emits byte-identically. The call and extended pair-combine use
//! an original continuation parse marker that LLVM6 schedules differently.
//! No OEM opcode stream or authentication bytes are imported into the emitter.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M};
use super::cdsp_late_runtime::encode_deallocframe;
use super::cdsp_late_rng_update;

pub const FILE_OFFSET:usize=0x56448;
pub const END_FILE_OFFSET:usize=0x56464;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc8448;
pub const RWE_LAYOUT_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_LAYOUT_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_rng_update::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_rng_update::RWE_LAYOUT_NOP_LEN;

fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// The call split is halfword-addressed with a high-six-bit carry at bit 16.
/// The original call at physical 0x96cc8448 targets the global handoff helper
/// at 0x96dc28ac and continues in a packet with the subsequent immext.
pub fn encode_handoff_call(start:u32,target:u32)->Result<u32,String>{
    if (start,target)!=(PHYSICAL_ADDRESS,0x96dc_28ac){
        return Err("CDSP seed handoff only permits validated call and target".into());
    }
    let delta=target.checked_sub(start).ok_or("handoff target before start")?;
    if delta&1!=0||delta>0x1fff_fe{return Err("Hexagon split call displacement out of range".into());}
    let word=0x5a00_4000|(((delta>>15)&0x3f)<<16)|((delta/2)&0x3fff);
    Ok(M::ContinuationA.apply(word))
}

/// Semantic duplex `R0=##0xc0364130; allocframe(#0)`; independent LLVM6
/// produces the exact 32-bit duplex encoding. Deliberately restrict to those
/// two operands instead of guessing unsupported duplex register layouts.
pub fn encode_zero_frame_seed_pointer(dst:G,address:u32,frame_bytes:u16)->Result<u32,String>{
    if dst.number()!=0||address!=0xc036_4130||frame_bytes!=0{
        return Err("Expected R0 seed-state pointer and allocframe zero".into());
    }
    Ok(0x6b00_3c00) // Exact verified duplex from independent semantic assembly.
}

/// One extended `R1:0=combine(#1,##0xc0226160)` with a leading immext.
/// High operand and lower absolute-address fragments have independently
/// demonstrated field placement; preserve original continuation marker.
pub fn encode_extended_pair_combine(low:G,high_immediate:u8,low_address:u32)
    ->Result<[u32;2],String>{
    if low.number()!=0||high_immediate!=1||low_address!=0xc022_6160{
        return Err("Handoff pair requires R1:0=combine(1,0xc0226160)".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(low,low_address as i32,false)[0];
    let body=0x7c80_0000 | (u32::from(high_immediate)<<20) | (low_address & 0x3f);
    Ok([ext,M::ContinuationA.apply(body)])
}

/// Tail branch from the exact physical handoff packet to the shared
/// controller at 0x96cce74c, with the original final-packet parse marker.
pub fn encode_handoff_tail_jump(from:u32,to:u32)->Result<u32,String>{
    if from!=0x96cc_8460||to!=0x96cc_e74c{return Err("Unverified CDSP handoff tail branch".into());}
    let delta=to.checked_sub(from).ok_or("Target before handoff tail")?;
    if delta&1!=0 ||delta>0x1fff_fe{return Err("Tail branch displacement outside encoding".into());}
    let word=0x5800_4000|(((delta>>15)&0x3f)<<16)|((delta/2)&0x3fff);
    Ok(M::Final.apply(word))
}

pub fn build_seed_handoff()->Result<Vec<u8>,String>{
    let r0=G::new(0)?;
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_handoff_call(PHYSICAL_ADDRESS,0x96dc_28ac)?);
    emit(&mut out,cdsp_boot::encode_extended_assignment(r0,0xc036_4130u32 as i32,false)[0]);
    emit(&mut out,encode_zero_frame_seed_pointer(r0,0xc036_4130,0)?);
    for word in encode_extended_pair_combine(r0,1,0xc022_6160)?{emit(&mut out,word);}
    emit(&mut out,encode_deallocframe());
    emit(&mut out,encode_handoff_tail_jump(0x96cc_8460,0x96cc_e74c)?);
    if out.len()!=EXECUTABLE_LEN{return Err("Seed handoff size is not 28 bytes".into());}
    Ok(out)
}

pub fn build_rwe_prefix_through_handoff()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_rng_update::build_rwe_prefix_through_seed_return()?;
    out.extend(build_seed_handoff()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN {return Err("CDSP handoff prefix has wrong length".into());}
    Ok(out)
}

pub fn verify_handoff(image:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(image)?;
    let generated=build_seed_handoff()?;
    if image.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(generated.as_slice()){
        return Err("CDSP seeded handoff differs from pinned original".into());
    }
    Ok(())
}

pub fn verify_prefix(image:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(image)?;
    let generated=build_rwe_prefix_through_handoff()?;
    if image.get(RWE_LAYOUT_START..END_FILE_OFFSET)!=Some(generated.as_slice()){
        return Err("CDSP RWE prefix through seed handoff differs from original".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]fn handoff_packet_fields_are_checked(){
        assert_eq!(encode_handoff_call(0x96cc8448,0x96dc28ac).unwrap(),0x5a1f_5232);
        assert!(encode_handoff_call(0x96cc8448,0x96dc28ae).is_err());
        assert_eq!(encode_zero_frame_seed_pointer(r(0),0xc0364130,0).unwrap(),0x6b00_3c00);
        assert!(encode_zero_frame_seed_pointer(r(0),0xc0364130,8).is_err());
        assert_eq!(encode_extended_pair_combine(r(0),1,0xc0226160).unwrap(),[0x0c02_4985,0x7c90_4020]);
        assert!(encode_extended_pair_combine(r(0),2,0xc0226160).is_err());
        assert_eq!(encode_handoff_tail_jump(0x96cc8460,0x96cce74c).unwrap(),0x5800_f176);
        assert!(encode_handoff_tail_jump(0x96cc8460,0x96cce74e).is_err());
    }
    #[test]fn source_built_handoff_has_exact_layout(){
        assert_eq!(build_seed_handoff().unwrap().len(),28);
        assert_eq!(build_rwe_prefix_through_handoff().unwrap().len(),0x464);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1092);
        assert_eq!(RWE_LAYOUT_NOP_LEN,32);
    }
}
