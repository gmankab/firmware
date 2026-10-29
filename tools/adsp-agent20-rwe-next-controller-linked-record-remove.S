// SPDX-License-Identifier: GPL-3.0-only
// V66 controller linked-record removal [file 0x13750,0x13828).
// The source preserves error continuations after a common return target.
.text
.globl adsp_linked_record_remove
adsp_linked_record_remove:
{ r16=r0; memd(r29+#-16)=r17:16; allocframe(#16) }
{ immext(#0xf0010840); r0=##0xf0010844; memd(r29+#0)=r19:18 }
{ call controller_lookup }
{ immext(#0xf0010840); r18=memw(gp+##0xf0010848); r17=memw(r16+#0x94) }
{ r0=#3; p0=cmp.eq(r0,#0); if(!p0.new) jump:t return_result }
{ p0=cmp.eq(r18,r17); if(p0.new) jump:nt check_head }
{ p0=cmp.eq(r18,#0); if(p0.new) jump:nt check_head }
walk_global:
{ r18=memw(r18+#0); if(cmp.eq(r18.new,r17)) jump:nt compare_owner }
{ p0=cmp.eq(r18,#0); if(!p0.new) jump:t walk_global }
{ jump notify_missing }
check_head:
{ p0=cmp.eq(r18,#0); if(p0.new) jump:nt notify_missing }
compare_owner:
{ r3=memw(r17+#12); if(cmp.eq(r3.new,r16)) jump:nt remove_head }
walk_owners:
{ r2=r3; if(cmp.eq(r2.new,#0)) jump:nt invalid_owner }
{ r3=memw(r3+#0); if(!cmp.eq(r3.new,r16)) jump:t walk_owners }
{ immext(#0xf0010840); r0=##0xf0010844; r3=memw(r16+#0); memw(r2+#0)=r3.new }
{ call controller_notify }
{ jump mark_removed }
notify_missing:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ r0=#-1; jump return_result }
remove_head:
{ immext(#0xf0010840); r0=##0xf0010844; r2=memw(r16+#0); memw(r17+#12)=r2.new }
{ call controller_notify }
{ r0=#-1; p0=cmp.eq(r16,#0); if(p0.new) jump:nt return_result }
mark_removed:
{ r2=add(r16,#0xe0); r0=#0; memw(r16+#0x90)=#0x25 }
{ r2=#-0x1112; memh(r2+#0)=setbit(#5) }
{ jump return_result; memw(r16+#12)=r2 }
invalid_owner:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ r0=#-1 }
return_result:
{ r17:16=memd(r29+#8); r19:18=memd(r29+#0) }
{ dealloc_return }
.set controller_lookup,adsp_linked_record_remove+(0x8be261d0-0x8be13750)
.set controller_notify,adsp_linked_record_remove+(0x8be261e8-0x8be13750)
