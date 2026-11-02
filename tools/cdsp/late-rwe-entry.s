.text
.global cdsp_RWE_runtime_loader
cdsp_RWE_runtime_loader:
{ allocframe(#8) }
{ memw(sp+#0)=r27 }
{ r0.h=#0xc028 }
{ r0.l=#0x9d20 }
{ r1=memw(r0+#0) }
{ p0=cmp.eq(r1,#0); if(p0.new) jump:nt skip_first }
{ r3=#0
  r2=#0
  r1=#0 }
{ r28.h=#0xc01e }
{ r28.l=#0xbfc }
{ callr r28 }
skip_first:
{ r27.h=#0xc028 }
{ r27.l=#0xb008 }
loop_callbacks:
{ r27=add(r27,#-4) }
{ r0=memw(r27+#0) }
{ p0=cmp.eq(r0,#0); if(p0.new) jump:nt return_main }
{ callr r0 }
{ jump loop_callbacks }
return_main:
{ r27=memw(sp+#0)
  deallocframe }
{ jumpr lr }
