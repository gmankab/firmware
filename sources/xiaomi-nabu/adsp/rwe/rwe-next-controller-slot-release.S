// SPDX-License-Identifier: GPL-3.0-only
// Release one ADSP controller slot, clear bitmap/state and emit notifications.
// File [0x11c88,0x11d68), physical [0x8be0ec88,0x8be0ed68).
.text
.globl adsp_controller_slot_release
adsp_controller_slot_release:
{ r17=r0; memd(r29+#-16)=r17:16; allocframe(#24) }
{ memd(r29+#8)=r19:18; memd(r29+#0)=r21:20 }
{ nop } // checked V66 R2=SGP0
{ call get_global_context; r16=memw(r2+#0) }
{ r20=r0; if(cmp.eq(r20.new,#0)) jump:nt clear_slot }
initial_return:
{ r0=r20; r17:16=memd(r29+#16); r19:18=memd(r29+#8) }
{ r21:20=memd(r29+#0); dealloc_return }
clear_slot:
{ r2=#0; r20=r17 }
{ immext(#0xf0414880); r20=add(##0xf04148a0,asl(r20,#4)) }
clear_atomic_retry:
{ r18=memw_locked(r20) }
{ memw_locked(r20,p0)=r2 }
{ if(!p0) jump:nt clear_atomic_retry }
{ r2=asl(r17,#4); p0=cmp.eq(r18,#0); if(p0.new) jump:nt no_record; if(!p0.new) r0=memw(r20+#4) }
recycle_controller:
{ r2=sub(#-1,r0); immext(#0xf0414880); r19=memw(r2<<#0+##0xf04148a8) }
{ r3=add(r19,#212) }
clear_low_bitmap_retry:
{ r21=memw_locked(r3) }
{ r21=and(r21,r2) }
{ memw_locked(r3,p0)=r21 }
{ if(!p0) jump:nt clear_low_bitmap_retry }
{ r1:0=combine(#2,#11); r2=r17; immext(#0xf00263c0); r3=memw(GP+##0xf00263c4) }
{ call publish_registration; r4=setbit(r17,#16); memw(r3+#0)=r4.new }
{ r2=memw(r20+#12); memw(r20+#4)=#0 }
{ p0=tstbit(r2,#0); if(p0.new) r1:0=combine(r20,r17) }
{ immext(#0x17b00); if(p0) call trigger_slot_callback }
{ r20=#0; p0=cmp.eq(r21,#0); if(!p0.new) jump:t initial_return }
{ r2=add(r19,#216); immext(#0x80000000); r3=##0x80000000 }
clear_high_bitmap_retry:
{ r4=memw_locked(r2) }
{ memw_locked(r2,p0)=r3 }
{ if(!p0) jump:nt clear_high_bitmap_retry }
{ immext(#0x80000000); r1:0=combine(##0x80000000,r18); r3:2=combine(r16,r19); memw(r16+#144)=#0 }
{ call resource_sync }
{ jump initial_return }
no_record:
{ r20=#-1; jump initial_return }
.set get_global_context,adsp_controller_slot_release+(0x8be25bd0-0x8be0ec88)
.set publish_registration,adsp_controller_slot_release+(0x8be254f0-0x8be0ec88)
.set trigger_slot_callback,adsp_controller_slot_release+(0x8be26854-0x8be0ec88)
.set resource_sync,adsp_controller_slot_release+(0x8be22d20-0x8be0ec88)
