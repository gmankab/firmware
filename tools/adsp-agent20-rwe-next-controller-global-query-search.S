// SPDX-License-Identifier: GPL-3.0-only
// ADSP global controller query memory and hardware-loop search [0x125f4,0x12664).
.text
.globl adsp_global_query_search
adsp_global_query_search:
{ immext(#0xf01f1500); r4=##0xf01f1510; r3=memw(r18+#28); r2=memw(r18+#256) }
{ r5=extractu(r3,#7,#8) }
{ r1=r5 }
{ immext(#0x300000); r1=or(##0x300000,asl(r1,#25)) }
{ immext(#0xf00254c0); r3=memw(GP+##0xf00254e0); if(cmp.eq(r3.new,r2)) jump:nt found_direct }
{ immext(#0xf0026c40); r6=memw(GP+##0xf0026c40); if(cmp.eq(r6.new,#0)) jump:nt no_match }
{ immext(#0xf0026d40); r6=memw(GP+##0xf0026d68) }
{ p0=cmp.eq(r6,#0); if(p0.new) jump:nt no_match; immext(#0xf00254c0); if(!p0.new) r7=memw(##0xf00254dc) }
{ loop0(scan_entries,r6) }
{ r6=add(r7,#8) }
scan_entries:
{ r7=memw(r6+#-8); if(cmp.eq(r7.new,r5)) jump:nt match_entry }
{ r6=add(r6,#160); nop }:endloop0
{ jump no_match }
found_direct:
{ r4=memw(r4+#0) }
{ r1=or(r4,r1) }
.set match_entry,adsp_global_query_search+(0x8be1267c-0x8be125f4)
.set no_match,adsp_global_query_search+(0x8be12664-0x8be125f4)
