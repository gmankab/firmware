// SPDX-License-Identifier: GPL-3.0-only
//! Qualcomm WCN3990 Bluetooth patch/NVM metadata reconstruction.
//! TLV framing and numeric fields are typed source, but Bluetooth executable
//! patch bytes and all 61 NVM tag values are NOT rebuilt.

pub const PATCH_BYTES: usize = 127_544;
pub const NVM_BYTES: usize = 5_504;
pub const PATCH_METADATA_BYTES: usize = 28;
pub const NVM_METADATA_BYTES: usize = 744;

pub struct Span { pub offset: usize, pub data: Vec<u8> }

const TAGS_0: &[(u16,u16)] = &[
    (2, 6), (8, 8), (17, 6), (22, 104),
    (27, 9), (31, 2), (32, 2), (35, 68),
    (36, 16), (37, 44), (38, 80), (42, 50),
    (44, 64), (50, 222), (51, 68), (56, 3),
    (62, 48), (64, 192), (65, 44), (72, 65),
    (78, 2), (80, 16), (83, 8), (84, 112),
    (85, 124), (86, 16), (87, 80), (88, 144),
    (89, 196), (90, 8), (91, 192), (92, 64),
    (95, 132), (111, 204), (112, 232), (114, 132),
    (151, 18), (153, 98), (154, 252), (161, 116),
    (176, 236), (177, 220), (178, 28), (180, 68),
    (181, 36), (200, 28),
];
const TAGS_1: &[(u16,u16)] = &[
    (22, 38), (30, 244), (31, 5), (37, 37),
    (38, 6), (40, 14), (42, 20), (44, 58),
    (46, 59), (48, 14), (68, 20), (69, 182),
    (72, 9), (74, 63), (76, 128),
];

fn outer(kind:u8,size:usize)->[u8;4] {
    assert!(size<=0x00ff_ffff);
    (((size as u32)<<8)|kind as u32).to_le_bytes()
}
fn tag(id:u16,size:u16)->Vec<u8> {
    let mut data=Vec::with_capacity(12);
    data.extend_from_slice(&id.to_le_bytes());
    data.extend_from_slice(&size.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data
}

pub fn patch_prefix()->Vec<u8> {
    let mut out=Vec::with_capacity(PATCH_METADATA_BYTES);
    out.extend_from_slice(&outer(1,PATCH_BYTES-4));
    out.extend_from_slice(&127_508u32.to_le_bytes()); // total_size
    out.extend_from_slice(&127_472u32.to_le_bytes()); // data_length
    out.extend_from_slice(&[1,0,3,0]); // format, signature, mode, padding
    out.extend_from_slice(&10u16.to_le_bytes()); // product ID
    out.extend_from_slice(&0x0302u16.to_le_bytes()); // ROM build
    out.extend_from_slice(&0x373fu16.to_le_bytes()); // patch version
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // patch entry
    assert_eq!(out.len(),PATCH_METADATA_BYTES);
    out
}

pub fn nvm_spans()->Vec<Span> {
    let sets=[(2,TAGS_0),(3,TAGS_1)];
    let total_body:usize=sets.iter().map(|(_,tags)|
        4+tags.iter().map(|(_,size)|12+*size as usize).sum::<usize>()).sum();
    let mut spans=Vec::with_capacity(64);
    spans.push(Span {offset:0,data:outer(4,total_body).to_vec()});
    let mut cursor=4;
    for (kind,tags) in sets {
        let set_body:usize=tags.iter().map(|(_,size)|12+*size as usize).sum();
        spans.push(Span {offset:cursor,data:outer(kind,set_body).to_vec()});
        cursor+=4;
        for &(id,size) in tags {
            spans.push(Span {offset:cursor,data:tag(id,size)});
            cursor+=12+size as usize; // skip NVM values
        }
    }
    assert_eq!(cursor,NVM_BYTES);
    assert_eq!(spans.len(),64);
    assert_eq!(spans.iter().map(|s|s.data.len()).sum::<usize>(),NVM_METADATA_BYTES);
    spans
}

pub fn nvm_prefix()->Vec<u8> {
    let spans=nvm_spans();
    let mut out=Vec::with_capacity(20);
    for span in &spans[..3] { out.extend_from_slice(&span.data); }
    assert_eq!(out.len(),20);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_sparse_nvm_headers_skip_values() {
        let spans=nvm_spans();
        assert_eq!(spans.len(),64);
        assert_eq!(spans[0].offset,0);
        assert_eq!(spans[1].offset,4);
        assert_eq!(spans[2].offset,8);
        assert_eq!(spans.iter().map(|s|s.data.len()).sum::<usize>(),744);
        assert_eq!(nvm_prefix().len(),20);
    }
    #[test]
    fn generated_patch_prefix_is_header_only() {
        let bytes=patch_prefix();
        assert_eq!(bytes.len(),28);
        assert_eq!(bytes[0],1);
        assert_eq!(&bytes[20..22],&0x373fu16.to_le_bytes());
    }
}
