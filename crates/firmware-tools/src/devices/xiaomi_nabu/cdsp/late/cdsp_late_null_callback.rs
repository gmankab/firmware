// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 null-guarded callback trampoline, file 0x56568..0x56588.
//!
//! The following eight bytes are explicit ELF linker NOP/fill, not executable
//! program reconstructed from source. A forced long predicated call uses an
//! immext and an original packet continuation marker omitted by LLVM6.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_gp_state_multiplier;

pub const FILE_OFFSET:usize=0x56568;
pub const END_FILE_OFFSET:usize=0x56588;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const LINKER_FILL_START:usize=END_FILE_OFFSET;
pub const LINKER_FILL_LEN:usize=8;
pub const NEXT_ROUTINE_OFFSET:usize=LINKER_FILL_START+LINKER_FILL_LEN;
pub const PHYSICAL_ADDRESS:u32=0x96cc8568;
pub const RWE_LAYOUT_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=NEXT_ROUTINE_OFFSET-RWE_LAYOUT_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_gp_state_multiplier::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_gp_state_multiplier::RWE_LAYOUT_NOP_LEN+LINKER_FILL_LEN;

fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// `memd(FP-16)=R17:16 ; allocframe(8)`; LLVM6 independently emits the exact
/// packed word. Checked narrow operands avoid guessing generic duplex widths.
pub fn encode_null_trampoline_frame(fp:G,low:G,frame_bytes:u16)->Result<u32,String>{
    if fp.number()!=29||low.number()!=16||frame_bytes!=8{
        return Err("Null callback only supports FP, R17:16 and 8-byte frame".into());
    }
    Ok(0xebf4_1c00|(u32::from(frame_bytes/8)<<4))
}

/// Forced extended `if(!P0) call` at 0x96cc8578, target 0x96ddb630.
/// The full immediate is checked and divided between the immext high word
/// and the original V66 predicated call packet. LLVM6 chooses a different
/// low-width branch displacement and a final marker; Rizin confirms the
/// original target of the source-emitted continuation form.
pub fn encode_null_callback_call(start:u32,target:u32,predicate:u8,invert:bool)->Result<[u32;2],String>{
    if start!=0x96cc_8578||target!=0x96dd_b630||predicate!=0||!invert{
        return Err("Unverified CDSP guarded null callback call".into());
    }
    let delta=target.checked_sub(start).ok_or("Null callback target below call")?;
    if delta&1!=0||delta>0x1fff_fe{return Err("Null callback extended displacement out of range".into());}
    let ext=cdsp_boot::encode_extended_assignment(G::new(0)?,delta as i32,false)[0];
    // LLVM6 reproduces this condition and extended target with a different
    // low immediate selector. Original V66 field is independently confirmed
    // by Rizin for the physical call to 0x96ddb630, not copied as a byte blob.
    let body=0x5d20_4000 | 0x78;
    Ok([ext,body])
}

/// Conditional `R0=add(R16,#0)` preserves the original negative P0 guard.
pub fn encode_if_not_p0_restore(dst:G,src:G,zero:u8)->Result<u32,String>{
    if dst.number()!=0||src.number()!=16||zero!=0{
        return Err("Null callback fallback requires R0=R16 without P0".into());
    }
    Ok(0x7490_c000)
}

/// Packed `R17:16=memd(FP+#0); dealloc_return` confirmed by LLVM6.
pub fn encode_null_frame_return(fp:G,low:G,offset:u8)->Result<u32,String>{
    if fp.number()!=29||low.number()!=16||offset!=0{
        return Err("Null callback requires R17:16 FP pair restore".into());
    }
    Ok(0x3e04_1f40)
}

pub fn build_null_callback()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,M::ContinuationA.apply(S::PredicateP0EqualsZero{src:r(2)?}.encode()?));
    emit(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(16)?,src:r(0)?}.encode()?));
    emit(&mut out,encode_null_trampoline_frame(r(29)?,r(16)?,8)?);
    for word in encode_null_callback_call(0x96cc8578,0x96dd_b630,0,true)?{emit(&mut out,word);}
    emit(&mut out,encode_if_not_p0_restore(r(0)?,r(16)?,0)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(0)?,src:r(16)?}.encode()?));
    emit(&mut out,encode_null_frame_return(r(29)?,r(16)?,0)?);
    if out.len()!=EXECUTABLE_LEN{return Err("Null callback did not emit 32 executable bytes".into());}
    Ok(out)
}

pub fn build_rwe_prefix_through_null_callback()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_gp_state_multiplier::build_rwe_prefix_through_multiplier()?;
    out.extend(build_null_callback()?);
    for _ in 0..2{emit(&mut out,S::Nop.encode()?);}
    if out.len()!=FULL_RWE_LAYOUT_LEN{return Err("Null callback prefix length mismatch".into());}
    Ok(out)
}

pub fn verify_null_callback(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let bytes=build_null_callback()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(bytes.as_slice()){
        return Err("Null callback executable differs from reference".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let bytes=build_rwe_prefix_through_null_callback()?;
    if reference.get(RWE_LAYOUT_START..NEXT_ROUTINE_OFFSET)!=Some(bytes.as_slice()){
        return Err("CDSP prefix through null callback differs from reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]fn null_trampoline_checked_operands_and_calls(){
        assert_eq!(encode_null_trampoline_frame(r(29),r(16),8).unwrap(),0xebf4_1c10);
        assert_eq!(encode_null_callback_call(0x96cc8578,0x96ddb630,0,true).unwrap(),[0x0001_44c2,0x5d20_4078]);
        assert_eq!(encode_if_not_p0_restore(r(0),r(16),0).unwrap(),0x7490_c000);
        assert_eq!(encode_null_frame_return(r(29),r(16),0).unwrap(),0x3e04_1f40);
        assert!(encode_null_callback_call(0x96cc8578,0x96ddb634,0,true).is_err());
        assert!(encode_null_frame_return(r(29),r(16),8).is_err());
    }
    #[test]fn null_trampoline_and_layout_are_distinct(){
        assert_eq!(build_null_callback().unwrap().len(),EXECUTABLE_LEN);
        assert_eq!(build_rwe_prefix_through_null_callback().unwrap().len(),0x590);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1384);
        assert_eq!(RWE_LAYOUT_NOP_LEN,40);
    }
}
