// SPDX-License-Identifier: GPL-3.0-only
//! Typed modem bootstrap configuration records and linker-alignment padding.
//!
//! This section is **DATA**, not a stream of Hexagon instructions. It begins
//! immediately after the reset-vector/bootstrap code, at ELF file `0x3044`,
//! and ends before the next code island at `0x3500`. It describes mappings,
//! the WLAN process label, source/destination memory copies, and two device
//! configuration records. Record type semantics remain partly provisional;
//! lengths, locations, and encoding are checked independently.

pub const FILE_START: usize = 0x3044;
pub const CODE_RESUMES: usize = 0x3500;
pub const LENGTH: usize = CODE_RESUMES - FILE_START;
pub const UNDEFINED_PADDING: u8 = 0x7f;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RecordType {
    MemoryMap = 3,
    SmallOption = 6,
    ProcessName = 16,
    TransferRegion = 17,
    CopyRegion = 18,
    RegionReference = 19,
    BootControl = 21,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mapping {
    pub address: u32,
    pub physical: u32,
    pub length: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transfer {
    pub from: u32,
    pub to: u32,
    pub size: u32,
}

fn word(output: &mut Vec<u8>, value: u32) { output.extend_from_slice(&value.to_le_bytes()); }
fn header(output: &mut Vec<u8>, ty: RecordType, encoded_length: u16) -> Result<(), String> {
    if encoded_length < 8 || encoded_length % 4 != 0 {
        return Err("invalid length of modem boot record".to_owned());
    }
    word(output, ((ty as u32) << 16) | encoded_length as u32);
    Ok(())
}
fn map(output: &mut Vec<u8>, m: Mapping) -> Result<(), String> {
    header(output, RecordType::MemoryMap, 16)?;
    for x in [m.address, m.physical, m.length] { word(output, x); }
    Ok(())
}
fn transfer(output: &mut Vec<u8>, ty: RecordType, t: Transfer) -> Result<(), String> {
    header(output, ty, 16)?;
    for x in [t.from, t.to, t.size] { word(output, x); }
    Ok(())
}

pub fn rebuild() -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(LENGTH);
    // End-marker for the preceding bootstrap vectors.
    word(&mut out, 0);
    map(&mut out, Mapping { address: 0xc081_0000, physical: 0x0540_0000, length: 0x1_0000 })?;
    map(&mut out, Mapping { address: 0xc082_0000, physical: 0x0541_0000, length: 0x1_0000 })?;
    // Ten 16-KiB mappings of contiguous staging SRAM. The low bit of the
    // physical descriptor is a hardware attribute and must be preserved.
    for index in 0..10u32 {
        let page_offset = index * 0x4000;
        map(&mut out, Mapping { address: 0xc084_0000 + page_offset,
                                physical: 0xc378_0001 + page_offset,
                                length: if index == 9 { 0x1000 } else { 0x4000 } })?;
    }
    map(&mut out, Mapping { address: 0xc086_5000, physical: 0xc37a_5001, length: 0x1000 })?;
    for option in [0x000c_0001, 0x000f_0001, 0x0010_0001] {
        header(&mut out, RecordType::SmallOption, 8)?;
        word(&mut out, option);
    }
    header(&mut out, RecordType::ProcessName, 20)?;
    let mut label=[0u8;16];
    const PROCESS_NAME: &[u8]=b"wlan_process";
    label[..PROCESS_NAME.len()].copy_from_slice(PROCESS_NAME);
    out.extend_from_slice(&label);
    // Two sections with explicit execution/loading parameters.
    header(&mut out, RecordType::TransferRegion, 20)?;
    for x in [0x1e40_0000, 0x9000_0000, 0x1_0000, 5] { word(&mut out, x); }
    header(&mut out, RecordType::TransferRegion, 20)?;
    for x in [0x1e41_0000, 0x9001_0000, 0xc000, 3] { word(&mut out, x); }
    // Descriptors sourced from the 0x1e41xxxx address range. These are data
    // transfers; do not reinterpret them as signed executable instructions.
    for item in [
        Transfer {from:0x1e41_0000, to:0xa050_3000, size:0x1d9a},
        Transfer {from:0x1e41_1da0, to:0xa050_4da0, size:4},
        Transfer {from:0x1e41_3000, to:0xa050_4da8, size:0x310},
        Transfer {from:0x1e41_3310, to:0xa050_50b8, size:12},
        Transfer {from:0x1e41_331c, to:0xa050_50c4, size:12},
        Transfer {from:0x1e41_b000, to:0xa050_50d0, size:0x80},
    ] { transfer(&mut out, RecordType::CopyRegion, item)?; }
    header(&mut out, RecordType::RegionReference, 8)?;
    word(&mut out, 0x1e40_0000);
    header(&mut out, RecordType::BootControl, 8)?;
    word(&mut out, 0xff01_0002);
    // End of configuration data; the remainder is deliberate fixed linker
    // fill, not BSS and not a reconstruction of OEM executable code.
    word(&mut out, 0);
    let actual_data_end=FILE_START + out.len();
    if actual_data_end != 0x31e0 {
        return Err(format!("boot table layout ended at {actual_data_end:#x}; expected 0x31e0"));
    }
    out.resize(LENGTH, UNDEFINED_PADDING);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manifest_has_exact_record_and_fill_boundaries() {
        let manifest=rebuild().unwrap();
        assert_eq!(manifest.len(), 0x4bc);
        assert_eq!(FILE_START+manifest.len(),CODE_RESUMES);
        assert_eq!(&manifest[(0x31e0-FILE_START)..],vec![0x7f;0x320].as_slice());
    }
    #[test]
    fn number_of_map_records_and_wlan_name() {
        let manifest=rebuild().unwrap();
        assert_eq!(&manifest[4..8], &0x0003_0010u32.to_le_bytes());
        assert_eq!(&manifest[(0x3134-FILE_START)..(0x3140-FILE_START)], b"wlan_process");
        assert_eq!(&manifest[(0x31d8-FILE_START)..(0x31dc-FILE_START)],&0xff01_0002u32.to_le_bytes());
    }
}
