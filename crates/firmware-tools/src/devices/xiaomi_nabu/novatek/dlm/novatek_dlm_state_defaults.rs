// SPDX-License-Identifier: GPL-3.0-only
//! Reset-compatible controller state defaults from reconstructed upper ILM.
//!
//! The separately reconstructed Agent 18 NDS32 upper-ILM routines explicitly
//! store -1 to two GP-relative frame counters (file runtime 0x14e16/0x14e18
//! and 0x155e2/0x155e4), store 1 to a monitor flag (0x1570a/0x15738),
//! and store a signed -1 byte to a status sentinel (0x16dee/0x16df8).
//! These *independent program operations* establish the reset values and
//! destinations. In the pinned Novatek image, those same initial DLM fields
//! already contain exactly the reset values. Thus these 13 initial bytes can
//! be generated as typed controller state defaults without reading OEM bytes.
//!
//! Crucially this does NOT claim that all adjacent control words or the
//! unrelated counter at SRAM 0x2c184 are independently reconstructed: even
//! though that counter happens to start at -1, a provenance-safe derivation
//! of its initial value is not established here.

use super::novatek_dlm::DataFragment;
use super::novatek_layout::{DLM_FILE_START,DLM_FILE_END};

pub const POSTMARKETOS_GP: i64 = 0x2d9e0;
pub const FILE_MINUS_SRAM: i64 = 0x2000;

/// GP-displacements from the reconstructed NDS32 upper-ILM source. Keeping
/// signed displacements (not absolute observed firmware offsets) documents
/// the actual pointer arithmetic performed by the independent source.
pub const FIRST_COUNTER_DISP:i64=-6244;
pub const SECOND_COUNTER_DISP:i64=-6240;
pub const MONITOR_ENABLED_DISP:i64=-6232;
pub const CHANNEL_SENTINEL_DISP:i64=-6228;

pub fn dlm_file_for_gp(displacement:i64, data_bytes:usize)->Result<usize,String>{
    let sram=POSTMARKETOS_GP.checked_add(displacement)
        .ok_or("GP-relative displacement overflow")?;
    let file=sram.checked_add(FILE_MINUS_SRAM)
        .ok_or("SRAM-to-file mapping overflow")?;
    let addr=usize::try_from(file)
        .map_err(|_|"negative DLM file address")?;
    if addr<DLM_FILE_START || addr.checked_add(data_bytes)
        .filter(|end|*end<=DLM_FILE_END).is_none(){
        return Err(format!("GP-relative state initializer at file 0x{addr:x} outside DLM"));
    }
    Ok(addr)
}

pub fn fragments()->Result<Vec<DataFragment>,String>{
    let mut out=vec![
        DataFragment {
            name:"invalid-frame-counter-initializer",
            file_start:dlm_file_for_gp(FIRST_COUNTER_DISP,4)?,
            bytes:(-1_i32).to_le_bytes().to_vec(),
            evidence:"Agent18 ILM 0x155e2 movi -1; 0x155e4 swi.gp -6244: identical reset-compatible DLM initializer",
        },
        DataFragment {
            name:"invalid-input-counter-initializer",
            file_start:dlm_file_for_gp(SECOND_COUNTER_DISP,4)?,
            bytes:(-1_i32).to_le_bytes().to_vec(),
            evidence:"Agent18 ILM 0x14e16 movi -1; 0x14e18 swi.gp -6240: identical reset-compatible DLM initializer",
        },
        DataFragment {
            name:"monitor-enabled-initializer",
            file_start:dlm_file_for_gp(MONITOR_ENABLED_DISP,4)?,
            bytes:1_u32.to_le_bytes().to_vec(),
            evidence:"Agent18 ILM 0x1570a movi 1; 0x15738 swi.gp -6232: identical reset-compatible DLM initializer",
        },
        DataFragment {
            name:"channel-status-invalid-sentinel",
            file_start:dlm_file_for_gp(CHANNEL_SENTINEL_DISP,1)?,
            bytes:vec![u8::MAX],
            evidence:"Agent18 ILM 0x16dee movi -1; 0x16df8 sbi.gp -6228: identical reset-compatible byte initializer",
        },
    ];
    out.sort_by_key(|x|x.file_start);
    Ok(out)
}

#[cfg(test)]mod tests{
    use super::*;

    #[test]
    fn initializers_map_from_signed_gp_displacements(){
        let f=fragments().unwrap();
        assert_eq!(f.iter().map(|x|x.file_start).collect::<Vec<_>>(),
                   [0x2e17c,0x2e180,0x2e188,0x2e18c]);
        assert_eq!(f.iter().map(|x|x.bytes.len()).sum::<usize>(),13);
        assert_eq!(f[0].bytes,0xffff_ffffu32.to_le_bytes());
        assert_eq!(f[1].bytes,0xffff_ffffu32.to_le_bytes());
        assert_eq!(f[2].bytes,1_u32.to_le_bytes());
        assert_eq!(f[3].bytes,[255]);
    }

    #[test]
    fn does_not_claim_unexplained_adjacent_counter_or_flags(){
        let f=fragments().unwrap();
        assert_eq!(f[1].file_end(),0x2e184);
        assert_eq!(f[2].file_start,0x2e188);
        assert_eq!(f[3].file_end(),0x2e18d);
        assert_eq!(dlm_file_for_gp(-1_000_000,4).is_err(),true);
        assert_eq!(dlm_file_for_gp(SECOND_COUNTER_DISP,0x100).is_err(),true);
        assert_eq!(dlm_file_for_gp(FIRST_COUNTER_DISP,usize::MAX).is_err(),true);
    }
}
