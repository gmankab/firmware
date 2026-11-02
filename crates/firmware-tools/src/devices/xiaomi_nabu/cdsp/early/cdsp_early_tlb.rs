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
pub const SWEEP_LEN:usize=0x3c;
pub const TAIL_LEN:usize=0x44;
pub const REGION_END: usize = FILE_OFFSET + PREFIX_LEN + SWEEP_LEN + TAIL_LEN;


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


/// Hexagon V66 TLB read/write operations and individual scalar DSP ops used
/// by this invalidation routine. Sources/condition codes are constrained to
/// the variants independently tested by LLVM6 (ordinary scalar cases) or
/// cross-decoded by Rizin (privileged TLB cases).
#[derive(Clone,Copy,Debug)]
enum TlbScalar {
    ReadEntry {low:Gpr,index:Gpr},
    Lock,
    WriteEntry {low:Gpr,index:Gpr},
    Unlock,
    CountTrailingZeros {dst:Gpr,src:Gpr},
    ClearBitByRegister {dst:Gpr,src:Gpr,bit:Gpr},
    ArithmeticShiftRight {dst:Gpr,src:Gpr,amount:u8},
    AndRegisters {dst:Gpr,left:Gpr,right:Gpr},
    EqualAndJumpNotTaken {left:Gpr,right:Gpr,bytes:u16},
    EqualP1AndJumpNotTaken {left:Gpr,right:Gpr,bytes:u16},
    GreaterUnsignedP0AndJumpTaken {left:Gpr,right:Gpr,bytes:u16},
    GreaterUnsignedP1AndJumpTaken {left:Gpr,right:Gpr,bytes:u16},
}
impl TlbScalar {
    fn encode(self)->Result<u32,String>{
        use TlbScalar::*;
        match self {
            ReadEntry{low,index} => {
                if low.number()%2!=0 {return Err("TLBR requires even register-pair low half".into());}
                Ok(0x6c40_c000 | u32::from(index.number())<<16 |u32::from(low.number()))
            }
            Lock=>Ok(0x6c20_c020),
            Unlock=>Ok(0x6c20_c040),
            WriteEntry{low,index}=>{
                if low.number()%2!=0 {return Err("TLBW requires even register-pair low half".into());}
                Ok(0x6c00_c000 |u32::from(low.number())<<16|u32::from(index.number())<<8)
            }
            CountTrailingZeros{dst,src}=>Ok(0x8c40_c080 |u32::from(src.number())<<16|u32::from(dst.number())),
            ClearBitByRegister{dst,src,bit}=>Ok(0xc680_c040 |u32::from(src.number())<<16|u32::from(bit.number())<<8|u32::from(dst.number())),
            ArithmeticShiftRight{dst,src,amount}=>{
                if amount>=32 {return Err("ASR shift out of range".into());}
                Ok(0x8c00_c000 |u32::from(src.number())<<16|u32::from(amount)<<8|u32::from(dst.number()))
            }
            AndRegisters{dst,left,right}=>Ok(0xf100_c000 |u32::from(left.number())<<16|u32::from(right.number())<<8|u32::from(dst.number())),
            EqualAndJumpNotTaken{left,right,bytes}=>Ok(0x1400_c000|u32::from(left.number())<<16|u32::from(right.number())<<8|checked_delta(bytes)?),
            EqualP1AndJumpNotTaken{left,right,bytes}=>Ok(0x1400_d000|u32::from(left.number())<<16|u32::from(right.number())<<8|checked_delta(bytes)?),
            GreaterUnsignedP0AndJumpTaken{left,right,bytes}=>Ok(0x1500_e000|u32::from(left.number())<<16|u32::from(right.number())<<8|checked_delta(bytes)?),
            GreaterUnsignedP1AndJumpTaken{left,right,bytes}=>Ok(0x1500_f000|u32::from(left.number())<<16|u32::from(right.number())<<8|checked_delta(bytes)?),
        }
    }
}
fn checked_delta(bytes:u16)->Result<u32,String>{
    if bytes==0||bytes>0x1fe||bytes%2!=0{return Err("relative branch must be positive, even, and <=0x1fe".into());}
    Ok(u32::from(bytes/2))
}

/// The 15 packet words of the loop body, including the privileged TLB
/// sequence. A single loop-end parse marker belongs on R3 increment.
pub fn build_first_sweep()->Result<Vec<u8>,String>{
    use TlbScalar::*;
    let r=|n|Gpr::new(n);
    let mut code=Vec::with_capacity(SWEEP_LEN);
    let put=|code:&mut Vec<u8>,word:u32,p:P| code.extend_from_slice(&p.apply(word).to_le_bytes());
    // TLB register pair R5:4 is read at physical controller index R3.
    put(&mut code,ReadEntry{low:r(4)?,index:r(3)?}.encode()?,P::Final);
    put(&mut code,H::SkipIfSourceZeroNotTaken{src:r(5)?,displacement_bytes:48}.encode()?,P::Final);
    for op in [
        CountTrailingZeros{dst:r(6)?,src:r(4)?},
        ClearBitByRegister{dst:r(4)?,src:r(4)?,bit:r(6)?},
        ArithmeticShiftRight{dst:r(4)?,src:r(4)?,amount:1},
        AndRegisters{dst:r(4)?,left:r(4)?,right:r(10)?},
        EqualAndJumpNotTaken{left:r(4)?,right:r(0)?,bytes:12},
        GreaterUnsignedP0AndJumpTaken{left:r(4)?,right:r(2)?,bytes:24},
        GreaterUnsignedP1AndJumpTaken{left:r(0)?,right:r(4)?,bytes:20},
    ] {put(&mut code,op.encode()?,P::Final);}
    put(&mut code,H::CombineZeroPair{low:r(6)?}.encode()?,P::Final);
    for op in [Lock,WriteEntry{low:r(6)?,index:r(3)?},Unlock] {put(&mut code,op.encode()?,P::Final);}
    put(&mut code,H::AddSmallImmediate{dst:r(3)?,src:r(3)?,value:1}.encode()?,P::LoopEnd);
    put(&mut code,H::Nop.encode()?,P::Final);
    assert_eq!(code.len(),SWEEP_LEN);
    Ok(code)
}

/// Signed backward branch with a new-value P0 predicate. The branch is
/// packet-relative, and its initial word must share a packet with an earlier
/// P0 producer. Width and direction are checked before encoding the split
/// signed halfword displacement confirmed with LLVM6 samples.
fn if_not_new_p0_jump_backward(packet_delta:i16)->Result<u32,String>{
    if packet_delta >= 0 || packet_delta < -0x200 || packet_delta%2!=0{
        return Err("Hexagon backward P0.new branch requires negative even packet displacement <=0x200".into());
    }
    let half=(i32::from(packet_delta)/2) as u16;
    Ok(0x5c00_e800 | (u32::from(half & 0xff00) << 8) | u32::from(half & 0xff))
}

/// Exact closure of the first CDSP TLB flush function. No trailing ELF
/// alignment or unknown instruction bytes are added.
pub fn build_tlb_tail()->Result<Vec<u8>,String>{
    use TlbScalar::*;
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(TAIL_LEN);
    let mut push=|word:u32,marker:P|out.extend_from_slice(&marker.apply(word).to_le_bytes());
    let relocated=cdsp_boot::encode_extended_assignment(r(4)?,-0x1f39958,false);
    push(relocated[0],P::ContinuationA);
    push(relocated[1],P::ContinuationA);
    let gp_limit=cdsp_boot::encode_extended_gp_word_load(r(5)?,0xfe02_6fd0);
    push(gp_limit[0],P::ContinuationA);
    push(gp_limit[1],P::Final);
    push(H::AssignSmallImmediate{dst:r(10)?,value:5}.encode()?,P::ContinuationA);
    push(H::AssignSmallImmediate{dst:r(8)?,value:0}.encode()?,P::Final);
    push(H::LoadDoublePostIncrement{low:r(0)?,base:r(4)?,increment:8}.encode()?,P::Final);
    push(H::SkipIfSourceZero{src:r(0)?,displacement_bytes:36}.encode()?,P::Final);
    for instruction in [Lock,WriteEntry{low:r(0)?,index:r(3)?},Unlock]{push(instruction.encode()?,P::Final);}
    push(H::AddSignedSmallImmediate{dst:r(3)?,src:r(3)?,value:-1}.encode()?,P::ContinuationA);
    push(H::AddSmallImmediate{dst:r(8)?,src:r(8)?,value:1}.encode()?,P::Final);
    push(EqualP1AndJumpNotTaken{left:r(3)?,right:r(5)?,bytes:12}.encode()?,P::Final);
    push(H::PredicateP0EqualsRegisters{left:r(8)?,right:r(10)?}.encode()?,P::ContinuationA);
    push(if_not_new_p0_jump_backward(-32)?,P::Final);
    push(H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=TAIL_LEN{return Err("TLB flush tail wrong length".into());}
    Ok(out)
}

pub fn build_span()->Result<Vec<u8>,String>{
    let mut data=build_prefix()?;
    data.extend(build_first_sweep()?);
    data.extend(build_tlb_tail()?);
    if data.len()!=REGION_END-FILE_OFFSET {return Err("CDSP TLB body length wrong".into());}
    Ok(data)
}

pub fn verify_span(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let got=build_span()?;
    let expected=reference.get(FILE_OFFSET..REGION_END).ok_or("CDSP reference truncated at TLB sweep")?;
    if let Some((i,(x,y)))=got.iter().zip(expected).enumerate().find(|(_, (x,y))|x!=y){
        return Err(format!("CDSP TLB sweep differs at {:#x}: generated {x:02x}, reference {y:02x}",FILE_OFFSET+i));
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
    #[test]fn tlb_sweep_encodes_all_fifteen_words(){
        assert_eq!(build_first_sweep().unwrap().len(),0x3c);
        assert_eq!(build_tlb_tail().unwrap().len(),0x44);
        assert_eq!(build_span().unwrap().len(),REGION_END-FILE_OFFSET);
    }
    #[test]fn signed_predicate_branch_rejects_unsupported_displacements(){
        assert!(if_not_new_p0_jump_backward(0).is_err());
        assert!(if_not_new_p0_jump_backward(2).is_err());
        assert!(if_not_new_p0_jump_backward(-33).is_err());
        assert!(if_not_new_p0_jump_backward(-0x202).is_err());
        assert_eq!(if_not_new_p0_jump_backward(-32).unwrap(),0x5cff_e8f0);
    }
    #[test] fn branches_and_packet_boundaries_are_checked() {
        let r1=Gpr::new(1).unwrap();
        assert!(H::SkipIfSourceZeroNotTaken{src:r1,displacement_bytes:0x9d}.encode().is_err());
        assert!(H::HardwareLoopImmediate{loop_index:0,iterations:0,displacement_bytes:12}.encode().is_err());
        assert!(H::LogicalShiftRight{dst:r1,src:r1,amount:32}.encode().is_err());
    }
}
