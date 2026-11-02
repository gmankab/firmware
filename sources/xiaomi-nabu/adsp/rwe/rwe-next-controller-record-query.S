// SPDX-License-Identifier: GPL-3.0-only
// ADSP second indexed record query with ordinary return [0x130d0,0x131e0).
// Error/linked-list continuation at 0x131e0 is a separate fragment.
.text
.globl adsp_record_query
adsp_record_query:
{ r17:16=combine(r1,r0); memd(r29+#-16)=r17:16; allocframe(#24) }
{ r19=r2; memd(r29+#8)=r19:18; memd(r29+#0)=r21:20 }
{ nop } // checked R2=SGP0
{ r20=#-2; immext(#0xf01eb000); r3=memw(gp+##0xf01eb018); if(!cmp.gtu(r3.new,r16)) jump:t finish }
{ call controller_lookup; immext(#0xf0010840); r0=##0xf0010844; r18=memw(r2+#0) }
{ r2=asl(r16,#5) }
{ immext(#0xf02d1240); r2=memw(r2<<#0+##0xf02d1258); if(!cmp.eq(r2.new,r17)) jump:t mismatch }
{ immext(#0xf02d1240); r16=add(##0xf02d1250,asl(r16,#5)) }
{ r2=memb(r16+#7); if(cmp.eq(r2.new,#0)) jump:nt no_object }
{ r2=memw(r18+#28); r4=memub(r16+#4) }
{ r3=extractu(r2,#7,#8); if(!cmp.eq(r3.new,r4)) jump:t wrong_type }
{ r2=memb(r16+#6); if(cmp.eq(r2.new,#0)) jump:nt unset_active }
{ immext(#0xf0010840); r0=##0xf0010844; p0=cmp.eq(r19,#2); r3:2=memd(r16+#24) }
{ call controller_notify; r20=#0; memd(r18+#0x88)=r3:2; if(!p0) memb(r16+#6)=#0 }
{ jump finish }
mismatch:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump finish }
no_object:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump finish }
wrong_type:
{ r4=memub(r16+#5); if(cmp.eq(r4.new,#3)) jump:nt check_exception }
{ p0=cmp.eq(r4,#1); if(p0.new) jump:nt check_identity }
{ p0=cmp.eq(r4,#0); if(!p0.new) jump:t rescan }
{ p0=!tstbit(r2,#19); if(p0.new) jump:t report_error }
{ jump rescan }
unset_active:
{ if(!p0.new) r20=#-3; p0=cmp.eq(r19,#0); if(p0.new) jump:nt list_walk; if(!p0.new) r3:2=memd(r16+#24) }
{ immext(#0xf0010840); r0=##0xf0010844; memd(r18+#0x88)=r3:2 }
{ call controller_notify }
{ jump finish }
check_identity:
{ p0=cmp.eq(r3,#0); if(!p0.new) jump:t report_error }
{ jump rescan }
check_exception:
{ r2=memw(r18+#256) }
{ r2=memw(r2+#16); if(!cmp.eq(r2.new,#0)) jump:t rescan }
report_error:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r20=#-1 }
finish:
{ r0=r20; r17:16=memd(r29+#16); r19:18=memd(r29+#8) }
{ r21:20=memd(r29+#0); dealloc_return }
.set rescan,adsp_record_query+(0x8be1312c-0x8be130d0)
.set list_walk,adsp_record_query+(0x8be131e0-0x8be130d0)
.set controller_lookup,adsp_record_query+(0x8be261bc-0x8be130d0)
.set controller_notify,adsp_record_query+(0x8be261e8-0x8be130d0)
