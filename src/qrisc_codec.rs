// SPDX-License-Identifier: GPL-3.0-only
//! Minimal QRISC instruction-field decoder/encoder, derived from public Mesa
//! QRISC ISA documentation. No OEM firmware bytes or programs are embedded.
//!
//! This is *not* an instruction semantics simulator. In particular, values
//! marked `ignored_bits` may have vendor-specific effects that are unknown.
//! A decode->encode identity test checks bit-field plumbing ONLY, and is not
//! evidence of independently reconstructed executable behavior.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QriscWord {
    /// QRISC-A6xx nop with 24 bits of metadata (also used as FW ID header).
    Nop { metadata: u32 },
    ImmediateAlu { opcode: u8, dst: u8, src: u8, imm: u16 },
    MovImmediate { repeat: bool, dst: u8, shift: u8, imm: u16 },
    RegisterAlu { src1: u8, src2: u8, dst: u8, function: u16 },
    ControlWrite { base: u8, source: u8, reg: u16 },
    ControlRead { base: u8, dest: u8, reg: u16 },
    Branch { opcode: u8, src: u8, condition: u8, delta: i16 },
    Return { ignored_bits: u32 },
    Call { target: u32 },
}

impl QriscWord {
    /// Publicly described instruction encodings. Some field values are opaque;
    /// callers must not equate lossless repacking with understanding a program.
    pub fn decode(w: u32) -> Option<Self> {
        let opcode = (w >> 26) as u8;
        let reg_a = ((w >> 21) & 31) as u8;
        let reg_b = ((w >> 16) & 31) as u8;
        let immediate = w as u16;
        match opcode {
            0x00 if w >> 24 == 1 => Some(Self::Nop { metadata: w & 0x00ff_ffff }),
            0x02 | 0x0a | 0x0c | 0x0e | 0x12 | 0x14 | 0x18 =>
                Some(Self::ImmediateAlu { opcode, src: reg_a, dst: reg_b, imm: immediate }),
            0x22 | 0x23 => Some(Self::MovImmediate {
                repeat: opcode == 0x23, dst: reg_b, shift: reg_a, imm: immediate,
            }),
            0x26 => Some(Self::RegisterAlu {
                src1: reg_a, src2: reg_b, dst: ((w >> 11) & 31) as u8,
                function: (w & 0x07ff) as u16,
            }),
            0x2a => Some(Self::ControlWrite { base: reg_a, source: reg_b, reg: immediate }),
            0x2e => Some(Self::ControlRead { base: reg_a, dest: reg_b, reg: immediate }),
            0x30..=0x33 => Some(Self::Branch {
                opcode, src: reg_a, condition: reg_b, delta: immediate as i16,
            }),
            0x34 => Some(Self::Return { ignored_bits: w & 0x03ff_ffff }),
            0x35 => Some(Self::Call { target: w & 0x03ff_ffff }),
            _ => None,
        }
    }

    pub fn encode(&self) -> Result<u32, &'static str> {
        fn register(n: u8) -> Result<u32, &'static str> {
            if n < 32 { Ok(n as u32) } else { Err("QRISC register index exceeds 31") }
        }
        let ins = match *self {
            Self::Nop { metadata } => {
                if metadata > 0x00ff_ffff { return Err("NOP metadata exceeds 24 bits"); }
                (1 << 24) | metadata
            }
            Self::ImmediateAlu { opcode, dst, src, imm } => {
                if !matches!(opcode, 0x02 | 0x0a | 0x0c | 0x0e | 0x12 | 0x14 | 0x18) {
                    return Err("unsupported immediate ALU opcode");
                }
                (u32::from(opcode) << 26) | (register(src)? << 21) |
                    (register(dst)? << 16) | u32::from(imm)
            }
            Self::MovImmediate { repeat, dst, shift, imm } => {
                ((if repeat { 0x23 } else { 0x22 }) << 26) | (register(shift)? << 21) |
                    (register(dst)? << 16) | u32::from(imm)
            }
            Self::RegisterAlu { src1, src2, dst, function } => {
                if function > 0x07ff { return Err("ALU function exceeds field width"); }
                (0x26 << 26) | (register(src1)? << 21) | (register(src2)? << 16) |
                    (register(dst)? << 11) | u32::from(function)
            }
            Self::ControlWrite { base, source, reg } =>
                (0x2a << 26) | (register(base)? << 21) |
                (register(source)? << 16) | u32::from(reg),
            Self::ControlRead { base, dest, reg } =>
                (0x2e << 26) | (register(base)? << 21) |
                (register(dest)? << 16) | u32::from(reg),
            Self::Branch { opcode, src, condition, delta } => {
                if !(0x30..=0x33).contains(&opcode) { return Err("unsupported QRISC branch"); }
                (u32::from(opcode) << 26) | (register(src)? << 21) |
                    (register(condition)? << 16) | u32::from(delta as u16)
            }
            Self::Return { ignored_bits } => {
                if ignored_bits > 0x03ff_ffff { return Err("return field exceeds 26 bits"); }
                (0x34 << 26) | ignored_bits
            }
            Self::Call { target } => {
                if target > 0x03ff_ffff { return Err("call target exceeds 26 bits"); }
                (0x35 << 26) | target
            }
        };
        Ok(ins)
    }
}

/// Count control instructions that decode to public QRISC fields and losslessly
/// reencode. Omit the embedded IR3 range. This is NOT an independent rebuild.
pub fn check_isa_fields(stream: &[u8], shader: Option<(usize,usize)>) -> Result<(usize, usize), String> {
    if stream.len() % 4 != 0 { return Err("stream has partial word".into()); }
    let words=stream.len()/4;
    if shader.is_some_and(|(start,end)| start>end || end>words) {
        return Err("invalid shader bounds".into());
    }
    let mut decoded=0;
    let mut unknown=0;
    for i in 0..words {
        if shader.is_some_and(|(start,end)| (start..end).contains(&i)) { continue; }
        let original=u32::from_le_bytes(stream[i*4..i*4+4].try_into().unwrap());
        if let Some(ins)=QriscWord::decode(original) {
            if ins.encode().map_err(str::to_string)? != original {
                return Err(format!("QRISC field mismatch at word {i}"));
            }
            decoded+=1;
        } else {unknown+=1;}
    }
    Ok((decoded,unknown))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_roundtrip_for_public_isa_fields() {
        let examples = [
            QriscWord::Nop { metadata: 0x0040_0001 },
            QriscWord::ImmediateAlu { opcode: 0x0a, src: 2, dst: 5, imm: 0xabcd },
            QriscWord::MovImmediate { repeat: false, dst: 9, shift: 16, imm: 0x1234 },
            QriscWord::MovImmediate { repeat: true, dst: 4, shift: 0, imm: 0 },
            QriscWord::RegisterAlu { src1: 7, src2: 3, dst: 8, function: 6 },
            QriscWord::ControlWrite { base: 0, source: 4, reg: 0x12a },
            QriscWord::ControlRead { base: 0, dest: 5, reg: 0x20 },
            QriscWord::Branch { opcode: 0x31, src: 2, condition: 1, delta: -5 },
            QriscWord::Return { ignored_bits: 0 },
            QriscWord::Return { ignored_bits: 1 << 24 },
            QriscWord::Call { target: 0x1024 },
        ];
        for original in &examples {
            let packed = original.encode().unwrap();
            assert_eq!(QriscWord::decode(packed), Some(original.clone()));
        }
    }
    #[test]
    fn reject_invalid_operands_and_unsupported_opcodes() {
        assert!(QriscWord::MovImmediate { repeat:false, dst:32, shift:0, imm:5 }.encode().is_err());
        assert!(QriscWord::ImmediateAlu { opcode:0x7f, dst:0, src:0, imm:0 }.encode().is_err());
        assert_eq!(QriscWord::decode(0xffff_ffff), None);
    }
}
