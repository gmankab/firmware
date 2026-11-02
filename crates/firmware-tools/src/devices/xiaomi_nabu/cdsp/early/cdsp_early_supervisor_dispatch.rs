// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 supervisor initialisation and indexed callback dispatch.
//! The executable begins immediately after one verified four-byte ELF
//! alignment word at `0x5a5c`; that alignment is not source-counted code.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_controller_restoration as previous;
use super::cdsp_early_controllers;

pub const FILE_OFFSET:usize=0x5a60;
pub const REGION_END:usize=0x5aa8;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2a60;
const ALIGNMENT_START:usize=0x5a5c;

fn put(out:&mut Vec<u8>,code:u32,parse:P){out.extend_from_slice(&parse.apply(code).to_le_bytes());}
fn allocate_zero_frame(bytes:u16)->Result<u32,String>{
    if bytes!=0{return Err("V66 supervisor init only allocates original zero-byte frame".into());}
    Ok(0xa09d_c000)
}
fn supervisor_interrupt_address(src:Gpr)->Result<u32,String>{
    if src.number()!=0{return Err("V66 SIAD operation only verified with R0".into());}
    Ok(0x6480_c060)
}
fn deallocate_frame()->u32{0x901e_c01e}
/// Conditional `P0=cmp.gtu(R5,#13);if(P0.new)jump:nt` with positive packet
/// start-relative displacement. LLVM6 independently checks source/constant.
fn dispatch_if_greater(here:u32,target:u32,src:Gpr,limit:u8)->Result<u32,String>{
    if here%4!=0 || target%4!=0 ||target<=here ||target-here>0x1fe
       ||src.number()!=5 ||limit!=13{
        return Err("CDSP supervisor dispatch outside checked R5/#13 branch profile".into());
    }
    Ok(0x1100_c000|(u32::from(src.number())<<16)|(u32::from(limit)<<8)|((target-here)/2))
}
/// Indexed memw from absolute table base + R5*4. Under LLVM6 this
/// instruction is emitted as a two-word immext+load, with split low bits.
fn indexed_callback_load(dst:Gpr,index:Gpr,address:u32)->Result<[u32;2],String>{
    if dst.number()>31 || index.number()>31 || address%4!=0 {
        return Err("V66 indexed callback table requires aligned address".into());
    }
    let ext=((address>>20)<<16)|((address>>6)&0x3fff)|0x4000;
    let body=0x9d80_f000|(u32::from(index.number())<<16)
            |(((address&0x3f)>>2)<<8)|u32::from(dst.number());
    Ok([ext,body])
}

pub fn build_supervisor_dispatch()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,allocate_zero_frame(0)?,P::Final);
    let counter=cdsp_boot::encode_extended_assignment(r(0)?,0x04c4_b400,false);
    put(&mut out,counter[0],P::ContinuationA);put(&mut out,counter[1],P::Final);
    put(&mut out,supervisor_interrupt_address(r(0)?)?,P::Final);
    let address=cdsp_boot::encode_extended_assignment(r(0)?,0xfe02_6a70u32 as i32,true);
    put(&mut out,address[0],P::ContinuationA);put(&mut out,address[1],P::Final);
    let count=cdsp_boot::encode_extended_assignment(r(1)?,47,true);
    put(&mut out,count[0],P::ContinuationA);put(&mut out,count[1],P::Final);
    let call=cdsp_early_controllers::encode_extended_call(0x96c0_2a80,0x96c3_23c4)?;
    put(&mut out,call[0],P::ContinuationA);put(&mut out,call[1],P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:0}.encode()?,P::Final);
    put(&mut out,deallocate_frame(),P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=0x34{return Err("CDSP supervisor startup should end at 0x5a94".into());}
    put(&mut out,dispatch_if_greater(0x96c0_2a94,0x96c0_2aa8,r(5)?,13)?,P::Final);
    let table=indexed_callback_load(r(8)?,r(5)?,0xfe02_4d28)?;
    put(&mut out,table[0],P::ContinuationA);
    put(&mut out,table[1],P::Final);
    put(&mut out,H::JumpRegister{source:r(8)?}.encode()?,P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP supervisor startup/dispatch output not 72 bytes".into());}
    Ok(out)
}
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=ALIGNMENT_START ||
       reference.get(ALIGNMENT_START..FILE_OFFSET)!=Some(&[0,0,0,0]){
        return Err("CDSP aligned supervisor stub requires four explicit zero linker bytes".into());
    }
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_supervisor_dispatch()?.as_slice()){
        return Err("CDSP source-built supervisor dispatcher differs from pinned reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn supervisor_start_and_dispatch_fields_are_bounded(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,72);
        assert!(allocate_zero_frame(16).is_err());
        assert!(supervisor_interrupt_address(r(1)).is_err());
        assert_eq!(dispatch_if_greater(0x96c0_2a94,0x96c0_2aa8,r(5),13).unwrap(),0x1105_cd0a);
        assert!(dispatch_if_greater(0x96c0_2a94,0x96c0_2aa8,r(4),13).is_err());
        assert_eq!(indexed_callback_load(r(8),r(5),0xfe02_4d28).unwrap(),[0x0fe0_4934,0x9d85_fa08]);
        assert!(indexed_callback_load(r(8),r(5),0xfe02_4d29).is_err());
        assert_eq!(build_supervisor_dispatch().unwrap().len(),72);
    }
    #[test]
    fn source_executable_and_structural_padding_accounting_is_disjoint() {
        // Earlier checkpoint progress summaries inadvertently counted the
        // internal eight-byte linker gap at 0x4bb8 in their executable sum.
        // Keep the exact disjoint coverage ledger here as an invariant.
        let code_ranges=[(0x41e0usize,0x4418usize),
                         (0x4500,0x4bb8),
                         (0x4bc0,0x5a5c),
                         (FILE_OFFSET,REGION_END)];
        let noncode=[(0x4418usize,0x4500usize),(0x4bb8,0x4bc0),
                     (0x5a5c,FILE_OFFSET)];
        let executable:usize=code_ranges.iter().map(|(start,end)|end-start).sum();
        let alignment:usize=noncode.iter().map(|(start,end)|end-start).sum();
        assert_eq!(executable,6100);
        assert_eq!(alignment,244);
        assert_eq!(executable+alignment,REGION_END-0x41e0);
    }
}
