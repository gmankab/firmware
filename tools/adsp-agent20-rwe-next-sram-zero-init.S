// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller SRAM zeroing startup prefix; packet-loop source only.
// File [0x116a0,0x116d8), physical [0x8be0e6a0,0x8be0e6d8).
.text
.globl adsp_sram_zero_init
adsp_sram_zero_init:
{ memd(r29+#-16)=r17:16; allocframe(#16) }
{ memd(sp+#0)=r19:18; immext(#0xf0003000); r2=memw(gp+##0xf0003008) }:mem_noshuf
{ r2=##0xf000cf20; p0=cmp.eq(r2,#0); if(p0.new) jump:nt zero_context_unavailable }
{ loop0(clear_sram,#128); r1:0=combine(#0,#0) }
clear_sram:
{ nop; memd(r2++#8)=r1:0 }:endloop0
{ r17:16=memd(r29+#8); r19:18=memd(r29+#0) }
{ jump cleanup_return; deallocframe }
.set zero_context_unavailable,adsp_sram_zero_init+(0x8be0e6d8-0x8be0e6a0)
.set cleanup_return,adsp_sram_zero_init+(0x8be1d63c-0x8be0e6a0)
