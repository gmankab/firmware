// SPDX-License-Identifier: GPL-3.0-only
// Restore 128 controller TLB register pairs from previously checkpointed SRAM.
// ADSP MBN [0x6c10,0x6c30), Hexagon physical [0x8be03c10,0x8be03c30).
// TLB write is represented as a checked typed V66 instruction instead of
// pasting the OEM opcode; this LLVM6 source contains only a NOP sentinel.
.text
.globl adsp_rwe_tlb_restore
adsp_rwe_tlb_restore:
{ r4=#127; immext(#0xf000cf00); r1=##0xf000cf20; r0=#0 }
restore_loop:
{ r3:2=memd(r1++#8) }
{ nop } // checked tlbw(R3:2,R0)
{ p0=cmp.eq(r0,r4); if(!p0.new) jump:t restore_loop; r0=add(r0,#1) }
finish_restore:
{ jumpr lr }
