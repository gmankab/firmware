// SPDX-License-Identifier: GPL-3.0-only
//! Variable-width memory copy entrypoint. A four-entry pointer table selects
//! byte, halfword, word or doubleword copying; the P0/P1-predicated stores
//! below are expressed with checked operand and increment widths.
//! The continuation at 0x4b70 is reconstructed in the next section.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_runtime_maintenance;

pub const FILE_OFFSET:usize=0x4b1c;
pub const REGION_END:usize=0x4b70;
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
    let mut out=Vec::with_capacity(CODE_BYTES);
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
    if out.len()!=CODE_BYTES {return Err(format!("CDSP width dispatcher size {}, expected {CODE_BYTES}",out.len()));}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_runtime_maintenance::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_dispatch_prefix()?.as_slice()){
        return Err("CDSP variable-width dispatcher does not match pinned reference".into());
    }
    Ok(())
}
pub fn build_supervisor_with_dispatch()->Result<Vec<u8>,String>{
    if cdsp_early_runtime_maintenance::REGION_END!=FILE_OFFSET{
        return Err("CDSP maintenance and variable-width dispatcher have a gap".into());
    }
    let mut out=cdsp_early_runtime_maintenance::build_supervisor_with_maintenance()?;
    out.extend(build_dispatch_prefix()?);
    Ok(out)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn width_dispatch_has_no_unexplained_padding(){
        assert_eq!(build_dispatch_prefix().unwrap().len(),84);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
        assert!(p0_post_store(Width::Byte,Gpr::new(2).unwrap(),Gpr::new(0).unwrap(),0).is_err());
        assert!(p0_post_store(Width::Half,Gpr::new(2).unwrap(),Gpr::new(0).unwrap(),3).is_err());
        assert!(predicated_store(Width::Double,1,Gpr::new(3).unwrap(),Gpr::new(1).unwrap(),0).is_err());
        assert!(predicated_store(Width::Word,2,Gpr::new(2).unwrap(),Gpr::new(1).unwrap(),0).is_err());
        assert!(index_table_word_load(Gpr::new(7).unwrap(),Gpr::new(5).unwrap(),0xfe026dc8,1).is_err());
        assert!(return_unsupported_width(0,Gpr::new(31).unwrap()).is_err());
    }
}
