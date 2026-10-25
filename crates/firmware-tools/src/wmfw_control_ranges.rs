// SPDX-License-Identifier: GPL-3.0-only
//! Independently describe the 57 named HALO/CSPL controls as relative DSP
//! XM/YM memory spans. WMFW control sizes are host-visible 32-bit slots; do
//! *not* confuse them with packed XM/PM wire-word lengths or absolute
//! algorithm base addresses supplied by the running firmware.

use crate::wmfw_algorithms::{self, Algorithm};
use std::collections::{BTreeMap,BTreeSet};
use std::ops::Range;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub enum ControlMemory { X, Y }
impl ControlMemory {
    pub fn from_wmfw_type(t:u16)->Result<Self,String> {
        match t {
            5=>Ok(Self::X),
            6=>Ok(Self::Y),
            _=>Err(format!("unrecognized HALO algorithm control memory type {t}")),
        }
    }
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ControlSlot {
    pub algorithm_id:u32,
    pub memory:ControlMemory,
    pub name:String,
    pub word_span:Range<u32>,
    pub flags:u16,
}

pub fn compile_control_slots(algorithms:&[Algorithm])->Result<Vec<ControlSlot>,String> {
    let mut slots=Vec::new();
    let mut ids=BTreeSet::new();
    for alg in algorithms {
        if !ids.insert(alg.id) {return Err(format!("duplicate HALO algorithm ID 0x{:x}",alg.id));}
        let mut known_names=BTreeSet::new();
        let mut per_mem:BTreeMap<ControlMemory,Vec<Range<u32>>>=BTreeMap::new();
        for control in &alg.controls {
            if !known_names.insert(control.name.as_str()) {
                return Err(format!("duplicate control name {} in algorithm {}",control.name,alg.name));
            }
            if control.name.is_empty()||control.full_name.is_empty() {
                return Err("HALO control name must be nonempty".into());
            }
            if control.control_type!=4 {return Err(format!("unexpected HALO control type for {}",control.name));}
            if control.flags & !0x8007 !=0 {
                return Err(format!("unsupported HALO control flags for {}",control.name));
            }
            if control.size_bytes==0 || control.size_bytes%4!=0 {
                return Err(format!("HALO control {} has non-32-bit-aligned byte size",control.name));
            }
            let memory=ControlMemory::from_wmfw_type(control.memory_type)?;
            let start=u32::from(control.offset);
            let end=start.checked_add(control.size_bytes/4)
                .ok_or_else(||format!("HALO control {} range overflow",control.name))?;
            let range=start..end;
            per_mem.entry(memory).or_default().push(range.clone());
            slots.push(ControlSlot {algorithm_id:alg.id,memory,name:control.name.clone(),
                word_span:range,flags:control.flags});
        }
        for (mem,spans) in &mut per_mem {
            spans.sort_by_key(|r|r.start);
            let mut end=0;
            for range in spans {
                if range.start<end {
                    return Err(format!("HALO algorithm {} {:?} controls overlap at 0x{:x}",
                        alg.name,mem,range.start));
                }
                end=range.end;
            }
        }
    }
    Ok(slots)
}

pub fn cspl_control_slots()->Result<Vec<ControlSlot>,String> {
    compile_control_slots(&[wmfw_algorithms::framework(),
        wmfw_algorithms::cspl(),wmfw_algorithms::event_logger()])
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct AlgorithmControlCoverage {
    pub algorithm_id:u32,
    pub x_control_words:u32,
    pub y_control_words:u32,
    pub x_highest_word:u32,
    pub y_highest_word:u32,
}

pub fn coverage() -> Result<Vec<AlgorithmControlCoverage>,String> {
    let slots=cspl_control_slots()?;
    let mut reports:BTreeMap<u32,AlgorithmControlCoverage>=BTreeMap::new();
    for slot in slots {
        let c=reports.entry(slot.algorithm_id).or_insert(AlgorithmControlCoverage {
            algorithm_id:slot.algorithm_id,x_control_words:0,y_control_words:0,
            x_highest_word:0,y_highest_word:0,
        });
        let width=slot.word_span.end-slot.word_span.start;
        match slot.memory {
            ControlMemory::X=>{
                c.x_control_words+=width;
                c.x_highest_word=c.x_highest_word.max(slot.word_span.end);
            }
            ControlMemory::Y=>{
                c.y_control_words+=width;
                c.y_highest_word=c.y_highest_word.max(slot.word_span.end);
            }
        }
    }
    Ok(reports.into_values().collect())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]
    fn all_fifty_seven_controls_have_nonoverlapping_memory_spans() {
        let slots=cspl_control_slots().unwrap();
        assert_eq!(slots.len(),57);
        assert_eq!(slots.iter().filter(|x|x.memory==ControlMemory::X).count(),56);
        assert_eq!(slots.iter().filter(|x|x.memory==ControlMemory::Y).count(),1);
        let reports=coverage().unwrap();
        assert_eq!(reports.len(),3);
        let framework=reports.iter().find(|x|x.algorithm_id==0x400a4).unwrap();
        assert_eq!(framework.x_highest_word,0xe);
        let cspl=reports.iter().find(|x|x.algorithm_id==0xcd).unwrap();
        assert_eq!(cspl.x_highest_word,0x4d);
        assert_eq!((cspl.y_highest_word,cspl.y_control_words),(100,100));
        let logger=reports.iter().find(|x|x.algorithm_id==0xf20b).unwrap();
        assert_eq!(logger.x_highest_word,0x4b);
    }
    #[test]
    fn rejects_bad_control_size_unknown_memory_duplicate_and_overlap() {
        let mut a=wmfw_algorithms::framework();
        a.controls[0].size_bytes=5;
        assert!(compile_control_slots(&[a.clone()]).unwrap_err().contains("aligned"));
        a.controls[0].size_bytes=4;
        a.controls[0].memory_type=0x99;
        assert!(compile_control_slots(&[a.clone()]).unwrap_err().contains("memory type"));
        a.controls[0].memory_type=5;
        a.controls[1].offset=0;
        assert!(compile_control_slots(&[a.clone()]).unwrap_err().contains("overlap"));
        a.controls[1].offset=1;
        a.controls[1].name=a.controls[0].name.clone();
        assert!(compile_control_slots(&[a.clone()]).unwrap_err().contains("duplicate control name"));
        assert!(compile_control_slots(&[wmfw_algorithms::framework(),wmfw_algorithms::framework()])
            .unwrap_err().contains("duplicate HALO algorithm ID"));
    }
}
