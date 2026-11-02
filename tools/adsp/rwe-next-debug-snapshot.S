// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 debugger-register snapshot and 3-cacheline synchronization.
// ADSP file [0x10460,0x104b8), physical [0x8be0d460,0x8be0d4b8).
// The seven unsupported privileged ISDBCFG/BRKPT reads are represented by
// source-authored add-by-zero producers for LLVM's `.new` pairing only. The
// emitter validates and replaces those producers with checked V66 register
// encodings; no proprietary instruction bytes are imported.
.text
.globl adsp_rwe_debug_snapshot
adsp_rwe_debug_snapshot:
{ immext(#0xf0010dc0); r1=##0xf0010dd0 }
{ r0=add(r0,#0); memw(r1++#4)=r0.new } // R0=ISDBCFG0
{ r0=add(r0,#0); memw(r1++#4)=r0.new } // R0=ISDBCFG1
{ r0=add(r0,#0); memw(r1++#4)=r0.new } // R0=BRKPTPC0
{ r0=add(r0,#0); memw(r1++#4)=r0.new } // R0=BRKPTCFG0
{ r0=add(r0,#0); memw(r1++#4)=r0.new } // R0=BRKPTPC1
{ r0=add(r0,#0); memw(r1++#4)=r0.new } // R0=BRKPTCFG1
{ r0=add(r0,#0); memw(r1+#0)=r0.new } // R0=ISDBGPR
{ immext(#0xf0010dc0); r1=##0xf0010dd0; r0=#3 }
{ loop0(flush_lines,r0); nop }
flush_lines:
{ r1=add(r1,#32); dccleana(r1) }:endloop0
