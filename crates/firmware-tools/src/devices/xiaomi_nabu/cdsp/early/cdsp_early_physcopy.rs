// SPDX-License-Identifier: GPL-3.0-only
//! Auxiliary Hexagon V66 controller physical-word transfer to an output ring.
//! The routine is adjacent to the two prior reconstructed code functions.
//! Privileged `memw_phys` is confirmed by Rizin V66 decoding, whereas the
//! variable shift and memory postincrement are independently LLVM6 assembled.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P,
                      ScalarPacket as H};
use super::cdsp_early_copier;

pub const FILE_OFFSET:usize=0x4318;
pub const REGION_END:usize=0x4364;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1318;
const CODE_BYTES:usize=REGION_END-FILE_OFFSET;

#[derive(Copy,Clone,Debug)]
enum PhysOperation{
    LogicalShiftRightByRegister{dst:Gpr,value:Gpr,amount_reg:Gpr},
    LoadPhysicalWord{dst:Gpr,address:Gpr,selector:Gpr},
    StoreWordPostIncrement{src:Gpr,address:Gpr,step:u8},
}
impl PhysOperation{
    fn encode(self)->Result<u32,String>{
        use PhysOperation::*;
        match self{
            LogicalShiftRightByRegister{dst,value,amount_reg}=>Ok(0xc640_c040 |u32::from(value.number())<<16 |u32::from(amount_reg.number())<<8|u32::from(dst.number())),
            // The selector occupies bits 12:8; bit 12 is **not** a fixed
            // opcode bit. The generic earlier CDSP ReadPhysicalWord helper
            // was only exercised with selectors that had bit 4 set. This
            // checked local constructor avoids altering that shared module.
            LoadPhysicalWord{dst,address,selector}=>Ok(0x9200_e000|u32::from(address.number())<<16|u32::from(selector.number())<<8|u32::from(dst.number())),
            StoreWordPostIncrement{src,address,step}=>{
                if step==0||step>28||step%4!=0 {return Err("Hexagon word store postincrement requires 4-byte aligned nonzero stride <=28".into());}
                Ok(0xab80_c000|u32::from(address.number())<<16|u32::from(src.number())<<8|u32::from(step)*2)
            }
        }
    }
}

fn push(out:&mut Vec<u8>, word:u32, marker:P){
    out.extend_from_slice(&marker.apply(word).to_le_bytes());
}
fn gp(out:&mut Vec<u8>,reg:Gpr,absolute:u32,is_final:bool){
    let words=cdsp_boot::encode_extended_gp_word_load(reg,absolute);
    push(out,words[0],P::ContinuationA);
    push(out,words[1],if is_final {P::Final} else {P::ContinuationA});
}

/// Read source/size from controller GP slots, validate both and copy mapped
/// physical words through a 32-bit hardware selector. Preserve the Hexagon
/// loop0 end-marker on pointer increment rather than following store.
pub fn build_physical_copy()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(CODE_BYTES);
    gp(&mut out,r(1)?,0xfe0a_5e68,false);
    gp(&mut out,r(3)?,0xfe0c_6680,true);
    push(&mut out,H::SkipIfSourceZeroNotTaken{src:r(1)?,displacement_bytes:0x38}.encode()?,P::Final);
    push(&mut out,H::SkipIfSourceZeroNotTaken{src:r(3)?,displacement_bytes:0x34}.encode()?,P::Final);
    push(&mut out,H::PredicateP0GreaterThan32{src:r(3)?}.encode()?,P::ContinuationA);
    push(&mut out,H::JumpIfNotNewP0NotTaken{displacement_bytes:0x30}.encode()?,P::ContinuationA);
    gp(&mut out,r(0)?,0xfe0c_6734,true);
    push(&mut out,H::LogicalShiftRight{dst:r(2)?,src:r(3)?,amount:2}.encode()?,P::ContinuationA);
    push(&mut out,H::SkipIfSourceZeroNotTaken{src:r(0)?,displacement_bytes:0x20}.encode()?,P::Final);
    push(&mut out,H::HardwareLoop{loop_index:0,counter:r(2)?,displacement_bytes:8}.encode()?,P::ContinuationA);
    push(&mut out,H::AssignSmallImmediate{dst:r(2)?,value:11}.encode()?,P::Final);
    push(&mut out,PhysOperation::LogicalShiftRightByRegister{dst:r(4)?,value:r(0)?,amount_reg:r(2)?}.encode()?,P::Final);
    push(&mut out,PhysOperation::LoadPhysicalWord{dst:r(5)?,address:r(0)?,selector:r(4)?}.encode()?,P::Final);
    push(&mut out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:4}.encode()?,P::LoopEnd);
    push(&mut out,PhysOperation::StoreWordPostIncrement{src:r(5)?,address:r(1)?,step:4}.encode()?,P::Final);
    push(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=CODE_BYTES{return Err("CDSP physcopy instruction count mismatch".into());}
    Ok(out)
}

pub fn build_joined()->Result<Vec<u8>,String>{
    let mut output=cdsp_early_copier::build_joined()?;
    if cdsp_early_copier::REGION_END!=FILE_OFFSET {return Err("CDSP copy/physcopy function gap".into());}
    output.extend(build_physical_copy()?);
    if output.len()!=REGION_END-super::cdsp_early_tlb::FILE_OFFSET{return Err("CDSP joined controller code size mismatch".into());}
    Ok(output)
}

pub fn verify_joined(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_joined()?;
    let offset=super::cdsp_early_tlb::FILE_OFFSET;
    let expected=reference.get(offset..REGION_END).ok_or("CDSP truncated in physcopy")?;
    if let Some((i,(x,y)))=out.iter().zip(expected).enumerate().find(|(_, (x,y))|x!=y){
        return Err(format!("CDSP physcopy differs at {:#x}: generated {x:02x}, reference {y:02x}",offset+i));
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn precise_physical_address_and_adjacency(){
        assert_eq!(build_physical_copy().unwrap().len(),CODE_BYTES);
        assert_eq!(build_joined().unwrap().len(),0x184);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn physical_memory_encoder_rejects_unsupported_stride(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(PhysOperation::LoadPhysicalWord{dst:r(5),address:r(0),selector:r(4)}.encode().unwrap(),0x9200_e405);
        assert_eq!(PhysOperation::LogicalShiftRightByRegister{dst:r(4),value:r(0),amount_reg:r(2)}.encode().unwrap(),0xc640_c244);
        assert_eq!(PhysOperation::StoreWordPostIncrement{src:r(5),address:r(1),step:4}.encode().unwrap(),0xab81_c508);
        for stride in [0,2,3,32]{assert!(PhysOperation::StoreWordPostIncrement{src:r(5),address:r(1),step:stride}.encode().is_err());}
    }
}
