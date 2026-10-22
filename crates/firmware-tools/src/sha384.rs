// SPDX-License-Identifier: GPL-3.0-only
//! SHA-384 (FIPS 180-4) for deterministic firmware segment hashes.
//!
//! This small dependency-free implementation is verified against published
//! SHA-384 test vectors and is used to generate image hash-table entries.
//! It is NOT a signature verifier and makes no claim of constant-time behavior.

const INIT: [u64; 8] = [
    0xcbbb9d5dc1059ed8, 0x629a292a367cd507,
    0x9159015a3070dd17, 0x152fecd8f70e5939,
    0x67332667ffc00b31, 0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4,
];
const ROUND: [u64; 80] = [
    0x428a2f98d728ae22,0x7137449123ef65cd,0xb5c0fbcfec4d3b2f,0xe9b5dba58189dbbc,
    0x3956c25bf348b538,0x59f111f1b605d019,0x923f82a4af194f9b,0xab1c5ed5da6d8118,
    0xd807aa98a3030242,0x12835b0145706fbe,0x243185be4ee4b28c,0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,0x80deb1fe3b1696b1,0x9bdc06a725c71235,0xc19bf174cf692694,
    0xe49b69c19ef14ad2,0xefbe4786384f25e3,0x0fc19dc68b8cd5b5,0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,0x4a7484aa6ea6e483,0x5cb0a9dcbd41fbd4,0x76f988da831153b5,
    0x983e5152ee66dfab,0xa831c66d2db43210,0xb00327c898fb213f,0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,0xd5a79147930aa725,0x06ca6351e003826f,0x142929670a0e6e70,
    0x27b70a8546d22ffc,0x2e1b21385c26c926,0x4d2c6dfc5ac42aed,0x53380d139d95b3df,
    0x650a73548baf63de,0x766a0abb3c77b2a8,0x81c2c92e47edaee6,0x92722c851482353b,
    0xa2bfe8a14cf10364,0xa81a664bbc423001,0xc24b8b70d0f89791,0xc76c51a30654be30,
    0xd192e819d6ef5218,0xd69906245565a910,0xf40e35855771202a,0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,0x1e376c085141ab53,0x2748774cdf8eeb99,0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,0x4ed8aa4ae3418acb,0x5b9cca4f7763e373,0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,0x78a5636f43172f60,0x84c87814a1f0ab72,0x8cc702081a6439ec,
    0x90befffa23631e28,0xa4506cebde82bde9,0xbef9a3f7b2c67915,0xc67178f2e372532b,
    0xca273eceea26619c,0xd186b8c721c0c207,0xeada7dd6cde0eb1e,0xf57d4f7fee6ed178,
    0x06f067aa72176fba,0x0a637dc5a2c898a6,0x113f9804bef90dae,0x1b710b35131c471b,
    0x28db77f523047d84,0x32caab7b40c72493,0x3c9ebe0a15c9bebc,0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,0x597f299cfc657e2a,0x5fcb6fab3ad6faec,0x6c44198c4a475817,
];

fn compress(state: &mut [u64; 8], chunk: &[u8]) {
    debug_assert_eq!(chunk.len(), 128);
    let mut schedule = [0u64; 80];
    for (i, bytes) in chunk.chunks_exact(8).enumerate() {
        schedule[i] = u64::from_be_bytes(bytes.try_into().unwrap());
    }
    for i in 16..80 {
        let low = schedule[i-15].rotate_right(1) ^ schedule[i-15].rotate_right(8) ^ (schedule[i-15] >> 7);
        let high = schedule[i-2].rotate_right(19) ^ schedule[i-2].rotate_right(61) ^ (schedule[i-2] >> 6);
        schedule[i] = schedule[i-16].wrapping_add(low).wrapping_add(schedule[i-7]).wrapping_add(high);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..80 {
        let sum1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
        let choose = (e & f) ^ (!e & g);
        let one = h.wrapping_add(sum1).wrapping_add(choose)
            .wrapping_add(ROUND[i]).wrapping_add(schedule[i]);
        let sum0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
        let majority = (a & b) ^ (a & c) ^ (b & c);
        let two = sum0.wrapping_add(majority);
        h=g;g=f;f=e;e=d.wrapping_add(one);d=c;c=b;b=a;a=one.wrapping_add(two);
    }
    for (target, increment) in state.iter_mut().zip([a,b,c,d,e,f,g,h]) {
        *target = target.wrapping_add(increment);
    }
}

/// Hash an in-memory byte slice with SHA-384. The input length must fit in the
/// 128-bit FIPS length field (all practical Rust slices satisfy that bound).
pub fn digest(data: &[u8]) -> [u8; 48] {
    let mut state = INIT;
    let mut pieces = data.chunks_exact(128);
    for chunk in &mut pieces { compress(&mut state, chunk); }
    let remainder = pieces.remainder();
    let mut block = [0u8; 256];
    block[..remainder.len()].copy_from_slice(remainder);
    block[remainder.len()] = 0x80;
    let final_bytes = if remainder.len() >= 112 { 256 } else { 128 };
    let bit_length = (data.len() as u128).wrapping_mul(8);
    block[final_bytes-16..final_bytes].copy_from_slice(&bit_length.to_be_bytes());
    for piece in block[..final_bytes].chunks_exact(128) { compress(&mut state, piece); }
    let mut out = [0u8; 48];
    for (destination, word) in out.chunks_exact_mut(8).zip(state.iter().take(6)) {
        destination.copy_from_slice(&word.to_be_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn hex(data: &[u8]) -> String { data.iter().map(|x| format!("{x:02x}")).collect() }
    #[test]
    fn fips_180_sha384_vectors() {
        assert_eq!(hex(&digest(b"")),
            "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b");
        assert_eq!(hex(&digest(b"abc")),
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7");
    }
    #[test]
    fn multiple_block_lengths_and_padding_boundary_are_stable() {
        let a = vec![b'a'; 1000];
        assert_eq!(hex(&digest(&a)),
            "f54480689c6b0b11d0303285d9a81b21a93bca6ba5a1b4472765dca4da45ee328082d469c650cd3b61b16d3266ab8ced");
        // Cross-check block and padding boundaries with Python hashlib/OpenSSL.
        const VECTORS: &[(usize, &str)] = &[
            (111, "ea9ef03be46bf58140090210c172bca6812a5b99a7425434551eb9af1ce4e5f1d96991569aace9cabff8c0b79782e827"),
            (112, "bac3eb7c844a1ad09997188f4bfe8b1b3dc052459b06a9946f8b8a4b8a88a46d1a81428bea8488c42cd3a79e98813dc8"),
            (127, "2f92ec0d825e9e32a33bf5daf705e6e03ef961f9d53da939c8817e889ff9d7495fb50521b4c6b57cc7bb9ab4ef88c4fb"),
            (128, "077a8afdb61ee1c7a22e0a666e2bdb7408d449fff72d408d9dfd11491d96b60460354203d5672f1f374342579667b406"),
            (129, "98d14fc24559af74e3387768c83381aa35368c7efd47635f6771b3ee9ff3c819fb5b87be802eb370e86e0e5e3af5aff7"),
            (255, "b7c10755ba9fe0a9f3513600458212e1127d7f962fe074896d4e8e8688557037b6a7d2e67292e0c28d338c8b1cc519ed"),
            (256, "5ef7c15a6277c971366e84bee023026f9d650f7cc9dc04ef595b006b0f80e1a9841a3a5beb608234b1dc42b7948c449a"),
            (257, "833fa0ced6db72266bc28d543ebda8b61a5fdb1c6ce4c50e68c154ede700b0ff7b7b2baf7e7ca1160c0705c398b3503c"),
        ];
        for (count, expected) in VECTORS {
            let data: Vec<_> = (0..*count).map(|i| (i * 13 + 7) as u8).collect();
            assert_eq!(&hex(&digest(&data)), expected, "SHA384 padding length {count}");
        }
    }
}
