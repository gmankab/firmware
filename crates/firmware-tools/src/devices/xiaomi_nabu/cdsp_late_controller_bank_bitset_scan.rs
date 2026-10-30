// SPDX-License-Identifier: GPL-3.0-only
//! Initial Hexagon V66 bitmap scan in the CDSP controller bank enumerator.
//! Typed register, immediate and physical branch operands assemble the
//! original packet order at file [0x56bb4,0x56bd0). No OEM code is copied.
use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M, ScalarPacket as S};
use super::cdsp_late_controller_bank_bitset_entry;

pub const FILE_OFFSET:usize=0x56bb4;
pub const END_FILE_OFFSET:usize=0x56bd0;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8bb4;
pub const RWE_START:usize=cdsp_late_controller_bank_bitset_entry::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_bank_bitset_entry::RWE_PREFIX_LAYOUT_LEN;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_bank_bitset_entry::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
const BITMAP_EMPTY:u32=0x96cc_8d4c;
const OUT_OF_RANGE:u32=0x96cc_8e30;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// Compare two register pairs for equality, `P0=cmp.eq(R1:0,R23:22)`.
/// Both even-register fields were independently varied using LLVM6 and
/// checked against Rizin's original Hexagon V66 decoder.
pub fn encode_bitmap_pair_equals(left:G,right:G)->Result<u32,String>{
    if (left.number(),right.number())!=(0,22) || left.number()%2!=0 ||right.number()%2!=0 {
        return Err("unsupported bank bitmap comparison register pair".into());
    }
    Ok(M::ContinuationA.apply(0xd280_c000 | (u32::from(left.number())<<16)
        | (u32::from(right.number())<<8)))
}

/// `if(P0.new) jump:nt` from packet start `0x96cc8bb4`, not from the
/// branch instruction at `0x96cc8bbc`. A separate LLVM6 comparison packet
/// places its branch four bytes later and therefore encodes a shorter offset.
pub fn encode_if_bitmap_empty(packet:u32,target:u32)->Result<u32,String>{
    if (packet,target)!=(PHYSICAL_ADDRESS,BITMAP_EMPTY){
        return Err("unverified bitmap-empty physical packet or target".into());
    }
    let dist=target.checked_sub(packet).ok_or("bitmap-empty branch backwards")?;
    if dist==0 || dist>0x1fe || dist%2!=0{return Err("bitmap-empty branch offset out of range".into());}
    Ok(0x5c00_c800 | (dist>>1))
}

/// `R3=ct0(R2)` with source/destination register fields evidenced by an
/// independent LLVM6 assembly of several register permutations.
pub fn encode_trailing_zero_count(dst:G,src:G)->Result<u32,String>{
    if (dst.number(),src.number())!=(3,2){return Err("unsupported bitmap trailing-zero registers".into());}
    Ok(M::ContinuationA.apply(0x8c40_c080 | (u32::from(src.number())<<16)
        | u32::from(dst.number())))
}

/// Fused `if(cmp.gtu(R3.new,#6)) jump:nt` in the packet beginning at
/// physical `0x96cc8bc0`. The folded 9-bit branch field and signed/immediate
/// compare fields were checked with LLVM6 by changing the target and limit.
pub fn encode_if_bit_index_out_of_range(limit:u8,packet:u32,target:u32)->Result<u32,String>{
    if (limit,packet,target)!=(6,PHYSICAL_ADDRESS+0xc,OUT_OF_RANGE){
        return Err("unsupported bit-index capacity limit or branch packet".into());
    }
    let distance=target.checked_sub(packet).ok_or("bit-index branch backwards")?;
    if distance<0x200 || distance>0x3fe || distance%2!=0 {
        return Err("bit-index branch outside independently checked width".into());
    }
    Ok(0x2512_c000 | (u32::from(limit)<<8) | ((distance>>1)&0xff))
}

/// Scalar shift left `R3=asl(R3,#1)` is encoded by the common checked ISA
/// constructor; `R16=lsl(#1,R3)` is the separately verified reverse-order
/// shift family with explicit destination and shift-register fields.
pub fn encode_one_shifted_by_bit_index(dst:G,shift:G,constant:u8)->Result<u32,String>{
    if (dst.number(),shift.number(),constant)!=(16,3,1){
        return Err("bitmap bit selector must compute R16=lsl(1,R3)".into());
    }
    Ok(0xc680_c0e0 | (u32::from(shift.number())<<8) | u32::from(dst.number()))
}

pub fn build_bitmap_scan()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,M::ContinuationA.apply(S::SetBit{dst:r(2)?,src:r(0)?,position:7}.encode()?));
    emit(&mut out,encode_bitmap_pair_equals(r(0)?,r(22)?)?);
    emit(&mut out,encode_if_bitmap_empty(PHYSICAL_ADDRESS,BITMAP_EMPTY)?);
    emit(&mut out,encode_trailing_zero_count(r(3)?,r(2)?)?);
    emit(&mut out,encode_if_bit_index_out_of_range(6,PHYSICAL_ADDRESS+0xc,OUT_OF_RANGE)?);
    emit(&mut out,S::ArithmeticShiftLeft{dst:r(3)?,src:r(3)?,amount:1}.encode()?);
    emit(&mut out,encode_one_shifted_by_bit_index(r(16)?,r(3)?,1)?);
    if out.len()!=EXECUTABLE_LEN{return Err("CDSP bank scan must contain seven original words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut bytes=cdsp_late_controller_bank_bitset_entry::build_prefix()?;
    bytes.extend(build_bitmap_scan()?);
    if bytes.len()!=FULL_RWE_LAYOUT_LEN || bytes.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN {
        return Err("RWE layout accounting includes unexpected alignment/code bytes".into());
    }
    Ok(bytes)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bitmap_scan()?.as_slice()){
        return Err("CDSP bank-scan typed region differs from original".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP bank-scan extended RWE prefix differs from reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn source_ops_have_exact_packet_fields(){
        let bytes=build_bitmap_scan().unwrap();assert_eq!(bytes.len(),28);
        let words:Vec<u32>=bytes.chunks_exact(4).map(|x|u32::from_le_bytes(x.try_into().unwrap())).collect();
        assert_eq!(words,[0x8cc0_4702,0xd280_5600,0x5c00_c8cc,0x8c42_4083,
                           0x2512_c638,0x8c03_c143,0xc680_c3f0]);
        assert!(encode_bitmap_pair_equals(r(2).unwrap(),r(22).unwrap()).is_err());
        assert!(encode_if_bitmap_empty(PHYSICAL_ADDRESS+4,BITMAP_EMPTY).is_err());
        assert!(encode_if_bit_index_out_of_range(5,PHYSICAL_ADDRESS+12,OUT_OF_RANGE).is_err());
        assert!(encode_one_shifted_by_bit_index(r(16).unwrap(),r(2).unwrap(),1).is_err());
    }
    #[test]fn joined_source_prefix_tracks_instruction_vs_alignment_bytes(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,3024);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2964);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),3024);
    }
}
