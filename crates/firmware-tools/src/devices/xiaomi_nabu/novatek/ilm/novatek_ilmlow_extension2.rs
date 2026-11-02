// SPDX-License-Identifier: GPL-3.0-only
//! Agent 17 postmarketOS-only lower ILM continuation: event and sensor data
//! flow, arithmetic and EX9-encoded shared helpers. All instructions are
//! generated from semantic fields through checked typed NDS32 encoders.
//! The private firmware reference is NOT an emitter input; branches and
//! mixed-width instruction offsets are explicitly modeled in the source.
use super::{Located,Profile};
use super::isa::{Insn,Reg,Width,AluOp,AluRegOp,CompactMisc};
use Insn::*;
const FILE_START:usize=0x1c1a;
const FILE_END:usize=0x23d0;

/// File range [0x1c1a,0x1e40), 198 NDS32 instructions.
fn post_event_flag_updates()->Vec<Located>{
    let r=Reg::new;
    vec![
        Located{pc:0x1a1a,instruction:MultiplyAddRounded32{dest:r(0),left:r(6),right:r(2)}}, // maddr32, 4 bytes
        Located{pc:0x1a1e,instruction:Ex9(78)}, // ex9.it, 2 bytes
        Located{pc:0x1a20,instruction:ShiftRightSmall{reg:r(0),amount:0}}, // srli45, 2 bytes
        Located{pc:0x1a22,instruction:Stack{pop:true,last:r(10),extra:8}}, // push25/pop25, 2 bytes
        Located{pc:0x1a24,instruction:Stack{pop:false,last:r(14),extra:8}}, // push25/pop25, 2 bytes
        Located{pc:0x1a26,instruction:WordAtRegister{store:false,reg:r(4),base:r(0)}}, // lwi450/swi450, 2 bytes
        Located{pc:0x1a28,instruction:BranchZero{reg:r(3),nonzero:false,target:0x1a4a}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1a2a,instruction:BranchZero{reg:r(4),nonzero:false,target:0x1a38}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1a2c,instruction:SubtractSmall3{dest:r(5),source:r(4),value:4}}, // subi333, 2 bytes
        Located{pc:0x1a2e,instruction:LessThanSmallImmediate{reg:r(5),value:2}}, // sltsi45/slti45, 2 bytes
        Located{pc:0x1a30,instruction:BranchTa{nonzero:true,target:0x1a38}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1a32,instruction:MoveSmall{dest:r(5),value:0}}, // movi55, 2 bytes
        Located{pc:0x1a34,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(5),base:r(0),offset:125}}, // memory immediate, 4 bytes
        Located{pc:0x1a38,instruction:LoadByteGp{dest:r(5),address:0x2c3d3}}, // lbi.gp, 4 bytes
        Located{pc:0x1a3c,instruction:LoadByteGp{dest:r(6),address:0x2c396}}, // lbi.gp, 4 bytes
        Located{pc:0x1a40,instruction:LessThanWide{dest:r(5),source:r(5),value:1,signed:false}}, // slti, 4 bytes
        Located{pc:0x1a44,instruction:LessThanWide{dest:r(9),source:r(6),value:1,signed:false}}, // slti, 4 bytes
        Located{pc:0x1a48,instruction:JumpShort{target:0x1a4e}}, // j8, 2 bytes
        Located{pc:0x1a4a,instruction:Move{dest:r(9),source:r(3)}}, // mov55, 2 bytes
        Located{pc:0x1a4c,instruction:Move{dest:r(5),source:r(3)}}, // mov55, 2 bytes
        Located{pc:0x1a4e,instruction:BranchZeroWide{reg:r(4),nonzero:false,target:0x2088}}, // beqz/bnez, 4 bytes
        Located{pc:0x1a52,instruction:AddSmall{reg:r(4),value:4,subtract:true}}, // addi45/subi45, 2 bytes
        Located{pc:0x1a54,instruction:LessThanSmallImmediate{reg:r(4),value:2}}, // sltsi45/slti45, 2 bytes
        Located{pc:0x1a56,instruction:BranchZeroWide{reg:r(15),nonzero:true,target:0x2088}}, // beqz/bnez, 4 bytes
        Located{pc:0x1a5a,instruction:Move{dest:r(6),source:r(0)}}, // mov55, 2 bytes
        Located{pc:0x1a5c,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(1),offset:4}}, // memory333, 2 bytes
        Located{pc:0x1a5e,instruction:Move{dest:r(10),source:r(3)}}, // mov55, 2 bytes
        Located{pc:0x1a60,instruction:Move{dest:r(11),source:r(2)}}, // mov55, 2 bytes
        Located{pc:0x1a62,instruction:Move{dest:r(7),source:r(1)}}, // mov55, 2 bytes
        Located{pc:0x1a64,instruction:BranchZeroWide{reg:r(0),nonzero:true,target:0x1bc2}}, // beqz/bnez, 4 bytes
        Located{pc:0x1a68,instruction:BranchZeroWide{reg:r(5),nonzero:true,target:0x1bc2}}, // beqz/bnez, 4 bytes
        Located{pc:0x1a6c,instruction:Ex9(404)}, // ex9.it, 2 bytes
        Located{pc:0x1a6e,instruction:BranchZero{reg:r(3),nonzero:false,target:0x1ac0}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1a70,instruction:ShortMemory{store:false,width:Width::Word,reg:r(3),base:r(2),offset:4}}, // memory333, 2 bytes
        Located{pc:0x1a72,instruction:LoadWordGp{dest:r(5),address:0x2c508}}, // lwi.gp/swi.gp, 4 bytes
        Located{pc:0x1a76,instruction:ShortMemory{store:false,width:Width::Half,reg:r(2),base:r(3),offset:0}}, // memory333, 2 bytes
        Located{pc:0x1a78,instruction:ShortMemory{store:false,width:Width::Half,reg:r(4),base:r(3),offset:2}}, // memory333, 2 bytes
        Located{pc:0x1a7a,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(3),offset:98}}, // memory immediate, 4 bytes
        Located{pc:0x1a7e,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(3),base:r(3),offset:96}}, // memory immediate, 4 bytes
        Located{pc:0x1a82,instruction:LessThanWide{dest:r(15),source:r(5),value:800,signed:false}}, // slti, 4 bytes
        Located{pc:0x1a86,instruction:SignExtendHalf{dest:r(2),source:r(2)}}, // seh33, 2 bytes
        Located{pc:0x1a88,instruction:SignExtendHalf{dest:r(4),source:r(4)}}, // seh33, 2 bytes
        Located{pc:0x1a8a,instruction:SignExtendHalf{dest:r(0),source:r(0)}}, // seh33, 2 bytes
        Located{pc:0x1a8c,instruction:SignExtendHalf{dest:r(3),source:r(3)}}, // seh33, 2 bytes
        Located{pc:0x1a8e,instruction:BranchTa{nonzero:false,target:0x1a9c}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1a90,instruction:Ex9(406)}, // ex9.it, 2 bytes
        Located{pc:0x1a92,instruction:BranchImmediate{reg:r(5),value:1,nonzero:true,target:0x1a9c}}, // beqc/bnec, 4 bytes
        Located{pc:0x1a96,instruction:AddImmediate{dest:r(1),source:r(1),value:1296}}, // addi, 4 bytes
        Located{pc:0x1a9a,instruction:InlineCall{target:0x1abc}}, // ifcall9, 2 bytes
        Located{pc:0x1a9c,instruction:ImmediateMemory{store:false,width:Width::Byte,reg:r(5),base:r(7),offset:8}}, // memory immediate, 4 bytes
        Located{pc:0x1aa0,instruction:BranchZero{reg:r(5),nonzero:true,target:0x1aa8}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1aa2,instruction:AddSmallRegister{dest:r(2),source:r(4),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x1aa4,instruction:Ex9(459)}, // ex9.it, 2 bytes
        Located{pc:0x1aa6,instruction:JumpShort{target:0x1ab0}}, // j8, 2 bytes
        Located{pc:0x1aa8,instruction:BranchImmediate{reg:r(5),value:49,nonzero:true,target:0x1ab8}}, // beqc/bnec, 4 bytes
        Located{pc:0x1aac,instruction:AddSmallRegister{dest:r(0),source:r(3),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x1aae,instruction:Ex9(280)}, // ex9.it, 2 bytes
        Located{pc:0x1ab0,instruction:BranchTa{nonzero:true,target:0x1ab8}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1ab2,instruction:MoveSmall{dest:r(0),value:1}}, // movi55, 2 bytes
        Located{pc:0x1ab4,instruction:Ex9(416)}, // ex9.it, 2 bytes
        Located{pc:0x1ab6,instruction:InlineCall{target:0x1abc}}, // ifcall9, 2 bytes
        Located{pc:0x1ab8,instruction:AddImmediate{dest:r(1),source:r(1),value:648}}, // addi, 4 bytes
        Located{pc:0x1abc,instruction:JumpRelative{target:0x20a0,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1ac0,instruction:AddImmediate{dest:r(1),source:r(1),value:2118}}, // addi, 4 bytes
        Located{pc:0x1ac4,instruction:AddImmediate{dest:r(3),source:r(6),value:110}}, // addi, 4 bytes
        Located{pc:0x1ac8,instruction:MoveSmall{dest:r(2),value:0}}, // movi55, 2 bytes
        Located{pc:0x1aca,instruction:Move{dest:r(0),source:r(11)}}, // mov55, 2 bytes
        Located{pc:0x1acc,instruction:Move{dest:r(4),source:r(2)}}, // mov55, 2 bytes
        Located{pc:0x1ace,instruction:Ex9(506)}, // ex9.it, 2 bytes
        Located{pc:0x1ad0,instruction:HalfGp{store:false,signed:false,reg:r(12),address:0x19478}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1ad4,instruction:WordAtStack{store:true,reg:r(0),offset:4}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x1ad6,instruction:HalfGp{store:false,signed:false,reg:r(8),address:0x194b8}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1ada,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x1b68}}, // beqz/bnez, 4 bytes
        Located{pc:0x1ade,instruction:WordAtRegister{store:false,reg:r(1),base:r(11)}}, // lwi450/swi450, 2 bytes
        Located{pc:0x1ae0,instruction:AddressFromStack{dest:r(0),offset:4}}, // addri36.sp, 2 bytes
        Located{pc:0x1ae2,instruction:AddSmall3{dest:r(2),source:r(1),value:2}}, // addi333, 2 bytes
        Located{pc:0x1ae4,instruction:AddImmediate{dest:r(3),source:r(7),value:16}}, // addi, 4 bytes
        Located{pc:0x1ae8,instruction:AddImmediate{dest:r(4),source:r(7),value:40}}, // addi, 4 bytes
        Located{pc:0x1aec,instruction:Move{dest:r(5),source:r(12)}}, // mov55, 2 bytes
        Located{pc:0x1aee,instruction:Ex9(497)}, // ex9.it, 2 bytes
        Located{pc:0x1af0,instruction:ShortMemory{store:false,width:Width::Word,reg:r(0),base:r(7),offset:20}}, // memory333, 2 bytes
        Located{pc:0x1af2,instruction:ShortMemory{store:false,width:Width::Word,reg:r(5),base:r(7),offset:16}}, // memory333, 2 bytes
        Located{pc:0x1af4,instruction:ShiftLeftSmall3{dest:r(1),source:r(0),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1af6,instruction:RegisterAlu{op:AluRegOp::Sub,dest:r(4),left:r(12),right:r(8),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1afa,instruction:OrShiftRight{dest:r(2),left:r(1),right:r(5),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1afe,instruction:ShiftLeftSmall3{dest:r(1),source:r(5),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1b00,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(5),left:r(12),right:r(4),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1b04,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(16),base:r(31),offset:4}}, // memory immediate, 4 bytes
        Located{pc:0x1b08,instruction:CompactMiscOp{dest:r(5),source:r(5),op:CompactMisc::Neg}}, // misc33, 2 bytes
        Located{pc:0x1b0a,instruction:ShiftLeftSmall3{dest:r(5),source:r(5),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1b0c,instruction:ArithmeticShiftRight{dest:r(17),source:r(16),amount:31}}, // srai, 4 bytes
        Located{pc:0x1b10,instruction:Ex9(412)}, // ex9.it, 2 bytes
        Located{pc:0x1b12,instruction:ShiftLeftSmall3{dest:r(4),source:r(4),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1b14,instruction:MultiplyWide{dest:r(0),left:r(17),right:r(4)}}, // mul, 4 bytes
        Located{pc:0x1b18,instruction:MultiplyAddRounded32{dest:r(0),left:r(5),right:r(16)}}, // maddr32, 4 bytes
        Located{pc:0x1b1c,instruction:MultiplyRegister64{dest:r(4),left:r(4),right:r(16),signed:false}}, // mulr64, 4 bytes
        Located{pc:0x1b20,instruction:AddSmallRegister{dest:r(5),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1b22,instruction:CompactAlu3{dest:r(0),left:r(1),right:r(4),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1b24,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(3),left:r(0),right:r(1),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1b28,instruction:CompactAlu3{dest:r(1),left:r(2),right:r(5),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1b2a,instruction:Ex9(478)}, // ex9.it, 2 bytes
        Located{pc:0x1b2c,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(5),base:r(7),offset:44}}, // memory immediate, 4 bytes
        Located{pc:0x1b30,instruction:AddSmallRegister{dest:r(2),source:r(16),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1b32,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(4),left:r(2),right:r(16),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1b36,instruction:AddSmallRegister{dest:r(5),source:r(17),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1b38,instruction:AddSmallRegister{dest:r(1),source:r(3),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1b3a,instruction:CompactAlu3{dest:r(3),left:r(4),right:r(5),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1b3c,instruction:Ex9(42)}, // ex9.it, 2 bytes
        Located{pc:0x1b3e,instruction:ImmediateMemory{store:true,width:Width::Half,reg:r(0),base:r(6),offset:16}}, // memory immediate, 4 bytes
        Located{pc:0x1b42,instruction:Ex9(47)}, // ex9.it, 2 bytes
        Located{pc:0x1b44,instruction:BranchZeroWide{reg:r(0),nonzero:true,target:0x1cd6}}, // beqz/bnez, 4 bytes
        Located{pc:0x1b48,instruction:Ex9(326)}, // ex9.it, 2 bytes
        Located{pc:0x1b4a,instruction:BranchZero{reg:r(0),nonzero:false,target:0x1b52}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1b4c,instruction:MoveImmediate{dest:r(15),value:700}}, // movi, 4 bytes
        Located{pc:0x1b50,instruction:JumpShort{target:0x1b56}}, // j8, 2 bytes
        Located{pc:0x1b52,instruction:MoveImmediate{dest:r(15),value:1300}}, // movi, 4 bytes
        Located{pc:0x1b56,instruction:Ex9(490)}, // ex9.it, 2 bytes
        Located{pc:0x1b58,instruction:LessThanSmallRegister{left:r(0),right:r(15)}}, // slt45, 2 bytes
        Located{pc:0x1b5a,instruction:BranchZeroWide{reg:r(15),nonzero:false,target:0x1cd6}}, // beqz/bnez, 4 bytes
        Located{pc:0x1b5e,instruction:Ex9(402)}, // ex9.it, 2 bytes
        Located{pc:0x1b60,instruction:BranchZeroWide{reg:r(0),nonzero:true,target:0x1cd6}}, // beqz/bnez, 4 bytes
        Located{pc:0x1b64,instruction:JumpRelative{target:0x1cd2,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1b68,instruction:ShortMemory{store:false,width:Width::Word,reg:r(5),base:r(7),offset:20}}, // memory333, 2 bytes
        Located{pc:0x1b6a,instruction:ShortMemory{store:false,width:Width::Word,reg:r(1),base:r(7),offset:16}}, // memory333, 2 bytes
        Located{pc:0x1b6c,instruction:ShiftLeftSmall3{dest:r(5),source:r(5),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1b6e,instruction:HalfGp{store:false,signed:true,reg:r(2),address:0x2c0dc}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1b72,instruction:OrShiftRight{dest:r(16),left:r(5),right:r(1),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1b76,instruction:ZeroExtendWide{dest:r(5),source:r(12)}}, // zeh, 4 bytes
        Located{pc:0x1b7a,instruction:RegisterAlu{op:AluRegOp::Sub,dest:r(4),left:r(5),right:r(8),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1b7e,instruction:CompactMiscOp{dest:r(2),source:r(0),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x1b80,instruction:HalfGp{store:false,signed:true,reg:r(3),address:0x2c0e4}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1b84,instruction:LessThanSmallRegister{left:r(5),right:r(4)}}, // slt45, 2 bytes
        Located{pc:0x1b86,instruction:Ex9(172)}, // ex9.it, 2 bytes
        Located{pc:0x1b88,instruction:ReverseSubtractImmediate{dest:r(5),source:r(15),value:0}}, // subri, 4 bytes
        Located{pc:0x1b8c,instruction:ShiftLeftSmall3{dest:r(5),source:r(5),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1b8e,instruction:Ex9(412)}, // ex9.it, 2 bytes
        Located{pc:0x1b90,instruction:Ex9(480)}, // ex9.it, 2 bytes
        Located{pc:0x1b92,instruction:ShiftLeftSmall3{dest:r(4),source:r(4),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1b94,instruction:CompactMiscOp{dest:r(3),source:r(4),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x1b96,instruction:MultiplyAddRounded32{dest:r(3),left:r(5),right:r(2)}}, // maddr32, 4 bytes
        Located{pc:0x1b9a,instruction:MultiplyRegister64{dest:r(4),left:r(4),right:r(2),signed:false}}, // mulr64, 4 bytes
        Located{pc:0x1b9e,instruction:Ex9(478)}, // ex9.it, 2 bytes
        Located{pc:0x1ba0,instruction:ShiftLeftSmall3{dest:r(1),source:r(1),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1ba2,instruction:AddSmallRegister{dest:r(5),source:r(3),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1ba4,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(3),base:r(7),offset:44}}, // memory immediate, 4 bytes
        Located{pc:0x1ba8,instruction:AluShiftLeft{op:AluOp::Add,dest:r(17),left:r(1),right:r(4),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x1bac,instruction:AddSmallRegister{dest:r(2),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1bae,instruction:AddSmallRegister{dest:r(5),source:r(16),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1bb0,instruction:ArithmeticShiftRight{dest:r(16),source:r(0),amount:31}}, // srai, 4 bytes
        Located{pc:0x1bb4,instruction:LessThanSmallRegister{left:r(2),right:r(0)}}, // slt45, 2 bytes
        Located{pc:0x1bb6,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(1),left:r(17),right:r(1),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1bba,instruction:AddSmallRegister{dest:r(3),source:r(16),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1bbc,instruction:Move{dest:r(0),source:r(17)}}, // mov55, 2 bytes
        Located{pc:0x1bbe,instruction:JumpRelative{target:0x1d30,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1bc2,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(7),offset:5}}, // memory333, 2 bytes
        Located{pc:0x1bc4,instruction:BranchImmediate{reg:r(0),value:31,nonzero:false,target:0x1bcc}}, // beqc/bnec, 4 bytes
        Located{pc:0x1bc8,instruction:JumpRelative{target:0x1d38,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1bcc,instruction:BranchZeroWide{reg:r(5),nonzero:true,target:0x1d38}}, // beqz/bnez, 4 bytes
        Located{pc:0x1bd0,instruction:Ex9(404)}, // ex9.it, 2 bytes
        Located{pc:0x1bd2,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x1c2c}}, // beqz/bnez, 4 bytes
        Located{pc:0x1bd6,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(3),base:r(11),offset:4}}, // memory immediate, 4 bytes
        Located{pc:0x1bda,instruction:LoadWordGp{dest:r(5),address:0x2c508}}, // lwi.gp/swi.gp, 4 bytes
        Located{pc:0x1bde,instruction:ShortMemory{store:false,width:Width::Half,reg:r(2),base:r(3),offset:0}}, // memory333, 2 bytes
        Located{pc:0x1be0,instruction:ShortMemory{store:false,width:Width::Half,reg:r(4),base:r(3),offset:2}}, // memory333, 2 bytes
        Located{pc:0x1be2,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(3),offset:98}}, // memory immediate, 4 bytes
        Located{pc:0x1be6,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(3),base:r(3),offset:96}}, // memory immediate, 4 bytes
        Located{pc:0x1bea,instruction:LessThanWide{dest:r(15),source:r(5),value:800,signed:false}}, // slti, 4 bytes
        Located{pc:0x1bee,instruction:SignExtendHalf{dest:r(2),source:r(2)}}, // seh33, 2 bytes
        Located{pc:0x1bf0,instruction:SignExtendHalf{dest:r(4),source:r(4)}}, // seh33, 2 bytes
        Located{pc:0x1bf2,instruction:SignExtendHalf{dest:r(0),source:r(0)}}, // seh33, 2 bytes
        Located{pc:0x1bf4,instruction:SignExtendHalf{dest:r(3),source:r(3)}}, // seh33, 2 bytes
        Located{pc:0x1bf6,instruction:BranchTa{nonzero:false,target:0x1c04}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1bf8,instruction:Ex9(406)}, // ex9.it, 2 bytes
        Located{pc:0x1bfa,instruction:BranchImmediate{reg:r(5),value:1,nonzero:true,target:0x1c04}}, // beqc/bnec, 4 bytes
        Located{pc:0x1bfe,instruction:AddImmediate{dest:r(1),source:r(1),value:1458}}, // addi, 4 bytes
        Located{pc:0x1c02,instruction:InlineCall{target:0x1c28}}, // ifcall9, 2 bytes
        Located{pc:0x1c04,instruction:ImmediateMemory{store:false,width:Width::Byte,reg:r(5),base:r(7),offset:8}}, // memory immediate, 4 bytes
        Located{pc:0x1c08,instruction:BranchZero{reg:r(5),nonzero:true,target:0x1c10}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1c0a,instruction:AddSmallRegister{dest:r(2),source:r(4),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x1c0c,instruction:Ex9(459)}, // ex9.it, 2 bytes
        Located{pc:0x1c0e,instruction:JumpShort{target:0x1c18}}, // j8, 2 bytes
        Located{pc:0x1c10,instruction:BranchImmediate{reg:r(5),value:49,nonzero:true,target:0x1c24}}, // beqc/bnec, 4 bytes
        Located{pc:0x1c14,instruction:AddSmallRegister{dest:r(0),source:r(3),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x1c16,instruction:Ex9(280)}, // ex9.it, 2 bytes
        Located{pc:0x1c18,instruction:BranchTa{nonzero:true,target:0x1c24}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1c1a,instruction:MoveSmall{dest:r(0),value:1}}, // movi55, 2 bytes
        Located{pc:0x1c1c,instruction:AddImmediate{dest:r(1),source:r(1),value:162}}, // addi, 4 bytes
        Located{pc:0x1c20,instruction:Ex9(416)}, // ex9.it, 2 bytes
        Located{pc:0x1c22,instruction:InlineCall{target:0x1c28}}, // ifcall9, 2 bytes
        Located{pc:0x1c24,instruction:AddImmediate{dest:r(1),source:r(1),value:810}}, // addi, 4 bytes
        Located{pc:0x1c28,instruction:JumpRelative{target:0x20a8,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1c2c,instruction:AddImmediate{dest:r(1),source:r(1),value:1956}}, // addi, 4 bytes
        Located{pc:0x1c30,instruction:AddImmediate{dest:r(3),source:r(6),value:110}}, // addi, 4 bytes
        Located{pc:0x1c34,instruction:Move{dest:r(0),source:r(11)}}, // mov55, 2 bytes
        Located{pc:0x1c36,instruction:MoveSmall{dest:r(2),value:1}}, // movi55, 2 bytes
        Located{pc:0x1c38,instruction:MoveSmall{dest:r(4),value:0}}, // movi55, 2 bytes
        Located{pc:0x1c3a,instruction:Ex9(506)}, // ex9.it, 2 bytes
        Located{pc:0x1c3c,instruction:HalfGp{store:false,signed:false,reg:r(12),address:0x194b6}}, // lhi.gp/shi.gp, 4 bytes
    ]
}
/// File range [0x1e40,0x20a0), 213 NDS32 instructions.
fn status_and_sample_dispatch()->Vec<Located>{
    let r=Reg::new;
    vec![
        Located{pc:0x1c40,instruction:WordAtStack{store:true,reg:r(0),offset:4}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x1c42,instruction:HalfGp{store:false,signed:false,reg:r(8),address:0x194f6}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1c46,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x1cdc}}, // beqz/bnez, 4 bytes
        Located{pc:0x1c4a,instruction:WordAtRegister{store:false,reg:r(2),base:r(11)}}, // lwi450/swi450, 2 bytes
        Located{pc:0x1c4c,instruction:AddressFromStack{dest:r(0),offset:4}}, // addri36.sp, 2 bytes
        Located{pc:0x1c4e,instruction:AddImmediate{dest:r(1),source:r(2),value:62}}, // addi, 4 bytes
        Located{pc:0x1c52,instruction:AddImmediate{dest:r(3),source:r(7),value:16}}, // addi, 4 bytes
        Located{pc:0x1c56,instruction:AddImmediate{dest:r(2),source:r(2),value:60}}, // addi, 4 bytes
        Located{pc:0x1c5a,instruction:AddImmediate{dest:r(4),source:r(7),value:40}}, // addi, 4 bytes
        Located{pc:0x1c5e,instruction:Move{dest:r(5),source:r(12)}}, // mov55, 2 bytes
        Located{pc:0x1c60,instruction:Ex9(497)}, // ex9.it, 2 bytes
        Located{pc:0x1c62,instruction:AluShiftLeft{op:AluOp::Add,dest:r(4),left:r(12),right:r(8),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x1c66,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(16),base:r(31),offset:4}}, // memory immediate, 4 bytes
        Located{pc:0x1c6a,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(5),left:r(4),right:r(12),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1c6e,instruction:ShortMemory{store:false,width:Width::Word,reg:r(0),base:r(7),offset:20}}, // memory333, 2 bytes
        Located{pc:0x1c70,instruction:ShiftLeftSmall3{dest:r(5),source:r(5),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1c72,instruction:ArithmeticShiftRight{dest:r(17),source:r(16),amount:31}}, // srai, 4 bytes
        Located{pc:0x1c76,instruction:Ex9(412)}, // ex9.it, 2 bytes
        Located{pc:0x1c78,instruction:ShiftLeftSmall3{dest:r(4),source:r(4),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1c7a,instruction:ShortMemory{store:false,width:Width::Word,reg:r(3),base:r(7),offset:16}}, // memory333, 2 bytes
        Located{pc:0x1c7c,instruction:ShiftLeftSmall3{dest:r(1),source:r(0),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1c7e,instruction:MultiplyWide{dest:r(0),left:r(17),right:r(4)}}, // mul, 4 bytes
        Located{pc:0x1c82,instruction:MultiplyAddRounded32{dest:r(0),left:r(5),right:r(16)}}, // maddr32, 4 bytes
        Located{pc:0x1c86,instruction:MultiplyRegister64{dest:r(4),left:r(4),right:r(16),signed:false}}, // mulr64, 4 bytes
        Located{pc:0x1c8a,instruction:OrShiftRight{dest:r(2),left:r(1),right:r(3),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1c8e,instruction:ShiftLeftSmall3{dest:r(1),source:r(3),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1c90,instruction:AddSmallRegister{dest:r(5),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1c92,instruction:CompactAlu3{dest:r(0),left:r(1),right:r(4),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1c94,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(3),left:r(0),right:r(1),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1c98,instruction:CompactAlu3{dest:r(1),left:r(2),right:r(5),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1c9a,instruction:Ex9(478)}, // ex9.it, 2 bytes
        Located{pc:0x1c9c,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(5),base:r(7),offset:44}}, // memory immediate, 4 bytes
        Located{pc:0x1ca0,instruction:AddSmallRegister{dest:r(2),source:r(16),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1ca2,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(4),left:r(2),right:r(16),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1ca6,instruction:AddSmallRegister{dest:r(5),source:r(17),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1ca8,instruction:AddSmallRegister{dest:r(1),source:r(3),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1caa,instruction:CompactAlu3{dest:r(3),left:r(4),right:r(5),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1cac,instruction:Ex9(42)}, // ex9.it, 2 bytes
        Located{pc:0x1cae,instruction:ImmediateMemory{store:true,width:Width::Half,reg:r(0),base:r(6),offset:16}}, // memory immediate, 4 bytes
        Located{pc:0x1cb2,instruction:Ex9(47)}, // ex9.it, 2 bytes
        Located{pc:0x1cb4,instruction:BranchZero{reg:r(0),nonzero:true,target:0x1cd6}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1cb6,instruction:Ex9(326)}, // ex9.it, 2 bytes
        Located{pc:0x1cb8,instruction:BranchZero{reg:r(0),nonzero:false,target:0x1cc0}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1cba,instruction:MoveImmediate{dest:r(15),value:400}}, // movi, 4 bytes
        Located{pc:0x1cbe,instruction:JumpShort{target:0x1cc4}}, // j8, 2 bytes
        Located{pc:0x1cc0,instruction:MoveImmediate{dest:r(15),value:1000}}, // movi, 4 bytes
        Located{pc:0x1cc4,instruction:Ex9(490)}, // ex9.it, 2 bytes
        Located{pc:0x1cc6,instruction:LessThanSmallRegister{left:r(0),right:r(15)}}, // slt45, 2 bytes
        Located{pc:0x1cc8,instruction:BranchTa{nonzero:false,target:0x1cd6}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1cca,instruction:Ex9(402)}, // ex9.it, 2 bytes
        Located{pc:0x1ccc,instruction:BranchZero{reg:r(0),nonzero:true,target:0x1cd6}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1cce,instruction:MoveImmediate{dest:r(0),value:3199}}, // movi, 4 bytes
        Located{pc:0x1cd2,instruction:ImmediateMemory{store:true,width:Width::Half,reg:r(0),base:r(6),offset:16}}, // memory immediate, 4 bytes
        Located{pc:0x1cd6,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(6),offset:16}}, // memory immediate, 4 bytes
        Located{pc:0x1cda,instruction:JumpShort{target:0x1d4a}}, // j8, 2 bytes
        Located{pc:0x1cdc,instruction:ShortMemory{store:false,width:Width::Word,reg:r(5),base:r(7),offset:20}}, // memory333, 2 bytes
        Located{pc:0x1cde,instruction:ShortMemory{store:false,width:Width::Word,reg:r(1),base:r(7),offset:16}}, // memory333, 2 bytes
        Located{pc:0x1ce0,instruction:HalfGp{store:false,signed:true,reg:r(2),address:0x2c0de}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1ce4,instruction:ShiftLeftSmall3{dest:r(5),source:r(5),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1ce6,instruction:OrShiftRight{dest:r(17),left:r(5),right:r(1),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1cea,instruction:CompactMiscOp{dest:r(2),source:r(0),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x1cec,instruction:ZeroExtendWide{dest:r(5),source:r(12)}}, // zeh, 4 bytes
        Located{pc:0x1cf0,instruction:HalfGp{store:false,signed:true,reg:r(3),address:0x2c0e4}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1cf4,instruction:AluShiftLeft{op:AluOp::Add,dest:r(4),left:r(5),right:r(8),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x1cf8,instruction:Ex9(172)}, // ex9.it, 2 bytes
        Located{pc:0x1cfa,instruction:LessThanSmallRegister{left:r(4),right:r(5)}}, // slt45, 2 bytes
        Located{pc:0x1cfc,instruction:ShiftLeftImmediate{dest:r(5),source:r(15),amount:1}}, // slli, 4 bytes
        Located{pc:0x1d00,instruction:Ex9(412)}, // ex9.it, 2 bytes
        Located{pc:0x1d02,instruction:Ex9(480)}, // ex9.it, 2 bytes
        Located{pc:0x1d04,instruction:ShiftLeftSmall3{dest:r(4),source:r(4),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1d06,instruction:CompactMiscOp{dest:r(3),source:r(4),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x1d08,instruction:MultiplyAddRounded32{dest:r(3),left:r(5),right:r(2)}}, // maddr32, 4 bytes
        Located{pc:0x1d0c,instruction:MultiplyRegister64{dest:r(4),left:r(4),right:r(2),signed:false}}, // mulr64, 4 bytes
        Located{pc:0x1d10,instruction:Ex9(478)}, // ex9.it, 2 bytes
        Located{pc:0x1d12,instruction:ShiftLeftSmall3{dest:r(1),source:r(1),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1d14,instruction:AddSmallRegister{dest:r(5),source:r(3),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1d16,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(3),base:r(7),offset:44}}, // memory immediate, 4 bytes
        Located{pc:0x1d1a,instruction:AluShiftLeft{op:AluOp::Add,dest:r(16),left:r(1),right:r(4),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x1d1e,instruction:AddSmallRegister{dest:r(2),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1d20,instruction:AddSmallRegister{dest:r(5),source:r(17),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1d22,instruction:ArithmeticShiftRight{dest:r(17),source:r(0),amount:31}}, // srai, 4 bytes
        Located{pc:0x1d26,instruction:LessThanSmallRegister{left:r(2),right:r(0)}}, // slt45, 2 bytes
        Located{pc:0x1d28,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(1),left:r(16),right:r(1),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1d2c,instruction:AddSmallRegister{dest:r(3),source:r(17),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1d2e,instruction:Move{dest:r(0),source:r(16)}}, // mov55, 2 bytes
        Located{pc:0x1d30,instruction:AddSmallRegister{dest:r(1),source:r(5),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1d32,instruction:AddSmallRegister{dest:r(3),source:r(15),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1d34,instruction:Ex9(42)}, // ex9.it, 2 bytes
        Located{pc:0x1d36,instruction:JumpShort{target:0x1d58}}, // j8, 2 bytes
        Located{pc:0x1d38,instruction:MoveSmall{dest:r(0),value:-1}}, // movi55, 2 bytes
        Located{pc:0x1d3a,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x1d50}}, // beqz/bnez, 4 bytes
        Located{pc:0x1d3e,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(0),base:r(6),offset:108}}, // memory immediate, 4 bytes
        Located{pc:0x1d42,instruction:SignedMemoryImmediate{width:Width::Half,dest:r(0),base:r(6),offset:8}}, // lbsi/lhsi, 4 bytes
        Located{pc:0x1d46,instruction:ImmediateMemory{store:true,width:Width::Half,reg:r(0),base:r(6),offset:16}}, // memory immediate, 4 bytes
        Located{pc:0x1d4a,instruction:ImmediateMemory{store:true,width:Width::Half,reg:r(0),base:r(6),offset:48}}, // memory immediate, 4 bytes
        Located{pc:0x1d4e,instruction:JumpShort{target:0x1d5c}}, // j8, 2 bytes
        Located{pc:0x1d50,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(0),base:r(6),offset:110}}, // memory immediate, 4 bytes
        Located{pc:0x1d54,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(6),offset:24}}, // memory immediate, 4 bytes
        Located{pc:0x1d58,instruction:ImmediateMemory{store:true,width:Width::Half,reg:r(0),base:r(6),offset:28}}, // memory immediate, 4 bytes
        Located{pc:0x1d5c,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(7),offset:6}}, // memory333, 2 bytes
        Located{pc:0x1d5e,instruction:BranchZeroWide{reg:r(0),nonzero:true,target:0x1eca}}, // beqz/bnez, 4 bytes
        Located{pc:0x1d62,instruction:BranchZeroWide{reg:r(9),nonzero:true,target:0x1eca}}, // beqz/bnez, 4 bytes
        Located{pc:0x1d66,instruction:Ex9(404)}, // ex9.it, 2 bytes
        Located{pc:0x1d68,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x1dc0}}, // beqz/bnez, 4 bytes
        Located{pc:0x1d6c,instruction:WordAtRegister{store:false,reg:r(3),base:r(11)}}, // lwi450/swi450, 2 bytes
        Located{pc:0x1d6e,instruction:LoadWordGp{dest:r(5),address:0x2c538}}, // lwi.gp/swi.gp, 4 bytes
        Located{pc:0x1d72,instruction:ShortMemory{store:false,width:Width::Half,reg:r(2),base:r(3),offset:0}}, // memory333, 2 bytes
        Located{pc:0x1d74,instruction:ShortMemory{store:false,width:Width::Half,reg:r(4),base:r(3),offset:2}}, // memory333, 2 bytes
        Located{pc:0x1d76,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(3),offset:62}}, // memory immediate, 4 bytes
        Located{pc:0x1d7a,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(3),base:r(3),offset:60}}, // memory immediate, 4 bytes
        Located{pc:0x1d7e,instruction:LessThanWide{dest:r(15),source:r(5),value:800,signed:false}}, // slti, 4 bytes
        Located{pc:0x1d82,instruction:SignExtendHalf{dest:r(2),source:r(2)}}, // seh33, 2 bytes
        Located{pc:0x1d84,instruction:SignExtendHalf{dest:r(4),source:r(4)}}, // seh33, 2 bytes
        Located{pc:0x1d86,instruction:SignExtendHalf{dest:r(0),source:r(0)}}, // seh33, 2 bytes
        Located{pc:0x1d88,instruction:SignExtendHalf{dest:r(3),source:r(3)}}, // seh33, 2 bytes
        Located{pc:0x1d8a,instruction:BranchTa{nonzero:false,target:0x1d98}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1d8c,instruction:Ex9(406)}, // ex9.it, 2 bytes
        Located{pc:0x1d8e,instruction:BranchImmediate{reg:r(5),value:1,nonzero:true,target:0x1d98}}, // beqc/bnec, 4 bytes
        Located{pc:0x1d92,instruction:AddImmediate{dest:r(1),source:r(1),value:1620}}, // addi, 4 bytes
        Located{pc:0x1d96,instruction:InlineCall{target:0x1dbc}}, // ifcall9, 2 bytes
        Located{pc:0x1d98,instruction:ImmediateMemory{store:false,width:Width::Byte,reg:r(5),base:r(7),offset:9}}, // memory immediate, 4 bytes
        Located{pc:0x1d9c,instruction:BranchZero{reg:r(5),nonzero:true,target:0x1da4}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1d9e,instruction:AddSmallRegister{dest:r(2),source:r(4),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x1da0,instruction:Ex9(459)}, // ex9.it, 2 bytes
        Located{pc:0x1da2,instruction:JumpShort{target:0x1dac}}, // j8, 2 bytes
        Located{pc:0x1da4,instruction:BranchImmediate{reg:r(5),value:31,nonzero:true,target:0x1db8}}, // beqc/bnec, 4 bytes
        Located{pc:0x1da8,instruction:AddSmallRegister{dest:r(0),source:r(3),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x1daa,instruction:Ex9(280)}, // ex9.it, 2 bytes
        Located{pc:0x1dac,instruction:BranchTa{nonzero:true,target:0x1db8}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1dae,instruction:MoveSmall{dest:r(0),value:1}}, // movi55, 2 bytes
        Located{pc:0x1db0,instruction:AddImmediate{dest:r(1),source:r(1),value:324}}, // addi, 4 bytes
        Located{pc:0x1db4,instruction:Ex9(416)}, // ex9.it, 2 bytes
        Located{pc:0x1db6,instruction:InlineCall{target:0x1dbc}}, // ifcall9, 2 bytes
        Located{pc:0x1db8,instruction:AddImmediate{dest:r(1),source:r(1),value:972}}, // addi, 4 bytes
        Located{pc:0x1dbc,instruction:JumpRelative{target:0x20b0,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1dc0,instruction:AddImmediate{dest:r(1),source:r(1),value:2118}}, // addi, 4 bytes
        Located{pc:0x1dc4,instruction:AddImmediate{dest:r(3),source:r(6),value:111}}, // addi, 4 bytes
        Located{pc:0x1dc8,instruction:Move{dest:r(0),source:r(11)}}, // mov55, 2 bytes
        Located{pc:0x1dca,instruction:MoveSmall{dest:r(2),value:0}}, // movi55, 2 bytes
        Located{pc:0x1dcc,instruction:MoveSmall{dest:r(4),value:1}}, // movi55, 2 bytes
        Located{pc:0x1dce,instruction:Ex9(506)}, // ex9.it, 2 bytes
        Located{pc:0x1dd0,instruction:HalfGp{store:false,signed:false,reg:r(9),address:0x193b0}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1dd4,instruction:WordAtStack{store:true,reg:r(0),offset:4}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x1dd6,instruction:HalfGp{store:false,signed:false,reg:r(8),address:0x19414}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1dda,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x1e6a}}, // beqz/bnez, 4 bytes
        Located{pc:0x1dde,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(1),base:r(11),offset:4}}, // memory immediate, 4 bytes
        Located{pc:0x1de2,instruction:Move{dest:r(5),source:r(9)}}, // mov55, 2 bytes
        Located{pc:0x1de4,instruction:AddSmall3{dest:r(2),source:r(1),value:2}}, // addi333, 2 bytes
        Located{pc:0x1de6,instruction:AddressFromStack{dest:r(0),offset:4}}, // addri36.sp, 2 bytes
        Located{pc:0x1de8,instruction:AddImmediate{dest:r(3),source:r(7),value:24}}, // addi, 4 bytes
        Located{pc:0x1dec,instruction:AddImmediate{dest:r(4),source:r(7),value:48}}, // addi, 4 bytes
        Located{pc:0x1df0,instruction:Ex9(497)}, // ex9.it, 2 bytes
        Located{pc:0x1df2,instruction:ShortMemory{store:false,width:Width::Word,reg:r(0),base:r(7),offset:28}}, // memory333, 2 bytes
        Located{pc:0x1df4,instruction:ShortMemory{store:false,width:Width::Word,reg:r(4),base:r(7),offset:24}}, // memory333, 2 bytes
        Located{pc:0x1df6,instruction:ShiftLeftSmall3{dest:r(1),source:r(0),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1df8,instruction:OrShiftRight{dest:r(2),left:r(1),right:r(4),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1dfc,instruction:ShiftLeftSmall3{dest:r(1),source:r(4),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1dfe,instruction:RegisterAlu{op:AluRegOp::Sub,dest:r(4),left:r(9),right:r(8),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1e02,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(0),left:r(9),right:r(4),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1e06,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(16),base:r(31),offset:4}}, // memory immediate, 4 bytes
        Located{pc:0x1e0a,instruction:CompactMiscOp{dest:r(0),source:r(0),op:CompactMisc::Neg}}, // misc33, 2 bytes
        Located{pc:0x1e0c,instruction:ShiftLeftSmall3{dest:r(0),source:r(0),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1e0e,instruction:ArithmeticShiftRight{dest:r(17),source:r(16),amount:31}}, // srai, 4 bytes
        Located{pc:0x1e12,instruction:OrShiftRight{dest:r(9),left:r(0),right:r(4),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1e16,instruction:ShiftLeftSmall3{dest:r(4),source:r(4),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1e18,instruction:MultiplyWide{dest:r(0),left:r(17),right:r(4)}}, // mul, 4 bytes
        Located{pc:0x1e1c,instruction:MultiplyAddRounded32{dest:r(0),left:r(9),right:r(16)}}, // maddr32, 4 bytes
        Located{pc:0x1e20,instruction:MultiplyRegister64{dest:r(4),left:r(4),right:r(16),signed:false}}, // mulr64, 4 bytes
        Located{pc:0x1e24,instruction:AddSmallRegister{dest:r(5),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1e26,instruction:CompactAlu3{dest:r(0),left:r(1),right:r(4),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1e28,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(3),left:r(0),right:r(1),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1e2c,instruction:CompactAlu3{dest:r(1),left:r(2),right:r(5),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1e2e,instruction:Ex9(479)}, // ex9.it, 2 bytes
        Located{pc:0x1e30,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(5),base:r(7),offset:52}}, // memory immediate, 4 bytes
        Located{pc:0x1e34,instruction:AddSmallRegister{dest:r(2),source:r(16),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1e36,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(4),left:r(2),right:r(16),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1e3a,instruction:AddSmallRegister{dest:r(5),source:r(17),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1e3c,instruction:AddSmallRegister{dest:r(1),source:r(3),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1e3e,instruction:CompactAlu3{dest:r(3),left:r(4),right:r(5),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1e40,instruction:Ex9(42)}, // ex9.it, 2 bytes
        Located{pc:0x1e42,instruction:Ex9(399)}, // ex9.it, 2 bytes
        Located{pc:0x1e44,instruction:Ex9(47)}, // ex9.it, 2 bytes
        Located{pc:0x1e46,instruction:BranchZeroWide{reg:r(0),nonzero:true,target:0x1fde}}, // beqz/bnez, 4 bytes
        Located{pc:0x1e4a,instruction:Ex9(326)}, // ex9.it, 2 bytes
        Located{pc:0x1e4c,instruction:BranchZero{reg:r(0),nonzero:false,target:0x1e54}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1e4e,instruction:MoveImmediate{dest:r(15),value:700}}, // movi, 4 bytes
        Located{pc:0x1e52,instruction:JumpShort{target:0x1e58}}, // j8, 2 bytes
        Located{pc:0x1e54,instruction:MoveImmediate{dest:r(15),value:900}}, // movi, 4 bytes
        Located{pc:0x1e58,instruction:Ex9(124)}, // ex9.it, 2 bytes
        Located{pc:0x1e5a,instruction:LessThanSmallRegister{left:r(0),right:r(15)}}, // slt45, 2 bytes
        Located{pc:0x1e5c,instruction:BranchZeroWide{reg:r(15),nonzero:false,target:0x1fde}}, // beqz/bnez, 4 bytes
        Located{pc:0x1e60,instruction:Ex9(402)}, // ex9.it, 2 bytes
        Located{pc:0x1e62,instruction:BranchZeroWide{reg:r(0),nonzero:true,target:0x1fde}}, // beqz/bnez, 4 bytes
        Located{pc:0x1e66,instruction:JumpRelative{target:0x1fdc,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1e6a,instruction:ShortMemory{store:false,width:Width::Word,reg:r(2),base:r(7),offset:28}}, // memory333, 2 bytes
        Located{pc:0x1e6c,instruction:ShortMemory{store:false,width:Width::Word,reg:r(3),base:r(7),offset:24}}, // memory333, 2 bytes
        Located{pc:0x1e6e,instruction:ShiftLeftSmall3{dest:r(1),source:r(2),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1e70,instruction:ZeroExtendWide{dest:r(2),source:r(9)}}, // zeh, 4 bytes
        Located{pc:0x1e74,instruction:RegisterAlu{op:AluRegOp::Sub,dest:r(4),left:r(2),right:r(8),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1e78,instruction:LessThanSmallRegister{left:r(2),right:r(4)}}, // slt45, 2 bytes
        Located{pc:0x1e7a,instruction:ReverseSubtractImmediate{dest:r(2),source:r(15),value:0}}, // subri, 4 bytes
        Located{pc:0x1e7e,instruction:ShiftLeftSmall3{dest:r(2),source:r(2),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1e80,instruction:OrShiftRight{dest:r(9),left:r(2),right:r(4),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1e84,instruction:HalfGp{store:false,signed:true,reg:r(2),address:0x2c0e0}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1e88,instruction:OrShiftRight{dest:r(5),left:r(1),right:r(3),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1e8c,instruction:CompactMiscOp{dest:r(2),source:r(0),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x1e8e,instruction:ShiftLeftSmall3{dest:r(1),source:r(3),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1e90,instruction:HalfGp{store:false,signed:true,reg:r(3),address:0x2c0e4}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1e94,instruction:ShiftLeftSmall3{dest:r(4),source:r(4),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1e96,instruction:Ex9(172)}, // ex9.it, 2 bytes
        Located{pc:0x1e98,instruction:Ex9(480)}, // ex9.it, 2 bytes
        Located{pc:0x1e9a,instruction:CompactMiscOp{dest:r(3),source:r(4),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x1e9c,instruction:MultiplyAddRounded32{dest:r(3),left:r(9),right:r(2)}}, // maddr32, 4 bytes
    ]
}
/// File range [0x20a0,0x23d0), 291 NDS32 instructions.
fn sample_processing_and_register_access()->Vec<Located>{
    let r=Reg::new;
    vec![
        Located{pc:0x1ea0,instruction:Move{dest:r(16),source:r(3)}}, // mov55, 2 bytes
        Located{pc:0x1ea2,instruction:MultiplyRegister64{dest:r(2),left:r(4),right:r(2),signed:false}}, // mulr64, 4 bytes
        Located{pc:0x1ea6,instruction:AluShiftLeft{op:AluOp::Add,dest:r(17),left:r(1),right:r(2),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x1eaa,instruction:AddSmallRegister{dest:r(3),source:r(16),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1eac,instruction:Ex9(479)}, // ex9.it, 2 bytes
        Located{pc:0x1eae,instruction:LessThanSmallRegister{left:r(17),right:r(1)}}, // slt45, 2 bytes
        Located{pc:0x1eb0,instruction:CompactAlu3{dest:r(1),left:r(5),right:r(3),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1eb2,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(3),base:r(7),offset:52}}, // memory immediate, 4 bytes
        Located{pc:0x1eb6,instruction:ArithmeticShiftRight{dest:r(5),source:r(0),amount:31}}, // srai, 4 bytes
        Located{pc:0x1eba,instruction:AddSmallRegister{dest:r(2),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1ebc,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(4),left:r(2),right:r(0),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1ec0,instruction:AddSmallRegister{dest:r(3),source:r(5),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1ec2,instruction:Move{dest:r(0),source:r(17)}}, // mov55, 2 bytes
        Located{pc:0x1ec4,instruction:AddSmallRegister{dest:r(1),source:r(15),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1ec6,instruction:JumpRelative{target:0x203e,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1eca,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(0),base:r(7),offset:7}}, // memory333, 2 bytes
        Located{pc:0x1ecc,instruction:BranchImmediate{reg:r(0),value:49,nonzero:false,target:0x1ed4}}, // beqc/bnec, 4 bytes
        Located{pc:0x1ed0,instruction:JumpRelative{target:0x2044,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1ed4,instruction:BranchZeroWide{reg:r(9),nonzero:true,target:0x2044}}, // beqz/bnez, 4 bytes
        Located{pc:0x1ed8,instruction:Ex9(404)}, // ex9.it, 2 bytes
        Located{pc:0x1eda,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x1f32}}, // beqz/bnez, 4 bytes
        Located{pc:0x1ede,instruction:WordAtRegister{store:false,reg:r(3),base:r(11)}}, // lwi450/swi450, 2 bytes
        Located{pc:0x1ee0,instruction:LoadWordGp{dest:r(5),address:0x2c538}}, // lwi.gp/swi.gp, 4 bytes
        Located{pc:0x1ee4,instruction:ShortMemory{store:false,width:Width::Half,reg:r(2),base:r(3),offset:0}}, // memory333, 2 bytes
        Located{pc:0x1ee6,instruction:ShortMemory{store:false,width:Width::Half,reg:r(4),base:r(3),offset:2}}, // memory333, 2 bytes
        Located{pc:0x1ee8,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(3),offset:62}}, // memory immediate, 4 bytes
        Located{pc:0x1eec,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(3),base:r(3),offset:60}}, // memory immediate, 4 bytes
        Located{pc:0x1ef0,instruction:LessThanWide{dest:r(15),source:r(5),value:800,signed:false}}, // slti, 4 bytes
        Located{pc:0x1ef4,instruction:SignExtendHalf{dest:r(2),source:r(2)}}, // seh33, 2 bytes
        Located{pc:0x1ef6,instruction:SignExtendHalf{dest:r(4),source:r(4)}}, // seh33, 2 bytes
        Located{pc:0x1ef8,instruction:SignExtendHalf{dest:r(0),source:r(0)}}, // seh33, 2 bytes
        Located{pc:0x1efa,instruction:SignExtendHalf{dest:r(3),source:r(3)}}, // seh33, 2 bytes
        Located{pc:0x1efc,instruction:BranchTa{nonzero:false,target:0x1f0a}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1efe,instruction:Ex9(406)}, // ex9.it, 2 bytes
        Located{pc:0x1f00,instruction:BranchImmediate{reg:r(5),value:1,nonzero:true,target:0x1f0a}}, // beqc/bnec, 4 bytes
        Located{pc:0x1f04,instruction:AddImmediate{dest:r(1),source:r(1),value:1782}}, // addi, 4 bytes
        Located{pc:0x1f08,instruction:InlineCall{target:0x1f2e}}, // ifcall9, 2 bytes
        Located{pc:0x1f0a,instruction:ImmediateMemory{store:false,width:Width::Byte,reg:r(5),base:r(7),offset:9}}, // memory immediate, 4 bytes
        Located{pc:0x1f0e,instruction:BranchZero{reg:r(5),nonzero:true,target:0x1f16}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1f10,instruction:AddSmallRegister{dest:r(2),source:r(4),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x1f12,instruction:Ex9(459)}, // ex9.it, 2 bytes
        Located{pc:0x1f14,instruction:JumpShort{target:0x1f1e}}, // j8, 2 bytes
        Located{pc:0x1f16,instruction:BranchImmediate{reg:r(5),value:31,nonzero:true,target:0x1f2a}}, // beqc/bnec, 4 bytes
        Located{pc:0x1f1a,instruction:AddSmallRegister{dest:r(0),source:r(3),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x1f1c,instruction:Ex9(280)}, // ex9.it, 2 bytes
        Located{pc:0x1f1e,instruction:BranchTa{nonzero:true,target:0x1f2a}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1f20,instruction:MoveSmall{dest:r(0),value:1}}, // movi55, 2 bytes
        Located{pc:0x1f22,instruction:AddImmediate{dest:r(1),source:r(1),value:486}}, // addi, 4 bytes
        Located{pc:0x1f26,instruction:Ex9(416)}, // ex9.it, 2 bytes
        Located{pc:0x1f28,instruction:InlineCall{target:0x1f2e}}, // ifcall9, 2 bytes
        Located{pc:0x1f2a,instruction:AddImmediate{dest:r(1),source:r(1),value:1134}}, // addi, 4 bytes
        Located{pc:0x1f2e,instruction:JumpRelative{target:0x20b8,link:false}}, // j/jal, 4 bytes
        Located{pc:0x1f32,instruction:AddImmediate{dest:r(1),source:r(1),value:1956}}, // addi, 4 bytes
        Located{pc:0x1f36,instruction:AddImmediate{dest:r(3),source:r(6),value:111}}, // addi, 4 bytes
        Located{pc:0x1f3a,instruction:MoveSmall{dest:r(2),value:1}}, // movi55, 2 bytes
        Located{pc:0x1f3c,instruction:Move{dest:r(4),source:r(2)}}, // mov55, 2 bytes
        Located{pc:0x1f3e,instruction:Move{dest:r(0),source:r(11)}}, // mov55, 2 bytes
        Located{pc:0x1f40,instruction:Ex9(506)}, // ex9.it, 2 bytes
        Located{pc:0x1f42,instruction:Move{dest:r(4),source:r(0)}}, // mov55, 2 bytes
        Located{pc:0x1f44,instruction:WordAtStack{store:true,reg:r(0),offset:4}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x1f46,instruction:HalfGp{store:false,signed:false,reg:r(8),address:0x19412}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1f4a,instruction:HalfGp{store:false,signed:false,reg:r(9),address:0x19476}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1f4e,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x1fe4}}, // beqz/bnez, 4 bytes
        Located{pc:0x1f52,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(2),base:r(11),offset:4}}, // memory immediate, 4 bytes
        Located{pc:0x1f56,instruction:AddressFromStack{dest:r(0),offset:4}}, // addri36.sp, 2 bytes
        Located{pc:0x1f58,instruction:AddImmediate{dest:r(1),source:r(2),value:98}}, // addi, 4 bytes
        Located{pc:0x1f5c,instruction:AddImmediate{dest:r(3),source:r(7),value:24}}, // addi, 4 bytes
        Located{pc:0x1f60,instruction:AddImmediate{dest:r(2),source:r(2),value:96}}, // addi, 4 bytes
        Located{pc:0x1f64,instruction:AddImmediate{dest:r(4),source:r(7),value:48}}, // addi, 4 bytes
        Located{pc:0x1f68,instruction:Move{dest:r(5),source:r(8)}}, // mov55, 2 bytes
        Located{pc:0x1f6a,instruction:Ex9(497)}, // ex9.it, 2 bytes
        Located{pc:0x1f6c,instruction:ShortMemory{store:false,width:Width::Word,reg:r(0),base:r(7),offset:28}}, // memory333, 2 bytes
        Located{pc:0x1f6e,instruction:ShortMemory{store:false,width:Width::Word,reg:r(2),base:r(7),offset:24}}, // memory333, 2 bytes
        Located{pc:0x1f70,instruction:ShiftLeftSmall3{dest:r(1),source:r(0),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1f72,instruction:OrShiftRight{dest:r(16),left:r(1),right:r(2),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1f76,instruction:ShiftLeftSmall3{dest:r(1),source:r(2),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1f78,instruction:AluShiftLeft{op:AluOp::Add,dest:r(2),left:r(8),right:r(9),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x1f7c,instruction:WordAtStack{store:false,reg:r(4),offset:4}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x1f7e,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(5),left:r(2),right:r(8),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1f82,instruction:ShiftLeftSmall3{dest:r(5),source:r(5),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1f84,instruction:ArithmeticShiftRight{dest:r(17),source:r(4),amount:31}}, // srai, 4 bytes
        Located{pc:0x1f88,instruction:OrShiftRight{dest:r(5),left:r(5),right:r(2),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1f8c,instruction:ShiftLeftSmall3{dest:r(2),source:r(2),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1f8e,instruction:MultiplyWide{dest:r(0),left:r(17),right:r(2)}}, // mul, 4 bytes
        Located{pc:0x1f92,instruction:MultiplyAddRounded32{dest:r(0),left:r(5),right:r(4)}}, // maddr32, 4 bytes
        Located{pc:0x1f96,instruction:MultiplyRegister64{dest:r(2),left:r(2),right:r(4),signed:false}}, // mulr64, 4 bytes
        Located{pc:0x1f9a,instruction:AddSmallRegister{dest:r(3),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1f9c,instruction:CompactAlu3{dest:r(0),left:r(1),right:r(2),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x1f9e,instruction:Ex9(479)}, // ex9.it, 2 bytes
        Located{pc:0x1fa0,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(5),left:r(0),right:r(1),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1fa4,instruction:AluShiftLeft{op:AluOp::Add,dest:r(1),left:r(16),right:r(3),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x1fa8,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(3),base:r(7),offset:52}}, // memory immediate, 4 bytes
        Located{pc:0x1fac,instruction:AddSmallRegister{dest:r(2),source:r(4),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1fae,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(4),left:r(2),right:r(4),shift:0}}, // regalu, 4 bytes
        Located{pc:0x1fb2,instruction:AddSmallRegister{dest:r(3),source:r(17),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1fb4,instruction:AddSmallRegister{dest:r(1),source:r(5),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1fb6,instruction:AddSmallRegister{dest:r(3),source:r(4),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x1fb8,instruction:Ex9(42)}, // ex9.it, 2 bytes
        Located{pc:0x1fba,instruction:Ex9(399)}, // ex9.it, 2 bytes
        Located{pc:0x1fbc,instruction:Ex9(47)}, // ex9.it, 2 bytes
        Located{pc:0x1fbe,instruction:BranchZero{reg:r(0),nonzero:true,target:0x1fde}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1fc0,instruction:Ex9(326)}, // ex9.it, 2 bytes
        Located{pc:0x1fc2,instruction:BranchZero{reg:r(0),nonzero:false,target:0x1fca}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1fc4,instruction:MoveImmediate{dest:r(15),value:600}}, // movi, 4 bytes
        Located{pc:0x1fc8,instruction:JumpShort{target:0x1fce}}, // j8, 2 bytes
        Located{pc:0x1fca,instruction:MoveImmediate{dest:r(15),value:1000}}, // movi, 4 bytes
        Located{pc:0x1fce,instruction:Ex9(124)}, // ex9.it, 2 bytes
        Located{pc:0x1fd0,instruction:LessThanSmallRegister{left:r(0),right:r(15)}}, // slt45, 2 bytes
        Located{pc:0x1fd2,instruction:BranchTa{nonzero:false,target:0x1fde}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x1fd4,instruction:Ex9(402)}, // ex9.it, 2 bytes
        Located{pc:0x1fd6,instruction:BranchZero{reg:r(0),nonzero:true,target:0x1fde}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x1fd8,instruction:MoveImmediate{dest:r(0),value:5119}}, // movi, 4 bytes
        Located{pc:0x1fdc,instruction:Ex9(399)}, // ex9.it, 2 bytes
        Located{pc:0x1fde,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(6),offset:18}}, // memory immediate, 4 bytes
        Located{pc:0x1fe2,instruction:JumpShort{target:0x2054}}, // j8, 2 bytes
        Located{pc:0x1fe4,instruction:ShortMemory{store:false,width:Width::Word,reg:r(2),base:r(7),offset:28}}, // memory333, 2 bytes
        Located{pc:0x1fe6,instruction:ShortMemory{store:false,width:Width::Word,reg:r(3),base:r(7),offset:24}}, // memory333, 2 bytes
        Located{pc:0x1fe8,instruction:ShiftLeftSmall3{dest:r(1),source:r(2),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1fea,instruction:OrShiftRight{dest:r(16),left:r(1),right:r(3),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x1fee,instruction:ShiftLeftSmall3{dest:r(1),source:r(3),amount:1}}, // slli333, 2 bytes
        Located{pc:0x1ff0,instruction:HalfGp{store:false,signed:true,reg:r(3),address:0x2c0e2}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1ff4,instruction:ZeroExtendWide{dest:r(5),source:r(8)}}, // zeh, 4 bytes
        Located{pc:0x1ff8,instruction:CompactMiscOp{dest:r(3),source:r(4),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x1ffa,instruction:HalfGp{store:false,signed:true,reg:r(17),address:0x2c0e4}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x1ffe,instruction:AluShiftLeft{op:AluOp::Add,dest:r(2),left:r(5),right:r(9),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x2002,instruction:DivideRemainder{quotient:r(17),dividend:r(3),divisor:r(17),remainder:r(17),signed:true}}, // divsr/divr, 4 bytes
        Located{pc:0x2006,instruction:LessThanSmallRegister{left:r(2),right:r(5)}}, // slt45, 2 bytes
        Located{pc:0x2008,instruction:ShiftLeftImmediate{dest:r(5),source:r(15),amount:1}}, // slli, 4 bytes
        Located{pc:0x200c,instruction:OrShiftRight{dest:r(5),left:r(5),right:r(2),shift:31}}, // or_srli, 4 bytes
        Located{pc:0x2010,instruction:ArithmeticShiftRight{dest:r(0),source:r(17),amount:31}}, // srai, 4 bytes
        Located{pc:0x2014,instruction:ShiftLeftSmall3{dest:r(2),source:r(2),amount:1}}, // slli333, 2 bytes
        Located{pc:0x2016,instruction:CompactMiscOp{dest:r(0),source:r(2),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x2018,instruction:MultiplyAddRounded32{dest:r(0),left:r(5),right:r(17)}}, // maddr32, 4 bytes
        Located{pc:0x201c,instruction:MultiplyRegister64{dest:r(2),left:r(2),right:r(17),signed:false}}, // mulr64, 4 bytes
        Located{pc:0x2020,instruction:AddSmallRegister{dest:r(3),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x2022,instruction:CompactAlu3{dest:r(0),left:r(1),right:r(2),subtract:false}}, // add333/sub333, 2 bytes
        Located{pc:0x2024,instruction:Ex9(479)}, // ex9.it, 2 bytes
        Located{pc:0x2026,instruction:LessThanSmallRegister{left:r(0),right:r(1)}}, // slt45, 2 bytes
        Located{pc:0x2028,instruction:AluShiftLeft{op:AluOp::Add,dest:r(1),left:r(16),right:r(3),shift:0}}, // add_slli/or_slli, 4 bytes
        Located{pc:0x202c,instruction:ImmediateMemory{store:false,width:Width::Word,reg:r(3),base:r(7),offset:52}}, // memory immediate, 4 bytes
        Located{pc:0x2030,instruction:ArithmeticShiftRight{dest:r(16),source:r(4),amount:31}}, // srai, 4 bytes
        Located{pc:0x2034,instruction:AddSmallRegister{dest:r(2),source:r(4),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x2036,instruction:RegisterAlu{op:AluRegOp::Slt,dest:r(4),left:r(2),right:r(4),shift:0}}, // regalu, 4 bytes
        Located{pc:0x203a,instruction:AddSmallRegister{dest:r(3),source:r(16),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x203c,instruction:AddSmallRegister{dest:r(1),source:r(15),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x203e,instruction:AddSmallRegister{dest:r(3),source:r(4),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x2040,instruction:Ex9(42)}, // ex9.it, 2 bytes
        Located{pc:0x2042,instruction:JumpShort{target:0x2072}}, // j8, 2 bytes
        Located{pc:0x2044,instruction:MoveSmall{dest:r(0),value:-1}}, // movi55, 2 bytes
        Located{pc:0x2046,instruction:BranchZeroWide{reg:r(10),nonzero:false,target:0x206a}}, // beqz/bnez, 4 bytes
        Located{pc:0x204a,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(0),base:r(6),offset:109}}, // memory immediate, 4 bytes
        Located{pc:0x204e,instruction:SignedMemoryImmediate{width:Width::Half,dest:r(0),base:r(6),offset:10}}, // lbsi/lhsi, 4 bytes
        Located{pc:0x2052,instruction:Ex9(399)}, // ex9.it, 2 bytes
        Located{pc:0x2054,instruction:ImmediateMemory{store:true,width:Width::Half,reg:r(0),base:r(6),offset:50}}, // memory immediate, 4 bytes
        Located{pc:0x2058,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(6),offset:16}}, // memory immediate, 4 bytes
        Located{pc:0x205c,instruction:HalfGp{store:true,signed:false,reg:r(0),address:0x2c77c}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x2060,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(6),offset:18}}, // memory immediate, 4 bytes
        Located{pc:0x2064,instruction:HalfGp{store:true,signed:false,reg:r(0),address:0x2c77e}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x2068,instruction:JumpShort{target:0x20c0}}, // j8, 2 bytes
        Located{pc:0x206a,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(0),base:r(6),offset:111}}, // memory immediate, 4 bytes
        Located{pc:0x206e,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(6),offset:26}}, // memory immediate, 4 bytes
        Located{pc:0x2072,instruction:ImmediateMemory{store:true,width:Width::Half,reg:r(0),base:r(6),offset:30}}, // memory immediate, 4 bytes
        Located{pc:0x2076,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(6),offset:24}}, // memory immediate, 4 bytes
        Located{pc:0x207a,instruction:HalfGp{store:true,signed:false,reg:r(0),address:0x2c778}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x207e,instruction:ImmediateMemory{store:false,width:Width::Half,reg:r(0),base:r(6),offset:26}}, // memory immediate, 4 bytes
        Located{pc:0x2082,instruction:HalfGp{store:true,signed:false,reg:r(0),address:0x2c77a}}, // lhi.gp/shi.gp, 4 bytes
        Located{pc:0x2086,instruction:JumpShort{target:0x20c0}}, // j8, 2 bytes
        Located{pc:0x2088,instruction:MoveSmall{dest:r(1),value:-1}}, // movi55, 2 bytes
        Located{pc:0x208a,instruction:BranchZero{reg:r(3),nonzero:false,target:0x2096}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x208c,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(1),base:r(0),offset:108}}, // memory immediate, 4 bytes
        Located{pc:0x2090,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(1),base:r(0),offset:109}}, // memory immediate, 4 bytes
        Located{pc:0x2094,instruction:JumpShort{target:0x20c0}}, // j8, 2 bytes
        Located{pc:0x2096,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(1),base:r(0),offset:110}}, // memory immediate, 4 bytes
        Located{pc:0x209a,instruction:ImmediateMemory{store:true,width:Width::Byte,reg:r(1),base:r(0),offset:111}}, // memory immediate, 4 bytes
        Located{pc:0x209e,instruction:JumpShort{target:0x20c0}}, // j8, 2 bytes
        Located{pc:0x20a0,instruction:AddImmediate{dest:r(3),source:r(6),value:108}}, // addi, 4 bytes
        Located{pc:0x20a4,instruction:JumpRelative{target:0x1ac8,link:false}}, // j/jal, 4 bytes
        Located{pc:0x20a8,instruction:AddImmediate{dest:r(3),source:r(6),value:108}}, // addi, 4 bytes
        Located{pc:0x20ac,instruction:JumpRelative{target:0x1c34,link:false}}, // j/jal, 4 bytes
        Located{pc:0x20b0,instruction:AddImmediate{dest:r(3),source:r(6),value:109}}, // addi, 4 bytes
        Located{pc:0x20b4,instruction:JumpRelative{target:0x1dc8,link:false}}, // j/jal, 4 bytes
        Located{pc:0x20b8,instruction:AddImmediate{dest:r(3),source:r(6),value:109}}, // addi, 4 bytes
        Located{pc:0x20bc,instruction:JumpRelative{target:0x1f3a,link:false}}, // j/jal, 4 bytes
        Located{pc:0x20c0,instruction:Stack{pop:true,last:r(14),extra:8}}, // push25/pop25, 2 bytes
        Located{pc:0x20c2,instruction:Stack{pop:false,last:r(8),extra:0}}, // push25/pop25, 2 bytes
        Located{pc:0x20c4,instruction:LoadWordGp{dest:r(6),address:0x2c7c0}}, // lwi.gp/swi.gp, 4 bytes
        Located{pc:0x20c8,instruction:MoveSmall{dest:r(7),value:1}}, // movi55, 2 bytes
        Located{pc:0x20ca,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(5),base:r(6),offset:2}}, // memory333, 2 bytes
        Located{pc:0x20cc,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(6),base:r(6),offset:3}}, // memory333, 2 bytes
        Located{pc:0x20ce,instruction:BranchZero{reg:r(2),nonzero:false,target:0x20d2}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x20d0,instruction:MoveSmall{dest:r(7),value:-1}}, // movi55, 2 bytes
        Located{pc:0x20d2,instruction:BranchZero{reg:r(1),nonzero:true,target:0x20e2}}, // beqz38/bnez38, 2 bytes
        Located{pc:0x20d4,instruction:SubtractSmall3{dest:r(1),source:r(5),value:1}}, // subi333, 2 bytes
        Located{pc:0x20d6,instruction:CompactMiscOp{dest:r(5),source:r(0),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x20d8,instruction:MultiplyAddRounded32{dest:r(5),left:r(2),right:r(1)}}, // maddr32, 4 bytes
        Located{pc:0x20dc,instruction:WordAtRegister{store:true,reg:r(5),base:r(3)}}, // lwi450/swi450, 2 bytes
        Located{pc:0x20de,instruction:AddSmallRegister{dest:r(5),source:r(7),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x20e0,instruction:JumpShort{target:0x20f6}}, // j8, 2 bytes
        Located{pc:0x20e2,instruction:SubtractSmall3{dest:r(1),source:r(6),value:1}}, // subi333, 2 bytes
        Located{pc:0x20e4,instruction:CompactMiscOp{dest:r(2),source:r(1),op:CompactMisc::Mul}}, // misc33, 2 bytes
        Located{pc:0x20e6,instruction:Move{dest:r(1),source:r(0)}}, // mov55, 2 bytes
        Located{pc:0x20e8,instruction:MultiplyAddRounded32{dest:r(1),left:r(2),right:r(5)}}, // maddr32, 4 bytes
        Located{pc:0x20ec,instruction:AddSmallRegister{dest:r(2),source:r(7),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x20ee,instruction:MultiplyAddRounded32{dest:r(0),left:r(2),right:r(5)}}, // maddr32, 4 bytes
        Located{pc:0x20f2,instruction:WordAtRegister{store:true,reg:r(1),base:r(3)}}, // lwi450/swi450, 2 bytes
        Located{pc:0x20f4,instruction:Move{dest:r(5),source:r(0)}}, // mov55, 2 bytes
        Located{pc:0x20f6,instruction:WordAtRegister{store:true,reg:r(5),base:r(4)}}, // lwi450/swi450, 2 bytes
        Located{pc:0x20f8,instruction:Stack{pop:true,last:r(8),extra:0}}, // push25/pop25, 2 bytes
        Located{pc:0x20fa,instruction:Stack{pop:false,last:r(14),extra:24}}, // push25/pop25, 2 bytes
        Located{pc:0x20fc,instruction:MoveSmall{dest:r(6),value:0}}, // movi55, 2 bytes
        Located{pc:0x20fe,instruction:Move{dest:r(13),source:r(0)}}, // mov55, 2 bytes
        Located{pc:0x2100,instruction:Move{dest:r(10),source:r(1)}}, // mov55, 2 bytes
        Located{pc:0x2102,instruction:WordAtStack{store:true,reg:r(2),offset:4}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x2104,instruction:Move{dest:r(12),source:r(3)}}, // mov55, 2 bytes
        Located{pc:0x2106,instruction:Move{dest:r(7),source:r(4)}}, // mov55, 2 bytes
        Located{pc:0x2108,instruction:Move{dest:r(11),source:r(5)}}, // mov55, 2 bytes
        Located{pc:0x210a,instruction:Move{dest:r(8),source:r(3)}}, // mov55, 2 bytes
        Located{pc:0x210c,instruction:Move{dest:r(9),source:r(6)}}, // mov55, 2 bytes
        Located{pc:0x210e,instruction:LessThanSmallRegister{left:r(7),right:r(8)}}, // slt45, 2 bytes
        Located{pc:0x2110,instruction:BranchTa{nonzero:true,target:0x2134}}, // beqzs8/bnezs8, 2 bytes
        Located{pc:0x2112,instruction:Move{dest:r(0),source:r(8)}}, // mov55, 2 bytes
        Located{pc:0x2114,instruction:WordAtStack{store:false,reg:r(1),offset:72}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x2116,instruction:WordAtStack{store:false,reg:r(2),offset:4}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x2118,instruction:AddressFromStack{dest:r(3),offset:8}}, // addri36.sp, 2 bytes
        Located{pc:0x211a,instruction:AddressFromStack{dest:r(4),offset:12}}, // addri36.sp, 2 bytes
        Located{pc:0x211c,instruction:JumpRelative{target:0x20c2,link:true}}, // j/jal, 4 bytes
        Located{pc:0x2120,instruction:WordAtStack{store:false,reg:r(0),offset:8}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x2122,instruction:AddSmall{reg:r(8),value:1,subtract:false}}, // addi45/subi45, 2 bytes
        Located{pc:0x2124,instruction:IndexedMemory{store:false,width:Width::Half,signed:true,reg:r(0),base:r(13),index:r(0),shift:1}}, // indexed memory, 4 bytes
        Located{pc:0x2128,instruction:AddSmallRegister{dest:r(9),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x212a,instruction:WordAtStack{store:false,reg:r(0),offset:12}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x212c,instruction:IndexedMemory{store:false,width:Width::Half,signed:true,reg:r(0),base:r(13),index:r(0),shift:1}}, // indexed memory, 4 bytes
        Located{pc:0x2130,instruction:AddSmallRegister{dest:r(6),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x2132,instruction:JumpShort{target:0x210e}}, // j8, 2 bytes
        Located{pc:0x2134,instruction:AddSmallRegister{dest:r(7),source:r(12),subtract:true}}, // add45/sub45, 2 bytes
        Located{pc:0x2136,instruction:MoveSmall{dest:r(0),value:5}}, // movi55, 2 bytes
        Located{pc:0x2138,instruction:CompactMiscOp{dest:r(7),source:r(7),op:CompactMisc::Not}}, // misc33, 2 bytes
        Located{pc:0x213a,instruction:MultiplyAddRounded32{dest:r(6),left:r(7),right:r(0)}}, // maddr32, 4 bytes
        Located{pc:0x213e,instruction:MoveSmall{dest:r(2),value:1}}, // movi55, 2 bytes
        Located{pc:0x2140,instruction:MaximumRegister{dest:r(2),left:r(6),right:r(2)}}, // max, 4 bytes
        Located{pc:0x2144,instruction:Move{dest:r(0),source:r(10)}}, // mov55, 2 bytes
        Located{pc:0x2146,instruction:Move{dest:r(1),source:r(9)}}, // mov55, 2 bytes
        Located{pc:0x2148,instruction:JumpRelative{target:0xb850,link:true}}, // j/jal, 4 bytes
        Located{pc:0x214c,instruction:ImmediateMemory{store:false,width:Width::Byte,reg:r(2),base:r(11),offset:0}}, // memory immediate, 4 bytes
        Located{pc:0x2150,instruction:BranchImmediate{reg:r(2),value:255,nonzero:false,target:0x215c}}, // beqc/bnec, 4 bytes
        Located{pc:0x2154,instruction:AddSmall{reg:r(2),value:1,subtract:false}}, // addi45/subi45, 2 bytes
        Located{pc:0x2156,instruction:AddSmallRegister{dest:r(2),source:r(0),subtract:false}}, // add45/sub45, 2 bytes
        Located{pc:0x2158,instruction:ShiftRight{dest:r(0),source:r(2),amount:1}}, // srli, 4 bytes
        Located{pc:0x215c,instruction:Ex9(227)}, // ex9.it, 2 bytes
        Located{pc:0x215e,instruction:AddressFromStack{dest:r(1),offset:16}}, // addri36.sp, 2 bytes
        Located{pc:0x2160,instruction:AddressFromStack{dest:r(2),offset:20}}, // addri36.sp, 2 bytes
        Located{pc:0x2162,instruction:JumpRelative{target:0xb91e,link:true}}, // j/jal, 4 bytes
        Located{pc:0x2166,instruction:WordAtStack{store:false,reg:r(0),offset:16}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x2168,instruction:IndexedMemory{store:false,width:Width::Half,signed:true,reg:r(1),base:r(10),index:r(0),shift:1}}, // indexed memory, 4 bytes
        Located{pc:0x216c,instruction:WordAtStack{store:false,reg:r(0),offset:20}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x216e,instruction:IndexedMemory{store:false,width:Width::Half,signed:true,reg:r(2),base:r(10),index:r(0),shift:1}}, // indexed memory, 4 bytes
        Located{pc:0x2172,instruction:MoveSmall{dest:r(0),value:2}}, // movi55, 2 bytes
        Located{pc:0x2174,instruction:Ex9(194)}, // ex9.it, 2 bytes
        Located{pc:0x2176,instruction:MultiplyAddRounded32{dest:r(0),left:r(9),right:r(2)}}, // maddr32, 4 bytes
        Located{pc:0x217a,instruction:Ex9(78)}, // ex9.it, 2 bytes
        Located{pc:0x217c,instruction:Stack{pop:true,last:r(14),extra:24}}, // push25/pop25, 2 bytes
        Located{pc:0x217e,instruction:Stack{pop:false,last:r(14),extra:40}}, // push25/pop25, 2 bytes
        Located{pc:0x2180,instruction:Move{dest:r(9),source:r(1)}}, // mov55, 2 bytes
        Located{pc:0x2182,instruction:LoadWordGp{dest:r(1),address:0x2c7c0}}, // lwi.gp/swi.gp, 4 bytes
        Located{pc:0x2186,instruction:WordAtStack{store:true,reg:r(2),offset:8}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x2188,instruction:ImmediateMemory{store:false,width:Width::Byte,reg:r(2),base:r(1),offset:50}}, // memory immediate, 4 bytes
        Located{pc:0x218c,instruction:WordAtStack{store:true,reg:r(2),offset:16}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x218e,instruction:ImmediateMemory{store:false,width:Width::Byte,reg:r(2),base:r(1),offset:51}}, // memory immediate, 4 bytes
        Located{pc:0x2192,instruction:WordAtStack{store:true,reg:r(2),offset:20}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x2194,instruction:SignedMemoryImmediate{width:Width::Byte,dest:r(13),base:r(1),offset:57}}, // lbsi/lhsi, 4 bytes
        Located{pc:0x2198,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(2),base:r(1),offset:2}}, // memory333, 2 bytes
        Located{pc:0x219a,instruction:SignedMemoryImmediate{width:Width::Byte,dest:r(14),base:r(1),offset:58}}, // lbsi/lhsi, 4 bytes
        Located{pc:0x219e,instruction:ShortMemory{store:false,width:Width::Byte,reg:r(1),base:r(1),offset:3}}, // memory333, 2 bytes
        Located{pc:0x21a0,instruction:AddImmediate{dest:r(8),source:r(0),value:32}}, // addi, 4 bytes
        Located{pc:0x21a4,instruction:SubtractSmall3{dest:r(0),source:r(1),value:1}}, // subi333, 2 bytes
        Located{pc:0x21a6,instruction:WordAtStack{store:true,reg:r(0),offset:12}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x21a8,instruction:SignExtendByte{dest:r(0),source:r(1)}}, // seb33, 2 bytes
        Located{pc:0x21aa,instruction:WordAtStack{store:true,reg:r(0),offset:32}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x21ac,instruction:SubtractSmall3{dest:r(0),source:r(2),value:2}}, // subi333, 2 bytes
        Located{pc:0x21ae,instruction:WordAtStack{store:true,reg:r(0),offset:24}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x21b0,instruction:SignExtendByte{dest:r(0),source:r(2)}}, // seb33, 2 bytes
        Located{pc:0x21b2,instruction:WordAtStack{store:true,reg:r(0),offset:36}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x21b4,instruction:SubtractSmall3{dest:r(0),source:r(1),value:2}}, // subi333, 2 bytes
        Located{pc:0x21b6,instruction:MoveImmediate{dest:r(7),value:182148}}, // movi, 4 bytes
        Located{pc:0x21ba,instruction:MoveSmall{dest:r(10),value:0}}, // movi55, 2 bytes
        Located{pc:0x21bc,instruction:AddImmediate{dest:r(12),source:r(2),value:-1}}, // addi, 4 bytes
        Located{pc:0x21c0,instruction:WordAtStack{store:true,reg:r(0),offset:28}}, // lwi37.sp/swi37.sp, 2 bytes
        Located{pc:0x21c2,instruction:LoadWordGp{dest:r(0),address:0x2c7c0}}, // lwi.gp/swi.gp, 4 bytes
        Located{pc:0x21c6,instruction:ImmediateMemory{store:false,width:Width::Byte,reg:r(0),base:r(0),offset:9}}, // memory immediate, 4 bytes
        Located{pc:0x21ca,instruction:LessThanSmallRegister{left:r(10),right:r(0)}}, // slt45, 2 bytes
        Located{pc:0x21cc,instruction:BranchZeroWide{reg:r(15),nonzero:false,target:0x23a2}}, // beqz/bnez, 4 bytes
    ]
}

pub(super) fn instructions(profile:Profile)->Vec<Located>{
    if profile==Profile::Map220v{return vec![];}
    let mut items=post_event_flag_updates();
    items.extend(status_and_sample_dispatch());
    items.extend(sample_processing_and_register_access());
    debug_assert_eq!(items.first().unwrap().pc as usize,FILE_START-0x200);
    let last=items.last().unwrap();
    debug_assert_eq!(last.pc as usize+last.instruction.encode(last.pc,profile.gp()).unwrap().len(),FILE_END-0x200);
    items
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn consecutive_typed_source_matches_exact_file_interval(){
        let entries=instructions(Profile::Postmarketos);
        assert_eq!(entries.len(),702);
        let mut file=FILE_START;
        for op in entries{
            assert_eq!(op.pc as usize+0x200,file);
            file+=op.instruction.encode(op.pc,Profile::Postmarketos.gp()).unwrap().len();
        }
        assert_eq!(file,FILE_END);
    }
}
