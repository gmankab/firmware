// SPDX-License-Identifier: GPL-3.0-only
//! Checked multi-stage ELF32/MBNv6 assembler for independently authored
//! firmware programs. This builder refuses unknown file bytes and keeps
//! externally provided OEM authentication separate from generated coverage.
//! It deliberately cannot create a new Qualcomm hardware-trusted signature.
use std::ops::Range;
use crate::{elf32_prefix::{write_elf32_first_page,Elf32Spec},
    image_plan::{ImagePlan,CoverageReport}, parse_elf32,
    qcom_mbn::{V6Header,V6Layout},
    qcom_mbn_validation::sha384_table_from_generated_plan,
    qcom_mbn_profiles::{unsigned_header_and_policy,unsigned_metadata_plan}};

pub struct MbnSourceBuilder<'a> {
    spec: &'a Elf32Spec,
    plan: ImagePlan,
    signed_layout: V6Layout,
    hash_segment_index: usize,
    hash_segment_file_start: usize,
    signed_header: V6Header,
}

impl<'a> MbnSourceBuilder<'a> {
    pub fn new(spec:&'a Elf32Spec,total_file_bytes:usize)->Result<Self,String>{
        let plan=unsigned_metadata_plan(spec,total_file_bytes)?;
        let (signed_header,_) = unsigned_header_and_policy(spec)?;
        let mut candidates=spec.headers.iter().enumerate()
            .filter(|(_,ph)|ph.flags & 0x0700_0000==0x0200_0000);
        let (idx,hash)=candidates.next().ok_or("missing MBNv6 hash segment")?;
        if candidates.next().is_some() {return Err("multiple MBNv6 hash segments".into());}
        let signed_layout=signed_header.layout(hash.filesz as usize)?;
        if signed_layout.padding.len()!=0 {
            return Err("unexpected MBNv6 unsigned metadata or trailing padding".into());
        }
        Ok(Self{spec,plan,signed_layout,hash_segment_index:idx,
            hash_segment_file_start:hash.offset as usize,signed_header})
    }

    pub fn coverage(&self)->CoverageReport{self.plan.coverage()}
    pub fn plan(&self)->&ImagePlan{&self.plan}
    pub fn into_plan(self)->ImagePlan{self.plan}

    /// Add exactly one complete source-built ELF segment; allow matching
    /// first-page metadata already present but refuse overlapping unknown
    /// partial content and any attempt to import the signed MBN hash segment.
    pub fn add_generated_segment(&mut self,index:usize,bytes:&[u8])->Result<(),String>{
        let seg=self.spec.headers.get(index).ok_or("ELF program-header index out of bounds")?;
        if index==self.hash_segment_index {return Err("cannot import signed MBN hash segment".into());}
        if seg.filesz==0 {return Err("BSS has no file bytes; do not synthesize it".into());}
        if bytes.len()!=seg.filesz as usize {
            return Err(format!("ELF segment {index} requires {} bytes, got {}",seg.filesz,bytes.len()));
        }
        let at=seg.offset as usize;
        let end=at.checked_add(bytes.len()).ok_or("segment offset overflow")?;
        if let Ok(existing)=self.plan.generated_slice(at..end) {
            if existing!=bytes {
                return Err(format!("source-generated ELF segment {index} differs from existing image"));
            }
            return Ok(());
        }
        self.plan.place(&format!("independently built ELF segment {index}"),at,bytes)
    }

    /// Explicitly reconstructed zero padding only if **no program header**
    /// assigns file bytes to this interval. The caller must have established
    /// the reference format's padding invariant; no automatic filling occurs.
    pub fn add_verified_zero_padding(&mut self,span:Range<usize>)->Result<(),String>{
        if span.is_empty(){return Err("zero padding region cannot be empty".into());}
        if span.end>self.plan.total_bytes() || span.start>span.end {
            return Err("padding range extends outside ELF file".into());
        }
        for (index,h) in self.spec.headers.iter().enumerate(){
            if h.filesz==0{continue;}
            let start=h.offset as usize;
            let end=start.checked_add(h.filesz as usize).ok_or("ELF span overflow")?;
            if span.start<end && start<span.end {
                return Err(format!("zero padding overlaps file-backed ELF segment {index}"));
            }
        }
        self.plan.place("explicit format-defined zero alignment",span.start,&vec![0;span.len()])
    }

    /// Only generate SHA-384 entries once every file-backed hashed segment
    /// has original, independently authored bytes. No reference slices enter.
    pub fn generate_hashes(&mut self)->Result<(),String>{
        let elf=parse_elf32(&write_elf32_first_page(self.spec)?)?;
        let table=sha384_table_from_generated_plan(&elf,self.hash_segment_index,&self.plan)?;
        if table.len()!=self.signed_layout.hashes.len(){
            return Err("SHA-384 entry count contradicts MBN header".into());
        }
        self.plan.place("generated ELF segment SHA-384 digests",
            self.hash_segment_file_start+self.signed_layout.hashes.start,&table)
    }

    /// Optional original OEM signature and DER certificate field, supplied
    /// **only** by the device owner after independently generating all code.
    /// This leaves those bytes classified as external, not new firmware code.
    pub fn add_external_oem_authentication(&mut self,data:&[u8])->Result<(),String>{
        if self.signed_header.qti_signature_size!=0 || self.signed_header.qti_certificate_size!=0 {
            return Err("dual-signer MBNv6 is not handled by this builder".into());
        }
        let start=self.signed_layout.oem_signature.start;
        let end=self.signed_layout.oem_certificate.end;
        if data.len()!=end-start {
            return Err(format!("expected {0} external OEM authentication bytes, got {1}",end-start,data.len()));
        }
        self.plan.place_external_authentication("original external OEM signature+certificate",
            self.hash_segment_file_start+start,data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::xiaomi_nabu::{a640_plan,elf_specs::KNOWN_ELFS};
    fn spec()->&'static Elf32Spec{KNOWN_ELFS.iter().find(|p|p.name=="a640_zap.mbn").unwrap()}
    #[test]
    fn a640_header_hash_executable_and_alignment_are_source_only() {
        let spec=spec();let mut b=MbnSourceBuilder::new(spec,0x37b0).unwrap();
        let elf=write_elf32_first_page(spec).unwrap();
        b.add_generated_segment(0,&elf[..148]).unwrap();
        assert!(b.generate_hashes().unwrap_err().contains("unimplemented file byte at 0x3000"));
        b.add_generated_segment(2,&a640_plan::partial_executable().unwrap().build_complete().unwrap()).unwrap();
        assert!(b.add_verified_zero_padding(0x1000..0x1030).is_err());
        b.add_verified_zero_padding(0x2a38..0x3000).unwrap();
        b.generate_hashes().unwrap();
        assert_eq!(b.coverage().generated_bytes,7856);
        assert_eq!(b.coverage().holes,vec![0x1138..0x2a38]);
        assert!(b.plan().build_complete().is_err());
        let comparison=b.plan().clone();
        assert!(b.add_external_oem_authentication(&[]).unwrap_err().contains("6400"));
        b.add_external_oem_authentication(&vec![0x55;6400]).unwrap();
        assert_eq!(b.coverage().externally_supplied_bytes,6400);
        assert!(b.plan().build_independently_generated().is_err());
        let output=b.into_plan().build_complete().unwrap();
        comparison.compare_known(&output).unwrap();
        crate::qcom_mbn_validation::verify_elf32_mbnv6_hashes(&output).unwrap();
    }
    #[test]
    fn refuses_wrong_segment_size_or_external_hash_block() {
        let spec=spec();let mut b=MbnSourceBuilder::new(spec,0x37b0).unwrap();
        assert!(b.add_generated_segment(1,&vec![0;0x1a38]).unwrap_err().contains("cannot import"));
        assert!(b.add_generated_segment(2,&[1,2]).unwrap_err().contains("requires"));
        assert!(b.add_generated_segment(3,&[]).is_err());
        assert!(b.add_verified_zero_padding(0x3000..0x3001).unwrap_err().contains("overlaps"));
        assert!(b.add_external_oem_authentication(&[0;15]).is_err());
    }
}
