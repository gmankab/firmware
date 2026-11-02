// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller indexed record reset and list draining [0x13250,0x13390).
.text
.globl adsp_record_reset
adsp_record_reset:
{ memd(r29+#-16)=r17:16; allocframe(#24) }
{ r19:18=combine(r1,r0); memd(r29+#8)=r19:18; memd(r29+#0)=r21:20 }
{ nop } // checked V66 R2=SGP0 at 0x1325c
{ r17=#-2; immext(#0xf01eb000); r3=memw(gp+##0xf01eb018); if(!cmp.gtu(r3.new,r18)) jump:t return_result }
{ call controller_lookup; immext(#0xf0010840); r0=##0xf0010844; r16=memw(r2+#0) }
{ r2=asl(r18,#5) }
{ immext(#0xf02d1240); r2=memw(r2<<#0+##0xf02d1258); if(!cmp.eq(r2.new,r19)) jump:t missing_record }
{ immext(#0xf02d1240); r18=add(##0xf02d1250,asl(r18,#5)) }
{ r2=memb(r18+#7); if(cmp.eq(r2.new,#0)) jump:nt inactive }
{ r2=memub(r18+#5); if(cmp.eq(r2.new,#3)) jump:nt check_linked }
{ p0=cmp.eq(r2,#1); if(p0.new) jump:nt check_trailing; if(p0.new) r2=memw(r16+#28) }
{ p0=cmp.eq(r2,#0); if(!p0.new) jump:t clear_record; if(p0.new) r2=memub(r16+#30) }
{ p0=!tstbit(r2,#3); if(p0.new) jump:t record_error }
{ jump clear_record }
missing_record:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump return_result }
inactive:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump return_result }
check_linked:
{ r2=memw(r16+#256) }
{ r2=memw(r2+#16); if(cmp.eq(r2.new,#0)) jump:nt record_error }
clear_record:
{ r17=memw(r18+#12); memw(r18+#12)=#0 }
{ p0=cmp.eq(r17,#0); if(p0.new) jump:nt clear_done }
{ nop } // checked V66 k0lock at 0x132f4
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r18=#-0x1112 }
{ r21:20=combine(#0,#0) }
linked_loop:
{ r2=memw(r17+#256) }
{ r2=memw(r2+#16); if(!cmp.eq(r2.new,#4)) jump:t deactivate }
{ r2=add(r17,#0xe0) }
{ memh(r2+#0)=setbit(#5) }
{ memw(r17+#12)=r18 }
{ r17=memw(r17+#0); if(!cmp.eq(r17.new,#0)) jump:t linked_loop }
{ jump notify_cleanup }
deactivate:
{ r0=r17; memw(r17+#0x90)=#-6; memd(r17+#8)=r21:20 }
{ call controller_deactivate; r17=memw(r17+#0); immext(#0xf0021100); memw(r17+#0xec)=##0xf002111c }
{ p0=cmp.eq(r17,#0); if(!p0.new) jump:t linked_loop }
notify_cleanup:
{ call controller_clear; r0=r16; r1=memub(r16+#21); memw(r16+#0x90)=#0 }
clear_done:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r17=#0 }
{ jump return_result }
check_trailing:
{ r2=extractu(r2,#7,#8); if(cmp.eq(r2.new,#0)) jump:nt clear_record }
record_error:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r17=#-1 }
return_result:
{ r0=r17; r17:16=memd(r29+#16); r19:18=memd(r29+#8) }
{ r21:20=memd(r29+#0); dealloc_return }
.set controller_lookup,adsp_record_reset+(0x8be261bc-0x8be13250)
.set controller_notify,adsp_record_reset+(0x8be261e8-0x8be13250)
.set controller_deactivate,adsp_record_reset+(0x8be26ac4-0x8be13250)
.set controller_clear,adsp_record_reset+(0x8be2692c-0x8be13250)
