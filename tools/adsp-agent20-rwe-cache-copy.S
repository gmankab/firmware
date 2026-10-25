// SPDX-License-Identifier: GPL-3.0-only
// V66 source-authored controller words copier with per-cacheline cleanup.
// Exact ADSP MBN file [0x6734,0x676c), mapped physical [0x8be03734,0x8be0376c).
.text
.globl adsp_rwe_cache_copy
adsp_rwe_cache_copy:
{ r2=lsr(r2,#2); r6=r2; r5=r1 }
{ loop0(copy_loop,r2); r2=#0 }
copy_loop:
{ r4=memw(r0++#4) }
{ nop; memw(r1++#4)=r4 }:endloop0
{ r1=r5; r0=r6 }
{ r0=lsr(r0,#5) }
{ r0=add(r0,#1) }
{ loop0(clean_loop,r0) }
clean_loop:
{ r1=add(r1,#32); dccleana(r1) }:endloop0
{ jumpr lr }
