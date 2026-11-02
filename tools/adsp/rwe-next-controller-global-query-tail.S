// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller global status getter and alternate table walk [0x12664,0x126a4).
.text
.globl adsp_global_query_tail
adsp_global_query_tail:
return_state:
{ p0=cmp.eq(r2,r3); r17:16=memd(r29+#8); r19:18=memd(r29+#0) }
{ if(!p0) r0=#0; immext(#0xf01f1500); if(p0) r0=memw(##0xf01f1514); dealloc_return }
match_slot:
{ r6=#-1; r5=memw(r6+#0) }
read_table:
{ r7=memh(r5+#0); if(!cmp.eq(r7.new,#0)) jump:nt data_found }
{ r5=add(r5,#2); r6=add(r6,#1); if(!cmp.gtu(r6.new,#14)) jump:t read_table }
{ jump return_state }
data_found:
{ r4=memw(r4+#0) }
{ r1=or(r4,r1); jump return_state }
