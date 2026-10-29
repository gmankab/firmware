// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller global query prefix [0x125a0,0x125f4).
.text
.globl adsp_global_query_prefix
adsp_global_query_prefix:
{ r17:16=combine(r0,r1); memd(r29+#-16)=r17:16; allocframe(#16) }
{ memd(sp+#0)=r19:18 }
{ nop } // typed V66 R2=SGP0 register read in the verified emitter
{ r18=memw(r2+#0) }
{ call register_service; r0=memw(r18+#256) }
{ p0=cmp.eq(r0,#0); if(p0.new) r1=add(r17,#0); immext(#0x100); if(p0.new) r0=memw(r18+##0x100) }
{ immext(#0x10800); if(p0) call first_callback }
{ call second_callback; r0=memw(r18+#256) }
{ p0=cmp.eq(r0,#0); if(p0.new) r1=add(r16,#0); immext(#0x100); if(p0.new) r0=memw(r18+##0x100) }
{ immext(#0x107c0); if(p0) call third_callback }
.set register_service,adsp_global_query_prefix+(0x8be29434-0x8be125a0)
.set first_callback,adsp_global_query_prefix+(0x8be22dd0-0x8be125a0)
.set second_callback,adsp_global_query_prefix+(0x8be22de8-0x8be125a0)
.set third_callback,adsp_global_query_prefix+(0x8be22ddc-0x8be125a0)
