// SPDX-License-Identifier: GPL-3.0-only
// Independently authored Hexagon V60 semantic assembly for CDSP callback
// cleanup. LLVM6 may use a shorter GP addressing mode and may add an
// immediate extender; the verified firmware keeps different packet choices.
.text
.global cdsp_rwe_callback_registry
cdsp_rwe_callback_registry:
.space 0x1a0
runtime_check:
{ r2=memw(gp+##0xc0363e00)
  allocframe(#8) }
{ p0=cmp.eq(r2,#0)
  if(!p0.new) jump:nt done
  memd(sp+#0)=r17:16 }
{ r2=memw(r16=##0xc028c024)
  if(cmp.eq(r2.new,#0)) jump:nt done }
{ r2=memw(gp+##0xc028c014)
  if(cmp.eq(r2.new,#0)) jump:nt zero }
{ r1:0=combine(#4,r16)
  callr r2 }
zero:
{ r2=##0
  memw(r16+#0)=r2.new }
done:
{ r17:16=memd(r29+#0)
  dealloc_return }
