// SPDX-License-Identifier: GPL-3.0-only
//! Source-authored, independently reconstructed 6,156-byte Venus controller
//! cluster. Every constituent function has its own strict address verifier.
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_quality.rs"]mod quality;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_record_metrics.rs"]mod record_metrics;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_magnitude.rs"]mod magnitude;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_alt_occupancy.rs"]mod alt_occupancy;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_occupancy_commit.rs"]mod occupancy_commit;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_record_append.rs"]mod record_append;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_record_select.rs"]mod record_select;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_color_unpack.rs"]mod color_unpack;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_controller_limits.rs"]mod controller_limits;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_controller_limits_tail.rs"]mod controller_limits_tail;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_throughput_tiers.rs"]mod throughput_tiers;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_controller_alloc.rs"]mod controller_alloc;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let out=std::env::args().nth(1).ok_or("usage: agent20_05_controller_chain OUTPUT [PRIVATE_VENUS_MBN]")?;
    let pieces: [(usize,usize,fn()->Result<Vec<u8>,String>);12] = [
        (quality::START,quality::END,quality::fragment),
        (record_metrics::START,record_metrics::END,record_metrics::fragment),
        (magnitude::START,magnitude::END,magnitude::fragment),
        (alt_occupancy::START,alt_occupancy::END,alt_occupancy::fragment),
        (occupancy_commit::START,occupancy_commit::END,occupancy_commit::fragment),
        (record_append::START,record_append::END,record_append::fragment),
        (record_select::START,record_select::END,record_select::fragment),
        (color_unpack::START,color_unpack::END,color_unpack::fragment),
        (controller_limits::START,controller_limits::END,controller_limits::fragment),
        (controller_limits_tail::START,controller_limits_tail::END,controller_limits_tail::fragment),
        (throughput_tiers::START,throughput_tiers::END,throughput_tiers::fragment),
        (controller_alloc::START,controller_alloc::END,controller_alloc::fragment),
    ];
    for base in [quality::FILE_BASE,record_metrics::FILE_BASE,magnitude::FILE_BASE,
                 alt_occupancy::FILE_BASE,occupancy_commit::FILE_BASE,
                 record_append::FILE_BASE,record_select::FILE_BASE,
                 color_unpack::FILE_BASE,controller_limits::FILE_BASE,
                 controller_limits_tail::FILE_BASE,throughput_tiers::FILE_BASE,controller_alloc::FILE_BASE] {
        if base != 0x4000 {return Err("unexpected Venus file-to-runtime offset".into());}
    }
    let mut bytes=Vec::with_capacity(7252);let mut position=0x710c0;
    for (start,end,emit) in pieces {
        if start!=position{return Err(format!("gap or overlap at 0x{position:x}").into());}
        let next=emit()?;
        if next.len()!=end-start{return Err(format!("unexpected fragment length at 0x{start:x}").into());}
        bytes.extend_from_slice(&next);position=end;
    }
    if position!=0x72d14 || bytes.len()!=7252{return Err("unexpected controller-chain coverage".into());}
    if let Some(reference)=std::env::args().nth(2){
        let firmware=std::fs::read(reference)?;
        let file_offset=0x710c0+0x4000;
        if firmware.get(file_offset..file_offset+bytes.len())!=Some(bytes.as_slice()){
            return Err("controller chain does not match reference firmware".into());
        }
    }
    std::fs::write(out,bytes)?;Ok(())
}
