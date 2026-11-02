// SPDX-License-Identifier: GPL-3.0-only
//! The first worker-runtime dispatch sequence following the startup helper.
//! All bytes come from checked Hexagon V66 scalar/packet encoders; the
//! original signed CDSP reference stays external to this Git repository.
//! This fragment is a prefix of the larger controller state machine.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P};
use super::cdsp_early_controllers;
use super::cdsp_early_worker_bootstrap;

pub const FILE_OFFSET:usize=0x4c1c;
pub const REGION_END:usize=0x4c64;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1c1c;

fn append(out:&mut Vec<u8>, opcode:u32, marker:P) {
    out.extend(marker.apply(opcode).to_le_bytes());
}
/// Two 32-bit sources joined as a 64-bit destination (odd/even register
/// pair); validates Hexagon's even-low-register encoding restriction.
fn combine_register_pair(low:Gpr, upper:Gpr, lower_src:Gpr, upper_src:Gpr)->Result<u32,String>{
    if low.number()%2 !=0 || low.number()>30 || upper.number()!=low.number()+1 {
        return Err("Hexagon COMBINE pair must have matching even/odd destinations".into());
    }
    Ok(0xf500_c000 | (u32::from(upper_src.number())<<16)
        |(u32::from(lower_src.number())<<8)|u32::from(low.number()))
}
/// Ordinary V66 CALL. The signed packet-relative halfword displacement has
/// *split* fields: bits 13:0 at [13:0], bits 21:14 at [23:16]. For far
/// calls, encoding the full halfword count contiguously would be incorrect.
fn checked_short_call(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0 || target%4!=0 {return Err("unaligned Hexagon direct call".into());}
    let distance=i64::from(target)-i64::from(packet_start);
    if distance%2!=0 || !(-0x100000..=0xffffc).contains(&distance) {
        return Err("Hexagon ordinary CALL displacement outside validated range".into());
    }
    let half=(distance/2) as i32 as u32;
    Ok(0x5a00_c000 | ((half>>14 & 0xff)<<16) | (half & 0x3fff))
}
/// Compound `P0=cmp.eq(Rs,#imm); if(P0.new) jump:t|nt target`.
/// The short packet-relative jump uses a nine-bit halfword displacement with
/// its high bit at opcode bit 20 rather than contiguously adjoining bit 7.
fn compare_eq_branch_new_p0(src:Gpr,immediate:u8,packet_start:u32,target:u32,taken:bool)
    ->Result<u32,String>{
    if src.number()>7 || immediate>3 || packet_start%4!=0 || target%4!=0 || target<=packet_start {
        return Err("Hexagon P0 compound branch has invalid source/immediate/address".into());
    }
    let delta=target-packet_start;
    if delta>0x3fe || delta%2!=0 {return Err("Hexagon P0 compound branch too far".into());}
    let half=delta/2;
    Ok(0x1000_c000 | (u32::from(src.number())<<16) | (u32::from(immediate)<<8)
       | if taken {0x2000} else {0}
       | ((half&0x100)<<12) | (half&0xff))
}

/// Initial mode dispatch: invoke controller state setup, retain the double
/// register pair and select mode-1/mode-2 processing paths.  Further state
/// handling starts at `0x4c64` and is intentionally not represented here.
pub fn build_worker_runtime_prefix()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    append(&mut out,checked_short_call(0x96c0_1c1c,0x96c3_28bc)?,P::Final);
    append(&mut out,combine_register_pair(r(22)?,r(23)?,r(0)?,r(1)?)?,P::Final);
    append(&mut out,checked_short_call(0x96c0_1c24,0x96c0_2184)?,P::Final);
    append(&mut out,checked_short_call(0x96c0_1c28,0x96c0_2268)?,P::Final);
    append(&mut out,combine_register_pair(r(0)?,r(1)?,r(22)?,r(23)?)?,P::Final);
    append(&mut out,compare_eq_branch_new_p0(r(0)?,2,0x96c0_1c30,0x96c0_1f24,false)?,P::Final);
    append(&mut out,compare_eq_branch_new_p0(r(0)?,1,0x96c0_1c34,0x96c0_1cd8,true)?,P::Final);
    let assign=cdsp_boot::encode_extended_assignment(r(29)?,-0x01fe_90a0,true);
    append(&mut out,assign[0],P::ContinuationA);
    append(&mut out,assign[1],P::Final);
    append(&mut out,checked_short_call(0x96c0_1c40,0x96c0_29b0)?,P::Final);
    for (here,target) in [(0x96c0_1c44,0x96c0_1318),
                          (0x96c0_1c4c,0x96c0_1364),
                          (0x96c0_1c54,0x96c0_cdb0),
                          (0x96c0_1c5c,0x96c0_c004)] {
        let bundle=cdsp_early_controllers::encode_extended_call(here,target)?;
        append(&mut out,bundle[0],P::ContinuationA);
        append(&mut out,bundle[1],P::Final);
    }
    if out.len()!=FUNCTION_BYTES {
        return Err(format!("CDSP worker-runtime prefix returned {} expected {}",out.len(),FUNCTION_BYTES));
    }
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_worker_bootstrap::verify_reference(reference)?;
    if cdsp_early_worker_bootstrap::REGION_END!=FILE_OFFSET {
        return Err("CDSP worker-runtime does not directly follow worker bootstrap".into());
    }
    let fragment=build_worker_runtime_prefix()?;
    let original=reference.get(FILE_OFFSET..REGION_END).ok_or("CDSP reference too short")?;
    if let Some((i,(actual,expected)))=fragment.iter().zip(original).enumerate()
        .find(|(_, (a,b))|a!=b) {
        return Err(format!("CDSP worker runtime mismatch file {:#x}: typed {:02x} original {:02x}",FILE_OFFSET+i,actual,expected));
    }
    Ok(())
}
pub fn build_supervisor_with_worker_runtime()->Result<Vec<u8>,String>{
    let mut b=cdsp_early_worker_bootstrap::build_supervisor_with_worker()?;
    b.extend(build_worker_runtime_prefix()?);
    Ok(b)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn worker_runtime_checked_encoding_and_boundary(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_worker_runtime_prefix().unwrap().len(),72);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
        assert_eq!(checked_short_call(0x96c0_1c1c,0x96c3_28bc).unwrap(),0x5a06_c650);
        assert_eq!(combine_register_pair(r(22),r(23),r(0),r(1)).unwrap(),0xf501_c016);
        assert_eq!(combine_register_pair(r(0),r(1),r(22),r(23)).unwrap(),0xf517_d600);
        assert_eq!(compare_eq_branch_new_p0(r(0),2,0x96c0_1c30,0x96c0_1f24,false).unwrap(),0x1010_c27a);
        assert_eq!(compare_eq_branch_new_p0(r(0),1,0x96c0_1c34,0x96c0_1cd8,true).unwrap(),0x1000_e152);
        assert!(combine_register_pair(r(21),r(22),r(0),r(1)).is_err());
        assert!(checked_short_call(0x96c0_1c20,0x96c0_1c22).is_err());
        assert!(compare_eq_branch_new_p0(r(0),2,0x96c0_1c30,0x96c0_2130,false).is_err());
        assert!(compare_eq_branch_new_p0(r(8),2,0x96c0_1c30,0x96c0_1f24,false).is_err());
    }
}
