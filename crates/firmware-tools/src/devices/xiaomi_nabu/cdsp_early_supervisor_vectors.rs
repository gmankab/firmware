// SPDX-License-Identifier: GPL-3.0-only
//! CDSP supervisor context restoration and exception-vector trampoline.
//! 0x45c0..0x471c contains 20 setup instructions, 60 architectural NOP
//! instructions, and seven jumps; none of the
//! NOPs are linker fill. Packet boundaries intentionally match OEM layout.
//! This is a checked Hexagon V66 re-encoding, not proof of device execution.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_interrupt_debug;

pub const FILE_OFFSET:usize=0x45c0;
pub const REGION_END:usize=0x471c;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_15c0;
pub const NOP_OFFSET:usize=0x4610;
pub const NOP_COUNT:usize=60;
pub const VECTOR_OFFSET:usize=0x4700;
pub const VECTOR_COUNT:usize=7;

#[derive(Debug,Clone,Copy)]
enum ExceptionRegister { Evb, PmuEvtCfg, PredicateGroup, Pair48, Pair44, Stack47, Pair50, Start }
impl ExceptionRegister {
    fn emit(self,register:Gpr)->u32 {
        let n=u32::from(register.number());
        match self {
            Self::Evb=>0x6700_c010 | (n<<16),
            Self::PmuEvtCfg=>0x6e80_c000 | (52<<16)|n,
            Self::PredicateGroup=>0x6220_c004 | (n<<16),
            Self::Pair48=>0x6f30_c000 | n,
            Self::Pair44=>0x6f2c_c000 | n,
            Self::Stack47=>0x6eaf_c000 | n,
            Self::Pair50=>0x6f32_c000 | n,
            Self::Start=>0x6460_c020 | (n<<16),
        }
    }
}

fn compare_predicate_eq_imm(predicate:u8,register:Gpr,immediate:u8)->Result<u32,String>{
    if predicate>3 || immediate>127 {
        return Err("special Hexagon predicate compare bounds exceeded".into());
    }
    Ok(0x7500_c000|(u32::from(register.number())<<16)
       |(u32::from(immediate)<<5)|u32::from(predicate))
}
/// A short ordinary `call <target>` whose packet-relative signed field is
/// stored in halfword units. Extended calls have a separate typed encoder.
fn short_call(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0 || target%4!=0 {return Err("Hexagon CALL addresses must be 4-byte aligned".into());}
    let diff=i64::from(target)-i64::from(packet_start);
    if !(-0x100_000..=0xff_ffc).contains(&diff) || diff%2!=0 {
        return Err("short call signed displacement outside confirmed encoding".into());
    }
    Ok(0x5a00_c000|(((diff/2) as i32 as u32)&0x01ff_ffff))
}
/// Conditional not-taken branch with a single predicate bit and split
/// high/low displacement. The packet start determines the branch target.
fn predicate_jump_nt(packet_start:u32,target:u32,predicate:u8,negate:bool)->Result<u32,String>{
    if packet_start%4!=0 || target%4!=0 || target<=packet_start || predicate>1 {
        return Err("exception predicate jump requires aligned short forward branch".into());
    }
    let delta=target-packet_start;
    if delta>0x7fe {return Err("exception branch exceeds decoded short format".into());}
    let half=delta/2;
    Ok(0x5c00_c000|((negate as u32)<<21)|((half>>9)<<16)
       |(u32::from(predicate)<<8)|(half&0x1ff))
}
fn put(out:&mut Vec<u8>,word:u32,p:P) {
    out.extend_from_slice(&p.apply(word).to_le_bytes());
}

pub fn build_supervisor_vectors()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    let base=cdsp_boot::encode_extended_add(r(1)?,r(23)?,-0x1ffe900,true);
    put(&mut out,base[0],P::ContinuationA);
    put(&mut out,base[1],P::Final);
    put(&mut out,ExceptionRegister::Evb.emit(r(1)?),P::Final);
    // The packetized short CALL target is the earlier controller routine.
    put(&mut out,short_call(0x96c0_15cc,0x96c0_0d30)?,P::Final);
    let context=cdsp_boot::encode_extended_word_load(r(25)?,r(23)?,-0x1fe92bc)?;
    put(&mut out,context[0],P::ContinuationA);
    put(&mut out,context[1],P::Final);
    put(&mut out,compare_predicate_eq_imm(0,r(25)?,2)?,P::ContinuationA);
    put(&mut out,compare_predicate_eq_imm(1,r(25)?,0)?,P::Final);
    put(&mut out,predicate_jump_nt(0x96c0_15e0,0x96c0_1ab0,0,false)?,P::Final);
    put(&mut out,predicate_jump_nt(0x96c0_15e4,0x96c0_1ac0,1,true)?,P::Final);
    put(&mut out,ExceptionRegister::PmuEvtCfg.emit(r(24)?),P::Final);
    put(&mut out,ExceptionRegister::PredicateGroup.emit(r(24)?),P::Final);
    put(&mut out,ExceptionRegister::Pair48.emit(r(0)?),P::Final);
    put(&mut out,ExceptionRegister::Pair44.emit(r(24)?),P::Final);
    put(&mut out,ExceptionRegister::Stack47.emit(r(29)?),P::Final);
    put(&mut out,ExceptionRegister::Pair50.emit(r(30)?),P::Final);
    let start=cdsp_boot::encode_extended_assignment(r(23)?,-1,true);
    put(&mut out,start[0],P::ContinuationA);
    put(&mut out,start[1],P::Final);
    put(&mut out,ExceptionRegister::Start.emit(r(23)?),P::Final);
    // This JMP shares a packet with its following two NOP words, which is
    // why its parse field is ContinuationA rather than standalone Final.
    put(&mut out,H::JumpRelative{displacement_bytes:0x110}.encode()?,P::ContinuationA);
    assert_eq!(FILE_OFFSET+out.len(),NOP_OFFSET);
    for index in 0..NOP_COUNT {
        let marker=if index<2 {P::ContinuationA}
            else if index==2 || index==3 {P::Final}
            else if (index-4)%4==3 {P::Final}
            else {P::ContinuationA};
        put(&mut out,H::Nop.encode()?,marker);
    }
    assert_eq!(FILE_OFFSET+out.len(),VECTOR_OFFSET);
    // First entry skips the six self-branching trap vectors. No instruction
    // bytes from the reference are embedded here; jump distances are typed.
    for index in 0..VECTOR_COUNT {
        let distance=if index==0 {0x1c} else {0};
        put(&mut out,H::JumpRelative{displacement_bytes:distance}.encode()?,P::Final);
    }
    if FILE_OFFSET+out.len()!=REGION_END {return Err(format!("CDSP exception vector emitted {} instead of {} bytes",out.len(),FUNCTION_BYTES));}
    Ok(out)
}

pub fn build_supervisor_merged()->Result<Vec<u8>,String>{
    let mut out=super::cdsp_early_system_start::build_system_start()?;
    if super::cdsp_early_system_start::REGION_END!=cdsp_early_interrupt_debug::FILE_OFFSET || cdsp_early_interrupt_debug::REGION_END!=FILE_OFFSET {
        return Err("CDSP supervisor byte ranges do not abut exactly".into());
    }
    out.extend(cdsp_early_interrupt_debug::build_interrupt_debug()?);
    out.extend(build_supervisor_vectors()?);
    if out.len()!=REGION_END-super::cdsp_early_system_start::FILE_OFFSET {return Err("CDSP supervisor cumulative size mismatch".into());}
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_interrupt_debug::verify_reference(reference)?;
    let expected=reference.get(FILE_OFFSET..REGION_END).ok_or("pinned CDSP image too small")?;
    let generated=build_supervisor_vectors()?;
    if let Some((i,(a,b)))=generated.iter().zip(expected).enumerate().find(|(_, (a,b))|a!=b) {
        return Err(format!("CDSP vectors differ at file {:#x}: typed {:02x}, original {:02x}",FILE_OFFSET+i,a,b));
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn hexagon_supervisor_has_real_nops_and_vectors(){
        let generated=build_supervisor_vectors().unwrap();
        assert_eq!(FUNCTION_BYTES,348);
        assert_eq!(generated.len(),348);
        assert_eq!(NOP_OFFSET,0x4610);
        assert_eq!(NOP_COUNT,60);
        assert_eq!(VECTOR_COUNT,7);
        let nop=&generated[NOP_OFFSET-FILE_OFFSET..VECTOR_OFFSET-FILE_OFFSET];
        assert_eq!(nop.len(),240);
        assert!(nop.chunks_exact(4).all(|x|{
            let word=u32::from_le_bytes(x.try_into().unwrap());
            (word & !0xc000)==(H::Nop.encode().unwrap() & !0xc000)
        }));
        assert_eq!(build_supervisor_merged().unwrap().len(),REGION_END-0x4500);
    }
    #[test] fn predicate_branches_validate_displacements_and_operands(){
        assert_eq!(short_call(0x96c015cc,0x96c00d30).unwrap(),0x5bff_fbb2);
        assert_eq!(predicate_jump_nt(0x96c015e0,0x96c01ab0,0,false).unwrap(),0x5c01_c068);
        assert_eq!(predicate_jump_nt(0x96c015e4,0x96c01ac0,1,true).unwrap(),0x5c21_c16e);
        assert!(predicate_jump_nt(3,16,0,false).is_err());
        assert!(predicate_jump_nt(0x96c0_15e0,0x96c0_1ee0,0,false).is_err());
        assert!(compare_predicate_eq_imm(4,Gpr::new(25).unwrap(),1).is_err());
    }
}
