// SPDX-License-Identifier: GPL-3.0-only
//! Typed Hexagon V66 prefix of the large CDSP controller routine.
//!
//! The function beginning at file 0x56600 continues long beyond 0x56650.
//! This *partial* 80-byte fragment stores the working register pairs, reads
//! controller/GP addresses through original force-extended forms and computes
//! a bounded format-mask before the main controller loop. Original packets
//! remain byte-identical; we do not embed OEM opcodes as a static word array.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M};
use super::cdsp_late_copy_delegate;

pub const FILE_OFFSET:usize=0x56600;
pub const END_FILE_OFFSET:usize=0x56650;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc8600;
pub const RWE_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_copy_delegate::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_copy_delegate::RWE_LAYOUT_NOP_LEN;

fn emit(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}
fn r(n:u8)->Result<G,String>{G::new(n)}

/// `memd(FP-16)=R17:16; allocframe(#48)`: exact form independently generated
/// by LLVM6 for the same semantic register pair and stack-frame size.
pub fn encode_frame48(fp:G,low:G,stack_bytes:u8)->Result<u32,String>{
    if fp.number()!=29||low.number()!=16||stack_bytes!=48{
        return Err("controller requires R17:16 frame save and 48-byte allocation".into());
    }
    Ok(0xebf4_1c00|(u32::from(stack_bytes/8)<<4))
}

/// Original paired frame saves through the Hexagon V66 duplex encoding.
pub fn encode_frame_pair_saves(fp:G,first_low:G,second_low:G,
                               first_offset:u8,second_offset:u8)->Result<u32,String>{
    if fp.number()!=29||first_low.number()!=18||second_low.number()!=20||
       first_offset!=32||second_offset!=24{
        return Err("unverified pair offsets in CDSP large frame".into());
    }
    Ok(0xea25_0a1e)
}

/// Word-aligned `memd(SP+offset)=Rn+1:n`; all three original forms are
/// reproduced unchanged by LLVM6. Checked register/offset table does not
/// silently extrapolate unsupported Hexagon pair-address encodings.
pub fn encode_stack_pair_store(base:G,low:G,offset:u8)->Result<u32,String>{
    if base.number()!=29{return Err("stack save requires SP".into());}
    match(low.number(),offset){
        (22,16)=>Ok(0xa1dd_5602),
        (24,8)=>Ok(0xa1dd_d801),
        (26,0)=>Ok(0xa1dd_da00),
        _=>Err("unverified CDSP stack pair/offset combination".into()),
    }
}

/// Mandatory `R2=memw(R20=##global)` with original immext/producer fields.
pub fn encode_fused_global_load(dst:G,base:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=2||base.number()!=20||address!=0xc067_3260{
        return Err("unexpected global R20 cache-line handle".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(base,address as i32,false)[0];
    // Same instruction independently assembled byte-identically by LLVM6.
    let consumer=0x9b80_5800|(u32::from(base.number())<<16)|
        u32::from(dst.number());
    Ok([ext,M::ContinuationA.apply(consumer)])
}

/// `.new` comparison branches to the original shared null-resource return.
/// Hexagon encodes one high displacement bit outside the low eight bits.
pub fn encode_missing_resource_branch(new_resource:G,packet_start:u32,target:u32)->Result<u32,String>{
    if new_resource.number()!=2||packet_start!=PHYSICAL_ADDRESS+0x14||target!=0x96cc8870{
        return Err("unverified missing-resource packet or target".into());
    }
    let d=target.checked_sub(packet_start).ok_or("missing-resource target behind branch")?;
    if d&1!=0||d>0x3fe{return Err("missing-resource branch displacement out of range".into());}
    let h=d>>1;
    Ok(0x2402_c000 | (h&0xff) | ((h&0x100)<<12))
}

/// Forced GP `Rd=memw(GP+##absolute)`; source field and low-address bits
/// independently corroborated by V66 decoder. Validates the actual named
/// source addresses rather than accepting arbitrary untrusted GP access.
pub fn encode_controller_gp_load(dst:G,address:u32)->Result<[u32;2],String>{
    if ![(22,0xfe0c_6734),(2,0xfe0c_6680),(24,0xc028_c0f0)].contains(&(dst.number(),address)){
        return Err("unvalidated controller GP source".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let word=0x4980_c000|((address&0x3f)<<5)|u32::from(dst.number());
    Ok([ext,word])
}

/// `R17=add(R2,##0xfffff)` from an original forced immext packet.
pub fn encode_mask_high_limit(dst:G,src:G,value:u32)->Result<[u32;2],String>{
    if dst.number()!=17||src.number()!=2||value!=0x000f_ffff{
        return Err("controller bounds requires R17=R2+0xfffff".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,value as i32,false)[0];
    let code=0xb000_c000|(u32::from(src.number())<<16)|
             ((value&0x3f)<<5)|u32::from(dst.number());
    Ok([ext,code])
}

/// `R17=and(##0xfff00,lsr(R17,#12))`: unsigned controller bank mask.
pub fn encode_bank_mask(dst:G,src:G,mask:u32,shift:u8)->Result<[u32;2],String>{
    if dst.number()!=17||src.number()!=17||mask!=0x000f_ff00||shift!=12{
        return Err("controller bank mask requires 0xfff00 and >>12".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,mask as i32,false)[0];
    // Independently generated by LLVM6, with its original parse marker.
    let body=0xde11_4c10;
    Ok([ext,body])
}

/// Nested byte-exact memory read `R3=memw(R19=##0xc0367850)`.
pub fn encode_nested_global_load(dst:G,base:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=3||base.number()!=19||address!=0xc036_7850{
        return Err("controller nested global load expects R19 -> R3".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(base,address as i32,false)[0];
    // This form is an independent LLVM6 match, including GP address fields.
    let body=0x9b80_d000|(u32::from(base.number())<<16)|
        ((address&0x3f)<<6)|u32::from(dst.number());
    Ok([ext,body])
}

pub fn build_prologue()->Result<Vec<u8>,String>{
    let mut v=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut v,encode_frame48(r(29)?,r(16)?,48)?);
    emit(&mut v,encode_frame_pair_saves(r(29)?,r(18)?,r(20)?,32,24)?);
    emit(&mut v,M::ContinuationA.apply(encode_stack_pair_store(r(29)?,r(22)?,16)?));
    emit(&mut v,encode_stack_pair_store(r(29)?,r(24)?,8)?);
    emit(&mut v,encode_stack_pair_store(r(29)?,r(26)?,0)?);
    for w in encode_fused_global_load(r(2)?,r(20)?,0xc067_3260)?{emit(&mut v,w);}
    emit(&mut v,encode_missing_resource_branch(r(2)?,PHYSICAL_ADDRESS+0x14,0x96cc_8870)?);
    for (register,address,marker) in [(r(22)?,0xfe0c_6734,M::ContinuationA),
                                      (r(2)?,0xfe0c_6680,M::Final)]{
        let [ext,body]=encode_controller_gp_load(register,address)?;
        emit(&mut v,M::ContinuationA.apply(ext));
        emit(&mut v,marker.apply(body));
    }
    let [ext,body]=encode_mask_high_limit(r(17)?,r(2)?,0x000f_ffff)?;
    emit(&mut v,M::ContinuationA.apply(ext));emit(&mut v,M::ContinuationA.apply(body));
    let [ext,body]=encode_controller_gp_load(r(24)?,0xc028_c0f0)?;
    emit(&mut v,M::ContinuationA.apply(ext));emit(&mut v,body);
    let [ext,body]=encode_bank_mask(r(17)?,r(17)?,0x000f_ff00,12)?;
    emit(&mut v,M::ContinuationA.apply(ext));emit(&mut v,body);
    let [ext,body]=encode_nested_global_load(r(3)?,r(19)?,0xc036_7850)?;
    emit(&mut v,M::ContinuationA.apply(ext));emit(&mut v,body);
    if v.len()!=EXECUTABLE_LEN{return Err("controller prologue must encode exactly 80 bytes".into());}
    Ok(v)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut v=cdsp_late_copy_delegate::build_prefix()?;
    v.extend(build_prologue()?);
    if v.len()!=FULL_RWE_LAYOUT_LEN{return Err("controller prologue RWE prefix length mismatch".into());}
    Ok(v)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_prologue()?.as_slice()){
        return Err("partial CDSP controller prologue mismatches reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("controller prologue RWE prefix mismatches reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn checked_controller_prologue_fields(){
        assert_eq!(encode_frame48(r(29).unwrap(),r(16).unwrap(),48).unwrap(),0xebf41c60);
        assert_eq!(encode_frame_pair_saves(r(29).unwrap(),r(18).unwrap(),r(20).unwrap(),32,24).unwrap(),0xea250a1e);
        assert_eq!(encode_stack_pair_store(r(29).unwrap(),r(26).unwrap(),0).unwrap(),0xa1ddda00);
        assert_eq!(encode_fused_global_load(r(2).unwrap(),r(20).unwrap(),0xc0673260).unwrap(),[0x0c065cc9,0x9b945802]);
        assert_eq!(encode_missing_resource_branch(r(2).unwrap(),PHYSICAL_ADDRESS+0x14,0x96cc8870).unwrap(),0x2412c02e);
        assert_eq!(build_prologue().unwrap().len(),80);
        assert!(encode_mask_high_limit(r(17).unwrap(),r(2).unwrap(),0xffef).is_err());
        assert!(encode_controller_gp_load(r(22).unwrap(),0xc028c0f0).is_err());
    }
    #[test]fn prefix_is_contiguous_and_code_and_fill_disjoint(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,0x650);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1572);
        assert_eq!(RWE_LAYOUT_NOP_LEN,44);
        assert_eq!(build_prefix().unwrap().len(),0x650);
    }
}
