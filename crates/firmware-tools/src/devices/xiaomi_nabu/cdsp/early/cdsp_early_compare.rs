// SPDX-License-Identifier: GPL-3.0-only
//! Validate CDSP physical memory against an expected buffer in a bounded
//! retry loop. Only exact Hexagon V66 typed instructions are counted as code.
//! Duplex packet layouts and .new predicate branches retain Hexagon parse
//! semantics. LLVM6 independently corroborates ordinary/duplex forms;
//! the privileged MEMW_PHYS instruction is Rizin V66 decoder-confirmed.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_physcopy;

pub const FILE_OFFSET:usize=0x4364;
pub const FUNCTION_LEN:usize=0x90;
pub const REGION_END:usize=FILE_OFFSET+FUNCTION_LEN;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1364;

fn write(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}

#[derive(Clone,Copy)]
enum HexagonCompare {
    SignedGreaterAndBranchNotTaken{left:Gpr,right:Gpr,delta:u16},
    EqualAndJumpNotTaken{left:Gpr,right:Gpr,delta:u16},
    GreaterSmallAndJumpNotTaken{src:Gpr,immediate:u8,delta:u16},
    RightShiftVariable{dst:Gpr,src:Gpr,amount_reg:Gpr},
    PhysicalWordRead{dst:Gpr,address:Gpr,selector:Gpr},
    LoadWord{dst:Gpr,address:Gpr},
    StoreWordPost{src:Gpr,address:Gpr,increment:u8},
}
fn half(delta:u16)->Result<u32,String>{
    if delta==0||delta%2!=0||delta>0x1fe{return Err("branch displacement must be positive, even and <=0x1fe".into());}
    Ok(u32::from(delta/2))
}
impl HexagonCompare {
    fn encode(self)->Result<u32,String>{
        use HexagonCompare::*;
        match self {
            SignedGreaterAndBranchNotTaken{left,right,delta}=>Ok(0x14c0_c000|u32::from(left.number())<<16|u32::from(right.number())<<8|half(delta)?),
            EqualAndJumpNotTaken{left,right,delta}=>Ok(0x1400_c000|u32::from(left.number())<<16|u32::from(right.number())<<8|half(delta)?),
            GreaterSmallAndJumpNotTaken{src,immediate,delta}=>{
                if immediate>31{return Err("small Hexagon predicate operand >31".into());}
                Ok(0x1080_c000|u32::from(src.number())<<16|u32::from(immediate)<<8|half(delta)?)
            }
            RightShiftVariable{dst,src,amount_reg}=>Ok(0xc640_c040|u32::from(src.number())<<16|u32::from(amount_reg.number())<<8|u32::from(dst.number())),
            PhysicalWordRead{dst,address,selector}=>Ok(0x9200_e000|u32::from(address.number())<<16|u32::from(selector.number())<<8|u32::from(dst.number())),
            LoadWord{dst,address}=>Ok(0x9180_c000|u32::from(address.number())<<16|u32::from(dst.number())),
            StoreWordPost{src,address,increment}=>{
                if increment==0||increment>28||increment%4!=0{return Err("word postincrement must be aligned and 4..28".into());}
                Ok(0xab80_c000|u32::from(address.number())<<16|u32::from(src.number())<<8|u32::from(increment)*2)
            }
        }
    }
}

/// Hexagon duplex pair: two compressed register-immediate assignments.
/// LLVM6 independently confirms distinct constant/register operands.
fn assign_two_small(first:Gpr,first_value:u8,second:Gpr,second_value:u8)->Result<u32,String>{
    if first.number()>7||second.number()>7||first_value>15||second_value>15 {
        return Err("duplex immediate pair requires low registers and 4-bit values".into());
    }
    Ok((0x2800|u32::from(first_value)<<4|u32::from(first.number()))<<16 |
        (0x2800|u32::from(second_value)<<4|u32::from(second.number())))
}
fn add_two_small(first:Gpr,first_value:u8,second:Gpr,second_value:u8)->Result<u32,String>{
    if first.number()>7||second.number()>7||first_value>15||second_value>15 {
        return Err("duplex add pair requires low registers and 4-bit values".into());
    }
    Ok((0x2000|u32::from(first_value)<<4|u32::from(first.number()))<<16 |
       (0x2000|u32::from(second_value)<<4|u32::from(second.number())))
}

/// LLP-supported two-word immext+duplex compound op that assigns an extended
/// register and adds an independent register value within the same packet.
/// Keep only the combination independently validated in the LLVM6 probes.
fn encode_extended_assign_and_add(dst:Gpr,constant:i32,add_dst:Gpr,add_src:Gpr)->Result<[u32;2],String>{
    if dst.number()!=6||add_dst.number()!=3||add_src.number()!=5||constant!=-0x1fe4720 {
        return Err("unsupported coupled V66 immext+duplex operand tuple".into());
    }
    let val=constant as u32;
    let extender=((val>>20)<<16)|((val>>6)&0x3fff)|0x4000;
    // Upper duplex slot stores the extended assignment's remaining low bits,
    // lower slot adds R5 to R3. Values were confirmed in LLVM6.
    let upper=0x0a00|((val&0x3f)<<8)|u32::from(dst.number());
    let lower=0x3800|u32::from(add_src.number())<<4|u32::from(add_dst.number());
    Ok([extender,(upper<<16)|lower])
}

/// Verify mapped 32-bit physical memory in a bounded hardware loop.
/// The controller populates an error ring when actual and expected words
/// differ, aborting once the mismatch count exceeds the retry threshold.
pub fn build_verify_physical()->Result<Vec<u8>,String>{
    use HexagonCompare::*;
    let r=|n|Gpr::new(n);
    let mut output=Vec::with_capacity(FUNCTION_LEN);
    let assign=cdsp_boot::encode_extended_assignment(r(1)?,0,false);
    write(&mut output,assign[0],P::ContinuationA);
    write(&mut output,assign[1],P::ContinuationA);
    let frame=cdsp_boot::encode_extended_assignment(r(4)?,-0x3fe3ffa8,true);
    write(&mut output,frame[0],P::ContinuationA);
    write(&mut output,frame[1],P::Final);
    let limit=cdsp_boot::encode_extended_assignment(r(0)?,0x8000,false);
    write(&mut output,limit[0],P::ContinuationA);
    write(&mut output,limit[1],P::ContinuationA);
    write(&mut output,H::SubtractRegisters{dst:r(6)?,left:r(4)?,right:r(1)?}.encode()?,P::Final);
    write(&mut output,SignedGreaterAndBranchNotTaken{left:r(0)?,right:r(6)?,delta:112}.encode()?,P::ContinuationA);
    let zero=cdsp_boot::encode_extended_assignment(r(0)?,0,true);
    write(&mut output,zero[0],P::ContinuationA);
    write(&mut output,zero[1],P::Final);
    write(&mut output,H::LogicalShiftRight{dst:r(2)?,src:r(6)?,amount:2}.encode()?,P::ContinuationA);
    write(&mut output,H::SkipIfSourceZeroNotTaken{src:r(0)?,displacement_bytes:100}.encode()?,P::Final);
    let source_gp=cdsp_boot::encode_extended_gp_word_load(r(8)?,0xfe0c_66a0);
    write(&mut output,source_gp[0],P::ContinuationA);
    write(&mut output,source_gp[1],P::ContinuationA);
    let end_gp=cdsp_boot::encode_extended_gp_word_load(r(9)?,0xfe0c_669c);
    write(&mut output,end_gp[0],P::ContinuationA);
    write(&mut output,end_gp[1],P::Final);
    write(&mut output,H::SubtractRegisters{dst:r(5)?,left:r(1)?,right:r(0)?}.encode()?,P::ContinuationA);
    write(&mut output,H::SubtractRegisters{dst:r(9)?,left:r(9)?,right:r(8)?}.encode()?,P::ContinuationA);
    let zero3=cdsp_boot::encode_extended_assignment(r(3)?,0,true);
    write(&mut output,zero3[0],P::ContinuationA);
    write(&mut output,zero3[1],P::Final);
    write(&mut output,H::AddRegisters{dst:r(3)?,left:r(3)?,right:r(9)?}.encode()?,P::Final);
    let compound=encode_extended_assign_and_add(r(6)?,-0x1fe4720,r(3)?,r(5)?)?;
    write(&mut output,compound[0],P::ContinuationA);
    // Duplex encodings have their own parse fields; do not overwrite them.
    output.extend_from_slice(&compound[1].to_le_bytes());
    write(&mut output,H::HardwareLoop{loop_index:0,counter:r(2)?,displacement_bytes:8}.encode()?,P::ContinuationA);
    output.extend_from_slice(&assign_two_small(r(2)?,11,r(7)?,0)?.to_le_bytes());
    write(&mut output,RightShiftVariable{dst:r(4)?,src:r(3)?,amount_reg:r(2)?}.encode()?,P::ContinuationA);
    write(&mut output,LoadWord{dst:r(0)?,address:r(1)?}.encode()?,P::Final);
    write(&mut output,PhysicalWordRead{dst:r(5)?,address:r(3)?,selector:r(4)?}.encode()?,P::Final);
    write(&mut output,EqualAndJumpNotTaken{left:r(5)?,right:r(0)?,delta:20}.encode()?,P::Final);
    write(&mut output,GreaterSmallAndJumpNotTaken{src:r(7)?,immediate:3,delta:24}.encode()?,P::Final);
    write(&mut output,H::AddSmallImmediate{dst:r(7)?,src:r(7)?,value:1}.encode()?,P::ContinuationA);
    write(&mut output,StoreWordPost{src:r(3)?,address:r(6)?,increment:4}.encode()?,P::Final);
    write(&mut output,StoreWordPost{src:r(5)?,address:r(6)?,increment:4}.encode()?,P::Final);
    write(&mut output,H::Nop.encode()?,P::LoopEnd);
    output.extend_from_slice(&add_two_small(r(1)?,4,r(3)?,4)?.to_le_bytes());
    write(&mut output,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if output.len()!=FUNCTION_LEN{return Err("CDSP compare function length mismatch".into());}
    Ok(output)
}

pub fn build_joined()->Result<Vec<u8>,String>{
    let mut result=cdsp_early_physcopy::build_joined()?;
    if cdsp_early_physcopy::REGION_END!=FILE_OFFSET{return Err("CDSP physical-copy compare function offset gap".into());}
    result.extend(build_verify_physical()?);
    if result.len()!=REGION_END-super::cdsp_early_tlb::FILE_OFFSET{return Err("CDSP compare joined length mismatch".into());}
    Ok(result)
}

pub fn verify_joined(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let output=build_joined()?;
    let offset=super::cdsp_early_tlb::FILE_OFFSET;
    let reference_span=reference.get(offset..REGION_END).ok_or("CDSP original truncated before compare end")?;
    if let Some((i,(x,y)))=output.iter().zip(reference_span).enumerate().find(|(_, (x,y))|x!=y){
        return Err(format!("CDSP physical-memory compare differs at file {:#x}: generated {x:02x}, expected {y:02x}",offset+i));
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn complete_physical_compare_length_and_no_gap(){
        assert_eq!(build_verify_physical().unwrap().len(),FUNCTION_LEN);
        assert_eq!(build_joined().unwrap().len(),0x214);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn duplex_immediate_fields_and_bounds(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(assign_two_small(r(2),11,r(7),0).unwrap(),0x28b2_2807);
        assert_eq!(add_two_small(r(1),4,r(3),4).unwrap(),0x2041_2043);
        assert!(assign_two_small(r(2),16,r(7),0).is_err());
        assert!(add_two_small(r(9),1,r(3),1).is_err());
        assert_eq!(encode_extended_assign_and_add(r(6),-0x1fe4720,r(3),r(5)).unwrap(),[0x0fe0_46e3,0x2a06_3853]);
        assert!(encode_extended_assign_and_add(r(6),0x123,r(3),r(5)).is_err());
    }
}
