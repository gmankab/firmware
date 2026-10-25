// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 64-byte DMA-style doubleword loop after cache initialization.
//! The previous controller function has 12 alignment bytes after `jumpr LR`;
//! these are explicit ELF text padding, not executable instructions.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P,
                      ScalarPacket as H};

pub const FILE_OFFSET:usize=0x41a0;
pub const PHYSICAL_ADDRESS:u32=0x96c0_11a0;
pub const FUNCTION_BYTES:usize=64;
pub const PADDING_START:usize=0x4194;
pub const PADDING_BYTES:usize=FILE_OFFSET-PADDING_START;

fn push(out:&mut Vec<u8>,op:H,marker:P)->Result<(),String>{
    out.extend_from_slice(&marker.apply(op.encode()?).to_le_bytes());Ok(())
}

/// Copy doublewords using a hardware loop, taking the fast return if either
/// the source address or global block count is zero. Branch conditions and
/// packet parse markers are explicit; no original opcodes are embedded.
pub fn build_copy()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    for (dst,offset,last) in [(3,-0x1f39980,false),(0,-0x1f398cc,true)] {
        for word in cdsp_boot::encode_extended_word_load(r(dst)?,r(25)?,offset)? {
            out.extend_from_slice(&(if last {word} else {P::ContinuationA.apply(word)}).to_le_bytes());
        }
    }
    push(&mut out,H::SkipIfSourceZeroNotTaken{src:r(3)?,displacement_bytes:44},P::Final)?;
    for word in cdsp_boot::encode_extended_add(r(1)?,r(25)?,-0x3f98cda0,false) {
        out.extend_from_slice(&word.to_le_bytes());
    }
    push(&mut out,H::PredicateP0GreaterThan32{src:r(3)?},P::ContinuationA)?;
    push(&mut out,H::JumpIfNotNewP0NotTaken{displacement_bytes:40},P::Final)?;
    push(&mut out,H::SkipIfSourceZeroNotTaken{src:r(1)?,displacement_bytes:24},P::Final)?;
    push(&mut out,H::LogicalShiftRight{dst:r(2)?,src:r(3)?,amount:3},P::Final)?;
    push(&mut out,H::HardwareLoop{loop_index:0,counter:r(2)?,displacement_bytes:4},P::Final)?;
    push(&mut out,H::LoadDoublePostIncrement{low:r(4)?,base:r(1)?,increment:8},P::Final)?;
    push(&mut out,H::Nop,P::LoopEnd)?;
    push(&mut out,H::MemDoubleStorePostIncrement{low:r(4)?,base:r(0)?,increment:8},P::Final)?;
    push(&mut out,H::JumpRegister{source:r(31)?},P::Final)?;
    if out.len()!=FUNCTION_BYTES {return Err("Hexagon DMA copy length !=64".into());}
    Ok(out)
}

/// Join the adjacent executable fragments without pretending ELF alignment
/// padding is executable: the 12 inter-function zero bytes are deliberately
/// represented as padding and counted separately from source-rebuilt code.
pub fn build_cluster_with_dma_copy()->Result<Vec<u8>,String>{
    let mut out=super::cdsp_cache_controller::build_joined_complete_cache_controller()?;
    let prior_end=super::cdsp_tlb::COMPLETE_CLUSTER_OFFSET+out.len();
    if prior_end!=PADDING_START{return Err("CDSP controller/copy padding boundary differs".into());}
    out.extend([0u8;PADDING_BYTES]);
    out.extend(build_copy()?);
    if out.len()!=1184 {return Err("CDSP controller/copy span wrong size".into());}
    Ok(out)
}

pub fn verify_copy(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+FUNCTION_BYTES)!=Some(build_copy()?.as_slice()){
        return Err("CDSP DMA copy does not match reference".into());
    }
    Ok(())
}

pub fn verify_cluster_with_dma_copy(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(PADDING_START..FILE_OFFSET)!=Some(&[0u8;PADDING_BYTES]) {
        return Err("CDSP expected alignment padding not zero".into());
    }
    let cluster=build_cluster_with_dma_copy()?;
    let start=super::cdsp_tlb::COMPLETE_CLUSTER_OFFSET;
    if reference.get(start..start+cluster.len())!=Some(cluster.as_slice()){
        return Err("CDSP supervisor/TLB/cache/DMA cluster mismatch".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test] fn doubleword_loop_is_encoded_and_padding_separate(){
        assert_eq!(FUNCTION_BYTES,64);
        assert_eq!(PADDING_BYTES,12);
        let bytes=build_copy().unwrap();assert_eq!(bytes.len(),FUNCTION_BYTES);
        assert_eq!(build_cluster_with_dma_copy().unwrap().len(),1184);
        let words:Vec<u32>=bytes.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();
        assert_eq!(words[4],0x1003_c016);
        assert_eq!(words[8],0x5c20_c814);
        assert_eq!(words[13]&0xc000,0x8000);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
}
