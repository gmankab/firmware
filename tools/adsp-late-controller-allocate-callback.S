// SPDX-License-Identifier: GPL-3.0-only
// [ADSP file 0x41220,0x412b4): allocate a callback-enabled controller
// context, register optional hooks and release a failed allocation.
.text
.globl adsp_late_controller_allocate_callback
adsp_late_controller_allocate_callback:
{ p0 = cmp.eq(r0,#3); r17:16 = combine(r5,#-1); memd(r29+#-16) = r17:16; allocframe(#24) }
{ r18 = r4; r20 = r3; memd(r29+#8) = r19:18; memd(r29+#0) = r21:20 }
{ if (p0) r0 = #364; if (!p0) jump:nt failed_allocate }
{ call allocate_context }
{ r19 = r0; if (cmp.eq(r19.new,#0)) jump:nt failed_allocate }
{ call create_descriptor; r1:0 = combine(#0,r19); r2 = #364 }
{ p0 = cmp.eq(r20,#0); r21 = add(r19,#204) }
{ immext(#233216); if (!p0) call register_callback; if (!p0) r1:0 = combine(r20,r21); if (!p0) r2 = #32 }
{ r3 = asl(r17,#12); r1:0 = combine(#1,r19); r2 = r21; memw(r19+#240) = r3.new }
{ call publish_context; memw(r19+#236) = r18 }
{ p0 = cmp.eq(r0,#0); if (!p0.new) r0 = add(r19,#24); if (p0.new) jump:nt free_context }
{ call init_callback; r1 = r18; r2 = r17 }
{ jump failed_allocate; r16 = memw(r19+#20) }
free_context:
{ call release_context; r0 = r19 }
failed_allocate:
{ r0 = r16; r17:16 = memd(r29+#16); r19:18 = memd(r29+#8) }
{ r21:20 = memd(r29+#0); dealloc_return }
.set allocate_context, adsp_late_controller_allocate_callback-0x66b4
.set create_descriptor, adsp_late_controller_allocate_callback+0x1007b0
.set register_callback, adsp_late_controller_allocate_callback+0x38f58
.set publish_context, adsp_late_controller_allocate_callback-0x500
.set init_callback, adsp_late_controller_allocate_callback+0x1030
.set release_context, adsp_late_controller_allocate_callback-0x6280
