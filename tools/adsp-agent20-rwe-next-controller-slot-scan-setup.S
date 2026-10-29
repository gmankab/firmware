// SPDX-License-Identifier: GPL-3.0-only
// Continuation of controller slot scanner: file [0x12220,0x12254).
.text
.globl adsp_scan_setup
adsp_scan_setup:
{ immext(#0xf0414880); r24=##0xf04148a8; r27:26=combine(#0,#0) }
{ immext(#0xf0026c40); r2=memw(GP+##0xf0026c5c) }
{ r2=lsr(r2,#5); memw(sp+#4)=r2.new }
{ memw(sp+#0)=r16 }
{ r2=memw(r20+#4) }
{ r25=memw(r2+r27<<#2); if(cmp.eq(r25.new,#0)) jump:nt next_table }
{ r23:22=combine(#0,r24); jump scan_bitmap }
.set next_table,adsp_scan_setup+(0x8be123fc-0x8be12220)
.set scan_bitmap,adsp_scan_setup+(0x8be12310-0x8be12220)
