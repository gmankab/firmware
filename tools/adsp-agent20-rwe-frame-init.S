// SPDX-License-Identifier: GPL-3.0-only
// Hexagon ADSP RWE per-thread codec frame metadata preparation.
// File offsets [0x625c,0x62c8). All packet grouping is source-authored.
.text
.globl adsp_agent20_rwe_frame_init
adsp_agent20_rwe_frame_init:
{ allocframe(#8) }
{ r1 = add(r0,#0x68); memd(sp+#0) = r1:0 }
{ r1 += mpyi(r28,#0xe0); call metadata_lookup; r0 = r18 }
{ r23 = r0; r1:0 = memd(sp+#0) }
{ r18 = memw(r0+#0) }
{ r19 = extract(r18,#3,#0); p0 = tstbit(r18,#28) }
{ p1 = cmp.eq(r19,r1) }
{ p2 = and(p0,!p1); if (p2.new) jump:nt rollback }
{ if (p1) jump:nt update }
{ p0 = cmp.eq(r1,#0); if (!p0.new) jump:t rollback }
update:
{ r2 = #2 }
{ r1 = insert(r2,#8,#16) }
{ r1 = setbit(r1,#30) }
{ r1 = setbit(r1,#29) }
{ r0 = add(r0,#8); r1 = memw(r23+#0); memw(r0+#0) = r1 }
{ call metadata_publish }
{ immext(#0); r0 = #32; immext(#0xf0000040); memw(r1=##0xf0000044) = r0.new }
{ dccleana(r1) }
.set metadata_lookup,adsp_agent20_rwe_frame_init+(0x8be2206c-0x8be0325c)
.set metadata_publish,adsp_agent20_rwe_frame_init+(0x8be21fb4-0x8be0325c)
.set rollback,adsp_agent20_rwe_frame_init+(0x8be03474-0x8be0325c)
