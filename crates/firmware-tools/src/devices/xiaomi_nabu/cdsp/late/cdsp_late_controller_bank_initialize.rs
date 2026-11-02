// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 CDSP bank enumerator: normalize bank capacity, resolve the
//! controller parameter block, and initialize the helper's stack arguments.
//! Source-generated packet words cover CDSP file [0x56b4c, 0x56b78).
//! Every operand is checked against the independently assembled LLVM6
//! instruction and decoded by Rizin; no reference firmware is used to emit.
use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M, ScalarPacket as S};
use super::cdsp_late_controller_bank_guards;

pub const FILE_OFFSET: usize = 0x56b4c;
pub const END_FILE_OFFSET: usize = 0x56b78;
pub const EXECUTABLE_LEN: usize = END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS: u32 = 0x96cc_8b4c;
pub const RWE_START: usize = cdsp_late_controller_bank_guards::RWE_START;
pub const FULL_RWE_LAYOUT_LEN: usize = END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN: usize =
    cdsp_late_controller_bank_guards::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN: usize =
    cdsp_late_controller_bank_guards::RWE_PREFIX_LAYOUT_LEN;
const CONTROLLER_PARAMETER_BLOCK: u32 = 0xc036_7850;
const CONTROLLER_HELPER: u32 = 0x96cc_eaf0;

fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// Original packet begins `R3=R16`, a normal register move.
pub fn encode_bank_count_argument(dst:G,src:G)->Result<u32,String>{
    if (dst.number(),src.number())!=(3,16){return Err("unverified initial bank-count register copy".into());}
    Ok(M::ContinuationA.apply(S::AssignRegister{dst,src}.encode()?))
}
/// `R0=add(SP,#28)`: the helper receives the caller-owned stack descriptor.
pub fn encode_stack_descriptor_pointer(dst:G,base:G,offset:u16)->Result<u32,String>{
    if (dst.number(),base.number(),offset)!=(0,29,28){return Err("unverified stack argument location".into());}
    Ok(M::ContinuationA.apply(S::AddUnsignedImmediate12{dst,src:base,value:offset}.encode()?))
}
/// `R1=memw(R21=##0xc0367850)` is a fused extended address assignment
/// with an immediate word load. The low address bits select the word offset.
pub fn encode_fused_parameter_load(dst:G,base:G,absolute:u32)->Result<[u32;2],String>{
    if (dst.number(),base.number(),absolute)!=(1,21,CONTROLLER_PARAMETER_BLOCK){
        return Err("unverified controller parameter pointer or registers".into());
    }
    if absolute%4!=0{return Err("parameter load is not word-aligned".into());}
    let [ext,_]=cdsp_boot::encode_extended_assignment(base,absolute as i32,false);
    let body=0x9b80_d000|(u32::from(base.number())<<16)|
        (((absolute&0x3f)/4)<<8)|u32::from(dst.number());
    Ok([M::ContinuationA.apply(ext),M::Final.apply(body)])
}
/// `R3-=lsr(R2,#12)` drops the old bank count from the updated controller
/// limit, preserving the original packet's continuation bit.
pub fn encode_subtract_shifted_bank_count(dst:G,source:G,shift:u8)->Result<u32,String>{
    if (dst.number(),source.number(),shift)!=(3,2,12){
        return Err("unverified bank delta operands or shift".into());
    }
    Ok(M::ContinuationA.apply(0x8e00_c020|
        (u32::from(source.number())<<16)|(u32::from(shift)<<8)|u32::from(dst.number())))
}
/// Signed pair R5:4=combine(#-1,#4): independently assembled by LLVM6.
pub fn encode_helper_limit_pair(low:G,high:i8,lower:u8)->Result<u32,String>{
    if (low.number(),high,lower)!=(4,-1,4){return Err("unverified helper argument pair".into());}
    Ok(M::ContinuationA.apply(0x7c02_dfe0|u32::from(low.number())))
}
/// Hexagon duplex `R2=#0; memw(SP+#0)=R18` (one 32-bit word, not two).
pub fn encode_saved_bank_count(zero_dst:G,stack:G,src:G,offset:u8)->Result<u32,String>{
    if (zero_dst.number(),stack.number(),src.number(),offset)!=(2,29,18,0){
        return Err("unverified bank-count/zero duplex operands".into());
    }
    Ok(0x6800_280a|(u32::from(zero_dst.number())<<16))
}
/// Store an immediate helper argument to a word-aligned stack slot. The
/// original code uses 3 distinct slots and packet terminators.
pub fn encode_stack_argument(stack:G,offset:u8,value:u8,continued:bool)->Result<u32,String>{
    if stack.number()!=29 || ![(12,16),(8,0),(4,4)].contains(&(offset,value)){
        return Err("unverified controller helper stack immediate".into());
    }
    let word=0x3c5d_c000|(u32::from(offset/4)<<7)|u32::from(value);
    Ok(if continued{M::ContinuationA}else{M::Final}.apply(word))
}
/// Packet-relative Hexagon V66 CALL, measured from physical 0x96cc8b70.
pub fn encode_controller_helper_call(packet:u32,target:u32)->Result<u32,String>{
    if (packet,target)!=(0x96cc_8b70,CONTROLLER_HELPER){
        return Err("unverified bank enumeration helper packet or target".into());
    }
    let distance=target.checked_sub(packet).ok_or("backward helper target")?;
    if distance&1!=0||distance>0x1ffffe{return Err("invalid direct CALL displacement".into());}
    Ok(M::ContinuationA.apply(0x5a00_0000|
        (((distance>>15)&0x3f)<<16)|((distance>>1)&0x3fff)))
}

pub fn build_bank_initializer()->Result<Vec<u8>,String>{
    let mut b=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut b,encode_bank_count_argument(r(3)?,r(16)?)?);
    emit(&mut b,encode_stack_descriptor_pointer(r(0)?,r(29)?,28)?);
    for word in encode_fused_parameter_load(r(1)?,r(21)?,CONTROLLER_PARAMETER_BLOCK)?{emit(&mut b,word);}
    emit(&mut b,encode_subtract_shifted_bank_count(r(3)?,r(2)?,12)?);
    emit(&mut b,encode_helper_limit_pair(r(4)?,-1,4)?);
    emit(&mut b,encode_saved_bank_count(r(2)?,r(29)?,r(18)?,0)?);
    emit(&mut b,encode_stack_argument(r(29)?,12,16,true)?);
    emit(&mut b,encode_stack_argument(r(29)?,8,0,false)?);
    emit(&mut b,encode_controller_helper_call(0x96cc_8b70,CONTROLLER_HELPER)?);
    emit(&mut b,encode_stack_argument(r(29)?,4,4,false)?);
    if b.len()!=EXECUTABLE_LEN{return Err("bank initializer must generate eleven instruction words".into());}
    Ok(b)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut p=cdsp_late_controller_bank_guards::build_prefix()?;
    p.extend(build_bank_initializer()?);
    if p.len()!=FULL_RWE_LAYOUT_LEN||
        p.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("bank initializer executable/layout accounting differs".into());
    }
    Ok(p)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bank_initializer()?.as_slice()){
        return Err("controller bank initializer mismatches pinned CDSP reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined CDSP RWE prefix mismatches reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn eleven_checked_packet_words(){
        let bytes=build_bank_initializer().unwrap();
        let got:Vec<u32>=bytes.chunks_exact(4).map(|b|u32::from_le_bytes(b.try_into().unwrap())).collect();
        assert_eq!(got,[0x7070_4003,0xb01d_4380,0x0c03_59e1,0x9b95_d401,
            0x8e02_4c23,0x7c02_5fe4,0x6802_280a,0x3c5d_4190,
            0x3c5d_c100,0x5a00_6fc0,0x3c5d_c084]);
        assert!(encode_bank_count_argument(r(3).unwrap(),r(17).unwrap()).is_err());
        assert!(encode_fused_parameter_load(r(1).unwrap(),r(21).unwrap(),CONTROLLER_PARAMETER_BLOCK+4).is_err());
        assert!(encode_subtract_shifted_bank_count(r(3).unwrap(),r(2).unwrap(),11).is_err());
        assert!(encode_helper_limit_pair(r(4).unwrap(),0,4).is_err());
        assert!(encode_stack_argument(r(29).unwrap(),12,17,true).is_err());
        assert!(encode_controller_helper_call(0x96cc8b74,CONTROLLER_HELPER).is_err());
    }
    #[test]fn prefix_accounts_for_prior_layout_and_no_extra_zeros(){
        assert_eq!(EXECUTABLE_LEN,44);
        assert_eq!(FULL_RWE_LAYOUT_LEN,2936);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2876);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),2936);
    }
}
