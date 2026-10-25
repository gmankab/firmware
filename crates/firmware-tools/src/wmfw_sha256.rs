// SPDX-License-Identifier: GPL-3.0-only
//! Minimal dependency-free SHA-256 used to pin optional *external* WMFW
//! proprietary DSP payload inputs. This is not a HALO ISA decoder.
//! FIPS 180-4 hash construction; never interpreted as a firmware signature.

const INIT: [u32;8] = [
    0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,
    0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19,
];
const K: [u32;64] = [
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
];

fn compress(state: &mut [u32;8], chunk: &[u8]) {
    debug_assert_eq!(chunk.len(),64);
    let mut w=[0u32;64];
    for (i,c) in chunk.chunks_exact(4).enumerate() {
        w[i]=u32::from_be_bytes(c.try_into().unwrap());
    }
    for t in 16..64 {
        let s0=w[t-15].rotate_right(7)^w[t-15].rotate_right(18)^(w[t-15]>>3);
        let s1=w[t-2].rotate_right(17)^w[t-2].rotate_right(19)^(w[t-2]>>10);
        w[t]=w[t-16].wrapping_add(s0).wrapping_add(w[t-7]).wrapping_add(s1);
    }
    let [mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut h]=*state;
    for t in 0..64 {
        let s1=e.rotate_right(6)^e.rotate_right(11)^e.rotate_right(25);
        let ch=(e&f)^(!e&g);
        let t1=h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[t]).wrapping_add(w[t]);
        let s0=a.rotate_right(2)^a.rotate_right(13)^a.rotate_right(22);
        let maj=(a&b)^(a&c)^(b&c);
        let t2=s0.wrapping_add(maj);
        h=g;g=f;f=e;e=d.wrapping_add(t1);d=c;c=b;b=a;a=t1.wrapping_add(t2);
    }
    for (current,delta) in state.iter_mut().zip([a,b,c,d,e,f,g,h]) {
        *current=current.wrapping_add(delta);
    }
}

pub fn digest(bytes:&[u8]) -> [u8;32] {
    let mut state=INIT;
    let mut chunks=bytes.chunks_exact(64);
    for block in &mut chunks {compress(&mut state,block);}
    let rest=chunks.remainder();
    let mut tail=[0u8;128];
    tail[..rest.len()].copy_from_slice(rest);
    tail[rest.len()]=0x80;
    let padded=if rest.len()>=56 {128}else{64};
    let bit_len=(bytes.len() as u64).wrapping_mul(8);
    tail[padded-8..padded].copy_from_slice(&bit_len.to_be_bytes());
    for block in tail[..padded].chunks_exact(64) {compress(&mut state,block);}
    let mut out=[0u8;32];
    for (dst,word) in out.chunks_exact_mut(4).zip(state) {dst.copy_from_slice(&word.to_be_bytes());}
    out
}

pub fn hex_digest(bytes:&[u8]) -> String {
    digest(bytes).iter().map(|x|format!("{x:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fips_sha256_known_vectors() {
        assert_eq!(hex_digest(b""),"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(hex_digest(b"abc"),"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(hex_digest(&vec![b'a';1000000]),
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0");
    }
    #[test]
    fn sha256_padding_and_block_boundaries() {
        // Independent Python hashlib/OpenSSL fixtures generated from sequence i*7+3.
        const EXAMPLES:&[(usize,&str)]=&[
            (0,"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            (1,"084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5"),
            (55,"e7313d333c272e639f790978283f9eb392e843d0f29b7016828bb1daa4aac70b"),
            (56,"4324d65f3c103567f5589c710bc08f8523f929a9272e3af36fc968e52abc6c27"),
            (63,"81c80242132f230c3bd41b3e63bbcff16107339549214a99614ff26664625055"),
            (64,"39e3d7b6b5d075d37d053ad89b24b41bef4f3c29760c84447cab3f3be1882241"),
            (65,"aacca6ff74fdbb296d165a45cecfa04e5127bc008770fbbdd48006f2d2fae95e"),
            (119,"9ce7368e4daf32341631b492e80359dc9f594b48453cd0dd5bf0b19279cc177e"),
            (120,"7836b787757e95e58b3ca5aec90b1b004e8deba1e50e9675af9cabf1a13a04b5"),
            (127,"a8d23e75d936f303d248888d9b165ee543f4cbafcad3c9dd2a79bd84faa11d07"),
            (128,"d2742f1f4ac6bb7ca2b239ee18402ba8b3f9f8e652d2a72973c2b9ba11c08cf6"),
            (129,"307f8fc2c1622b92762e818d39a185d4d667ad49a4b07ceae1f4afa008a93ec4"),
            (257,"b65c390e4482123ae81c3462cd2ce5bac55cd9dfde2fcf9295c7d320ee453fc5"),
        ];
        for (n,digest) in EXAMPLES {
            let bytes:Vec<u8>=(0..*n).map(|i|(i*7+3)as u8).collect();
            assert_eq!(&hex_digest(&bytes),digest);
        }
    }
}
