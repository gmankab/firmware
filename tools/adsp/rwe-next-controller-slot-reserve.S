// SPDX-License-Identifier: GPL-3.0-only
// Reserve a controller slot with an atomic compare/exchange lock and publish
// the chosen metadata to the controller table.
// ADSP file [0x11fd8,0x12098), physical [0x8be0efd8,0x8be0f098).
.text
.globl adsp_controller_slot_reserve
adsp_controller_slot_reserve:
{ r16=r0; memd(r29+#-16)=r17:16; allocframe(#24) }
{ r19:18=combine(r2,r3); r20=r1; memd(r29+#8)=r19:18; memd(r29+#0)=r21:20 }
{ nop } // V66 R2=SGP0; checked typed system-register read inserted here
{ call get_global_context; r21=memw(r2+#0) }
{ p0=cmp.eq(r0,#0); if (!p0.new) jump:t reserve_failed }
{ r3:2=combine(#1,#0) }
{ r0=#1; r17=r16 }
{ immext(#0xf0414880); r17=add(##0xf04148a0,asl(r17,#4)) }
reserve_retry:
{ r4=memw_locked(r17) }
{ p0=cmp.eq(r4,r2) }
{ if (!p0) jump:nt reserve_acquired }
{ memw_locked(r17,p0)=r3 }
{ if (!p0) jump:nt reserve_retry }
reserve_acquired:
{ p0=cmp.eq(r4,#0); if(!p0.new) jump:t reserve_failed; if(p0.new) r3=memw(r17+#12) }
{ r1:0=bitsplit(r16,#5); r2=memw(r21+#28); memw(r17+#4)=r2.new }
{ r0=lsl(#1,r0); immext(#0xf000eac0); r2=memw(##0xf000eac0); memw(r17+#8)=r20 }
{ r3=or(r18,and(r3,#1)); memw(r17+#0)|=r19 }
{ memw(r17+#12)=r3; memw(r2+r1<<#2)=r0 }
{ immext(#0xf00250c0); r2=memw(##0xf00250c4) }
{ r2=memw(r2+#0) }
{ r1:0=combine(#6,#11); r2=r16; immext(#0xf00263c0); r3=memw(##0xf00263c4) }
{ call publish_registration; memw(r3+#0)=r16 }
{ r2=memw(r17+#12) }
{ p0=tstbit(r2,#0); if(p0.new) r1:0=combine(r17,r16) }
{ immext(#0x177c0); if(p0) call trigger_slot_callback }
{ r0=#0 }
reserve_failed:
{ r17:16=memd(r29+#16); r19:18=memd(r29+#8) }
{ r21:20=memd(r29+#0); dealloc_return }
.set get_global_context,adsp_controller_slot_reserve+(0x8be25bd0-0x8be0efd8)
.set publish_registration,adsp_controller_slot_reserve+(0x8be254f0-0x8be0efd8)
.set trigger_slot_callback,adsp_controller_slot_reserve+(0x8be26854-0x8be0efd8)
