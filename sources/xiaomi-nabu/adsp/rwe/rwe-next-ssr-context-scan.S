// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon V66 SSR-gated controller context scan, physical file
// [0x11090,0x11108); linker fill [0x11088,0x11090) excluded.
.text
.globl adsp_rwe_ssr_context_scan
adsp_rwe_ssr_context_scan:
{ nop } // V66 R2=SSR checked typed encoder
{ p0=!tstbit(r2,#19); r2=#13; if(p0.new) jump:nt done }
{ immext(#0xf0026880); r3=memw(GP+##0xf0026894) }
{ immext(#0xf0024640); if(!p0.new) r5=##0xf0024660; p0=cmp.eq(r3,#0); if(p0.new) jump:nt done }
{ immext(#0xf001e340); r3:2=combine(##0xf001e360,#0); immext(#0xf00250c0); r4=##0xf00250e0 }
{ immext(#0xf0011680); r6=memw(GP+##0xf00116ac) }
{ loop0(scan_again,#3); memw(r0+r6<<#2)=r2 }
scan_again:
{ r7=#-1; r6=memw(r4+#0) }
{ r6=memw(r6+#0); if(cmp.eq(r6.new,r5)) jump:nt next_record }
{ p0=cmp.eq(r3,r6); if(p0.new) jump:nt next_record }
{ r7=memub(r6+#21); if(!cmp.eq(r7.new,r2)) jump:t advance }
{ r7=memw(r6+#208) }
next_record:
{ memw(r0+#0)=r7 }
advance:
{ r4=add(r4,#4); r3=add(r3,#320); r0=add(r0,#4); r2=add(r2,#1) }:endloop0
{ r2=#0 }
done:
{ r0=r2; jumpr lr }
