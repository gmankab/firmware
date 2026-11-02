// SPDX-License-Identifier: GPL-3.0-only
//! Read-only dependency between two DLM calibration profiles. This is a
//! reference-assisted relationship, NOT 428 newly source-derived bytes.
//!
//! The first 428-byte profile is mirrored later in DLM, with identical bytes.
//! A typed full-file builder can reuse a caller-provided *authored* first
//! profile to derive the second; until the first profile is understood and
//! generated without OEM payload, neither span has independent source credit.

pub const PRIMARY_FILE_START:usize=0x22224;
pub const PRIMARY_FILE_END:usize=0x223d0;
pub const MIRROR_FILE_START:usize=0x2d648;
pub const MIRROR_FILE_END:usize=0x2d7f4;
pub const PROFILE_BYTES:usize=PRIMARY_FILE_END-PRIMARY_FILE_START;
pub const DLM_FILE_BIAS:usize=0x2000;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ProfileHeader {
    pub mapped_span:u16,
    pub rows:u16,
    pub controller_parameter:u16,
    pub enabled:u16,
}

fn word(bytes:&[u8],off:usize)->Result<u16,String>{
    let slice=bytes.get(off..off+2).ok_or("truncated profile header")?;
    Ok(u16::from_le_bytes([slice[0],slice[1]]))
}

/// Inspect two observed 16-bit profile headers, at +4 and +0xd6.
/// Values have **not** been shown to be algorithmically determined by an ILM
/// consumer and should not be used to award source data coverage.
pub fn observed_headers(primary:&[u8])->Result<[ProfileHeader;2],String>{
    if primary.len()!=PROFILE_BYTES {return Err("calibration profile must be exactly 428 bytes".into());}
    fn header(bytes:&[u8],at:usize)->Result<ProfileHeader,String>{
        Ok(ProfileHeader {mapped_span:word(bytes,at)?,rows:word(bytes,at+2)?,
            controller_parameter:word(bytes,at+4)?,enabled:word(bytes,at+6)?})
    }
    Ok([header(primary,4)?,header(primary,0xd6)?])
}

/// Derive dependent output from an externally supplied earlier primary.
/// No guessing, padding or hidden reference fallback is allowed.
pub fn duplicate_from_external_primary(primary:&[u8])->Result<Vec<u8>,String>{
    observed_headers(primary)?;
    Ok(primary.to_vec())
}

/// Assert an exact, length-checked intra-DLM dependency in a caller-supplied
/// reference image; this verifies a relationship, **not** independent authorship.
pub fn verify_reference_assisted_copy(image:&[u8])->Result<usize,String>{
    let first=image.get(PRIMARY_FILE_START..PRIMARY_FILE_END)
        .ok_or("missing first DLM calibration profile")?;
    let second=image.get(MIRROR_FILE_START..MIRROR_FILE_END)
        .ok_or("missing repeated DLM calibration profile")?;
    let derived=duplicate_from_external_primary(first)?;
    if let Some(i)=derived.iter().zip(second).position(|(a,b)|a!=b){
        return Err(format!("repeated calibration profile differs at file 0x{:x}",
            MIRROR_FILE_START+i));
    }
    Ok(derived.len())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]
    fn boundaries_are_disjoint_within_dlm_and_have_equal_length(){
        assert_eq!(PROFILE_BYTES,428);
        assert_eq!(MIRROR_FILE_END-MIRROR_FILE_START,PROFILE_BYTES);
        assert!(PRIMARY_FILE_END<MIRROR_FILE_START);
        assert_eq!(PRIMARY_FILE_START-DLM_FILE_BIAS,0x20224);
        assert_eq!(MIRROR_FILE_START-DLM_FILE_BIAS,0x2b648);
    }
    #[test]
    fn repeated_profile_requires_external_primary_and_gets_no_authorship_credit(){
        let primary=(0..PROFILE_BYTES).map(|i|((i*7+3)%256) as u8).collect::<Vec<_>>();
        let duplicated=duplicate_from_external_primary(&primary).unwrap();
        assert_eq!(duplicated,primary);
        let mut full=vec![0;MIRROR_FILE_END];
        full[PRIMARY_FILE_START..PRIMARY_FILE_END].copy_from_slice(&primary);
        full[MIRROR_FILE_START..MIRROR_FILE_END].copy_from_slice(&duplicated);
        assert_eq!(verify_reference_assisted_copy(&full).unwrap(),PROFILE_BYTES);
        full[MIRROR_FILE_START+210]^=1;
        assert!(verify_reference_assisted_copy(&full).unwrap_err().contains("0x2d71a"));
        assert!(verify_reference_assisted_copy(&full[..MIRROR_FILE_END-1]).is_err());
        assert!(duplicate_from_external_primary(&primary[..PROFILE_BYTES-1]).is_err());
    }
    #[test]
    fn inspect_headers_at_the_two_record_offsets(){
        let mut primary=vec![0u8;PROFILE_BYTES];
        for (at,entries) in [(4usize,[1600u16,48,32,1]),(0xd6,[2600,48,70,1])]{
            for (idx,value) in entries.iter().enumerate(){
                primary[at+2*idx..at+2*idx+2].copy_from_slice(&value.to_le_bytes());
            }
        }
        let headers=observed_headers(&primary).unwrap();
        assert_eq!(headers[0],ProfileHeader{mapped_span:1600,rows:48,controller_parameter:32,enabled:1});
        assert_eq!(headers[1],ProfileHeader{mapped_span:2600,rows:48,controller_parameter:70,enabled:1});
        assert_ne!(headers[0].controller_parameter,headers[1].controller_parameter);
    }
}
