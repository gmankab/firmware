// SPDX-License-Identifier: GPL-3.0-only
//! First 80 bytes of the CDSP Hexagon V66 cache-controller setup.
//! The routine is incomplete: the cache-clean loop and return follow this
//! prefix. All words are assembled from typed operands, packet parse markers
//! and named offsets, not extracted reference opcode arrays.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P,
                      ScalarPacket as H, SystemRegister};

pub const FILE_OFFSET: usize=0x3f24;
pub const PHYSICAL_ADDRESS: u32=0x96c0_0f24;
pub const PREFIX_SIZE:usize=80;

fn put(out:&mut Vec<u8>, op:H, marker:P)->Result<(),String>{
    out.extend_from_slice(&marker.apply(op.encode()?).to_le_bytes());
    Ok(())
}

/// Read three physical configuration words using CFGBASE-scaled addressing;
/// each value is shifted into the appropriate device register field and
/// written through the controller's negative-offset register window.
pub fn build_prefix()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut bytes=Vec::with_capacity(PREFIX_SIZE);
    put(&mut bytes,H::ReadSystem{system:SystemRegister::Cfgbase,dst:r(21)?},P::Final)?;
    put(&mut bytes,H::ArithmeticShiftLeft{dst:r(21)?,src:r(21)?,amount:5},P::ContinuationA)?;
    for (address,shift,write_offset) in [(76u32,16u8,-0x1fe92a4i32),
                                                 (56,16,-0x1fe92a0),
                                                 (60,10,-0x1fe929c)]
                                              .into_iter(){
        // The original package explicitly uses immext even for small
        // constants; forcing the two-word form preserves relocation width.
        for word in cdsp_boot::encode_extended_assignment(r(22)?,address as i32,true) {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        put(&mut bytes,H::ReadPhysicalWord{dst:r(23)?,offset:r(22)?,base:r(21)?},P::Final)?;
        put(&mut bytes,H::ArithmeticShiftLeft{dst:r(23)?,src:r(23)?,amount:shift},P::ContinuationA)?;
        for word in cdsp_boot::encode_extended_new_value_store(r(25)?,write_offset)? {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
    }
    if bytes.len()!=PREFIX_SIZE {
        return Err(format!("CDSP CFGBASE setup length {} != {PREFIX_SIZE}",bytes.len()));
    }
    Ok(bytes)
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_prefix()?;
    let src=reference.get(FILE_OFFSET..FILE_OFFSET+PREFIX_SIZE)
        .ok_or("CDSP reference too short for CFGBASE setup")?;
    if let Some((i,(x,y)))=out.iter().zip(src).enumerate().find(|(_, (x,y))|x!=y) {
        return Err(format!("CDSP CFGBASE mismatch at file {:#x}, generated {x:02x}, reference {y:02x}",FILE_OFFSET+i));
    }
    Ok(())
}

pub fn build_joined_cache_controller_prefix()->Result<Vec<u8>,String>{
    let mut code=super::cdsp_copyfast::build_cluster_with_copyfast()?;
    if super::cdsp_copyfast::FILE_OFFSET + super::cdsp_copyfast::TOTAL_BYTES != FILE_OFFSET {
        return Err("gap before CDSP cache-controller setup".into());
    }
    code.extend(build_prefix()?);
    if code.len()!=484+PREFIX_SIZE { return Err("joined CFGBASE/fastcopy span length incorrect".into()); }
    Ok(code)
}

pub fn verify_joined_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_joined_cache_controller_prefix()?;
    let begin=super::cdsp_tlb::COMPLETE_CLUSTER_OFFSET;
    if reference.get(begin..begin+out.len())!=Some(out.as_slice()) {
        return Err("CDSP contiguous supervisor/TLB/cache/CFGBASE bytes differ".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test] fn typed_cache_controller_setup_has_explicit_packets(){
        assert_eq!(build_prefix().unwrap().len(),PREFIX_SIZE);
        assert_eq!(build_joined_cache_controller_prefix().unwrap().len(),564);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn new_value_store_checks_alignment_and_preserves_system_registers(){
        let r=|x|Gpr::new(x).unwrap();
        assert_eq!(H::ReadSystem{system:SystemRegister::Cfgbase,dst:r(21)}.encode().unwrap(),0x6e9b_c015);
        assert_eq!(H::ReadPhysicalWord{dst:r(23),offset:r(22),base:r(21)}.encode().unwrap(),0x9216_f517);
        assert_eq!(cdsp_boot::encode_extended_new_value_store(r(25),-0x1fe92a4).unwrap(),[0x0fe0_45b5,0xa1b9_d21c]);
        assert!(cdsp_boot::encode_extended_new_value_store(r(25),-3).is_err());
    }
}
