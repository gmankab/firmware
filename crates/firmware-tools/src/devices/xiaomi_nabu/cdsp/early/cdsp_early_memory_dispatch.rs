// SPDX-License-Identifier: GPL-3.0-only
//! Variable-width memory copy entrypoint. A four-entry pointer table selects
//! byte, halfword, word or doubleword copying; the P0/P1-predicated stores
//! below are expressed with checked operand and increment widths.
//! Includes the cache-line-zeroing continuation through 0x4bb8. The eight
//! zero bytes thereafter are verified as linker fill, not executable source.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_runtime_maintenance;

pub const FILE_OFFSET:usize=0x4b1c;
pub const PREFIX_END:usize=0x4b70;
pub const REGION_END:usize=0x4bb8;
pub const ALIGNMENT_END:usize=0x4bc0;
pub const ALIGNMENT_BYTES:usize=ALIGNMENT_END-REGION_END;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1b1c;
pub const CODE_BYTES:usize=REGION_END-FILE_OFFSET;
fn put(out:&mut Vec<u8>,word:u32,p:P){out.extend_from_slice(&p.apply(word).to_le_bytes());}

#[derive(Debug,Clone,Copy)] enum Width { Byte,Half,Word,Double }
impl Width {
    fn kind(self)->u32{match self{Self::Byte=>0,Self::Half=>1,Self::Word=>2,Self::Double=>3}}
    fn bytes(self)->u8{match self{Self::Byte=>1,Self::Half=>2,Self::Word=>4,Self::Double=>8}}
}

fn check_copy_operands(width:Width,src:Gpr,offset:u8)->Result<(),String>{
    if matches!(width,Width::Double) && src.number()%2!=0 {return Err("CDSP doubleword copy must use even register pair".into());}
    if offset%width.bytes()!=0||offset>16{return Err("CDSP predicated copy unaligned or out-of-range offset".into());}
    Ok(())
}
fn p0_post_store(width:Width,src:Gpr,base:Gpr,bytes:u8)->Result<u32,String>{
    check_copy_operands(width,src,bytes)?;
    if bytes==0||bytes/width.bytes()>2 {return Err("CDSP P0 store postincrement supports one or two elements".into());}
    Ok(0xab00_6000|(width.kind()<<22)|(u32::from(base.number())<<16)
       |(u32::from(src.number())<<8)|(u32::from(bytes/width.bytes())<<3))
}
fn predicated_store(width:Width,predicate:u8,src:Gpr,base:Gpr,offset:u8)->Result<u32,String>{
    if predicate>1 {return Err("CDSP memory dispatcher only supports P0/P1".into());}
    check_copy_operands(width,src,offset)?;
    Ok(0x4000_c000|(width.kind()<<22)|(u32::from(base.number())<<16)
       |(u32::from(src.number())<<8)|u32::from(predicate)|u32::from(offset))
}
fn index_table_word_load(dst:Gpr,index:Gpr,address:u32,shift:u8)->Result<[u32;2],String>{
    if address%4!=0||shift!=2{return Err("CDSP four-way jump table requires aligned address and index<<2".into());}
    let extender=((address>>20)<<16)|((address>>6)&0x3fff)|0x4000;
    let word=0x9d80_f000|(u32::from(index.number())<<16)
       |((address&0x3f)<<6)|u32::from(dst.number());
    Ok([extender,word])
}
/// Only this known Hexagon duplex architecture instruction family is modeled
/// (r0=-1;jumpr lr). Other values/pairs need a separate encoder.
fn return_unsupported_width(value:i16,destination:Gpr)->Result<u32,String>{
    if value!=-1||destination.number()!=31 {return Err("CDSP unsupported duplex return variant".into());}
    Ok(0x5a00_3fc0)
}
fn compare_width_and_skip(source:Gpr,max:u8,branch_bytes:u16)->Result<u32,String>{
    if max>7||branch_bytes==0||branch_bytes>0x1fe||branch_bytes%2!=0 {
        return Err("CDSP memory width selector not representable".into());
    }
    Ok(0x1100_c000 | (u32::from(source.number())<<16)
       | (u32::from(max)<<8) | u32::from(branch_bytes/2))
}

pub fn build_dispatch_prefix()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(PREFIX_END-FILE_OFFSET);
    put(&mut out,compare_width_and_skip(r(5)?,3,0x10)?,P::Final);
    let table=index_table_word_load(r(7)?,r(5)?,0xfe02_6dc8,2)?;
    put(&mut out,table[0],P::ContinuationA);
    put(&mut out,table[1],P::Final);
    put(&mut out,H::JumpRegister{source:r(7)?}.encode()?,P::Final);
    // Hexagon duplex has its own parse format; adding P::Final here would
    // corrupt an architecturally valid paired return/assignment instruction.
    out.extend_from_slice(&return_unsupported_width(-1,r(31)?)?.to_le_bytes());
    for (width,size,branch) in [(Width::Byte,1,0x40),(Width::Half,2,0x34),
                                (Width::Word,4,0x28)] {
        put(&mut out,H::JumpRelative{displacement_bytes:branch}.encode()?,P::ContinuationA);
        put(&mut out,p0_post_store(width,r(2)?,r(0)?,size)?,P::ContinuationA);
        put(&mut out,predicated_store(width,1,r(2)?,r(1)?,0)?,P::Final);
    }
    put(&mut out,H::JumpRelative{displacement_bytes:0x1c}.encode()?,P::ContinuationA);
    put(&mut out,p0_post_store(Width::Double,r(2)?,r(0)?,8)?,P::ContinuationA);
    put(&mut out,predicated_store(Width::Double,1,r(2)?,r(1)?,0)?,P::Final);
    put(&mut out,p0_post_store(Width::Double,r(2)?,r(0)?,16)?,P::ContinuationA);
    put(&mut out,predicated_store(Width::Double,0,r(2)?,r(0)?,8)?,P::Final);
    put(&mut out,predicated_store(Width::Double,1,r(2)?,r(1)?,0)?,P::ContinuationA);
    put(&mut out,predicated_store(Width::Double,1,r(2)?,r(1)?,8)?,P::Final);
    if out.len()!=PREFIX_END-FILE_OFFSET {return Err(format!("CDSP width dispatcher size {}, expected {}",out.len(),PREFIX_END-FILE_OFFSET));}
    Ok(out)
}

fn compare_greater_and_jump(left:Gpr,right:Gpr,packet_start:u32,target:u32)->Result<u32,String>{
    if target<=packet_start||packet_start%4!=0||target%4!=0||target-packet_start>0x1fe{
        return Err("CDSP copy loop conditional branch must be aligned, short, forward".into());
    }
    Ok(0x1540_0000|(u32::from(left.number())<<16)
        |(u32::from(right.number())<<8)|((target-packet_start)/2))
}
fn or_registers(dst:Gpr,left:Gpr,right:Gpr)->u32{
    0xf120_c000|(u32::from(left.number())<<16)|(u32::from(right.number())<<8)|u32::from(dst.number())
}
fn ct0(dst:Gpr,src:Gpr)->u32{
    0x8c40_c080|(u32::from(src.number())<<16)|u32::from(dst.number())
}
fn compare_new_gtu_and_skip(limit:u8,packet_start:u32,target:u32)->Result<u32,String>{
    if limit!=4||target<=packet_start||target%4!=0||packet_start%4!=0||target-packet_start>0x1fe{
        return Err("CDSP new-value GTU jump requires limit 4 and bounded forward target".into());
    }
    Ok(0x2546_c400 | ((target-packet_start)/2))
}
fn compare_eq_and_self_loop(left:Gpr,right:Gpr,packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start!=target||packet_start%4!=0 {
        return Err("CDSP copy cache-line spinloop requires a packet-aligned self target".into());
    }
    Ok(0x1440_2000|(u32::from(left.number())<<16)|(u32::from(right.number())<<8))
}
fn scaled_load_helper(dst:Gpr,index:Gpr,offset:u32,multiplier:u8)->Result<[u32;2],String>{
    if offset%4!=0||multiplier!=12||offset!=0xfe00_1b30 {
        return Err("CDSP scaled dispatcher permits only verified x12 table".into());
    }
    let immext=((offset>>20)<<16)|((offset>>6)&0x3fff)|0x4000;
    let add_mpyi=0xd860_4000|(u32::from(index.number())<<16)
        |(u32::from(dst.number())<<8)|u32::from(multiplier);
    Ok([immext,add_mpyi])
}
fn test_bit_register(predicate:u8,src:Gpr,index:Gpr)->Result<u32,String>{
    if predicate>3{return Err("CDSP tstbit predicate index outside P0..P3".into());}
    Ok(0xc700_c000|(u32::from(src.number())<<16)
        |(u32::from(index.number())<<8)|u32::from(predicate))
}
fn and_registers(dst:Gpr,left:Gpr,right:Gpr)->u32{
    0xf100_c000|(u32::from(left.number())<<16)
        |(u32::from(right.number())<<8)|u32::from(dst.number())
}
fn and_with_inverted_register(dst:Gpr,left:Gpr,right:Gpr)->u32{
    0xf180_c000|(u32::from(right.number())<<16)
        |(u32::from(left.number())<<8)|u32::from(dst.number())
}

pub fn build_dispatch_tail()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(REGION_END-PREFIX_END);
    put(&mut out,compare_greater_and_jump(r(1)?,r(0)?,0x96c0_1b70,0x96c0_1b94)?,P::ContinuationA);
    put(&mut out,or_registers(r(3)?,r(1)?,r(0)?),P::Final);
    put(&mut out,ct0(r(2)?,r(3)?),P::ContinuationA);
    put(&mut out,H::ReverseSubtractImmediate{dst:r(4)?,src:r(3)?,value:0}.encode()?,P::ContinuationA);
    put(&mut out,H::AddSignedSmallImmediate{dst:r(7)?,src:r(1)?,value:-32}.encode()?,P::ContinuationA);
    put(&mut out,compare_new_gtu_and_skip(4,0x96c0_1b78,0x96c0_1b98)?,P::Final);
    put(&mut out,compare_eq_and_self_loop(r(0)?,r(7)?,0x96c0_1b88,0x96c0_1b88)?,P::ContinuationA);
    put(&mut out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:32}.encode()?,P::ContinuationA);
    put(&mut out,H::CacheZeroLine{address:r(0)?}.encode()?,P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    let table=scaled_load_helper(r(5)?,r(2)?,0xfe00_1b30,12)?;
    put(&mut out,table[0],P::ContinuationA);
    put(&mut out,table[1],P::ContinuationA);
    put(&mut out,test_bit_register(1,r(1)?,r(2)?)?,P::ContinuationA);
    put(&mut out,and_registers(r(3)?,r(3)?,r(4)?),P::Final);
    put(&mut out,test_bit_register(0,r(0)?,r(2)?)?,P::ContinuationA);
    put(&mut out,H::JumpRegister{source:r(5)?}.encode()?,P::ContinuationA);
    put(&mut out,and_with_inverted_register(r(1)?,r(1)?,r(3)?),P::ContinuationA);
    put(&mut out,H::CombineZeroPair{low:r(2)?}.encode()?,P::Final);
    if out.len()!=REGION_END-PREFIX_END{return Err("CDSP memory dispatcher tail length mismatch".into());}
    Ok(out)
}

pub fn build_full_memory_dispatch()->Result<Vec<u8>,String>{
    let mut out=build_dispatch_prefix()?;
    out.extend(build_dispatch_tail()?);
    if out.len()!=CODE_BYTES{return Err("CDSP memory dispatcher has a code hole".into());}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_runtime_maintenance::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_full_memory_dispatch()?.as_slice()){
        return Err("CDSP variable-width dispatcher does not match pinned reference".into());
    }
    if original.get(REGION_END..ALIGNMENT_END)!=Some([0u8;ALIGNMENT_BYTES].as_slice()) {
        return Err("CDSP memory dispatch end alignment not zero".into());
    }
    Ok(())
}
pub fn build_supervisor_with_dispatch()->Result<Vec<u8>,String>{
    if cdsp_early_runtime_maintenance::REGION_END!=FILE_OFFSET{
        return Err("CDSP maintenance and variable-width dispatcher have a gap".into());
    }
    let mut out=cdsp_early_runtime_maintenance::build_supervisor_with_maintenance()?;
    out.extend(build_full_memory_dispatch()?);
    Ok(out)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn tail_cache_zero_and_register_indirect_path_have_fixed_offsets(){
        let all=build_full_memory_dispatch().unwrap();
        let tail=build_dispatch_tail().unwrap();
        assert_eq!(&all[PREFIX_END-FILE_OFFSET..],tail.as_slice());
        let word=|addr:usize|u32::from_le_bytes(all[addr-FILE_OFFSET..addr-FILE_OFFSET+4].try_into().unwrap());
        assert_eq!(word(0x4b74),P::Final.apply(or_registers(Gpr::new(3).unwrap(),Gpr::new(1).unwrap(),Gpr::new(0).unwrap())));
        assert_eq!(word(0x4b90),P::Final.apply(H::CacheZeroLine{address:Gpr::new(0).unwrap()}.encode().unwrap()));
        assert_eq!(word(0x4b94),P::Final.apply(H::JumpRegister{source:Gpr::new(31).unwrap()}.encode().unwrap()));
        assert_eq!(tail.len(),18*4);
    }
    #[test]fn tail_cache_zero_rejects_incorrect_branch_targets(){
        let r=|n|Gpr::new(n).unwrap();
        assert!(compare_greater_and_jump(r(1),r(0),0x96c01b70,0x96c01b6c).is_err());
        assert!(compare_new_gtu_and_skip(5,0x96c01b78,0x96c01b98).is_err());
        assert!(compare_eq_and_self_loop(r(0),r(7),0x96c01b88,0x96c01b84).is_err());
        assert!(scaled_load_helper(r(5),r(2),0xfe001b34,12).is_err());
    }
    #[test]fn width_dispatch_has_no_unexplained_padding(){
        assert_eq!(build_dispatch_prefix().unwrap().len(),84);
        assert_eq!(build_dispatch_tail().unwrap().len(),72);
        assert_eq!(build_full_memory_dispatch().unwrap().len(),156);
        assert_eq!(ALIGNMENT_BYTES,8);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
        assert!(p0_post_store(Width::Byte,Gpr::new(2).unwrap(),Gpr::new(0).unwrap(),0).is_err());
        assert!(p0_post_store(Width::Half,Gpr::new(2).unwrap(),Gpr::new(0).unwrap(),3).is_err());
        assert!(predicated_store(Width::Double,1,Gpr::new(3).unwrap(),Gpr::new(1).unwrap(),0).is_err());
        assert!(predicated_store(Width::Word,2,Gpr::new(2).unwrap(),Gpr::new(1).unwrap(),0).is_err());
        assert!(index_table_word_load(Gpr::new(7).unwrap(),Gpr::new(5).unwrap(),0xfe026dc8,1).is_err());
        assert!(return_unsupported_width(0,Gpr::new(31).unwrap()).is_err());
    }
}
