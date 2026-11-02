// SPDX-License-Identifier: GPL-3.0-only
//! CDSP Hexagon V66 callback completion and 8-byte alignment, file 0x562c8..0x56320.
//!
//! The callable wrapper `0x562c8..0x56318` consists of 80 source-derived
//! instruction bytes; the two four-byte NOP words `0x56318..0x56320` are
//! **layout**, not independently reconstructed controller functionality.
//! Original hexagon packet ordering matters: LLVM6 reorders the first call
//! and compresses the GP load. Rizin independently checks original V66
//! packet ordering and the source emitter is compared with private CDSP MBN.
use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M, ScalarPacket as S};
use super::cdsp_late_callback_poll::{self,encode_short_backward_call};

pub const FILE_OFFSET:usize=0x562c8;
pub const PHYSICAL_ADDRESS:u32=0x96cc_82c8;
pub const EXECUTABLE_LEN:usize=80;
pub const ALIGNMENT_NOP_LEN:usize=8;
pub const RWE_PREFIX_START:usize=0x56000;
pub const RWE_PREFIX_END:usize=FILE_OFFSET+EXECUTABLE_LEN+ALIGNMENT_NOP_LEN; // 0x56320
pub const RWE_LAYOUT_LEN:usize=RWE_PREFIX_END-RWE_PREFIX_START; // 800
pub const RWE_EXECUTABLE_LEN:usize=cdsp_late_callback_poll::RWE_EXECUTABLE_LEN+EXECUTABLE_LEN; // 768
pub const RWE_FILL_BYTES:usize=RWE_LAYOUT_LEN-RWE_EXECUTABLE_LEN; // 32

fn push(output:&mut Vec<u8>,value:u32){output.extend_from_slice(&value.to_le_bytes());}

/// Hexagon signed small immediate literal `R0 = #-2` in an existing packet.
/// LLVM6 produces the same instruction word but schedules it after a call in
/// a different packet. The packet parse marker is supplied explicitly.
pub fn encode_minus_two(dst:G,value:i32,marker:M)->Result<u32,String>{
    if dst.number()!=0 || value!=-2{return Err("Only independently tested R0=-2 is supported".into());}
    let signed=(value as u32)&0x1ff;
    Ok(marker.apply(0x78df_c000 | (signed<<5) |u32::from(dst.number())))
}

/// `memd(FP+#-16)=R17:16; allocframe(#8)`, independently LLVM6-assembled.
/// Rizin preserves the original *duplex* packing as one Hexagon word.
pub fn encode_save_pair_and_allocframe8(base:G,low:G,frame:u8)->Result<u32,String>{
    if base.number()!=29 ||low.number()!=16 ||frame!=8{return Err("RWE completion expects FP, R17:16 and frame 8".into());}
    Ok(0xebf4_1c00|(u32::from(frame)*2))
}

/// `R0=##0xfffe; R16/17=add(R0,#1)` in one Hexagon duplex, preceded by
/// a checked architecture immext. Source-assembled in LLVM6 for both R16/17.
pub fn encode_extended_negative_one_pair(result:G,src:G,value:u16)->Result<[u32;2],String>{
    if !(16..=17).contains(&result.number()) ||src.number()!=0 ||value!=0xfffe {
        return Err("Checked paired -2 and increment needs R0, result R16/R17 and #0xfffe".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(src,i32::from(value),false)[0];
    let low=u32::from(result.number()-16);
    Ok([ext,0x2be0_3108|low])
}

/// `R1:0=combine(R17,R16)` with the independently LLVM6-assembled register
/// selection. This narrow form is explicit about both physical operands.
pub fn encode_combine_pair(low:G,high:G,other:G)->Result<u32,String>{
    if low.number()!=0 ||high.number()!=17 ||other.number()!=16{return Err("Only tested R1:0=combine(R17,R16)".into());}
    Ok(0xf500_d000|(u32::from(high.number())<<16))
}

/// `R17:16=memd(FP+#0); deallocframe`, an original continuation packet.
pub fn encode_restore_pair_dealloc(low:G,base:G,offset:u8)->Result<u32,String>{
    if low.number()!=16 || base.number()!=29 || offset!=0{return Err("Invalid CDSP final pair/deallocframe operands".into());}
    Ok(0x3e04_1f00)
}

/// Direct Hexagon call with narrow checked forward physical displacement,
/// from first packet instruction address (not consumer position).
pub fn encode_forward_call(start:u32,target:u32)->Result<u32,String>{
    let delta=target.checked_sub(start).ok_or("CDSP call target behind packet start")?;
    if start!=PHYSICAL_ADDRESS+0x20 || target!=0x96cc_a640 ||delta!=0x2358 ||delta&1!=0 {
        return Err("Unsupported CDSP next callback direct-call destination".into());
    }
    Ok(0x5a00_4000|(delta/2))
}

/// Direct non-conditional jump to firmware continuation `0x96ccc7a0`.
/// The 0x448c-byte forward displacement occupies Hexagon split jump fields.
pub fn encode_forward_jump(start:u32,target:u32)->Result<u32,String>{
    let d=target.checked_sub(start).ok_or("CDSP jump target behind packet start")?;
    if start!=PHYSICAL_ADDRESS+0x4c || target!=0x96cc_c7a0 ||d!=0x448c ||d&1!=0 {
        return Err("Unsupported CDSP callback continuation jump".into());
    }
    Ok(0x5800_c000 |d/2)
}

/// Recreate the 20 executable instruction words through semantic operands
/// and explicit packet parse markers. No reference bytes are consulted here.
pub fn build_completion_wrapper()->Result<Vec<u8>,String>{
    let r=|index|G::new(index);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    push(&mut out,encode_minus_two(r(0)?,-2,M::ContinuationA)?);
    push(&mut out,encode_save_pair_and_allocframe8(r(29)?,r(16)?,8)?);
    push(&mut out,encode_short_backward_call(PHYSICAL_ADDRESS+8,0x96cc_8230)?);
    for w in cdsp_boot::encode_extended_gp_word_load(r(16)?,0xc028_c538) {push(&mut out,w);}
    push(&mut out,encode_short_backward_call(PHYSICAL_ADDRESS+0x14,0x96cc_8230)?);
    for w in encode_extended_negative_one_pair(r(17)?,r(0)?,0xfffe)? {push(&mut out,w);}
    push(&mut out,encode_forward_call(PHYSICAL_ADDRESS+0x20,0x96cc_a640)?);
    push(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(2)?,src:r(0)?,value:1}.encode()?));
    push(&mut out,encode_combine_pair(r(0)?,r(17)?,r(16)?)?);
    push(&mut out,encode_short_backward_call(PHYSICAL_ADDRESS+0x2c,0x96cc_8230)?);
    push(&mut out,encode_minus_two(r(0)?,-2,M::Final)?);
    push(&mut out,encode_short_backward_call(PHYSICAL_ADDRESS+0x34,0x96cc_8230)?);
    for w in encode_extended_negative_one_pair(r(16)?,r(0)?,0xfffe)?{push(&mut out,w);}
    push(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(1)?,src:r(0)?,value:1}.encode()?));
    push(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(0)?,src:r(16)?}.encode()?));
    push(&mut out,encode_restore_pair_dealloc(r(16)?,r(29)?,0)?);
    push(&mut out,encode_forward_jump(PHYSICAL_ADDRESS+0x4c,0x96cc_c7a0)?);
    if out.len()!=EXECUTABLE_LEN{return Err(format!("completion wrapper expected 80 bytes, emitted {}",out.len()));}
    Ok(out)
}

/// Strict file-oriented RWE prefix with explicit architecture NOP alignment.
/// These eight bytes are *not* counted as source-derived executable logic.
pub fn build_prefix_with_completion()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_callback_poll::build_rwe_prefix_through_poll()?;
    out.extend(build_completion_wrapper()?);
    push(&mut out,S::Nop.encode()?);
    push(&mut out,S::Nop.encode()?);
    if out.len()!=RWE_LAYOUT_LEN{return Err("CDSP callback completion prefix length mismatch".into());}
    Ok(out)
}

pub fn verify_completion_wrapper(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let generated=build_completion_wrapper()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+EXECUTABLE_LEN)!=Some(generated.as_slice()) {
        return Err("CDSP completion wrapper differs from pinned reference".into());
    }
    Ok(())
}

pub fn verify_prefix_with_completion(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let prefix=build_prefix_with_completion()?;
    if reference.get(RWE_PREFIX_START..RWE_PREFIX_END)!=Some(prefix.as_slice()){
        return Err("CDSP RWE prefix through callback completion differs".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]fn packet_fields_and_source_operands_are_checked(){
        assert_eq!(encode_minus_two(r(0),-2,M::ContinuationA).unwrap(),0x78df_7fc0);
        assert_eq!(encode_minus_two(r(0),-2,M::Final).unwrap(),0x78df_ffc0);
        assert!(encode_minus_two(r(1),-2,M::Final).is_err());
        assert_eq!(encode_save_pair_and_allocframe8(r(29),r(16),8).unwrap(),0xebf4_1c10);
        assert!(encode_save_pair_and_allocframe8(r(29),r(16),16).is_err());
        assert_eq!(encode_extended_negative_one_pair(r(17),r(0),0xfffe).unwrap(),[0x0000_43ff,0x2be0_3109]);
        assert_eq!(encode_extended_negative_one_pair(r(16),r(0),0xfffe).unwrap(),[0x0000_43ff,0x2be0_3108]);
        assert_eq!(encode_combine_pair(r(0),r(17),r(16)).unwrap(),0xf511_d000);
        assert!(encode_combine_pair(r(0),r(17),r(15)).is_err());
        assert_eq!(encode_forward_call(0x96cc_82e8,0x96cc_a640).unwrap(),0x5a00_51ac);
        assert_eq!(encode_forward_jump(0x96cc_8314,0x96cc_c7a0).unwrap(),0x5800_e246);
        assert!(encode_forward_jump(0x96cc_8314,0x96cc_c7a4).is_err());
    }
    #[test]fn callback_completion_is_full_and_nop_layout_is_separate(){
        assert_eq!(build_completion_wrapper().unwrap().len(),80);
        assert_eq!(build_prefix_with_completion().unwrap().len(),0x320);
        assert_eq!(RWE_EXECUTABLE_LEN,768);
        assert_eq!(RWE_FILL_BYTES,32);
        assert_eq!(&build_prefix_with_completion().unwrap()[0x318..0x320],&[0x00,0xc0,0x00,0x7f,0x00,0xc0,0x00,0x7f]);
    }
}
