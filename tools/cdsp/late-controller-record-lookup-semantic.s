.text
.global record_lookup
record_lookup:
{ r3=memw(r17+r23<<#0) }
{ p1=cmp.eq(r3,#-1)
  if(p1.new) jump:nt done }
{ r0=##0xc02261f4 }
{ memw(sp+#8)=r2
  memw(sp+#4)=r3 }
{ call trace }
{ memw(sp+#0)=r22 }
{ r17=add(r16,#24) }
{ r2=memw(r16+#16) }
{ r3=add(r2,r23) }
{ r0=r17
  r1=memw(r2+r23<<#0) }
{ r3=memw(r3+#4) }
{ call service }
{ r2=lsr(r3,#12) }
{ r2=memw(r18+#0) }
{ p1=cmp.eq(r2,r16)
  if(p1.new) jump:nt done }
.org 0x78
done: { nop }
.org 0x8800
service: { nop }
.org 0xf817c
trace: { nop }
