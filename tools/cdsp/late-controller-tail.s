.text
.global cdsp_controller_tail
cdsp_controller_tail:
.space 0x408
{ r0=##0x0830002c }
{ r1=memw(r0+#0) }
{ r2=##0x700
  r3=##0x1f }
{ r2=or(r2,r3) }
{ r2=sub(#-1,r2) }
{ r1=and(r1,r2) }
{ r3=asl(r16,#0)
  r2=##0x200 }
{ r2=or(r2,r3) }
{ r1=or(r1,r2) }
{ memw(r0+#0)=r1 }
{ barrier }
{ r0=##0x08300028 }
{ r1=memw(r0+#0) }
{ r2=##1 }
{ r1=or(r1,r2) }
{ memw(r0+#0)=r1 }
{ barrier }
loop_wait:
{ r1=memw(r0+#0) }
{ r3=and(r1,r2) }
{ p0=cmp.eq(r3,r2)
  if (p0.new) jump:t loop_wait }
{ r0=##0x083a0040 }
{ memw(r0+#0)=##0 }
{ r0=##0x083a0044 }
{ memw(r0+#0)=##0 }
{ r0=##0x083a0048 }
{ memw(r0+#0)=##0 }
{ barrier }
{ jumpr lr }
