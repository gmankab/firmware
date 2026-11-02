// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 exception-entry thread-local pointer/stack switch, following
//! the validated seven-vector dispatch at file 0x4700–0x471c. The next
//! instructions save the interrupted thread context and remain unimplemented.
//! Every word is generated using typed instruction fields, not copied from the
//! reference firmware or assumed to be a newly signed CDSP image.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_supervisor_vectors;

pub const FILE_OFFSET:usize=0x471c;
pub const REGION_END:usize=0x4760;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_171c;

#[derive(Clone,Copy)]
enum SupervisorRead {Htid,S54}
impl SupervisorRead {
    fn selector(self)->u8{match self {Self::Htid=>8,Self::S54=>54}}
    fn encode(self,dst:Gpr)->u32{
        0x6e80_c000 | (u32::from(self.selector())<<16) | u32::from(dst.number())
    }
}
#[derive(Clone,Copy)]
enum Half {Lower,Upper}
fn assign_half_register(dst:Gpr,half:Half,value:u16)->u32{
    let family=match half{Half::Lower=>0x7120_c000,Half::Upper=>0x7220_c000};
    family | (u32::from(dst.number())<<16) | (u32::from(value)&0x3fff)
        | ((u32::from(value)>>14)<<22)
}
fn inclusive_shift_right(dst:Gpr,src:Gpr,count:u8)->Result<u32,String>{
    if count>=32{return Err("Hexagon right-shift count must be <32".into());}
    Ok(0x8e40_c0a0 | (u32::from(src.number())<<16)
       | (u32::from(count)<<8)|u32::from(dst.number()))
}
fn store_double_word(base:Gpr,low:Gpr)->Result<u32,String>{
    if low.number()%2!=0 || low.number()>30 {
        return Err("Hexagon doubleword pair must start at an even GPR".into());
    }
    Ok(0xa1c0_c000 | (u32::from(base.number())<<16)
       | (u32::from(low.number())<<8))
}
fn accumulate_mpy_imm(dst:Gpr,operand:Gpr,value:u8)->u32{
    0xe100_c000 | (u32::from(operand.number())<<16)
        | (u32::from(value)<<5)|u32::from(dst.number())
}
fn put(output:&mut Vec<u8>,encoded:u32,marker:P){
    output.extend_from_slice(&marker.apply(encoded).to_le_bytes());
}

pub fn build_exception_entry()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut output=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut output,SupervisorRead::Htid.encode(r(23)?),P::Final);
    put(&mut output,H::ArithmeticShiftLeft{dst:r(23)?,src:r(23)?,amount:2}.encode()?,P::Final);
    put(&mut output,H::ArithmeticShiftLeft{dst:r(23)?,src:r(23)?,amount:18}.encode()?,P::Final);
    put(&mut output,assign_half_register(r(23)?,Half::Lower,0x58a0),P::Final);
    put(&mut output,inclusive_shift_right(r(23)?,r(23)?,16)?,P::Final);
    put(&mut output,assign_half_register(r(23)?,Half::Upper,0xfe02),P::Final);
    put(&mut output,SupervisorRead::S54.encode(r(22)?),P::Final);
    put(&mut output,H::AddRegisters{dst:r(23)?,left:r(23)?,right:r(22)?}.encode()?,P::Final);
    let base=cdsp_boot::encode_extended_add(r(23)?,r(22)?,-0x1fdabe0,false);
    put(&mut output,base[0],P::ContinuationA);
    put(&mut output,base[1],P::ContinuationA);
    put(&mut output,store_double_word(r(23)?,r(0)?)?,P::Final);
    put(&mut output,SupervisorRead::Htid.encode(r(22)?),P::ContinuationA);
    put(&mut output,H::AddSmallImmediate{dst:r(1)?,src:r(23)?,value:104}.encode()?,P::Final);
    put(&mut output,accumulate_mpy_imm(r(1)?,r(22)?,224),P::ContinuationA);
    // The jump is measured from the beginning of the *same* packet at
    // file 0x4750, so the 0x4758 target is +8 from the packet start.
    put(&mut output,H::JumpRelative{displacement_bytes:8}.encode()?,P::Final);
    put(&mut output,H::AssignRegister{dst:r(1)?,src:r(29)?}.encode()?,P::ContinuationA);
    put(&mut output,H::AssignRegister{dst:r(29)?,src:r(1)?}.encode()?,P::Final);
    if output.len()!=FUNCTION_BYTES{return Err("CDSP exception-entry typed length mismatch".into());}
    Ok(output)
}

/// Two contiguous supervisor source-encoded fragments: earlier context
/// setup/vector table followed by thread-local exception-entry prologue.
pub fn build_supervisor_with_entry()->Result<Vec<u8>,String>{
    let mut out=cdsp_early_supervisor_vectors::build_supervisor_merged()?;
    if cdsp_early_supervisor_vectors::REGION_END!=FILE_OFFSET {
        return Err("Hexagon exception prologue does not abut vectors".into());
    }
    out.extend(build_exception_entry()?);
    if out.len()!=REGION_END-super::cdsp_early_system_start::FILE_OFFSET {
        return Err("Hexagon supervisor cumulative size mismatch".into());
    }
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_supervisor_vectors::verify_reference(reference)?;
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_exception_entry()?.as_slice()) {
        return Err("CDSP exception-entry byte comparison failed".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn exception_entry_abuts_verified_vector_table(){
        assert_eq!(FILE_OFFSET,cdsp_early_supervisor_vectors::REGION_END);
        assert_eq!(FUNCTION_BYTES,68);
        assert_eq!(build_exception_entry().unwrap().len(),68);
        assert_eq!(build_supervisor_with_entry().unwrap().len(),0x260);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn halfword_address_fields_have_no_fixed_vendor_words(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(assign_half_register(r(23),Half::Lower,0x58a0),0x7177_d8a0);
        assert_eq!(assign_half_register(r(23),Half::Upper,0xfe02),0x72f7_fe02);
        assert_ne!(assign_half_register(r(23),Half::Lower,0x58a0),assign_half_register(r(24),Half::Lower,0x58a0));
        assert_eq!(accumulate_mpy_imm(r(1),r(22),224),0xe116_dc01);
        assert!(store_double_word(r(23),r(1)).is_err());
        assert!(inclusive_shift_right(r(23),r(23),32).is_err());
    }
}
