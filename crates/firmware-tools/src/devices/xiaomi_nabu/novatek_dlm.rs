// SPDX-License-Identifier: GPL-3.0-only
//! Independently structured Novatek DLM data, starting with packed index walks.
//!
//! Each observed record represents 50 unsigned 11-bit indices: 50 low bytes,
//! two alignment zeros, then five little-endian words containing ten 3-bit
//! high fields each. The top two bits of each word are zero. 0x7ff is the
//! all-ones sentinel. These are mathematical data recipes, not instructions.
//! The firmware's physical interpretation of the indices is not established.
//! Unknown configuration and apparent zero records outside the explicit spans
//! below remain unimplemented; they are never copied from the reference.

pub const SLOTS: usize = 50;
pub const RECORD_BYTES: usize = 72;
pub const SENTINEL: u16 = 0x7ff;
pub const INITIAL_WALK_START: usize = 0x24904;
pub const INITIAL_WALK_END: usize = 0x24fc4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFragment {
    pub name: &'static str,
    pub file_start: usize,
    pub bytes: Vec<u8>,
    pub evidence: &'static str,
}

impl DataFragment {
    pub fn file_end(&self) -> usize { self.file_start + self.bytes.len() }
    pub fn sram_start(&self) -> usize { self.file_start - 0x2000 }
}

/// Encode a bounded 50-entry semantic index record. No firmware input needed.
pub fn encode_record(slots: &[u16]) -> Result<[u8; RECORD_BYTES], String> {
    if slots.len() != SLOTS { return Err("index record requires exactly 50 slots".into()); }
    let mut out = [0; RECORD_BYTES];
    let mut high = [0u32; 5];
    for (i, &index) in slots.iter().enumerate() {
        if index > SENTINEL { return Err(format!("index {index} exceeds 11 bits at slot {i}")); }
        out[i] = index as u8;
        high[i / 10] |= u32::from(index >> 8) << (3 * (i % 10));
    }
    for (i, word) in high.into_iter().enumerate() {
        out[52 + 4*i..56 + 4*i].copy_from_slice(&word.to_le_bytes());
    }
    Ok(out)
}

/// Checked format decoder for inspection; not a source-generation fallback.
pub fn decode_record(bytes: &[u8]) -> Result<[u16; SLOTS], String> {
    if bytes.len() != RECORD_BYTES { return Err("packed index record must be 72 bytes".into()); }
    if bytes[50..52] != [0, 0] { return Err("nonzero packed index alignment bytes".into()); }
    let mut out = [0; SLOTS];
    for group in 0..5 {
        let offset = 52 + group*4;
        let word = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
        if word >> 30 != 0 { return Err("nonzero reserved bits in index high word".into()); }
        for i in 0..10 {
            out[10*group+i] = u16::from(bytes[10*group+i])
                | (((word >> (3*i)) & 7) as u16) << 8;
        }
    }
    Ok(out)
}

fn descending(first: i32, step: i32, count: usize) -> Result<Vec<u16>, String> {
    if step <= 0 || count > SLOTS { return Err("invalid descending index walk".into()); }
    (0..count).map(|i| {
        let value = first.checked_sub(step.checked_mul(i as i32).ok_or("walk overflow")?)
            .ok_or("walk overflow")?;
        if !(0..i32::from(SENTINEL)).contains(&value) { return Err("index walk exceeds live range".into()); }
        Ok(value as u16)
    }).collect()
}

fn modular_walk(first: i32, stride: i32, modulus: i32) -> Result<Vec<u16>, String> {
    if modulus <= 0 || modulus > i32::from(SENTINEL) || first < 0 || first >= modulus
        || stride <= 0 || stride >= modulus {
        return Err("invalid modular index walk".into());
    }
    Ok((0..SLOTS).map(|i| (first - stride*i as i32).rem_euclid(modulus) as u16).collect())
}

fn paired_rows(first_row: i32, reverse_pair: bool) -> Result<Vec<u16>, String> {
    let mut out = Vec::with_capacity(SLOTS);
    for row in descending(first_row, 2, SLOTS/2)? {
        for column in if reverse_pair { [1,0] } else { [0,1] } {
            out.push(row*32+column);
        }
    }
    Ok(out)
}

fn append(out: &mut Vec<u8>, slots: Vec<u16>) -> Result<(), String> {
    out.extend_from_slice(&encode_record(&slots)?);
    Ok(())
}

/// Twenty-four records generated from bounded linear, paired, modular and
/// transposed-grid walks. The wrapping ring has 48*32-1 = 1535 indices.
pub fn initial_index_walks() -> Result<DataFragment, String> {
    let mut out = Vec::with_capacity(INITIAL_WALK_END-INITIAL_WALK_START);
    append(&mut out, descending(49, 1, SLOTS)?)?;
    append(&mut out, descending(74, 1, SLOTS)?)?;
    for _ in 0..2 { append(&mut out, descending(49*32, 32, SLOTS)?)?; }
    append(&mut out, paired_rows(49, false)?)?;
    append(&mut out, paired_rows(48, true)?)?;
    append(&mut out, paired_rows(49, true)?)?;
    for phase in 0..4 {
        append(&mut out, modular_walk(3*32+3+phase*65, 32, 48*32-1)?)?;
        append(&mut out, modular_walk(41*32+12-phase*65, 32, 48*32-1)?)?;
    }
    append(&mut out, modular_walk(11*32+7, 32, 48*32-1)?)?;
    let mut lower = descending(33*32+8, 32, 34)?;
    lower.resize(SLOTS, SENTINEL);
    append(&mut out, lower)?;
    let mut upper = vec![SENTINEL; 14];
    upper.extend(descending(47*32+7, 32, 36)?);
    append(&mut out, upper)?;
    append(&mut out, descending(99, 1, SLOTS)?)?;
    append(&mut out, descending(49, 1, SLOTS)?)?;
    append(&mut out, vec![SENTINEL; SLOTS])?;
    let mut mask = vec![SENTINEL; 30];
    mask.resize(SLOTS, 0);
    append(&mut out, mask)?;
    let mut transpose = Vec::with_capacity(SLOTS);
    for column in (0..8).rev() {
        for row in (0..6).rev() { transpose.push(row*16+column); }
    }
    transpose.resize(SLOTS, SENTINEL);
    append(&mut out, transpose)?;
    append(&mut out, descending(98, 2, SLOTS)?)?;
    if out.len() != INITIAL_WALK_END-INITIAL_WALK_START {
        return Err("initial index schedule extent disagrees with source recipe".into());
    }
    Ok(DataFragment { name: "initial-index-walks", file_start: INITIAL_WALK_START,
        bytes: out, evidence: "50x11-bit records: linear, paired, modulo-1535 and 6x8 index walks" })
}

/// The second wide schedule translates live ring indices by half a 32-index
/// row. The scalar/paired maps and explicit sentinel masks stay identical.
pub fn translated_index_walks() -> Result<DataFragment, String> {
    let mut data = initial_index_walks()?;
    for record in data.bytes.chunks_exact_mut(RECORD_BYTES).skip(7).take(11) {
        let mut values = decode_record(record)?;
        for value in &mut values {
            if *value != SENTINEL { *value += 16; }
        }
        record.copy_from_slice(&encode_record(&values)?);
    }
    data.name = "translated-wide-index-walks";
    data.file_start = 0x28904;
    data.evidence = "wide index walks with 16-index translation of live ring slots";
    Ok(data)
}

/// Twenty records of 48-by-30 index walks. The second bank translates live
/// ring slots by 15 and uses two full stride-32 maps as its final records.
pub fn narrow_index_walks(translated: bool) -> Result<DataFragment, String> {
    let mut out = Vec::with_capacity(20*RECORD_BYTES);
    append(&mut out, descending(74,1,SLOTS)?)?;
    append(&mut out, descending(49,1,SLOTS)?)?;
    append(&mut out, descending(49*32,32,SLOTS)?)?;
    let translation = if translated {15} else {0};
    let mut straight = descending(47*30+translation,30,48)?;
    straight.resize(SLOTS,SENTINEL);
    append(&mut out,straight)?;
    for row in 4..18 {
        let lane = (row-4)/2;
        let origin = if row%2 == 0 {1363-61*lane} else {32+61*lane};
        let mut values = modular_walk(origin,30,48*30-1)?;
        for (i,value) in values.iter_mut().enumerate() {
            let disabled = (row==14 && (36..48).contains(&i))
                || (row==16 && i>=22) || (row==17 && !(14..26).contains(&i));
            *value = if disabled {SENTINEL} else {*value+translation as u16};
        }
        append(&mut out,values)?;
    }
    for row in 18..20 {
        let values = if translated {descending(49*32,32,SLOTS)?} else {
            let mut live = descending(if row==18 {95} else {47},1,48)?;
            live.resize(SLOTS,SENTINEL);
            live
        };
        append(&mut out,values)?;
    }
    if out.len()!=20*RECORD_BYTES {return Err("narrow index schedule extent mismatch".into());}
    Ok(DataFragment {
        name: if translated {"translated-narrow-index-walks"} else {"narrow-index-walks"},
        file_start: if translated {0x29d4c} else {0x25d4c}, bytes:out,
        evidence: if translated {"50x11-bit records: modulo-1439 walks translated by 15 with explicit sentinel masks"}
            else {"50x11-bit records: modulo-1439 walks with explicit sentinel masks"},
    })
}

/// Two independent mailboxes begin with the Nabu controller's idle token.
/// The two servicing paths return this identical token after consuming a
/// transaction; this is a semantic initializer, not a copied payload span.
/// Only the eight initialized bytes are claimed, not their adjacent fields.
pub const IDLE_MAILBOX_TOKEN: u32 = 0x1234_5678;
pub const MAILBOX_FILE_START: usize = 0x2e130;
pub fn idle_mailbox_initializers() -> DataFragment {
    let mut bytes = Vec::with_capacity(8);
    for _ in 0..2 { bytes.extend_from_slice(&IDLE_MAILBOX_TOKEN.to_le_bytes()); }
    DataFragment {name:"controller-idle-mailboxes",file_start:MAILBOX_FILE_START,bytes,
        evidence:"two NDS32 mailbox slots initialized to controller idle token 0x12345678"}
}

pub fn fragments() -> Result<Vec<DataFragment>, String> {
    Ok(vec![initial_index_walks()?,narrow_index_walks(false)?,
        translated_index_walks()?,narrow_index_walks(true)?, idle_mailbox_initializers()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitplanes_have_known_word_positions_and_reserved_bits() {
        let slots: Vec<u16> = (0..SLOTS).map(|i| ((i % 8) as u16)*256 + i as u16).collect();
        let encoded = encode_record(&slots).unwrap();
        assert_eq!(&encoded[..50], &(0u8..50).collect::<Vec<_>>());
        assert_eq!(&encoded[50..52], &[0,0]);
        // 0,1,2,3,4,5,6,7,0,1 packed in independent base-8 arithmetic.
        assert_eq!(u32::from_le_bytes(encoded[52..56].try_into().unwrap()),
                   8 + 2*64 + 3*512 + 4*4096 + 5*32768 + 6*262144 + 7*2097152 + 134217728);
        assert_eq!(decode_record(&encoded).unwrap().as_slice(), slots);
    }

    #[test]
    fn dual_idle_mailboxes_have_source_token_not_payload_copy() {
        let f=idle_mailbox_initializers();
        assert_eq!(f.file_start, 0x2e130);
        assert_eq!(f.bytes, vec![0x78,0x56,0x34,0x12,0x78,0x56,0x34,0x12]);
        assert_eq!(f.sram_start(), 0x2c130);
    }

    #[test]
    fn malformed_slots_and_reserved_bits_are_rejected() {
        assert!(encode_record(&[0;49]).is_err());
        assert!(encode_record(&[2048;50]).is_err());
        assert!(decode_record(&[0;71]).is_err());
        let mut raw = encode_record(&[SENTINEL;50]).unwrap();
        assert_eq!(&raw[52..56], &[0xff,0xff,0xff,0x3f]);
        raw[50] = 1;
        assert!(decode_record(&raw).is_err());
        raw[50] = 0;
        raw[55] |= 0x80;
        assert!(decode_record(&raw).is_err());
    }

    #[test]
    fn arithmetic_walks_reject_wraparound_and_distinguish_sentinels() {
        assert!(descending(1,2,3).is_err());
        assert!(descending(2047,1,1).is_err());
        assert!(descending(0,0,1).is_err());
        assert!(modular_walk(0,1,0).is_err());
        assert!(modular_walk(1535,32,1535).is_err());
        assert_eq!(modular_walk(99,32,1535).unwrap()[..5], [99,67,35,3,1506]);
    }

    #[test]
    fn initial_span_has_exact_record_boundaries_and_semantic_dimensions() {
        let data = initial_index_walks().unwrap();
        assert_eq!(data.bytes.len(),24*RECORD_BYTES);
        assert_eq!(data.file_end(),INITIAL_WALK_END);
        assert_eq!(data.sram_start(),0x22904);
        let records: Vec<_> = data.bytes.chunks_exact(RECORD_BYTES).map(|b|decode_record(b).unwrap()).collect();
        assert_eq!(records[16].iter().filter(|&&v|v==SENTINEL).count(),16);
        assert_eq!(records[17].iter().filter(|&&v|v==SENTINEL).count(),14);
        let mut transpose = records[22][..48].to_vec();
        transpose.sort_unstable(); transpose.dedup();
        assert_eq!(transpose.len(),48);
        assert!(transpose.iter().all(|&v|v/16<6 && v%16<8));
    }

    #[test]
    fn translated_banks_preserve_disable_masks_and_move_only_live_ring_slots() {
        let primary=initial_index_walks().unwrap();
        let translated=translated_index_walks().unwrap();
        for (row,(a,b)) in primary.bytes.chunks_exact(RECORD_BYTES)
            .zip(translated.bytes.chunks_exact(RECORD_BYTES)).enumerate() {
            for (left,right) in decode_record(a).unwrap().into_iter().zip(decode_record(b).unwrap()) {
                assert_eq!(right,if (7..18).contains(&row) && left!=SENTINEL {left+16} else {left});
            }
        }
        let a=narrow_index_walks(false).unwrap();
        let b=narrow_index_walks(true).unwrap();
        assert_eq!(a.file_end(),0x262ec);
        assert_eq!(b.file_end(),0x2a2ec);
        for row in [14,16,17] {
            let decode=|data:&DataFragment|decode_record(&data.bytes[row*72..(row+1)*72]).unwrap();
            assert_eq!(decode(&a).map(|v|v==SENTINEL),decode(&b).map(|v|v==SENTINEL));
        }
        assert_eq!(fragments().unwrap().iter().filter(|f|f.name.contains("index-walks")).map(|f|f.bytes.len()).sum::<usize>(),6336);
    }
}
