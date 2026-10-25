// SPDX-License-Identifier: GPL-3.0-only
//! Agent 17 typed instruction-level reconstruction of lower-ILM touchscreen status
//! and controller event helpers for the pinned postmarketOS firmware.
//! Every instruction below uses a named checked NDS32 operand/encoding;
//! firmware reference bytes are NEVER imported into the emitter.
//! Compact EX9 use references an independent table owned by Agent 18.
use super::{Located,Profile};
use super::isa::{Insn,Reg,Width,AluOp};
fn r(n:u8)->Reg {Reg::new(n)}

/// Increment a mode counter, update interface pointers and emit a synchronization callback.
fn configure_touch_report_state(out:&mut Vec<Located>){
    use Insn::*;
    out.push(Located{pc:0x942,instruction:MultipleWords{store:true,first:r(31),base:r(31),last:r(31),specials:10,decrement:true,writeback:true}});
    out.push(Located{pc:0x946,instruction:LoadWordGp{dest:r(0),address:0x2c5b0}});
    out.push(Located{pc:0x94a,instruction:AddSmall{reg:r(0),value:1,subtract:false}});
    out.push(Located{pc:0x94c,instruction:StoreWordGp{source:r(0),address:0x2c5b0}});
    out.push(Located{pc:0x950,instruction:Ex9(6)});
    out.push(Located{pc:0x952,instruction:BranchImmediate{reg:r(0),value:2,nonzero:true,target:0x9c6}});
    out.push(Located{pc:0x956,instruction:Ex9(0)});
    out.push(Located{pc:0x958,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(0),offset:1}});
    out.push(Located{pc:0x95a,instruction:BranchImmediate{reg:r(0),value:3,nonzero:true,target:0x9be}});
    out.push(Located{pc:0x95e,instruction:Ex9(27)});
    out.push(Located{pc:0x960,instruction:BranchImmediate{reg:r(0),value:2,nonzero:true,target:0x9be}});
    out.push(Located{pc:0x964,instruction:Ex9(2)});
    out.push(Located{pc:0x966,instruction:BranchZero{reg:r(0),nonzero:false,target:0x9c6}});
    out.push(Located{pc:0x968,instruction:LoadByteGp{dest:r(0),address:0x2c394}});
    out.push(Located{pc:0x96c,instruction:LessThanSmallImmediate{reg:r(0),value:3}});
    out.push(Located{pc:0x96e,instruction:BranchTa{nonzero:false,target:0x974}});
    out.push(Located{pc:0x970,instruction:AddSmall{reg:r(0),value:1,subtract:false}});
    out.push(Located{pc:0x972,instruction:JumpShort{target:0x976}});
    out.push(Located{pc:0x974,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0x976,instruction:StoreByteGp{source:r(0),address:0x2c394}});
    out.push(Located{pc:0x97a,instruction:LoadByteGp{dest:r(3),address:0x2c394}});
    out.push(Located{pc:0x97e,instruction:MoveImmediate{dest:r(0),value:191128}});
    out.push(Located{pc:0x982,instruction:Ex9(290)});
    out.push(Located{pc:0x984,instruction:AluShiftLeft{op:AluOp::Add,dest:r(0),left:r(0),right:r(3),shift:7}});
    out.push(Located{pc:0x988,instruction:Ex9(163)});
    out.push(Located{pc:0x98a,instruction:Ex9(126)});
    out.push(Located{pc:0x98c,instruction:Ex9(163)});
    out.push(Located{pc:0x98e,instruction:Ex9(126)});
    out.push(Located{pc:0x990,instruction:Ex9(163)});
    out.push(Located{pc:0x992,instruction:Ex9(126)});
    out.push(Located{pc:0x994,instruction:MultipleWords{store:false,first:r(16),base:r(0),last:r(17),specials:0,decrement:false,writeback:false}});
    out.push(Located{pc:0x998,instruction:MultipleWords{store:true,first:r(16),base:r(1),last:r(17),specials:0,decrement:false,writeback:false}});
    out.push(Located{pc:0x99c,instruction:BranchImmediate{reg:r(3),value:1,nonzero:true,target:0x9a8}});
    out.push(Located{pc:0x9a0,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0x9a2,instruction:Ex9(208)});
    out.push(Located{pc:0x9a4,instruction:Ex9(381)});
    out.push(Located{pc:0x9a6,instruction:JumpShort{target:0x9b8}});
    out.push(Located{pc:0x9a8,instruction:MoveSmall{dest:r(0),value:6}});
    out.push(Located{pc:0x9aa,instruction:Ex9(429)});
    out.push(Located{pc:0x9ac,instruction:MoveImmediate{dest:r(0),value:125}});
    out.push(Located{pc:0x9b0,instruction:Ex9(454)});
    out.push(Located{pc:0x9b2,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0x9b4,instruction:Ex9(208)});
    out.push(Located{pc:0x9b6,instruction:Ex9(340)});
    out.push(Located{pc:0x9b8,instruction:JumpRelative{target:0x922,link:true}});
    out.push(Located{pc:0x9bc,instruction:JumpShort{target:0x9c6}});
    out.push(Located{pc:0x9be,instruction:Ex9(2)});
    out.push(Located{pc:0x9c0,instruction:BranchZero{reg:r(0),nonzero:false,target:0x9c6}});
    out.push(Located{pc:0x9c2,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0x9c4,instruction:Ex9(306)});
    out.push(Located{pc:0x9c6,instruction:LoadByteGp{dest:r(0),address:0x2c702}});
    out.push(Located{pc:0x9ca,instruction:BranchImmediate{reg:r(0),value:1,nonzero:true,target:0x9ea}});
    out.push(Located{pc:0x9ce,instruction:LoadWordGp{dest:r(1),address:0x2c5b0}});
    out.push(Located{pc:0x9d2,instruction:LoadByteGp{dest:r(0),address:0x2c701}});
    out.push(Located{pc:0x9d6,instruction:LessThanSmallRegister{left:r(1),right:r(0)}});
    out.push(Located{pc:0x9d8,instruction:BranchTa{nonzero:false,target:0x9de}});
    out.push(Located{pc:0x9da,instruction:Ex9(340)});
    out.push(Located{pc:0x9dc,instruction:JumpShort{target:0x9ea}});
    out.push(Located{pc:0x9de,instruction:Ex9(381)});
    out.push(Located{pc:0x9e0,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0x9e2,instruction:Ex9(208)});
    out.push(Located{pc:0x9e4,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0x9e6,instruction:StoreByteGp{source:r(0),address:0x2c702}});
    out.push(Located{pc:0x9ea,instruction:MultipleWords{store:false,first:r(31),base:r(31),last:r(31),specials:10,decrement:false,writeback:true}});
    out.push(Located{pc:0x9ee,instruction:Return{reg:r(30)}});
}

/// Apply state flags, check wakeup conditions and return via saved LP.
fn update_suspend_resume_state(out:&mut Vec<Located>){
    use Insn::*;
    out.push(Located{pc:0x9f0,instruction:MultipleWords{store:true,first:r(31),base:r(31),last:r(31),specials:10,decrement:true,writeback:true}});
    out.push(Located{pc:0x9f4,instruction:LoadWordGp{dest:r(0),address:0x2c5c0}});
    out.push(Located{pc:0x9f8,instruction:AddSmall{reg:r(0),value:1,subtract:false}});
    out.push(Located{pc:0x9fa,instruction:StoreWordGp{source:r(0),address:0x2c5c0}});
    out.push(Located{pc:0x9fe,instruction:Ex9(0)});
    out.push(Located{pc:0xa00,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(0),offset:0}});
    out.push(Located{pc:0xa02,instruction:AddSmall{reg:r(0),value:1,subtract:true}});
    out.push(Located{pc:0xa04,instruction:LessThanSmallImmediate{reg:r(0),value:2}});
    out.push(Located{pc:0xa06,instruction:BranchTa{nonzero:false,target:0xa60}});
    out.push(Located{pc:0xa08,instruction:LoadByteGp{dest:r(0),address:0x2c3c2}});
    out.push(Located{pc:0xa0c,instruction:BranchImmediate{reg:r(0),value:85,nonzero:true,target:0xa20}});
    out.push(Located{pc:0xa10,instruction:HalfGp{store:false,signed:false,reg:r(0),address:0x2c434}});
    out.push(Located{pc:0xa14,instruction:AddSmall{reg:r(0),value:1,subtract:false}});
    out.push(Located{pc:0xa16,instruction:ZeroExtend{dest:r(0),source:r(0),half:true}});
    out.push(Located{pc:0xa18,instruction:LessThanSmallImmediate{reg:r(0),value:10}});
    out.push(Located{pc:0xa1a,instruction:Ex9(505)});
    out.push(Located{pc:0xa1c,instruction:BranchTa{nonzero:true,target:0xa56}});
    out.push(Located{pc:0xa1e,instruction:JumpShort{target:0xa2e}});
    out.push(Located{pc:0xa20,instruction:Ex9(2)});
    out.push(Located{pc:0xa22,instruction:BranchZero{reg:r(0),nonzero:false,target:0xa52}});
    out.push(Located{pc:0xa24,instruction:LoadByteGp{dest:r(0),address:0x2c3b8}});
    out.push(Located{pc:0xa28,instruction:BranchImmediate{reg:r(0),value:85,nonzero:true,target:0xa52}});
    out.push(Located{pc:0xa2c,instruction:JumpShort{target:0xa10}});
    out.push(Located{pc:0xa2e,instruction:MoveSmall{dest:r(0),value:4}});
    out.push(Located{pc:0xa30,instruction:Ex9(1)});
    out.push(Located{pc:0xa32,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xa34,instruction:StoreByteGp{source:r(0),address:0x2c752}});
    out.push(Located{pc:0xa38,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0xa3a,instruction:Ex9(40)});
    out.push(Located{pc:0xa3c,instruction:Ex9(0)});
    out.push(Located{pc:0xa3e,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(0),offset:0}});
    out.push(Located{pc:0xa40,instruction:BranchImmediate{reg:r(0),value:1,nonzero:true,target:0xa4a}});
    out.push(Located{pc:0xa44,instruction:Ex9(169)});
    out.push(Located{pc:0xa46,instruction:Ex9(89)});
    out.push(Located{pc:0xa48,instruction:JumpShort{target:0xa56}});
    out.push(Located{pc:0xa4a,instruction:AddSmall{reg:r(0),value:2,subtract:true}});
    out.push(Located{pc:0xa4c,instruction:LessThanSmallImmediate{reg:r(0),value:2}});
    out.push(Located{pc:0xa4e,instruction:BranchTa{nonzero:true,target:0xa4e}});
    out.push(Located{pc:0xa50,instruction:JumpShort{target:0xa56}});
    out.push(Located{pc:0xa52,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0xa54,instruction:Ex9(505)});
    out.push(Located{pc:0xa56,instruction:Ex9(48)});
    out.push(Located{pc:0xa58,instruction:StoreByteGp{source:r(0),address:0x2c3c2}});
    out.push(Located{pc:0xa5c,instruction:Ex9(149)});
    out.push(Located{pc:0xa5e,instruction:JumpShort{target:0xa64}});
    out.push(Located{pc:0xa60,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0xa62,instruction:Ex9(505)});
    out.push(Located{pc:0xa64,instruction:MultipleWords{store:false,first:r(31),base:r(31),last:r(31),specials:10,decrement:false,writeback:true}});
    out.push(Located{pc:0xa68,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xa6a,instruction:MultipleWords{store:true,first:r(6),base:r(31),last:r(6),specials:2,decrement:true,writeback:true}});
    out.push(Located{pc:0xa6e,instruction:MoveImmediate{dest:r(0),value:258155}});
    out.push(Located{pc:0xa72,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(0),offset:0}});
    out.push(Located{pc:0xa74,instruction:StoreByteGp{source:r(0),address:0x2c410}});
    out.push(Located{pc:0xa78,instruction:Ex9(6)});
    out.push(Located{pc:0xa7a,instruction:BranchZero{reg:r(0),nonzero:true,target:0xa86}});
    out.push(Located{pc:0xa7c,instruction:LoadByteGp{dest:r(0),address:0x2c410}});
    out.push(Located{pc:0xa80,instruction:BitFieldExtractImmediate{reg:r(0),bit:3}});
    out.push(Located{pc:0xa82,instruction:BranchImmediate{reg:r(0),value:15,nonzero:false,target:0xaae}});
    out.push(Located{pc:0xa86,instruction:MovePositive{dest:r(0),value:32}});
    out.push(Located{pc:0xa88,instruction:Ex9(1)});
    out.push(Located{pc:0xa8a,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xa8c,instruction:StoreWordGp{source:r(0),address:0x2c5d0}});
    out.push(Located{pc:0xa90,instruction:Ex9(415)});
    out.push(Located{pc:0xa92,instruction:JumpRelative{target:0xf662,link:true}});
    out.push(Located{pc:0xa96,instruction:Ex9(0)});
    out.push(Located{pc:0xa98,instruction:MoveSmall{dest:r(1),value:2}});
    out.push(Located{pc:0xa9a,instruction:Ex9(20)});
    out.push(Located{pc:0xa9c,instruction:JumpRelative{target:0xf696,link:true}});
    out.push(Located{pc:0xaa0,instruction:MoveSmall{dest:r(1),value:0}});
    out.push(Located{pc:0xaa2,instruction:MoveSmall{dest:r(0),value:2}});
    out.push(Located{pc:0xaa4,instruction:MoveSmall{dest:r(2),value:6}});
    out.push(Located{pc:0xaa6,instruction:Move{dest:r(3),source:r(1)}});
    out.push(Located{pc:0xaa8,instruction:Move{dest:r(4),source:r(1)}});
    out.push(Located{pc:0xaaa,instruction:Move{dest:r(5),source:r(1)}});
    out.push(Located{pc:0xaac,instruction:Ex9(23)});
    out.push(Located{pc:0xaae,instruction:Ex9(31)});
    out.push(Located{pc:0xab0,instruction:Return{reg:r(30)}});
}

/// Dispatch touchscreen status, error and reset callbacks.
fn dispatch_touch_controller_events(out:&mut Vec<Located>){
    use Insn::*;
    out.push(Located{pc:0xab2,instruction:MultipleWords{store:true,first:r(6),base:r(31),last:r(7),specials:10,decrement:true,writeback:true}});
    out.push(Located{pc:0xab6,instruction:JumpRelative{target:0xeb92,link:true}});
    out.push(Located{pc:0xaba,instruction:BranchZero{reg:r(0),nonzero:false,target:0xae8}});
    out.push(Located{pc:0xabc,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0xabe,instruction:JumpRelative{target:0xeb9a,link:true}});
    out.push(Located{pc:0xac2,instruction:InlineCall{target:0xb06}});
    out.push(Located{pc:0xac4,instruction:Move{dest:r(6),source:r(0)}});
    out.push(Located{pc:0xac6,instruction:BranchImmediate{reg:r(0),value:1,nonzero:true,target:0xae8}});
    out.push(Located{pc:0xaca,instruction:Ex9(6)});
    out.push(Located{pc:0xacc,instruction:BranchImmediate{reg:r(0),value:2,nonzero:true,target:0xad4}});
    out.push(Located{pc:0xad0,instruction:StoreWordGp{source:r(6),address:0x2c5b4}});
    out.push(Located{pc:0xad4,instruction:MoveImmediate{dest:r(0),value:128}});
    out.push(Located{pc:0xad8,instruction:Ex9(1)});
    out.push(Located{pc:0xada,instruction:Ex9(0)});
    out.push(Located{pc:0xadc,instruction:Ex9(348)});
    out.push(Located{pc:0xade,instruction:Ex9(224)});
    out.push(Located{pc:0xae0,instruction:Ex9(504)});
    out.push(Located{pc:0xae2,instruction:MoveSmall{dest:r(4),value:1}});
    out.push(Located{pc:0xae4,instruction:JumpRelative{target:0x52ea,link:true}});
    out.push(Located{pc:0xae8,instruction:JumpRelative{target:0xeb80,link:true}});
    out.push(Located{pc:0xaec,instruction:BranchZero{reg:r(0),nonzero:false,target:0xafa}});
    out.push(Located{pc:0xaee,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xaf0,instruction:StoreWordGp{source:r(0),address:0x2c600}});
    out.push(Located{pc:0xaf4,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0xaf6,instruction:JumpRelative{target:0xeb88,link:true}});
    out.push(Located{pc:0xafa,instruction:JumpRelative{target:0xebf2,link:true}});
    out.push(Located{pc:0xafe,instruction:BranchZero{reg:r(0),nonzero:false,target:0xb48}});
    out.push(Located{pc:0xb00,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0xb02,instruction:JumpRelative{target:0xebfa,link:true}});
    out.push(Located{pc:0xb06,instruction:JumpRelative{target:0xec98,link:true}});
    out.push(Located{pc:0xb0a,instruction:BranchImmediate{reg:r(0),value:1,nonzero:true,target:0xb48}});
    out.push(Located{pc:0xb0e,instruction:Ex9(132)});
    out.push(Located{pc:0xb10,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(0),offset:0}});
    out.push(Located{pc:0xb12,instruction:BranchImmediate{reg:r(0),value:19,nonzero:true,target:0xb2e}});
    out.push(Located{pc:0xb16,instruction:Ex9(18)});
    out.push(Located{pc:0xb18,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(1),offset:0}});
    out.push(Located{pc:0xb1a,instruction:AddSmall{reg:r(0),value:2,subtract:true}});
    out.push(Located{pc:0xb1c,instruction:LessThanSmallImmediate{reg:r(0),value:3}});
    out.push(Located{pc:0xb1e,instruction:BranchTa{nonzero:true,target:0xb26}});
    out.push(Located{pc:0xb20,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(1),offset:1}});
    out.push(Located{pc:0xb22,instruction:BranchImmediate{reg:r(0),value:8,nonzero:true,target:0xb2e}});
    out.push(Located{pc:0xb26,instruction:MoveSmall{dest:r(1),value:0}});
    out.push(Located{pc:0xb28,instruction:Ex9(132)});
    out.push(Located{pc:0xb2a,instruction:ShortMemory{store:true,width:Width::Byte,reg:r(1),base:r(0),offset:0}});
    out.push(Located{pc:0xb2c,instruction:JumpShort{target:0xb58}});
    out.push(Located{pc:0xb2e,instruction:MoveSmall{dest:r(6),value:1}});
    out.push(Located{pc:0xb30,instruction:MoveImmediate{dest:r(0),value:128}});
    out.push(Located{pc:0xb34,instruction:StoreWordGp{source:r(6),address:0x2c580}});
    out.push(Located{pc:0xb38,instruction:Ex9(1)});
    out.push(Located{pc:0xb3a,instruction:Ex9(0)});
    out.push(Located{pc:0xb3c,instruction:Ex9(348)});
    out.push(Located{pc:0xb3e,instruction:Ex9(224)});
    out.push(Located{pc:0xb40,instruction:Ex9(504)});
    out.push(Located{pc:0xb42,instruction:Move{dest:r(4),source:r(6)}});
    out.push(Located{pc:0xb44,instruction:JumpRelative{target:0x52ea,link:true}});
    out.push(Located{pc:0xb48,instruction:JumpRelative{target:0xebe0,link:true}});
    out.push(Located{pc:0xb4c,instruction:BranchZero{reg:r(0),nonzero:false,target:0xb58}});
    out.push(Located{pc:0xb4e,instruction:MoveSmall{dest:r(0),value:0}});
    out.push(Located{pc:0xb50,instruction:JumpRelative{target:0xebe8,link:true}});
    out.push(Located{pc:0xb54,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xb56,instruction:Ex9(444)});
    out.push(Located{pc:0xb58,instruction:MultipleWords{store:false,first:r(6),base:r(31),last:r(7),specials:10,decrement:false,writeback:true}});
    out.push(Located{pc:0xb5c,instruction:Return{reg:r(30)}});
}

/// Poll error and busy flags with guarded callback dispatch.
fn poll_input_error_status(out:&mut Vec<Located>){
    use Insn::*;
    out.push(Located{pc:0xb5e,instruction:MultipleWords{store:true,first:r(31),base:r(31),last:r(31),specials:10,decrement:true,writeback:true}});
    out.push(Located{pc:0xb62,instruction:Ex9(25)});
    out.push(Located{pc:0xb64,instruction:MoveSmall{dest:r(1),value:1}});
    out.push(Located{pc:0xb66,instruction:StoreWordGp{source:r(1),address:0x2c604}});
    out.push(Located{pc:0xb6a,instruction:ShortMemory{store:true,width:Width::Byte,reg:r(1),base:r(0),offset:0}});
    out.push(Located{pc:0xb6c,instruction:Ex9(61)});
    out.push(Located{pc:0xb6e,instruction:MoveImmediate{dest:r(2),value:232771}});
    out.push(Located{pc:0xb72,instruction:StoreByteGp{source:r(0),address:0x2c3c2}});
    out.push(Located{pc:0xb76,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(2),offset:0}});
    out.push(Located{pc:0xb78,instruction:AddSmallRegister{dest:r(0),source:r(1),subtract:false}});
    out.push(Located{pc:0xb7a,instruction:ShortMemory{store:true,width:Width::Byte,reg:r(0),base:r(2),offset:0}});
    out.push(Located{pc:0xb7c,instruction:Ex9(0)});
    out.push(Located{pc:0xb7e,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(2),base:r(0),offset:1}});
    out.push(Located{pc:0xb80,instruction:AndImmediate{dest:r(2),source:r(2),value:253}});
    out.push(Located{pc:0xb84,instruction:BranchImmediate{reg:r(2),value:4,nonzero:false,target:0xb8a}});
    out.push(Located{pc:0xb88,instruction:Ex9(20)});
    out.push(Located{pc:0xb8a,instruction:JumpRelative{target:0xf648,link:true}});
    out.push(Located{pc:0xb8e,instruction:BranchZero{reg:r(0),nonzero:false,target:0xba2}});
    out.push(Located{pc:0xb90,instruction:MoveSmall{dest:r(0),value:9}});
    out.push(Located{pc:0xb92,instruction:Ex9(1)});
    out.push(Located{pc:0xb94,instruction:MovePositive{dest:r(0),value:40}});
    out.push(Located{pc:0xb96,instruction:Ex9(57)});
    out.push(Located{pc:0xb98,instruction:MoveSmall{dest:r(0),value:3}});
    out.push(Located{pc:0xb9a,instruction:Ex9(154)});
    out.push(Located{pc:0xb9c,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xb9e,instruction:StoreByteGp{source:r(0),address:0x2c3e5}});
    out.push(Located{pc:0xba2,instruction:LoadByteGp{dest:r(0),address:0x2c6cb}});
    out.push(Located{pc:0xba6,instruction:AddSmall{reg:r(0),value:1,subtract:false}});
    out.push(Located{pc:0xba8,instruction:ZeroExtend{dest:r(0),source:r(0),half:false}});
    out.push(Located{pc:0xbaa,instruction:StoreByteGp{source:r(0),address:0x2c6cb}});
    out.push(Located{pc:0xbae,instruction:Ex9(2)});
    out.push(Located{pc:0xbb0,instruction:BranchZero{reg:r(0),nonzero:true,target:0xbb8}});
    out.push(Located{pc:0xbb2,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xbb4,instruction:StoreByteGp{source:r(0),address:0x2c6b8}});
    out.push(Located{pc:0xbb8,instruction:MultipleWords{store:false,first:r(31),base:r(31),last:r(31),specials:10,decrement:false,writeback:true}});
    out.push(Located{pc:0xbbc,instruction:Return{reg:r(30)}});
}

/// Set initial controller global flags and return after each small helper.
fn initialize_controller_flags(out:&mut Vec<Located>){
    use Insn::*;
    out.push(Located{pc:0xbbe,instruction:MultipleWords{store:true,first:r(6),base:r(31),last:r(6),specials:2,decrement:true,writeback:true}});
    out.push(Located{pc:0xbc2,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xbc4,instruction:StoreWordGp{source:r(0),address:0x2c614}});
    out.push(Located{pc:0xbc8,instruction:MovePositive{dest:r(0),value:33}});
    out.push(Located{pc:0xbca,instruction:Ex9(1)});
    out.push(Located{pc:0xbcc,instruction:MoveImmediate{dest:r(0),value:258178}});
    out.push(Located{pc:0xbd0,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(0),offset:0}});
    out.push(Located{pc:0xbd2,instruction:BranchZero{reg:r(0),nonzero:false,target:0xbd8}});
    out.push(Located{pc:0xbd4,instruction:Ex9(169)});
    out.push(Located{pc:0xbd6,instruction:Ex9(89)});
    out.push(Located{pc:0xbd8,instruction:Ex9(31)});
    out.push(Located{pc:0xbda,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xbdc,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xbde,instruction:StoreWordGp{source:r(0),address:0x2c5a8}});
    out.push(Located{pc:0xbe2,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xbe4,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xbe6,instruction:StoreWordGp{source:r(0),address:0x2c5d4}});
    out.push(Located{pc:0xbea,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xbec,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xbee,instruction:StoreWordGp{source:r(0),address:0x2c5c4}});
    out.push(Located{pc:0xbf2,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xbf4,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xbf6,instruction:StoreWordGp{source:r(0),address:0x2c5f4}});
    out.push(Located{pc:0xbfa,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xbfc,instruction:MultipleWords{store:true,first:r(6),base:r(31),last:r(6),specials:2,decrement:true,writeback:true}});
    out.push(Located{pc:0xc00,instruction:MovePositive{dest:r(0),value:34}});
    out.push(Located{pc:0xc02,instruction:Ex9(1)});
    out.push(Located{pc:0xc04,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xc06,instruction:StoreWordGp{source:r(0),address:0x2c5c8}});
    out.push(Located{pc:0xc0a,instruction:Ex9(31)});
    out.push(Located{pc:0xc0c,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xc0e,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xc10,instruction:StoreWordGp{source:r(0),address:0x2c5b8}});
    out.push(Located{pc:0xc14,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xc16,instruction:MultipleWords{store:true,first:r(6),base:r(31),last:r(6),specials:2,decrement:true,writeback:true}});
    out.push(Located{pc:0xc1a,instruction:MovePositive{dest:r(0),value:35}});
    out.push(Located{pc:0xc1c,instruction:Ex9(1)});
    out.push(Located{pc:0xc1e,instruction:Ex9(31)});
    out.push(Located{pc:0xc20,instruction:Return{reg:r(30)}});
    out.push(Located{pc:0xc22,instruction:MultipleWords{store:true,first:r(6),base:r(31),last:r(6),specials:2,decrement:true,writeback:true}});
    out.push(Located{pc:0xc26,instruction:MovePositive{dest:r(0),value:36}});
    out.push(Located{pc:0xc28,instruction:Ex9(1)});
    out.push(Located{pc:0xc2a,instruction:MoveSmall{dest:r(0),value:1}});
    out.push(Located{pc:0xc2c,instruction:StoreWordGp{source:r(0),address:0x2c590}});
    out.push(Located{pc:0xc30,instruction:Ex9(31)});
    out.push(Located{pc:0xc32,instruction:Return{reg:r(30)}});
}

/// PostmarketOS runtime [0x942,0xc34) == file [0xb42,0xe34).
/// Map220v's corresponding routines have a different layout and need a
/// separate, independently tested instruction list instead of shifted PCs.
pub fn instructions(profile:Profile)->Vec<Located>{
    if profile!=Profile::Postmarketos{return Vec::new();}
    let mut out=Vec::with_capacity(287);
    configure_touch_report_state(&mut out);
    update_suspend_resume_state(&mut out);
    dispatch_touch_controller_events(&mut out);
    poll_input_error_status(&mut out);
    initialize_controller_flags(&mut out);
    out
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn controller_functions_are_contiguous_at_natural_return_boundaries(){
        let output=instructions(Profile::Postmarketos);
        let mut pc=0x942;
        for item in &output {
            assert_eq!(item.pc,pc);
            pc+=item.instruction.encode(item.pc,Profile::Postmarketos.gp()).unwrap().len() as u32;
        }
        assert_eq!(pc,0xc34);
        assert_eq!(output.len(),287);
        assert_eq!(instructions(Profile::Map220v).len(),0);
    }
}
