// SPDX-License-Identifier: GPL-3.0-only
// Independent semantics for 0x56568..0x56588, plus two linker NOPs.
.text
.space 0x568
.global cdsp_null_callback
cdsp_null_callback:
{ p0=cmp.eq(r2,#0) }
{ r16=r0
  memd(r29+#-16)=r17:16
  allocframe(#8) }
{ if(!p0) call ##0x1130b8 }
{ if(!p0) r0=add(r16,#0) }
{ r0=r16
  r17:16=memd(r29+#0)
  dealloc_return }
{ nop }
{ nop }
