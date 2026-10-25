// SPDX-License-Identifier: GPL-3.0-only
//! CDSP later `R E` code segment: initialization of a three-word controller
//! MMIO register bank. This is a Hexagon V66 **code** region, unlike the
//! preceding `R E` ELF segment that contains mostly data and structural fill.
//!
//! Address spaces are distinct: CDSP MBN file [0x55000,0x55038), ELF VMA
//! [0xc00c7000,0xc00c7038), physical [0x96cc7000,0x96cc7038).
//! The direct jump to physical 0x96cc7070 is external to this short fragment.
//! No original firmware opcode sequences are embedded in this source.
use super::cdsp_boot::{self,GeneralRegister,ScalarPacket,PacketMarker};

pub const FILE_OFFSET:usize=0x55000;
pub const ELF_VADDR:u32=0xc00c7000;
pub const PHYSICAL_ADDRESS:u32=0x96cc7000;
pub const FIRST_ENTRY_LEN:usize=0x38;

/// Checked Hexagon `memw(Rbase + #offset) = ##value` with an immediate extender.
/// Only the small verified 32-bit, word-aligned addressing form is admitted;
/// it is independently field-tested with LLVM6 and decoded with Rizin V66.
/// The two words constitute one packet, extender followed by the consuming
/// immediate store. No generic memory-store support is implied.
pub fn encode_extended_word_store_immediate(base:GeneralRegister,
                                            offset_bytes:u8,value:u32)->Result<[u32;2],String>{
    if offset_bytes%4!=0 || offset_bytes>12 {
        return Err("Hexagon immediate word store: offset must be 0,4,8,12".into());
    }
    let extender=((value>>20)<<16)|((value>>6)&0x3fff)|0x4000;
    let store=0x3c40_c000 | (u32::from(base.number())<<16)
        | (u32::from(offset_bytes)<<5) | (value&0x3f);
    Ok([extender,store])
}

fn push_word(out:&mut Vec<u8>,value:u32){out.extend_from_slice(&value.to_le_bytes());}

/// Three MMIO control writes: addresses 0x83a0040, +4 and +8 each receive 7.
/// A Hexagon barrier orders the writes before control transfers to the shared
/// state routine at physical address 0x96cc7070. The emitter reads *no*
/// private reference bytes; the OEM firmware is solely an external verifier.
pub fn build_mmio_setup()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(FIRST_ENTRY_LEN);
    let r0=GeneralRegister::new(0)?;
    for register_address in [0x083a_0040u32,0x083a_0044,0x083a_0048] {
        for word in cdsp_boot::encode_extended_assignment(r0,register_address as i32,true){
            push_word(&mut out,word);
        }
        for word in encode_extended_word_store_immediate(r0,0,7)? {
            push_word(&mut out,word);
        }
    }
    // Hexagon `barrier`, independently encoded by LLVM6 and decoded by Rizin.
    push_word(&mut out,0xa800_c000);
    // Hexagon `jump +0x3c`: from physical 0x96cc7034 to 0x96cc7070.
    push_word(&mut out,PacketMarker::Final.apply(
        ScalarPacket::JumpRelative{displacement_bytes:0x3c}.encode()?));
    if out.len()!=FIRST_ENTRY_LEN{return Err("CDSP later MMIO setup length != 56".into());}
    Ok(out)
}

pub fn verify_mmio_setup(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let actual=build_mmio_setup()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+FIRST_ENTRY_LEN)!=Some(actual.as_slice()){
        return Err("CDSP later MMIO setup differs from pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mmio_store_immediate_fields_are_checked(){
        let r0=GeneralRegister::new(0).unwrap();
        let r1=GeneralRegister::new(1).unwrap();
        assert_eq!(encode_extended_word_store_immediate(r0,0,7).unwrap(),[0x0000_4000,0x3c40_c007]);
        assert_eq!(encode_extended_word_store_immediate(r1,4,11).unwrap(),[0x0000_4000,0x3c41_c08b]);
        assert_eq!(encode_extended_word_store_immediate(r0,0,0x1234).unwrap(),[0x0000_4048,0x3c40_c034]);
        assert!(encode_extended_word_store_immediate(r0,2,7).is_err());
        assert!(encode_extended_word_store_immediate(r0,16,7).is_err());
    }
    #[test]
    fn initial_mmio_setup_typed_packets_are_contiguous(){
        let bytes=build_mmio_setup().unwrap();
        assert_eq!(bytes.len(),FIRST_ENTRY_LEN);
        let words:Vec<_>=bytes.chunks_exact(4).map(|b|u32::from_le_bytes(b.try_into().unwrap())).collect();
        assert_eq!(words[0],cdsp_boot::encode_extended_assignment(GeneralRegister::new(0).unwrap(),0x083a0040,true)[0]);
        assert_eq!(words[12],0xa800_c000);
        assert_eq!(words[13],ScalarPacket::JumpRelative{displacement_bytes:0x3c}.encode().unwrap());
        assert_eq!(ELF_VADDR+FIRST_ENTRY_LEN as u32,0xc00c7038);
        assert_eq!(PHYSICAL_ADDRESS+FIRST_ENTRY_LEN as u32,0x96cc7038);
    }
}
