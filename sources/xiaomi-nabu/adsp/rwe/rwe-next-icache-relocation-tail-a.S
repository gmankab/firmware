// SPDX-License-Identifier: GPL-3.0-only
// Intermediate cache-relocation controller state, [file 0x111dc,0x1121c).
.text
.globl adsp_relocation_middle
adsp_relocation_middle:
{ p0=cmp.eq(r6,#0); if(!p0.new) jump:nt finalize_cache; memw(r16+#0)=r7 }
{ p0=!cmp.gtu(r5,r3); if(!p0.new) r3=sub(r3,r4); if(p0.new) jump:nt finalize_cache }
{ p0=cmp.gtu(r3,#55); immext(#0xf000e380); if(p0.new) r0=##0xf000e394; if(p0.new) jump:t controller_range_error }
{ r3=and(r3,#-8); immext(#0x57e0c000); r0=##0x57e0c000 }
{ immext(#0x57e0c000); r1=##0x57e0c000; r2=add(r2,r3) }
{ jump finalize_cache; memd(r2+#200)=r1:0 }
.set finalize_cache,adsp_relocation_middle+(0x8be0e34c-0x8be0e1dc)
.set controller_range_error,adsp_relocation_middle+(0x8be0e260-0x8be0e1dc)
