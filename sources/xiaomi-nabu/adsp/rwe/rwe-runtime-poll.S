// SPDX-License-Identifier: GPL-3.0-only
// Hexagon GP-mapped global-state poll and buffer reset, [0x6440,0x6474).
.text
.globl adsp_agent20_rwe_runtime_poll
adsp_agent20_rwe_runtime_poll:
{ immext(#0xf0025580); r0=memw(gp+##0xf00255b0); if(cmp.eq(r0.new,#0)) jump:t done }
{ immext(#0xf0026f00); r0=memw(gp+##0xf0026f20); if(cmp.eq(r0.new,#0)) jump:t done }
{ immext(#0xf0025540); r0=memw(gp+##0xf0025560); if(cmp.eq(r0.new,#0)) jump:nt done }
{ memw(r0+#0x24)=#0; memw(r0+#0x20)=#0 }
{ memw(r0+#0x10)=#0; memw(r0+#0x14)=#0 }
{ memw(r0+#4)=clrbit(#21) }
done:
{ jump done }
