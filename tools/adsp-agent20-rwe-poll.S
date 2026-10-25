// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 ADSP RWE warm-controller readiness branch [0x6148,0x6194).
// Three guarded global config pointers are checked, the controller status
// fields are cleared, then a self-loop waits for the next hardware event.
.text
.globl adsp_agent20_rwe_poll
adsp_agent20_rwe_poll:
{ immext(#0xf0025580); r0 = memw(gp+##0xf00255b0); if (cmp.eq(r0.new,#0)) jump:t wait }
{ immext(#0xf0026f00); r0 = memw(gp+##0xf0026f20); if (cmp.eq(r0.new,#0)) jump:t wait }
{ immext(#0xf0025540); r0 = memw(gp+##0xf0025560); if (cmp.eq(r0.new,#0)) jump:nt wait }
{ memw(r0+#0x24) = #0; memw(r0+#0x20) = #0 }
{ memw(r0+#0x10) = #0; memw(r0+#0x14) = #0 }
{ memw(r0+#0x40) = #0; memw(r0+#0x3c) = #0 }
{ memw(r0+#0x5c) = #0; memw(r0+#0x58) = #0 }
{ memw(r0+#0x78) = #0; memw(r0+#0x74) = #0 }
{ memw(r0+#0x4) = clrbit(#21) }
wait:
{ jump wait }
