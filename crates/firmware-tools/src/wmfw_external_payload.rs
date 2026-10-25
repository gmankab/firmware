// SPDX-License-Identifier: GPL-3.0-only
//! Opt-in *external* firmware payload importer for a byte-identical WMFW file.
//!
//! The seven Cirrus DSP payloads remain opaque, proprietary source inputs.
//! They are never embedded in the repository and are NOT independently
//! reconstructed HALO code. Separate pinned SHA-256 hashes prevent accidental
//! mixing of firmware revisions. The WMFW header, descriptors, INFO text and
//! 57 algorithm-control records always come from independently authored Rust.

use crate::{wmfw_plan,wmfw_sha256};
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::io::Write;

pub const REFERENCE_SHA256:&str="d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c";

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct OpaqueRegion {
    pub file_offset:usize,
    pub size:usize,
    pub kind:u8,
    pub expected_sha256:&'static str,
}
impl OpaqueRegion {
    pub fn filename(self) -> String {format!("region-{:04x}.bin",self.file_offset)}
}

pub const OPAQUE_REGIONS:[OpaqueRegion;7]=[
    OpaqueRegion {file_offset:0x0068,size:72,kind:0x11,expected_sha256:"44510542e96ef4ba1c5a4fea0cb0b516cd81698bc83cb254d30b055f1329dd7f"},
    OpaqueRegion {file_offset:0x00b8,size:528,kind:0x11,expected_sha256:"a4a920f6087690c76ddedf75356474a526337c8d1484def61a66c1b8ece4e311"},
    OpaqueRegion {file_offset:0x02d0,size:1584,kind:0x11,expected_sha256:"8fc53279ac0562afd122ff8133edd26d51c5ae620317aab9810050628af09e78"},
    OpaqueRegion {file_offset:0x0908,size:1284,kind:0x11,expected_sha256:"c7f0a17c379cd0e7bd7bd54685cfef97db7857ff43616b5b111670520894caa5"},
    OpaqueRegion {file_offset:0x0e14,size:372,kind:0x11,expected_sha256:"e599a34d425aa7136cabd30cc375212584c2f3c3541a8b6939b810bd10fe09d4"},
    OpaqueRegion {file_offset:0x0f90,size:6132,kind:0x12,expected_sha256:"a4d56ca068e2631f6f93e7c31ab8a01a4353b38108766bd6de7a10f14fcd7220"},
    OpaqueRegion {file_offset:0x278c,size:20280,kind:0x10,expected_sha256:"0edacef35965ea2bd8c709f8cad73fd08714bb47922065d25e3b9ba1bf34c517"},
];

pub const EXTERNAL_SOURCE_BYTES:usize=30_252;

/// Build a complete WMFW using externally authored *opaque* DSP payloads.
/// The closure must supply every missing payload and its bytes are separately
/// integrity checked. This does not increase independently reconstructed code.
pub fn build_with_external_payloads<F>(mut read_region:F)->Result<Vec<u8>,String>
where F:FnMut(OpaqueRegion)->Result<Vec<u8>,String> {
    let mut plan=wmfw_plan::partial_container()?;
    let regions=wmfw_plan::regions()?;
    if plan.holes().len()!=OPAQUE_REGIONS.len() {return Err("unexpected WMFW opaque region count".into());}
    let mut total=0;
    for (item,real) in OPAQUE_REGIONS.iter().zip(regions.iter().filter(|r|matches!(r.kind,0x10..=0x12))) {
        if item.file_offset!=real.payload_offset||item.size!=real.payload_length||item.kind!=real.kind {
            return Err(format!("WMFW opaque manifest mismatch at 0x{:x}",item.file_offset));
        }
        let source=read_region(*item)?;
        if source.len()!=item.size {
            return Err(format!("external payload {} length {} != {}",item.filename(),source.len(),item.size));
        }
        if wmfw_sha256::hex_digest(&source)!=item.expected_sha256 {
            return Err(format!("external payload {} SHA-256 mismatch",item.filename()));
        }
        plan.place(&format!("EXTERNAL proprietary WMFW payload {}",item.filename()),
                   item.file_offset,&source)?;
        total+=source.len();
    }
    if total!=EXTERNAL_SOURCE_BYTES || plan.filled_bytes()!=wmfw_plan::FILE_SIZE {
        return Err("external WMFW coverage incomplete".into());
    }
    let assembled=plan.build_complete()?;
    if wmfw_sha256::hex_digest(&assembled)!=REFERENCE_SHA256 {
        return Err("assembled WMFW has wrong full-file SHA-256".into());
    }
    Ok(assembled)
}

pub fn assemble_directory(input_dir:&Path)->Result<Vec<u8>,String>{
    build_with_external_payloads(|item|fs::read(input_dir.join(item.filename()))
        .map_err(|e|format!("reading externally supplied {}: {e}",item.filename())))
}

/// Temporary directory used for an all-or-nothing extraction. If any write,
/// sync, or final directory rename fails, the staging directory and its
/// original DSP input bytes are removed. A *new* output folder is required so
/// existing user files can never be overwritten.
struct StagedDirectory {
    path:PathBuf,
    committed:bool,
}
impl Drop for StagedDirectory {
    fn drop(&mut self) {
        if !self.committed { let _=fs::remove_dir_all(&self.path); }
    }
}
static STAGING_SEQUENCE:AtomicU64=AtomicU64::new(0);

fn stage_new_private_directory(output_dir:&Path)->Result<StagedDirectory,String> {
    if fs::symlink_metadata(output_dir).is_ok() {
        return Err(format!("WMFW output directory already exists: {}",output_dir.display()));
    }
    let parent=output_dir.parent().filter(|p|!p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(|e|format!("creating private payload parent: {e}"))?;
    let folder=output_dir.file_name().ok_or("WMFW payload path must name a new folder")?
        .to_string_lossy();
    for _ in 0..128 {
        let unique=STAGING_SEQUENCE.fetch_add(1,Ordering::Relaxed);
        let staged=parent.join(format!(".{folder}.wmfw-staging-{}-{unique}",std::process::id()));
        let mut builder=fs::DirBuilder::new();
        #[cfg(unix)] {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        match builder.create(&staged) {
            Ok(())=>return Ok(StagedDirectory{path:staged,committed:false}),
            Err(e) if e.kind()==std::io::ErrorKind::AlreadyExists=>continue,
            Err(e)=>return Err(format!("creating private WMFW staging directory: {e}")),
        }
    }
    Err("could not allocate a unique private WMFW staging directory".into())
}

fn write_private_region_directory(output_dir:&Path,entries:&[(String,Vec<u8>)])->Result<(),String>{
    let mut stage=stage_new_private_directory(output_dir)?;
    for (filename,data) in entries {
        if filename.is_empty() || filename=="." || filename==".." ||
           filename.contains('/') || filename.contains('\\') {
            return Err(format!("invalid WMFW region filename: {filename}"));
        }
        let path=stage.path.join(filename);
        let mut opts=fs::OpenOptions::new();
        opts.write(true).create_new(true);
        #[cfg(unix)] {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        let mut file=opts.open(&path).map_err(|e|format!("creating private WMFW region {filename}: {e}"))?;
        file.write_all(data).map_err(|e|format!("writing private WMFW region {filename}: {e}"))?;
        file.sync_all().map_err(|e|format!("syncing private WMFW region {filename}: {e}"))?;
    }
    // Refuse an existing destination before the commit. The staged folder is
    // in the same parent filesystem, making the normal rename atomic. As with
    // any pathname-based operation, callers must protect untrusted parents
    // from concurrent modification by other users.
    if fs::symlink_metadata(output_dir).is_ok() {
        return Err(format!("WMFW output directory appeared during extraction: {}",output_dir.display()));
    }
    fs::rename(&stage.path,output_dir)
        .map_err(|e|format!("committing private WMFW payload directory: {e}"))?;
    stage.committed=true;
    Ok(())
}

/// Opt-in extraction of reference payloads into a newly created private
/// directory. All seven SHA-pinned payloads are verified before touching the
/// destination; failed extractions leave no partial proprietary blob files.
/// The folder must not exist, and it is committed only after all files sync.
pub fn extract_from_private_reference(reference:&[u8],output_dir:&Path)->Result<(),String>{
    if wmfw_sha256::hex_digest(reference)!=REFERENCE_SHA256 {
        return Err("reference WMFW does not match this exact revision".into());
    }
    wmfw_plan::verify_container(reference)?;
    let mut inputs=Vec::with_capacity(OPAQUE_REGIONS.len());
    for item in OPAQUE_REGIONS {
        let section=reference.get(item.file_offset..item.file_offset+item.size)
            .ok_or("opaque reference section exceeds bounds")?;
        if wmfw_sha256::hex_digest(section)!=item.expected_sha256 {
            return Err(format!("reference region {} checksum does not match manifest",item.filename()));
        }
        inputs.push((item.filename(),section.to_vec()));
    }
    write_private_region_directory(output_dir,&inputs)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]
    fn external_manifest_has_seven_nonoverlapping_sections() {
        assert_eq!(OPAQUE_REGIONS.iter().map(|x|x.size).sum::<usize>(),EXTERNAL_SOURCE_BYTES);
        let regions=wmfw_plan::regions().unwrap();
        let memory:Vec<_>=regions.iter().filter(|r|matches!(r.kind,0x10..=0x12)).collect();
        for (item,region) in OPAQUE_REGIONS.iter().zip(memory) {
            assert_eq!((item.file_offset,item.size,item.kind),
                (region.payload_offset,region.payload_length,region.kind));
            assert_eq!(item.expected_sha256.len(),64);
        }
        assert_eq!(EXTERNAL_SOURCE_BYTES+wmfw_plan::INDEPENDENT_METADATA_BYTES,wmfw_plan::FILE_SIZE);
    }
    #[test]
    fn no_opaque_region_may_be_missing_or_silently_zero_filled() {
        let err=build_with_external_payloads(|_item|Err("missing independently authored DSP input".into())).unwrap_err();
        assert!(err.contains("missing"));
        let err=build_with_external_payloads(|item| Ok(vec![0;item.size])).unwrap_err();
        assert!(err.contains("SHA-256"));
        let err=build_with_external_payloads(|item|Ok(vec![0;item.size-1])).unwrap_err();
        assert!(err.contains("length"));
    }
    #[test]
    fn staged_extraction_commits_atomically_and_preserves_existing_files() {
        let mut base=std::env::temp_dir();
        let unique=STAGING_SEQUENCE.fetch_add(1,Ordering::Relaxed);
        base.push(format!("agent08-extract-test-{}-{unique}",std::process::id()));
        fs::create_dir(&base).unwrap();
        let target=base.join("private-payloads");
        let entries=[("first.bin".to_owned(),vec![1,2,3]),
                     ("second.bin".to_owned(),vec![4,5,6])];
        write_private_region_directory(&target,&entries).unwrap();
        assert_eq!(fs::read(target.join("first.bin")).unwrap(),[1,2,3]);
        assert_eq!(fs::read(target.join("second.bin")).unwrap(),[4,5,6]);
        assert!(write_private_region_directory(&target,&entries).unwrap_err().contains("already exists"));
        assert_eq!(fs::read(target.join("second.bin")).unwrap(),[4,5,6]);
        #[cfg(unix)] {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(fs::metadata(&target).unwrap().permissions().mode() & 0o777,0o700);
            assert_eq!(fs::metadata(target.join("first.bin")).unwrap().permissions().mode() & 0o777,0o600);
        }
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn failed_extraction_leaves_no_partial_private_files() {
        let unique=STAGING_SEQUENCE.fetch_add(1,Ordering::Relaxed);
        let base=std::env::temp_dir().join(format!("agent08-extract-rollback-{}-{unique}",std::process::id()));
        fs::create_dir(&base).unwrap();
        let target=base.join("private-payloads");
        let entries=[("first.bin".to_owned(),vec![1,2,3]),
                     ("invalid/nested.bin".to_owned(),vec![4,5,6])];
        assert!(write_private_region_directory(&target,&entries).is_err());
        assert!(!target.exists());
        assert_eq!(fs::read_dir(&base).unwrap().count(),0);
        #[cfg(unix)] {
            use std::os::unix::fs::symlink;
            let symlink_path=base.join("destination-link");
            symlink(&base,&symlink_path).unwrap();
            assert!(write_private_region_directory(&symlink_path,&entries[..1]).unwrap_err().contains("already exists"));
            assert!(symlink_path.is_symlink());
        }
        fs::remove_dir_all(base).unwrap();
    }

}
