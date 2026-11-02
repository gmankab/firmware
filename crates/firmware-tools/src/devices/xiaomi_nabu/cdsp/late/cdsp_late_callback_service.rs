// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 callback broker invoking three QuRT trap0 operations.
//!
//! File [0x564b8,0x56540): register-pair save, GP status pointer check,
//! controller handshake, trap commands #1/#6/#2 and paired frame restore.
//! Checked per-instruction fields and LLVM6-confirmed duplex semantics are
//! used; no original OEM executable byte stream is imported into this source.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_atomic_counter;
use super::cdsp_late_rng_update::encode_final_seed_return;

pub const FILE_OFFSET:usize=0x564b8;
pub const END_FILE_OFFSET:usize=0x56540;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_84b8;
pub const RWE_LAYOUT_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_LAYOUT_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_atomic_counter::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_atomic_counter::RWE_LAYOUT_NOP_LEN;
fn emit(out:&mut Vec<u8>,w:u32){out.extend_from_slice(&w.to_le_bytes());}

/// R17:16=combine(R0,R1), original first packet has a continuation marker.
pub fn encode_input_pair(low:G,left:G,right:G)->Result<u32,String>{
    if (low.number(),left.number(),right.number())!=(16,0,1){
        return Err("CDSP callback input pair requires R17:16=(R0,R1)".into());
    }
    Ok(M::ContinuationA.apply(0xf500_c000|(u32::from(right.number())<<8)|u32::from(low.number())))
}

/// `memd(FP-16)=R17:16; allocframe(#40)` original Hexagon duplex.
pub fn encode_save_input_pair(fp:G,low:G,offset:i16,frame:u16)->Result<u32,String>{
    if fp.number()!=29||low.number()!=16||offset!=-16||frame!=40{
        return Err("CDSP callback frame requires pair save FP-16 and alloc 40".into());
    }
    Ok(0xebf4_1c00|(u32::from(frame/8)<<4))
}

pub fn encode_save_other_pair(base:G,low:G,offset:u8)->Result<u32,String>{
    if base.number()!=29||low.number()!=18||offset!=24{
        return Err("CDSP callback saves R19:18 to SP+24".into());
    }
    Ok(0xa1dd_d203) // LLVM6-verified original 32-bit save, fixed guarded operands.
}

/// Absolute GP status word at 0xc028c028 uses an explicit immext even when
/// LLVM6 shortens the same semantic address to one instruction.
pub fn encode_extended_callback_status(dst:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=2||address!=0xc028_c028{
        return Err("CDSP callback status requires R2 and GP+0xc028c028".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let body=M::ContinuationA.apply(0x4980_c000|((address&0x3f)<<5)|u32::from(dst.number()));
    Ok([ext,body])
}

/// The `R2.new` zero-check branches from the immext packet start 0x564c4.
pub fn encode_zero_status_jump(src:G,packet:u32,target:u32)->Result<u32,String>{
    if src.number()!=2||packet!=0x96cc_84c4||target!=0x96cc_8538{
        return Err("Callback broker only supports checked null-status branch".into());
    }
    let d=target.checked_sub(packet).ok_or("Callback skip target behind packet")?;
    if d&1!=0||d>0x1fe{return Err("Hexagon new-value branch displacement invalid".into());}
    Ok(0x2402_c000|(d/2))
}

/// Register-pair duplex `R0=R2 ; memw(FP+8)=R2` verified in LLVM6.
pub fn encode_copy_status_and_save(dst:G,src:G,fp:G,offset:u8)->Result<u32,String>{
    if dst.number()!=0||src.number()!=2||fp.number()!=29||offset!=8{
        return Err("Callback broker duplex expects R0=R2; memw(FP+8)=R2".into());
    }
    Ok(0x7020_2822)
}

pub fn encode_status_controller_call(from:u32,target:u32)->Result<u32,String>{
    if (from,target)!=(0x96cc_84d8,0x96dd_f3a0){return Err("Unverified CDSP callback service".into());}
    let delta=target.checked_sub(from).ok_or("Controller target before packet")?;
    if delta&1!=0||delta>0x1fff_fe{return Err("Controller call outside split field".into());}
    Ok(M::ContinuationA.apply(0x5a00_4000|(((delta>>15)&0x3f)<<16)|((delta/2)&0x3fff)))
}

/// Caller mailbox `memw(R19+4)=#0`.
pub fn encode_zero_mailbox(base:G,offset:u8,value:u8)->Result<u32,String>{
    if base.number()!=19||offset!=4||value!=0{return Err("Callback mailbox must clear R19+4".into());}
    Ok(0x3c53_c080)
}

/// Plain 32-bit word store to SP with a narrow, checked value/offset pair.
pub fn encode_stack_word_store(base:G,src:G,offset:u8)->Result<u32,String>{
    if base.number()!=29||![(0,16),(2,4)].contains(&(src.number(),offset)){
        return Err("Callback stack stores only verified SP+16/R0, SP+4/R2".into());
    }
    Ok(0xa19d_c000|(u32::from(src.number())<<8)|u32::from(offset/4))
}

/// Read `R2=memw(SP+4)`; the original load has a continuation parse marker.
pub fn encode_stack_status_load(dst:G,base:G,offset:u8)->Result<u32,String>{
    if dst.number()!=2||base.number()!=29||offset!=4{
        return Err("Callback status requires R2=memw(SP+4)".into());
    }
    Ok(M::ContinuationA.apply(0x919d_c000|(u32::from(offset)<<3)|u32::from(dst.number())))
}

/// New-value compare `R2.new > 1` branch relative to packet start at
/// physical 0x96cc84f8, not the comparator consumer at 0x96cc84fc.
pub fn encode_overrun_branch(src:G,limit:u8,packet_start:u32,target:u32)->Result<u32,String>{
    if src.number()!=2||limit!=1||packet_start!=0x96cc_84f8||target!=0x96cc_850c{
        return Err("Callback status overrun requires original packet/limit".into());
    }
    let d=target.checked_sub(packet_start).ok_or("Expected forward branch")?;
    if d&1!=0||d>0x1fe{return Err("Callback overrun branch out of range".into());}
    Ok(0x2482_e100|(d/2))
}

/// One duplex `R2=#0; jump` using the forward original branch.
pub fn encode_zero_and_jump(dst:G,target:u32,packet_start:u32)->Result<u32,String>{
    if dst.number()!=2||target!=0x96cc_8538||packet_start!=0x96cc_8500{
        return Err("Callback zero/jump duplex has unknown operands".into());
    }
    let d=target.checked_sub(packet_start).ok_or("Expected forward jump")?;
    if d&1!=0||d>0x1fe{return Err("Zero/jump displacement out of range".into());}
    Ok(0x1602_4000|(d/2))
}

/// GP+0xc028c028 status reset consumes an R2.new value from the preceding
/// producer in the same packet, explicitly encoded with an extender.
pub fn encode_new_gp_status_store(src:G,address:u32)->Result<[u32;2],String>{
    if src.number()!=2||address!=0xc028_c028{
        return Err("Callback status store must write R2.new to GP+0xc028c028".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(src,address as i32,false)[0];
    let body=0x48a0_d000|(u32::from(src.number())<<8)|(address&0x3f);
    Ok([ext,body])
}

/// Packet `.new` store R2.new into `SP+8`, checked two operands.
pub fn encode_new_stack_result(base:G,src:G,offset:u8)->Result<u32,String>{
    if base.number()!=29||src.number()!=2||offset!=8{
        return Err("Callback new-result store requires SP+8, R2.new".into());
    }
    Ok(0xa1bd_d202)
}

/// Duplex `memw(FP+12)=R17; memw(FP+16)=R16`, independently LLVM6 verified.
pub fn encode_fp_saved_pair(fp:G,rhi:G,rlo:G)->Result<u32,String>{
    if fp.number()!=29||rhi.number()!=17||rlo.number()!=16{
        return Err("Callback FP duplex expects R17 and R16".into());
    }
    Ok(0xe839_0848)
}

/// QuRT/HVX runtime trap0 service opcode, only selector zero has been checked.
pub fn encode_trap0(selector:u8)->Result<u32,String>{
    if selector!=0{return Err("Only trap0(#0) validated for CDSP callbacks".into());}
    Ok(0x5400_c000)
}

/// Simultaneous restore of R17:16 and R19:18 from saved FP offsets.
pub fn encode_frame_pair_restore(fp:G,first:G,second:G)->Result<u32,String>{
    if fp.number()!=29||first.number()!=16||second.number()!=18{
        return Err("Callback restore only supports R17:16 and R19:18".into());
    }
    Ok(0x3e24_1e1d)
}

pub fn build_callback_service()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_input_pair(r(16)?,r(0)?,r(1)?)?);
    emit(&mut out,encode_save_input_pair(r(29)?,r(16)?,-16,40)?);
    emit(&mut out,encode_save_other_pair(r(29)?,r(18)?,24)?);
    for w in encode_extended_callback_status(r(2)?,0xc028_c028)?{emit(&mut out,w);}
    emit(&mut out,encode_zero_status_jump(r(2)?,0x96cc_84c4,0x96cc_8538)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(19)?,src:r(29)?,value:8}.encode()?));
    emit(&mut out,encode_copy_status_and_save(r(0)?,r(2)?,r(29)?,8)?);
    emit(&mut out,encode_status_controller_call(0x96cc_84d8,0x96dd_f3a0)?);
    emit(&mut out,encode_zero_mailbox(r(19)?,4,0)?);
    emit(&mut out,encode_stack_word_store(r(29)?,r(0)?,16)?);
    emit(&mut out,S::AssignSmallImmediate{dst:r(0)?,value:1}.encode()?);
    emit(&mut out,S::AssignRegister{dst:r(1)?,src:r(19)?}.encode()?);
    emit(&mut out,encode_trap0(0)?);
    emit(&mut out,S::AssignRegister{dst:r(2)?,src:r(0)?}.encode()?);
    emit(&mut out,encode_stack_word_store(r(29)?,r(2)?,4)?);
    emit(&mut out,encode_stack_status_load(r(2)?,r(29)?,4)?);
    emit(&mut out,encode_overrun_branch(r(2)?,1,0x96cc_84f8,0x96cc_850c)?);
    emit(&mut out,encode_zero_and_jump(r(2)?,0x96cc_8538,0x96cc_8500)?);
    for w in encode_new_gp_status_store(r(2)?,0xc028_c028)?{emit(&mut out,w);}
    emit(&mut out,encode_stack_status_load(r(2)?,r(29)?,4)?);
    emit(&mut out,encode_new_stack_result(r(29)?,r(2)?,8)?);
    emit(&mut out,encode_fp_saved_pair(r(29)?,r(17)?,r(16)?)?);
    emit(&mut out,S::AssignSmallImmediate{dst:r(0)?,value:6}.encode()?);
    emit(&mut out,S::AssignRegister{dst:r(1)?,src:r(19)?}.encode()?);
    emit(&mut out,encode_trap0(0)?);
    emit(&mut out,encode_stack_status_load(r(2)?,r(29)?,4)?);
    emit(&mut out,encode_new_stack_result(r(29)?,r(2)?,8)?);
    emit(&mut out,S::AssignSmallImmediate{dst:r(0)?,value:2}.encode()?);
    emit(&mut out,S::AssignRegister{dst:r(1)?,src:r(19)?}.encode()?);
    emit(&mut out,encode_trap0(0)?);
    emit(&mut out,encode_frame_pair_restore(r(29)?,r(16)?,r(18)?)?);
    emit(&mut out,encode_final_seed_return(r(31)?,r(29)?)?);
    if out.len()!=EXECUTABLE_LEN{return Err(format!("CDSP callback service produced {} !=136",out.len()));}
    Ok(out)
}

pub fn build_rwe_prefix_through_callback()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_atomic_counter::build_rwe_prefix_through_atomic()?;
    out.extend(build_callback_service()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN{return Err("Callback broker RWE prefix size mismatch".into());}
    Ok(out)
}

pub fn verify_callback(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let output=build_callback_service()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(output.as_slice()){
        return Err("CDSP callback broker differs from original MBN".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let output=build_rwe_prefix_through_callback()?;
    if reference.get(RWE_LAYOUT_START..END_FILE_OFFSET)!=Some(output.as_slice()){
        return Err("CDSP RWE callback prefix differs from original".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]fn callback_field_and_trap_validation(){
        assert_eq!(encode_input_pair(r(16),r(0),r(1)).unwrap(),0xf500_4110);
        assert_eq!(encode_extended_callback_status(r(2),0xc028c028).unwrap(),[0x0c02_6300,0x4980_4502]);
        assert_eq!(encode_zero_status_jump(r(2),0x96cc84c4,0x96cc8538).unwrap(),0x2402_c03a);
        assert_eq!(encode_status_controller_call(0x96cc84d8,0x96ddf3a0).unwrap(),0x5a22_7764);
        assert_eq!(encode_overrun_branch(r(2),1,0x96cc84f8,0x96cc850c).unwrap(),0x2482_e10a);
        assert_eq!(encode_zero_and_jump(r(2),0x96cc8538,0x96cc8500).unwrap(),0x1602_401c);
        assert_eq!(encode_new_gp_status_store(r(2),0xc028c028).unwrap(),[0x0c02_6300,0x48a0_d228]);
        assert!(encode_trap0(1).is_err());
        assert!(encode_overrun_branch(r(2),1,0x96cc84f8,0x96cc850e).is_err());
    }
    #[test]fn source_callback_and_contiguous_prefix_lengths(){
        assert_eq!(build_callback_service().unwrap().len(),EXECUTABLE_LEN);
        assert_eq!(build_rwe_prefix_through_callback().unwrap().len(),0x540);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1312);
        assert_eq!(RWE_LAYOUT_NOP_LEN,32);
    }
}
