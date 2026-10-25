// SPDX-License-Identifier: GPL-3.0-only
//! Complete low-ILM controller helpers following the reset routine.
//! MMIO addresses remain numeric until a verified register map is available.
use super::{Located,Profile};
use super::isa::{Insn,Width,AluOp,R0,R1,R2,R3,R4,R5,R6,LP};

pub fn instructions(profile:Profile)->Vec<Located>{
    use Insn::*;
    use Width::{Byte,Half};
    let (mask2,workspace,poll_address,poll_byte,status_address,status_shift)=match profile{
        Profile::Postmarketos=>(0x7d,0x2c290,0x3a,0x36,0x1c5,0x166),
        Profile::Map220v=>(0x7a,0x2c278,0x36,0x31,0x1c3,0x173),
    };
    let source=[
        // Controller-address setup: conditionally clear byte 0x3f0ed,
        // retain bits 4..7 at 0x3f0da, and initialize two control bytes.
        (0x4bc,MoveImmediate{dest:R0,value:0x3f0ed}),
        (0x4c0,ShortMemory{store:false,width:Byte,reg:R1,base:R0,offset:0}),
        (0x4c2,BranchImmediate{reg:R1,value:1,nonzero:true,target:0x4ca}),
        (0x4c6,MoveSmall{dest:R1,value:0}),
        (0x4c8,ShortMemory{store:true,width:Byte,reg:R1,base:R0,offset:0}),
        (0x4ca,MoveImmediate{dest:R1,value:0x3f0da}),
        (0x4ce,ShortMemory{store:false,width:Byte,reg:R0,base:R1,offset:0}),
        (0x4d0,Ex9(0x121)), // ANDI R0,R0,0xf0 in both tables
        (0x4d2,ShortMemory{store:true,width:Byte,reg:R0,base:R1,offset:0}),
        (0x4d4,MoveSmall{dest:R0,value:4}),
        (0x4d6,Ex9(mask2)),
        (0x4d8,MoveSmall{dest:R1,value:5}),
        (0x4da,MoveImmediate{dest:R0,value:0x3f049}),
        (0x4de,ShortMemory{store:true,width:Byte,reg:R1,base:R0,offset:0}),
        (0x4e0,MoveImmediate{dest:R1,value:0x53}),
        (0x4e4,Ex9(3)), // MOVI R0,0x3f04a
        (0x4e6,ShortMemory{store:true,width:Byte,reg:R1,base:R0,offset:0}),
        (0x4e8,Return{reg:LP}),
        // Separate empty leaf at the next halfword.
        (0x4ea,Return{reg:LP}),
        // Configure a controller transfer while preserving the high control
        // byte. IFCALL9 shares the polling setup at 0x526; IFRET16 is a
        // conditional IFC-state return, not a conventional unconditional RET.
        (0x4ec,MoveImmediate{dest:R4,value:0x3f114}),
        (0x4f0,WordAtRegister{store:false,reg:R3,base:R4}),
        (0x4f2,ShiftRightSmall{reg:R3,amount:24}),
        (0x4f4,AluShiftLeft{op:AluOp::Or,dest:R0,left:R0,right:R3,shift:24}),
        (0x4f8,WordAtRegister{store:true,reg:R0,base:R4}),
        (0x4fa,MoveImmediate{dest:R0,value:0x3f138}),
        (0x4fe,ShortMemory{store:true,width:Half,reg:R1,base:R0,offset:0}),
        (0x500,MoveSmall{dest:R1,value:0}),
        (0x502,MoveImmediate{dest:R0,value:0x3f13a}),
        (0x506,ShortMemory{store:true,width:Byte,reg:R1,base:R0,offset:0}),
        (0x508,MoveImmediate{dest:R0,value:0x3f104}),
        (0x50c,WordAtRegister{store:true,reg:R2,base:R0}),
        (0x50e,MoveSmall{dest:R1,value:1}),
        (0x510,MoveImmediate{dest:R0,value:0x3f136}),
        (0x514,ShortMemory{store:true,width:Byte,reg:R1,base:R0,offset:0}),
        (0x516,InlineCall{target:0x526}),
        (0x518,ShortMemory{store:false,width:Byte,reg:R1,base:R0,offset:0}),
        (0x51a,LowBit{dest:R1,source:R1}),
        (0x51c,BranchZero{reg:R1,nonzero:false,target:0x522}),
        (0x51e,ShortMemory{store:true,width:Byte,reg:R3,base:R2,offset:0}),
        (0x520,JumpShort{target:0x518}),
        (0x522,MoveImmediate{dest:R1,value:0x3f134}),
        (0x526,Ex9(poll_address)), // MOVI R2,0x3f04a
        (0x528,Ex9(poll_byte)), // MOVI R3,0x55
        (0x52a,InlineReturn),
        (0x52c,ShortMemory{store:false,width:Byte,reg:R0,base:R1,offset:0}),
        (0x52e,BitMaskShort{reg:R0,bit:1}),
        (0x530,BranchZero{reg:R0,nonzero:true,target:0x536}),
        (0x532,ShortMemory{store:true,width:Byte,reg:R3,base:R2,offset:0}),
        (0x534,JumpShort{target:0x52c}),
        (0x536,MoveImmediate{dest:R0,value:0x3f134}),
        (0x53a,ShortMemory{store:false,width:Byte,reg:R0,base:R0,offset:0}),
        (0x53c,LowBit{dest:R0,source:R0}),
        (0x53e,Return{reg:LP}),
        // Counted byte copy: r0 source, r1 destination, r2 byte count, r5
        // cursor. The BEQS38 operand is r2; r5 is its implicit comparator.
        (0x540,MoveSmall{dest:R5,value:0}),
        (0x542,BranchA5{reg:R2,nonzero:false,target:0x550}),
        (0x544,IndexedByte{store:false,reg:R3,base:R0,index:R5,shift:0}),
        (0x548,IndexedByte{store:true,reg:R3,base:R1,index:R5,shift:0}),
        (0x54c,AddSmall{reg:R5,value:1,subtract:false}),
        (0x54e,JumpShort{target:0x542}),
        (0x550,Return{reg:LP}),
        // Query controller state, invoke a selector for 7 and 0, and report
        // a 0x100-byte workspace. EX9 call destinations remain dependencies.
        (0x552,Stack{pop:false,last:R6,extra:0}),
        (0x554,Ex9(2)),
        (0x556,BranchZero{reg:R0,nonzero:false,target:0x56e}),
        (0x558,MoveSmall{dest:R0,value:7}),
        (0x55a,Ex9(10)),
        (0x55c,MoveSmall{dest:R0,value:0}),
        (0x55e,Ex9(10)),
        (0x560,MoveImmediate{dest:R1,value:workspace}),
        (0x564,MoveSmall{dest:R0,value:2}),
        (0x566,Move{dest:R2,source:R1}),
        (0x568,MoveImmediate{dest:R3,value:0x100}),
        (0x56c,Ex9(5)),
        (0x56e,Stack{pop:true,last:R6,extra:0}),
        // Encode caller status into low/high bytes, then keep writing the
        // handshake byte while the saved caller status is nonzero.
        (0x570,Stack{pop:false,last:R6,extra:0}),
        (0x572,Move{dest:R6,source:R0}),
        (0x574,Ex9(2)),
        (0x576,BranchZero{reg:R0,nonzero:false,target:0x59a}),
        (0x578,MoveImmediate{dest:R1,value:-97}),
        (0x57c,Ex9(status_address)), // MOVI R0,0x2fe60
        (0x57e,ShortMemory{store:true,width:Byte,reg:R1,base:R0,offset:0}),
        (0x580,ZeroExtend{dest:R0,source:R6,half:false}),
        (0x582,MoveImmediate{dest:R1,value:0x2fe61}),
        (0x586,ShortMemory{store:true,width:Byte,reg:R0,base:R1,offset:0}),
        (0x588,Ex9(status_shift)), // SRLI R0,R6,8
        (0x58a,MoveImmediate{dest:R1,value:0x2fe62}),
        (0x58e,ShortMemory{store:true,width:Byte,reg:R0,base:R1,offset:0}),
        (0x590,Ex9(3)), // MOVI R0,0x3f04a
        (0x592,Ex9(4)), // MOVI R1,0x55
        (0x594,BranchZero{reg:R6,nonzero:false,target:0x59a}),
        (0x596,ShortMemory{store:true,width:Byte,reg:R1,base:R0,offset:0}),
        (0x598,JumpShort{target:0x594}),
        (0x59a,Stack{pop:true,last:R6,extra:0}),
    ];
    source.into_iter().map(|(pc,instruction)|Located{pc,instruction}).collect()
}
