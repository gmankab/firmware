// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 controller seed/state update (shift/XOR/insert), file 0x563c0..0x56448.
//!
//! This source-based 136-byte routine tail completes the seed-state function
//! begun at file 0x56320. The original Hexagon V66 packet order differs from
//! LLVM6 in multi-operation packets; each checked encoded instruction is
//! separately corroborated with LLVM6 semantics and Rizin's V66 decoder.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_rng_seed_state;

pub const FILE_OFFSET:usize=0x563c0;
pub const END_FILE_OFFSET:usize=0x56448;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_83c0;
pub const RWE_LAYOUT_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_LAYOUT_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_rng_seed_state::PREFIX_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_rng_seed_state::PREFIX_LAYOUT_NOP_LEN;

fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// Rdst ^= asl(Rsrc,#bits), independently LLVM6 assembled. The source also
/// works when src=dst; the instruction uses one accumulator register.
pub fn encode_xor_shift(dst:G,src:G,bits:u8,marker:M)->Result<u32,String>{
    if ![(2,2,3),(3,3,13),(4,4,2)].contains(&(dst.number(),src.number(),bits)){
        return Err("Unsupported CDSP xorshift register/shift encoding".into());
    }
    let word=0x8e80_c040|(u32::from(src.number())<<16)|(u32::from(bits)<<8)|u32::from(dst.number());
    Ok(marker.apply(word))
}

/// Rdst=asl(Rsrc,#N), from the validated Hexagon scalar instruction class.
pub fn encode_asl(dst:G,src:G,bits:u8,marker:M)->Result<u32,String>{
    if ![(6,2,17),(24,3,12)].contains(&(dst.number(),src.number(),bits)){
        return Err("Unsupported CDSP RNG left-shift operands".into());
    }
    Ok(marker.apply(S::ArithmeticShiftLeft{dst,src,amount:bits}.encode()?))
}

/// Rdst=lsr(Rsrc,#N), used for xorshift state mixing.
pub fn encode_lsr(dst:G,src:G,bits:u8,marker:M)->Result<u32,String>{
    if ![(3,3,19),(2,2,11),(2,4,25)].contains(&(dst.number(),src.number(),bits)){
        return Err("Unsupported CDSP RNG right-shift operands".into());
    }
    Ok(marker.apply(S::LogicalShiftRight{dst,src,amount:bits}.encode()?))
}

/// One Hexagon duplex scalar pair `R3=memw(R19+#0); R4=memw(R18+#0)`.
/// Both register and offset fields were checked in LLVM6 semantic assembly.
pub fn encode_two_gp_indirect_loads(dst0:G,base0:G,dst1:G,base1:G)->Result<u32,String>{
    if (dst0.number(),base0.number(),dst1.number(),base1.number())!=(3,19,4,18){
        return Err("Only original R3/R4 seed pair loads from R19/R18 are supported".into());
    }
    Ok(0x00b3_00a4)
}

/// `R5=#-128` is the original V66 signed word-immediate form.
pub fn encode_signed_minus_128(dst:G,value:i32)->Result<u32,String>{
    if dst.number()!=5||value!=-128{return Err("Seed hash mask must be R5=-128".into());}
    Ok(0x78df_c000|(((value as u32)&0x1ff)<<5)|u32::from(dst.number()))
}

/// `R5 &= asl(R4,#4)`, one of the paired seed-mix packet operations.
pub fn encode_and_asl(dst:G,src:G,bits:u8)->Result<u32,String>{
    if dst.number()!=5||src.number()!=4||bits!=4{return Err("Seed mask AND-ASL expected R5 & (R4 << 4)".into());}
    Ok(M::ContinuationA.apply(0x8e40_c040|(u32::from(src.number())<<16)|(u32::from(bits)<<8)|u32::from(dst.number())))
}

/// `R24 ^= xor(R6,R22)`, V66 three-register compound xor class.
pub fn encode_xor_xor(dst:G,left:G,right:G)->Result<u32,String>{
    if (dst.number(),left.number(),right.number())!=(24,6,22){
        return Err("Expected seed mixing R24 ^= xor(R6,R22)".into());
    }
    Ok(M::ContinuationA.apply(0xef80_c060|(u32::from(left.number())<<16)|
                              (u32::from(right.number())<<8)|u32::from(dst.number())))
}

/// `R24 ^= or(R2,R5)`, V66 accumulator OR composition.
pub fn encode_xor_or(dst:G,left:G,right:G)->Result<u32,String>{
    if (dst.number(),left.number(),right.number())!=(24,2,5){
        return Err("Expected seed mixing R24 ^= or(R2,R5)".into());
    }
    Ok(M::ContinuationA.apply(0xefc0_c060|(u32::from(left.number())<<16)|
                              (u32::from(right.number())<<8)|u32::from(dst.number())))
}

/// Seed words are written from a just-computed `.new` value. The implicit
/// source distance, rather than register number, is encoded by the instruction.
pub fn encode_seed_store_new(base:G,src:G,distance:u8)->Result<u32,String>{
    if ![(19,24,1),(20,6,3),(18,3,2)].contains(&(base.number(),src.number(),distance)){
        return Err("Unsupported CDSP RNG V66 new-value store register/distance".into());
    }
    Ok(0xa1a0_d000|(u32::from(base.number())<<16)|(u32::from(distance)<<9))
}

/// Checked split-field long call into CDSP shared services. The first call
/// is final in its packet; neither call encodes a raw absolute target.
pub fn encode_long_service_call(start:u32,target:u32)->Result<u32,String>{
    let delta=target.checked_sub(start).ok_or("CDSP seed service target below call site")?;
    if ![(0x96cc_8418,0x96dc_0680),(0x96cc_8434,0x96dc_4acc)].contains(&(start,target))||
       delta&1!=0 ||delta>0x1fff_fe {
        return Err("Unverified seed-state helper target/displacement".into());
    }
    let word=0x5a00_4000|(((delta>>15)&0x3f)<<16)|((delta/2)&0x3fff);
    Ok(M::Final.apply(word))
}

/// Packet-relative backwards P0.new branch; producer P0 comparison begins
/// at physical `0x96cc841c` and target is the earlier timestamp call.
pub fn encode_seed_loop_backedge(packet_start:u32,target:u32)->Result<u32,String>{
    let delta=(target as i64)-(packet_start as i64);
    if packet_start!=0x96cc_841c||target!=0x96cc_8368||delta>=0||delta< -0x200||delta&1!=0{
        return Err("Seed loop requires proven 0x5641c -> 0x56368 backedge".into());
    }
    Ok(0x5cdf_7800|(((delta/2) as i32 as u32)&0xff))
}

/// Conditionally publish R24 seed result to caller's SP+4 slot.
pub fn encode_if_not_new_p0_stack_store(base:G,src:G,offset:u8)->Result<u32,String>{
    if base.number()!=29||src.number()!=24||offset!=4{
        return Err("CDSP RNG failure status expects SP+4=R24".into());
    }
    Ok(0x469d_d000|(u32::from(offset)<<9)|u32::from(src.number()-16))
}

/// `R3:2=combine(R16,#0x53)` immediate split fields.
pub fn encode_combined_service_args(low:G,src:G,immediate:u8)->Result<u32,String>{
    if low.number()!=2||src.number()!=16||immediate!=0x53{
        return Err("Expected R3:2=combine(R16,#0x53)".into());
    }
    Ok(M::ContinuationA.apply(0x7300_e000|(u32::from(src.number())<<16)|
        (u32::from(immediate)<<5)|u32::from(low.number())))
}

/// V66 duplex `R5=#4; R1:0=combine(R17,#0)` used for asynchronous callback.
pub fn encode_callback_duplex(dst:G,low:G,src:G,val:u8)->Result<u32,String>{
    if (dst.number(),low.number(),src.number(),val)!=(5,0,17,4){
        return Err("Seed callback duplex requires R5=4, R1:0=(R17,0)".into());
    }
    Ok(0x2845_3d98)
}

/// Two-word packed restore and deallocation. These are LLVM6-confirmed
/// narrow duplexes for a saved 48-byte frame.
pub fn encode_pair_restore(fp:G,first_low:G,second_low:G)->Result<u32,String>{
    if fp.number()!=29{return Err("Restoring seed frame requires FP=R29".into());}
    match (first_low.number(),second_low.number()){
        (16,18)=>Ok(0x3e2c_1e25),
        (20,22)=>Ok(0x3e1e_1e17),
        _=>Err("Unverified CDSP paired seed-frame restores".into()),
    }
}

/// R25:24 = memd(SP+#8), source packet continues into final dealloc_return.
pub fn encode_restore_r25_pair(low:G,base:G,offset:u8)->Result<u32,String>{
    if low.number()!=24||base.number()!=29||offset!=8{
        return Err("CDSP RNG return restores R25:24 from SP+8".into());
    }
    Ok(M::ContinuationA.apply(0x91dd_c038))
}

/// `LR:FP=dealloc_return(FP)`; checked original V66 return-pair form.
pub fn encode_final_seed_return(link:G,fp:G)->Result<u32,String>{
    if link.number()!=31||fp.number()!=29{return Err("Seed dealloc_return requires LR and FP".into());}
    Ok(0x961e_c01e)
}

/// Emit the remaining 34 words of the seed-state PRNG-style mixing routine,
/// using checked instruction families and original Hexagon packet markers.
pub fn build_seed_update_and_return()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_xor_shift(r(2)?,r(2)?,3,M::ContinuationA)?);
    emit(&mut out,encode_asl(r(6)?,r(2)?,17,M::ContinuationA)?);
    emit(&mut out,encode_two_gp_indirect_loads(r(3)?,r(19)?,r(4)?,r(18)?)?);
    emit(&mut out,encode_xor_shift(r(3)?,r(3)?,13,M::ContinuationA)?);
    emit(&mut out,encode_asl(r(24)?,r(3)?,12,M::ContinuationA)?);
    emit(&mut out,encode_signed_minus_128(r(5)?,-128)?);
    emit(&mut out,encode_lsr(r(3)?,r(3)?,19,M::ContinuationA)?);
    emit(&mut out,encode_lsr(r(2)?,r(2)?,11,M::ContinuationA)?);
    for w in cdsp_boot::encode_extended_assignment(r(0)?,0xc036_4130u32 as i32,true){emit(&mut out,w);}
    emit(&mut out,encode_and_asl(r(5)?,r(4)?,4)?);
    emit(&mut out,M::ContinuationA.apply(S::InsertBitField{dst:r(24)?,src:r(3)?,width:13,offset:0}.encode()?));
    emit(&mut out,encode_seed_store_new(r(19)?,r(24)?,1)?);
    emit(&mut out,M::ContinuationA.apply(S::InsertBitField{dst:r(6)?,src:r(2)?,width:21,offset:0}.encode()?));
    emit(&mut out,encode_xor_shift(r(4)?,r(4)?,2,M::ContinuationA)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(3)?,src:r(5)?}.encode()?));
    emit(&mut out,encode_seed_store_new(r(20)?,r(6)?,3)?);
    emit(&mut out,encode_xor_xor(r(24)?,r(6)?,r(22)?)?);
    emit(&mut out,encode_lsr(r(2)?,r(4)?,25,M::Final)?);
    emit(&mut out,M::ContinuationA.apply(S::InsertBitField{dst:r(3)?,src:r(2)?,width:7,offset:0}.encode()?));
    emit(&mut out,encode_xor_or(r(24)?,r(2)?,r(5)?)?);
    emit(&mut out,encode_seed_store_new(r(18)?,r(3)?,2)?);
    emit(&mut out,encode_long_service_call(0x96cc_8418,0x96dc_0680)?);
    emit(&mut out,M::ContinuationA.apply(S::PredicateP0EqualsZero{src:r(24)?}.encode()?));
    emit(&mut out,encode_seed_loop_backedge(0x96cc_841c,0x96cc_8368)?);
    emit(&mut out,encode_if_not_new_p0_stack_store(r(29)?,r(24)?,4)?);
    emit(&mut out,encode_combined_service_args(r(2)?,r(16)?,0x53)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(4)?,src:r(29)?,value:4}.encode()?));
    emit(&mut out,encode_callback_duplex(r(5)?,r(0)?,r(17)?,4)?);
    emit(&mut out,encode_long_service_call(0x96cc_8434,0x96dc_4acc)?);
    emit(&mut out,encode_pair_restore(r(29)?,r(16)?,r(18)?)?);
    emit(&mut out,encode_pair_restore(r(29)?,r(20)?,r(22)?)?);
    emit(&mut out,encode_restore_r25_pair(r(24)?,r(29)?,8)?);
    emit(&mut out,encode_final_seed_return(r(31)?,r(29)?)?);
    if out.len()!=EXECUTABLE_LEN{return Err(format!("CDSP complete seed update requires 136 bytes, emitted {}",out.len()));}
    Ok(out)
}

/// The full reconstructed RNG-style routine `[0x56320,0x56448)` (296 bytes)
/// and file-layout prefix `[0x56000,0x56448)` containing explicit fill.
pub fn build_full_seed_function()->Result<Vec<u8>,String>{
    let mut out=super::cdsp_late_rng_init::build_seed_prologue()?;
    out.extend(cdsp_late_rng_seed_state::build_seed_initialization()?);
    out.extend(build_seed_update_and_return()?);
    if out.len()!=0x128{return Err("CDSP seed routine did not reach 296-byte total".into());}
    Ok(out)
}

pub fn build_rwe_prefix_through_seed_return()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_rng_seed_state::build_prefix_through_seed_state()?;
    out.extend(build_seed_update_and_return()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN{return Err("CDSP complete RNG RWE prefix size mismatch".into());}
    Ok(out)
}

pub fn verify_seed_update(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let generated=build_seed_update_and_return()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(generated.as_slice()){
        return Err("CDSP seed update/return differs from private pinned source".into());
    }
    Ok(())
}

pub fn verify_full_seed_function(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let full=build_full_seed_function()?;
    if reference.get(0x56320..0x56448)!=Some(full.as_slice()){
        return Err("CDSP seed function is not a complete byte-identical 296-byte routine".into());
    }
    Ok(())
}

pub fn verify_rwe_prefix_through_seed_return(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let prefix=build_rwe_prefix_through_seed_return()?;
    if reference.get(RWE_LAYOUT_START..END_FILE_OFFSET)!=Some(prefix.as_slice()){
        return Err("CDSP RWE prefix through full seed function differs from reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]fn seed_xorshift_and_callback_field_encodings_are_bounded(){
        assert_eq!(encode_xor_shift(r(2),r(2),3,M::ContinuationA).unwrap(),0x8e82_4342);
        assert_eq!(encode_asl(r(6),r(2),17,M::ContinuationA).unwrap(),0x8c02_5146);
        assert_eq!(encode_two_gp_indirect_loads(r(3),r(19),r(4),r(18)).unwrap(),0x00b3_00a4);
        assert_eq!(encode_xor_shift(r(3),r(3),13,M::ContinuationA).unwrap(),0x8e83_4d43);
        assert_eq!(encode_signed_minus_128(r(5),-128).unwrap(),0x78df_f005);
        assert_eq!(encode_seed_store_new(r(19),r(24),1).unwrap(),0xa1b3_d200);
        assert_eq!(encode_seed_store_new(r(20),r(6),3).unwrap(),0xa1b4_d600);
        assert_eq!(encode_xor_xor(r(24),r(6),r(22)).unwrap(),0xef86_5678);
        assert_eq!(encode_xor_or(r(24),r(2),r(5)).unwrap(),0xefc2_4578);
        assert_eq!(encode_seed_loop_backedge(0x96cc841c,0x96cc8368).unwrap(),0x5cdf_78a6);
        assert_eq!(encode_long_service_call(0x96cc8418,0x96dc0680).unwrap(),0x5a1f_c134);
        assert_eq!(encode_long_service_call(0x96cc8434,0x96dc4acc).unwrap(),0x5a1f_e34c);
        assert_eq!(encode_callback_duplex(r(5),r(0),r(17),4).unwrap(),0x2845_3d98);
        assert_eq!(encode_if_not_new_p0_stack_store(r(29),r(24),4).unwrap(),0x469d_d808);
        assert!(encode_if_not_new_p0_stack_store(r(29),r(24),8).is_err());
        assert_eq!(encode_restore_r25_pair(r(24),r(29),8).unwrap(),0x91dd_4038);
        assert_eq!(encode_final_seed_return(r(31),r(29)).unwrap(),0x961e_c01e);
        assert!(encode_seed_store_new(r(19),r(24),2).is_err());
        assert!(encode_long_service_call(0x96cc8418,0x96dc0684).is_err());
    }
    #[test]fn full_seed_function_and_layout_boundaries_are_exact(){
        assert_eq!(build_seed_update_and_return().unwrap().len(),136);
        assert_eq!(build_full_seed_function().unwrap().len(),296);
        assert_eq!(build_rwe_prefix_through_seed_return().unwrap().len(),0x448);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1064);
        assert_eq!(RWE_LAYOUT_NOP_LEN,32);
    }
}
