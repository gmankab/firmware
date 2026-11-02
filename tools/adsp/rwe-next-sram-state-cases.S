// SPDX-License-Identifier: GPL-3.0-only
// ADSP file [0x11754,0x11798): select and write controller SRAM state.
// The routine falls through into controller publish calls at 0x11798.
.text
.globl adsp_sram_state_cases
adsp_sram_state_cases:
{ p0=cmp.eq(r2,#0); immext(#0xf020a080); r3=##0xf020a080; if(p0.new) memw(r18+#0)=r3.new }
{ if(p0) jump:nt state_published; if(p0) memw(r16+#0)=r3 }
{ p0=cmp.eq(r2,#1); if(!p0.new) jump:nt state_published }
{ r2=#0; memw(r18+#0)=r17 }
{ jump state_published; memw(r16+#0)=r2 }
{ p0=cmp.eq(r2,#2); if(p0.new) jump:t state_published; if(p0.new) memw(r18+#0)=r17 }
{ p0=cmp.eq(r2,#3); immext(#0); if(p0.new) r2=##0; if(p0.new) memw(r18+#0)=r17 }
{ if(p0) memw(r16+#0)=r2 }
.set state_published,adsp_sram_state_cases+(0x8be0e798-0x8be0e754)
