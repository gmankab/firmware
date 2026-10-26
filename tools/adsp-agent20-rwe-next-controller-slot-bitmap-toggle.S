// SPDX-License-Identifier: GPL-3.0-only
// Update controller slot-class bitmap, publish matching state registers.
// ADSP file [0x11f58,0x11fd8), physical [0x8be0ef58,0x8be0efd8).
.text
.globl adsp_controller_slot_bitmap_toggle
adsp_controller_slot_bitmap_toggle:
{ call get_global_context; r17:16=combine(r0,r1); memd(r29+#-16)=r17:16; allocframe(#8) }
{ r2=asl(r17,#4); if (p0.new) r0=#13; p0=cmp.eq(r0,#0); if (!p0.new) jump:t not_found }
{ immext(#0xf0414880); r3=memw(r2<<#0+##0xf04148a0); if (cmp.eq(r3.new,#0)) jump:nt active_record }
not_found:
{ r17:16=memd(r29+#0); dealloc_return }
active_record:
{ r1:0=bitsplit(r17,#5); r2=extractu(r2,#5,#4); immext(#0xf000eac0); r3=memw(##0xf000eabc) }
{ p0=cmp.eq(r16,#0); immext(#0xf020a180); r4=memw(r1<<#2+##0xf020a188) }
{ r5=clrbit(r4,r2); r4=setbit(r4,r2) }
{ if(!p0) r5=add(r4,#0) }
{ r2=lsl(#1,r2); immext(#0xf020a180); memw(r1<<#2+##0xf020a188)=r5; memw(r3+r1<<#2)=r5 }
{ immext(#0xf000eac0); r3=memw(##0xf000eac0) }
{ memw(r3+r1<<#2)=r2 }
{ immext(#0xf00250c0); r2=memw(##0xf00250c4) }
{ r2=memw(r2+#0) }
{ r0=#0; r17:16=memd(r29+#0); dealloc_return }
.set get_global_context,adsp_controller_slot_bitmap_toggle+(0x8be25bd0-0x8be0ef58)
