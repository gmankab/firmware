// SPDX-License-Identifier: GPL-3.0-only
// Four adjacent ADSP controller slot helpers, file [0x11d68,0x11e6c).
.text
.globl adsp_controller_slot_helpers
adsp_controller_slot_helpers:
// Lookup an allocated controller slot.
slot_lookup:
{ call get_global_context; r16=r0; memd(r29+#-16)=r17:16; allocframe(#8) }
{ r2=asl(r16,#4); p0=cmp.eq(r0,#0); if(p0.new) jump:nt lookup_value; if(p0.new) r17:16=memd(r29+#0) }
{ r17:16=memd(r29+#0); dealloc_return }
lookup_value:
{ immext(#0xf0414880); r0=memw(r2<<#0+##0xf04148a0); dealloc_return }
// Mark a currently allocated slot as not active.
slot_clear:
{ r16=r0; memd(r29+#-16)=r17:16; allocframe(#8) }
{ r2=asl(r16,#4) }
{ immext(#0xf0414880); r2=memw(r2<<#0+##0xf04148a0); if(cmp.eq(r2.new,#0)) jump:nt clear_absent }
{ r2=r16; immext(#0xf00263c0); r3=memw(##0xf00263c4) }
{ immext(#0xf0414880); r2=add(##0xf04148a0,asl(r2,#4)); immext(#0xf00263c0); r0=##0xf00263cc }
{ memw(r2+#12)=clrbit(#1) }
{ call slot_notify; memw(r3+#0)=r16 }
{ call record_service; r1:0=combine(#3,#11); r2=r16 }
{ r0=#0; r17:16=memd(r29+#0); dealloc_return }
clear_absent:
{ call slot_notify; immext(#0xf00263c0); r0=##0xf00263cc }
{ r0=#1; r17:16=memd(r29+#0); dealloc_return }
// Mark a slot active and publish to the controller context.
slot_set:
{ r16=r0; memd(r29+#-16)=r17:16; allocframe(#8) }
{ r2=asl(r16,#4) }
{ immext(#0xf0414880); r2=memw(r2<<#0+##0xf04148a0); if(cmp.eq(r2.new,#0)) jump:nt set_absent }
{ r4=setbit(r16,#16); r2=r16; immext(#0xf00263c0); r3=memw(##0xf00263c4) }
{ immext(#0xf0414880); r2=add(##0xf04148a0,asl(r2,#4)); immext(#0xf00263c0); r0=##0xf00263cc }
{ memw(r3+#0)=r4 }
{ call slot_notify; memw(r2+#12)=setbit(#1) }
{ call record_service; r1:0=combine(#4,#11); r2=r16 }
{ r0=#0; r17:16=memd(r29+#0); dealloc_return }
set_absent:
{ call slot_notify; immext(#0xf00263c0); r0=##0xf00263cc }
{ r0=#1; r17:16=memd(r29+#0); dealloc_return }
// Sentinel returned for unsupported class indices.
slot_sentinel:
{ immext(#0x80000000); r0=##0x80000000; jumpr lr }
.set get_global_context,adsp_controller_slot_helpers+(0x8be25bd0-0x8be0ed68)
.set slot_notify,adsp_controller_slot_helpers+(0x8be231e8-0x8be0ed68)
.set record_service,adsp_controller_slot_helpers+(0x8be254f0-0x8be0ed68)
