// Semantic Hexagon V60 fixture for CDSP controller 0x56954..0x56990.
// LLVM6 may expand compressed duplex pairs and change packet scheduling.
.text
.global cdsp_controller_scan
cdsp_controller_scan:
loop:
{ p0=cmp.eq(r17,#0); if(p0.new) jump:nt skip_helper }
{ call helper }
{ r1=add(fp,#24); r0=r16 }
{ p0=cmp.eq(r0,#0)
  if(p0.new) r19=add(r17,#0) }
skip_helper:
{ r17=add(r17,#1) }
{ r16=add(r16,#160) }
{ r2=memb(r16+#160) }
{ p0=cmp.eq(r2,#0); if(!p0.new) jump:t loop }
{ nop }
{ if(p0) r2=memw(sp+#20) }
{ p0=r2 }
{ if(!p0) r21=#0 }
{ if(p0) jump:nt exit }
{ nop }
.org 0x17c
exit:
{ nop }
.org 0x116968+4
helper:
{ nop }
