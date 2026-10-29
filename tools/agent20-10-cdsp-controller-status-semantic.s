.text
.global cdsp_status
cdsp_status:
{ r2=memw(gp+##0xc028c0f0) }
{ r2=lsr(r2,#12)
  memw(r17+#32)=r2.new }
{ call service }
{ r1:0=combine(##0xc02261e1,r17) }
{ p0=cmp.eq(r0,#0)
  r2=memw(r17+#36) }
{ p1=cmp.eq(r2,#0)
  if(p1.new) jump:nt missing }
{ if(p0) memw(##0xc036415c)=r16 }
{ r23:22=combine(#32,#0) }
{ nop }
.org 0xc0
missing:
{ nop }
.org 0x1168c4
service:
{ nop }
