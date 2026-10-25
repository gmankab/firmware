// SPDX-License-Identifier: GPL-3.0-only
//! Two reference-confirmed, explicitly generated all-zero intervals in the
//! packed Y-memory image of this exact Cirrus CS35L41 WMFW revision.
//!
//! This is a strict *source-file compaction* format, not HALO executable
//! decompilation. The positions of zero-valued words were learned by
//! inspecting the original. Firmware is rebuilt from source-authored WMFW
//! metadata + a declarative zero-run map + hash-pinned opaque inputs.
//!
//! The earlier single-run importer in wmfw_sparse_ym remains supported.
use crate::{wmfw_external_payload,wmfw_plan,wmfw_sha256,wmfw_sparse_ym};
use std::{fs,path::Path};

pub const ZERO_RUNS:[(usize,usize);2]=[(15,116),(223,2038)];
pub const ZERO_WORDS:usize=101+1815;
pub const GENERATED_ZERO_BYTES:usize=ZERO_WORDS*3;
pub const EXTERNAL_SOURCE_BYTES:usize=wmfw_external_payload::EXTERNAL_SOURCE_BYTES-GENERATED_ZERO_BYTES;
const YM_START:usize=wmfw_sparse_ym::YM_FILE_START;
const YM_LENGTH:usize=wmfw_sparse_ym::YM_WORD_COUNT*3;
const YM_PARTS:[YPart;3]=[
    YPart { name:"region-0f90-head.bin",byte_offset:0,length:45,
        sha256:"8114718468f0f5cbea4e82347d86d57391f5bfe729e50d35af8e171b048b687b" },
    YPart { name:"region-0f90-middle.bin",byte_offset:348,length:321,
        sha256:"26a9d0ff548e3bd433d6916a19033e26fe0b0aa08b17ca60e4fc8b1b7a22f921" },
    YPart { name:"region-0f90-tail.bin",byte_offset:6114,length:18,
        sha256:"9a2123ce83ba0651e0672c7ce6fe5d992ced8ef9d71e101a9b580faf354c20d7" },
];

#[derive(Debug,Clone,Copy)]
pub struct YPart {
    pub name:&'static str,
    pub byte_offset:usize,
    pub length:usize,
    pub sha256:&'static str,
}

fn check_layout()->Result<(),String> {
    let zero_word_count=ZERO_RUNS.iter().try_fold(0usize,|a,(from,to)|
        if from>=to || *to>wmfw_sparse_ym::YM_WORD_COUNT { Err("bad zero run bounds") }
        else {a.checked_add(to-from).ok_or("zero count overflow")} )?;
    if zero_word_count != ZERO_WORDS {return Err("wrong total zero count".into());}
    let mut fragments=Vec::new();
    for &(a,b) in &ZERO_RUNS {
        fragments.push((3*a,3*b));
    }
    for part in YM_PARTS {
        if part.byte_offset%3!=0 || part.length%3!=0 ||
            part.byte_offset.checked_add(part.length).ok_or("Y extent overflow")?>YM_LENGTH {
            return Err("invalid sparse-Y fragment extent".into());
        }
        fragments.push((part.byte_offset,part.byte_offset+part.length));
    }
    fragments.sort_unstable();
    let mut cursor=0;
    for (from,to) in fragments {
        if from!=cursor {return Err(format!("sparse-Y overlap/hole at {cursor}..{from}"));}
        cursor=to;
    }
    if cursor!=YM_LENGTH {return Err("sparse-Y ranges do not cover YM".into());}
    let regions=wmfw_plan::regions()?;
    let ym=regions.iter().find(|r|r.kind==0x12).ok_or("Y region is absent")?;
    if ym.payload_offset!=YM_START || ym.payload_length!=YM_LENGTH ||
        wmfw_external_payload::OPAQUE_REGIONS.iter().filter(|r|r.kind==0x12).count()!=1 {
        return Err("WMFW Y region differs from compact-Y schema".into());
    }
    Ok(())
}

fn check_part(part:YPart,payload:&[u8])->Result<(),String>{
    if payload.len()!=part.length {
        return Err(format!("{} size {} != {}",part.name,payload.len(),part.length));
    }
    if wmfw_sha256::hex_digest(payload)!=part.sha256 {
        return Err(format!("{} SHA-256 mismatch",part.name));
    }
    Ok(())
}

pub fn build_with_compact_y<F,Y>(mut read_other:F,mut read_y:Y)->Result<Vec<u8>,String>
where F:FnMut(wmfw_external_payload::OpaqueRegion)->Result<Vec<u8>,String>,
      Y:FnMut(YPart)->Result<Vec<u8>,String> {
    check_layout()?;
    let mut plan=wmfw_plan::partial_container()?;
    let mut imported=0usize;
    for other in wmfw_external_payload::OPAQUE_REGIONS {
        if other.kind==0x12 {continue;}
        let data=read_other(other)?;
        if data.len()!=other.size {return Err(format!("{} unexpected size",other.filename()));}
        if wmfw_sha256::hex_digest(&data)!=other.expected_sha256 {
            return Err(format!("{} SHA-256 mismatch",other.filename()));
        }
        plan.place(&format!("external opaque {}",other.filename()),other.file_offset,&data)?;
        imported+=data.len();
    }
    for part in YM_PARTS {
        let bytes=read_y(part)?;
        check_part(part,&bytes)?;
        plan.place(&format!("external packed Y {}",part.name),YM_START+part.byte_offset,&bytes)?;
        imported+=bytes.len();
    }
    if imported!=EXTERNAL_SOURCE_BYTES {return Err("compact-Y external input accounting mismatch".into());}
    for (a,b) in ZERO_RUNS {
        let offset=YM_START+a*3;
        let size=(b-a)*3;
        plan.place(&format!("reference-verified zero packed Y word range {a}..{b}"),offset,&vec![0;size])?;
    }
    let out=plan.build_complete()?;
    if wmfw_sha256::hex_digest(&out)!=wmfw_external_payload::REFERENCE_SHA256 {
        return Err("compact-Y WMFW full-file SHA-256 mismatch".into());
    }
    Ok(out)
}

pub fn assemble_directory(dir:&Path)->Result<Vec<u8>,String> {
    build_with_compact_y(
        |item|fs::read(dir.join(item.filename())).map_err(|e|format!("{}: {e}",item.filename())),
        |item|fs::read(dir.join(item.name)).map_err(|e|format!("{}: {e}",item.name)),
    )
}

/// Extract only opaque external bytes, atomically and privately. Independent
/// zero runs are NOT read into new output files.
pub fn extract_inputs(reference:&[u8],dir:&Path)->Result<(),String> {
    check_layout()?;
    if wmfw_sha256::hex_digest(reference)!=wmfw_external_payload::REFERENCE_SHA256 {
        return Err("compact-Y source reference SHA-256 mismatch".into());
    }
    wmfw_plan::verify_container(reference)?;
    let mut external=Vec::with_capacity(9);
    for other in wmfw_external_payload::OPAQUE_REGIONS {
        if other.kind==0x12 {continue;}
        let data=reference.get(other.file_offset..other.file_offset+other.size)
            .ok_or("truncated opaque region")?;
        if wmfw_sha256::hex_digest(data)!=other.expected_sha256 {
            return Err(format!("{} reference hash mismatch",other.filename()));
        }
        external.push((other.filename(),data.to_vec()));
    }
    for part in YM_PARTS {
        let data=reference.get(YM_START+part.byte_offset..YM_START+part.byte_offset+part.length)
            .ok_or("truncated Y fragment")?;
        check_part(part,data)?;
        external.push((part.name.to_string(),data.to_vec()));
    }
    for (a,b) in ZERO_RUNS {
        let offset=YM_START+a*3;
        let bytes=reference.get(offset..offset+(b-a)*3).ok_or("truncated Y zero run")?;
        if bytes.iter().any(|v|*v!=0) {return Err("nonzero byte in declared Y zero run".into());}
    }
    wmfw_external_payload::write_private_region_directory(dir,&external)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_y_layout_covers_every_byte_exactly_once() {
        check_layout().unwrap();
        assert_eq!(ZERO_WORDS,1916);
        assert_eq!(GENERATED_ZERO_BYTES,5748);
        assert_eq!(EXTERNAL_SOURCE_BYTES,24504);
        assert_eq!(YM_PARTS.iter().map(|x|x.length).sum::<usize>(),384);
        assert_eq!(0xf90+45,0xfbd);
        assert_eq!(0xf90+348,0x10ec);
        assert_eq!(0xf90+669,0x122d);
        assert_eq!(0xf90+6114,0x2772);
    }
    #[test]
    fn refuses_missing_or_untrusted_external_inputs() {
        let e=build_with_compact_y(|_|Err("missing".into()), |_|Err("missing".into())).unwrap_err();
        assert!(e.contains("missing"));
        let e=build_with_compact_y(|x|Ok(vec![0;x.size]), |_|Ok(vec![])).unwrap_err();
        assert!(e.contains("SHA-256"));
        assert!(check_part(YM_PARTS[0],&[0]).unwrap_err().contains("size"));
        assert!(check_part(YM_PARTS[1],&vec![0;YM_PARTS[1].length]).unwrap_err().contains("SHA-256"));
    }
}
