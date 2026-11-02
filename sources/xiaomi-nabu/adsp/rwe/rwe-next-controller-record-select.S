// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller indexed record select, normal return path [0x12f50,0x13084).
// An additional error/recovery path at 0x13084 follows in a later fragment.
.text
.globl adsp_record_select
adsp_record_select:
{ r17:16=combine(r3,r2); memd(r29+#-16)=r17:16; allocframe(#32) }
{ r22=r4; r21:20=combine(r1,r0); memd(r29+#8)=r21:20; memd(r29+#0)=r23:22 }
{ memd(sp+#16)=r19:18 }
{ nop } // checked R2=SGP0
{ r19=#-2; immext(#0xf01eb000); r3=memw(gp+##0xf01eb018); if(!cmp.gtu(r3.new,r20)) jump:t return_result }
{ call controller_lookup; immext(#0xf0010840); r0=##0xf0010844; r18=memw(r2+#0) }
{ r2=asl(r20,#5) }
{ immext(#0xf02d1240); r2=memw(r2<<#0+##0xf02d1258); if(!cmp.eq(r2.new,r21)) jump:t error_mismatch }
{ immext(#0xf02d1240); r20=add(##0xf02d1250,asl(r20,#5)) }
{ r2=memb(r20+#7); if(cmp.eq(r2.new,#0)) jump:nt error_empty }
{ r2=memw(r18+#0x1c); r3=memub(r20+#4) }
{ r2=extractu(r2,#7,#8); if(!cmp.eq(r2.new,r3)) jump:t error_format }
{ p0=cmp.eq(r22,#1); if(!p0.new) jump:nt access_record }
{ r3=#255; r2=memub(r20+#6) }
{ p0=bitsclr(r2,r3); if(p0.new) jump:nt access_record }
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r19=#-4 }
{ jump return_result }
error_mismatch:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump return_result }
error_empty:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump return_result }
error_format:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r19=#-1 }
{ jump return_result }
access_record:
{ call controller_service; r0=memw(r18+#256) }
{ p0=cmp.eq(r0,#0); if(p0.new) jump:nt record_not_accessible; if(p0.new) memd(r20+#24)=r17:16 }
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r19=#-7 }
{ jump return_result }
record_not_accessible:
{ r2=#-0x1112; r19=memw(r20+#12) }
lock_walk:
{ p0=cmp.eq(r19,#0); if(p0.new) jump:nt update_status; immext(#0x100); if(!p0.new) r3=memw(r19+##0x100) }
{ r3=memw(r3+#16); if(!cmp.eq(r3.new,#4)) jump:nt error_recover }
{ r3=add(r19,#0xe0) }
{ r3=add(r19,#8); memh(r3+#0)=setbit(#5) }
{ r19=memw(r19+#0); memw(r3+#4)=r2 }
{ jump lock_walk; memw(r20+#12)=r19 }
update_status:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; memb(r20+#6)=#1 }
{ r19=#0 }
return_result:
{ r0=r19; r17:16=memd(r29+#24); r19:18=memd(r29+#16) }
{ r21:20=memd(r29+#8); r23:22=memd(r29+#0) }
{ dealloc_return }
.set error_recover,adsp_record_select+(0x8be13084-0x8be12f50)
.set controller_lookup,adsp_record_select+(0x8be261bc-0x8be12f50)
.set controller_service,adsp_record_select+(0x8be22c10-0x8be12f50)
.set controller_notify,adsp_record_select+(0x8be261e8-0x8be12f50)
