// SPDX-License-Identifier: GPL-3.0-only
//! Three controller error handlers after the shared return paths, encoded
//! from Hexagon V66 call targets, immediate operands and packet boundaries.
//! The 12 subsequent NOP bytes are structural layout, never executable code.
use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M, ScalarPacket as S};
use super::cdsp_late_controller_return_paths;

pub const FILE_OFFSET:usize=0x56890;
pub const END_FILE_OFFSET:usize=0x568e4;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const END_LAYOUT_OFFSET:usize=0x568f0;
pub const NEW_LAYOUT_NOP_LEN:usize=END_LAYOUT_OFFSET-END_FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8890;
pub const RWE_START:usize=cdsp_late_controller_return_paths::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_LAYOUT_OFFSET-RWE_START;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_controller_return_paths::RWE_PREFIX_LAYOUT_LEN+NEW_LAYOUT_NOP_LEN;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_return_paths::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;

const FIRST_MESSAGE:u32=0xc022_6180;
const SECOND_MESSAGE:u32=0xc022_61a6;
const LOW_PAIR_LITERAL:u32=0xc022_6175;
const TRACE_HELPER:u32=0x96dc_0bac;
const REGISTER_HELPER:u32=0x96cc_d8c4;

fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// Checked direct Hexagon call from a packet start to one of two identified
/// physical service addresses. Unlike an address literal, the 22-bit call
/// displacement is split between two nonadjacent instruction fields.
pub fn encode_error_helper_call(from:u32,target:u32)->Result<u32,String>{
    if ![(0x96cc_8890,TRACE_HELPER),(0x96cc_88a0,REGISTER_HELPER),
         (0x96cc_88ac,TRACE_HELPER),(0x96cc_88bc,REGISTER_HELPER),
         (0x96cc_88c8,TRACE_HELPER),(0x96cc_88d8,REGISTER_HELPER)]
         .contains(&(from,target)) {
        return Err("controller diagnostic call site/target has not been verified".into());
    }
    let d=target.checked_sub(from).ok_or("diagnostic call may not jump backward")?;
    if d&1!=0 || d>0x1fff_fe {return Err("diagnostic call displacement out of range".into());}
    Ok(M::ContinuationA.apply(0x5a00_0000 | (((d>>15)&0x3f)<<16) | ((d>>1)&0x3fff)))
}

/// Extended R0 immediate holds a string/controller diagnostic address.
/// Packet parse markers are explicit: both the extender and the consumer
/// are non-final because the last R18 status assignment ends the packet.
pub fn encode_message_pointer(dst:G,message:u32)->Result<[u32;2],String>{
    if dst.number()!=0 || ![FIRST_MESSAGE,SECOND_MESSAGE].contains(&message){
        return Err("diagnostic R0 message pointer is unknown".into());
    }
    let [ext,word]=cdsp_boot::encode_extended_assignment(dst,message as i32,false);
    Ok([M::ContinuationA.apply(ext),M::ContinuationA.apply(word)])
}

/// The three observed numeric diagnostics are independently assembled Hexagon
/// `R18=#imm` instructions, all fitting the checked unsigned 12-bit field.
pub fn encode_diagnostic_code(dst:G,code:u16)->Result<u32,String>{
    if dst.number()!=18 || ![212,284,307].contains(&code){
        return Err("unsupported controller diagnostic severity/record code".into());
    }
    S::AssignUnsignedImmediate{dst,value:code}.encode()
}

/// V66 `R1:0=combine(R18,##0xc0226175)`: immext carries the upper part of
/// the literal, while the consumer holds the low six bits and register pair.
pub fn encode_register_pair(low:G,high:G,address:u32)->Result<[u32;2],String>{
    if low.number()!=0 || high.number()!=18 || address!=LOW_PAIR_LITERAL{
        return Err("unsupported diagnostic register pair/literal".into());
    }
    let [ext,_]=cdsp_boot::encode_extended_assignment(low,address as i32,false);
    let body=M::Final.apply(0x7300_2000 | (u32::from(high.number())<<16)
                             | ((address&0x3f)<<5) | u32::from(low.number()));
    Ok([M::ContinuationA.apply(ext),body])
}

/// Emit one seven-word diagnostic record with two calls. All addresses and
/// status values are named semantic inputs; no original opcode arrays appear.
fn emit_record(out:&mut Vec<u8>,site:u32,message:u32,code:u16)->Result<(),String>{
    if ![(PHYSICAL_ADDRESS,FIRST_MESSAGE,212),
         (PHYSICAL_ADDRESS+0x1c,SECOND_MESSAGE,284),
         (PHYSICAL_ADDRESS+0x38,SECOND_MESSAGE,307)]
        .contains(&(site,message,code)){
        return Err("unsupported controller diagnostic packet".into());
    }
    emit(out,encode_error_helper_call(site,TRACE_HELPER)?);
    for w in encode_message_pointer(r(0)?,message)?{emit(out,w);}
    emit(out,encode_diagnostic_code(r(18)?,code)?);
    emit(out,encode_error_helper_call(site+0x10,REGISTER_HELPER)?);
    for w in encode_register_pair(r(0)?,r(18)?,LOW_PAIR_LITERAL)?{emit(out,w);}
    Ok(())
}

pub fn build_error_tail()->Result<Vec<u8>,String>{
    let mut result=Vec::with_capacity(EXECUTABLE_LEN);
    emit_record(&mut result,PHYSICAL_ADDRESS,FIRST_MESSAGE,212)?;
    emit_record(&mut result,PHYSICAL_ADDRESS+0x1c,SECOND_MESSAGE,284)?;
    emit_record(&mut result,PHYSICAL_ADDRESS+0x38,SECOND_MESSAGE,307)?;
    if result.len()!=EXECUTABLE_LEN{return Err("controller error tail is not 84 bytes".into());}
    Ok(result)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_return_paths::build_prefix()?;
    out.extend(build_error_tail()?);
    for _ in 0..NEW_LAYOUT_NOP_LEN/4{emit(&mut out,S::Nop.encode()?);}
    if out.len()!=FULL_RWE_LAYOUT_LEN || out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_LAYOUT_NOP_LEN{
        return Err("controller tail source/layout accounting mismatch".into());
    }
    Ok(out)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_error_tail()?.as_slice()){
        return Err("controller error-tail bytes differ from pinned CDSP".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_LAYOUT_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("full controller RWE prefix through alignment differs from CDSP".into());
    }
    Ok(())
}

#[cfg(test)] mod tests{
    use super::*;
    #[test] fn error_records_have_checked_calls_and_literals(){
        let out=build_error_tail().unwrap();assert_eq!(out.len(),84);
        let w:Vec<u32>=out.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();
        assert_eq!(w[0],0x5a1f_418e);
        assert_eq!(w[1],0x0c02_4986);
        assert_eq!(w[2],0x7800_4000);
        assert_eq!(w[3],0x7800_da92);
        assert_eq!(w[4],0x5a00_6812);
        assert_eq!(w[6],0x7312_e6a0);
        assert_eq!(w[7],0x5a1f_4180);
        assert_eq!(w[10],0x7800_e392);
        assert_eq!(w[17],0x7800_e672);
        assert_eq!(w[20],0x7312_e6a0);
        assert!(encode_error_helper_call(0x96cc_8894,TRACE_HELPER).is_err());
        assert!(encode_register_pair(r(0).unwrap(),r(16).unwrap(),LOW_PAIR_LITERAL).is_err());
        assert!(encode_diagnostic_code(r(18).unwrap(),308).is_err());
        assert!(encode_message_pointer(r(1).unwrap(),FIRST_MESSAGE).is_err());
    }
    #[test] fn original_alignment_is_not_counted_as_executable(){
        let all=build_prefix().unwrap();
        assert_eq!(FULL_RWE_LAYOUT_LEN,0x8f0);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2232);
        assert_eq!(RWE_LAYOUT_NOP_LEN,56);
        assert_eq!(all.len(),2288);
        assert_eq!(&all[all.len()-12..],&[0x00,0xc0,0x00,0x7f,0x00,0xc0,0x00,0x7f,0x00,0xc0,0x00,0x7f]);
    }
}
