// SPDX-License-Identifier: GPL-3.0-only
//! V66 I-cache tag/data recovery: cache-tag inspection, nested hardware
//! loops, callback comparison, ring-buffer snapshots and controller flags.
//! All 67 instruction words are emitted from checked operands; the privileged
//! I-cache register selectors are corroborated by the Rizin V66 decoder (LLVM6
//! does not assemble those system-register opcodes).
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::{cdsp_early_icache_guard as previous,cdsp_early_worker_command as worker,
            cdsp_early_controllers as controllers};

pub const FILE_OFFSET:usize=0x5298;
pub const REGION_END:usize=0x53a4;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2298;
fn put(v:&mut Vec<u8>,word:u32,marker:P){v.extend_from_slice(&marker.apply(word).to_le_bytes());}

fn compare_predicate_immediate(predicate:u8,src:Gpr,value:u8)->Result<u32,String>{
    if predicate>3 || src.number()>7 {return Err("V66 tested immediate compare needs P0..P3 and a low GP register".into());}
    Ok(0x7500_c000|(u32::from(src.number())<<16)|(u32::from(value)<<5)|u32::from(predicate))
}
fn select_cache_lines(predicate:u8,dst:Gpr,selected:bool,lines:u16)->Result<u32,String>{
    if predicate!=3 || dst.number()!=10 || (selected && lines!=128) || (!selected && lines!=256){
        return Err("V66 cache selector only independently assembled for P3/R10 and 128/256".into());
    }
    // LLVM6 assembles `{if (p3) r10=#128}` and
    // `{if (!p3) r10=#256}` to precisely these field layouts.
    Ok(if selected {0x7e60_d00a} else {0x7ee1_c00a})
}
fn read_loop_counter(dst:Gpr,index:u8)->Result<u32,String>{
    if index>1 {return Err("LC loop index must be 0 or 1".into());}
    Ok(0x6a00_c000|((if index==0{1}else{3})<<16)|u32::from(dst.number()))
}
fn read_icache_register(dst:Gpr,index:Gpr,data_not_tag:bool)->Result<u32,String>{
    if dst.number()>7 {return Err("I-cache tag/data reader verified for low destinations only".into());}
    Ok((if data_not_tag{0x55a0_c000}else{0x55e0_c000})
       |(u32::from(index.number())<<16)|u32::from(dst.number()))
}
fn compare_gt_and_branch_new(src:Gpr,limit:u8,here:u32,dest:u32)->Result<u32,String>{
    if src.number()>7 || limit>31 || here%4!=0 || dest%4!=0 || dest<=here || dest-here>0x1fe {
        return Err("V66 new compare+branch target or register out of range".into());
    }
    // Predicted taken, packet-relative displacement measured from its head.
    Ok(0x1080_e000|(u32::from(src.number())<<16)|
       (u32::from(limit)<<8)|((dest-here)>>1))
}
fn compare_equal_new_branch(start:u32,dest:u32)->Result<u32,String>{
    if start%4!=0 || dest%4!=0 || dest<=start || dest-start>0x1fe {
        return Err("V66 P0.new taken branch requires positive packet-relative displacement".into());
    }
    Ok(0x5c00_d800|((dest-start)>>1))
}
fn compare_new_increment_branch(start:u32,dest:u32,counter:Gpr,limit:u8)->Result<u32,String>{
    if counter.number()!=4 || limit!=8 || start%4!=0 || dest%4!=0 ||
       dest>=start || start-dest>0x1fe {
        return Err("Verified V66 new-value increment branch requires R4,#8 and small backward jump".into());
    }
    let half=((i64::from(dest)-i64::from(start))/2) as i32 as u32;
    Ok(0x2472_e800|(half&0xff))
}
fn tag_and_data_registers(dst:Gpr,left:Gpr,right:Gpr)->u32{
    0xf100_c000|(u32::from(left.number())<<16)|
      (u32::from(right.number())<<8)|u32::from(dst.number())
}
fn word_store_postincrement(src:Gpr,base:Gpr,increment:u8)->Result<u32,String>{
    if increment!=4 {return Err("I-cache ring writer only verified with four-byte stride".into());}
    Ok(0xab80_c000|(u32::from(base.number())<<16)|
       (u32::from(src.number())<<8)|u32::from(increment<<1))
}
fn absolute_state_store(src:Gpr,base:Gpr,address:u32)->Result<[u32;2],String>{
    if src.number()>7||base.number()>7||address%4!=0 {
        return Err("Controller status write requires compact GP registers and aligned address".into());
    }
    Ok([((address>>20)<<16)|((address>>6)&0x3fff)|0x4000,
       0xab80_c080|(u32::from(base.number())<<16)|
       (u32::from(src.number())<<8)|(address&0x3f)])
}

pub fn build_icache_tag_recovery()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut output=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut output,H::AssignSmallImmediate{dst:r(7)?,value:0}.encode()?,P::ContinuationA);
    let dest=cdsp_boot::encode_extended_assignment(r(8)?,0xfe01_6e30u32 as i32,true);
    put(&mut output,dest[0],P::ContinuationA);
    put(&mut output,dest[1],P::Final);
    let gp=cdsp_boot::encode_extended_gp_word_load(r(1)?,0xfe03_7dc0);
    put(&mut output,gp[0],P::ContinuationA);
    put(&mut output,gp[1],P::Final);
    put(&mut output,H::AddSmallImmediate{dst:r(8)?,src:r(8)?,value:4}.encode()?,P::ContinuationA);
    put(&mut output,compare_predicate_immediate(3,r(1)?,128)?,P::Final);
    for (dst,value,marker) in [(r(14)?,0xfe02_3900_u32,P::ContinuationA),
                                (r(15)?,0xfe01_b900_u32,P::Final)] {
        let pair=cdsp_boot::encode_extended_assignment(dst,value as i32,true);
        put(&mut output,pair[0],P::ContinuationA);
        put(&mut output,pair[1],marker);
    }
    put(&mut output,H::ReadSystem{system:SystemRegister::Syscfg,dst:r(0)?}.encode()?,P::Final);
    put(&mut output,H::ClearBit{dst:r(0)?,src:r(0)?,position:1}.encode()?,P::Final);
    put(&mut output,H::WriteSystem{system:SystemRegister::Syscfg,src:r(0)?}.encode()?,P::Final);
    put(&mut output,H::Isync.encode()?,P::Final);
    put(&mut output,select_cache_lines(3,r(10)?,true,128)?,P::ContinuationA);
    put(&mut output,select_cache_lines(3,r(10)?,false,256)?,P::Final);
    put(&mut output,H::HardwareLoop{loop_index:1,counter:r(10)?,displacement_bytes:4}.encode()?,P::Final);
    put(&mut output,read_loop_counter(r(10)?,1)?,P::Final);
    put(&mut output,H::AddSignedSmallImmediate{dst:r(12)?,src:r(10)?,value:-1}.encode()?,P::Final);
    put(&mut output,read_loop_counter(r(10)?,0)?,P::Final);
    put(&mut output,H::AddSignedSmallImmediate{dst:r(13)?,src:r(10)?,value:-1}.encode()?,P::ContinuationA);
    put(&mut output,H::AddSignedSmallImmediate{dst:r(10)?,src:r(10)?,value:-1}.encode()?,P::Final);
    put(&mut output,read_icache_register(r(1)?,r(10)?,false)?,P::Final);
    put(&mut output,H::TestBitPredicate{predicate:0,src:r(1)?,bit:1}.encode()?,P::ContinuationA);
    put(&mut output,H::JumpIfNotNewP0NotTaken{displacement_bytes:0x64}.encode()?,P::Final);
    for _ in 0..2 {put(&mut output,H::AssignSmallImmediate{dst:r(4)?,value:0}.encode()?,P::Final);}
    put(&mut output,read_icache_register(r(1)?,r(10)?,true)?,P::Final);
    put(&mut output,compare_gt_and_branch_new(r(7)?,2,0x96c0_230c,0x96c0_2350)?,P::Final);
    put(&mut output,H::AssignRegister{dst:r(0)?,src:r(1)?}.encode()?,P::Final);
    let call1=controllers::encode_extended_call(0x96c0_2314,0x96c0_2488)?;
    put(&mut output,call1[0],P::ContinuationA);
    put(&mut output,call1[1],P::Final);
    put(&mut output,tag_and_data_registers(r(16)?,r(0)?,r(1)?),P::Final);
    put(&mut output,H::AssignRegister{dst:r(0)?,src:r(6)?}.encode()?,P::Final);
    let call2=controllers::encode_extended_call(0x96c0_2324,0x96c0_2488)?;
    put(&mut output,call2[0],P::ContinuationA);
    put(&mut output,call2[1],P::Final);
    put(&mut output,tag_and_data_registers(r(11)?,r(0)?,r(6)?),P::Final);
    put(&mut output,H::PredicateP0EqualsRegisters{left:r(16)?,right:r(11)?}.encode()?,P::ContinuationA);
    put(&mut output,compare_equal_new_branch(0x96c0_2330,0x96c0_2350)?,P::Final);
    for src in [3,2,10,6,1] {put(&mut output,word_store_postincrement(r(src)?,r(8)?,4)?,P::Final);}
    put(&mut output,H::AddSmallImmediate{dst:r(7)?,src:r(7)?,value:1}.encode()?,P::Final);
    put(&mut output,H::AddSmallImmediate{dst:r(10)?,src:r(10)?,value:4}.encode()?,P::Final);
    put(&mut output,H::AddSmallImmediate{dst:r(4)?,src:r(4)?,value:1}.encode()?,P::ContinuationA);
    put(&mut output,compare_new_increment_branch(0x96c0_2354,0x96c0_2308,r(4)?,8)?,P::Final);
    for marker in [P::LoopEnd,P::LoopEnd,P::Final] {put(&mut output,H::Nop.encode()?,marker);}
    put(&mut output,H::ReadSystem{system:SystemRegister::Syscfg,dst:r(1)?}.encode()?,P::Final);
    put(&mut output,H::SetBit{dst:r(1)?,src:r(1)?,position:1}.encode()?,P::Final);
    put(&mut output,H::WriteSystem{system:SystemRegister::Syscfg,src:r(1)?}.encode()?,P::Final);
    put(&mut output,H::Isync.encode()?,P::Final);
    put(&mut output,H::AssignSmallImmediate{dst:r(1)?,value:2}.encode()?,P::Final);
    let first=absolute_state_store(r(1)?,r(0)?,0xfe01_6e20)?;
    put(&mut output,first[0],P::ContinuationA);put(&mut output,first[1],P::Final);
    put(&mut output,worker::cache_clean(r(0)?),P::Final);
    put(&mut output,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:32}.encode()?,P::Final);
    put(&mut output,worker::cache_clean(r(0)?),P::Final);
    let second=absolute_state_store(r(7)?,r(0)?,0xfe01_6e30)?;
    put(&mut output,second[0],P::ContinuationA);put(&mut output,second[1],P::Final);
    put(&mut output,worker::cache_clean(r(0)?),P::Final);
    // LLVM6 independently encodes the zero-operand Hexagon deallocframe.
    put(&mut output,0x901e_c01e,P::Final);
    put(&mut output,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if output.len()!=FUNCTION_BYTES {
        return Err(format!("I-cache tag recovery emitted {} bytes, expected {}",output.len(),FUNCTION_BYTES));
    }
    Ok(output)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("I-cache guard does not abut I-cache recovery".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_icache_tag_recovery()?.as_slice()){
        return Err("CDSP I-cache tag recovery differs from original pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn nested_icache_tag_loops_and_operand_forms_are_checked(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_icache_tag_recovery().unwrap().len(),268);
        assert_eq!(compare_predicate_immediate(3,r(1),128).unwrap(),0x7501_d003);
        assert!(compare_predicate_immediate(4,r(1),128).is_err());
        assert_eq!(select_cache_lines(3,r(10),true,128).unwrap(),0x7e60_d00a);
        assert!(select_cache_lines(3,r(10),true,256).is_err());
        assert_eq!(read_loop_counter(r(10),1).unwrap(),0x6a03_c00a);
        assert!(read_loop_counter(r(10),2).is_err());
        assert_eq!(read_icache_register(r(1),r(10),false).unwrap(),0x55ea_c001);
        assert_eq!(read_icache_register(r(1),r(10),true).unwrap(),0x55aa_c001);
        assert_eq!(compare_gt_and_branch_new(r(7),2,0x96c0_230c,0x96c0_2350).unwrap(),0x1087_e222);
        assert!(compare_gt_and_branch_new(r(7),2,0x96c0_230c,0x96c0_2308).is_err());
        assert_eq!(compare_equal_new_branch(0x96c0_2330,0x96c0_2350).unwrap(),0x5c00_d810);
        assert_eq!(compare_new_increment_branch(0x96c0_2354,0x96c0_2308,r(4),8).unwrap(),0x2472_e8da);
        assert!(compare_new_increment_branch(0x96c0_2354,0x96c0_2308,r(5),8).is_err());
        assert_eq!(word_store_postincrement(r(3),r(8),4).unwrap(),0xab88_c308);
        assert!(word_store_postincrement(r(3),r(8),8).is_err());
        assert_eq!(absolute_state_store(r(7),r(0),0xfe01_6e30).unwrap(),[0x0fe0_45b8,0xab80_c7b0]);
        assert!(absolute_state_store(r(7),r(0),0xfe01_6e31).is_err());
    }
}
