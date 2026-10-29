// SPDX-License-Identifier: GPL-3.0-only
//! Continuous original-ISA source encoding for postmarketOS Novatek lower ILM.
//! The two previously unknown 0xfdxx halfwords are N16 MOVD44 operations on
//! even-register *pairs*, according to the GNU NDS32 V3MUP opcode definition.
//! Emitted output is assembled exclusively from typed NDS32 instructions and
//! semantic branch-target tables, never by copying private reference bytes.
//! This represents firmware encoding equivalence, not hardware validation.
use super::{Profile,FILE_START,ILM_FILE_BASE};
use super::isa::{Insn,Reg};

/// Exclusive upper boundary of Agent17's assigned ILM file range.
pub const FILE_END:usize=0x10000;
/// The instruction begins two bytes before Agent17/18's file split. It is
/// encoded as a complete NDS32 MOVI, but only the first halfword is owned here.
pub const CROSS_OWNER_WORD_FILE:usize=0xfffe;
pub const MOVD44_GAP_ONE:usize=0x38bc;
pub const MOVD44_GAP_TWO:usize=0x39f8;

fn append(bytes:&mut Vec<u8>,expected_file_offset:usize,part:&[u8])->Result<(),String>{
    if FILE_START+bytes.len()!=expected_file_offset{
        return Err(format!("nonadjacent source at file 0x{:x}; next expected 0x{:x}",
                           expected_file_offset,FILE_START+bytes.len()));
    }
    bytes.extend_from_slice(part);
    Ok(())
}
fn append_source_fragment(bytes:&mut Vec<u8>,frag:super::extension6_disjoint::Fragment)->Result<(),String>{
    let start=frag.file_start;
    let encoded=super::extension6_disjoint::emit(&frag)?;
    append(bytes,start,&encoded)
}
/// Return one completely adjacent source-built interval. The separately
/// pinned map220v firmware layout is not assumed to share this address map.
pub fn build_postmarketos()->Result<Vec<u8>,String>{
    let mut output=super::build(Profile::Postmarketos)?;
    append(&mut output,MOVD44_GAP_ONE,
        &Insn::MoveDoubleCompact{dest_even:Reg::new(4),source_even:Reg::new(0)}
            .encode((MOVD44_GAP_ONE-ILM_FILE_BASE) as u32,Profile::Postmarketos.gp())?)?;
    append_source_fragment(&mut output,super::extension6_disjoint::first_after_unresolved())?;
    append(&mut output,MOVD44_GAP_TWO,
        &Insn::MoveDoubleCompact{dest_even:Reg::new(6),source_even:Reg::new(12)}
            .encode((MOVD44_GAP_TWO-ILM_FILE_BASE) as u32,Profile::Postmarketos.gp())?)?;
    for frag in [
        super::extension6_disjoint::second_after_unresolved(),
        super::extension7_disjoint::fragment(),
        super::extension8_disjoint::fragment(),
        super::extension9_disjoint::fragment(),
        super::extension10_disjoint::fragment(),
        super::extension11::fragment(),
        super::extension12::fragment(),
        super::extension13::fragment(),
        super::extension14::fragment(),
        super::extension15::fragment(),
        super::extension16::fragment(),
        super::extension17::fragment(),
    ]{append_source_fragment(&mut output,frag)?;}
    // Complete source-level N32 instruction encodes 0x44620104 and occupies
    // file [0xfffe,0x10002). Agent18 owns [0x10000,0x10002); do not output it.
    let straddling=Insn::MoveImmediate{dest:Reg::new(6),value:131332}
        .encode((CROSS_OWNER_WORD_FILE-ILM_FILE_BASE) as u32,Profile::Postmarketos.gp())?;
    if straddling.len()!=4{return Err("boundary N32 instruction not exactly four bytes".into());}
    append(&mut output,CROSS_OWNER_WORD_FILE,&straddling[..2])?;
    if FILE_START+output.len()!=FILE_END{
        return Err("typed contiguous lower-ILM writer has unexpected length".into());
    }
    Ok(output)
}

pub fn compare_postmarketos(firmware:&[u8])->Result<(),String>{
    let expected=firmware.get(FILE_START..FILE_END).ok_or("postmarketOS image too short")?;
    let emitted=build_postmarketos()?;
    if let Some(i)=emitted.iter().zip(expected).position(|(a,b)|a!=b){
        return Err(format!("NDS32 contiguous byte mismatch at file 0x{:x}",FILE_START+i));
    }
    if emitted.len()!=expected.len(){return Err("incorrect contiguous length".into());}
    Ok(())
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn formerly_disjoint_regions_have_zero_gap_and_correct_tables(){
        let src=build_postmarketos().unwrap();
        assert_eq!(src.len(),FILE_END-FILE_START);
        assert_eq!(&src[MOVD44_GAP_ONE-FILE_START..MOVD44_GAP_ONE-FILE_START+2],[0xfd,0x20]);
        assert_eq!(&src[MOVD44_GAP_TWO-FILE_START..MOVD44_GAP_TWO-FILE_START+2],[0xfd,0x36]);
        assert_eq!(&src[0x76e6-FILE_START..0x76e8-FILE_START],[0xfd,0x14]);
        assert_eq!(FILE_END,0x10000);
        assert_eq!(&src[CROSS_OWNER_WORD_FILE-FILE_START..],[0x44,0x62]);
        let whole=Insn::MoveImmediate{dest:Reg::new(6),value:131332}
            .encode((CROSS_OWNER_WORD_FILE-ILM_FILE_BASE) as u32,Profile::Postmarketos.gp()).unwrap();
        assert_eq!(whole,[0x44,0x62,0x01,0x04]);
        let mut fw=vec![0;FILE_END];
        fw[FILE_START..FILE_END].copy_from_slice(&src);
        assert!(compare_postmarketos(&fw).is_ok());
        fw[0x39f8]^=1;
        assert!(compare_postmarketos(&fw).is_err());
    }
}
