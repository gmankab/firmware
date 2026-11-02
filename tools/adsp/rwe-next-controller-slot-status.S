// SPDX-License-Identifier: GPL-3.0-only
// Query ADSP controller slot state using two GP-backed bitmaps.
// File [0x11e6c,0x11ed0), physical [0x8be0ee6c,0x8be0eed0].
.text
.globl adsp_controller_slot_status
adsp_controller_slot_status:
{ call get_global_context; r17:16=combine(r2,r0); memd(r29+#-16)=r17:16; allocframe(#8) }
{ p0=cmp.eq(r0,#0); if(p0.new) jump:nt query_bitmap }
return_input:
{ r17:16=memd(r29+#0); dealloc_return }
query_bitmap:
{ r1:0=bitsplit(r16,#5); p0=cmp.eq(r17,#1); if(p0.new) jump:nt query_preallocated }
{ r0=#12; p0=cmp.eq(r17,#0); if(!p0.new) jump:nt return_input }
{ r1:0=bitsplit(r16,#5); immext(#0xf000eac0); r2=memw(##0xf000eac4) }
{ r2=memw(r2+r1<<#2) }
{ p0=tstbit(r2,r0); r0=#1; if(p0.new) jump:t return_input }
{ r0=#0; r17:16=memd(r29+#0); dealloc_return }
query_preallocated:
{ immext(#0xf000ea80); r2=memw(##0xf000eab8) }
{ r2=memw(r2+r1<<#2) }
{ p0=tstbit(r2,r0); r0=#1 }
{ if(!p0) r0=#0; r17:16=memd(r29+#0); dealloc_return }
.set get_global_context,adsp_controller_slot_status+(0x8be25bd0-0x8be0ee6c)
