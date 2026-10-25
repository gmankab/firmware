// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 ADSP early controller memory initialization at [0x6194,0x6220).
// Model special-register reads through validated ISA encoders, and build all
// packet branch/call offsets using symbols derived from physical addresses.
.text
.globl adsp_agent20_rwe_memory_init
adsp_agent20_rwe_memory_init:
{ immext(#0xf000d300); r0 = memw(gp+##0xf000d320); if (cmp.eq(r0.new,#1)) jump:nt external_reschedule }
{ immext(#0); r0 = #27; immext(#0xf0000040); memw(r1=##0xf0000044) = r0.new }
{ dccleana(r1) }
{ nop } // V66 r1=HTID
{ sp = ##-0xffdd750 }
{ sp = addasl(sp,r1,#5) }
{ nop; r19 = r2 } // V66 R28 = P3:0, packet-continuing typed substitution
{ immext(#0x12340); call controller_runtime }
{ r2 = r19 }
{ r0 = #5; immext(#0xf000d300); memw(r1=##0xf000d320) = r0.new }
{ dccleana(r1) }
{ immext(#0xf0025100); r0 = memw(gp+##0xf0025128); immext(#0xf0025580); r1 = memw(gp+##0xf00255b0) }
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:t ready; immext(#0xf0026f00); r1 = memw(gp+##0xf0026f20) }
{ p0 = cmp.eq(r1,#0); if (!p0.new) jump:nt ready; immext(#0xf0025540); r1 = memw(gp+##0xf0025560) }
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:t ready }
{ memw(r1+#0x24) = #0; memw(r1+#0x20) = #0 }
{ memw(r1+#0x10) = #0; memw(r1+#0x14) = #0 }
{ memw(r1+#0x4) = clrbit(#21) }
ready:
.set external_reschedule,adsp_agent20_rwe_memory_init+(0x8be03440-0x8be03194)
.set controller_runtime,adsp_agent20_rwe_memory_init+(0x8be15544-0x8be03194)
