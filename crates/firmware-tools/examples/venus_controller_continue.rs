// SPDX-License-Identifier: GPL-3.0-only
//! Lightweight verifier for only the Agent 20-04 controller continuation.
//! Reference firmware is a private input, never bundled with this code.
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_55400 as initial;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_55800 as continuation_55800;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_56000 as continuation_56000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_56800 as continuation_56800;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_57000 as continuation_57000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_58000 as continuation_58000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_59000 as continuation_59000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_5a000 as continuation_5a000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_5b000 as continuation_5b000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_5c000 as continuation_5c000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_5d000 as continuation_5d000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_5e000 as continuation_5e000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_5f000 as continuation_5f000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_60000 as continuation_60000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_61000 as continuation_61000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_62000 as continuation_62000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_63000 as continuation_63000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_64000 as continuation_64000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_65000 as continuation_65000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_66000 as continuation_66000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_67000 as continuation_67000;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>> {
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2 || !["emit","verify"].contains(&args[0].as_str()) {
      return Err("usage: venus_controller_continue <emit output.bin|verify /private/venus.mbn>".into());
    }
    let intervals=[(initial::START,initial::fragment()?),(continuation_55800::START,continuation_55800::fragment()?),(continuation_56000::START,continuation_56000::fragment()?),(continuation_56800::START,continuation_56800::fragment()?),(continuation_57000::START,continuation_57000::fragment()?),(continuation_58000::START,continuation_58000::fragment()?),(continuation_59000::START,continuation_59000::fragment()?),(continuation_5a000::START,continuation_5a000::fragment()?),(continuation_5b000::START,continuation_5b000::fragment()?),(continuation_5c000::START,continuation_5c000::fragment()?),(continuation_5d000::START,continuation_5d000::fragment()?),(continuation_5e000::START,continuation_5e000::fragment()?),(continuation_5f000::START,continuation_5f000::fragment()?),(continuation_60000::START,continuation_60000::fragment()?),(continuation_61000::START,continuation_61000::fragment()?),(continuation_62000::START,continuation_62000::fragment()?),(continuation_63000::START,continuation_63000::fragment()?),(continuation_64000::START,continuation_64000::fragment()?),(continuation_65000::START,continuation_65000::fragment()?),(continuation_66000::START,continuation_66000::fragment()?),(continuation_67000::START,continuation_67000::fragment()?)];
    let mut next=initial::START;
    for (start,bytes) in &intervals {
       if *start!=next {return Err(format!("continuation gap at 0x{next:x}").into());}
       next+=bytes.len();
    }
    if next>0x68000 {return Err("Agent 20-04 reservation exceeded".into());}
    let mut all=Vec::new();
    for (_,bytes) in &intervals { all.extend_from_slice(bytes); }
    match args[0].as_str() {
      "emit"=>fs::write(&args[1],&all)?,
      "verify"=>{
        let reference=fs::read(&args[1])?;
        let start=initial::FILE_OFFSET;
        let expected=reference.get(start..start+all.len()).ok_or("reference too short")?;
        if let Some(idx)=all.iter().zip(expected).position(|(a,b)|a!=b) {
            return Err(format!("byte mismatch at ARM32 0x{:x}",initial::START+idx).into());
        }
      },
      _=>unreachable!(),
    }
    println!("Agent 04 {}: {} byte-identical source bytes at ARM32 0x{:x}..0x{:x}",args[0],all.len(),initial::START,next);
    Ok(())
}
