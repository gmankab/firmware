// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 indexed controller record detach and linked-list cleanup.
// The first stage is [file 0x13480,0x13514); the rest belongs to this function.
.text
.globl adsp_record_detach
adsp_record_detach:
{ memd(r29+#-16)=r17:16; allocframe(#24) }
{ r19:18=combine(r1,r0); memd(r29+#8)=r19:18; memd(r29+#0)=r21:20 }
{ nop } // checked privileged R2=SGP0 at 0x1348c
{ r17=r18; r18=#-2 }
{ immext(#0xf01eb000); r3=memw(gp+##0xf01eb018); if(!cmp.gtu(r3.new,r17)) jump:t return_result }
{ immext(#0xf0010840); r0=##0xf0010844; r16=memw(r2+#0) }
{ immext(#0xf02d1240); r17=add(##0xf02d1250,asl(r17,#5)); call controller_lookup }
{ immext(#0xf0010840); r2=memw(gp+##0xf0010848); if(cmp.eq(r2.new,r17)) jump:nt have_match }
{ r4=r2; if(cmp.eq(r4.new,#0)) jump:nt have_match }
scan_list:
{ r3=r4 }
{ r4=memw(r3+#0); if(cmp.eq(r4.new,r17)) jump:nt matched_link }
{ p0=cmp.eq(r4,#0); if(!p0.new) jump:t scan_list }
{ jump not_found }
have_match:
{ r3=r2; p0=cmp.eq(r2,#0); if(p0.new) jump:nt not_found }
matched_link:
{ p0=cmp.eq(r3,r17); if(p0.new) jump:nt check_record }
{ r4=memw(r3+#0); if(cmp.eq(r4.new,r17)) jump:nt check_record }
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump return_result }
not_found:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump return_result }
// Next original instruction at 0x13514: record class validation
.set controller_lookup,adsp_record_detach+(0x8be261bc-0x8be13480)
.set controller_notify,adsp_record_detach+(0x8be261e8-0x8be13480)
.set return_result,adsp_record_detach+(0x13604-0x13480)
.set check_record,adsp_record_detach+(0x13514-0x13480)
