// SPDX-License-Identifier: GPL-3.0-only
// Source-authored Hexagon controller interval selection, file [0x1121c,0x112d4).
// The code scans a bounded series of controller relocation entries.
.text
.globl adsp_relocation_select_address
adsp_relocation_select_address:
{ immext(#0x57e0c000); r8=##0x57e0c000; r13:12=memd(r2+#248) }
{ immext(#0x57e0c000); r9=##0x57e0c000 }
{ p0=cmp.eq(r13:12,r9:8); if(p0.new) r7=#6; if(!p0.new) jump:t test_slot5; if(!p0.new) r13:12=memd(r2+#240) }
{ r8=add(r2,#248); jump finish_slot }
test_slot5:
{ p0=cmp.eq(r13:12,r9:8); if(p0.new) r7=#5; if(!p0.new) jump:t test_slot4; if(!p0.new) r13:12=memd(r2+#232) }
{ r8=add(r2,#240); jump finish_slot }
// Explicit lower-priority branch target, entered from an earlier path.
call_range_error:
{ call platform_assert }
{ jump finalize_cache }
test_slot4:
{ p0=cmp.eq(r13:12,r9:8); if(p0.new) r7=#4; if(!p0.new) jump:t test_slot3; if(!p0.new) r13:12=memd(r2+#224) }
{ r8=add(r2,#232); jump finish_slot }
test_slot3:
{ p0=cmp.eq(r13:12,r9:8); if(p0.new) r7=#3; if(!p0.new) jump:t test_slot2; if(!p0.new) r13:12=memd(r2+#216) }
{ r8=add(r2,#224); jump finish_slot }
test_slot2:
{ p0=cmp.eq(r13:12,r9:8); if(p0.new) r7=#2; if(!p0.new) jump:t test_slot1; if(!p0.new) r13:12=memd(r2+#208) }
{ r8=add(r2,#216); jump finish_slot }
test_slot1:
{ p0=cmp.eq(r13:12,r9:8); if(!p0.new) r7=#0; if(!p0.new) jump:t test_slot0; if(!p0.new) r13:12=memd(r2+#200) }
{ r7=#1; jump finish_slot; r8=add(r2,#208) }
test_slot0:
{ p0=cmp.eq(r13:12,r9:8); r8=r4; if(!p0.new) jump:t invalid_slot }
.set platform_assert,adsp_relocation_select_address+(0x8be11438-0x8be0e21c)
.set finalize_cache,adsp_relocation_select_address+(0x8be0e34c-0x8be0e21c)
.set finish_slot,adsp_relocation_select_address+(0x8be0e2d4-0x8be0e21c)
.set invalid_slot,adsp_relocation_select_address+(0x8be0e35c-0x8be0e21c)
