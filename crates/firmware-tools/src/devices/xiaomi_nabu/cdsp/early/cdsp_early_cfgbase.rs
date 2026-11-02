// SPDX-License-Identifier: GPL-3.0-only
//! Early CDSP CFGBASE probe and GP new-value store. The 36 executable bytes
//! run up to file 0x4418. Following 232 zero bytes to 0x4500 are validated
//! separately as *linker alignment*, not source-reconstructed machine code.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,
                      ScalarPacket as H,SystemRegister};
use super::cdsp_early_compare;

pub const FILE_OFFSET:usize=0x43f4;
pub const FUNCTION_BYTES:usize=36;
pub const REGION_END:usize=FILE_OFFSET+FUNCTION_BYTES;
pub const ZERO_FILL_END:usize=0x4500;
pub const ZERO_FILL_BYTES:usize=ZERO_FILL_END-REGION_END;
pub const PHYSICAL_ADDRESS:u32=0x96c0_13f4;

/// Generate V66 `memw(GP+##abs)=R2.new`, with the new value supplied by an
/// earlier producer in this same packet. The address contributes bits 31:6
/// to IMMEXT and bits 5:0 to the consumer. The special NEW source selector
/// is fixed to register R2; other forms are rejected rather than guessed.
fn gp_new_word_store_r2(address:u32)->Result<[u32;2],String>{
    if address%4!=0{return Err("Hexagon GP new-value store must be word aligned".into());}
    let ext=((address>>20)<<16)|((address>>6)&0x3fff)|0x4000;
    let body=0x48a0_d400|(address&0x3f);
    Ok([ext,body])
}
fn put(out:&mut Vec<u8>,instruction:H,parse:P)->Result<(),String>{
    out.extend_from_slice(&parse.apply(instruction.encode()?).to_le_bytes());
    Ok(())
}

pub fn build_cfgbase_probe()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut output=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut output,H::ReadSystem{system:SystemRegister::Cfgbase,dst:r(1)?},P::Final)?;
    put(&mut output,H::ArithmeticShiftLeft{dst:r(1)?,src:r(1)?,amount:5},P::ContinuationA)?;
    let zero=cdsp_boot::encode_extended_assignment(r(0)?,0,true);
    output.extend_from_slice(&zero[0].to_le_bytes());
    output.extend_from_slice(&zero[1].to_le_bytes());
    // V66 physical read: base+selector are independent registers, and the
    // selector bit 4 is not a fixed opcode bit (Rizin corroborated).
    let mem_phys=0x9200_e000|u32::from(r(0)?.number())<<16|u32::from(r(1)?.number())<<8|u32::from(r(2)?.number());
    output.extend_from_slice(&mem_phys.to_le_bytes());
    put(&mut output,H::ArithmeticShiftLeft{dst:r(2)?,src:r(2)?,amount:16},P::ContinuationA)?;
    put(&mut output,H::JumpRegister{source:r(31)?},P::ContinuationA)?;
    let gp=gp_new_word_store_r2(0xfe0c_6734)?;
    output.extend_from_slice(&P::ContinuationA.apply(gp[0]).to_le_bytes());
    output.extend_from_slice(&gp[1].to_le_bytes());
    if output.len()!=FUNCTION_BYTES{return Err("CDSP CFGBASE probe length mismatch".into());}
    Ok(output)
}

pub fn build_joined()->Result<Vec<u8>,String>{
    let mut output=cdsp_early_compare::build_joined()?;
    if cdsp_early_compare::REGION_END!=FILE_OFFSET{return Err("CDSP compare/CFGBASE routine offset gap".into());}
    output.extend(build_cfgbase_probe()?);
    if output.len()!=REGION_END-super::cdsp_early_tlb::FILE_OFFSET{return Err("CDSP CFGBASE joined length mismatch".into());}
    Ok(output)
}

/// Structural zero-fill is verified against the private reference but never
/// counted as independent executable source or appended to build_joined().
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let code=build_joined()?;
    let source_start=super::cdsp_early_tlb::FILE_OFFSET;
    if reference.get(source_start..REGION_END)!=Some(code.as_slice()){
        return Err("CDSP CFGBASE join differs from pinned executable text".into());
    }
    if reference.get(REGION_END..ZERO_FILL_END)!=Some([0u8;ZERO_FILL_BYTES].as_slice()){
        return Err("CDSP expected 0x4418..0x4500 ELF alignment bytes are nonzero".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test] fn cfgbase_probe_is_executable_without_embedded_padding(){
        assert_eq!(build_cfgbase_probe().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(build_joined().unwrap().len(),0x238);
        assert_eq!(ZERO_FILL_BYTES,232);
        assert_eq!(PHYSICAL_ADDRESS,0x96c00000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn gp_new_store_checks_alignment(){
        assert!(gp_new_word_store_r2(0xfe0c_6732).is_err());
        assert_eq!(gp_new_word_store_r2(0xfe0c_6734).unwrap(),[0x0fe0_719c,0x48a0_d434]);
    }
}
