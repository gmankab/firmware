// SPDX-License-Identifier: GPL-3.0-only
//! Partial CDSP exception controller startup, using decoded V66 ISA forms.
//! Caller/handler paths continue after file 0x4860, not yet reconstructed.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_exception_context;
use super::cdsp_early_supervisor_vectors;

pub const FILE_OFFSET:usize=0x481c;
pub const REGION_END:usize=0x4860;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_181c;

fn read_special_s54(dst:Gpr)->u32{
    0x6e80_c000|(54<<16)|u32::from(dst.number())
}
fn memory_word_load(dst:Gpr,address:Gpr)->u32{
    0x9180_c000 | (u32::from(address.number())<<16)|u32::from(dst.number())
}
fn memory_word_store(src:Gpr,address:Gpr)->u32{
    0xa180_c000 | (u32::from(address.number())<<16)|(u32::from(src.number())<<8)
}
/// One extender, then a duplex with signed 32-bit register assignment and
/// a 32-bit word load. LLVM6 independently corroborates its register and
/// low-immediate fields after changing both operands.
fn extended_assignment_plus_word_load(assign_dst:Gpr,value:i32,
                                      load_dst:Gpr,load_base:Gpr)->Result<[u32;2],String>{
    if assign_dst.number()>7||load_dst.number()>7||load_base.number()>7 {
        return Err("CDSP coupled extended load supports only observed low-register duplex bank".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(assign_dst,value,true)[0];
    let low=(value as u32)&0x3f;
    Ok([ext,0x4800_0000 | (low<<20)
        |(u32::from(assign_dst.number())<<16)
        |(u32::from(load_base.number())<<4)|u32::from(load_dst.number())])
}
fn put(out:&mut Vec<u8>,word:u32,p:P) {out.extend_from_slice(&p.apply(word).to_le_bytes());}
fn put_duplex(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

pub fn build_controller_setup()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,H::Ickill.encode()?,P::Final);
    put(&mut out,read_special_s54(r(23)?),P::Final);
    let first=cdsp_boot::encode_extended_add(r(22)?,r(23)?,-0x1fdabe0,false);
    put(&mut out,first[0],P::ContinuationA);
    put(&mut out,first[1],P::ContinuationA);
    let second=cdsp_boot::encode_extended_add(r(1)?,r(23)?,-0x1fc6e38,true);
    put(&mut out,second[0],P::ContinuationA);
    put(&mut out,second[1],P::Final);
    let paired=extended_assignment_plus_word_load(r(0)?,-0x1fc94b8,r(1)?,r(1)?)?;
    put(&mut out,paired[0],P::ContinuationA);
    put_duplex(&mut out,paired[1]);
    put(&mut out,H::AddRegisters{dst:r(1)?,left:r(1)?,right:r(0)?}.encode()?,P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(0)?,src:r(22)?,value:8}.encode()?,P::ContinuationA);
    put(&mut out,memory_word_load(r(1)?,r(1)?),P::Final);
    put(&mut out,cdsp_early_supervisor_vectors::short_call(0x96c0_1848,0x96c0_195c)?,P::Final);
    let reset=cdsp_boot::encode_extended_add(r(0)?,r(23)?,-0x1fda71c,false);
    put(&mut out,reset[0],P::ContinuationA);
    put(&mut out,reset[1],P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:2}.encode()?,P::Final);
    put(&mut out,memory_word_store(r(1)?,r(0)?),P::Final);
    put(&mut out,cdsp_early_supervisor_vectors::short_call(0x96c0_185c,0x96c0_1a38)?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP exception setup emitted wrong number of words".into());}
    Ok(out)
}

pub fn build_supervisor_with_setup()->Result<Vec<u8>,String>{
    let mut out=cdsp_early_exception_context::build_supervisor_with_context()?;
    if cdsp_early_exception_context::REGION_END!=FILE_OFFSET {return Err("CDSP exception setup has an unimplemented gap".into());}
    out.extend(build_controller_setup()?);
    if out.len()!=REGION_END-super::cdsp_early_system_start::FILE_OFFSET {return Err("CDSP supervisor/setup cumulative size mismatch".into());}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_exception_context::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_controller_setup()?.as_slice()){
        return Err("CDSP exception setup instructions differ from reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn setup_carries_verifiable_opcode_and_packet_fields(){
        assert_eq!(FILE_OFFSET,cdsp_early_exception_context::REGION_END);
        assert_eq!(FUNCTION_BYTES,68);
        assert_eq!(build_controller_setup().unwrap().len(),68);
        let op=extended_assignment_plus_word_load(Gpr::new(0).unwrap(),-0x1fc94b8,
                         Gpr::new(1).unwrap(),Gpr::new(1).unwrap()).unwrap();
        assert_eq!(op,[0x0fe0_4dad,0x4880_0011]);
        assert_ne!(extended_assignment_plus_word_load(Gpr::new(2).unwrap(),-0x1fc94b8,
                         Gpr::new(1).unwrap(),Gpr::new(1).unwrap()).unwrap()[1],op[1]);
    }
}
