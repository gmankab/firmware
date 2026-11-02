// SPDX-License-Identifier: GPL-3.0-only
// Controller-slot remove/search routine prefix [0x12430,0x12498).
.text
.globl controller_slot_remove_prefix
controller_slot_remove_prefix:
{ immext(#0xf0026c40); r2=memw(GP+##0xf0026c40); allocframe(#56) }
{ p0=cmp.eq(r2,#0); r16=r0; memd(r29+#48)=r17:16; memd(r29+#40)=r19:18 }
{ memd(r29+#32)=r21:20; memd(r29+#24)=r23:22 }
{ if(p0) r0=#1; if(p0) jump:nt done; memd(sp+#16)=r25:24; memd(sp+#8)=r27:26 }
{ r2=memw(r16+#212); if(cmp.eq(r2.new,#0)) jump:nt not_found }
{ r19=add(r16,#212); r21:20=combine(#31,#0); r18=#0 }
{ if(p0.new) r0=add(r16,#0); p0=cmp.eq(r18,#1); if(p0.new) jump:nt two_ways }
{ if(p0.new) r0=add(r16,#0); p0=cmp.eq(r18,#0); if(!p0.new) jump:nt search }
{ call platform_lock }
{ r17=r0; jump search }
two_ways:
{ call platform_lock; r17=memw(r16+#256) }
{ if(p0.new) r0=#0; p0=cmp.eq(r17,r0); if(p0.new) jump:nt done }
.set done,controller_slot_remove_prefix+(0x8be12588-0x8be12430)
.set not_found,controller_slot_remove_prefix+(0x8be12584-0x8be12430)
.set search,controller_slot_remove_prefix+(0x8be12498-0x8be12430)
.set platform_lock,controller_slot_remove_prefix+(0x8be293d0-0x8be12430)
