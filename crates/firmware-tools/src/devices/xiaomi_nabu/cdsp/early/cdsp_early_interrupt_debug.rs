// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 controller interrupt-debug setup. This function continues
//! the separately reconstructed supervisor routine without crossing the
//! 0x4418..0x4500 linker alignment (which is not executable source).
//! Privileged physical accesses are only decoder-corroborated in Rizin V66;
//! scalar operations can be independently checked with LLVM6 where supported.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,
                      ScalarPacket as H,SystemRegister};
use super::cdsp_early_system_start;

pub const FILE_OFFSET:usize=0x4568;
pub const REGION_END:usize=0x45c0;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1568;

#[derive(Clone,Copy,Debug)]
enum ControllerRegister { InterruptDebugEnable }
impl ControllerRegister {
    fn selector(self)->u32 { match self {Self::InterruptDebugEnable=>42} }
    fn read(self,dst:Gpr)->u32 {0x6e80_c000|(self.selector()<<16)|u32::from(dst.number())}
    fn write(self,src:Gpr)->u32 {0x6700_c000|(u32::from(src.number())<<16)|self.selector()}
}

fn branch_if_not_p0_new_taken(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0 || target%4!=0 || target<=packet_start {
        return Err("interrupt-debug conditional jump must be aligned and forward".into());
    }
    let bytes=target-packet_start;
    if bytes>0x1fe || bytes%2!=0 {
        return Err("interrupt-debug conditional branch outside checked short range".into());
    }
    Ok(0x5c20_d800 | bytes/2)
}

/// Hexagon V66 `Rd=memw_phys(Roffset,Rmapped)`.
/// The selector bit that differs from the older generic implementation is
/// architecture-specific. Distinct operands occupy [20:16], [12:8], [4:0].
fn physical_read(dst:Gpr,offset:Gpr,mapped:Gpr)->u32{
    0x9200_e000|(u32::from(offset.number())<<16)
        |(u32::from(mapped.number())<<8)|u32::from(dst.number())
}
fn and_registers(dst:Gpr,left:Gpr,right:Gpr)->u32{
    0xf100_c000|(u32::from(left.number())<<16)
        |(u32::from(right.number())<<8)|u32::from(dst.number())
}
fn add_negative_11(dst:Gpr,src:Gpr,value:i16)->Result<u32,String>{
    if !(-1024..=-1).contains(&value) {
        return Err("checked signed Hexagon ADD immediate must be in [-1024,-1]".into());
    }
    Ok(0xbfe0_0000 |(u32::from(src.number())<<16)
        | (((i32::from(value) as u32)&0x7ff)<<5)
        |u32::from(dst.number()))
}

fn append(out:&mut Vec<u8>,word:u32,marker:P){
    out.extend_from_slice(&marker.apply(word).to_le_bytes());
}

pub fn build_interrupt_debug()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    append(&mut out,ControllerRegister::InterruptDebugEnable.read(r(1)?),P::Final);
    append(&mut out,H::TestBitPredicate{predicate:0,src:r(1)?,bit:2}.encode()?,P::ContinuationA);
    append(&mut out,branch_if_not_p0_new_taken(0x96c0_156c,0x96c0_15b0)?,P::Final);
    append(&mut out,H::ReadSystem{system:SystemRegister::Cfgbase,dst:r(1)?}.encode()?,P::ContinuationA);
    append(&mut out,H::AssignSmallImmediate{dst:r(25)?,value:0}.encode()?,P::Final);
    append(&mut out,H::InsertBitField{dst:r(1)?,src:r(25)?,width:12,offset:20}.encode()?,P::Final);
    append(&mut out,H::ArithmeticShiftLeft{dst:r(1)?,src:r(1)?,amount:5}.encode()?,P::ContinuationA);
    append(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:8}.encode()?,P::Final);
    append(&mut out,physical_read(r(1)?,r(0)?,r(1)?),P::Final);
    append(&mut out,H::InsertBitField{dst:r(1)?,src:r(25)?,width:12,offset:20}.encode()?,P::Final);
    append(&mut out,H::ArithmeticShiftLeft{dst:r(1)?,src:r(1)?,amount:5}.encode()?,P::ContinuationA);
    append(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:24}.encode()?,P::Final);
    append(&mut out,add_negative_11(r(1)?,r(1)?,-256)?,P::Final);
    append(&mut out,physical_read(r(1)?,r(0)?,r(1)?),P::Final);
    let gp=cdsp_boot::encode_extended_word_load(r(24)?,r(23)?,-0x1fd92f4)?;
    append(&mut out,gp[0],P::ContinuationA);
    append(&mut out,gp[1],P::Final);
    append(&mut out,and_registers(r(0)?,r(1)?,r(24)?),P::Final);
    append(&mut out,H::SkipIfSourceZero{src:r(0)?,displacement_bytes:20}.encode()?,P::Final);
    append(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:1}.encode()?,P::Final);
    append(&mut out,ControllerRegister::InterruptDebugEnable.write(r(1)?),P::Final);
    append(&mut out,H::Isync.encode()?,P::Final);
    // Hexagon V66 unprivileged software breakpoint is a fixed architectural
    // opcode with no register operands, not data extracted from the MBN.
    append(&mut out,0x6c20_c000,P::Final);
    if out.len()!=FUNCTION_BYTES {return Err("Hexagon interrupt-debug length mismatch".into());}
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_system_start::verify_reference(reference)?;
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_interrupt_debug()?.as_slice()){
        return Err("CDSP interrupt debug sequence differs from pinned MBN".into());
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn interrupt_debug_instruction_region_is_exact(){
        assert_eq!(FILE_OFFSET,cdsp_early_system_start::REGION_END);
        assert_eq!(FUNCTION_BYTES,88);
        assert_eq!(build_interrupt_debug().unwrap().len(),88);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test] fn interrupt_branches_are_packet_start_relative(){
        assert_eq!(branch_if_not_p0_new_taken(0x96c0_156c,0x96c0_15b0).unwrap(),0x5c20_d822);
        assert!(branch_if_not_p0_new_taken(0x96c0_156c,0x96c0_156c).is_err());
        assert!(branch_if_not_p0_new_taken(0x96c0_156d,0x96c0_15b0).is_err());
        assert!(branch_if_not_p0_new_taken(0x96c0_156c,0x96c0_1900).is_err());
    }
    #[test]fn privileged_operand_fields_are_checked(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(ControllerRegister::InterruptDebugEnable.read(r(1)),0x6eaa_c001);
        assert_eq!(ControllerRegister::InterruptDebugEnable.write(r(1)),0x6701_c02a);
        assert_eq!(physical_read(r(1),r(0),r(1)),0x9200_e101);
        assert_ne!(physical_read(r(1),r(2),r(1)),physical_read(r(1),r(0),r(1)));
        assert_eq!(add_negative_11(r(1),r(1),-256).unwrap(),0xbfe1_e001);
        assert!(add_negative_11(r(1),r(1),-1025).is_err());
    }
}
