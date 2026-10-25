// SPDX-License-Identifier: GPL-3.0-only
//! Checked HALO packed-memory *loading model*, not a DSP interpreter.
//!
//! A static WMFW image supplies sparse memory writes. XM/YM host-register
//! groups decode to logical 24-bit DSP data words at their declared word
//! addresses. PM remains a 40-bit transport-only representation. Absent XM
//! regions remain unknown rather than becoming silently initialized zeros.
//! The model validates register-mapped transfer extents (CS35L41 AHB bases),
//! and proves every serialized word can be recovered byte-for-byte from the
//! resulting address space. It never generates proprietary HALO opcodes.
use crate::{wmfw_cs35l41_bus::{self,BusTransfer},wmfw_halo_memory::{self,HaloMemoryKind},
            wmfw_memory_map,wmfw_plan::{self,WmfwLayoutRegion}};
use std::ops::Range;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct PackedMemoryImage {
    pub kind:HaloMemoryKind,
    pub words:Vec<Option<u64>>,
}
impl PackedMemoryImage {
    pub fn initialized(&self)->usize{self.words.iter().filter(|w|w.is_some()).count()}
    pub fn uninitialized_spans(&self)->Vec<Range<u32>> {
        let mut gaps=Vec::new();let mut start=None;
        for (index,word) in self.words.iter().enumerate(){
            match (start,word) {
                (None,None)=>start=Some(index),
                (Some(a),Some(_))=>{gaps.push((a as u32)..(index as u32));start=None;},
                _=>{},
            }
        }
        if let Some(a)=start {gaps.push((a as u32)..(self.words.len() as u32));}
        gaps
    }
}

#[derive(Debug,Clone)]
pub struct LoadImage {
    pub xm:PackedMemoryImage,
    pub ym:PackedMemoryImage,
    pub pm:PackedMemoryImage,
    pub original_file_bytes_written:usize,
    pub mapped:Vec<BusTransfer>,
}
impl LoadImage {
    pub fn memory(&self,kind:HaloMemoryKind)->&PackedMemoryImage {
        match kind {HaloMemoryKind::PackedX=>&self.xm,
            HaloMemoryKind::PackedY=>&self.ym,
            HaloMemoryKind::PackedProgram=>&self.pm}
    }
}

fn new_space(kind:HaloMemoryKind,capacity:u32)->Result<PackedMemoryImage,String>{
    let count=usize::try_from(capacity).map_err(|_|"DSP capacity exceeds addressable memory")?;
    // Caller obtains capacity from fixed checked WMFW reference header, not
    // unbounded attacker-provided header fields.
    Ok(PackedMemoryImage{kind,words:vec![None;count]})
}

fn insert_region(img:&mut PackedMemoryImage,r:&WmfwLayoutRegion,data:&[u8])
    ->Result<(),String> {
    if img.kind!=HaloMemoryKind::from_wmfw_type(r.kind)? {
        return Err("packed memory kind does not match WMFW section".into());
    }
    if data.len()!=r.payload_length{return Err("packed DSP section length mismatch".into());}
    if img.kind!=HaloMemoryKind::PackedProgram && r.dsp_address%4!=0 {
        return Err("packed XM/YM section must begin on a four-word group boundary".into());
    }
    let decoded=wmfw_halo_memory::unpack_words(img.kind,data)?;
    let begin=usize::try_from(r.dsp_address).map_err(|_|"DSP word offset overflow")?;
    let end=begin.checked_add(decoded.len()).ok_or("DSP packed extent overflow")?;
    let range=img.words.get_mut(begin..end)
        .ok_or_else(||format!("{:?} write outside DSP memory size",img.kind))?;
    if range.iter().any(Option::is_some) {
        return Err(format!("overlapping {:?} DSP word write",img.kind));
    }
    for (dest,word) in range.iter_mut().zip(decoded){*dest=Some(word);}
    Ok(())
}

/// Apply the exact seven packed WMFW sections to separate X, Y and program
/// memory word images. This simulates *address placement*, not hardware state.
pub fn load_packed_memory(wmfw:&[u8])->Result<LoadImage,String>{
    wmfw_plan::verify_container(wmfw)?;
    let layouts=wmfw_memory_map::layouts()?;
    let mapped=wmfw_cs35l41_bus::all_transfers()?;
    let mut xm=new_space(HaloMemoryKind::PackedX,layouts[0].capacity_words)?;
    let mut ym=new_space(HaloMemoryKind::PackedY,layouts[1].capacity_words)?;
    let mut pm=new_space(HaloMemoryKind::PackedProgram,layouts[2].capacity_words)?;
    let mut source_bytes=0usize;
    for r in wmfw_plan::regions()? {
        let kind=match HaloMemoryKind::from_wmfw_type(r.kind) {
            Ok(kind)=>kind,Err(_)=>continue,
        };
        let data=wmfw.get(r.payload_offset..r.end()).ok_or("WMFW packed region truncated")?;
        match kind {
            HaloMemoryKind::PackedX=>insert_region(&mut xm,&r,data)?,
            HaloMemoryKind::PackedY=>insert_region(&mut ym,&r,data)?,
            HaloMemoryKind::PackedProgram=>insert_region(&mut pm,&r,data)?,
        }
        source_bytes=source_bytes.checked_add(data.len()).ok_or("WMFW loader payload accounting overflow")?;
    }
    for (memory,layout) in [(&xm,&layouts[0]),(&ym,&layouts[1]),(&pm,&layouts[2])] {
        if memory.initialized()!=layout.explicitly_initialized_words as usize ||
           memory.uninitialized_spans()!=layout.uninitialized_intervals {
            return Err(format!("{:?} loader coverage differs from declared region map",memory.kind));
        }
    }
    if source_bytes!=crate::wmfw_external_payload::EXTERNAL_SOURCE_BYTES {
        return Err("WMFW loader did not consume expected packed payloads".into());
    }
    Ok(LoadImage{xm,ym,pm,original_file_bytes_written:source_bytes,mapped})
}

/// Re-encode the bytes from the loaded sparse DSP word map. Unknown memory
/// cannot be read as if it contained zeroes; this catches invalid routing.
pub fn extract_packed_region(img:&LoadImage,region:&WmfwLayoutRegion)->Result<Vec<u8>,String>{
    let kind=HaloMemoryKind::from_wmfw_type(region.kind)?;
    if kind!=HaloMemoryKind::PackedProgram && region.dsp_address%4!=0 {
        return Err("packed XM/YM section must begin on a four-word group boundary".into());
    }
    let space=img.memory(kind);
    let start=region.dsp_address as usize;
    let count=region.word_count.ok_or("WMFW section has no word count")?;
    let end=start.checked_add(count).ok_or("DSP word extent overflow")?;
    let words=space.words.get(start..end).ok_or("DSP word slice out of bounds")?;
    let present=words.iter().map(|w|w.ok_or("trying to serialize missing DSP word"))
        .collect::<Result<Vec<_>,_>>()?;
    let bytes=wmfw_halo_memory::pack_words(kind,&present)?;
    if bytes.len()!=region.payload_length{return Err("regenerated packed DSP section length mismatch".into());}
    Ok(bytes)
}

pub fn compare_loaded_regions(wmfw:&[u8],img:&LoadImage)->Result<(),String>{
    for region in wmfw_plan::regions()? {
        if HaloMemoryKind::from_wmfw_type(region.kind).is_err(){continue;}
        let actual=extract_packed_region(img,&region)?;
        if wmfw.get(region.payload_offset..region.end())!=Some(actual.as_slice()) {
            return Err(format!("loader memory differs from packed WMFW region 0x{:x}",region.payload_offset));
        }
    }
    Ok(())
}

/// One bounded AHB write descriptor; actual bytes are *not* included in this
/// manifest. The hardware/SPI transport may split it differently at runtime.
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct PlannedAhbWrite {
    pub memory:HaloMemoryKind,
    pub file_span:Range<usize>,
    pub ahb_span:Range<u32>,
}

/// Divide each region into at-most max_bytes transfers, retaining 4-byte
/// address alignment and exact WMFW payload order. These are upper-bounded
/// regmap *write spans*, not guaranteed physical SPI/I2C transaction sizes.
pub fn bounded_writes(image:&LoadImage,max_bytes:usize)->Result<Vec<PlannedAhbWrite>,String>{
    if max_bytes<4 || max_bytes%4!=0 || max_bytes>u32::MAX as usize {
        return Err("invalid packed AHB write size/alignment".into());
    }
    let mut result=Vec::new();
    for region in &image.mapped {
        if region.original_file_span.len()!=region.ahb_register_span.len() as usize {
            return Err("mapped WMFW bytes do not match AHB address width".into());
        }
        let mut offset=0usize;
        while offset<region.original_file_span.len() {
            let size=(region.original_file_span.len()-offset).min(max_bytes);
            let reg_offset=u32::try_from(offset).map_err(|_|"AHB write offset overflow")?;
            let size_u32=u32::try_from(size).map_err(|_|"AHB write size overflow")?;
            let ahb_start=region.ahb_register_span.start.checked_add(reg_offset).ok_or("AHB byte offset overflow")?;
            let ahb_end=ahb_start.checked_add(size_u32).ok_or("AHB span end overflow")?;
            let file_start=region.original_file_span.start.checked_add(offset).ok_or("WMFW file offset overflow")?;
            result.push(PlannedAhbWrite {memory:region.memory,
                file_span:file_start..file_start+size,
                ahb_span:ahb_start..ahb_end});
            offset+=size;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn synthetic_wmfw()->Vec<u8> {
        let mut plan=wmfw_plan::partial_container().unwrap();
        for (n,r) in wmfw_plan::regions().unwrap().iter().filter(|r|
            HaloMemoryKind::from_wmfw_type(r.kind).is_ok()).enumerate() {
            let payload=(0..r.payload_length).map(|i|((n*17+i*13) & 0xff) as u8).collect::<Vec<_>>();
            plan.place("synthetic packed memory",r.payload_offset,&payload).unwrap();
        }
        plan.build_complete().unwrap()
    }
    #[test]
    fn preserves_sparse_xm_and_explicit_ym_pm_from_synthetic_input(){
        let source=synthetic_wmfw();
        let image=load_packed_memory(&source).unwrap();
        assert_eq!((image.xm.initialized(),image.ym.initialized(),image.pm.initialized()),(1280,2044,4056));
        assert_eq!(image.original_file_bytes_written,30252);
        assert_eq!(image.xm.uninitialized_spans(),vec![0x18..0x34,0xe4..0x140,0x350..0x354,0x500..0x530]);
        assert_eq!(image.ym.uninitialized_spans(),vec![]);
        assert_eq!(image.pm.uninitialized_spans(),vec![]);
        compare_loaded_regions(&source,&image).unwrap();
        assert_eq!(image.mapped.len(),7);
    }
    #[test]
    fn bounded_writes_cover_only_explicit_payloads_without_silent_holes(){
        let image=load_packed_memory(&synthetic_wmfw()).unwrap();
        let writes=bounded_writes(&image,128).unwrap();
        assert!(writes.len()>7);
        assert_eq!(writes.iter().map(|w|w.file_span.len()).sum::<usize>(),30252);
        for w in &writes {
            assert_eq!(w.file_span.len(),w.ahb_span.len() as usize);
            assert_eq!(w.ahb_span.start%4,0);
            assert_eq!(w.ahb_span.end%4,0);
            assert!(w.file_span.len()<=128);
        }
        for region in &image.mapped {
            let chunks=writes.iter().filter(|w|w.memory==region.memory &&
                w.ahb_span.start>=region.ahb_register_span.start &&
                w.ahb_span.end<=region.ahb_register_span.end).collect::<Vec<_>>();
            assert!(!chunks.is_empty());
            assert_eq!(chunks.first().unwrap().file_span.start,region.original_file_span.start);
            assert_eq!(chunks.last().unwrap().file_span.end,region.original_file_span.end);
        }
        for invalid in [0,3,5,usize::MAX] {
            assert!(bounded_writes(&image,invalid).is_err());
        }
    }
    #[test]
    fn cannot_serialize_unknown_or_overlapping_dsp_words(){
        let source=synthetic_wmfw();
        let mut image=load_packed_memory(&source).unwrap();
        let first=wmfw_plan::regions().unwrap()[1].clone();
        image.xm.words[0]=None;
        assert!(extract_packed_region(&image,&first).unwrap_err().contains("missing"));
        let mut image=load_packed_memory(&source).unwrap();
        let second=wmfw_plan::regions().unwrap()[2].clone();
        let payload=vec![1;second.payload_length];
        assert!(insert_region(&mut image.xm,&second,&payload).unwrap_err().contains("overlapping"));
        assert!(insert_region(&mut image.ym,&second,&payload).unwrap_err().contains("kind"));
        let mut image=load_packed_memory(&source).unwrap();
        image.pm.words[0]=Some(1<<40);
        let pm=wmfw_plan::regions().unwrap()[7].clone();
        assert!(extract_packed_region(&image,&pm).unwrap_err().contains("exceeds"));
    }

    #[test]
    fn places_logical_x_y_words_from_register_lanes_and_preserves_sparse_holes() {
        let mut source=synthetic_wmfw();
        let first=wmfw_plan::regions().unwrap()[1].clone();
        let packed=[0x66,0x11,0x22,0x33,0x88,0x99,0x44,0x55,0xaa,0xbb,0xcc,0x77];
        source[first.payload_offset..first.payload_offset+12].copy_from_slice(&packed);
        let image=load_packed_memory(&source).unwrap();
        assert_eq!(image.xm.words[..4],
            [Some(0x112233),Some(0x445566),Some(0x778899),Some(0xaabbcc)]);
        assert_eq!(image.xm.words[0x18],None);
        assert_eq!(&extract_packed_region(&image,&first).unwrap()[..12],&packed);
        let mut unaligned=first;
        unaligned.dsp_address=1;
        assert!(extract_packed_region(&image,&unaligned).unwrap_err().contains("boundary"));
        let mut target=new_space(HaloMemoryKind::PackedX,0x5ac).unwrap();
        assert!(insert_region(&mut target,&unaligned,
            &source[unaligned.payload_offset..unaligned.end()]).unwrap_err().contains("boundary"));
        assert_eq!(target.initialized(),0);
    }
}
