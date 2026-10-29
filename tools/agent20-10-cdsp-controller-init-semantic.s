// Independently-authored Hexagon V60 semantic fixture for CDSP 0x56930.
// LLVM may move the R17:16 duplex ahead of the byte-load extender.
.text
.global cdsp_controller_init
cdsp_controller_init:
{ call boot_helper }
{ memw(r18=##0xc0364ae0) = r19 }
{ r2=memb(r20=##0xc02a0e40)
  r17:16=combine(#0,r20)
  p0=cmp.eq(r2,#0); if (p0.new) jump:nt failure }
{ r2=p0
  memw(sp+#20)=r2.new }
.org 0x1a0
failure:
{ nop }
.org 0xfb918
boot_helper:
{ nop }
