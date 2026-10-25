// SPDX-License-Identifier: GPL-3.0-only
//! A small *typed* Andes NDS32 (AndeStar) encoder and the first Novatek
//! interrupt entry/return handler. This is code reconstruction, NOT a copy
//! of opaque firmware instruction bytes.
//!
//! The instruction byte stream is **big-endian**, even though the Novatek
//! firmware's partition descriptors and vector table are little-endian. The
//! instruction stream freely intermixes 16- and 32-bit instructions.
//!
//! Encoding references: AndeStar ISA v1.3, and independent Ghidra NDS32
//! SLEIGH patterns (`nds32.sinc`, `lsmw.sinc`). No claim is made that the
//! complete ILM, DLM, or the precise Novatek microcontroller model is known.

pub const FIRST_HANDLER_OFFSET: usize = 0x450;
pub const FIRST_HANDLER_END: usize = 0x4aa;
/// Two related full context save/return paths using a narrower GPR range.
pub const EXTRA_INTERRUPTS_END:usize=0x544;

/// General-purpose NDS32 register number (0–31), using ABI conventional names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gpr(u8);
impl Gpr {
    pub const A0: Self = Self(0);
    pub const A1: Self = Self(1);
    pub const A2: Self = Self(2);
    pub const A3: Self = Self(3);
    pub const A5: Self = Self(5);
    pub const TA: Self = Self(15);
    pub const P1: Self = Self(27);
    pub const SP: Self = Self(31);
    pub const fn number(self) -> u32 { self.0 as u32 }
    pub fn new(n: u8) -> Result<Self, String> {
        if n > 31 { return Err(format!("invalid NDS32 GPR {n}: must be 0..31")); }
        Ok(Self(n))
    }
}

/// The four special registers encoded by NDS32 LMW/SMW Enable4 mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SavedSpecials(u8);
impl SavedSpecials {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(15);
    pub const LINK: Self = Self(2);
    #[allow(dead_code)] // Public checked constructor; current source uses named masks.
    pub fn new(mask: u8) -> Result<Self, String> {
        if mask & !15 != 0 { return Err("invalid LMW/SMW specials mask".into()); }
        Ok(Self(mask))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryDirection { Load, Store }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressUpdate { BeforeIncrement, AfterDecrement }

/// Encode `lmw/smw` with optional first/last GPR and fp/gp/lp/sp mask.
/// In this Nabu ISR, all observed forms update SP (`LsmwM=1`). The native
/// bitfields and placement match the public NDS32 v1.3 instruction manual.
pub fn multiple_words(
    direction: MemoryDirection,
    first: Gpr,
    base: Gpr,
    last: Gpr,
    specials: SavedSpecials,
    update: AddressUpdate,
    writeback: bool,
) -> Result<u32, String> {
    if first.number() > last.number() {
        return Err("LMW/SMW first register must not exceed last register".into());
    }
    let (after, decrement) = match update {
        AddressUpdate::BeforeIncrement => (0_u32, 0_u32),
        AddressUpdate::AfterDecrement => (1_u32, 1_u32),
    };
    Ok((0b011101_u32 << 25)
        | (first.number() << 20)
        | (base.number() << 15)
        | (last.number() << 10)
        | ((specials.0 as u32) << 6)
        | ((direction == MemoryDirection::Store) as u32) << 5
        | after << 4 | decrement << 3 | ((writeback as u32) << 2))
}

/// User special-register read/write (`mfusr`/`mtusr`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserSpecial { IfcLp }
impl UserSpecial {
    fn fields(self) -> (u32,u32) {
        match self { Self::IfcLp => (0,29) }
    }
}
pub fn user_register(rt:Gpr, special:UserSpecial, write:bool) -> u32 {
    let (group, usr) = special.fields();
    (0b100001_u32<<25) | (rt.number()<<20) | (usr<<15)
        | (group<<10) | (0b100000 | write as u32)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemRegister {
    /// Processor status (PSW).
    Psw,
    /// Interrupt processor status (IPSW).
    Ipsw,
    /// Interrupt program counter (IPC).
    Ipc,
    /// CSR 0x99; register interpretation requires an exact controller map.
    Csr099,
}
impl SystemRegister {
    pub fn index(self) -> u32 {
        match self {
            Self::Psw => 0x80,
            Self::Ipsw => 0x81,
            Self::Ipc => 0xa9,
            Self::Csr099 => 0x99,
        }
    }
}
pub fn system_register(rt:Gpr, source:SystemRegister, write:bool) -> u32 {
    (0b110010_u32<<25) | (rt.number()<<20)
        | (source.index()<<10) | (0b00010 + write as u32)
}

/// `setgie.d` disables global interrupts in the running CPU.
pub fn disable_global_interrupts() -> u32 {
    (0b110010_u32<<25) | (0x80<<10) | (2<<5) | 3
}
pub fn data_synchronization_barrier() -> u32 { (0b110010_u32<<25)|8 }
pub fn interrupt_return() -> u32 { (0b110010_u32<<25)|4 }

pub fn mov55(rt:Gpr, ra:Gpr) -> u16 {
    0x8000 | ((rt.number() as u16)<<5) | ra.number() as u16
}

pub fn shift_right_immediate(rt:Gpr, shift:u8) -> Result<u16,String> {
    if rt.number() > 15 || shift > 31 {
        return Err("16-bit srli45 needs a0..ta and shift 0..31".into());
    }
    Ok(0x8000 | (0b001001<<9) | ((rt.number() as u16)<<5) | shift as u16)
}

/// 16-bit `fexti33` bitmask of the least-significant (width+1) bits.
pub fn extract_low_bits(rt:Gpr,width:u8) -> Result<u16,String> {
    if rt.number()>7 || width>7 {
        return Err("16-bit fexti33 needs a0..s1 and width 0..7".into());
    }
    Ok(0x8000|(0b001011<<9)|((rt.number() as u16)<<6)
        | ((width as u16)<<3) | 0b111)
}

pub fn move_immediate(rt:Gpr, immediate:i32) -> Result<u32,String> {
    if !(-524_288..=524_287).contains(&immediate) {
        return Err("movi immediate must fit signed 20 bits".into());
    }
    Ok((0b100010_u32<<25) | (rt.number()<<20)
        | ((immediate as u32) & 0x000f_ffff))
}

pub fn load_word_indexed(rt:Gpr, base:Gpr, index:Gpr,shift:u8) -> Result<u32,String>{
    if shift > 3 { return Err("NDS32 MEM scaled index must be 0..3".into()); }
    Ok((0b011100_u32<<25)|(rt.number()<<20)|(base.number()<<15)
        |(index.number()<<10)|((shift as u32)<<8)|0b00000010)
}

/// 16-bit `subi45`: decrement one of a0..s8/ta registers by a 5-bit value.
pub fn subtract_reg_small(rt:Gpr,amount:u8)->Result<u16,String>{
    if rt.number()>15 || amount>31 {
        return Err("subi45 requires a0..ta and unsigned five-bit immediate".into());
    }
    Ok(0x8000|(0b000111<<9)|((rt.number() as u16)<<5)|amount as u16)
}

pub fn subtract_small_immediate(rt:Gpr,src:Gpr,value:u8)->Result<u16,String>{
    if rt.number()>7 || src.number()>7 || value>7 {
        return Err("subi333 uses a0..s1 and unsigned three-bit immediate".into());
    }
    Ok(0x8000|(0b001111<<9)|((rt.number() as u16)<<6)
        |((src.number() as u16)<<3)|value as u16)
}

/// NDS32 short branch compares one of a0..s1 with a5; displacement is a
/// signed number of 2-byte units from the *instruction start*.
pub fn branch_not_equal_to_a5(at:usize,rt:Gpr,target:usize)->Result<u16,String>{
    if rt.number()>7 || at & 1 != 0 || target & 1 != 0 {
        return Err("NDS32 short branch needs a0..s1 and aligned offsets".into());
    }
    let delta = (target as i64 - at as i64) / 2;
    if !(-128..=127).contains(&delta) {
        return Err("NDS32 bnes38 displacement exceeds signed 8-bit limit".into());
    }
    Ok(0x8000|(0b1011<<11)|((rt.number() as u16)<<8)|(delta as u16 & 0xff))
}

fn be32(dst:&mut Vec<u8>,inst:u32){dst.extend_from_slice(&inst.to_be_bytes());}
fn be16(dst:&mut Vec<u8>,inst:u16){dst.extend_from_slice(&inst.to_be_bytes());}

/// Reconstruct a full exception entry / return *path* in the Nabu ILM.
/// The first instruction saves the general and special register context,
/// followed by IPC/IPSW handling. The IRET at the end restores execution.
/// Another branch at file 0x484 jumps to 0x4c8 outside this linear block.
/// The emitted path matches *both* Nabu revisions and requires no reference
/// bytes or externally supplied executable data.
pub fn initial_interrupt_handler()->Result<Vec<u8>,String>{
    use AddressUpdate::{AfterDecrement as Ad,BeforeIncrement as Bi};
    use Gpr as Reg;
    let (a0,a1,a2,a3,a5,sp)=(Reg::A0,Reg::A1,Reg::A2,Reg::A3,Reg::A5,Reg::SP);
    use MemoryDirection::{Load as L,Store as S};
    let (all,none)=(SavedSpecials::ALL,SavedSpecials::NONE);
    let mut v=Vec::with_capacity(FIRST_HANDLER_END-FIRST_HANDLER_OFFSET);
    be32(&mut v,multiple_words(S,a0,sp,Gpr::new(27)?,all,Ad,true)?);
    be32(&mut v,user_register(a1,UserSpecial::IfcLp,false));
    be32(&mut v,multiple_words(S,a1,sp,a2,none,Ad,true)?);
    be32(&mut v,system_register(a1,SystemRegister::Ipc,false));
    be32(&mut v,system_register(a2,SystemRegister::Ipsw,false));
    be32(&mut v,multiple_words(S,a1,sp,a2,none,Ad,true)?);
    be16(&mut v,mov55(a1,sp));
    be32(&mut v,system_register(a0,SystemRegister::Csr099,false));
    be16(&mut v,shift_right_immediate(a0,5)?);
    be16(&mut v,extract_low_bits(a0,6)?);
    be32(&mut v,move_immediate(a2,300)?);
    be32(&mut v,load_word_indexed(a2,a2,a0,2)?);
    be32(&mut v,system_register(a3,SystemRegister::Psw,false));
    be16(&mut v,subtract_small_immediate(a3,a3,1)?);
    be32(&mut v,system_register(a3,SystemRegister::Psw,true));
    be16(&mut v,branch_not_equal_to_a5(0x484,a5,0x4c8)?);
    be32(&mut v,disable_global_interrupts());
    be32(&mut v,data_synchronization_barrier());
    be32(&mut v,multiple_words(L,a1,sp,a2,none,Bi,true)?);
    be32(&mut v,system_register(a1,SystemRegister::Ipc,true));
    be32(&mut v,system_register(a2,SystemRegister::Ipsw,true));
    be32(&mut v,multiple_words(L,a1,sp,a2,none,Bi,true)?);
    be32(&mut v,user_register(a1,UserSpecial::IfcLp,true));
    be32(&mut v,multiple_words(L,a0,sp,Gpr::new(27)?,all,Bi,true)?);
    be32(&mut v,interrupt_return());
    if v.len()!=FIRST_HANDLER_END-FIRST_HANDLER_OFFSET {
        return Err(format!("ISR emitter produced {} bytes, expected {}",
            v.len(),FIRST_HANDLER_END-FIRST_HANDLER_OFFSET));
    }
    Ok(v)
}

/// Reconstruct two further ILM interrupt entry/return paths without copying
/// firmware bytes. The first saves ta..p1 plus lp and a0..a5; the second
/// additionally saves IPC/IPSW and manages the PSW global-interrupt bit.
/// The 16-bit branches target 0x510 and 0x55e respectively; the latter is
/// outside this span and will be handled in later reconstruction.
pub fn additional_interrupt_paths()->Result<Vec<u8>,String>{
    use AddressUpdate::{AfterDecrement as Ad,BeforeIncrement as Bi};
    use MemoryDirection::{Load as L,Store as S};
    let (a0,a1,a2,a3,a5,sp,ta,p1)=(Gpr::A0,Gpr::A1,Gpr::A2,Gpr::A3,Gpr::A5,Gpr::SP,Gpr::TA,Gpr::P1);
    let (none,link)=(SavedSpecials::NONE,SavedSpecials::LINK);
    let mut v=Vec::with_capacity(EXTRA_INTERRUPTS_END-FIRST_HANDLER_END);
    // Entry 0x4aa: secondary interrupt branch; saves caller and link state.
    be32(&mut v,multiple_words(S,ta,sp,p1,link,Ad,true)?);
    be32(&mut v,multiple_words(S,a0,sp,a5,none,Ad,true)?);
    be32(&mut v,user_register(a1,UserSpecial::IfcLp,false));
    be32(&mut v,multiple_words(S,a1,sp,a2,none,Ad,true)?);
    be32(&mut v,system_register(a0,SystemRegister::Csr099,false));
    be16(&mut v,shift_right_immediate(a0,5)?);
    be16(&mut v,extract_low_bits(a0,6)?);
    be32(&mut v,move_immediate(a2,300)?);
    be32(&mut v,load_word_indexed(a2,a2,a0,2)?);
    be16(&mut v,subtract_reg_small(a0,9)?);
    be16(&mut v,branch_not_equal_to_a5(0x4cc,a5,0x510)?);
    be32(&mut v,multiple_words(L,a1,sp,a2,none,Bi,true)?);
    be32(&mut v,user_register(a1,UserSpecial::IfcLp,true));
    be32(&mut v,multiple_words(L,a0,sp,a5,none,Bi,true)?);
    be32(&mut v,multiple_words(L,ta,sp,p1,link,Bi,true)?);
    be32(&mut v,interrupt_return());
    if v.len()!=0x4e2-FIRST_HANDLER_END {
        return Err(format!("secondary NDS32 handler length {} != 56",v.len()));
    }
    // Entry 0x4e2: similar ISR, with explicit IPC and IPSW state.
    be32(&mut v,multiple_words(S,ta,sp,p1,link,Ad,true)?);
    be32(&mut v,multiple_words(S,a0,sp,a5,none,Ad,true)?);
    be32(&mut v,user_register(a1,UserSpecial::IfcLp,false));
    be32(&mut v,multiple_words(S,a1,sp,a2,none,Ad,true)?);
    be32(&mut v,system_register(a1,SystemRegister::Ipc,false));
    be32(&mut v,system_register(a2,SystemRegister::Ipsw,false));
    be32(&mut v,multiple_words(S,a1,sp,a2,none,Ad,true)?);
    be32(&mut v,system_register(a0,SystemRegister::Csr099,false));
    be16(&mut v,shift_right_immediate(a0,5)?);
    be16(&mut v,extract_low_bits(a0,6)?);
    be32(&mut v,move_immediate(a2,300)?);
    be32(&mut v,load_word_indexed(a2,a2,a0,2)?);
    be16(&mut v,subtract_reg_small(a0,9)?);
    be32(&mut v,system_register(a3,SystemRegister::Psw,false));
    be16(&mut v,subtract_small_immediate(a3,a3,1)?);
    be32(&mut v,system_register(a3,SystemRegister::Psw,true));
    be16(&mut v,branch_not_equal_to_a5(0x51a,a5,0x55e)?);
    be32(&mut v,disable_global_interrupts());
    be32(&mut v,data_synchronization_barrier());
    be32(&mut v,multiple_words(L,a1,sp,a2,none,Bi,true)?);
    be32(&mut v,system_register(a1,SystemRegister::Ipc,true));
    be32(&mut v,system_register(a2,SystemRegister::Ipsw,true));
    be32(&mut v,multiple_words(L,a1,sp,a2,none,Bi,true)?);
    be32(&mut v,user_register(a1,UserSpecial::IfcLp,true));
    be32(&mut v,multiple_words(L,a0,sp,a5,none,Bi,true)?);
    be32(&mut v,multiple_words(L,ta,sp,p1,link,Bi,true)?);
    be32(&mut v,interrupt_return());
    if v.len()!=EXTRA_INTERRUPTS_END-FIRST_HANDLER_END {
        return Err(format!("NDS32 additional interrupts produced {} bytes, expected {}",
            v.len(),EXTRA_INTERRUPTS_END-FIRST_HANDLER_END));
    }
    Ok(v)
}

pub fn verify_additional_interrupt_paths(firmware:&[u8])->Result<(),String>{
    if firmware.len()<EXTRA_INTERRUPTS_END {return Err("firmware too short for extra ISRs".into());}
    let bytes=additional_interrupt_paths()?;
    let candidate=&firmware[FIRST_HANDLER_END..EXTRA_INTERRUPTS_END];
    if let Some(offset)=(0..bytes.len()).find(|&i|bytes[i]!=candidate[i]) {
        return Err(format!("NDS32 extra ISR mismatch at file offset 0x{:x}: built 0x{:02x} reference 0x{:02x}",
            FIRST_HANDLER_END+offset,bytes[offset],candidate[offset]));
    }
    Ok(())
}

/// All source-authored NDS32 exception paths currently reconstructed at the
/// beginning of ILM. The real firmware may branch out to later handlers;
/// those targets must not be claimed reconstructed by this function.
pub fn authored_interrupt_prefix()->Result<Vec<u8>,String>{
    let mut result=initial_interrupt_handler()?;
    result.extend_from_slice(&additional_interrupt_paths()?);
    if result.len()!=EXTRA_INTERRUPTS_END-FIRST_HANDLER_OFFSET {
        return Err("NDS32 authored-prefix concatenation failed".into());
    }
    Ok(result)
}

pub fn verify_authored_interrupt_prefix(firmware:&[u8])->Result<(),String>{
    verify_first_handler(firmware)?;
    verify_additional_interrupt_paths(firmware)
}

/// Confirm the independently built ISR *path* matches the provided *private*
/// reference, and that it is physically inside the pinned ILM partition.
pub fn verify_first_handler(firmware:&[u8])->Result<(),String>{
    if firmware.len() <= FIRST_HANDLER_END {return Err("firmware too short for ISR".into());}
    let built=initial_interrupt_handler()?;
    let reference=&firmware[FIRST_HANDLER_OFFSET..FIRST_HANDLER_END];
    if let Some(i)=(0..built.len()).find(|&i|built[i]!=reference[i]) {
        return Err(format!("NDS32 ISR mismatch at file offset 0x{:x}: built 0x{:02x}, reference 0x{:02x}",
            FIRST_HANDLER_OFFSET+i,built[i],reference[i]));
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn rejects_invalid_nds32_operands(){
        assert!(Gpr::new(32).is_err());
        assert!(SavedSpecials::new(16).is_err());
        assert!(multiple_words(MemoryDirection::Store,Gpr::A3,Gpr::SP,Gpr::A1,
            SavedSpecials::NONE,AddressUpdate::AfterDecrement,true).is_err());
        assert!(shift_right_immediate(Gpr::SP,1).is_err());
        assert!(extract_low_bits(Gpr::SP,3).is_err());
        assert!(move_immediate(Gpr::A0,524288).is_err());
        assert!(move_immediate(Gpr::A0,-524289).is_err());
        assert!(load_word_indexed(Gpr::A0,Gpr::A1,Gpr::A2,4).is_err());
        assert!(subtract_small_immediate(Gpr::A0,Gpr::A1,8).is_err());
        assert!(subtract_reg_small(Gpr::SP,3).is_err());
        assert!(branch_not_equal_to_a5(0x483,Gpr::A5,0x4c8).is_err());
        assert!(branch_not_equal_to_a5(0x484,Gpr::A5,0x684).is_err());
    }
    #[test]fn individual_nds32_isa_words_have_expected_field_layout(){
        assert_eq!(system_register(Gpr::A1,SystemRegister::Ipc,false),0x6412a402);
        assert_eq!(system_register(Gpr::A2,SystemRegister::Ipsw,false),0x64220402);
        assert_eq!(system_register(Gpr::A3,SystemRegister::Psw,true),0x64320003);
        assert_eq!(user_register(Gpr::A1,UserSpecial::IfcLp,false),0x421e8020);
        assert_eq!(user_register(Gpr::A1,UserSpecial::IfcLp,true),0x421e8021);
        assert_eq!(mov55(Gpr::A1,Gpr::SP),0x803f);
        assert_eq!(shift_right_immediate(Gpr::A0,5).unwrap(),0x9205);
        assert_eq!(subtract_reg_small(Gpr::A0,9).unwrap(),0x8e09);
        assert_eq!(extract_low_bits(Gpr::A0,6).unwrap(),0x9637);
        assert_eq!(move_immediate(Gpr::A2,300).unwrap(),0x4420012c);
        assert_eq!(load_word_indexed(Gpr::A2,Gpr::A2,Gpr::A0,2).unwrap(),0x38210202);
        assert_eq!(branch_not_equal_to_a5(0x484,Gpr::A5,0x4c8).unwrap(),0xdd22);
        assert_eq!(disable_global_interrupts(),0x64020043);
        assert_eq!(data_synchronization_barrier(),0x64000008);
        assert_eq!(interrupt_return(),0x64000004);
    }
    #[test]fn reconstructs_two_more_paths_without_vendor_opcode_arrays() {
        let second=additional_interrupt_paths().unwrap();
        assert_eq!(second.len(),154);
        assert_eq!(&second[..4],&multiple_words(
            MemoryDirection::Store,Gpr::TA,Gpr::SP,Gpr::P1,
            SavedSpecials::LINK,AddressUpdate::AfterDecrement,true).unwrap().to_be_bytes());
        assert_eq!(&second[52..56],&interrupt_return().to_be_bytes());
        assert_eq!(&second[second.len()-4..],&interrupt_return().to_be_bytes());
        let mut candidate=vec![0;EXTRA_INTERRUPTS_END];
        candidate[FIRST_HANDLER_END..EXTRA_INTERRUPTS_END].copy_from_slice(&second);
        verify_additional_interrupt_paths(&candidate).unwrap();
        candidate[0x4d8]^=1;
        assert!(verify_additional_interrupt_paths(&candidate)
            .unwrap_err().contains("0x4d8"));
    }
    #[test]fn generated_isr_prefix_is_contiguous_and_typed() {
        let prefix=authored_interrupt_prefix().unwrap();
        assert_eq!(prefix.len(),244);
        assert_eq!(&prefix[..90],&initial_interrupt_handler().unwrap());
        assert_eq!(&prefix[90..],&additional_interrupt_paths().unwrap());
        let mut b=vec![0;EXTRA_INTERRUPTS_END];
        b[FIRST_HANDLER_OFFSET..EXTRA_INTERRUPTS_END].copy_from_slice(&prefix);
        verify_authored_interrupt_prefix(&b).unwrap();
    }
    #[test]fn reconstructs_full_isr_without_raw_firmware_bytes(){
        let built=initial_interrupt_handler().unwrap();
        assert_eq!(built.len(),0x5a);
        assert_eq!(&built[..4],&multiple_words(
            MemoryDirection::Store,Gpr::A0,Gpr::SP,Gpr::new(27).unwrap(),
            SavedSpecials::ALL,AddressUpdate::AfterDecrement,true).unwrap().to_be_bytes());
        assert_eq!(&built[built.len()-4..],&interrupt_return().to_be_bytes());
        let mut broken=vec![0u8;FIRST_HANDLER_END+1];
        assert!(verify_first_handler(&broken).unwrap_err().contains("mismatch"));
        broken[FIRST_HANDLER_OFFSET..FIRST_HANDLER_END].copy_from_slice(&built);
        assert!(verify_first_handler(&broken).is_ok());
        broken[0x46d] ^= 1;
        assert!(verify_first_handler(&broken).unwrap_err().contains("0x46d"));
    }
}
