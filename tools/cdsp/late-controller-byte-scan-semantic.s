.text
.global cdsp_scan
cdsp_scan:
{ r0=##0xc02261cd }
{ r2=memw(r16+#16)
  memw(r29+#8)=r16 }
{ call trace }
{ r22=add(sp,#24)
  memw(r29+#4)=r2
  memw(r29+#0)=r21 }
{ r2=#0
  r17=memw(r16+#16) }
loop:
{ p0=cmp.gtu(r2,#31)
  if(p0.new) jump:nt done
}
{ r3=memb(r22+#0) }
{ p1=cmp.eq(r3,#0)
  if(p1.new) jump:nt done }
{ r2=add(r2,#1)
  r22=add(r22,#1)
  r4=memb(r17+r2<<#0) }
{ p0=cmp.eq(r4,r3)
  if(p0.new) jump:t loop }
{ jump result }
done:
{ nop }
.org 0x4c
result:
{ nop }
.org 0xf81ec
trace:
{ nop }
