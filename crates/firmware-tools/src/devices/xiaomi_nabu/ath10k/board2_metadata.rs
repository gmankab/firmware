// SPDX-License-Identifier: GPL-3.0-only
//! Independently encoded TLV framing for all 18 WCN3990 board records.
//! Calibration payloads are proprietary and NOT rebuilt; these are sparse
//! framing spans rather than a complete/loadable board file.

pub const IMAGE_BYTES: usize = 461_060;
pub const FRAMING_BYTES: usize = 1_508;
pub const PREFIX_BYTES: usize = 68;
const BOARDS: &[(&str, u32)] = &[
    ("bus=snoc,qmi-board-id=67", 26328),
    ("bus=snoc,qmi-board-id=ff", 26328),
    ("bus=snoc,qmi-board-id=67,qmi-chip-id=4320,variant=GO_POMPOM", 26328),
    ("bus=snoc,qmi-board-id=67,qmi-chip-id=320,variant=GO_POMPOM", 26328),
    ("bus=snoc,qmi-board-id=67,qmi-chip-id=4320,variant=GO_LAZOR", 26328),
    ("bus=snoc,qmi-board-id=67,qmi-chip-id=320,variant=GO_LAZOR", 26328),
    ("bus=snoc,qmi-board-id=67,qmi-chip-id=320,variant=GO_QUACKINGSTICK", 26328),
    ("bus=snoc,qmi-board-id=67,qmi-chip-id=4320,variant=GO_QUACKINGSTICK", 26328),
    ("bus=snoc,qmi-board-id=67,qmi-chip-id=320,variant=GO_QUACKINGSTICK_LTE", 26328),
    ("bus=snoc,qmi-board-id=67,qmi-chip-id=4320,variant=GO_QUACKINGSTICK_LTE", 26328),
    ("bus=snoc,qmi-board-id=ff,qmi-chip-id=4320,variant=GO_HOMESTAR", 26328),
    ("bus=snoc,qmi-board-id=ff,qmi-chip-id=320,variant=GO_HOMESTAR", 26328),
    ("bus=snoc,qmi-board-id=ff,qmi-chip-id=4320,variant=GO_WORMDINGLER", 26328),
    ("bus=snoc,qmi-board-id=ff,qmi-chip-id=320,variant=GO_WORMDINGLER", 26328),
    ("bus=snoc,qmi-board-id=ff,qmi-chip-id=4320,variant=GO_MRBLAND", 26328),
    ("bus=snoc,qmi-board-id=ff,qmi-chip-id=320,variant=GO_MRBLAND", 26328),
    ("bus=snoc,qmi-board-id=ff,qmi-chip-id=30214,variant=Thundercomm_DB845C", 19152),
    ("bus=snoc,qmi-board-id=ff,qmi-chip-id=30214", 19152),
];

pub struct HeaderSpan {
    pub offset: usize,
    pub data: Vec<u8>,
}
fn field(out: &mut Vec<u8>, kind: u32, size: u32) {
    out.extend_from_slice(&kind.to_le_bytes());
    out.extend_from_slice(&size.to_le_bytes());
}
fn board_header(name: &str, payload_len: u32) -> Vec<u8> {
    let n=name.as_bytes();
    let aligned=(n.len()+3)&!3;
    let mut out=Vec::with_capacity(24+aligned);
    field(&mut out, 0, (8+aligned+8) as u32+payload_len);
    field(&mut out, 0, n.len() as u32);
    out.extend_from_slice(n);
    out.resize(out.len()+aligned-n.len(), b'm');
    field(&mut out, 1, payload_len);
    out
}
/// Build only metadata spans, skipping every opaque calibration byte.
pub fn framing_spans() -> Vec<HeaderSpan> {
    let mut spans=Vec::with_capacity(BOARDS.len()+1);
    spans.push(HeaderSpan {offset:0, data:b"QCA-ATH10K-BOARD\0mmm".to_vec()});
    let mut cursor=20;
    for &(name,size) in BOARDS {
        let header=board_header(name,size);
        let length=header.len();
        spans.push(HeaderSpan {offset:cursor, data:header});
        cursor+=length+size as usize;
    }
    assert_eq!(cursor,IMAGE_BYTES);
    assert_eq!(spans.iter().map(|s|s.data.len()).sum::<usize>(),FRAMING_BYTES);
    spans
}
/// A 68-byte prefix (only initial framing, not calibration).
pub fn first_prefix() -> Vec<u8> {
    let spans=framing_spans();
    let mut out=spans[0].data.clone();
    out.extend_from_slice(&spans[1].data);
    assert_eq!(out.len(),PREFIX_BYTES);
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn framing_has_exact_nonoverlapping_layout() {
        let spans=framing_spans();
        assert_eq!(spans.len(),19);
        assert_eq!(spans[0].offset,0);
        assert_eq!(spans[1].offset,20);
        assert_eq!(spans[2].offset,0x671c);
        assert_eq!(spans.last().unwrap().offset,0x6bdf0);
        assert_eq!(spans.iter().map(|s|s.data.len()).sum::<usize>(),1508);
        assert_eq!(first_prefix().len(),68);
        for i in 1..spans.len()-1 {
            assert_eq!(spans[i].offset+spans[i].data.len()+BOARDS[i-1].1 as usize,
                       spans[i+1].offset);
        }
    }
}
