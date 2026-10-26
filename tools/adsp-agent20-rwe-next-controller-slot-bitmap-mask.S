// SPDX-License-Identifier: GPL-3.0-only
// Query a slot-class bitmap and return its controller bit mask.
// File [0x11f18,0x11f58), physical [0x8be0ef18,0x8be0ef58).
.text
.globl adsp_controller_slot_bitmap_mask
adsp_controller_slot_bitmap_mask:
{ call get_global_context; r16=r0; memd(r29+#-16)=r17:16; allocframe(#8) }
{ r2=and(r16,#31); p0=cmp.eq(r0,#0); if(p0.new) jump:nt have_table }
{ r17:16=memd(r29+#0); dealloc_return }
have_table:
{ immext(#0x1fffffc0); r16=and(##0x1ffffffc,lsr(r16,#3)) }
{ immext(#0xf020a180); r3=memw(r16<<#0+##0xf020a188); r17:16=memd(r29+#0) }
{ p0=!tstbit(r3,r2); if(p0.new) r0=#0; immext(#0x10000); if(!p0.new) r0=##0x10000 }
{ dealloc_return }
.set get_global_context,adsp_controller_slot_bitmap_mask+(0x8be25bd0-0x8be0ef18)
