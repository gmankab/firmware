// SPDX-License-Identifier: GPL-3.0-only
// Publish an ADSP controller slot bitmap bit, then notify the platform.
// File [0x11ed0,0x11f18) physical [0x8be0eed0,0x8be0ef18).
.text
.globl adsp_controller_slot_bitmap_publish
adsp_controller_slot_bitmap_publish:
{ call get_global_context; r16=r0; memd(r29+#-16)=r17:16; allocframe(#8) }
{ p0=cmp.eq(r0,#0); if(p0.new) jump:nt publish; immext(#0xf000eac0); if(p0.new) r2=memw(##0xf000eac0) }
{ r17:16=memd(r29+#0); dealloc_return }
publish:
{ r1:0=bitsplit(r16,#5) }
{ r3=lsl(#1,r0); memw(r2+r1<<#2)=r3.new }
{ immext(#0xf00250c0); r2=memw(GP+##0xf00250c4) }
{ r2=memw(r2+#0) }
{ call publish_slot_state; r1:0=combine(#5,#11); r2=r16 }
{ r0=#0; r17:16=memd(r29+#0); dealloc_return }
.set get_global_context,adsp_controller_slot_bitmap_publish+(0x8be25bd0-0x8be0eed0)
.set publish_slot_state,adsp_controller_slot_bitmap_publish+(0x8be254f0-0x8be0eed0)
