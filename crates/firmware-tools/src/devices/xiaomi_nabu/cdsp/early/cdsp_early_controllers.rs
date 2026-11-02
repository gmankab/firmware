// SPDX-License-Identifier: GPL-3.0-only
//! Small CDSP Hexagon controller bootstrap function following the verified
//! early TLB scrub. Save/restore LR, install the controller stack, call TLB
//! sweep, load input context via GP and call the next callback at 0x42bc.
//!
//! The call extender is packet-start relative; using the address of the
//! second instruction would be incorrect by four bytes and fails exact match.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_tlb;

pub const FILE_OFFSET:usize=0x4290;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1290;
pub const FUNCTION_SIZE:usize=0x2c;
pub const REGION_END:usize=FILE_OFFSET+FUNCTION_SIZE;

/// Force the standard 2-word Hexagon extended `call` form. The displacement
/// is measured from the packet's start, not the address of the call word.
/// This is checked on LLVM6 independently, for negative and positive values.
pub fn encode_extended_call(packet_start:u32,target:u32)->Result<[u32;2],String>{
    if packet_start%4!=0||target%4!=0 {return Err("Hexagon CALL packet/target must be 4-byte aligned".into());}
    let displacement=i64::from(target)-i64::from(packet_start);
    if displacement < i64::from(i32::MIN)||displacement>i64::from(i32::MAX){
        return Err("Hexagon CALL displacement exceeds signed32".into());
    }
    let bits=displacement as i32 as u32;
    let extender=((bits>>20)<<16)|((bits>>6)&0x3fff)|0x4000;
    let branch=0x5a00_c000|((bits&0x3f)<<1);
    Ok([extender,branch])
}

/// Reconstruct source-authored 44-byte startup subroutine, no opaque bytes.
pub fn build_controller_startup()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_SIZE);
    let mut emit=|instruction:u32, marker:P|{
        out.extend_from_slice(&marker.apply(instruction).to_le_bytes());
    };
    emit(H::AssignRegister{dst:r(17)?,src:r(31)?}.encode()?,P::Final);
    let stack=cdsp_boot::encode_extended_assignment(r(29)?,-0x1fc8c98,true);
    emit(stack[0],P::ContinuationA);
    emit(stack[1],P::Final);
    let sweep=encode_extended_call(0x96c0129c,0x96c011e0)?;
    emit(sweep[0],P::ContinuationA);
    emit(sweep[1],P::Final);
    let gp_input=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe0c6728);
    emit(gp_input[0],P::ContinuationA);
    emit(gp_input[1],P::Final);
    let next=encode_extended_call(0x96c012ac,0x96c012bc)?;
    emit(next[0],P::ContinuationA);
    emit(next[1],P::Final);
    emit(H::AssignRegister{dst:r(31)?,src:r(17)?}.encode()?,P::Final);
    emit(H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_SIZE{return Err("CDSP startup output size mismatch".into());}
    Ok(out)
}

pub fn build_joined()->Result<Vec<u8>,String>{
    let mut out=cdsp_early_tlb::build_span()?;
    if cdsp_early_tlb::REGION_END!=FILE_OFFSET {return Err("CDSP TLB/startup boundary gap".into());}
    out.extend(build_controller_startup()?);
    if out.len()!=REGION_END-cdsp_early_tlb::FILE_OFFSET {return Err("CDSP joined code size mismatch".into());}
    Ok(out)
}

pub fn verify_joined(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let output=build_joined()?;
    let file_offset=cdsp_early_tlb::FILE_OFFSET;
    let actual=reference.get(file_offset..REGION_END).ok_or("CDSP firmware truncation in startup")?;
    if let Some((i,(x,y)))=output.iter().zip(actual).enumerate().find(|(_, (x,y))|x!=y){
        return Err(format!("CDSP TLB/startup mismatch at file {:#x}: generated {x:02x}, reference {y:02x}",file_offset+i));
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn extended_call_tracks_packet_start_and_sign(){
        assert_eq!(encode_extended_call(0x96c0129c,0x96c011e0).unwrap(),[0x0fff_7ffd,0x5a00_c008]);
        assert_eq!(encode_extended_call(0x96c012ac,0x96c012bc).unwrap(),[0x0000_4000,0x5a00_c020]);
        assert!(encode_extended_call(1,4).is_err());
    }
    #[test]fn controller_startup_abuts_tlb_body(){
        assert_eq!(build_controller_startup().unwrap().len(),FUNCTION_SIZE);
        assert_eq!(build_joined().unwrap().len(),0xdc);
        assert_eq!(PHYSICAL_ADDRESS,0x96c00000+(FILE_OFFSET-0x3000) as u32);
    }
}
