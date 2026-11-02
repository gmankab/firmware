// SPDX-License-Identifier: GPL-3.0-only
//! Structured upper-ILM lookup and profile tables.
//!
//! These are decoded metadata tables, not NDS32 instructions. Some samples
//! follow independently checkable integer/trigonometric constructions; the
//! mask basis, selected permutations, terminal tangent samples and small
//! calibration profiles are *observed constants* in the pinned OEM image.
//! Their original vendor names and generating algorithms are not recovered.
//! No original byte span is pasted into the emitter.
use super::novatek_upper_allocations as allocations;

pub const LOOKUPS_START: u32 = 0x196f8;
pub const LOOKUPS_END: u32 = 0x198dc;
pub const LOOKUP_BYTES: usize = (LOOKUPS_END-LOOKUPS_START) as usize;
pub const PROFILES_START: u32 = 0x19bbc;
pub const PROFILES_END: u32 = 0x19bf0;
pub const PROFILE_BYTES: usize = (PROFILES_END-PROFILES_START) as usize;
pub const TOTAL_BYTES: usize = LOOKUP_BYTES+PROFILE_BYTES;
const BITMASK_20: u32 = 0xfffff;

/// Reconstructed threshold curve: first 28 samples use floor(100*tan(3*i°)).
/// The exact 45° endpoint is rational 1, avoiding host libm drift. Three
/// values near the singularity are explicitly reference-decoded, not claimed
/// as results from a vendor's unknown high-angle approximation.
pub fn angular_thresholds() -> Result<[u16;31],String> {
    let mut result = [0_u16;31];
    for index in 0..28 {
        let value = if index==15 {100}
        else { (100.0_f64 * (3.0_f64 * index as f64).to_radians().tan()).floor() as u16 };
        result[index]=value;
    }
    result[28..].copy_from_slice(&[953,1913,5730]);
    if result.windows(2).any(|x|x[0]>=x[1]) {
        return Err("angular lookup must increase monotonically".into());
    }
    Ok(result)
}

/// The ordered channel identifiers form an XOR-affine 4x4 permutation.
/// Four bit-basis constants reproduce the sixteen unique byte entries.
pub fn channel_permutation() -> [u8;16] {
    let columns=[0_u8,9,11,2];
    let rows=[0_u8,15,7,8];
    let mut result=[0;16];
    for (row_i,mask) in rows.into_iter().enumerate() {
        for (column_i,code) in columns.into_iter().enumerate() {
            result[row_i*4+column_i]=mask^code;
        }
    }
    result
}

/// Eight observed, unequal 20-bit mask seeds. Their complements, repetitions
/// and row-wise permutations reconstruct the full 72-entry table. The seeds
/// remain revision-specific metadata, not independently invented firmware.
const MASK_SEEDS:[u32;8]=[
    0x06cf5,0x1b3d4,0x21b3d,0x286cf,0x33d43,0x4f50d,0x6a1b3,0x6cf50,
];
const MASK_BASE_ORDER:[usize;8]=[5,4,1,7,6,3,0,2];
const MASK_BLOCK_ROWS:[[usize;8];4]=[
    [1,4,6,2,0,3,5,7],
    [5,7,0,3,6,2,1,4],
    [2,6,4,1,7,5,3,0],
    [3,0,7,5,4,1,2,6],
];

fn mask(index:usize,complement:bool)->Result<u32,String>{
    let seed=*MASK_SEEDS.get(index).ok_or("mask index out of range")?;
    if seed==0 || seed>=BITMASK_20 {return Err("invalid 20-bit mask seed".into());}
    Ok(if complement {seed ^ BITMASK_20} else {seed})
}

pub fn masked_bit_planes()->Result<[u32;72],String>{
    let mut values=Vec::with_capacity(72);
    // Base row and complementary row share exactly the same ordering.
    for invert in [true,false]{for index in MASK_BASE_ORDER {values.push(mask(index,invert)?);}}
    values.extend([mask(5,false)?,mask(0,false)?,mask(6,true)?,mask(1,true)?,0,0,0,0]);
    for index in [4,7,3,2]{values.push(mask(index,true)?);}
    values.extend([0,0,0,0]);
    for row in MASK_BLOCK_ROWS {
        for (column,index) in row.into_iter().enumerate() {
            values.push(mask(index,column>=4)?);
        }
    }
    for index in MASK_BASE_ORDER{values.push(mask(index,true)?);}
    let result:[u32;72]=values.try_into().map_err(|_|"mask table is not 72 words")?;
    if result.iter().any(|value| *value>BITMASK_20) {
        return Err("bitmask exceeds 20 bits".into());
    }
    Ok(result)
}

fn put_u16(out:&mut Vec<u8>, value:u16){out.extend_from_slice(&value.to_le_bytes());}
fn put_u32(out:&mut Vec<u8>, value:u32){out.extend_from_slice(&value.to_le_bytes());}

/// Construct 484 bytes from typed scalar, index, permutation and mask tables.
pub fn emit_lookups()->Result<Vec<u8>,String>{
    let mut result=Vec::with_capacity(LOOKUP_BYTES);
    result.extend([9,8,8,9,8,8]);
    put_u16(&mut result,1);
    for angle in angular_thresholds()?{put_u16(&mut result,angle);}
    // Five ordered graph edges over four controller channels.
    for (source,destination) in [(0,0),(0,1),(0,2),(1,3),(2,3)]{
        result.extend([source,destination]);
    }
    let allocations=allocations::allocations()?;
    if allocations.len()!=24 {return Err("controller allocation table changed".into());}
    for alloc in &allocations[20..24] {put_u32(&mut result,alloc.address);}
    for alloc in &allocations[16..20] {put_u32(&mut result,alloc.address);}
    // A related quartet of 1600-byte planes has an independently decoded base.
    for slot in 0..4_u32 {put_u32(&mut result,0x28e20+slot*0x640);}
    for alloc in &allocations[8..12] {put_u32(&mut result,alloc.address);}
    result.extend(channel_permutation());
    result.extend(1_u8..7_u8);
    result.extend([0,7]);
    // Three runs of six-channel and empty-lane selectors plus a final quartet.
    result.extend([8,0,0,0]);
    result.extend([6;9]);
    result.extend([0;3]);
    result.extend([4,5,6,7]);
    for mask in masked_bit_planes()? {put_u32(&mut result,mask);}
    result.extend([2,2,2,3,3,3,4,4]);
    if result.len()!=LOOKUP_BYTES {return Err(format!("lookup emitted {} bytes, expected {LOOKUP_BYTES}",result.len()));}
    Ok(result)
}

/// Three seven-element revision-specific profiles with explicit null
/// terminators and one two-word alignment tail. Values are decoded scalar
/// calibration constants; no generating physical measurement is asserted.
pub const OBSERVED_CALIBRATION_PROFILES:[[u16;7];3]=[
    [109,111,112,114,115,117,118],
    [109,103,100,94,85,85,85],
    [0,5,10,19,25,40,60],
];

pub fn emit_profiles()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(PROFILE_BYTES);
    for curve in OBSERVED_CALIBRATION_PROFILES {
        for value in curve {put_u16(&mut out,value);}
        put_u16(&mut out,0);
    }
    // Two final u16 zero-terminators belong to the table layout, not BSS.
    out.extend([0;4]);
    if out.len()!=PROFILE_BYTES {return Err("profile byte length changed".into());}
    Ok(out)
}

pub fn emit()->Result<Vec<(u32,Vec<u8>)>,String>{
    let result=vec![(LOOKUPS_START,emit_lookups()?), (PROFILES_START,emit_profiles()?)];
    if result.iter().map(|(_,bytes)|bytes.len()).sum::<usize>()!=TOTAL_BYTES {
        return Err("auxiliary lookup accounting changed".into());
    }
    Ok(result)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn tangent_thresholds_match_the_known_math_and_observed_terminal_three() {
        let curve=angular_thresholds().unwrap();
        assert_eq!(curve[0],0);
        assert_eq!(curve[15],100);
        assert_eq!(curve[27],631);
        assert_eq!(&curve[28..],&[953,1913,5730]);
    }
    #[test]
    fn permutation_is_complete_and_xor_affine(){
        let p=channel_permutation();
        assert_eq!(p,[0,9,11,2,15,6,4,13,7,14,12,5,8,1,3,10]);
        let mut sorted=p.to_vec();sorted.sort();
        assert_eq!(sorted,(0_u8..16).collect::<Vec<_>>());
    }
    #[test]
    fn compressed_masks_are_closed_under_complement(){
        let values=masked_bit_planes().unwrap();
        assert_eq!(values.len(),72);
        for i in 0..8 {assert_eq!(values[i]^BITMASK_20,values[i+8]);}
        assert_eq!(values[32],MASK_SEEDS[1]);
        assert!(values.iter().filter(|&&v|v==0).count()==8);
    }
    #[test]
    fn explicit_reference_decoded_data_profiles_have_no_holes(){
        assert_eq!(emit_lookups().unwrap().len(),484);
        assert_eq!(emit_profiles().unwrap().len(),52);
        assert_eq!(emit().unwrap().len(),2);
    }
}
