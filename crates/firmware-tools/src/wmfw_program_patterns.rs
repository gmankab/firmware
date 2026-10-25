// SPDX-License-Identifier: GPL-3.0-only
//! Conservative structural inspection of HALO 40-bit packed PM words.
//!
//! Equal words or repeating startup tuples are *patterns*, not proofs of
//! opcode meaning. This module does not disassemble HALO instruction words,
//! invent source code, or emit any vendor-derived payload into a firmware.
use crate::wmfw_halo_memory::{self,HaloMemoryKind};
use std::collections::BTreeMap;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct RepeatedTailPattern {
    pub initial_word:usize,
    pub entries:usize,
    pub entry_words:usize,
    pub repeated_tail_words:usize,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ProgramPatternSummary {
    pub total_words:usize,
    pub distinct_words:usize,
    pub all_zero_words:usize,
    pub most_common:Vec<(u64,usize)>,
    pub repeated_four_word_tails:Vec<RepeatedTailPattern>,
}

/// Scan for sequences of at least 3 four-word entries whose final two words
/// are identical across consecutive entries. Candidate startup-table shape,
/// not a proven call/branch/instruction-packet decoding.
pub fn repeated_four_word_tails(words:&[u64])->Vec<RepeatedTailPattern> {
    let mut result=Vec::new();
    let mut start=0;
    while start+12<=words.len() {
        let expected0=words[start+2];
        let expected1=words[start+3];
        let mut entries=1;
        while start+(entries+1)*4<=words.len() {
            let tail=start+entries*4+2;
            if words[tail]!=expected0||words[tail+1]!=expected1 {break;}
            entries+=1;
        }
        if entries>=3 {
            result.push(RepeatedTailPattern {
                initial_word:start,entries,entry_words:4,repeated_tail_words:2,
            });
            start+=entries*4;
        } else {start+=1;}
    }
    result
}

pub fn analyze_packed_program(payload:&[u8])->Result<ProgramPatternSummary,String>{
    let words=wmfw_halo_memory::unpack_words(HaloMemoryKind::PackedProgram,payload)?;
    let mut histogram:BTreeMap<u64,usize>=BTreeMap::new();
    let mut zero=0;
    for word in &words {
        *histogram.entry(*word).or_default()+=1;
        if *word==0 {zero+=1;}
    }
    let mut sorted:Vec<(u64,usize)>=histogram.iter().map(|(w,n)|(*w,*n)).collect();
    sorted.sort_by(|a,b|b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    sorted.truncate(10);
    Ok(ProgramPatternSummary {
        total_words:words.len(),distinct_words:histogram.len(),
        all_zero_words:zero,most_common:sorted,
        repeated_four_word_tails:repeated_four_word_tails(&words),
    })
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn finds_repeated_table_without_claiming_instruction_semantics() {
        let mut words=Vec::new();
        for n in 0..6 {
            words.extend_from_slice(&[0x120000+n,0x220000+n,0x334455,0x667788]);
        }
        words.extend_from_slice(&[0x42,0x70,0x11,0x22,0x33,0x44]);
        let matches=repeated_four_word_tails(&words);
        assert_eq!(matches,vec![RepeatedTailPattern {
            initial_word:0,entries:6,entry_words:4,repeated_tail_words:2,
        }]);
    }
    #[test]
    fn arbitrary_words_and_truncated_packed_memory_rejected() {
        let d=wmfw_halo_memory::pack_words(HaloMemoryKind::PackedProgram,&[0,1,2,3,0]).unwrap();
        let summary=analyze_packed_program(&d).unwrap();
        assert_eq!((summary.total_words,summary.distinct_words,summary.all_zero_words),(5,4,2));
        assert!(summary.repeated_four_word_tails.is_empty());
        assert!(analyze_packed_program(&[0,0,0,0]).is_err());
    }
}
