// SPDX-License-Identifier: GPL-3.0-only
// Read back 128 TLB register pairs into the SRAM array, then perform a
// 32-byte D-cache clean for each of 32 cache-line groups.
// ADSP MBN [0x6c30,0x6c6c), Hexagon physical [0x8be03c30,0x8be03c6c).
// An unsupported V66 tlbr pair is produced by a checked typed encoding.
.text
.globl adsp_rwe_tlb_readback
adsp_rwe_tlb_readback:
{ r0=#128; immext(#0xf000cf00); r1=##0xf000cf20 }
{ r4=lsr(r0,#2) }
{ loop0(copy_loop,r0); r0=#0 }
copy_loop:
{ nop } // checked r3:2=tlbr(R0)
{ r0=add(r0,#1); memd(r1++#8)=r3:2 }:endloop0
{ immext(#0xf000cf00); r1=##0xf000cf20 }
{ loop0(clean_loop,r4) }
clean_loop:
{ r1=add(r1,#32); dccleana(r1) }:endloop0
{ jumpr lr }
