// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 96-byte controller SRAM cloning and cache-clean helper.
// File [0x103d0,0x10430), physical [0x8be0d3d0,0x8be0d430).
// Source reproduces branch-predicated shared state copy, 50 memory words
// and ten cache-line clean operations; no OEM opcode words are stored.
.text
.globl adsp_rwe_clone_sram
adsp_rwe_clone_sram:
{ immext(#0xf0025540); r0=memw(##0xf0025560); if(cmp.eq(r0.new,#0)) jump:t clone_done }
{ immext(#0xf0010e00); r3=##0xf0010e30; r2=memw(r0+#4) }
{ r4=extractu(r2,#1,#21); p0=!tstbit(r2,#21); immext(#0xf0010dc0); memw(r5=##0xf0010dcc)=r4.new }
{ if(p0) jump:nt clone_done; dccleana(r5) }
{ r1=clrbit(r2,#21) }
{ memw(r0+#4)=r1; memw(r3+#4)=r2 }
{ r4=memw(r0++#16) } // standalone packet; following .new store is paired with r4=#1
{ r4=#1; memw(r3++#16)=r4.new } // original .new dependency, not an inserted OEM word
{ loop0(copy_loop,#50) }
copy_loop:
{ r4=memw(r0++#4); memw(r3++#4)=r4.new }:endloop0
{ immext(#0xf0010e00); r1=##0xf0010e30; r0=#10 }
{ loop0(clean_loop,r0) }
clean_loop:
{ r1=add(r1,#32); dccleana(r1) }:endloop0
clone_done:
