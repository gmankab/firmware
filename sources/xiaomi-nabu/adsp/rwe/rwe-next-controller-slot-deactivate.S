// SPDX-License-Identifier: GPL-3.0-only
// Deactivate a controller slot and update shared selection and callback state.
// ADSP file [0x12098,0x12114), physical [0x8be0f098,0x8be0f114).
.text
.globl adsp_controller_slot_deactivate
adsp_controller_slot_deactivate:
{ call get_global_context; r16=r0; memd(r29+#-16)=r17:16; allocframe(#8) }
{ r2=asl(r16,#4); if(p0.new) r0=#1; p0=cmp.eq(r0,#0); if(!p0.new) jump:t no_slot }
{ immext(#0xf0414880); r2=memw(r2<<#0+##0xf04148a0) }
{ if(p0.new) r1=add(r16,#0); p0=tstbit(r2,#0); if(!p0.new) jump:t no_slot }
{ r3=setbit(r16,#16); immext(#0xf00263c0); r2=memw(##0xf00263c4) }
{ immext(#0xf0414880); r1=add(##0xf04148a0,asl(r1,#4)) }
{ memw(r1+#0)=#0; memw(r2+#0)=r3 }
{ memw(r1+#4)=#0 }
{ immext(#0xf00250c0); r2=memw(##0xf00250c4) }
{ r2=memw(r2+#0) }
{ r2=memw(r1+#12) }
{ p0=tstbit(r2,#0); if(p0.new) r0=add(r16,#0) }
{ immext(#0x17740); if(p0) call trigger_slot_callback }
{ call publish_registration; r1:0=combine(#7,#11); r2=r16 }
{ r0=#0 }
no_slot:
{ r17:16=memd(r29+#0); dealloc_return }
.set get_global_context,adsp_controller_slot_deactivate+(0x8be25bd0-0x8be0f098)
.set publish_registration,adsp_controller_slot_deactivate+(0x8be254f0-0x8be0f098)
.set trigger_slot_callback,adsp_controller_slot_deactivate+(0x8be26854-0x8be0f098)
