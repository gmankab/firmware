// SPDX-License-Identifier: GPL-3.0-only
// Controller-slot scanner prefix. File offset [0x121c0,0x12220),
// ADSP physical [0x8be0f1c0,0x8be0f220).
.text
.globl adsp_slot_scan_prefix
adsp_slot_scan_prefix:
{ immext(#0xf0026c40); r2=memw(GP+##0xf0026c40); allocframe(#56) }
{ p0=cmp.eq(r2,#0) }
{ r0=#1 }
{ r16=r0; memd(r29+#48)=r17:16 }
{ memd(r29+#40)=r19:18; memd(r29+#32)=r21:20 }
{ memd(sp+#24)=r23:22; memd(sp+#16)=r25:24 }
{ if(p0) jump:nt early_exit; memd(sp+#8)=r27:26 }
{ immext(#0xf0026d40); r2=memw(GP+##0xf0026d68) }
{ p0=cmp.eq(r2,#0); if(p0.new) jump:nt early_exit; immext(#0xf00254c0); if(!p0.new) r20=memw(##0xf00254dc) }
{ loop0(scan_slots,r2) }
scan_slots:
{ r2=memw(r20+#0); if(cmp.eq(r2.new,r16)) jump:nt slot_found }
{ r20=add(r20,#160); nop }:endloop0
{ jump early_exit }
slot_found:
{ if(!p0.new) r19=#0; p0=cmp.eq(r20,#0); if(p0.new) jump:nt early_exit }
.set early_exit,adsp_slot_scan_prefix+(0x8be12418-0x8be121c0)
