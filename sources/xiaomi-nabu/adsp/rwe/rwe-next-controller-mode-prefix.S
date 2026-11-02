// SPDX-License-Identifier: GPL-3.0-only
// Source-reconstructed V66 controller mode selection prefix, file [0x139e0,0x13a2c).
.text
.globl adsp_controller_mode_prefix
adsp_controller_mode_prefix:
{ r16=r1 }
{ memd(r29+#-16)=r17:16; allocframe(#32) }
{ memd(sp+#16)=r19:18 }
{ nop } // typed V66 privileged R2=SGP0
{ r18=#0; p0=cmp.eq(r0,#0); if(p0.new) jump:nt normal_case }
return_mode:
{ r0=r18 }
{ r17:16=memd(r29+#24); r19:18=memd(r29+#16) }
{ dealloc_return }
normal_case:
{ p0=cmp.gt(r16,#3); if(p0.new) jump:t extended_case }
{ r17=memw(r2+#0) }
{ p0=cmp.eq(r16,#0); if(p0.new) jump:nt first_case }
{ p0=cmp.eq(r16,#1); if(p0.new) jump:t clock_case }
{ jump return_mode }
extended_case:
{ p0=cmp.eq(r16,#4); if(p0.new) jump:nt clock_case }
{ p0=cmp.eq(r16,#5); if(!p0.new) jump:nt return_mode }
{ immext(#0xf02d1480); r2=memw(gp+##0xf02d1488); if(cmp.eq(r2.new,#0)) jump:nt absent_controller }
.set clock_case,adsp_controller_mode_prefix+(0x13a2c-0x139e0)
.set first_case,adsp_controller_mode_prefix+(0x13a7c-0x139e0)
.set absent_controller,adsp_controller_mode_prefix+(0x13b9c-0x139e0)
