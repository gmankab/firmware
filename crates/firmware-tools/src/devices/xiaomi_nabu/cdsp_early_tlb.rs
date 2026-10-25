// SPDX-License-Identifier: GPL-3.0-only
//! First QDSP6 v66 controller TLB invalidation service immediately after the
//! previously reconstructed CDSP DMA copy, at ELF file 0x41e0. The builder
//! names registers, packet-end flags and typed immediate fields; it contains
//! no OEM instruction dumps and does not import bytes from cdsp.mbn.
//!
//! Physical runtime address = 0x96c00000 + (file offset - 0x3000).
//! ELF vaddr is a different mapping and is NOT used for branch arithmetic.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P,
                      ScalarPacket as H};

pub const FILE_OFFSET: usize = 0x41e0;
pub const PHYSICAL_ADDRESS: u32 = 0x96c0_11e0;
pub const PREFIX_LEN: usize = 0x30;
pub const REGION_END: usize = FILE_OFFSET + PREFIX_LEN;

/// Rebuild the initial controller TLB sweep setup from typed Hexagon
/// instructions, preserving the two multi-operation instruction packets.
/// The following `tlbr` sweep and exceptional fallback are not yet emitted.
fn write(code:&mut Vec<u8>,instr:H,parse:P)->Result<(),String>{
    code.extend_from_slice(&parse.apply(instr.encode()?).to_le_bytes());
    Ok(())
}

pub fn build_prefix() -> Result<Vec<u8>,String> {
    let r=|n| Gpr::new(n);
    let mut code=Vec::with_capacity(PREFIX_LEN);
    let gp_counter=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe0c_6734);
    code.extend_from_slice(&gp_counter[0].to_le_bytes());
    code.extend_from_slice(&P::ContinuationA.apply(gp_counter[1]).to_le_bytes());
    let gp_table=cdsp_boot::encode_extended_gp_word_load(r(1)?,0xfe0c_6680);
    code.extend_from_slice(&gp_table[0].to_le_bytes());
    code.extend_from_slice(&gp_table[1].to_le_bytes());
    write(&mut code,H::SkipIfSourceZeroNotTaken{src:r(1)?,displacement_bytes:0x9c},P::Final)?;
    write(&mut code,H::LogicalShiftRight{dst:r(0)?,src:r(0)?,amount:12},P::ContinuationA)?;
    write(&mut code,H::LogicalShiftRight{dst:r(1)?,src:r(1)?,amount:12},P::ContinuationA)?;
    for word in cdsp_boot::encode_extended_assignment(r(10)?,0x3f_ffff,true) {
        code.extend_from_slice(&word.to_le_bytes());
    }
    write(&mut code,H::HardwareLoopImmediate{loop_index:0,iterations:127,displacement_bytes:12},P::ContinuationA)?;
    write(&mut code,H::AddRegisters{dst:r(2)?,left:r(0)?,right:r(1)?},P::ContinuationA)?;
    write(&mut code,H::AssignSmallImmediate{dst:r(3)?,value:0},P::Final)?;
    if code.len()!=PREFIX_LEN {return Err(format!("CDSP controller prefix {} != {}",code.len(),PREFIX_LEN));}
    Ok(code)
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let got=build_prefix()?;
    let expected=reference.get(FILE_OFFSET..REGION_END).ok_or("CDSP file too short for controller prefix")?;
    if let Some((i,(x,y)))=got.iter().zip(expected).enumerate().find(|(_, (x,y))|x!=y) {
        return Err(format!("CDSP controller prefix differs at file {:#x}: generated {x:02x}, reference {y:02x}",FILE_OFFSET+i));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn typed_tlb_sweep_prefix_and_physical_offset() {
        let code=build_prefix().unwrap();
        assert_eq!(code.len(),PREFIX_LEN);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test] fn branches_and_packet_boundaries_are_checked() {
        let r1=Gpr::new(1).unwrap();
        assert!(H::SkipIfSourceZeroNotTaken{src:r1,displacement_bytes:0x9d}.encode().is_err());
        assert!(H::HardwareLoopImmediate{loop_index:0,iterations:0,displacement_bytes:12}.encode().is_err());
        assert!(H::LogicalShiftRight{dst:r1,src:r1,amount:32}.encode().is_err());
    }
}
