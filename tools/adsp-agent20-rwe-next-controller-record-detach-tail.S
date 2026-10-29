// SPDX-License-Identifier: GPL-3.0-only
// V66 controller record-detach completion [ADSP 0x13514, 0x13610).
// The normal/corrupt paths continue from the exact prefix checkpoint 88.
.text
.globl adsp_record_detach_tail
adsp_record_detach_tail:
check_record:
{ r4=memw(r17+#8); if(!cmp.eq(r4.new,r19)) jump:t report_mismatch }
{ r4=memw(r16+#28); r5=memub(r17+#4) }
{ r4=extractu(r4,#7,#8); if(!cmp.eq(r4.new,r5)) jump:t report_bad_type }
{ p0=!cmp.eq(r3,r2); if(p0.new) jump:nt update_link }
{ p0=!cmp.eq(r3,r17); if(p0.new) jump:nt update_link }
{ jump clear_descriptor }
clear_head:
{ r2=memw(r17+#0); immext(#0xf0010840); memw(gp+##0xf0010848)=r2.new }
report_mismatch:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ jump return_result }
report_bad_type:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r18=#-1 }
{ jump return_result }
update_link:
{ r2=memw(r17+#0); memw(r3+#0)=r2.new }
clear_descriptor:
{ immext(#0xf0010840); r1:0=combine(r17,##0xf001084c); r21:20=combine(#0,#0); r18=memw(r17+#12) }
{ memh(r17+#6)=#0; memd(r17+#16)=r21:20 }
{ call controller_publish; memw(r17+#12)=#0 }
{ p0=cmp.eq(r18,#0); if(p0.new) jump:nt notify_done }
{ nop } // checked V66 k0lock
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r17=#-0x1112 }
linked_loop:
{ r2=memw(r18+#256) }
{ r2=memw(r2+#16); if(!cmp.eq(r2.new,#4)) jump:t deactivate }
{ r2=add(r18,#0xe0) }
{ memh(r2+#0)=setbit(#5) }
{ memw(r18+#12)=r17 }
{ r18=memw(r18+#0); if(!cmp.eq(r18.new,#0)) jump:t linked_loop }
{ jump callback_cleanup }
deactivate:
{ r0=r18; memw(r18+#0x90)=#-5; memd(r18+#8)=r21:20 }
{ call controller_deactivate; r18=memw(r18+#0); immext(#0xf0021100); memw(r18+#0xec)=##0xf002111c }
{ p0=cmp.eq(r18,#0); if(!p0.new) jump:t linked_loop }
callback_cleanup:
{ call controller_clear; r0=r16; r1=memub(r16+#21); memw(r16+#0x90)=#0 }
notify_done:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r18=#0 }
return_result:
{ r0=r18; r17:16=memd(r29+#16); r19:18=memd(r29+#8) }
{ r21:20=memd(r29+#0); dealloc_return }
.set controller_notify,adsp_record_detach_tail+(0x8be261e8-0x8be13514)
.set controller_publish,adsp_record_detach_tail+(0x8be26214-0x8be13514)
.set controller_deactivate,adsp_record_detach_tail+(0x8be26ac4-0x8be13514)
.set controller_clear,adsp_record_detach_tail+(0x8be2692c-0x8be13514)
